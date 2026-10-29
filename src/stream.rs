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
/// Particle layers: the HEIGHT word rides the high layer, the COLOR word
/// the low one (v5.1 — one carrier hue, words told apart by plane height).
/// A layer is decodable when the particle renders in front of geometry
/// (26.3 reversed-Z hides particles against the sky), so the low layer
/// decodes from the ground, the high one from elevated vantage points.
pub const LAYER_Y: [f64; 2] = [80.0, 120.0];
/// Legacy plane constants (kept for packgen templating of the layer list).
pub const PLANE_HI_Y: f64 = LAYER_Y[1];
pub const PLANE_LO_Y: f64 = LAYER_Y[0];
/// Water color index in the 6x6x6 lattice.
pub const COLOR_WATER: u16 = 216;

/// v5.3 proximity culling: particles never spawn within this radius (blocks,
/// 3D, eye-centered) of a player. MUST stay well below the capture's decode
/// zone (~28..96 blocks — beyond ~96 the fog washes the hue below the strobe
/// threshold and the cell stops decoding; the v5.3.0 64-block cull killed
/// the whole transport live on the bench). Near dots are the biggest, so the
/// eraser mask discs stay small; the dome's near gate paints nothing inside
/// the vanilla chunk bubble anyway.
pub const CULL_RADIUS: f64 = 28.0;

/// v5.3.1 STEALTH far gate: particles never spawn FARTHER than this (blocks,
/// 3D, eye-centered) from a player either. Beyond ~96 blocks the fog-washed
/// hue stops decoding (the capture's un-fog can only stretch so far), so a
/// far dot is PERMANENTLY visible garbage: rendered (force mode survives the
/// client's render-distance clip) but never decoded, hence never erased by
/// the dust-mask pass — exactly the sky specks seen from high vantages.
/// The persistent capture buffers make the disk cumulative: as players move,
/// the zone sweeps the grid and every cell still decodes within seconds.
pub const ZONE_FAR: f64 = 112.0;

/// v5.3.1 view-frustum spawn gate. The client's vertical FOV is a video
/// setting (30..110); the server cannot read it, so the cone assumes the
/// widest setting plus a margin. Dots outside the cone would be rendered
/// (force mode) but never looked at — invisible waste at best, and in
/// sky-blind views (looking down at water: no sky pixels -> the rot channel
/// decays -> decode AND eraser die) they would be plainly visible.
/// The margin keeps the gate safe: every dot that COULD land on someone's
/// screen is spawned; the extra cone is invisible waste only.
pub const ASSUMED_FOV_DEG: f64 = 110.0;
pub const FRUSTUM_MARGIN_DEG: f64 = 8.0;

/// v5.3.1 sky-blind gate: the camera channel is measured from SKY pixels
/// (rot_solve), and its confidence decays once no sky is in view. The eraser
/// shares the same camera, so dots spawned into a sky-blind frustum are
/// never erased. The server knows the player pitch exactly (Bukkit syncs it
/// every tick): treat the view as sky-blind when even the frustum's TOP edge
/// (pitch + ASSUMED_FOV/2 + margin) stays below the horizon. The 47° limit
/// bakes in an 8° terrain-silhouette margin for mountain bowls.
pub const SKY_PITCH_LIMIT_DEG: f32 = 47.0;

/// Cosine of the gate half-cone (ASSUMED_FOV/2 + FRUSTUM_MARGIN).
pub fn cos_half_cone() -> f64 {
    let half = (ASSUMED_FOV_DEG * 0.5 + FRUSTUM_MARGIN_DEG).to_radians();
    half.cos()
}

/// Bukkit yaw/pitch (degrees) -> unit look vector.
/// yaw 0 = +Z (south), 90 = -X (west); pitch -90 = up, +90 = down.
pub fn look_dir(yaw_deg: f32, pitch_deg: f32) -> [f64; 3] {
    let yaw = f64::from(yaw_deg).to_radians();
    let pitch = f64::from(pitch_deg).to_radians();
    let cp = pitch.cos();
    [-yaw.sin() * cp, -pitch.sin(), yaw.cos() * cp]
}

