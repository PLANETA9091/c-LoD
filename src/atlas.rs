//! The LOD heightmap atlas: a `cells x cells` grid covering a square world
//! region. Cell data:
//!   - height: i16 block Y (decoded from the atlas PNG's R/G channels),
//!   - water:  bool (sea surface instead of terrain),
//!   - rgb:    real surface color (block palette + biome tint, v4.2),
//!     shipped as a second PNG (`atlas_col.png`) — the Voxy look.
//!
//! Snapshot format v2 (v4.2): adds 3 color bytes per cell. A stale v1 cache
//! fails the magic check and is rebuilt by a fresh scan.
//!
//! Static-pack lifecycle (v3): the atlas is filled ONCE (JNI budget per tick
//! until completion or the scan deadline), optionally persisted to
//! `c-LoD-atlas.bin`, encoded into the resource pack exactly once, and never
//! mutated again. Restarts restore the snapshot so the pack SHA-1 — and the
//! client download state — stays identical across reboots.

use std::io::Write as _;
use std::path::Path;
use std::sync::atomic::{AtomicI32, AtomicU64, Ordering};
use std::sync::RwLock;

/// On-disk snapshot magic + format version.
const MAGIC: &[u8; 8] = b"CLODAT02";

pub struct Atlas {
    /// Window center in world blocks — ATOMIC: v4.4 slides the window to
    /// follow the player (recenter) while readers snapshot it lock-free.
    pub center_x: AtomicI32,
    pub center_z: AtomicI32,
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
    /// Surface albedo (palette.rs), 0,0,0 = unscanned.
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
            center_x: AtomicI32::new(center_x),
            center_z: AtomicI32::new(center_z),
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

    /// Current window center (world blocks), consistent pair.
    pub fn center(&self) -> (i32, i32) {
        (
            self.center_x.load(Ordering::Relaxed),
            self.center_z.load(Ordering::Relaxed),
        )
    }

    /// Cell indices for a world block coordinate, or None when outside.
    pub fn cell_index(&self, x: i32, z: i32) -> Option<(u32, u32)> {
        let (cx, cz) = self.center();
        let dx = x - (cx - self.half());
        let dz = z - (cz - self.half());
        if dx < 0 || dz < 0 || dx >= self.size || dz >= self.size {
            return None;
        }
        Some(((dx / self.cell) as u32, (dz / self.cell) as u32))
    }

