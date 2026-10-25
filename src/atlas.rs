//! The LOD heightmap atlas: a `cells x cells` grid covering a square world
//! region. Cell data:
//!   - height: i16 block Y (decoded from the atlas PNG's R/G channels),
//!   - water:  bool (sea surface instead of terrain),
//!   - top:    0xRRGGBB average colour of the block's top face (mip average,
//!     biome tint applied) — what Voxy's model atlas collapses to,
//!   - side:   0xRRGGBB same for the side face, used on cliff/step faces.
//!
//! Static-pack lifecycle (v3): the atlas is filled ONCE (JNI budget per tick
//! until completion or the scan deadline), optionally persisted to
//! `c-LoD-atlas.bin`, encoded into the resource pack exactly once, and never
//! mutated again. Restarts restore the snapshot so the pack SHA-1 — and the
//! client download state — stays identical across reboots.
//!
//! The scan walks the grid in CENTRE-OUT (Chebyshev-ring) order, so a scan
//! cut short by the deadline still covers a disc around the atlas centre —
//! the camera position — instead of a west-side half-plane: the LOD horizon
//! edge stays symmetric in every direction and sinks into the fog band
//! (Distant Horizons prioritises LOD generation around the player the same
//! way; its ungenerated frontier never shows as an asymmetric wall).

use std::io::Write as _;
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::RwLock;

/// On-disk snapshot magic + format version. CLODAT04 adds the DH vertical
/// slice table (see [`SliceCell`]); a v03 snapshot is stale by definition
/// (the scan re-runs, restoring the pack SHA-1 determinism for new bytes).
const MAGIC: &[u8; 8] = b"CLODAT04";
/// Bytes per cell in the on-disk snapshot: i16 height, u32 top, u32 side,
/// u8 flags, u16 top tile id, u16 side tile id.
const CELL_BYTES: usize = 15;
/// Bytes per cell for the DH vertical slice table (see [`SliceCell::BYTES`]).
const SLICE_BYTES: usize = crate::slices::SliceRun::BYTES * SLICE_RAW + 1;

/// Raw runs kept per cell before the DH reducer trims them for encoding.
/// The scan cuts runs on Material change from the surface DOWN, so the first
/// SLICE_RAW bands are the visually relevant ones; deeper terrain merges into
/// the last run (the reducer's force-bottom step does the same).
pub const SLICE_RAW: usize = 8;

/// The DH vertical slice table of one atlas cell: up to [`SLICE_RAW`] raw
/// runs, top-down (slice 0 = the surface band). Persisted byte-exact so the
/// static pack stays SHA-1-stable across restarts.
#[derive(Clone, Copy, Default, PartialEq, Debug)]
pub struct SliceCell {
    pub n: u8,
    pub top: [i16; SLICE_RAW],
    pub color: [u32; SLICE_RAW],
    pub tex_top: [u16; SLICE_RAW],
    pub tex_side: [u16; SLICE_RAW],
    /// Per-run flags: bit0 liquid, bit1 grass (DH FADE_TO_DIRT material).
    pub flags: [u8; SLICE_RAW],
}

impl SliceCell {
    fn runs(&self) -> Vec<crate::slices::SliceRun> {
        (0..usize::from(self.n).min(SLICE_RAW))
            .map(|i| crate::slices::SliceRun {
                top: self.top[i],
                color: self.color[i] & 0xFF_FF_FF,
                tex_top: self.tex_top[i],
                tex_side: self.tex_side[i],
                liquid: self.flags[i] & 0b01 != 0,
                grass: self.flags[i] & 0b10 != 0,
            })
            .collect()
    }

    fn from_runs(runs: &[crate::slices::SliceRun]) -> Self {
        let mut c = Self {
            n: runs.len().min(SLICE_RAW) as u8,
            ..Self::default()
        };
        for (i, r) in runs.iter().take(SLICE_RAW).enumerate() {
            c.top[i] = r.top;
            c.color[i] = r.color & 0xFF_FF_FF;
            c.tex_top[i] = r.tex_top;
            c.tex_side[i] = r.tex_side;
            c.flags[i] = u8::from(r.liquid) | (u8::from(r.grass) << 1);
        }
        c
    }
}

impl crate::slices::SliceRun {
    /// Serialized size: i16 top + u24 colour + u16 top tile + u16 side tile
    /// + u8 flags.
    pub const BYTES: usize = 2 + 3 + 2 + 2 + 1;

    fn to_bytes(self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.top.to_le_bytes());
        out.extend_from_slice(&(self.color & 0xFF_FF_FF).to_le_bytes()[..3]);
        out.extend_from_slice(&self.tex_top.to_le_bytes());
        out.extend_from_slice(&self.tex_side.to_le_bytes());
        out.push(u8::from(self.liquid) | (u8::from(self.grass) << 1));
    }

    fn from_bytes(buf: &[u8], o: usize) -> Self {
        Self {
            top: i16::from_le_bytes([buf[o], buf[o + 1]]),
            color: u32::from_le_bytes([buf[o + 2], buf[o + 3], buf[o + 4], 0]) & 0xFF_FF_FF,
            tex_top: u16::from_le_bytes([buf[o + 5], buf[o + 6]]),
            tex_side: u16::from_le_bytes([buf[o + 7], buf[o + 8]]),
            liquid: buf[o + 9] & 0b01 != 0,
            grass: buf[o + 9] & 0b10 != 0,
        }
    }
}

pub struct Atlas {
    pub center_x: i32,
    pub center_z: i32,
    pub size: i32,
    pub cell: i32,
    pub cells: u32,
    /// Kept for future y-band filtering; encoding uses it via cfg today.
    #[allow(dead_code)]
    pub sea_level: i32,
    data: RwLock<Vec<Cell>>,
    /// DH vertical slice table (parallel to `data`; every cell has an entry,
    /// defaulting to n = 0 = "no bands" — the shader falls back to the side
    /// pyramid for those cells).
    slices: RwLock<Vec<SliceCell>>,
    /// Monotonic geometry version: bumped whenever any cell changes.
    version: AtomicU64,
    /// Number of cells scanned at least once (progress tracking).
    scanned: AtomicU64,
    /// Scan sequence: row-major cell index at position `seq` of the
    /// centre-out ring order (see [`ring_order`]). Never mutated after
    /// construction, so partial scans are reproducible byte-for-byte.
    order: Vec<u32>,
}

/// Centre-out scan order over a `side x side` grid: row-major indices sorted
/// by integer Chebyshev ring distance from the grid centre, then by Euclidean
/// distance, then by the index itself. Integer arithmetic only — the order is
/// bit-identical on every platform, which keeps partial-scan pack SHA-1s
/// reproducible.
pub fn ring_order(side: u32) -> Vec<u32> {
    let c2 = side as u64 - 1; // doubled centre coordinate (half-cell units)
    let mut idx: Vec<u32> = (0..side * side).collect();
    idx.sort_unstable_by_key(|&i| {
        let ix = (i % side) as u64;
        let iz = (i / side) as u64;
        let dx = (2 * ix).abs_diff(c2);
        let dz = (2 * iz).abs_diff(c2);
        (dx.max(dz), dx * dx + dz * dz, i as u64)
    });
    idx
}

