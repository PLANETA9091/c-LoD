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

/// On-disk snapshot magic + format version.
const MAGIC: &[u8; 8] = b"CLODAT02";
/// Bytes per cell in the on-disk snapshot: i16 height, u32 top, u32 side,
/// u8 flags.
const CELL_BYTES: usize = 11;

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

    pub fn set(&self, ix: u32, iz: u32, height: i16, water: bool, top: u32, side: u32) {
        let idx = (iz * self.cells + ix) as usize;
        if let Ok(mut data) = self.data.write() {
            if let Some(cell) = data.get_mut(idx) {
                if !cell.valid
                    || cell.height != height
                    || cell.water != water
                    || cell.top != top
                    || cell.side != side
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

    /// Top-face colour image (RGB, alpha 255), bilinearly filtered on the GPU.
    pub fn encode_colors(&self) -> (u32, u32, Vec<u8>) {
        let snap = self.snapshot();
        let n = self.cells as usize;
        let mut px = vec![0u8; n * n * 4];
        for z in 0..n {
            for x in 0..n {
                let cell = snap[z * n + x];
                let o = (z * n + x) * 4;
                if cell.valid {
                    px[o] = ((cell.top >> 16) & 0xFF) as u8;
                    px[o + 1] = ((cell.top >> 8) & 0xFF) as u8;
                    px[o + 2] = (cell.top & 0xFF) as u8;
                }
                px[o + 3] = 255;
            }
        }
        (self.cells, self.cells, px)
    }

    // -----------------------------------------------------------------------
    // Disk persistence (static-pack determinism across restarts).
    // -----------------------------------------------------------------------

    /// Serialize the atlas snapshot: MAGIC + header + 11 bytes per cell
    /// (height i16 LE, top u32, side u32, flags: bit0 water, bit1 valid).
    pub fn to_bytes(&self) -> Vec<u8> {
        let snap = self.snapshot();
        let n = snap.len();
        let mut out = Vec::with_capacity(MAGIC.len() + 20 + n * CELL_BYTES);
        out.extend_from_slice(MAGIC);
        out.extend_from_slice(&self.center_x.to_le_bytes());
        out.extend_from_slice(&self.center_z.to_le_bytes());
        out.extend_from_slice(&self.size.to_le_bytes());
        out.extend_from_slice(&self.cell.to_le_bytes());
        out.extend_from_slice(&self.sea_level.to_le_bytes());
        for c in &snap {
            out.extend_from_slice(&c.height.to_le_bytes());
            out.extend_from_slice(&(c.top & 0xFFFFFF).to_le_bytes());
            out.extend_from_slice(&(c.side & 0xFFFFFF).to_le_bytes());
            out.push((u8::from(c.water)) | (u8::from(c.valid) << 1));
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
        if buf.len() < MAGIC.len() + 20 + n * CELL_BYTES {
            return None;
        }
        let mut data = atlas.data.write().ok()?;
        let base = MAGIC.len() + 20;
        let mut scanned = 0u64;
        for (i, cell_out) in data.iter_mut().enumerate() {
            let o = base + i * CELL_BYTES;
            let height = i16::from_le_bytes([buf[o], buf[o + 1]]);
            let top = u32::from_le_bytes([buf[o + 2], buf[o + 3], buf[o + 4], 0]) & 0xFFFFFF;
            let side = u32::from_le_bytes([buf[o + 6], buf[o + 7], buf[o + 8], 0]) & 0xFFFFFF;
            let flags = buf[o + 10];
            if flags & 0b10 != 0 {
                *cell_out = Cell {
                    height,
                    water: flags & 0b01 != 0,
                    valid: true,
                    top,
                    side,
                };
                scanned += 1;
            }
        }
        drop(data);
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
        a.set(3, 5, 100, false, 0x808080, 0x707070);
        a.set(0, 0, 62, true, 0x808080, 0x707070);
        assert_eq!(a.get(3, 5).unwrap().height, 100);
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
    }

    #[test]
    fn version_bumps_on_change() {
        let a = Atlas::new(0, 0, 64, 4, 63);
        let v0 = a.version();
        a.set(1, 1, 10, false, 0x808080, 0x707070);
        let v1 = a.version();
        a.set(1, 1, 10, false, 0x808080, 0x707070); // same value — no bump
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
        a.set(3, 5, 100, false, 0x808080, 0x707070);
        a.set(0, 0, 62, true, 0x808080, 0x707070);
        a.set(15, 15, 200, false, 0x808080, 0x707070);
        a.save(&path).unwrap();
        // tmp file must be gone after atomic rename
        assert!(!path.with_extension("tmp").exists());

        let b = Atlas::try_restore(&path, 123, -456, 64, 4, 62).expect("restore");
        assert_eq!(b.center_x, 123);
        assert_eq!(b.center_z, -456);
        assert_eq!(b.scanned_cells(), 3);
        assert_eq!(b.get(3, 5).unwrap().height, 100);
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