/// v5.3.1 spawn visibility gate for ONE dot against ONE player.
/// A dot is spawned to this player only when the player can actually SEE it
/// AND the client machinery can decode AND erase it:
///   1. sky in view (else the rot channel is dying -> no decode, no erase);
///   2. inside the decode band [CULL_RADIUS, ZONE_FAR] (3D, eye-centered);
///   3. inside the frustum cone (widest-FOV assumption + margin).
///
/// Every spawned dot is therefore on-screen, decodable and erased — the
/// transport is invisible from every vantage, sky-blind views included:
/// there is simply nothing there to see.
pub fn spawn_visible(
    s: &Spawn,
    eye: [f64; 3],
    look: [f64; 3],
    sky_visible: bool,
    cos_half: f64,
) -> bool {
    if !sky_visible {
        return false;
    }
    let dx = s.x - eye[0];
    let dy = s.y - eye[1];
    let dz = s.z - eye[2];
    let d2 = dx * dx + dy * dy + dz * dz;
    if d2 <= CULL_RADIUS * CULL_RADIUS || d2 > ZONE_FAR * ZONE_FAR {
        return false;
    }
    let inv = 1.0 / d2.sqrt();
    (dx * look[0] + dy * look[1] + dz * look[2]) * inv > cos_half
}

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