#[derive(Clone, Copy, Default, PartialEq)]
pub struct Cell {
    pub height: i16,
    pub water: bool,
    pub valid: bool,
    /// Average top-face colour (0xRRGGBB), already biome tinted.
    pub top: u32,
    /// Average side-face colour (0xRRGGBB), already biome tinted.
    pub side: u32,
    /// DH 3.3.4 textured LODs: ratio-tile ids (0 = flat) for the top and side
    /// faces of the column's top block (blocktextures::BLOCK_SETS).
    pub tex_top: u16,
    pub tex_side: u16,
}

impl Atlas {
    pub fn new(center_x: i32, center_z: i32, size: i32, cell: i32, sea_level: i32) -> Self {
        let cells = (size / cell).max(1) as u32;
        Self {
            center_x,
            center_z,
            size,
            cell,
            cells,
            sea_level,
            data: RwLock::new(vec![Cell::default(); (cells * cells) as usize]),
            slices: RwLock::new(vec![SliceCell::default(); (cells * cells) as usize]),
            version: AtomicU64::new(1),
            scanned: AtomicU64::new(0),
            order: ring_order(cells),
        }
    }

    pub fn half(&self) -> i32 {
        self.size / 2
    }

    /// Cell indices for a world block coordinate, or None when outside.
    pub fn cell_index(&self, x: i32, z: i32) -> Option<(u32, u32)> {
        let dx = x - (self.center_x - self.half());
        let dz = z - (self.center_z - self.half());
        if dx < 0 || dz < 0 || dx >= self.size || dz >= self.size {
            return None;
        }
        Some(((dx / self.cell) as u32, (dz / self.cell) as u32))
    }

    #[allow(clippy::too_many_arguments)]
    pub fn set(
        &self,
        ix: u32,
        iz: u32,
        height: i16,
        water: bool,
        top: u32,
        side: u32,
        tex_top: u16,
        tex_side: u16,
    ) {
        let idx = (iz * self.cells + ix) as usize;
        if let Ok(mut data) = self.data.write() {
            if let Some(cell) = data.get_mut(idx) {
                if !cell.valid
                    || cell.height != height
                    || cell.water != water
                    || cell.top != top
                    || cell.side != side
                    || cell.tex_top != tex_top
                    || cell.tex_side != tex_side
                {
                    if !cell.valid {
                        self.scanned.fetch_add(1, Ordering::Relaxed);
                    }
                    *cell = Cell {
                        height,
                        water,
                        valid: true,
                        top,
                        side,
                        tex_top,
                        tex_side,
                    };
                    self.version.fetch_add(1, Ordering::Relaxed);
                }
            }
        }
    }

    #[allow(dead_code)] // used by tests and upcoming y-band filters
    pub fn get(&self, ix: u32, iz: u32) -> Option<Cell> {
        let idx = (iz * self.cells + ix) as usize;
        if let Ok(d) = self.data.read() {
            return d.get(idx).copied().filter(|c| c.valid);
        }
        None
    }

    /// Store the DH vertical run list of one cell (top-down, slice 0 =
    /// surface band). Only meaningful together with [`Self::set`] for the
    /// same cell; the version counter bumps when the table changes.
    pub fn set_slices(&self, ix: u32, iz: u32, runs: &[crate::slices::SliceRun]) {
        let idx = (iz * self.cells + ix) as usize;
        let cell = SliceCell::from_runs(runs);
        if let Ok(mut data) = self.slices.write() {
            if let Some(slot) = data.get_mut(idx) {
                if *slot != cell {
                    *slot = cell;
                    self.version.fetch_add(1, Ordering::Relaxed);
                }
            }
        }
    }

    /// Copy of the raw slice table (row-major, z outer).
    pub fn slice_snapshot(&self) -> Vec<SliceCell> {
        self.slices.read().map(|d| d.clone()).unwrap_or_default()
    }

    pub fn version(&self) -> u64 {
        self.version.load(Ordering::Relaxed)
    }

    pub fn scanned_cells(&self) -> u64 {
        self.scanned.load(Ordering::Relaxed)
    }

    pub fn total_cells(&self) -> u64 {
        self.cells as u64 * self.cells as u64
    }

    /// Snapshot the grid into a flat buffer (row-major, z outer).
    pub fn snapshot(&self) -> Vec<Cell> {
        self.data.read().map(|d| d.clone()).unwrap_or_default()
    }

    /// The next `count` scan targets starting at sequence `seq`, in the
    /// centre-out ring order. Wraps over the whole atlas exactly like the
    /// previous row-major cursor did.
    pub fn scan_task(&self, seq: u64, count: u32) -> Vec<(u32, u32)> {
        let total = self.order.len() as u64;
        if total == 0 {
            return Vec::new();
        }
        let cells = self.cells;
        let mut out = Vec::with_capacity(count as usize);
        let mut s = seq % total;
        while out.len() < count as usize {
            let o = self.order[s as usize];
            out.push((o % cells, o / cells));
            s = (s + 1) % total;
            if out.len() as u64 == total {
                break;
            }
        }
        out
    }

    /// World X of cell center.
    pub fn cell_world_x(&self, ix: u32) -> i32 {
        self.center_x - self.half() + (ix * self.cell as u32) as i32 + self.cell / 2
    }

    /// World Z of cell center.
    pub fn cell_world_z(&self, iz: u32) -> i32 {
        self.center_z - self.half() + (iz * self.cell as u32) as i32 + self.cell / 2
    }

    /// Distinct side colours, sorted: the atlas stores a 1-byte index into this
    /// table (a u8 alpha channel) instead of a full colour, which keeps both
    /// per-cell colours in two RGBA textures.
    pub fn side_lut(&self) -> Vec<u32> {
        let mut v: Vec<u32> = self
            .snapshot()
            .iter()
            .filter(|c| c.valid)
            .map(|c| c.side & 0xFFFFFF)
            .collect();
        v.sort_unstable();
        v.dedup();
        v.truncate(255);
        v
    }

    fn side_index(lut: &[u32], side: u32) -> u8 {
        match lut.binary_search(&(side & 0xFFFFFF)) {
            Ok(i) => i as u8,
            Err(_) => 0,
        }
    }

