//! The streaming core: SKY-GRID BIT-STREAM (v5.0).
//!
//! Every byte of LOD data travels to the vanilla client as DUST PARTICLES
//! spawned above the scan disk — no map carriers, no offhand item, no
//! textures in the pack at all.
//!
//! Two particle planes float over the disk:
//!   HI (Y=260) — the 8-bit height word of the cell (h + 64, clamped 0..255);
//!   LO (Y=272) — the 8-bit color index of the cell (6x6x6 RGB lattice
//!   0..215, water = 216) in the HIGH byte of the word.
//!
//! One 16-bit word encodes EIGHT data bits as FOUR particles:
//!   particle slot s (0..3, Z axis) carries bits (14 - 2*s) .. (15 - 2*s);
//!   its X sub-position inside the cell IS the 2-bit value, its Z
//!   sub-position IS the slot. The sub-cell quantum is cell/4, centered.
//!   (The low byte of the word stays zero — the decoder reads the high
//!   byte only, keeping the codec dead simple and symmetric.)
//!
//! Server-side pacing: `tick_batch()` returns the next chunk of spawns
//! (at most `particles_per_tick`), marching the disk row-major, valid
//! cells only. A full 256x256 disk at 1024 particles/tick (128 cells =
//! 4 + 4 planes) takes 512 engine ticks — one lap every ~26 s at the
//! default 50 ms cadence, continuously refreshing the client's picture.
//!
//! The client-side decoder (`assets/glsl/capture.fsh`) re-projects all 16
//! sub-cell positions of a cell onto the screen, strobes them against the
//! depth buffer (dust renders on the OPAQUE layer and writes depth; sky
//! reads 1.0), and OR-merges decoded words into persistent ping-pong
//! targets. A particle that never made it through the strobe simply keeps
//! the previous value — the stream self-heals on the next lap.

/// Sub-cell lattice: 4 positions per axis inside one cell.
pub const SUB_PER_CELL: f64 = 4.0;
/// Particle layers: each cell's word spawns its 4 particles on EVERY layer
/// (a vertical "column" of sub-lattices). A layer is decodable when the
/// particle renders in front of geometry (26.3 reversed-Z hides particles
/// against the sky), so low layers decode from the ground, high ones from
/// elevated vantage points.
pub const LAYER_Y: [f64; 2] = [80.0, 120.0];
/// Legacy plane constants (kept for packgen templating of the layer list).
pub const PLANE_HI_Y: f64 = LAYER_Y[1];
pub const PLANE_LO_Y: f64 = LAYER_Y[0];
/// Dust particle scale (blocks) — size must be stable for the decoder.
pub const DUST_SCALE: f64 = 0.8;
/// Dust colors: the client multiplies ALL channels by one random factor, so
/// the HUE survives — the capture shader strobes on hue + the sky-depth test
/// (dust renders in front of the sky, where device depth reads 0).
/// HI plane (heights): BLACK dust — dark points on the bright day sky.
pub const DUST_RGB: u32 = 0x000000;
/// LO plane (colors): WHITE dust — bright points on the day sky.
pub const DUST_RGB_LO: u32 = 0xFFFFFF;
/// Water color index in the 6x6x6 lattice.
pub const COLOR_WATER: u16 = 216;

#[derive(Clone, Copy, Default, PartialEq)]
pub struct Cell {
    pub height: i16,
    pub water: bool,
    pub valid: bool,
    /// Surface albedo (palette.rs).
    pub rgb: [u8; 3],
}

impl Cell {
    /// The 8-bit color index (6x6x6 RGB lattice, water=216).
    pub fn color_index(&self) -> u16 {
        if self.water {
            return COLOR_WATER;
        }
        let r = (self.rgb[0] / 51).min(5) as u16;
        let g = (self.rgb[1] / 51).min(5) as u16;
        let b = (self.rgb[2] / 51).min(5) as u16;
        r + g * 6 + b * 36
    }
}