/// One villager-sparkle particle to spawn this tick (v5.1 carrier: the
/// only particle texture the 26.3 software rasterizers actually draw).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Spawn {
    pub x: f64,
    pub y: f64,
    pub z: f64,
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
    pub fn word_spawns(word: u16, cx: f64, cz: f64, sub: f64, plane_y: f64, out: &mut Vec<Spawn>) {
        for slot in 0..4usize {
            let bits = ((word >> (14 - 2 * slot)) & 3) as f64;
            let dx = (bits - 1.5) * sub;
            let dz = (slot as f64 - 1.5) * sub;
            out.push(Spawn {
                x: cx + dx,
                y: plane_y,
                z: cz + dz,
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
            // v5.1: the HI word rides ONLY the high layer and the LO word
            // ONLY the low layer — the villager-sparkle carrier has one hue,
            // so the capture pass tells words apart by plane height.
            Self::word_spawns(hword, cxw, czw, sub, LAYER_Y[1], &mut self.batch);
            Self::word_spawns(cword, cxw, czw, sub, LAYER_Y[0], &mut self.batch);
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
        Streamer::word_spawns(word, 0.0, 0.0, 1.0, 0.0, &mut out);
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
        // 2048 particles / tick = 256 cells (4 height + 4 color = 8/cell).
        let b: Vec<Spawn> = s.tick_batch(2048).to_vec();
        assert_eq!(b.len(), 2048);
        assert_eq!(s.streamed_cells, 256);
        // Full lap: 16384 cells / 256 per tick = 64 ticks.
        for _ in 0..63 {
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
        // 2 cells x (4 height + 4 color) particles each.
        assert_eq!(b.len(), 16);
        assert_eq!(s.streamed_cells, 2);
        // Water cell streams its color word on the LOW layer (last 4 spawns).
        let lo: Vec<&Spawn> = b.iter().skip(12).collect();
        assert_eq!(lo.len(), 4);
        assert!(lo.iter().all(|p| (p.y - LAYER_Y[0]).abs() < 1e-9));
        // All spawns of the second cell sit at cell (15,15) center.
        let (wx, wz) = s.cell_origin(15, 15);
        for p in lo {
            assert!((p.x - (wx + 2.0)).abs() <= 1.6);
            assert!((p.z - (wz + 2.0)).abs() <= 1.6);
        }
    }

    #[test]
    fn look_dir_bukkit_conventions() {
        // yaw 0, level -> +Z (south).
        let d = look_dir(0.0, 0.0);
        assert!((d[0]).abs() < 1e-9 && (d[1]).abs() < 1e-9 && (d[2] - 1.0).abs() < 1e-9);
        // yaw 90 -> -X (west).
        let d = look_dir(90.0, 0.0);
        assert!((d[0] + 1.0).abs() < 1e-9 && (d[2]).abs() < 1e-9);
        // pitch -90 -> straight up; +90 -> straight down.
        let up = look_dir(0.0, -90.0);
        assert!((up[1] - 1.0).abs() < 1e-9);
        let down = look_dir(0.0, 90.0);
        assert!((down[1] + 1.0).abs() < 1e-9);
        // Unit length at an oblique pose.
        let d = look_dir(37.0, -21.0);
        let len = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
        assert!((len - 1.0).abs() < 1e-9);
    }

    #[test]
    fn spawn_visible_gate_band_and_cone() {
        let cos_half = cos_half_cone();
        // Eye at the origin looking south (+Z); dot 50 blocks ahead: visible.
        let s = Spawn { x: 0.0, y: 80.0, z: 50.0 };
        let eye = [0.0, 80.0, 0.0];
        let look = look_dir(0.0, 0.0);
        assert!(spawn_visible(&s, eye, look, true, cos_half));
        // Same dot BEHIND the camera: outside the cone.
        let s = Spawn { x: 0.0, y: 80.0, z: -50.0 };
        assert!(!spawn_visible(&s, eye, look, true, cos_half));
        // Inside the cone but too close (inside the 28-block cull bubble).
        let s = Spawn { x: 0.0, y: 80.0, z: 20.0 };
        assert!(!spawn_visible(&s, eye, look, true, cos_half));
        // Inside the cone but beyond the 112-block decode zone: permanent
        // undecodable garbage — never spawned.
        let s = Spawn { x: 0.0, y: 80.0, z: 200.0 };
        assert!(!spawn_visible(&s, eye, look, true, cos_half));
        // Sky-blind view: nothing spawns at all, cone or not.
        let s = Spawn { x: 0.0, y: 80.0, z: 50.0 };
        assert!(!spawn_visible(&s, eye, look, false, cos_half));
        // Sideways dot at the cone edge: 90 degrees off the look axis.
        let s = Spawn { x: 50.0, y: 80.0, z: 0.0 };
        assert!(!spawn_visible(&s, eye, look, true, cos_half));
    }

    #[test]
    fn spawn_gate_zone_is_symmetric_3d() {
        // A dot 50 blocks straight BELOW the eye decodes fine when the
        // player looks down at it with sky in view (high vantage, horizon
        // in frame): the band test is 3D, not horizontal.
        let cos_half = cos_half_cone();
        let eye = [0.0, 130.0, 0.0];
        let s = Spawn { x: 0.0, y: 80.0, z: 0.0 };
        let look = look_dir(0.0, 0.0); // level — dots below are OUTSIDE the cone
        assert!(!spawn_visible(&s, eye, look, true, cos_half));
        let look = look_dir(0.0, 40.0); // pitched down 40, sky still in view
        assert!(spawn_visible(&s, eye, look, true, cos_half));
    }

    #[test]
    fn batch_positions_on_sub_lattice() {
        let mut s = Streamer::new(100.0, -40.0, 64, 4);
        s.set_cell(3, 5, cell(70, false, [200, 100, 50]));
        let b: Vec<Spawn> = s.tick_batch(2048).to_vec();
        assert_eq!(b.len(), 8);
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
        }
        // v5.1: first the height word on the HIGH layer, then the color word
        // on the LOW layer (one hue — words are told apart by plane height).
        assert_eq!(b[0].y, LAYER_Y[1]);
        assert_eq!(b[3].y, LAYER_Y[1]);
        assert_eq!(b[4].y, LAYER_Y[0]);
        assert_eq!(b[7].y, LAYER_Y[0]);
    }
}