    /// Height/flags image:
    ///   R = high byte, G = low byte of `(height + 64) << 6`,
    ///   B = 255 when water, A = index into [`Self::side_lut`].
    /// Unscanned cells encode height 0 and side index 255 — the reserved VOID
    /// marker (valid side indices are 0..=254 because the LUT is truncated at
    /// 255 entries). The renderer treats void exactly like Distant Horizons
    /// treats un-generated data: no geometry, no walls, the scene shows
    /// through (`drawnTo` semantics of DH's fog pass).
    pub fn encode_rgba(&self) -> (u32, u32, Vec<u8>) {
        let snap = self.snapshot();
        let lut = self.side_lut();
        let n = self.cells as usize;
        let mut px = vec![0u8; n * n * 4];
        for z in 0..n {
            for x in 0..n {
                let cell = snap[z * n + x];
                let (y16, water, side) = if cell.valid {
                    let h = (i32::from(cell.height) + 64).max(0) << 6;
                    (
                        h.min(0xFFFF) as u16,
                        cell.water,
                        Self::side_index(&lut, cell.side),
                    )
                } else {
                    (0, false, 255)
                };
                let o = (z * n + x) * 4;
                px[o] = (y16 >> 8) as u8;
                px[o + 1] = (y16 & 0xFF) as u8;
                px[o + 2] = if water { 255 } else { 0 };
                px[o + 3] = side;
            }
        }
        (self.cells, self.cells, px)
    }

    /// Child visit order for the 2x2 pyramid merge (dz, dx): DH's column
    /// vote gives tie priority to the first sampled column, so the order is
    /// fixed and scan-like.
    const PYR_CHILDREN: [(usize, usize); 4] = [(0, 0), (0, 1), (1, 0), (1, 1)];

    /// DH full-data 2x2 merge winner map (`FullDataSourceV2
    /// .mergeInputTwoByTwoDataColumn` -> `determineMostCommonValueInColumnSlice`):
    /// the parent's top slice reaches the TALLEST child's top and the vote at
    /// its middle is only carried by columns whose datapoint spans that
    /// middle — the argmax-height children (air/void always loses). Returns
    /// one layer per level >= 1; per coarse cell: the index into
    /// [`Self::PYR_CHILDREN`] of the winning child, or None when every child
    /// is void (un-scanned) — the parent is void too.
    fn pyramid_winners(snap: &[Cell], n: usize) -> Vec<Vec<Option<u8>>> {
        let mut valid: Vec<bool> = snap.iter().map(|c| c.valid).collect();
        let mut height: Vec<i32> = snap
            .iter()
            .map(|c| {
                if c.valid {
                    i32::from(c.height)
                } else {
                    i32::MIN
                }
            })
            .collect();
        let mut side = n;
        let mut all: Vec<Vec<Option<u8>>> = Vec::new();
        while side > 1 {
            let ns = side.div_ceil(2);
            let mut win = vec![None; ns * ns];
            let mut nvalid = vec![false; ns * ns];
            let mut nheight = vec![i32::MIN; ns * ns];
            for z in 0..ns {
                for x in 0..ns {
                    let mut best_h = i32::MIN;
                    let mut best_i: u8 = 0;
                    let mut any = false;
                    for (di, (dz, dx)) in Self::PYR_CHILDREN.iter().enumerate() {
                        let (sz, sx) = (z * 2 + dz, x * 2 + dx);
                        if sz < side && sx < side && valid[sz * side + sx] {
                            let h = height[sz * side + sx];
                            // strict >: the FIRST max-height child wins the
                            // tie, exactly like DH's value0 column priority
                            if !any || h > best_h {
                                any = true;
                                best_h = h;
                                best_i = di as u8;
                            }
                        }
                    }
                    if any {
                        win[z * ns + x] = Some(best_i);
                        nvalid[z * ns + x] = true;
                        nheight[z * ns + x] = best_h;
                    }
                }
            }
            all.push(win);
            valid = nvalid;
            height = nheight;
            side = ns;
        }
        all
    }

    /// Top-face colour image (RGB, alpha 255), bilinearly filtered on the GPU.
    ///
    /// Colour mip strip (the DH detail pyramid) with the DH 3.3.4 full-data
    /// merge semantics (`FullDataSourceV2.mergeInputTwoByTwoDataColumn`): the
    /// parent's top slice spans up to the TALLEST child's top, and the id
    /// voted at that slice's middle is carried only by columns whose top
    /// reaches it — the argmax-height children (air/void always loses the
    /// vote). So every level: height = max over the valid 2x2, colour = the
    /// winning column's colour (ties -> first child in scan order). This is
    /// why DH's far LODs keep pure surface colours (a lone tree on a plain
    /// stays green-dark instead of muddying the field) — and why far water
    /// yields to adjacent high land exactly like DH.
    ///
    /// All levels are stacked vertically into ONE `n x (2n-1)` image:
    ///   level L occupies rows `[2n - 2*(n>>L), 2n - (n>>L))`.
    /// The shader picks the octave from the pixel's world footprint, so the
    /// far band samples aggregated datapoints instead of point-sampling the
    /// 2-block mosaic (that point sample was the "noisy pixels").
    pub fn encode_colors_pyramid(&self) -> (u32, u32, Vec<u8>) {
        let snap = self.snapshot();
        let n = self.cells as usize;
        let winners = Self::pyramid_winners(&snap, n);
        // Level 0 = the per-cell colour; every next level takes the winning
        // child's colour of the level below (DH's voted datapoint id).
        let mut cur: Vec<Option<[u8; 3]>> = snap
            .iter()
            .map(|c| {
                if c.valid {
                    Some([
                        ((c.top >> 16) & 0xFF) as u8,
                        ((c.top >> 8) & 0xFF) as u8,
                        (c.top & 0xFF) as u8,
                    ])
                } else {
                    None
                }
            })
            .collect();
        let mut levels = vec![cur.clone()];
        let mut heights = vec![n];
        let mut side = n;
        for win in &winners {
            let ns = side.div_ceil(2);
            let mut next = vec![None; ns * ns];
            for z in 0..ns {
                for x in 0..ns {
                    if let Some(di) = win[z * ns + x] {
                        let (dz, dx) = Self::PYR_CHILDREN[di as usize];
                        next[z * ns + x] = cur[(z * 2 + dz) * side + (x * 2 + dx)];
                    }
                }
            }
            levels.push(next.clone());
            heights.push(ns);
            cur = next;
            side = ns;
        }
        // Stack into the strip: each level keeps its tile layout (stride
        // `cells`), so its row band is exactly its tile side tall. Odd sizes
        // round up; the pad columns stay black and are never sampled because
        // the shader clamps to the tile bounds.
        let strip_h: usize = heights.iter().sum();
        let mut px = vec![0u8; n * strip_h * 4];
        let mut row = 0usize;
        for (level, &lh) in levels.iter().zip(&heights) {
            for (i, c) in level.iter().enumerate() {
                if let Some([r, g, b]) = c {
                    let (x, z) = (i % lh, i / lh);
                    let o = ((row + z) * n + x) * 4;
                    px[o] = *r;
                    px[o + 1] = *g;
                    px[o + 2] = *b;
                    px[o + 3] = 255;
                }
            }
            row += lh;
        }
        (self.cells, strip_h as u32, px)
    }

