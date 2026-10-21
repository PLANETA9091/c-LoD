//! The LOD heightmap atlas: a `cells x cells` grid covering a square world
//! region. Cell data:
//!   - height: i16 block Y (decoded from the atlas PNG's R/G channels),
//!   - water:  bool (sea surface instead of terrain),
//!   - state:  u8 blockstate id in the pack state table (v4.3, Voxy),
//!   - light:  u8 Voxy lightId (block<<4 | sky),
//!   - tint:   RGB biome tint for the shader (white = untinted),
//!     shipped as a second PNG (`atlas_col.png`).
//!
//! The height PNG carries the Voxy mip pyramid (analysis/voxy-e2e.md):
//! level 0 (cells x cells) plus levels 1..4 stacked below, each reduced by
//! `voxy_core::mip4` — the max-opacity corner voxel keeps its full id.
//!
//! Snapshot format v3 (v4.3): adds state + light bytes per cell. A stale v2
//! cache fails the magic check and is rebuilt by a fresh scan.
//!
//! Static-pack lifecycle (v3): the atlas is filled ONCE (JNI budget per tick
//! until completion or the scan deadline), optionally persisted to
//! `c-LoD-atlas.bin`, encoded into the resource pack exactly once, and never
//! mutated again. Restarts restore the snapshot so the pack SHA-1 — and the
//! client download state — stays identical across reboots.

use std::io::Write as _;
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::RwLock;

/// On-disk snapshot magic + format version.
const MAGIC: &[u8; 8] = b"CLODAT03";

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
    /// Scan order: cell indices sorted by distance from the atlas center
    /// (v4.2 radial sweep). The budget then always fills the disc around
    /// the players first — the horizon they SEE — instead of row-major
    /// stripes that walk straight into unloaded chunks.
    order: Vec<u32>,
}

#[derive(Clone, Copy, Default, PartialEq)]
pub struct Cell {
    pub height: i16,
    pub water: bool,
    pub valid: bool,
    /// Blockstate id in the pack state table (voxy_core), 0 = unknown.
    pub state: u8,
    /// Voxy lightId: (block << 4) | sky (Voxy voxel id bits 56-63; sunlit
    /// surface = 0x0F, block-lit lava = 0xF0).
    pub light: u8,
    /// Biome tint for the shader (white = untinted block).
    pub rgb: [u8; 3],
}