/// One dust particle to spawn this tick.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Spawn {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub rgb: u32,
    pub scale: f64,
}

/// The whole streaming state (server side): one fixed disk of cells plus
/// the phase cursor of the particle stream.
pub struct Streamer {
    /// World-space corner (min x/z) of the cell grid.
    pub origin_x: f64,
    pub origin_z: f64,
    pub cell_size: i32,
    /// Grid side in cells (region_blocks / cell_size).
    pub side: u32,
    /// side x side cells, row-major z outer.
    cells: Vec<Cell>,
    /// Next cell offset (0..side^2) to stream.
    cursor: u64,
    /// Pre-allocated spawn batch for the current tick.
    batch: Vec<Spawn>,
    /// Cells streamed since boot (diagnostics).
    pub streamed_cells: u64,
    /// Laps completed over the whole disk.
    pub laps: u64,
}

impl Streamer {
    pub fn new(origin_x: f64, origin_z: f64, region_blocks: i32, cell_size: i32) -> Self {
        let side = (region_blocks / cell_size).max(1) as u32;
        Streamer {
            origin_x,
            origin_z,
            cell_size,
            side,
            cells: vec![Cell::default(); side as usize * side as usize],
            cursor: 0,
            batch: Vec::with_capacity(4096),
            streamed_cells: 0,
            laps: 0,
        }
    }

    pub fn cells(&self) -> u64 {
        (self.side as u64) * (self.side as u64)
    }

    pub fn valid_cells(&self) -> u64 {
        self.cells.iter().filter(|c| c.valid).count() as u64
    }

    pub fn get(&self, cx: u32, cz: u32) -> Option<Cell> {
        let side = self.side as usize;
        self.cells.get(cz as usize * side + cx as usize).copied()
    }

    pub fn set_cell(&mut self, cx: u32, cz: u32, cell: Cell) {
        let side = self.side as usize;
        if cx as usize >= side || cz as usize >= side {
            return;
        }
        if let Some(slot) = self.cells.get_mut(cz as usize * side + cx as usize) {
            *slot = cell;
        }
    }

    /// World-space min corner of one cell.
    pub fn cell_origin(&self, cx: u32, cz: u32) -> (f64, f64) {
        (
            self.origin_x + cx as f64 * self.cell_size as f64,
            self.origin_z + cz as f64 * self.cell_size as f64,
        )
    }

    /// Encode one cell into its two 16-bit words (HI height, LO color).
    pub fn encode_words(cell: &Cell) -> (u16, u16) {
        let h = (i32::from(cell.height) + 64).clamp(0, 255) as u16;
        (h << 8, cell.color_index() << 8)
    }

    /// Spawn positions of one word: 4 particles, slot = Z sub, bits = X sub.
    /// msb-first: slot 0 carries bits 14..15 (the HIGH pair).
    pub fn word_spawns(
        word: u16,
        cx: f64,
        cz: f64,
        sub: f64,
        plane_y: f64,
        rgb: u32,
        out: &mut Vec<Spawn>,
    ) {
        for slot in 0..4usize {
            let bits = ((word >> (14 - 2 * slot)) & 3) as f64;
            let dx = (bits - 1.5) * sub;
            let dz = (slot as f64 - 1.5) * sub;
            out.push(Spawn {
                x: cx + dx,
                y: plane_y,
                z: cz + dz,
                rgb,
                scale: DUST_SCALE,
            });
        }
    }

