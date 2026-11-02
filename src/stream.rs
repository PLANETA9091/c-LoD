//! The streaming core: SKY-GRID BIT-STREAM (v5.0) + LIVE DELTA CHANNEL (v5.3).
//!
//! Every byte of LOD data travels to the vanilla client as DUST PARTICLES
//! spawned above the scan disk — no map carriers, no offhand item, no
//! textures in the pack at all.
//!
//! Two particle planes float over the disk:
//!   HI (Y=120) — the 8-bit height word of the cell (h + 64, clamped 0..255);
//!   LO (Y=80) — the 8-bit color index of the cell (6x6x6 RGB lattice
//!   0..215, water = 216) in the HIGH byte of the word.
//!
//! One 16-bit word encodes EIGHT data bits as FOUR particles:
//!   particle slot s (0..3, Z axis) carries bits (14 - 2*s) .. (15 - 2*s);
//!   its X sub-position inside the cell IS the 2-bit value, its Z
//!   sub-position IS the slot. The sub-cell quantum is cell/4, centered.
//!   (The low byte of the word stays zero — the decoder reads the high
//!   byte only, keeping the codec dead simple and symmetric.)
//!
//! Server-side pacing: `tick_batch_for()` returns the next chunk of spawns
//! (at most `particles_per_tick`), in two phases:
//!
//!   1. DELTA phase — cells whose words CHANGED since they last streamed
//!      (set_cell_checked detects the change; the engine's player-centric
//!      rescan sweep feeds it). Changed cells ride at the HEAD of the
//!      batch, nearest player first, so a block edit lands on the client
//!      within one engine tick of its detection — no relog, no repack.
//!   2. REFRESH phase — the classic row-major lap cursor over the whole
//!      disk (self-heal: particles the client missed get another chance
//!      every lap).
//!
//! The client-side decoder (`assets/glsl/capture.fsh`) re-projects all 16
//! sub-cell positions of a cell onto the screen, strobes them against the
//! depth buffer (dust renders on the OPAQUE layer and writes depth; sky
//! reads 1.0), and merges decoded words into persistent ping-pong
//! targets. v5.3 merge rule: when ALL FOUR slots of a word strobe in one
//! frame, the byte is REPLACED atomically (exact delta updates); partial
//! strobes OR-merge as before. A particle that never made it through the
//! strobe simply keeps the previous value — the stream self-heals on the
//! next lap.

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

/// Outcome of a checked cell write (v5.3 delta accounting).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CellChange {
    /// The encoded words differ from the previous VALID content (or the
    /// cell became valid). Only changed cells are worth particles.
    pub changed: bool,
    /// The change is OR-merge-exact on the client: every bit set in the
    /// old words is also set in the new ones, so `old | new == new` and
    /// even a partial strobe converges. Bit-drop changes (height
    /// decreases, most color flips) need a full-strobe frame (atomic
    /// replace in the v5.3 capture shader) or a rebuild pulse.
    pub or_safe: bool,
}

impl CellChange {
    const UNCHANGED: CellChange = CellChange {
        changed: false,
        or_safe: true,
    };
}