    /// Side-index strip for the DH detail pyramid — the same winner rule as
    /// [`Self::encode_colors_pyramid`]: every coarse cell carries the SIDE
    /// INDEX OF THE TALLEST column (DH renders a datapoint's walls in the
    /// datapoint's own material; the wall colour follows the top-slice
    /// winner, not a vote across all children). Same `n x (2n-1)` strip
    /// layout; R = index, nearest-filtered on the GPU (indices must never
    /// blend — a linear mix of two LUT keys is a third, wrong colour).
    pub fn encode_side_pyramid(&self) -> (u32, u32, Vec<u8>) {
        let snap = self.snapshot();
        let lut = self.side_lut();
        let n = self.cells as usize;
        let winners = Self::pyramid_winners(&snap, n);
        let mut cur: Vec<Option<u8>> = snap
            .iter()
            .map(|c| {
                if c.valid {
                    Some(Self::side_index(&lut, c.side))
                } else {
                    None
                }
            })
            .collect();
        let mut levels = vec![cur.clone()];
        let mut heights = vec![n];
        let mut side = n;
        for win in &winners {
            let ns = side.div_ceil(2);
            let mut next = vec![None; ns * ns];
            for z in 0..ns {
                for x in 0..ns {
                    if let Some(di) = win[z * ns + x] {
                        let (dz, dx) = Self::PYR_CHILDREN[di as usize];
                        next[z * ns + x] = cur[(z * 2 + dz) * side + (x * 2 + dx)];
                    }
                }
            }
            levels.push(next.clone());
            heights.push(ns);
            cur = next;
            side = ns;
        }
        let strip_h: usize = heights.iter().sum();
        let mut px = vec![0u8; n * strip_h * 4];
        let mut row = 0usize;
        for (level, &lh) in levels.iter().zip(&heights) {
            for (i, c) in level.iter().enumerate() {
                if let Some(idx) = c {
                    let (x, z) = (i % lh, i / lh);
                    let o = ((row + z) * n + x) * 4;
                    px[o] = *idx;
                    px[o + 3] = 255;
                }
            }
            row += lh;
        }
        (self.cells, strip_h as u32, px)
    }

    // ---------------------------------------------------------------------
    // DH vertical slices (analysis/dh-lod-pipeline.md §1/§2/§9.1, ported in
    // slices.rs): the banded cliff walls of Distant Horizons. Raw level-0
    // run lists live in the SliceCell table; every octave up merges 2x2
    // with the full-data vote rule; each octave encodes the list reduced to
    // EDhApiVerticalQuality.MEDIUM[detail] (detail = log2(cell) + L).
    //
    // Strip layout (width 2n, slices' texel A|B pairs side by side):
    //   octave L occupies rows [rowBase(L), rowBase(L) + K(L) * (n>>L)),
    //   cell (x,z) slice j: texel A at column 2x, texel B at column 2x+1,
    //   row = rowBase(L) + z*K(L) + j.
    //   texel A: RGB = run colour, A = high byte of (top+64)<<6.
    //   texel B: R = low byte, G/B = side tile id big-endian,
    //            A = flags (bit0 present, bit1 liquid, bit2 grass).
    // The shader recomputes rowBase from the same MEDIUM table, so the
    // layout must stay in sync with lod_terrain.fsh.
    // ---------------------------------------------------------------------

    /// GL-safe strip height budget: llvmpipe reports max texture sizes well
    /// above this, but 8192 is the portable floor (GLES-class GPUs); octaves
    /// that would exceed it keep the single-side-colour wall path (the
    /// SidePyr fallback), exactly like the old renderer.
    pub const SLICE_STRIP_MAX_H: usize = 8192;

    /// (octaves, max_octave_encoded, K per octave, rowBase per octave, H)
    fn slice_layout(&self) -> (usize, usize, Vec<usize>, Vec<usize>, usize) {
        let n = self.cells as usize;
        let levels = if n > 0 { n.ilog2() as usize + 1 } else { 1 };
        let detail0 = self.cell.trailing_zeros() as i64; // log2(cell_size)
        let mut k = Vec::with_capacity(levels);
        let mut row_base = Vec::with_capacity(levels);
        let mut base = 0usize;
        for (l, _i) in (0..levels).enumerate() {
            let kl = crate::slices::slices_at_detail(detail0 + l as i64);
            k.push(kl);
            row_base.push(base);
            base += kl * (n >> l);
        }
        // Largest octave whose cumulative height fits the strip budget.
        let mut max_oct = 0usize;
        let mut cum = 0usize;
        for (l, kl) in k.iter().enumerate() {
            cum += kl * (n >> l);
            if cum <= Self::SLICE_STRIP_MAX_H {
                max_oct = l;
            } else {
                break;
            }
        }
        let h = row_base[max_oct] + k[max_oct] * (n >> max_oct);
        (levels, max_oct, k, row_base, h)
    }

    /// Encode the DH slice strip: (width 2n, height, pixels, max octave).
    /// Octaves above `max octave` are left unwritten — the shader falls back
    /// to the side pyramid there (big-atlas configs only; the default 1024
    /// atlas fits every octave).
    pub fn encode_slices(&self) -> (u32, u32, Vec<u8>, u32) {
        let slice_snap = self.slice_snapshot();
        let n = self.cells as usize;
        let (levels, max_oct, k, row_base, h) = self.slice_layout();
        let mut px = vec![0u8; 2 * n * h * 4];

        if n == 0 || h == 0 {
            return (self.cells, h as u32, px, max_oct as u32);
        }

        // Level-0 raw lists (top-down; cells without a slice table are
        // empty — DH's un-generated columns).
        let mut cur: Vec<Vec<crate::slices::SliceRun>> =
            slice_snap.iter().map(|sc| sc.runs()).collect();

        let put = |px: &mut [u8], col: usize, row: usize, rgba: [u8; 4]| {
            let o = (row * 2 * n + col) * 4;
            if o + 3 < px.len() {
                px[o..o + 4].copy_from_slice(&rgba);
            }
        };

        for level in 0..levels {
            if level > 0 {
                // 2x2 vote merge (FullDataSourceV2) + bound the raw list.
                let side = n >> (level - 1);
                let ns = side.div_ceil(2);
                let mut next = vec![Vec::new(); ns * ns];
                for z in 0..ns {
                    for x in 0..ns {
                        let g = |dz: usize, dx: usize| -> &[crate::slices::SliceRun] {
                            let (sz, sx) = (z * 2 + dz, x * 2 + dx);
                            if sz < side && sx < side {
                                &cur[sz * side + sx]
                            } else {
                                &[]
                            }
                        };
                        let merged =
                            crate::slices::merge_two_by_two([g(0, 0), g(0, 1), g(1, 0), g(1, 1)]);
                        next[z * ns + x] = crate::slices::reduce(&merged, crate::slices::RAW_CAP);
                    }
                }
                cur = next;
            }

            if level > max_oct {
                // Data still merged (a parent needs merged children), but no
                // strip space: stop writing, keep merging.
                continue;
            }

            let kl = k[level];
            for (i, list) in cur.iter().enumerate() {
                if list.is_empty() {
                    continue;
                }
                let enc = crate::slices::reduce(list, kl);
                let x = i % (n >> level);
                let z = i / (n >> level);
                for (j, r) in enc.iter().enumerate() {
                    if j >= kl {
                        break;
                    }
                    let row = row_base[level] + z * kl + j;
                    let y16 = ((i32::from(r.top) + 64).max(0) << 6).min(0xFFFF) as u16;
                    let a = (y16 >> 8) as u8;
                    let flags = 0b001 | (u8::from(r.liquid) << 1) | (u8::from(r.grass) << 2);
                    put(
                        &mut px,
                        2 * x,
                        row,
                        [
                            ((r.color >> 16) & 0xFF) as u8,
                            ((r.color >> 8) & 0xFF) as u8,
                            (r.color & 0xFF) as u8,
                            a,
                        ],
                    );
                    put(
                        &mut px,
                        2 * x + 1,
                        row,
                        [
                            (y16 & 0xFF) as u8,
                            (r.tex_side >> 8) as u8,
                            (r.tex_side & 0xFF) as u8,
                            flags,
                        ],
                    );
                }
            }
        }

        (2 * self.cells, h as u32, px, max_oct as u32)
    }

