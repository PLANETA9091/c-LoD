//! The streaming core: per-region LOD stores encoded straight into the
//! vanilla map-color alphabet, served to clients as `ClientboundMapItemDataPacket`
//! patches over 8 virtual "pages" (128x128 maps) per player.
//!
//! Geometry: one REGION is `region_blocks` (512) wide -> `region_cells` (256)
//! cells of `cell_size` (2) blocks. A region needs 8 pages:
//!   page 0..3 -> COLOR layer, quadrant q = p & 3 (qx = q & 1, qz = q >> 1);
//!   page 4..7 -> HEIGHT layer, same quadrants.
//! A page texel (x, y) maps to region cell ((qx*128 + x), (qz*128 + y)) except
//! texels (0,0) and (1,0), which are the PAGE MARKERS:
//!   (0,0) = region slot digit (0..stream_slots-1),
//!   (1,0) = page digit (0..7),
//!   (2,0) = base-Y high nibble, (3,0) = base-Y low nibble (base = nibbles-128).
//! The two data cells under the marker texels are permanently void (2x2
//! blocks per 512x512 region — invisible at LOD distance).
//!
//! Region slots: a region always streams into slot `parity(rx) + 2*parity(rz)`
//! — the SAME mapping the client shader derives from the world position, so
//! the server and the marcher agree without any out-of-band state, and up to
//! 4 regions (current + history) stay resident in the client's persistent
//! scrape targets.
//!
//! Per page the streamer keeps a `sent` shadow of the last encoded 128x128
//! bytes; every tick each page advances its cursor, re-encodes visited cells
//! (they may have been rescan-updated), and produces a bounding-rect patch of
//! the rows that differ. Patches travel as raw map packets (one per dirty
//! page per tick).

use crate::mapcolor;
use std::collections::HashMap;

pub const PAGE_MAP_SIZE: usize = 128;
pub const PAGES_PER_REGION: usize = 8;
/// Markers: slot, page, base-Y hi, base-Y lo.
pub const MARKER_TEXELS: usize = 4;

#[derive(Clone, Copy, Default, PartialEq)]
pub struct Cell {
    pub height: i16,
    pub water: bool,
    pub valid: bool,
    /// Surface albedo (palette.rs) — quantized lazily at encode time.
    pub rgb: [u8; 3],
}

pub struct Region {
    /// region_cells^2 cells, row-major z outer.
    cells: Vec<Cell>,
    /// 8 page images: last bytes SENT to the client (16 KiB each).
    sent: Vec<Vec<u8>>,
    /// 8 cursors: next cell offset (0..16384) to re-check on each page.
    cursor: [u32; PAGES_PER_REGION],
    pub scanned: u64,
}

impl Region {
    fn new(_rx: i32, _rz: i32, cells_side: usize) -> Self {
        Region {
            cells: vec![Cell::default(); cells_side * cells_side],
            sent: vec![vec![0u8; PAGE_MAP_SIZE * PAGE_MAP_SIZE]; PAGES_PER_REGION],
            cursor: [0; PAGES_PER_REGION],
            scanned: 0,
        }
    }

    pub fn valid_cells(&self) -> u64 {
        self.cells.iter().filter(|c| c.valid).count() as u64
    }

    pub fn get(&self, cx: u32, cz: u32) -> Option<Cell> {
        let side = (self.cells.len() as f64).sqrt() as usize;
        self.cells.get(cz as usize * side + cx as usize).copied()
    }

    fn set(&mut self, cx: u32, cz: u32, side: u32, cell: Cell) {
        let idx = cz as usize * side as usize + cx as usize;
        if let Some(slot) = self.cells.get_mut(idx) {
            let was_valid = slot.valid;
            if *slot != cell {
                *slot = cell;
                if !was_valid && cell.valid {
                    self.scanned += 1;
                }
            }
        }
    }
}

/// The whole streaming state (server side).
pub struct Streamer {
    pub region_blocks: i32,
    pub cell_size: i32,
    pub slots: u32,
    regions: HashMap<(i32, i32), Region>,
}

/// A patch ready for the wire.
pub struct PagePatch {
    /// Bounding rect in page texels.
    pub x: u8,
    pub y: u8,
    pub w: u8,
    pub h: u8,
    /// Row-major rect bytes (w*h).
    pub bytes: Vec<u8>,
}

impl Streamer {
    pub fn new(region_blocks: i32, cell_size: i32, slots: u32) -> Self {
        Streamer {
            region_blocks,
            cell_size,
            slots: slots.clamp(1, 4),
            regions: HashMap::new(),
        }
    }

    pub fn region_cells(&self) -> u32 {
        (self.region_blocks / self.cell_size).max(1) as u32
    }

    pub fn region_of(&self, bx: i32, bz: i32) -> (i32, i32) {
        let rb = self.region_blocks;
        (bx.div_euclid(rb), bz.div_euclid(rb))
    }