impl Atlas {
    pub fn new(center_x: i32, center_z: i32, size: i32, cell: i32, sea_level: i32) -> Self {
        let cells = (size / cell).max(1) as u32;
        let n = cells;
        // Radial order: sort cell indices by Chebyshev ring distance from
        // the center POINT (n-1)/2 in half-cell units, so even grids stay
        // exactly symmetric (cells 7..8 both map to ring 0 on a 16x16
        // atlas). Deterministic, cheap, ties broken by index. A full
        // Euclidean sort of 1M cells is wasted precision for a scan sweep;
        // rings read naturally from any direction.
        let nn = i64::from(n);
        let mut order: Vec<u32> = (0..n * n).collect();
        order.sort_unstable_by_key(|&i| {
            let ix = i % n;
            let iz = i / n;
            let dx = (2 * i64::from(ix) + 1 - nn).abs();
            let dz = (2 * i64::from(iz) + 1 - nn).abs();
            (dx.max(dz), ix, iz)
        });
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
            order,
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

    #[allow(clippy::too_many_arguments)] // Voxy voxel id pieces: height+water+tint+state+light
    pub fn set(
        &self,
        ix: u32,
        iz: u32,
        height: i16,
        water: bool,
        rgb: [u8; 3],
        state: u8,
        light: u8,
    ) {
        let idx = (iz * self.cells + ix) as usize;
        if let Ok(mut data) = self.data.write() {
            if let Some(cell) = data.get_mut(idx) {
                if !cell.valid
                    || cell.height != height
                    || cell.water != water
                    || cell.rgb != rgb
                    || cell.state != state
                    || cell.light != light
                {
                    if !cell.valid {
                        self.scanned.fetch_add(1, Ordering::Relaxed);
                    }
                    *cell = Cell {
                        height,
                        water,
                        valid: true,
                        rgb,
                        state,
                        light,
                    };
                    self.version.fetch_add(1, Ordering::Relaxed);
                }
            }
        }
    }

    /// Highest valid terrain height in the snapshot (for the shader's
    /// ray-culling ceiling). -64 when nothing was scanned.
    pub fn max_height(&self) -> i16 {
        self.data
            .read()
            .map(|d| {
                d.iter()
                    .filter(|c| c.valid)
                    .map(|c| c.height)
                    .max()
                    .unwrap_or(-64)
            })
            .unwrap_or(-64)
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

    /// Iterate cell indices for a scan sweep: v4.2 radial order from the
    /// atlas center (rings first), wrapping around the total.
    pub fn scan_task(&self, seq: u64, count: u32) -> Vec<(u32, u32)> {
        let total = self.order.len() as u64;
        if total == 0 {
            return Vec::new();
        }
        let n = self.cells;
        let mut out = Vec::with_capacity(count as usize);
        let mut s = seq % total;
        while out.len() < count as usize {
            let i = self.order[s as usize];
            out.push((i % n, i / n));
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

    /// Encode ONE level of the pyramid as RGBA8 pixels:
    ///   R = high byte, G = low byte of `(height + 64) << 6`,
    ///   B = 255 when water,
    ///   A = blockstate id (0 = unknown — shader falls back to albedo).
    /// Unscanned cells encode height 0 (decodes to -64: void).
    #[allow(dead_code)] // used by encode_rgba (tests)
    fn encode_level(cells: &[Cell], n: u32) -> Vec<u8> {
        let mut px = vec![0u8; (n * n * 4) as usize];
        for z in 0..n as usize {
            for x in 0..n as usize {
                let cell = cells[z * n as usize + x];
                let (y16, water) = if cell.valid {
                    let h = (i32::from(cell.height) + 64).max(0) << 6;
                    (h.min(0xFFFF) as u16, cell.water)
                } else {
                    (0, false)
                };
                let o = (z * n as usize + x) * 4;
                px[o] = (y16 >> 8) as u8;
                px[o + 1] = (y16 & 0xFF) as u8;
                px[o + 2] = if water { 255 } else { 0 };
                px[o + 3] = cell.state;
            }
        }
        px
    }

    /// Encode the full Voxy mip pyramid as ONE PNG: level 0 (cells x cells)
    /// with levels 1..4 stacked BELOW it, each halved by voxy_core::mip4.
    /// The shader picks the level by ray distance (Voxy screen-area detail)
    /// and marches the coarse levels for the far silhouette.
    pub fn encode_pyramid_rgba(&self) -> (u32, u32, Vec<u8>) {
        let snap = self.snapshot();
        let n = self.cells as usize;
        let v0: Vec<crate::voxy_core::VCell> = snap
            .iter()
            .map(|c| crate::voxy_core::VCell {
                height: c.height,
                water: c.water,
                valid: c.valid,
                state: c.state,
                light: c.light,
            })
            .collect();
        let pyramid = crate::voxy_core::build_pyramid(&v0, n);
        let mut total_h = 0u32;
        let mut widths = Vec::new();
        for lvl in &pyramid {
            let ln = (lvl.len() as f64).sqrt().ceil() as u32;
            widths.push(ln);
            total_h += ln;
        }
        let mut px = vec![0u8; (self.cells * total_h * 4) as usize];
        let mut y_off = 0u32;
        for (lvl, ln) in pyramid.iter().zip(&widths) {
            let level_px = Self::encode_level_padded(lvl, *ln);
            let row_bytes = (*ln * 4) as usize;
            for y in 0..*ln as usize {
                let dst = (((y_off as usize) + y) * self.cells as usize) * 4;
                px[dst..dst + row_bytes]
                    .copy_from_slice(&level_px[y * row_bytes..(y + 1) * row_bytes]);
            }
            y_off += *ln;
        }
        (self.cells, total_h, px)
    }

    /// Encode one pyramid level, padded to `n` (missing cells = void).
    fn encode_level_padded(cells: &[crate::voxy_core::VCell], n: u32) -> Vec<u8> {
        let mut px = vec![0u8; (n * n * 4) as usize];
        let cn = (cells.len() as f64).sqrt().ceil() as usize;
        for z in 0..n as usize {
            for x in 0..n as usize {
                let cell = cells.get(z * cn + x).copied().unwrap_or_default();
                let (y16, water) = if cell.valid {
                    let h = (i32::from(cell.height) + 64).max(0) << 6;
                    (h.min(0xFFFF) as u16, cell.water)
                } else {
                    (0, false)
                };
                let o = (z * n as usize + x) * 4;
                px[o] = (y16 >> 8) as u8;
                px[o + 1] = (y16 & 0xFF) as u8;
                px[o + 2] = if water { 255 } else { 0 };
                px[o + 3] = cell.state;
            }
        }
        px
    }

    /// Legacy flat encode (v4.2 tests + a debug path): level 0 only.
    #[allow(dead_code)] // used by tests
    pub fn encode_rgba(&self) -> (u32, u32, Vec<u8>) {
        let snap = self.snapshot();
        let n = self.cells;
        let px = Self::encode_level(&snap, n);
        (n, n, px)
    }

    /// Encode the per-cell tint PNG (v4.3): RGB = biome tint the shader
    /// multiplies into the UP-face texture (white = untinted), A = Voxy
    /// lightId. Void cells diffuse from the nearest valid neighbor so the
    /// scan frontier never shows black fringes under NEAREST sampling.
    pub fn encode_color_rgba(&self) -> (u32, u32, Vec<u8>) {
        let snap = self.snapshot();
        let n = self.cells as usize;
        let mut px = vec![0u8; n * n * 4];
        for z in 0..n {
            for x in 0..n {
                let cell = snap[z * n + x];
                if !cell.valid {
                    continue; // filled below by diffusion
                }
                let o = (z * n + x) * 4;
                px[o] = cell.rgb[0];
                px[o + 1] = cell.rgb[1];
                px[o + 2] = cell.rgb[2];
                px[o + 3] = cell.light;
            }
        }
        // Diffusion pass: every void cell takes the nearest valid color by
        // scanning left/right/up/down (deterministic, order-free).
        let color_at = |x: usize, z: usize| -> Option<[u8; 3]> {
            let c = snap.get(z * n + x)?;
            c.valid.then_some(c.rgb)
        };
        for z in 0..n {
            for x in 0..n {
                if snap[z * n + x].valid {
                    continue;
                }
                let mut found = None;
                for d in 1..n {
                    let mut consider = |xx: usize, zz: usize| {
                        if found.is_none() {
                            if let Some(c) = color_at(xx, zz) {
                                found = Some(c);
                            }
                        }
                    };
                    if x >= d {
                        consider(x - d, z);
                    }
                    if x + d < n {
                        consider(x + d, z);
                    }
                    if z >= d {
                        consider(x, z - d);
                    }
                    if z + d < n {
                        consider(x, z + d);
                    }
                    if found.is_some() {
                        break;
                    }
                }
                if let Some(c) = found {
                    let o = (z * n + x) * 4;
                    px[o] = c[0];
                    px[o + 1] = c[1];
                    px[o + 2] = c[2];
                    px[o + 3] = 255;
                }
            }
        }
        (self.cells, self.cells, px)
    }

    // -----------------------------------------------------------------------
    // Disk persistence (static-pack determinism across restarts).
    // -----------------------------------------------------------------------

    /// Serialize the atlas snapshot: MAGIC + header + 8 bytes per cell
    /// (height i16 LE, flags: bit0 water bit1 valid, tint RGB888,
    /// state u8, light u8).
    pub fn to_bytes(&self) -> Vec<u8> {
        let snap = self.snapshot();
        let n = snap.len();
        let mut out = Vec::with_capacity(MAGIC.len() + 20 + n * 8);
        out.extend_from_slice(MAGIC);
        out.extend_from_slice(&self.center_x.to_le_bytes());
        out.extend_from_slice(&self.center_z.to_le_bytes());
        out.extend_from_slice(&self.size.to_le_bytes());
        out.extend_from_slice(&self.cell.to_le_bytes());
        out.extend_from_slice(&self.sea_level.to_le_bytes());
        for c in &snap {
            out.extend_from_slice(&c.height.to_le_bytes());
            out.push((u8::from(c.water)) | (u8::from(c.valid) << 1));
            out.extend_from_slice(&c.rgb);
            out.push(c.state);
            out.push(c.light);
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
        if buf.len() < MAGIC.len() + 20 + n * 8 {
            return None;
        }
        let mut data = atlas.data.write().ok()?;
        let base = MAGIC.len() + 20;
        let mut scanned = 0u64;
        for (i, cell_out) in data.iter_mut().enumerate() {
            let o = base + i * 8;
            let height = i16::from_le_bytes([buf[o], buf[o + 1]]);
            let flags = buf[o + 2];
            if flags & 0b10 != 0 {
                *cell_out = Cell {
                    height,
                    water: flags & 0b01 != 0,
                    valid: true,
                    rgb: [buf[o + 3], buf[o + 4], buf[o + 5]],
                    state: buf[o + 6],
                    light: buf[o + 7],
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
        a.set(3, 5, 100, false, [0x91, 0xBD, 0x59], 1, 0xF0);
        a.set(0, 0, 62, true, [0x3F, 0x76, 0xE4], 1, 0xF0);
        assert_eq!(a.get(3, 5).unwrap().height, 100);
        assert_eq!(a.get(3, 5).unwrap().rgb, [0x91, 0xBD, 0x59]);
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
        // invalid -> zero
        let o3 = (7 * 16 + 7) * 4;
        assert_eq!(px[o3], 0);
        assert_eq!(px[o3 + 2], 0);

        // Color PNG: valid cells carry rgb, invalid stay black.
        let (_, _, cpx) = a.encode_color_rgba();
        assert_eq!(&cpx[o..o + 3], &[0x91, 0xBD, 0x59]);
        assert_eq!(&cpx[o2..o2 + 3], &[0x3F, 0x76, 0xE4]);
        assert!(cpx[o3..o3 + 3].iter().all(|&v| v == 0));
    }

    #[test]
    fn max_height_tracks_scan() {
        let a = Atlas::new(0, 0, 64, 4, 63);
        assert_eq!(a.max_height(), -64);
        a.set(1, 1, 70, false, [1, 2, 3], 1, 0xF0);
        a.set(2, 2, 130, false, [1, 2, 3], 1, 0xF0);
        a.set(3, 3, 90, false, [1, 2, 3], 1, 0xF0);
        assert_eq!(a.max_height(), 130);
    }

    #[test]
    fn version_bumps_on_change() {
        let a = Atlas::new(0, 0, 64, 4, 63);
        let v0 = a.version();
        a.set(1, 1, 10, false, [1, 1, 1], 1, 0xF0);
        a.set(1, 1, 10, false, [1, 1, 1], 1, 0xF0); // same value — no bump
        a.set(1, 1, 10, false, [2, 2, 2], 1, 0xF0); // new color — bump
        assert_eq!(v0 + 2, a.version());
        assert_eq!(a.scanned_cells(), 1);
    }

    #[test]
    fn scan_task_is_radial_and_wraps() {
        let a = Atlas::new(0, 0, 64, 4, 63);
        // Asking for more than `total` yields exactly `total` unique cells.
        let t = a.scan_task(0, 257);
        assert_eq!(t.len(), 256);
        // Radial: the first tasks are the 4 center cells (Chebyshev ring 0),
        // and every returned cell sits inside the claimed ring.
        let center = |c: (u32, u32)| c.0.abs_diff(7).max(c.1.abs_diff(7));
        assert!(t.iter().all(|&c| center(c) == 0) || center(t[0]) <= center(t[t.len() - 1]));
        // Wrap: seq 256 restarts the ring order.
        let t2 = a.scan_task(256, 4);
        let t3 = a.scan_task(0, 4);
        assert_eq!(t2, t3);
    }

    #[test]
    fn radial_order_prefers_center() {
        // 16x16 atlas, center point 7.5 (half-cell units): ring 0 holds the
        // 4 center cells, ring 1 the 12 cells at Chebyshev 1 — together
        // exactly the first 16 tasks. The disc around players fills before
        // the corners of the atlas.
        let a = Atlas::new(0, 0, 64, 4, 63);
        let t = a.scan_task(0, 16);
        let ring = |c: (u32, u32)| {
            (2 * i64::from(c.0) + 1 - 16)
                .abs()
                .max((2 * i64::from(c.1) + 1 - 16).abs())
        };
        assert!(t.iter().all(|&c| ring(c) <= 3), "t: {t:?}");
    }

    #[test]
    fn save_restore_roundtrip() {
        let dir = std::env::temp_dir().join(format!("clod-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("atlas.bin");

        let a = Atlas::new(123, -456, 64, 4, 62);
        a.set(3, 5, 100, false, [0x10, 0x20, 0x30], 1, 0xF0);
        a.set(0, 0, 62, true, [0x40, 0x50, 0x60], 1, 0xF0);
        a.set(15, 15, 200, false, [0x70, 0x80, 0x90], 1, 0xF0);
        a.save(&path).unwrap();
        // tmp file must be gone after atomic rename
        assert!(!path.with_extension("tmp").exists());

        let b = Atlas::try_restore(&path, 123, -456, 64, 4, 62).expect("restore");
        assert_eq!(b.center_x, 123);
        assert_eq!(b.center_z, -456);
        assert_eq!(b.scanned_cells(), 3);
        assert_eq!(b.get(3, 5).unwrap().height, 100);
        assert_eq!(b.get(3, 5).unwrap().rgb, [0x10, 0x20, 0x30]);
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

    // ---- shader-contract mirrors (assets/glsl/lod_terrain.fsh) -------------

    /// Mirror of the shader's `atlas_cell` addressing (v4.3.1): level L is a
    /// (cells/2^L)-pixel square at row topPx(L) of a cells-wide PNG, u
    /// normalized by the FULL width. v4.3.0 multiplied by 2^L instead of
    /// dividing and normalized u by the level's cell count, so every L>=1
    /// fetch read the wrong rows (padding / the next strip) — garbage
    /// heights and states across the whole far LOD band.
    fn shader_atlas_fetch(
        px: &[u8],
        w: u32,
        h: u32,
        cells: f64,
        level: u32,
        local: (f64, f64),
    ) -> (i16, bool, u8) {
        let cells_l = cells / 2f64.powi(level as i32);
        let top_px = cells * (2.0 - 2f64.powi(1 - level as i32));
        let total_h = cells * (2.0 - 2f64.powi(-4));
        let cx = ((local.0 * cells_l).floor() + 0.5) / cells;
        let cy = (top_px + (local.1 * cells_l).floor() + 0.5) / total_h;
        let tx = ((cx * f64::from(w)).floor() as u32).min(w - 1);
        let ty = ((cy * f64::from(h)).floor() as u32).min(h - 1);
        let o = ((ty * w + tx) * 4) as usize;
        let y16 = (u16::from(px[o]) << 8) | u16::from(px[o + 1]);
        ((y16 as i16) / 64 - 64, px[o + 2] > 0, px[o + 3])
    }

    #[test]
    fn pyramid_addressing_matches_the_shader_contract() {
        let a = Atlas::new(0, 0, 256, 4, 63);
        assert_eq!(a.cells, 64);
        let height = |ix: u32, iz: u32| 50 + ((ix * 7 + iz * 13) % 45) as i16;
        let state = |ix: u32, iz: u32| ((ix * 3 + iz * 5) % 254 + 1) as u8;
        let rgb = |ix: u32, iz: u32| [ix as u8, iz as u8, (ix + iz) as u8];
        for iz in 0..64u32 {
            for ix in 0..64u32 {
                a.set(
                    ix,
                    iz,
                    height(ix, iz),
                    (ix + iz) % 11 == 0,
                    rgb(ix, iz),
                    state(ix, iz),
                    0xF0,
                );
            }
        }
        let (w, h, px) = a.encode_pyramid_rgba();
        assert_eq!((w, h), (64, 124), "cells-wide PNG, levels 64+32+16+8+4");

        // Contract: the shader's mapping must read back EXACTLY the cell the
        // Mipper stored on each level (water mixes in: land (op 15) beats
        // water (op 1), ties resolve to the +x+z corner — recursively).
        let snap = a.snapshot();
        let v0: Vec<crate::voxy_core::VCell> = snap
            .iter()
            .map(|c| crate::voxy_core::VCell {
                height: c.height,
                water: c.water,
                valid: c.valid,
                state: c.state,
                light: c.light,
            })
            .collect();
        let pyramid = crate::voxy_core::build_pyramid(&v0, 64);
        for level in 0..=4u32 {
            let groups = 64u32 >> level;
            for gz in 0..groups {
                for gx in 0..groups {
                    let local = (
                        (f64::from(gx) + 0.5) * f64::from(1 << level) / 64.0,
                        (f64::from(gz) + 0.5) * f64::from(1 << level) / 64.0,
                    );
                    let want = pyramid[level as usize][(gz * groups + gx) as usize];
                    let (fh, fw, fs) = shader_atlas_fetch(&px, w, h, 64.0, level, local);
                    assert_eq!(fh, want.height, "L{level} ({gx},{gz}) height");
                    assert_eq!(fs, want.state, "L{level} ({gx},{gz}) state");
                    assert_eq!(fw, want.water, "L{level} ({gx},{gz}) water");
                }
            }
        }
    }

    #[test]
    fn color_fetch_targets_the_mipper_winner_cell() {
        // The colour PNG is L0-resolution; the shader's col_fetch maps a
        // level-L cell to the group's +x+z L0 cell. Mirror the mapping and
        // assert the tint/light read back is the winner's.
        let a = Atlas::new(0, 0, 256, 4, 63);
        let rgb = |ix: u32, iz: u32| [ix as u8, iz as u8, (ix + iz) as u8];
        for iz in 0..64u32 {
            for ix in 0..64u32 {
                a.set(ix, iz, 70, false, rgb(ix, iz), 1, 0xF0);
            }
        }
        let (w, h, px) = a.encode_color_rgba();
        assert_eq!((w, h), (64, 64));
        for level in 1..=4u32 {
            let groups = 64u32 >> level;
            let gz = groups / 2;
            let gx = groups / 2;
            let cells_l = f64::from(groups);
            let local = (
                (f64::from(gx) + 0.5) / cells_l,
                (f64::from(gz) + 0.5) / cells_l,
            );
            let step = f64::from(1 << level);
            let ix0 = (((local.0 * cells_l).floor() + 1.0) * step - 1.0).min(63.0) as u32;
            let iz0 = (((local.1 * cells_l).floor() + 1.0) * step - 1.0).min(63.0) as u32;
            let corner = ((gx + 1) * (1 << level)) - 1;
            let corner_z = ((gz + 1) * (1 << level)) - 1;
            assert_eq!((ix0, iz0), (corner, corner_z), "winner cell L{level}");
            let o = ((iz0 * 64 + ix0) * 4) as usize;
            assert_eq!(&px[o..o + 3], &rgb(ix0, iz0), "tint at L{level}");
            assert_eq!(px[o + 3], 0xF0, "light at L{level}");
        }
    }
}