    /// Per-cell ratio-tile ids (DH 3.3.4 textured LODs): R/G = top tile id
    /// big-endian, B/A = side tile id big-endian. Nearest-filtered on the GPU
    /// so ids never blend between cells. Tile id 0 = the reserved flat tile.
    pub fn encode_tiles(&self) -> (u32, u32, Vec<u8>) {
        let snap = self.snapshot();
        let n = self.cells as usize;
        let mut px = vec![0u8; n * n * 4];
        for z in 0..n {
            for x in 0..n {
                let cell = snap[z * n + x];
                if cell.valid {
                    let o = (z * n + x) * 4;
                    px[o] = (cell.tex_top >> 8) as u8;
                    px[o + 1] = (cell.tex_top & 0xFF) as u8;
                    px[o + 2] = (cell.tex_side >> 8) as u8;
                    px[o + 3] = (cell.tex_side & 0xFF) as u8;
                }
            }
        }
        (self.cells, self.cells, px)
    }

    // -----------------------------------------------------------------------
    // Disk persistence (static-pack determinism across restarts).
    // -----------------------------------------------------------------------

    /// Serialize the atlas snapshot: MAGIC + header + 15 bytes per cell
    /// (height i16 LE, top u32, side u32, flags: bit0 water, bit1 valid),
    /// then the DH slice table: 1 + 8*10 bytes per cell.
    pub fn to_bytes(&self) -> Vec<u8> {
        let snap = self.snapshot();
        let slice_snap = self.slice_snapshot();
        let n = snap.len();
        let mut out = Vec::with_capacity(MAGIC.len() + 20 + n * (CELL_BYTES + SLICE_BYTES));
        out.extend_from_slice(MAGIC);
        out.extend_from_slice(&self.center_x.to_le_bytes());
        out.extend_from_slice(&self.center_z.to_le_bytes());
        out.extend_from_slice(&self.size.to_le_bytes());
        out.extend_from_slice(&self.cell.to_le_bytes());
        out.extend_from_slice(&self.sea_level.to_le_bytes());
        // Blocked layout (what from_bytes reads): [all cells][all slices].
        for c in &snap {
            out.extend_from_slice(&c.height.to_le_bytes());
            out.extend_from_slice(&(c.top & 0xFFFFFF).to_le_bytes());
            out.extend_from_slice(&(c.side & 0xFFFFFF).to_le_bytes());
            out.push((u8::from(c.water)) | (u8::from(c.valid) << 1));
            out.extend_from_slice(&c.tex_top.to_le_bytes());
            out.extend_from_slice(&c.tex_side.to_le_bytes());
        }
        for sc in &slice_snap {
            out.push(sc.n);
            for i in 0..SLICE_RAW {
                crate::slices::SliceRun {
                    top: sc.top[i],
                    color: sc.color[i],
                    tex_top: sc.tex_top[i],
                    tex_side: sc.tex_side[i],
                    liquid: sc.flags[i] & 0b01 != 0,
                    grass: sc.flags[i] & 0b10 != 0,
                }
                .to_bytes(&mut out);
            }
        }
        out
    }

    /// Parse a snapshot produced by [`Self::to_bytes`]. Returns None on any
    /// header/geometry mismatch (stale cache for another world or config).
    pub fn from_bytes(
        buf: &[u8],
        want_center_x: i32,
        want_center_z: i32,
        want_size: i32,
        want_cell: i32,
        want_sea_level: i32,
    ) -> Option<Atlas> {
        if buf.len() < MAGIC.len() + 20 || &buf[..8] != MAGIC {
            return None;
        }
        let rd_i32 = |o: usize| -> Option<i32> {
            Some(i32::from_le_bytes(buf.get(o..o + 4)?.try_into().ok()?))
        };
        let cx = rd_i32(8)?;
        let cz = rd_i32(12)?;
        let size = rd_i32(16)?;
        let cell = rd_i32(20)?;
        let sea = rd_i32(24)?;
        if cx != want_center_x
            || cz != want_center_z
            || size != want_size
            || cell != want_cell
            || sea != want_sea_level
        {
            return None;
        }
        let atlas = Atlas::new(cx, cz, size, cell, sea);
        let n = (atlas.cells as usize) * (atlas.cells as usize);
        if buf.len() < MAGIC.len() + 20 + n * (CELL_BYTES + SLICE_BYTES) {
            return None;
        }
        let mut data = atlas.data.write().ok()?;
        let mut slices = atlas.slices.write().ok()?;
        let base = MAGIC.len() + 20;
        let mut scanned = 0u64;
        for (i, cell_out) in data.iter_mut().enumerate() {
            let o = base + i * CELL_BYTES;
            let height = i16::from_le_bytes([buf[o], buf[o + 1]]);
            let top = u32::from_le_bytes([buf[o + 2], buf[o + 3], buf[o + 4], 0]) & 0xFFFFFF;
            let side = u32::from_le_bytes([buf[o + 6], buf[o + 7], buf[o + 8], 0]) & 0xFFFFFF;
            let flags = buf[o + 10];
            let tex_top = u16::from_le_bytes([buf[o + 11], buf[o + 12]]);
            let tex_side = u16::from_le_bytes([buf[o + 13], buf[o + 14]]);
            let so = base + n * CELL_BYTES + i * SLICE_BYTES;
            let sc = SliceCell {
                n: buf[so].min(SLICE_RAW as u8),
                ..SliceCell::default()
            };
            let mut sc = sc;
            for j in 0..SLICE_RAW {
                let r = crate::slices::SliceRun::from_bytes(
                    buf,
                    so + 1 + j * crate::slices::SliceRun::BYTES,
                );
                sc.top[j] = r.top;
                sc.color[j] = r.color;
                sc.tex_top[j] = r.tex_top;
                sc.tex_side[j] = r.tex_side;
                sc.flags[j] = u8::from(r.liquid) | (u8::from(r.grass) << 1);
            }
            slices[i] = sc;
            if flags & 0b10 != 0 {
                *cell_out = Cell {
                    height,
                    water: flags & 0b01 != 0,
                    valid: true,
                    top,
                    side,
                    tex_top,
                    tex_side,
                };
                scanned += 1;
            }
        }
        drop(data);
        drop(slices);
        atlas.scanned.store(scanned, Ordering::Relaxed);
        Some(atlas)
    }