    /// Deterministic client-side slot for a region (parity mapping, shared
    /// with the GLSL marcher).
    pub fn slot_of(&self, rx: i32, rz: i32) -> u32 {
        // TEMPORARY BENCH: single-slot mode — the parity mapping returns once
        // the carrier loop is visually proven end-to-end.
        let _ = (rx, rz);
        0
    }

    pub fn region_mut(&mut self, rx: i32, rz: i32) -> &mut Region {
        let side = self.region_cells();
        self.regions
            .entry((rx, rz))
            .or_insert_with(|| Region::new(rx, rz, side as usize))
    }

    pub fn region(&self, rx: i32, rz: i32) -> Option<&Region> {
        self.regions.get(&(rx, rz))
    }

    pub fn set_cell(&mut self, rx: i32, rz: i32, cx: u32, cz: u32, cell: Cell) {
        let side = self.region_cells();
        if cx >= side || cz >= side {
            return;
        }
        self.region_mut(rx, rz).set(cx, cz, side, cell);
    }

    /// Advance one page's cursor: re-encode visited cells and collect the
    /// changed ones into a patch. Returns None when the page is clean.
    pub fn page_patch(
        &mut self,
        rx: i32,
        rz: i32,
        page: usize,
        budget: usize,
    ) -> Option<PagePatch> {
        assert!(page < PAGES_PER_REGION);
        let side = self.region_cells();
        let slot = self.slot_of(rx, rz);
        let slot_digit = (slot % self.slots.max(1)) as u8;
        let total = (PAGE_MAP_SIZE * PAGE_MAP_SIZE) as u32;
        let mut changed: Vec<(usize, u8)> = Vec::new();
        {
            let region = self.regions.get_mut(&(rx, rz))?;
            let mut cur = region.cursor[page];
            let mut steps = 0usize;
            let take = (budget.max(1)).min(total as usize);
            let layer_color = page < 4;
            let q = page & 3;
            let qx = (q & 1) as u32;
            let qz = (q >> 1) as u32;

            // Region base Y (must match encode_page).
            let mut base = i32::MAX;
            for c in &region.cells {
                if c.valid && !c.water {
                    base = base.min(c.height as i32);
                }
            }
            if base == i32::MAX {
                base = 63;
            }
            base = (base - 4).clamp(-128, 127);
            let nib = (base + 128) as u8;

            while steps < take {
                let t = cur as usize;
                let x = t % PAGE_MAP_SIZE;
                let y = t / PAGE_MAP_SIZE;
                let want = if y == 0 && x < MARKER_TEXELS {
                    match x {
                        0 => mapcolor::marker_byte(slot_digit),
                        1 => mapcolor::marker_byte(page as u8),
                        2 => mapcolor::marker_byte(nib >> 4),
                        3 => mapcolor::marker_byte(nib & 0x0F),
                        4 => mapcolor::marker_byte(((rx % 16) + 16) as u8 % 16),
                        5 => mapcolor::marker_byte(((rz % 16) + 16) as u8 % 16),
                        _ => mapcolor::marker_byte(15),
                    }
                } else {
                    let cx = qx * PAGE_MAP_SIZE as u32 + x as u32;
                    let cz = qz * PAGE_MAP_SIZE as u32 + y as u32;
                    let cell = region
                        .cells
                        .get(cz as usize * side as usize + cx as usize)
                        .copied()
                        .unwrap_or_default();
                    if !cell.valid {
                        mapcolor::void_code()
                    } else if layer_color {
                        if cell.water {
                            mapcolor::color_byte([64, 64, 255])
                        } else {
                            mapcolor::color_byte(cell.rgb)
                        }
                    } else if cell.water {
                        mapcolor::water_code()
                    } else {
                        mapcolor::height_code(cell.height as i32, base)
                    }
                };
                if region.sent[page][t] != want {
                    region.sent[page][t] = want;
                    changed.push((t, want));
                }
                cur = (cur + 1) % total;
                region.cursor[page] = cur;
                steps += 1;
            }
        }
        if changed.is_empty() {
            return None;
        }
        // Bounding rect of the changed texels.
        let minx = changed
            .iter()
            .map(|(t, _)| t % PAGE_MAP_SIZE)
            .min()
            .unwrap();
        let maxx = changed
            .iter()
            .map(|(t, _)| t % PAGE_MAP_SIZE)
            .max()
            .unwrap();
        let miny = changed
            .iter()
            .map(|(t, _)| t / PAGE_MAP_SIZE)
            .min()
            .unwrap();
        let maxy = changed
            .iter()
            .map(|(t, _)| t / PAGE_MAP_SIZE)
            .max()
            .unwrap();
        let w = maxx - minx + 1;
        let h = maxy - miny + 1;
        let mut bytes = vec![0u8; w * h];
        for (t, b) in changed {
            let x = t % PAGE_MAP_SIZE;
            let y = t / PAGE_MAP_SIZE;
            bytes[(y - miny) * w + (x - minx)] = b;
        }
        let _ = slot; // carried by the page markers themselves
        Some(PagePatch {
            x: minx as u8,
            y: miny as u8,
            w: w as u8,
            h: h as u8,
            bytes,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn streamer() -> Streamer {
        Streamer::new(512, 2, 4)
    }

    #[test]
    fn region_and_slot_math() {
        let s = streamer();
        assert_eq!(s.region_cells(), 256);
        assert_eq!(s.region_of(-1, 800), (-1, 1));
        // TEMPORARY BENCH: single-slot mode — every region maps to slot 0.
        assert_eq!(s.slot_of(0, 0), 0);
        assert_eq!(s.slot_of(1, 0), 0);
        assert_eq!(s.slot_of(0, 1), 0);
        assert_eq!(s.slot_of(-1, -1), 0);
        assert_eq!(s.slot_of(3, -2), 0);
    }

    #[test]
    fn patch_flow_full_and_incremental() {
        let mut s = streamer();
        // Scan a row of the current region.
        for cx in 10..80u32 {
            s.set_cell(
                0,
                0,
                cx,
                5,
                Cell {
                    height: 70,
                    water: false,
                    valid: true,
                    rgb: [127, 178, 56],
                },
            );
        }
        // First sweep with a big budget must produce one big patch containing
        // the whole row (plus markers).
        let p = s.page_patch(0, 0, 0, 1 << 20).expect("dirty page 0");
        assert!(p.w as usize + p.x as usize <= PAGE_MAP_SIZE);
        // The row must be inside the rect.
        assert!(p.y <= 5 && p.y + p.h > 5);
        // Height layer (page 4) also dirty (same cells).
        assert!(s.page_patch(0, 0, 4, 1 << 20).is_some());
        // First full sweep of every remaining page (they were never visited).
        for page in 0..PAGES_PER_REGION {
            let _ = s.page_patch(0, 0, page, 1 << 20);
        }
        // Second sweep: everything sent, nothing dirty (markers included).
        for page in 0..PAGES_PER_REGION {
            assert!(
                s.page_patch(0, 0, page, 1 << 20).is_none(),
                "page {page} must be clean"
            );
        }
    }

    #[test]
    fn patch_budget_spreads_work() {
        let mut s = streamer();
        for cx in 0..128u32 {
            for cz in 0..128u32 {
                s.set_cell(
                    0,
                    0,
                    cx,
                    cz,
                    Cell {
                        height: 65,
                        water: false,
                        valid: true,
                        rgb: [127, 178, 56],
                    },
                );
            }
        }
        // Small budget: each sweep reports SOME progress until the page is
        // fully transmitted; page 0 must take many sweeps for 16K texels.
        let mut sweeps = 0;
        while s.page_patch(0, 0, 0, 128).is_some() && sweeps < 100_000 {
            sweeps += 1;
        }
        assert!(sweeps >= 100, "a full page needs many budgeted sweeps");
    }

    #[test]
    fn markers_carry_slot_page_base() {
        let mut s = streamer();
        s.set_cell(
            0,
            0,
            5,
            5,
            Cell {
                height: 90,
                water: false,
                valid: true,
                rgb: [127, 178, 56],
            },
        );
        let _p = s.page_patch(0, 0, 1, 1 << 20).expect("page 1 dirty");
        // Decode the marker digits back out of the page image via `sent`.
        let img = &s.region(0, 0).unwrap().sent[1];
        assert_eq!(mapcolor::marker_value(img[0]), Some(0)); // slot
        assert_eq!(mapcolor::marker_value(img[1]), Some(1)); // page
        let hi = mapcolor::marker_value(img[2]).unwrap();
        let lo = mapcolor::marker_value(img[3]).unwrap();
        let base = (hi << 4 | lo) as i32 - 128;
        assert_eq!(base, 86, "base = min height - 4");
    }

    #[test]
    fn void_and_water_semantics() {
        let mut s = streamer();
        // Water column + untouched void.
        s.set_cell(
            0,
            0,
            9,
            9,
            Cell {
                height: 62,
                water: true,
                valid: true,
                rgb: [63, 118, 228],
            },
        );
        let _ = s.page_patch(0, 0, 4, 1 << 20).unwrap(); // height page quadrant 0
        let img = &s.region(0, 0).unwrap().sent[4];
        // Texel of cell (9,9) in quadrant 1 of the height layer: x=9,y=9.
        let t = 9 * PAGE_MAP_SIZE + 9;
        assert_eq!(img[t], mapcolor::water_code());
        // An untouched cell stays void.
        assert_eq!(img[64 * PAGE_MAP_SIZE + 64], mapcolor::void_code());
    }
}
