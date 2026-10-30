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
//!
//! v5.4 DELTA-ZONE scheduler: the client's buffers are PERSISTENT, so a
//! cell only needs re-streaming when (a) its words CHANGED on the server
//! (the dirty phase), or (b) it is decodable right now — inside the decode
//! band and some player's frustum — and has not been sent for
//! VIEW_REFRESH_TICKS (cone phase for view priority, then the zone sweep
//! row-major). Cells no player can decode were always culled at delivery;
//! now they cost ZERO budget. Steady state with a painted zone: the batch
//! collapses to a few dozen particles per tick — a ~25x idle-traffic cut
//! versus the v5.3.3 constant lap, with edits still propagating in
//! seconds. Passing NO views keeps the legacy full-disk lap (diagnostics,
//! no-player self-heal).

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

/// v5.3.3 cone-priority view hint (the SPEED half of the stealth transport).
/// v5.3.1/v5.3.2 made every spawned dot decodable and erasable, but the
/// batch still walked the disk row-major: after a 180° turn the cells now
/// inside the frustum meet the cursor only half a lap later (~2-8 s of dead
/// sky). The scheduler fixes exactly that: cells inside the decode band AND
/// the frustum cone stream FIRST (stalest first, near-first), so the visible
/// zone refills within seconds of any turn — DH-class subjective fill speed
/// at the same particle budget. The per-player spawn gate keeps filtering
/// the legacy lap untouched (other players, view=None).
#[derive(Clone, Copy, Debug)]
pub struct ViewHint {
    /// Player eye position (world, blocks; by + 1.62).
    pub eye: [f64; 3],
    /// Unit look vector (stream::look_dir of the live yaw/pitch).
    pub look: [f64; 3],
    /// Engine tick of this call (age bookkeeping).
    pub now: u64,
}

/// A decodable cell (cone or zone sweep) is re-streamed at most once per
/// this many ticks — the client's persistent buffers still hold the dots'
/// words, so fresher refreshes are pure waste. This gates the CONE phase;
/// its value bounds the worst-case turn-refill latency (~3 s), the DH-class
/// speed the transport is tuned for.
pub const VIEW_REFRESH_TICKS: u64 = 60;

/// v5.4: the zone sweep's age gate — a much slower background self-heal
/// (packet loss, cleared client buffers). Edits do NOT wait for it (the
/// dirty phase), and turn refills do NOT wait for it (the cone phase).
pub const ZONE_REFRESH_TICKS: u64 = 600;

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
    /// v5.3.3: engine tick each cell was last streamed at (0 = never).
    last_stream: Vec<u64>,
    /// Internal tick counter when no engine tick is supplied.
    tick_counter: u64,
    /// Cached cone cells (near-first) + the eye/look they were built for;
    /// rebuilt only when the eye moves > 8 blocks or the look turns > 5 deg.
    view_cache: Option<([f64; 3], [f64; 3], Vec<u32>)>,
    /// Cone-phase cells streamed since boot (diagnostics).
    pub cone_cells: u64,
    /// v5.4: cells flagged for re-stream (real edits, words differ).
    dirty: Vec<bool>,
    /// FIFO over dirty cells (phase 0 order = edit order).
    dirty_queue: std::collections::VecDeque<u32>,
    /// Cells currently marked dirty (fast skip when 0).
    pub dirty_pending: usize,
    /// Dirty-phase cells streamed since boot (diagnostics).
    pub dirty_streams: u64,
    /// Zone-sweep cells streamed since boot (diagnostics).
    pub zone_cells: u64,
}