    /// Atomically persist the snapshot (tmp file + rename).
    pub fn save(&self, path: &Path) -> std::io::Result<()> {
        let bytes = self.to_bytes();
        let tmp = path.with_extension("tmp");
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent)?;
            }
        }
        {
            let mut f = std::fs::File::create(&tmp)?;
            f.write_all(&bytes)?;
            f.sync_all()?;
        }
        std::fs::rename(&tmp, path)
    }

    /// Load a snapshot if it matches the expected geometry, else None.
    pub fn try_restore(
        path: &Path,
        center_x: i32,
        center_z: i32,
        size: i32,
        cell: i32,
        sea_level: i32,
    ) -> Option<Atlas> {
        let buf = std::fs::read(path).ok()?;
        Self::from_bytes(&buf, center_x, center_z, size, cell, sea_level)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn indices_and_bounds() {
        let a = Atlas::new(0, 0, 1024, 4, 63);
        assert_eq!(a.cells, 256);
        assert_eq!(a.half(), 512);
        assert_eq!(a.cell_index(0, 0), Some((128, 128)));
        assert_eq!(a.cell_index(-512, -512), Some((0, 0)));
        assert_eq!(a.cell_index(511, 511), Some((255, 255)));
        assert_eq!(a.cell_index(512, 0), None);
        assert_eq!(a.cell_index(-513, 0), None);
    }

    #[test]
    fn set_get_encode() {
        let a = Atlas::new(0, 0, 64, 4, 63);
        assert_eq!(a.cells, 16);
        a.set(3, 5, 100, false, 0x808080, 0x707070, 33, 281);
        a.set(0, 0, 62, true, 0x808080, 0x707070, 0, 0);
        assert_eq!(a.get(3, 5).unwrap().height, 100);
        assert_eq!(a.get(3, 5).unwrap().tex_top, 33);
        assert_eq!(a.get(3, 5).unwrap().tex_side, 281);
        assert!(a.get(0, 0).unwrap().water);
        assert!(a.get(1, 1).is_none());

        let (w, h, px) = a.encode_rgba();
        assert_eq!((w, h), (16, 16));
        // height 100 -> (100+64)<<6 = 10496 = 0x2900
        let o = (5 * 16 + 3) * 4;
        assert_eq!(px[o], 0x29);
        assert_eq!(px[o + 1], 0x00);
        // water cell at (0,0)
        let o2 = 0;
        assert_eq!(px[o2], ((62 + 64) << 6 >> 8) as u8);
        assert_eq!(px[o2 + 2], 255);
        // invalid -> height 0, side byte 255 (the reserved VOID marker)
        let o3 = (7 * 16 + 7) * 4;
        assert_eq!(px[o3], 0);
        assert_eq!(px[o3 + 2], 0);
        assert_eq!(px[o3 + 3], 255);

        // tile ids ride a separate nearest-filtered image, big-endian pairs
        let (tw, th, tpx) = a.encode_tiles();
        assert_eq!((tw, th), (16, 16));
        let t = (5 * 16 + 3) * 4;
        assert_eq!(tpx[t], 0);
        assert_eq!(tpx[t + 1], 33);
        assert_eq!(tpx[t + 2], (281u16 >> 8) as u8);
        assert_eq!(tpx[t + 3], (281u16 & 0xFF) as u8);
        // invalid cells stay 0/0/0/0 (both tiles flat)
        assert_eq!(&tpx[(7 * 16 + 7) * 4..(7 * 16 + 7) * 4 + 4], &[0, 0, 0, 0]);
    }

    #[test]
    fn color_pyramid_takes_the_tallest_column() {
        let a = Atlas::new(0, 0, 64, 4, 63); // 16x16 cells
                                             // One 2x2 corner: three equal-height fields and one TALLER column.
                                             // DH's 2x2 merge votes the parent id at the top slice's middle, which
                                             // only the argmax-height columns reach — the coarse colour is the tall
                                             // column's own colour, never an average with the lower fields.
        a.set(0, 0, 10, false, 0xFF0000, 0x707070, 0, 0);
        a.set(1, 0, 10, false, 0x00FF00, 0x707070, 0, 0);
        a.set(0, 1, 10, false, 0x0000FF, 0x707070, 0, 0);
        a.set(1, 1, 20, false, 0x123456, 0x707070, 0, 0);

        let (w, h, px) = a.encode_colors_pyramid();
        assert_eq!((w, h), (16, 31)); // 16+8+4+2+1 strip
                                      // byte offset of the texel at (strip row, tile z, tile x)
        let at = |row: usize, z: usize, x: usize| ((row + z) * 16 + x) * 4;
        // level 0 rows [0,16): (0,0) is pure red
        assert_eq!(&px[at(0, 0, 0)..at(0, 0, 0) + 3], &[255, 0, 0]);
        // the invalid cell (7,7) stays black in level 0
        assert_eq!(&px[at(0, 7, 7)..at(0, 7, 7) + 3], &[0, 0, 0]);
        // level 1 rows [16,24): cell (0,0) is the TALL column (h20 beats h10)
        assert_eq!(&px[at(16, 0, 0)..at(16, 0, 0) + 3], &[0x12, 0x34, 0x56]);
        // top level row 30 (1x1): the tallest column of the whole atlas
        assert_eq!(&px[at(30, 0, 0)..at(30, 0, 0) + 3], &[0x12, 0x34, 0x56]);
        // alpha is opaque padding on every WRITTEN texel (RGB-only strip;
        // pad columns/rows stay zero and are never sampled — the shader
        // clamps to the tile bounds)
        assert_eq!(px[at(0, 0, 0) + 3], 255);
        assert_eq!(px[at(16, 0, 0) + 3], 255);
    }

    #[test]
    fn pyramid_tie_keeps_the_first_column_and_void_yields() {
        let a = Atlas::new(0, 0, 64, 4, 63); // 16x16 cells
                                             // Tie: three equal-height cells with distinct colours + one invalid;
                                             // the coarse colour is the FIRST valid child in scan order ((0,0)) —
                                             // DH's value0 column priority — and a partly-void 2x2 still yields a
                                             // colour (air never wins the vote).
        a.set(0, 0, 10, false, 0xFF0000, 0x707070, 0, 0);
        a.set(1, 0, 10, false, 0x00FF00, 0x707070, 0, 0);
        a.set(0, 1, 10, false, 0x0000FF, 0x707070, 0, 0);

        let (w, h, px) = a.encode_colors_pyramid();
        assert_eq!((w, h), (16, 31));
        let at = |row: usize, z: usize, x: usize| ((row + z) * 16 + x) * 4;
        assert_eq!(&px[at(16, 0, 0)..at(16, 0, 0) + 3], &[255, 0, 0]);
        // the whole-atlas top level follows the same winner chain
        assert_eq!(&px[at(30, 0, 0)..at(30, 0, 0) + 3], &[255, 0, 0]);
    }

    #[test]
    fn side_pyramid_follows_the_winner() {
        let a = Atlas::new(0, 0, 64, 4, 63); // 16x16 cells
                                             // The coarse side index is the TALLEST column's index (DH renders a
                                             // datapoint's walls in its own material) — here the tall column is
                                             // also the index minority, proving winner-over-mode.
        a.set(0, 0, 10, false, 0x00FF00, 0x101010, 0, 0);
        a.set(1, 0, 10, false, 0x00FF00, 0x101010, 0, 0);
        a.set(0, 1, 10, false, 0x00FF00, 0x101010, 0, 0);
        a.set(1, 1, 20, false, 0x00FF00, 0x202020, 0, 0);
        let lut = a.side_lut();
        let i10 = super::Atlas::side_index(&lut, 0x101010);
        let i20 = super::Atlas::side_index(&lut, 0x202020);
        assert_ne!(i10, i20);

        let (w, h, px) = a.encode_side_pyramid();
        assert_eq!((w, h), (16, 31));
        let at = |row: usize, z: usize, x: usize| ((row + z) * 16 + x) * 4;
        // level 0 keeps the raw indices
        assert_eq!(px[at(0, 0, 0)], i10);
        // level 1 (row 16): winner = the tall (1,1) column -> i20 (not mode i10)
        assert_eq!(px[at(16, 0, 0)], i20);
        // unwritten pad texels stay zero (never sampled)
        assert_eq!(px[at(16, 7, 7)], 0);
    }

    #[test]
    fn slice_strip_layout_and_pyramid() {
        let a = Atlas::new(0, 0, 64, 4, 63); // 16x16 cells, cell 4 (detail0=2)
        let grass = crate::slices::SliceRun {
            top: 10,
            color: 0x00FF00,
            tex_top: 7,
            tex_side: 9,
            liquid: false,
            grass: true,
        };
        let dirt = crate::slices::SliceRun {
            top: 8,
            color: 0x8B4513,
            ..grass
        };
        let stone = crate::slices::SliceRun {
            top: 0,
            color: 0x808080,
            ..grass
        };
        a.set(3, 5, 11, false, 0x00FF00, 0x707070, 7, 9);
        a.set_slices(3, 5, &[grass, dirt, stone]);

        let (w, h, px, max_oct) = a.encode_slices();
        assert_eq!(w, 32, "two texels per cell column");
        // cell=4 -> detail0 = 2 -> K = MEDIUM[2] = 6; n=16 -> 5 octaves:
        // 6*16 + 4*8 + 4*4 + 4*2 + 4*1 = 156 rows, all under the 8192 cap.
        assert_eq!(h, 156);
        assert_eq!(max_oct, 4);
        let at = |col: usize, row: usize| (row * 32 + col) * 4;
        // Octave 0, cell (3,5), slice 0 (grass): row = 0 + 5*6 + 0 = 30.
        assert_eq!(&px[at(6, 30)..at(6, 30) + 3], &[0, 255, 0]);
        assert_eq!(px[at(6, 30) + 3], ((10 + 64) << 6 >> 8) as u8, "top hi");
        assert_eq!(px[at(7, 30)], ((10 + 64) << 6 & 0xFF) as u8, "top lo");
        assert_eq!(px[at(7, 30) + 1], 0, "side tile hi");
        assert_eq!(px[at(7, 30) + 2], 9, "side tile lo");
        assert_eq!(px[at(7, 30) + 3] & 0b001, 1, "present");
        assert_eq!(px[at(7, 30) + 3] >> 2, 1, "grass flag");
        // slice 1 (dirt): row 31.
        assert_eq!(&px[at(6, 31)..at(6, 31) + 3], &[0x8B, 0x45, 0x13]);
        // slice 2 (stone): row 32.
        assert_eq!(&px[at(6, 32)..at(6, 32) + 3], &[0x80, 0x80, 0x80]);
        // absent slices stay zero (never sampled).
        assert_eq!(px[at(7, 33) + 3] & 1, 0);
        // a cell without slices stays absent.
        assert_eq!(px[at(2 * 8, 0) + 3] & 1, 0);

        // Octave 1 (rowBase 96): cell (1,2) covers children (2..4, 4..6);
        // only (3,5) has data -> merged list = that list; encoded at
        // row 96 + 2*4 + 0 with K1 = MEDIUM[3] = 4.
        assert_eq!(&px[at(2, 104)..at(2, 104) + 3], &[0, 255, 0]);
        assert_eq!(&px[at(2, 105)..at(2, 105) + 3], &[0x8B, 0x45, 0x13]);
        assert_eq!(&px[at(2, 106)..at(2, 106) + 3], &[0x80, 0x80, 0x80]);
    }

    #[test]
    fn slice_snapshot_roundtrip() {
        let dir = std::env::temp_dir().join(format!("clod-slc-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("atlas.bin");
        let a = Atlas::new(11, -22, 64, 4, 63);
        let grass = crate::slices::SliceRun {
            top: 40,
            color: 0x12_34_56,
            tex_top: 300,
            tex_side: 301,
            liquid: false,
            grass: true,
        };
        let water = crate::slices::SliceRun {
            top: 62,
            color: 0x3F76E4,
            liquid: true,
            ..grass
        };
        a.set(2, 3, 41, false, 0x123456, 0x654321, 300, 301);
        a.set_slices(2, 3, &[water, grass]);
        a.save(&path).unwrap();
        let b = Atlas::try_restore(&path, 11, -22, 64, 4, 63).expect("restore");
        assert_eq!(b.slice_snapshot(), a.slice_snapshot());
        // determinism: encoding matches after restore
        let (_, _, pa, ma) = a.encode_slices();
        let (_, _, pb, mb) = b.encode_slices();
        assert_eq!(pa, pb);
        assert_eq!(ma, mb);
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn version_bumps_on_slice_change() {
        let a = Atlas::new(0, 0, 64, 4, 63);
        let v0 = a.version();
        let r = crate::slices::SliceRun {
            top: 10,
            color: 1,
            tex_top: 0,
            tex_side: 0,
            liquid: false,
            grass: false,
        };
        a.set_slices(1, 1, &[r]);
        let v1 = a.version();
        a.set_slices(1, 1, &[r]); // same value — no bump
        assert_eq!(v0 + 1, v1);
        assert_eq!(a.version(), v1);
    }

    #[test]
    fn version_bumps_on_change() {
        let a = Atlas::new(0, 0, 64, 4, 63);
        let v0 = a.version();
        a.set(1, 1, 10, false, 0x808080, 0x707070, 0, 0);
        let v1 = a.version();
        a.set(1, 1, 10, false, 0x808080, 0x707070, 0, 0); // same value — no bump
        assert_eq!(v0 + 1, v1);
        assert_eq!(a.version(), v1);
        assert_eq!(a.scanned_cells(), 1);
    }

    #[test]
    fn scan_task_wraps() {
        let a = Atlas::new(0, 0, 64, 4, 63); // 16x16 cells
                                             // Asking for more than `total` yields exactly `total` unique cells.
        let t = a.scan_task(0, 257);
        assert_eq!(t.len(), 256);
        let mut uniq = t.clone();
        uniq.sort_unstable();
        uniq.dedup();
        assert_eq!(uniq.len(), 256, "full sweep covers every cell once");
        // The ring order decides WHERE the sweep starts; wrap continuity is
        // what this test pins down: seq=total-1 is followed by seq=0.
        let expected_last = {
            let o = ring_order(16);
            let l = o[255];
            (l % 16, l / 16)
        };
        let t2 = a.scan_task(255, 4);
        assert_eq!(t2[0], expected_last);
        assert_eq!(t2[1], t[0], "sweep wraps to the first ring cell");
    }

    #[test]
    fn save_restore_roundtrip() {
        let dir = std::env::temp_dir().join(format!("clod-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("atlas.bin");

        let a = Atlas::new(123, -456, 64, 4, 62);
        a.set(3, 5, 100, false, 0x808080, 0x707070, 12, 34);
        a.set(0, 0, 62, true, 0x808080, 0x707070, 0, 0);
        a.set(15, 15, 200, false, 0x808080, 0x707070, 0, 0);
        a.save(&path).unwrap();
        // tmp file must be gone after atomic rename
        assert!(!path.with_extension("tmp").exists());

        let b = Atlas::try_restore(&path, 123, -456, 64, 4, 62).expect("restore");
        assert_eq!(b.center_x, 123);
        assert_eq!(b.center_z, -456);
        assert_eq!(b.scanned_cells(), 3);
        assert_eq!(b.get(3, 5).unwrap().height, 100);
        assert_eq!(b.get(3, 5).unwrap().tex_top, 12);
        assert_eq!(b.get(3, 5).unwrap().tex_side, 34);
        assert!(b.get(0, 0).unwrap().water);
        assert_eq!(b.get(15, 15).unwrap().height, 200);

        // Deterministic encoding across instances (pack SHA-1 stability).
        let (_, _, px_a) = a.encode_rgba();
        let (_, _, px_b) = b.encode_rgba();
        assert_eq!(px_a, px_b);

        // Geometry mismatch -> no restore (stale cache).
        assert!(Atlas::try_restore(&path, 999, -456, 64, 4, 62).is_none());
        assert!(Atlas::try_restore(&path, 123, -456, 128, 4, 62).is_none());
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn from_bytes_rejects_garbage() {
        assert!(Atlas::from_bytes(b"", 0, 0, 64, 4, 63).is_none());
        assert!(Atlas::from_bytes(b"NOTMAGIxxxx", 0, 0, 64, 4, 63).is_none());
        let a = Atlas::new(0, 0, 64, 4, 63);
        let bytes = a.to_bytes();
        let mut truncated = bytes.clone();
        truncated.truncate(bytes.len() - 7);
        assert!(Atlas::from_bytes(&truncated, 0, 0, 64, 4, 63).is_none());
    }

    #[test]
    fn ring_order_is_a_bijection() {
        for side in [1u32, 2, 3, 4, 8, 64, 256] {
            let o = ring_order(side);
            let n = (side as u64) * (side as u64);
            assert_eq!(o.len() as u64, n);
            let mut seen = o.clone();
            seen.sort_unstable();
            seen.dedup();
            assert_eq!(seen.len() as u64, n, "side {side}: duplicates in order");
            assert_eq!(seen.last().copied(), Some((n - 1) as u32));
        }
    }

    #[test]
    fn ring_order_is_monotone_in_chebyshev_rings() {
        // Chebyshev distance (half-cell units) must never decrease along the
        // sequence — that is the whole point of the centre-out order.
        for side in [2u32, 3, 4, 8, 64] {
            let o = ring_order(side);
            let c2 = side as u64 - 1;
            let mut prev = 0u64;
            for &i in &o {
                let dx = (2 * (i % side) as u64).abs_diff(c2);
                let dz = (2 * (i / side) as u64).abs_diff(c2);
                let cheb = dx.max(dz);
                assert!(
                    cheb >= prev,
                    "side {side}: cheb {cheb} after {prev} at idx {i}"
                );
                prev = cheb;
            }
        }
    }

    #[test]
    fn ring_order_starts_at_the_centre() {
        // Odd side: exactly the middle cell first. Even side: the first four
        // entries are the 2x2 centre block.
        assert_eq!(ring_order(5)[0], 2 * 5 + 2);
        let first4: std::collections::BTreeSet<u32> = ring_order(4)[..4].iter().copied().collect();
        assert_eq!(
            first4,
            [4 + 1, 4 + 2, 8 + 1, 8 + 2].into_iter().collect(),
            "even grid must start with its 2x2 centre"
        );
    }

    #[test]
    fn scan_task_follows_ring_order() {
        let a = Atlas::new(0, 0, 64, 4, 63); // 16x16 cells
        let t = a.scan_task(0, 4);
        assert_eq!(
            t[0],
            (7, 7),
            "scan starts at the 2x2 centre (index tie-break)"
        );
        // Ring growth: Chebyshev distance (half-cell units) never decreases.
        let all = a.scan_task(0, 256);
        assert_eq!(all.len(), 256);
        let c2 = 15u64; // doubled centre in half-cells
        let mut prev = 0u64;
        for &(ix, iz) in &all {
            let dx = (2 * ix as u64).abs_diff(c2);
            let dz = (2 * iz as u64).abs_diff(c2);
            let cheb = dx.max(dz);
            assert!(cheb >= prev, "ring shrank at ({ix},{iz})");
            prev = cheb;
        }
    }
}