/// The whole streaming state (server side): one fixed disk of cells plus
/// the phase cursors of the particle stream (refresh lap + delta queue +
/// rescan cursor).
pub struct Streamer {
    /// World-space corner (min x/z) of the cell grid.
    pub origin_x: f64,
    pub origin_z: f64,
    pub cell_size: i32,
    /// Grid side in cells (region_blocks / cell_size).
    pub side: u32,
    /// side x side cells, row-major z outer.
    cells: Vec<Cell>,
    /// Next cell offset (0..side^2) to stream in the refresh lap.
    cursor: u64,
    /// Pre-allocated spawn batch for the current tick.
    batch: Vec<Spawn>,
    /// Cells streamed since boot (diagnostics, both phases).
    pub streamed_cells: u64,
    /// Laps completed over the whole disk (refresh cursor only).
    pub laps: u64,
    /// v5.3: cells whose words changed and have not streamed since.
    dirty: Vec<bool>,
    /// FIFO of dirty cell offsets (duplicates impossible — the flag gates).
    dirty_queue: std::collections::VecDeque<u32>,
    /// Cells delivered via the delta channel since boot.
    pub delta_cells: u64,
    /// v5.3 rescan cursor: row-major over the disk; the engine re-samples
    /// the valid cells it returns near players (skips are free).
    rescan_cursor: u64,
    /// Cells whose words changed since the last rebuild pulse (capped
    /// FIFO). A pulse clears the client's capture buffers, so these cells
    /// must be re-delivered at the delta head afterwards.
    recent_changes: std::collections::VecDeque<u32>,
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
            dirty: vec![false; n],
            dirty_queue: std::collections::VecDeque::new(),
            delta_cells: 0,
            rescan_cursor: 0,
            recent_changes: std::collections::VecDeque::new(),
        }
    }

    pub fn cells(&self) -> u64 {
        (self.side as u64) * (self.side as u64)
    }

    pub fn valid_cells(&self) -> u64 {
        self.cells.iter().filter(|c| c.valid).count() as u64
    }

    /// Number of cells waiting in the delta queue (diagnostics).
    pub fn dirty_now(&self) -> usize {
        self.dirty_queue.len()
    }

    pub fn get(&self, cx: u32, cz: u32) -> Option<Cell> {
        let side = self.side as usize;
        self.cells.get(cz as usize * side + cx as usize).copied()
    }

    /// Unchecked write used by tests and first-scan paths: writes the cell
    /// and marks a delta when the words changed against valid content.
    pub fn set_cell(&mut self, cx: u32, cz: u32, cell: Cell) {
        let _ = self.set_cell_checked(cx, cz, cell);
    }

    /// Checked write: detects whether the ENCODED words changed and, when
    /// they did, queues the cell for the delta channel (batch head).
    pub fn set_cell_checked(&mut self, cx: u32, cz: u32, cell: Cell) -> CellChange {
        let side = self.side as usize;
        if cx as usize >= side || cz as usize >= side {
            return CellChange::UNCHANGED;
        }
        let idx = cz as usize * side + cx as usize;
        let old = self.cells[idx];
        // Always persist the content (first scans must land in the store);
        // only valid->valid word CHANGES mark the delta channel.
        let change = match (old.valid, cell.valid) {
            // First scan (invalid -> valid) belongs to the REFRESH LAP: the
            // initial fill is budget-bound anyway, and delta-marking every
            // fresh cell would double-deliver it (delta + lap) at 2x the
            // network cost during boot. Only valid->valid word changes ride
            // the delta channel.
            (false, false) | (false, true) | (true, false) => CellChange::UNCHANGED,
            (true, true) => {
                let (oh, oc) = Self::encode_words(&old);
                let (nh, nc) = Self::encode_words(&cell);
                if oh == nh && oc == nc {
                    CellChange::UNCHANGED
                } else {
                    CellChange {
                        changed: true,
                        or_safe: (oh & !nh) == 0 && (oc & !nc) == 0,
                    }
                }
            }
        };
        // Persist the content unconditionally (identical writes are
        // idempotent 16-byte copies); only valid->valid word CHANGES mark
        // the delta channel below.
        self.cells[idx] = cell;
        if change.changed {
            if !self.dirty[idx] {
                self.dirty[idx] = true;
                self.dirty_queue.push_back(idx as u32);
            }
            self.recent_changes.push_back(idx as u32);
            while self.recent_changes.len() > 8192 {
                self.recent_changes.pop_front();
            }
        }
        change
    }

    /// Manually queue a cell for the delta channel (e.g. re-delivery after
    /// a rebuild pulse cleared the client's capture buffers).
    pub fn mark_dirty(&mut self, cx: u32, cz: u32) {
        let side = self.side as usize;
        if cx as usize >= side || cz as usize >= side {
            return;
        }
        let idx = cz as usize * side + cx as usize;
        if !self.dirty[idx] {
            self.dirty[idx] = true;
            self.dirty_queue.push_back(idx as u32);
        }
    }

    /// Re-queue every cell changed since the last call: after a rebuild
    /// pulse wipes the client's capture targets, the changed cells must
    /// stream again (the refresh lap refills the rest over one lap).
    /// Returns the number of cells re-queued.
    pub fn redirty_recent(&mut self) -> usize {
        let recent: Vec<u32> = self.recent_changes.drain(..).collect();
        let mut n = 0usize;
        for idx in recent {
            let was = self.dirty[idx as usize];
            let cx = idx % self.side;
            let cz = idx / self.side;
            self.mark_dirty(cx, cz);
            n += usize::from(!was);
        }
        n
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

    /// Advance the stream by one engine tick with NO player anchors:
    /// delta phase first (insertion order), then the refresh lap.
    /// (Test convenience wrapper; the engine always passes anchors.)
    #[cfg(test)]
    pub fn tick_batch(&mut self, budget: usize) -> &[Spawn] {
        self.tick_batch_for(budget, &[])
    }

    /// Advance the stream by one engine tick: fill the batch with at most
    /// `budget` dust spawns. Phase 1 drains the DELTA queue (changed cells,
    /// nearest to any player first — `players` holds world x/z); phase 2
    /// continues the row-major REFRESH lap over valid cells.
    pub fn tick_batch_for(&mut self, budget: usize, players: &[(f64, f64)]) -> &[Spawn] {
        self.batch.clear();
        let total = self.side as u64 * self.side as u64;
        if total == 0 {
            return &self.batch;
        }
        let sub = self.cell_size as f64 / SUB_PER_CELL;
        let budget = budget.max(8);
        // ---- phase 1: delta channel (changed cells, distance-ordered) ----
        if !self.dirty_queue.is_empty() {
            let mut drained: Vec<u32> = self.dirty_queue.drain(..).collect();
            for &idx in &drained {
                self.dirty[idx as usize] = false;
            }
            if !players.is_empty() {
                drained.sort_by_key(|&idx| {
                    let cx = (idx % self.side) as f64;
                    let cz = (idx / self.side) as f64;
                    let (wx, wz) = self.cell_origin(cx as u32, cz as u32);
                    let ccx = wx + self.cell_size as f64 * 0.5;
                    let ccz = wz + self.cell_size as f64 * 0.5;
                    players
                        .iter()
                        .map(|(px, pz)| {
                            let dx = ccx - px;
                            let dz = ccz - pz;
                            (dx * dx + dz * dz) as i64
                        })
                        .min()
                        .unwrap_or(i64::MAX)
                });
            }
            let mut requeue: Vec<u32> = Vec::new();
            for idx in drained {
                if self.batch.len() >= budget {
                    // Budget exhausted: the remainder keeps delta priority
                    // over the refresh lap on the next tick.
                    requeue.push(idx);
                    continue;
                }
                let cx = idx % self.side;
                let cz = idx / self.side;
                let Some(cell) = self.get(cx, cz) else {
                    continue;
                };
                if !cell.valid {
                    continue; // changed then invalidated — nothing to send
                }
                self.emit_cell(cell, cx, cz, sub);
                self.delta_cells += 1;
            }
            // Requeue preserving the (distance) order: push_front in reverse.
            for &idx in requeue.iter().rev() {
                self.dirty[idx as usize] = true;
                self.dirty_queue.push_front(idx);
            }
        }
        // ---- phase 2: refresh lap (row-major, valid cells only) ----
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
            self.emit_cell(cell, cx, cz, sub);
            // streamed_cells counted inside emit_cell (delta + refresh).
        }
        if self.cursor >= total {
            self.cursor %= total;
            self.laps += 1;
        }
        &self.batch
    }

    /// Emit one cell's two words (HI on the high plane, LO on the low).
    fn emit_cell(&mut self, cell: Cell, cx: u32, cz: u32, sub: f64) {
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

    /// v5.3 rescan targets: advance a row-major cursor over the disk and
    /// return up to `budget` VALID cells within `radius_blocks` of ANY
    /// player (skipped cells cost no samples — only the cursor moves, up
    /// to `max_skips` positions per call). The engine re-samples each
    /// returned cell: world edits near players become deltas without any
    /// event listener. The cursor keeps its position across calls, so
    /// repeated calls sweep the whole (player window x disk) fairly.
    pub fn rescan_targets(
        &mut self,
        players: &[(f64, f64)],
        radius_blocks: f64,
        budget: usize,
        max_skips: usize,
    ) -> Vec<(u32, u32)> {
        let total = self.side as u64 * self.side as u64;
        let mut out = Vec::new();
        if total == 0 || players.is_empty() || budget == 0 {
            return out;
        }
        let r2 = radius_blocks * radius_blocks;
        let mut skips = 0usize;
        let mut steps = 0u64;
        while out.len() < budget && steps < total && skips <= max_skips {
            let t = self.rescan_cursor % total;
            self.rescan_cursor += 1;
            steps += 1;
            let cx = (t % self.side as u64) as u32;
            let cz = (t / self.side as u64) as u32;
            let (wx, wz) = self.cell_origin(cx, cz);
            let ccx = wx + self.cell_size as f64 * 0.5;
            let ccz = wz + self.cell_size as f64 * 0.5;
            let near = players.iter().any(|(px, pz)| {
                let dx = ccx - px;
                let dz = ccz - pz;
                dx * dx + dz * dz <= r2
            });
            if !near {
                skips += 1;
                continue;
            }
            if self.get(cx, cz).map(|c| c.valid) != Some(true) {
                skips += 1;
                continue; // never-scanned cells belong to the scan sweep
            }
            out.push((cx, cz));
        }
        out
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
        // First scan never dirties: the refresh lap owns initial delivery.
        assert_eq!(s.dirty_now(), 0);
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

    // ---- v5.3: change detection (delta channel) ----

    #[test]
    fn change_detection_words_or_noise() {
        let mut s = Streamer::new(0.0, 0.0, 64, 4);
        // First scan: the refresh lap owns initial delivery — no delta.
        let ch = s.set_cell_checked(2, 3, cell(70, false, [90, 90, 90]));
        assert!(!ch.changed);
        assert_eq!(s.dirty_now(), 0);
        // Identical re-sample (within the same encoded byte): no delta.
        let ch = s.set_cell_checked(2, 3, cell(70, false, [90, 90, 90]));
        assert_eq!(ch, CellChange::UNCHANGED);
        assert_eq!(s.dirty_now(), 0, "identical words cost no traffic");
        // Height jitter inside one byte quantum — still no delta.
        let ch = s.set_cell_checked(2, 3, cell(70, false, [90, 90, 90]));
        assert!(!ch.changed);
        // Height increase that keeps every old bit: changed AND or-safe.
        let ch = s.set_cell_checked(2, 3, cell(71, false, [90, 90, 90]));
        assert!(ch.changed);
        // byte(70)=134=0b10000110 -> byte(71)=135=0b10000111: pure bit-add.
        assert!(ch.or_safe, "single +1 on an even byte is OR-exact");
        assert_eq!(s.dirty_now(), 1);
        // Height DECREASE drops bits: changed, NOT or-safe.
        let ch = s.set_cell_checked(2, 3, cell(70, false, [90, 90, 90]));
        assert!(ch.changed && !ch.or_safe);
        // Color flip (water): changed, not or-safe (216 vs arbitrary index).
        let ch = s.set_cell_checked(2, 3, cell(70, true, [0, 0, 0]));
        assert!(ch.changed && !ch.or_safe);
        // Still only ONE dirty entry for the same cell (flag dedups).
        assert_eq!(s.dirty_now(), 1);
    }

    #[test]
    fn delta_streams_before_refresh_lap() {
        let mut s = Streamer::new(0.0, 0.0, 64, 4);
        for cz in 0..16u32 {
            for cx in 0..16u32 {
                s.set_cell(cx, cz, cell(64, false, [90, 90, 90]));
            }
        }
        // Burn most of a lap so the cursor sits mid-disk.
        for _ in 0..28 {
            s.tick_batch(64); // 64 particles = 8 cells/tick, 28 ticks = 224 cells
        }
        assert_eq!(s.streamed_cells, 224);
        // An edit lands on cell (0,0) — row-major BEHIND the cursor.
        s.set_cell_checked(0, 0, cell(90, false, [90, 90, 90]));
        assert_eq!(s.dirty_now(), 1);
        // Next tick: the delta must lead the batch (head), not wait a lap.
        let b: Vec<Spawn> = s.tick_batch(64).to_vec();
        assert_eq!(b.len(), 64);
        let (wx, wz) = s.cell_origin(0, 0);
        // First 8 spawns = the edited cell (4 HI + 4 LO).
        for p in &b[..8] {
            assert!((p.x - (wx + 2.0)).abs() <= 1.6);
            assert!((p.z - (wz + 2.0)).abs() <= 1.6);
        }
        // The edited cell streamed by DELTA, so the refresh cursor must not
        // re-deliver it this lap — but it will on later laps (self-heal).
        assert_eq!(s.delta_cells, 1);
        assert_eq!(s.dirty_now(), 0);
    }

    #[test]
    fn delta_order_nearest_player_first() {
        let mut s = Streamer::new(0.0, 0.0, 128, 2); // 64x64 cells
        for cz in 0..64u32 {
            for cx in 0..64u32 {
                s.set_cell(cx, cz, cell(64, false, [90, 90, 90]));
            }
        }
        // Three edits: near player A (10,10), mid (30,30), far (60,60).
        s.set_cell_checked(60, 60, cell(80, false, [90, 90, 90]));
        s.set_cell_checked(10, 10, cell(80, false, [90, 90, 90]));
        s.set_cell_checked(30, 30, cell(80, false, [90, 90, 90]));
        assert_eq!(s.dirty_now(), 3);
        // Player stands at (10,10)-ish; budget covers only 2 cells (16 spawns).
        let players = vec![(21.0f64, 21.0f64)];
        let b: Vec<Spawn> = s.tick_batch_for(16, &players).to_vec();
        assert_eq!(b.len(), 16);
        // First 8 = cell (10,10) (nearest), next 8 = cell (30,30).
        let mid = &b[8];
        let (mx, mz) = s.cell_origin(30, 30);
        assert!((mid.x - (mx + 1.0)).abs() <= 1.5);
        assert!((mid.z - (mz + 1.0)).abs() <= 1.5);
        // The far cell stays queued for the next tick (priority preserved).
        assert_eq!(s.dirty_now(), 1);
        let b2: Vec<Spawn> = s.tick_batch_for(16, &players).to_vec();
        let far = &b2[0];
        let (fx, fz) = s.cell_origin(60, 60);
        assert!((far.x - (fx + 1.0)).abs() <= 1.5);
        assert!((far.z - (fz + 1.0)).abs() <= 1.5);
        assert_eq!(s.dirty_now(), 0);
    }

    #[test]
    fn delta_overflow_requeues_tail_next_tick() {
        let mut s = Streamer::new(0.0, 0.0, 64, 4);
        for cz in 0..16u32 {
            for cx in 0..16u32 {
                s.set_cell(cx, cz, cell(64, false, [90, 90, 90]));
            }
        }
        // 3 dirty cells, budget only covers 1 cell (8 particles).
        s.set_cell_checked(0, 0, cell(70, false, [90, 90, 90]));
        s.set_cell_checked(1, 0, cell(70, false, [90, 90, 90]));
        s.set_cell_checked(2, 0, cell(70, false, [90, 90, 90]));
        assert_eq!(s.dirty_now(), 3);
        let b: Vec<Spawn> = s.tick_batch_for(8, &[]).to_vec();
        assert_eq!(b.len(), 8, "budget exhausted by the first delta cell");
        assert_eq!(s.dirty_now(), 2, "remainder re-queued at the front");
        assert_eq!(s.delta_cells, 1);
        // No refresh-lap cells snuck in behind the delta.
        let (wx, _wz) = s.cell_origin(0, 0);
        assert!((b[0].x - (wx + 2.0)).abs() <= 1.6);
    }

    #[test]
    fn redirty_recent_requeues_changed_cells() {
        let mut s = Streamer::new(0.0, 0.0, 64, 4);
        s.set_cell(0, 0, cell(64, false, [90, 90, 90]));
        s.set_cell(1, 0, cell(64, false, [90, 90, 90]));
        s.set_cell(2, 0, cell(64, false, [90, 90, 90]));
        let _ = s.tick_batch(64).to_vec(); // initial delivery
        assert_eq!(s.dirty_now(), 0);
        // Two live edits (deltas stream on the next tick).
        s.set_cell_checked(0, 0, cell(70, false, [90, 90, 90]));
        s.set_cell_checked(1, 0, cell(70, false, [90, 90, 90]));
        assert_eq!(s.dirty_now(), 2);
        let _ = s.tick_batch(64).to_vec(); // deltas drained
        assert_eq!(s.dirty_now(), 0);
        // Rebuild pulse: re-queue exactly the changed cells for re-delivery.
        assert_eq!(s.redirty_recent(), 2);
        assert_eq!(s.dirty_now(), 2);
        // Idempotent right after a pulse: the ledger was drained, the two
        // cells stay queued for delivery.
        assert_eq!(s.redirty_recent(), 0);
        assert_eq!(s.dirty_now(), 2);
        let _ = s.tick_batch(64).to_vec();
        assert_eq!(s.dirty_now(), 0);
    }

    #[test]
    fn mark_dirty_requeues_after_stream() {
        let mut s = Streamer::new(0.0, 0.0, 64, 4);
        s.set_cell(0, 0, cell(64, false, [90, 90, 90]));
        let _ = s.tick_batch(64).to_vec(); // initial delivery (refresh lap)
        s.mark_dirty(0, 0);
        assert_eq!(s.dirty_now(), 1);
        let b: Vec<Spawn> = s.tick_batch(8).to_vec();
        assert_eq!(b.len(), 8);
        let (wx, _wz) = s.cell_origin(0, 0);
        assert!((b[0].x - (wx + 2.0)).abs() <= 1.6);
        assert_eq!(s.dirty_now(), 0);
        // mark_dirty is idempotent while already queued.
        s.mark_dirty(0, 0);
        s.mark_dirty(0, 0);
        assert_eq!(s.dirty_now(), 1);
    }

    // ---- v5.3: rescan targets (live event push sampling) ----

    #[test]
    fn rescan_targets_radius_gate_and_budget() {
        let mut s = Streamer::new(0.0, 0.0, 64, 4); // 16x16 cells of 4 blocks
        for cz in 0..16u32 {
            for cx in 0..16u32 {
                s.set_cell(cx, cz, cell(64, false, [90, 90, 90]));
            }
        }
        // Player at the disk center: radius 10 blocks -> cells whose centers
        // are within 10 blocks: a 5x5-ish blob (centers at 2+4k).
        let players = vec![(32.0f64, 32.0f64)];
        let picked = s.rescan_targets(&players, 10.0, 3, 4096);
        assert_eq!(picked.len(), 3, "budget bounds the sample count");
        for (cx, cz) in &picked {
            let (wx, wz) = s.cell_origin(*cx, *cz);
            let dx = wx + 2.0 - 32.0;
            let dz = wz + 2.0 - 32.0;
            assert!(dx * dx + dz * dz <= 100.0, "picked cell out of radius");
        }
    }

    #[test]
    fn rescan_targets_sweeps_cursor_across_calls() {
        let mut s = Streamer::new(0.0, 0.0, 64, 4);
        for cz in 0..16u32 {
            for cx in 0..16u32 {
                s.set_cell(cx, cz, cell(64, false, [90, 90, 90]));
            }
        }
        // Radius covers the whole tiny disk; three calls must return
        // DISJOINT cells advancing the cursor (fair sweep, no repeats
        // until a full wrap).
        let players = vec![(32.0f64, 32.0f64)];
        let mut seen = std::collections::HashSet::new();
        for _ in 0..3 {
            for c in s.rescan_targets(&players, 200.0, 4, 4096) {
                assert!(seen.insert(c), "cell {c:?} sampled twice in one sweep");
            }
        }
        assert_eq!(seen.len(), 12);
    }

    #[test]
    fn rescan_targets_skips_void_cells_free() {
        let mut s = Streamer::new(0.0, 0.0, 64, 4);
        // Only ONE valid cell on the disk.
        s.set_cell(7, 9, cell(64, false, [90, 90, 90]));
        let players = vec![(30.0f64, 38.0f64)]; // inside cell (7,9)
        let picked = s.rescan_targets(&players, 64.0, 5, 4096);
        assert_eq!(picked, vec![(7, 9)], "void cells cost skips, not samples");
        // Every sweep re-offers the same valid cell (that IS the re-check).
        let picked2 = s.rescan_targets(&players, 64.0, 5, 4096);
        assert_eq!(picked2, vec![(7, 9)]);
    }

    #[test]
    fn rescan_targets_no_players_no_work() {
        let mut s = Streamer::new(0.0, 0.0, 64, 4);
        s.set_cell(1, 1, cell(64, false, [90, 90, 90]));
        assert!(s.rescan_targets(&[], 64.0, 5, 4096).is_empty());
        let players = vec![(9999.0f64, 9999.0f64)];
        // Nothing within radius anywhere: the sweep is bounded by skips.
        assert!(s.rescan_targets(&players, 16.0, 5, 64).is_empty());
    }

    #[test]
    fn rescan_targets_multi_player_window() {
        let mut s = Streamer::new(0.0, 0.0, 128, 2); // 64x64 cells of 2 blocks
        for cz in 0..64u32 {
            for cx in 0..64u32 {
                s.set_cell(cx, cz, cell(64, false, [90, 90, 90]));
            }
        }
        // Two players far apart: BOTH windows feed the same budget.
        let players = vec![(5.0f64, 5.0f64), (120.0f64, 120.0f64)];
        let picked = s.rescan_targets(&players, 12.0, 8, 4096);
        assert_eq!(picked.len(), 8);
        let near_a = picked
            .iter()
            .filter(|(cx, cz)| {
                let (wx, wz) = s.cell_origin(*cx, *cz);
                (wx + 1.0 - 5.0).powi(2) + (wz + 1.0 - 5.0).powi(2) <= 144.0
            })
            .count();
        let near_b = picked
            .iter()
            .filter(|(cx, cz)| {
                let (wx, wz) = s.cell_origin(*cx, *cz);
                (wx + 1.0 - 120.0).powi(2) + (wz + 1.0 - 120.0).powi(2) <= 144.0
            })
            .count();
        assert_eq!(near_a + near_b, 8, "every pick belongs to a player window");
    }
}