impl Streamer {
    pub fn new(origin_x: f64, origin_z: f64, region_blocks: i32, cell_size: i32) -> Self {
        let side = (region_blocks / cell_size).max(1) as u32;
        let n = side as usize * side as usize;
        Streamer {
            origin_x,
            origin_z,
            cell_size,
            side,
            cells: vec![Cell::default(); n],
            cursor: 0,
            batch: Vec::with_capacity(4096),
            streamed_cells: 0,
            laps: 0,
            last_stream: vec![0; n],
            tick_counter: 0,
            view_cache: None,
            cone_cells: 0,
            dirty: vec![false; n],
            dirty_queue: std::collections::VecDeque::new(),
            dirty_pending: 0,
            dirty_streams: 0,
            zone_cells: 0,
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
        let idx = cz as usize * side + cx as usize;
        if let Some(slot) = self.cells.get_mut(idx) {
            // v5.4 delta: a REAL edit (previous cell valid, encoded words
            // differ) must re-stream. The initial scan fill overwrites
            // invalid cells — that is painting, not editing, and the
            // cone/sweep phases own it (marking it dirty would flood phase
            // 0 on every cold boot).
            let prev = *slot;
            *slot = cell;
            if cell.valid && prev.valid {
                let words = Self::encode_words(&cell);
                let old = Self::encode_words(&prev);
                if old != words && !self.dirty[idx] {
                    self.dirty[idx] = true;
                    self.dirty_queue.push_back(idx as u32);
                    self.dirty_pending += 1;
                }
            }
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

    /// Rebuild the cone cache (v5.3.3): every valid cell whose HI or LO
    /// probe point sits inside the decode band [CULL_RADIUS, ZONE_FAR] (3D
    /// from the eye) AND inside the frustum cone. Sorted near-first. Rebuilt
    /// only when the eye moved > 8 blocks or the look turned > 5 deg. The
    /// test is the per-player spawn gate geometry (cos_half_cone), so every
    /// cone-phase spawn passes the gate — no budget is wasted on culled dots.
    fn rebuild_view_cache(&mut self, eye: [f64; 3], look: [f64; 3]) {
        let side = self.side;
        let cell = self.cell_size as f64;
        let cos_half = cos_half_cone();
        let mut list: Vec<(f32, u32)> = Vec::new();
        for cz in 0..side {
            for cx in 0..side {
                let idx = cz * side + cx;
                if !self.cells[idx as usize].valid {
                    continue;
                }
                let (wx, wz) = self.cell_origin(cx, cz);
                let cxw = wx + cell * 0.5;
                let czw = wz + cell * 0.5;
                // The cell always streams HI+LO together, so BOTH plane probe
                // points must sit inside the band and the frustum — otherwise
                // the gate would cull the other plane's spawns as waste.
                let mut all_ok = true;
                let mut best = f64::INFINITY;
                for &py in &LAYER_Y {
                    let dx = cxw - eye[0];
                    let dy = py - eye[1];
                    let dz = czw - eye[2];
                    let d2 = dx * dx + dy * dy + dz * dz;
                    if d2 < best {
                        best = d2;
                    }
                    // Decode band with a 2-block safety margin on both ends
                    // so the sub-cell offsets stay inside it, plus the exact
                    // spawn-gate frustum test.
                    if d2 <= (CULL_RADIUS + 2.0) * (CULL_RADIUS + 2.0)
                        || d2 > (ZONE_FAR - 2.0) * (ZONE_FAR - 2.0)
                    {
                        all_ok = false;
                        break;
                    }
                    let inv = 1.0 / d2.sqrt();
                    if (dx * look[0] + dy * look[1] + dz * look[2]) * inv <= cos_half {
                        all_ok = false;
                        break;
                    }
                }
                if all_ok {
                    list.push((best as f32, idx));
                }
            }
        }
        list.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
        self.view_cache = Some((eye, look, list.into_iter().map(|(_, i)| i).collect()));
    }

    /// Spawn the particle batch of one cell (4 HI-plane + 4 LO-plane spawns).
    fn push_cell_spawns(&mut self, idx: u32, sub: f64, now: u64) {
        let side = self.side as usize;
        let cx = (idx as usize % side) as u32;
        let cz = (idx as usize / side) as u32;
        let Some(cell) = self.get(cx, cz) else {
            return;
        };
        if !cell.valid {
            return;
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
        let idx = idx as usize;
        self.last_stream[idx] = now;
        if self.dirty[idx] {
            self.dirty[idx] = false;
            self.dirty_pending = self.dirty_pending.saturating_sub(1);
        }
        self.streamed_cells += 1;
    }

    /// v5.4: is this cell decodable by ANY of the views right now? The same
    /// geometry as the per-player spawn gate / the cone cache membership:
    /// BOTH plane probe points inside the decode band [CULL_RADIUS+2,
    /// ZONE_FAR-2] and inside the frustum cone. Cells failing this for every
    /// view were always culled at delivery — the scheduler no longer spends
    /// budget on them.
    fn decodable_by_any(&self, idx: usize, views: &[ViewHint]) -> bool {
        let side = self.side as usize;
        let cx = (idx % side) as u32;
        let cz = (idx / side) as u32;
        let (wx, wz) = self.cell_origin(cx, cz);
        let cxw = wx + self.cell_size as f64 * 0.5;
        let czw = wz + self.cell_size as f64 * 0.5;
        let cos_half = cos_half_cone();
        for v in views {
            let mut all_ok = true;
            for &py in &LAYER_Y {
                let dx = cxw - v.eye[0];
                let dy = py - v.eye[1];
                let dz = czw - v.eye[2];
                let d2 = dx * dx + dy * dy + dz * dz;
                if d2 <= (CULL_RADIUS + 2.0) * (CULL_RADIUS + 2.0)
                    || d2 > (ZONE_FAR - 2.0) * (ZONE_FAR - 2.0)
                {
                    all_ok = false;
                    break;
                }
                let inv = 1.0 / d2.sqrt();
                if (dx * v.look[0] + dy * v.look[1] + dz * v.look[2]) * inv <= cos_half {
                    all_ok = false;
                    break;
                }
            }
            if all_ok {
                return true;
            }
        }
        false
    }

    /// Advance the stream by one engine tick. v5.4 DELTA-ZONE phases over
    /// the views slice (empty slice = legacy full-disk lap, diagnostics):
    ///   0. dirty delta — cells whose words changed on the server stream
    ///      first (edit propagation, capped at half the budget);
    ///   1. cone priority (v5.3.3) — decode-band cells inside the FIRST
    ///      view's frustum stream stalest/near-first (turn refill in
    ///      seconds);
    ///   2. zone sweep — the row-major cursor, but only decodable cells
    ///      (band + ANY view's frustum, age-gated) cost budget. Cells no
    ///      player can decode were always culled at delivery; now they cost
    ///      zero. A painted, unchanged zone collapses the batch to the
    ///      VIEW_REFRESH_TICKS trickle.
    pub fn tick_batch(&mut self, budget: usize, views: &[ViewHint]) -> &[Spawn] {
        self.batch.clear();
        let total = self.side as u64 * self.side as u64;
        if total == 0 {
            return &self.batch;
        }
        self.tick_counter = self.tick_counter.wrapping_add(1);
        let now = views.first().map(|v| v.now).unwrap_or(self.tick_counter);
        let sub = self.cell_size as f64 / SUB_PER_CELL;
        let budget = budget.max(8);
        let zone_mode = !views.is_empty();

        // ---- phase 0: dirty delta (v5.4) ------------------------------------
        if zone_mode && self.dirty_pending > 0 {
            let cap = budget / 2;
            let q_len = self.dirty_queue.len();
            for _ in 0..q_len {
                if self.batch.len() >= cap {
                    break;
                }
                let Some(idx_u32) = self.dirty_queue.pop_front() else {
                    break;
                };
                let idx = idx_u32 as usize;
                if !self.dirty[idx] {
                    continue; // already streamed via the cone/sweep phases
                }
                if self.decodable_by_any(idx, views) {
                    self.push_cell_spawns(idx_u32, sub, now);
                    self.dirty_streams += 1;
                } else {
                    // Nobody can decode it now — keep it queued for later.
                    self.dirty_queue.push_back(idx_u32);
                }
            }
        }

        // ---- phase 1: cone priority (v5.3.3) -------------------------------
        if let Some(v) = views.first() {
            let rebuild = match self.view_cache {
                Some((ceye, clook, _)) => {
                    let dx = ceye[0] - v.eye[0];
                    let dz = ceye[2] - v.eye[2];
                    dx * dx + dz * dz > 64.0 || dot3(ceye, clook, v.look) < 0.9962
                    // > ~5 deg
                }
                None => true,
            };
            if rebuild {
                self.rebuild_view_cache(v.eye, v.look);
            }
            let half_budget = budget / 2;
            // Clone the index list: spawning mutates self, the cached slice
            // cannot be borrowed across push_cell_spawns.
            let list: Vec<u32> = self
                .view_cache
                .as_ref()
                .map(|(_, _, l)| l.clone())
                .unwrap_or_default();
            for idx_u32 in list {
                if self.batch.len() >= half_budget {
                    break;
                }
                let idx = idx_u32 as usize;
                if now.wrapping_sub(self.last_stream[idx]) < VIEW_REFRESH_TICKS {
                    continue; // fresh enough — the client still holds it
                }
                self.push_cell_spawns(idx_u32, sub, now);
                self.cone_cells += 1;
            }
        }

        // ---- phase 2: zone sweep (v5.4) / legacy row-major lap -------------
        // zone_mode: only decodable cells cost budget (age-gated like the
        // cone — the client's persistent buffers still hold fresh words).
        // Legacy mode (no views): stream every valid cell (diagnostics,
        // self-heal with nobody online).
        let mut steps = 0u64;
        while self.batch.len() < budget && steps < total {
            let t = self.cursor % total;
            self.cursor += 1;
            steps += 1;
            let idx = t as usize;
            if !self.cells[idx].valid {
                continue;
            }
            if zone_mode {
                if self.last_stream[idx] != 0
                    && now.wrapping_sub(self.last_stream[idx]) < ZONE_REFRESH_TICKS
                {
                    continue; // fresh enough — the client still holds it
                }
                // last_stream == 0: never streamed — always sweep-eligible.
                if !self.decodable_by_any(idx, views) {
                    continue; // no player can decode it — zero budget
                }
                self.zone_cells += 1;
            } else if self.last_stream[idx] == now {
                continue; // already streamed this tick by the cone phase
            }
            self.push_cell_spawns(t as u32, sub, now);
        }
        if self.cursor >= total {
            self.cursor %= total;
            self.laps += 1;
        }
        &self.batch
    }
}

/// Cosine between two unit-ish vectors (look vectors are unit).
fn dot3(_a: [f64; 3], b: [f64; 3], c: [f64; 3]) -> f64 {
    b[0] * c[0] + b[1] * c[1] + b[2] * c[2]
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
        let b: Vec<Spawn> = s.tick_batch(2048, &[]).to_vec();
        assert_eq!(b.len(), 2048);
        assert_eq!(s.streamed_cells, 256);
        // Full lap: 16384 cells / 256 per tick = 64 ticks.
        for _ in 0..63 {
            s.tick_batch(2048, &[]);
        }
        assert_eq!(s.laps, 1);
        assert_eq!(s.streamed_cells, 16384);
        // Cursor wrapped: the next batch starts from cell 0 again.
        let b: Vec<Spawn> = s.tick_batch(16, &[]).to_vec();
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
        let b: Vec<Spawn> = s.tick_batch(2048, &[]).to_vec();
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
        let s = Spawn {
            x: 0.0,
            y: 80.0,
            z: 50.0,
        };
        let eye = [0.0, 80.0, 0.0];
        let look = look_dir(0.0, 0.0);
        assert!(spawn_visible(&s, eye, look, true, cos_half));
        // Same dot BEHIND the camera: outside the cone.
        let s = Spawn {
            x: 0.0,
            y: 80.0,
            z: -50.0,
        };
        assert!(!spawn_visible(&s, eye, look, true, cos_half));
        // Inside the cone but too close (inside the 28-block cull bubble).
        let s = Spawn {
            x: 0.0,
            y: 80.0,
            z: 20.0,
        };
        assert!(!spawn_visible(&s, eye, look, true, cos_half));
        // Inside the cone but beyond the 112-block decode zone: permanent
        // undecodable garbage — never spawned.
        let s = Spawn {
            x: 0.0,
            y: 80.0,
            z: 200.0,
        };
        assert!(!spawn_visible(&s, eye, look, true, cos_half));
        // Sky-blind view: nothing spawns at all, cone or not.
        let s = Spawn {
            x: 0.0,
            y: 80.0,
            z: 50.0,
        };
        assert!(!spawn_visible(&s, eye, look, false, cos_half));
        // Sideways dot at the cone edge: 90 degrees off the look axis.
        let s = Spawn {
            x: 50.0,
            y: 80.0,
            z: 0.0,
        };
        assert!(!spawn_visible(&s, eye, look, true, cos_half));
    }

    #[test]
    fn spawn_gate_zone_is_symmetric_3d() {
        // A dot 50 blocks straight BELOW the eye decodes fine when the
        // player looks down at it with sky in view (high vantage, horizon
        // in frame): the band test is 3D, not horizontal.
        let cos_half = cos_half_cone();
        let eye = [0.0, 130.0, 0.0];
        let s = Spawn {
            x: 0.0,
            y: 80.0,
            z: 0.0,
        };
        let look = look_dir(0.0, 0.0); // level — dots below are OUTSIDE the cone
        assert!(!spawn_visible(&s, eye, look, true, cos_half));
        let look = look_dir(0.0, 40.0); // pitched down 40, sky still in view
        assert!(spawn_visible(&s, eye, look, true, cos_half));
    }

    #[test]
    fn batch_positions_on_sub_lattice() {
        let mut s = Streamer::new(100.0, -40.0, 64, 4);
        s.set_cell(3, 5, cell(70, false, [200, 100, 50]));
        let b: Vec<Spawn> = s.tick_batch(2048, &[]).to_vec();
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

    fn vhint(x: f64, z: f64, yaw_deg: f64, pitch_deg: f64, now: u64) -> ViewHint {
        ViewHint {
            eye: [x, 64.0 + 1.62, z],
            look: look_dir(yaw_deg as f32, pitch_deg as f32),
            now,
        }
    }

    /// v5.3.3: cone-phase spawns precede the legacy lap and stay inside the
    /// decode band + frustum (the per-player gate will pass them).
    #[test]
    fn cone_phase_streams_first_and_in_band() {
        let mut s = Streamer::new(0.0, 0.0, 256, 2);
        for cz in 0..256u32 {
            for cx in 0..256u32 {
                s.set_cell(cx, cz, cell(70, false, [100, 100, 100]));
            }
        }
        // Player at the disk center, looking +Z, level pitch.
        let v = vhint(128.0, 128.0, 0.0, 0.0, 100);
        let b: Vec<Spawn> = s.tick_batch(2048, &[v]).to_vec();
        assert!(!b.is_empty());
        // The cone phase fills at least a quarter of the budget immediately
        // (row-major cells are far away and mostly out of the frustum).
        let half = b.len() / 2;
        assert!(s.cone_cells > 0, "cone phase must stream cells");
        for sp in &b[..half.min(b.len())] {
            // Every cone-phase spawn must pass the EXACT per-player spawn
            // gate geometry (decode band + frustum): the scheduler tested the
            // cell probe points, the 2-block band margin covers the sub-cell
            // offsets, so nothing may fall outside.
            let dx = sp.x - v.eye[0];
            let dy = sp.y - v.eye[1];
            let dz = sp.z - v.eye[2];
            let d2 = dx * dx + dy * dy + dz * dz;
            let inv = 1.0 / d2.sqrt();
            let in_cone =
                (dx * v.look[0] + dy * v.look[1] + dz * v.look[2]) * inv > cos_half_cone() - 0.02;
            let in_band = d2 > (CULL_RADIUS - 4.0) * (CULL_RADIUS - 4.0)
                && d2 <= (ZONE_FAR + 4.0) * (ZONE_FAR + 4.0);
            assert!(
                in_band && in_cone,
                "cone spawn ({},{},{}) outside frustum/band",
                sp.x,
                sp.y,
                sp.z
            );
        }
    }

    /// The age gate: cone cells stream once, then hold for
    /// VIEW_REFRESH_TICKS with an unchanged view.
    #[test]
    fn cone_age_gate_holds_fresh_cells() {
        let mut s = Streamer::new(0.0, 0.0, 256, 2);
        for cz in 0..256u32 {
            for cx in 0..256u32 {
                s.set_cell(cx, cz, cell(70, false, [100, 100, 100]));
            }
        }
        let v = vhint(128.0, 128.0, 0.0, 0.0, 100);
        let first: Vec<Spawn> = s.tick_batch(2048, &[v]).to_vec();
        // Cell identity via floor: sub-cell offsets never cross the border.
        let first_cells: Vec<(i64, i64)> = first
            .chunks(8)
            .map(|c| (c[0].x.floor() as i64, c[0].z.floor() as i64))
            .collect();
        let first_cone: Vec<(i64, i64)> =
            first_cells[..first_cells.len().min(s.cone_cells as usize)].to_vec();
        // Immediate next tick with the SAME view: no cone cell repeats.
        let v2 = vhint(128.0, 128.0, 0.0, 0.0, 101);
        let second: Vec<Spawn> = s.tick_batch(2048, &[v2]).to_vec();
        for sp in second.chunks(8).take(s.cone_cells as usize) {
            let key = (sp[0].x.floor() as i64, sp[0].z.floor() as i64);
            assert!(
                !first_cone.contains(&key),
                "cone cell {:?} re-streamed before VIEW_REFRESH_TICKS",
                key
            );
        }
        // After the age gate the stale cone cells refresh again.
        let v3 = vhint(128.0, 128.0, 0.0, 0.0, 100 + VIEW_REFRESH_TICKS);
        let before = s.cone_cells;
        let third: Vec<Spawn> = s.tick_batch(2048, &[v3]).to_vec();
        let third_cone = &third[..third.len().min((s.cone_cells - before) as usize * 8)];
        assert!(
            third_cone
                .iter()
                .any(|sp| first_cone.contains(&(sp.x.floor() as i64, sp.z.floor() as i64))),
            "stale cone cells must refresh after the age gate"
        );
    }

    /// Turning the camera rebuilds the cache: the new cone streams first.
    #[test]
    fn cone_follows_the_turn() {
        let mut s = Streamer::new(0.0, 0.0, 256, 2);
        for cz in 0..256u32 {
            for cx in 0..256u32 {
                s.set_cell(cx, cz, cell(70, false, [100, 100, 100]));
            }
        }
        // Face +Z first, then turn to -X (vanilla yaw 90).
        let v = vhint(128.0, 128.0, 0.0, 0.0, 10);
        let _ = s.tick_batch(2048, &[v]);
        let v2 = vhint(128.0, 128.0, 90.0, 0.0, 200);
        let b: Vec<Spawn> = s.tick_batch(2048, &[v2]).to_vec();
        assert!(s.cone_cells > 0);
        // Cone-phase spawns of THIS tick (the front of the batch) must lie
        // in the west frustum band (or the near bubble).
        let cone_spawns = (s.cone_cells.min(128) * 8) as usize;
        for sp in &b[..cone_spawns.min(b.len())] {
            let dx = sp.x - v2.eye[0];
            let dy = sp.y - v2.eye[1];
            let dz = sp.z - v2.eye[2];
            let d2 = dx * dx + dy * dy + dz * dz;
            let inv = 1.0 / d2.sqrt();
            let in_cone = (dx * v2.look[0] + dy * v2.look[1] + dz * v2.look[2]) * inv
                > cos_half_cone() - 0.02;
            let in_band = d2 > (CULL_RADIUS - 4.0) * (CULL_RADIUS - 4.0)
                && d2 <= (ZONE_FAR + 4.0) * (ZONE_FAR + 4.0);
            assert!(
                in_band && in_cone,
                "spawn ({},{},{}) not in the west cone band",
                sp.x,
                sp.y,
                sp.z
            );
        }
    }

    /// v5.4 delta: an edited cell (words changed) streams in phase 0 —
    /// ahead of the cone and the sweep — as long as SOME view can decode it.
    #[test]
    fn changed_cell_streams_first() {
        let mut s = Streamer::new(0.0, 0.0, 256, 2);
        for cz in 0..256u32 {
            for cx in 0..256u32 {
                s.set_cell(cx, cz, cell(70, false, [100, 100, 100]));
            }
        }
        let v = vhint(128.0, 128.0, 0.0, 0.0, 100);
        let _ = s.tick_batch(2048, &[v]);
        assert_eq!(s.dirty_pending, 0, "filling the disk never marks dirty");
        // Edit a cell far ahead of the row-major cursor: deep in the
        // frustum band (z ~ 200), inside the decode band. Grid is 128x128
        // cells (region 256 / cell 2), so (66,100) is in, (128,*) would be
        // out of the grid entirely.
        let (ex, ez) = (66u32, 100u32);
        s.set_cell(ex, ez, cell(90, false, [200, 100, 50]));
        assert_eq!(s.dirty_pending, 1);
        // Tiny budget: ONLY the dirty cell may stream (phase 0 cap = 4
        // cells at budget 32, and the dirty cell is first in the queue).
        let v2 = vhint(128.0, 128.0, 0.0, 0.0, 101);
        let b: Vec<Spawn> = s.tick_batch(32, &[v2]).to_vec();
        assert_eq!(s.dirty_pending, 0, "edited cell must stream immediately");
        assert_eq!(s.dirty_streams, 1);
        let (wx, wz) = s.cell_origin(ex, ez);
        assert!(b.len() >= 8);
        // The edit rides the FRONT of the batch (phase 0 first); the rest of
        // the budget still flows to the cone/sweep as usual.
        for sp in &b[..8] {
            assert!(
                (sp.x - (wx + 1.0)).abs() < 2.1 && (sp.z - (wz + 1.0)).abs() < 2.1,
                "dirty cell must be the first batch: spawn at {},{} vs cell {} {}",
                sp.x,
                sp.z,
                wx,
                wz
            );
        }
    }

    /// v5.4 delta: rewriting a cell with the SAME encoded words must NOT
    /// mark it dirty (idempotent re-scans are free); the INITIAL fill of an
    /// invalid cell is painting, not editing — the cone/sweep own it.
    #[test]
    fn unchanged_rewrites_stay_clean() {
        let mut s = Streamer::new(0.0, 0.0, 64, 4);
        s.set_cell(0, 0, cell(70, false, [100, 100, 100]));
        assert_eq!(s.dirty_pending, 0, "initial fill is painting, not editing");
        // Same words again: still clean.
        s.set_cell(0, 0, cell(70, false, [100, 100, 100]));
        assert_eq!(s.dirty_pending, 0);
        // Real edit (height changed on a live cell) -> dirty.
        s.set_cell(0, 0, cell(90, false, [100, 100, 100]));
        assert_eq!(s.dirty_pending, 1);
        // Stream it (legacy lap): dirty cleared.
        let _ = s.tick_batch(2048, &[]);
        assert_eq!(s.dirty_pending, 0);
        // Rewrite with the same words AFTER the send: stays clean.
        s.set_cell(0, 0, cell(90, false, [100, 100, 100]));
        assert_eq!(s.dirty_pending, 0, "identical words must not re-queue");
        // And a real change re-queues it.
        s.set_cell(0, 0, cell(90, true, [100, 100, 100]));
        assert_eq!(s.dirty_pending, 1);
    }

    /// v5.4 zone sweep: with a live view the batch NEVER leaves the
    /// decodable set (band + frustum) — out-of-zone cells cost zero budget,
    /// so a painted zone collapses to the refresh trickle.
    #[test]
    fn zone_sweep_spends_budget_only_on_decodable_cells() {
        let mut s = Streamer::new(0.0, 0.0, 256, 2);
        for cz in 0..256u32 {
            for cx in 0..256u32 {
                s.set_cell(cx, cz, cell(70, false, [100, 100, 100]));
            }
        }
        // Warm up: 50 ticks fully paint the decodable set (the cone list is
        // ~4k cells at 128 cells/tick, done by ~tick 135 of this window).
        for tick in 0..50u64 {
            let vt = vhint(128.0, 128.0, 0.0, 0.0, 100 + tick);
            let b: Vec<Spawn> = s.tick_batch(2048, &[vt]).to_vec();
            for sp in &b {
                let dx = sp.x - vt.eye[0];
                let dy = sp.y - vt.eye[1];
                let dz = sp.z - vt.eye[2];
                let d2 = dx * dx + dy * dy + dz * dz;
                let inv = 1.0 / d2.sqrt();
                let in_cone = (dx * vt.look[0] + dy * vt.look[1] + dz * vt.look[2]) * inv
                    > cos_half_cone() - 0.02;
                let in_band = d2 > (CULL_RADIUS - 4.0) * (CULL_RADIUS - 4.0)
                    && d2 <= (ZONE_FAR + 4.0) * (ZONE_FAR + 4.0);
                assert!(
                    in_band && in_cone,
                    "tick {tick}: spawn ({},{},{}) outside every decodable zone",
                    sp.x,
                    sp.y,
                    sp.z
                );
            }
        }
        // Steady state (v5.4 DELTA-ZONE): the painted, unchanged zone
        // collapses the batch to ZERO — cone cells age < 60, sweep cells
        // age < 600, no dirty cells. Zero waste.
        let v_idle = vhint(128.0, 128.0, 0.0, 0.0, 150);
        let b: Vec<Spawn> = s.tick_batch(2048, &[v_idle]).to_vec();
        assert!(b.is_empty(), "idle zone must collapse to zero spawns");
        // The refresh wave: cone cells painted at tick 100 come due at 160.
        let v_due = vhint(128.0, 128.0, 0.0, 0.0, 161);
        let b: Vec<Spawn> = s.tick_batch(2048, &[v_due]).to_vec();
        assert!(!b.is_empty(), "due cone cells must refresh");
        for sp in &b {
            let dx = sp.x - v_due.eye[0];
            let dy = sp.y - v_due.eye[1];
            let dz = sp.z - v_due.eye[2];
            let d2 = dx * dx + dy * dy + dz * dz;
            let inv = 1.0 / d2.sqrt();
            let in_cone = (dx * v_due.look[0] + dy * v_due.look[1] + dz * v_due.look[2]) * inv
                > cos_half_cone() - 0.02;
            let in_band = d2 > (CULL_RADIUS - 4.0) * (CULL_RADIUS - 4.0)
                && d2 <= (ZONE_FAR + 4.0) * (ZONE_FAR + 4.0);
            assert!(in_band && in_cone, "refresh spawn outside the zone");
        }
    }

    /// v5.4 delta: a dirty cell NOBODY can decode waits in the queue and
    /// streams the moment a view that covers it appears.
    #[test]
    fn dirty_cell_out_of_zone_waits_for_a_view() {
        let mut s = Streamer::new(0.0, 0.0, 256, 2);
        for cz in 0..256u32 {
            for cx in 0..256u32 {
                s.set_cell(cx, cz, cell(70, false, [100, 100, 100]));
            }
        }
        let v = vhint(128.0, 128.0, 0.0, 0.0, 100);
        let _ = s.tick_batch(2048, &[v]);
        // Edit BEHIND the player's back (north, -Z): not decodable now.
        // (66,20) sits ~88 blocks behind the eye — valid grid cell.
        s.set_cell(66, 20, cell(95, false, [200, 50, 50]));
        assert_eq!(s.dirty_pending, 1);
        let v2 = vhint(128.0, 128.0, 0.0, 0.0, 101);
        let before = s.dirty_streams;
        let _ = s.tick_batch(2048, &[v2]);
        assert_eq!(
            s.dirty_streams, before,
            "out-of-zone dirty cell must wait (budget never wasted)"
        );
        assert_eq!(s.dirty_pending, 1, "still queued");
        // Turn 180 degrees (vanilla yaw 180 = -Z): now decodable -> phase 0.
        let v3 = vhint(128.0, 128.0, 180.0, 0.0, 102);
        let b: Vec<Spawn> = s.tick_batch(2048, &[v3]).to_vec();
        assert_eq!(s.dirty_streams, before + 1);
        assert_eq!(s.dirty_pending, 0);
        let (wx, wz) = s.cell_origin(66, 20);
        assert!(b
            .iter()
            .any(|sp| (sp.x - (wx + 1.0)).abs() < 2.1 && (sp.z - (wz + 1.0)).abs() < 2.1));
    }
}