    /// Advance the stream by one engine tick: fill the batch with at most
    /// `budget` dust spawns (valid cells only, row-major cursor).
    pub fn tick_batch(&mut self, budget: usize) -> &[Spawn] {
        self.batch.clear();
        let total = self.side as u64 * self.side as u64;
        if total == 0 {
            return &self.batch;
        }
        let sub = self.cell_size as f64 / SUB_PER_CELL;
        let budget = budget.max(8);
        let mut steps = 0u64;
        while self.batch.len() < budget && steps < total {
            let t = self.cursor % total;
            self.cursor += 1;
            steps += 1;
            let cx = (t % self.side as u64) as u32;
            let cz = (t / self.side as u64) as u32;
            let Some(cell) = self.get(cx, cz) else {
                continue;
            };
            if !cell.valid {
                continue; // void cells stream nothing — budget goes to data
            }
            let (wx, wz) = self.cell_origin(cx, cz);
            let cxw = wx + self.cell_size as f64 * 0.5;
            let czw = wz + self.cell_size as f64 * 0.5;
            let (hword, cword) = Self::encode_words(&cell);
            for &ly in LAYER_Y.iter() {
                Self::word_spawns(hword, cxw, czw, sub, ly, DUST_RGB, &mut self.batch);
                Self::word_spawns(cword, cxw, czw, sub, ly, DUST_RGB_LO, &mut self.batch);
            }
            self.streamed_cells += 1;
        }
        if self.cursor >= total {
            self.cursor %= total;
            self.laps += 1;
        }
        &self.batch
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cell(h: i16, water: bool, rgb: [u8; 3]) -> Cell {
        Cell {
            height: h,
            water,
            valid: true,
            rgb,
        }
    }

    #[test]
    fn color_index_lattice_and_water() {
        let c = cell(70, false, [0, 0, 0]);
        assert_eq!(c.color_index(), 0);
        let c = cell(70, false, [255, 255, 255]);
        assert_eq!(c.color_index(), 5 + 5 * 6 + 5 * 36);
        let c = cell(70, false, [127, 178, 56]);
        assert_eq!(c.color_index(), 2 + 3 * 6 + 36);
        let c = cell(70, true, [10, 20, 30]);
        assert_eq!(c.color_index(), COLOR_WATER);
    }

    #[test]
    fn word_round_trip_msb_first() {
        // 8 data bits in the high byte, 4 slots msb-first.
        let byte: u16 = 0b10_01_11_00;
        let word = byte << 8;
        let mut out = Vec::new();
        Streamer::word_spawns(word, 0.0, 0.0, 1.0, 0.0, DUST_RGB, &mut out);
        assert_eq!(out.len(), 4);
        // slot 0 -> bits 14..15 = 0b10 = 2; slot 1 -> 0b01 = 1;
        // slot 2 -> 0b11 = 3; slot 3 -> 0b00 = 0.
        let got: Vec<f64> = out.iter().map(|s| s.x).collect();
        assert_eq!(got, vec![0.5, -0.5, 1.5, -1.5]);
        let zs: Vec<f64> = out.iter().map(|s| s.z).collect();
        assert_eq!(zs, vec![-1.5, -0.5, 0.5, 1.5]);
        // Re-encode from positions: bits = round(x/sub + 1.5)
        for (s, slot) in out.iter().zip(0..4usize) {
            let bits = (s.x + 1.5).round() as i32;
            let word_bits = (word >> (14 - 2 * slot)) & 3;
            assert_eq!(bits as u16, word_bits);
        }
    }

    #[test]
    fn height_word_clamps_to_byte() {
        let c = cell(-64, false, [0, 0, 0]);
        let (h, _) = Streamer::encode_words(&c);
        assert_eq!(h >> 8, 0);
        let c = cell(320, false, [0, 0, 0]);
        let (h, _) = Streamer::encode_words(&c);
        assert_eq!(h >> 8, 255);
        let c = cell(64, false, [0, 0, 0]);
        let (h, _) = Streamer::encode_words(&c);
        assert_eq!(h >> 8, 128);
    }

    #[test]
    fn batch_budget_and_phase_cycle() {
        let mut s = Streamer::new(0.0, 0.0, 512, 4);
        assert_eq!(s.side, 128);
        // Fill the whole disk with valid cells.
        for cz in 0..128u32 {
            for cx in 0..128u32 {
                s.set_cell(cx, cz, cell(64 + (cx % 8) as i16, false, [100, 100, 100]));
            }
        }
        // 2048 particles / tick = 128 cells (2 layers x (4 HI + 4 LO) = 16).
        let b: Vec<Spawn> = s.tick_batch(2048).to_vec();
        assert_eq!(b.len(), 2048);
        assert_eq!(s.streamed_cells, 128);
        // Full lap: 16384 cells / 128 per tick = 128 ticks.
        for _ in 0..127 {
            s.tick_batch(2048);
        }
        assert_eq!(s.laps, 1);
        assert_eq!(s.streamed_cells, 16384);
        // Cursor wrapped: the next batch starts from cell 0 again.
        let b: Vec<Spawn> = s.tick_batch(16).to_vec();
        assert_eq!(b.len(), 16);
        let first = &b[0];
        let (wx, _wz) = s.cell_origin(0, 0);
        assert!((first.x - (wx + 2.0)).abs() < 1.0);
    }

    #[test]
    fn void_cells_cost_nothing() {
        let mut s = Streamer::new(0.0, 0.0, 64, 4);
        // Only 2 valid cells on the disk.
        s.set_cell(0, 0, cell(70, false, [90, 90, 90]));
        s.set_cell(15, 15, cell(80, true, [0, 0, 200]));
        let b: Vec<Spawn> = s.tick_batch(2048).to_vec();
        // 2 cells x 2 layers x 8 particles each.
        assert_eq!(b.len(), 32);
        assert_eq!(s.streamed_cells, 2);
        // Water cell streams the water color word on both layers (white dust).
        let lo: Vec<&Spawn> = b.iter().skip(28).collect();
        assert_eq!(lo.len(), 4);
        assert!(lo.iter().all(|p| p.rgb == DUST_RGB_LO));
        // All spawns of the second cell sit at cell (15,15) center.
        let (wx, wz) = s.cell_origin(15, 15);
        for p in lo {
            assert!((p.x - (wx + 2.0)).abs() <= 1.6);
            assert!((p.z - (wz + 2.0)).abs() <= 1.6);
        }
    }

    #[test]
    fn batch_positions_on_sub_lattice() {
        let mut s = Streamer::new(100.0, -40.0, 64, 4);
        s.set_cell(3, 5, cell(70, false, [200, 100, 50]));
        let b: Vec<Spawn> = s.tick_batch(2048).to_vec();
        assert_eq!(b.len(), 16);
        let sub = 4.0 / SUB_PER_CELL; // 1.0
        let (wx, wz) = s.cell_origin(3, 5);
        let cxw = wx + 2.0;
        let czw = wz + 2.0;
        for p in b.iter() {
            // dx in {-1.5,-0.5,0.5,1.5} * sub from the cell center.
            let dx = p.x - cxw;
            let dz = p.z - czw;
            assert!((dx / sub + 1.5).fract().abs() < 1e-6);
            assert!((dz / sub + 1.5).fract().abs() < 1e-6);
            assert!(dx.abs() <= 2.0 && dz.abs() <= 2.0);
            let want_rgb = if b.iter().position(|s| s == p).unwrap() % 8 < 4 {
                DUST_RGB // height word (black)
            } else {
                DUST_RGB_LO // color word (white)
            };
            assert_eq!(p.rgb, want_rgb);
            assert!((p.scale - DUST_SCALE).abs() < 1e-9);
        }
        // Order: layer0 heights, layer0 colors, layer1 heights, layer1 colors.
        assert_eq!(b[0].y, LAYER_Y[0]);
        assert_eq!(b[0].rgb, DUST_RGB);
        assert_eq!(b[4].y, LAYER_Y[0]);
        assert_eq!(b[4].rgb, DUST_RGB_LO);
        assert_eq!(b[8].y, LAYER_Y[1]);
        assert_eq!(b[8].rgb, DUST_RGB);
        assert_eq!(b[12].y, LAYER_Y[1]);
        assert_eq!(b[12].rgb, DUST_RGB_LO);
    }
}