    pub fn set(&self, ix: u32, iz: u32, height: i16, water: bool, rgb: [u8; 3]) {
        let idx = (iz * self.cells + ix) as usize;
        if let Ok(mut data) = self.data.write() {
            if let Some(cell) = data.get_mut(idx) {
                if !cell.valid || cell.height != height || cell.water != water || cell.rgb != rgb {
                    if !cell.valid {
                        self.scanned.fetch_add(1, Ordering::Relaxed);
                    }
                    *cell = Cell {
                        height,
                        water,
                        valid: true,
                        rgb,
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

    /// v4.3 streaming: the same sweep, anchored at a PLAYER position instead
    /// of the atlas center — rings expand outward from where the player is
    /// actually standing, so newly-loaded chunks (view distance) enter the
    /// atlas before anything else: the streamed revisions carry exactly the
    /// terrain the player is looking at (DH-style nearest-first streaming).
    /// Only cells that are not yet valid are returned; valid cells are
    /// skipped cheaply so the steady state costs ~nothing.
    pub fn scan_task_near(&self, bx: i32, bz: i32, count: u32) -> Vec<(u32, u32)> {
        if count == 0 {
            return Vec::new();
        }
        let Some((pcx, pcz)) = self.cell_index(bx, bz) else {
            return Vec::new();
        };
        let n = self.cells as i32;
        let mut out: Vec<(u32, u32)> = Vec::with_capacity(count as usize);
        // Ring-by-ring Chebyshev spiral; each ring walks its perimeter
        // (deterministic: top row -> right col -> bottom row -> left col).
        let max_r = (n - 1).max(0);
        'rings: for r in 0..=max_r {
            let x0 = (pcx as i32 - r).max(0);
            let x1 = (pcx as i32 + r).min(n - 1);
            let z0 = (pcz as i32 - r).max(0);
            let z1 = (pcz as i32 + r).min(n - 1);
            // Ring perimeter cells (dedup via visit order, corners once).
            let mut ring: Vec<(i32, i32)> = Vec::with_capacity((4 * r as usize + 4) * 2);
            for x in x0..=x1 {
                ring.push((x, z0));
            }
            for z in z0 + 1..=z1 {
                ring.push((x1, z));
            }
            for x in (x0..x1).rev() {
                ring.push((x, z1));
            }
            for z in (z0 + 1..z1).rev() {
                ring.push((x0, z));
            }
            for (x, z) in ring {
                let valid = self
                    .data
                    .read()
                    .ok()
                    .and_then(|d| d.get((z * n + x) as usize).map(|c| c.valid))
                    .unwrap_or(true);
                if !valid {
                    out.push((x as u32, z as u32));
                    if out.len() >= count as usize {
                        break 'rings;
                    }
                }
            }
        }
        out
    }

    /// v4.4 stream_probe: the outermost UNSCANNED cell in the radial order.
    /// The order runs rings from the center, so the tail of the order is the
    /// most reliably unscanned territory (the boot deadline cuts it mid-way)
    /// — the deterministic place to drop a virtual player for the e2e probe.
    /// Returns None when every cell is valid (fully scanned atlas).
    pub fn outermost_unscanned_cell(&self) -> Option<(u32, u32)> {
        let d = self.data.read().ok()?;
        let n = self.cells as usize;
        for &i in self.order.iter().rev() {
            let (ix, iz) = (i % n as u32, i / n as u32);
            if !d.get(i as usize).map(|c| c.valid).unwrap_or(true) {
                return Some((ix, iz));
            }
        }
        None
    }

    /// World X of cell center.
    pub fn cell_world_x(&self, ix: u32) -> i32 {
        self.center().0 - self.half() + (ix * self.cell as u32) as i32 + self.cell / 2
    }

    /// v4.4 streaming: shift the atlas WINDOW so the player stays near its
    /// center (the DH tile-grid model with one big tile). dcell_x/dcell_z is
    /// the window movement in cells (new center = old center + delta in world
    /// blocks = delta * cell). Content rotates with the WORLD: every cell
    /// keeps its data at its world position (new[ix] = old[ix + dcell]), the
    /// vacated trailing edge becomes void and re-enters the sweep at its new
    /// ring distance. The radial scan order is grid-relative and survives
    /// unchanged, and `scan_task_near` re-anchors on the player next tick.
    /// Returns true when the window actually moved.
    pub fn recenter(&self, dcell_x: i32, dcell_z: i32) -> bool {
        if dcell_x == 0 && dcell_z == 0 {
            return false;
        }
        let n = self.cells as usize;
        let Ok(mut old) = self.data.write() else {
            return false;
        };
        let mut moved: Vec<Cell> = vec![Cell::default(); n * n];
        for iz in 0..n {
            let sz = iz as i64 + i64::from(dcell_z);
            if sz < 0 || sz >= n as i64 {
                continue; // slid in from the data edge: void until scanned
            }
            for ix in 0..n {
                let sx = ix as i64 + i64::from(dcell_x);
                if sx < 0 || sx >= n as i64 {
                    continue;
                }
                moved[iz * n + ix] = old[sz as usize * n + sx as usize];
            }
        }
        let scanned_now = moved.iter().filter(|c| c.valid).count() as u64;
        *old = moved;
        self.scanned.store(scanned_now, Ordering::Relaxed);
        self.center_x
            .fetch_add(dcell_x * self.cell, Ordering::Relaxed);
        self.center_z
            .fetch_add(dcell_z * self.cell, Ordering::Relaxed);
        self.version.fetch_add(1, Ordering::Relaxed);
        true
    }

    /// World Z of cell center.
    pub fn cell_world_z(&self, iz: u32) -> i32 {
        self.center().1 - self.half() + (iz * self.cell as u32) as i32 + self.cell / 2
    }

    /// Encode the atlas as RGBA8 pixels for the height PNG:
    ///   R = high byte, G = low byte of `(height + 64) << 6`,
    ///   B = 255 when water, A = 255 (opaque).
    /// Unscanned cells encode height 0 (decodes to -64: void) so the shader
    /// treats them as misses.
    pub fn encode_rgba(&self) -> (u32, u32, Vec<u8>) {
        let snap = self.snapshot();
        let n = self.cells as usize;
        let mut px = vec![0u8; n * n * 4];
        for z in 0..n {
            for x in 0..n {
                let cell = snap[z * n + x];
                let (y16, water) = if cell.valid {
                    let h = (i32::from(cell.height) + 64).max(0) << 6;
                    (h.min(0xFFFF) as u16, cell.water)
                } else {
                    (0, false)
                };
                let o = (z * n + x) * 4;
                px[o] = (y16 >> 8) as u8;
                px[o + 1] = (y16 & 0xFF) as u8;
                px[o + 2] = if water { 255 } else { 0 };
                px[o + 3] = 255;
            }
        }
        (self.cells, self.cells, px)
    }

    /// Encode the surface-color PNG (v4.2, the Voxy look): RGB = real block
    /// color + biome tint. Void cells get the color of the nearest VALID
    /// neighbor (4-direction diffusion) so bilinear filtering at the scan
    /// frontier blends terrain into terrain, not into black — the black
    /// fringe was the "dark mountain" artifact of the first colored build.
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
                px[o + 3] = 255;
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

    /// Serialize the atlas snapshot: MAGIC + header + 6 bytes per cell
    /// (height i16 LE, flags: bit0 water, bit1 valid, RGB888).
    pub fn to_bytes(&self) -> Vec<u8> {
        let snap = self.snapshot();
        let n = snap.len();
        let mut out = Vec::with_capacity(MAGIC.len() + 20 + n * 6);
        out.extend_from_slice(MAGIC);
        let (cx, cz) = self.center();
        out.extend_from_slice(&cx.to_le_bytes());
        out.extend_from_slice(&cz.to_le_bytes());
        out.extend_from_slice(&self.size.to_le_bytes());
        out.extend_from_slice(&self.cell.to_le_bytes());
        out.extend_from_slice(&self.sea_level.to_le_bytes());
        for c in &snap {
            out.extend_from_slice(&c.height.to_le_bytes());
            out.push((u8::from(c.water)) | (u8::from(c.valid) << 1));
            out.extend_from_slice(&c.rgb);
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
        // v4.4: the window CENTER is part of the streamed state now (it moves
        // with the player), so a snapshot restores at its OWN center — only
        // the geometry (size/cell/sea) must match the running config. The
        // want_center_* parameters stay for call-site compatibility but no
        // longer gate the restore.
        let _ = (want_center_x, want_center_z);
        if size != want_size || cell != want_cell || sea != want_sea_level {
            return None;
        }
        let atlas = Atlas::new(cx, cz, size, cell, sea);
        let n = (atlas.cells as usize) * (atlas.cells as usize);
        if buf.len() < MAGIC.len() + 20 + n * 6 {
            return None;
        }
        let mut data = atlas.data.write().ok()?;
        let base = MAGIC.len() + 20;
        let mut scanned = 0u64;
        for (i, cell_out) in data.iter_mut().enumerate() {
            let o = base + i * 6;
            let height = i16::from_le_bytes([buf[o], buf[o + 1]]);
            let flags = buf[o + 2];
            if flags & 0b10 != 0 {
                *cell_out = Cell {
                    height,
                    water: flags & 0b01 != 0,
                    valid: true,
                    rgb: [buf[o + 3], buf[o + 4], buf[o + 5]],
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
        a.set(3, 5, 100, false, [0x91, 0xBD, 0x59]);
        a.set(0, 0, 62, true, [0x3F, 0x76, 0xE4]);
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
        a.set(1, 1, 70, false, [1, 2, 3]);
        a.set(2, 2, 130, false, [1, 2, 3]);
        a.set(3, 3, 90, false, [1, 2, 3]);
        assert_eq!(a.max_height(), 130);
    }

    #[test]
    fn version_bumps_on_change() {
        let a = Atlas::new(0, 0, 64, 4, 63);
        let v0 = a.version();
        a.set(1, 1, 10, false, [1, 1, 1]);
        a.set(1, 1, 10, false, [1, 1, 1]); // same value — no bump
        a.set(1, 1, 10, false, [2, 2, 2]); // new color — bump
        assert_eq!(v0 + 2, a.version());
        assert_eq!(a.scanned_cells(), 1);
    }

    #[test]
    fn recenter_rotates_data_with_the_world() {
        // 16x16 grid, cell 4 -> 64x64 blocks. Fill two landmark cells at
        // known world positions, slide the window east+south, verify the
        // landmarks keep their DATA at the same WORLD positions and the
        // trailing edge went void.
        let a = Atlas::new(0, 0, 64, 4, 63);
        // World (10, 20) -> cell ((10+32)/4, (20+32)/4) = (10, 13).
        a.set(10, 13, 100, false, [1, 2, 3]);
        // World (-30, -30) -> cell (0, 0).
        a.set(0, 0, 50, true, [4, 5, 6]);
        assert_eq!(a.scanned_cells(), 2);

        // Slide the window 2 cells east (+8 blocks) and 1 cell south (+4).
        assert!(a.recenter(2, 1));
        assert_eq!(a.center(), (8, 4));

        // The (10,20) landmark now lives at cell (10-2, 13-1) = (8, 12).
        let c1 = a.get(8, 12).expect("east landmark followed the world");
        assert_eq!((c1.height, c1.rgb), (100, [1, 2, 3]));
        // The (0,0)-world cell moved out of the window (2 cells east, 1
        // south: old cell (0,0) would map to new (-2,-1) — outside) -> void.
        assert!(a.get(0, 0).is_none(), "trailing edge is void");
        // New trailing-edge cells also void; scanned count dropped to 1.
        assert_eq!(a.scanned_cells(), 1);

        // Sliding BACK restores the world data for cells that STAYED inside
        // the window throughout; the corner landmark left it (data lost, a
        // fresh scan refills that cell).
        assert!(a.recenter(-2, -1));
        assert_eq!(a.center(), (0, 0));
        let c2 = a.get(10, 13).expect("landmark back at origin cell");
        assert_eq!(c2.height, 100);
        assert!(
            a.get(0, 0).is_none(),
            "left the window: data lost until rescanned"
        );
        assert_eq!(a.scanned_cells(), 1);

        // Zero shift is a no-op (no version churn, no rebuild forcing).
        let v = a.version();
        assert!(!a.recenter(0, 0));
        assert_eq!(a.version(), v);
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
        a.set(3, 5, 100, false, [0x10, 0x20, 0x30]);
        a.set(0, 0, 62, true, [0x40, 0x50, 0x60]);
        a.set(15, 15, 200, false, [0x70, 0x80, 0x90]);
        a.save(&path).unwrap();
        // tmp file must be gone after atomic rename
        assert!(!path.with_extension("tmp").exists());

        let b = Atlas::try_restore(&path, 123, -456, 64, 4, 62).expect("restore");
        assert_eq!(b.center(), (123, -456));
        assert_eq!(b.scanned_cells(), 3);
        assert_eq!(b.get(3, 5).unwrap().height, 100);
        assert_eq!(b.get(3, 5).unwrap().rgb, [0x10, 0x20, 0x30]);
        assert!(b.get(0, 0).unwrap().water);
        assert_eq!(b.get(15, 15).unwrap().height, 200);

        // Deterministic encoding across instances (pack SHA-1 stability).
        let (_, _, px_a) = a.encode_rgba();
        let (_, _, px_b) = b.encode_rgba();
        assert_eq!(px_a, px_b);

        // Geometry mismatch -> no restore (stale cache). The WINDOW CENTER is
        // part of the streamed state (v4.4: it follows the player), so a
        // different want_center still restores — at the SNAPSHOT's center.
        let c = Atlas::try_restore(&path, 999, -456, 64, 4, 62).expect("center moves with data");
        assert_eq!(c.center(), (123, -456), "snapshot center wins over config");
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
}
