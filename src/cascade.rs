//! v6.59 THE CASCADE — real multi-scale LoDs out to MILLIONS of blocks.
//!
//! Ring 0 is the v5 sky-grid disk (region_blocks wide, cell_size cells).
//! The cascade adds six coarser rings behind it, each ring quadrupling the
//! cell size (16 m -> 16 km) and the covered disk (4 km -> 4.2 Mm across,
//! ±2.09 Mm = over two million blocks in every direction):
//!
//!   ring idx 0..5: cell = 16 m * 4^idx, disk = 256 cells wide, all
//!   centered on the ring-0 disk origin (GridInfo.xy);
//!   (continuation)
//!
//! TRANSPORT (the hard part — the decode band is [28, 112] m 3D from the
//! eye, so a world-anchored far cell can never stream): the CASCADE
//! SCAFFOLD. A 64x64 grid of 2 m transport slots anchored to the player's
//! eye (floor(eye / 128) * 128 — both sides compute the same anchor from
//! CameraBlockPos). Slot (i, j) carries ring-cell words for ALL six rings
//! at once; the rings are separated along Z (ring idx k's dots sit
//! k * ZSTRIDE meters north — 4 m apart, far past the 5-tap strobe
//! tolerance at every band distance). Each ring streams a 64x64 WINDOW of
//! its 256x256 grid per lap; the window index = lap % 16, and the sixteen
//! windows partition the grid (window w = (cx/64) + (cz/64)*4).
//!
//! THE TAG: a lap boundary on the client could decode a foreign window's
//! word into the wrong cell (the OR-merge never loses bits — pollution
//! would be permanent). The HEIGHT word carries the window index in its
//! low nibble, painted with two EXTRA dots (the 6-slot HI word: 4 height
//! bits and 2 tag bits). The capture accepts a word only when its tag
//! equals the pixel's own window — a stateless, clock-free, drift-free
//! gate. The COLOR word carries no tag: it streams with the height word
//! at the same slot, so the height's acceptance (the rank cross-check via
//! the height twin, the v6.36 pattern) vouches for the color word.
//!
//! THE BEACON: slot (63, 63) never carries cell words. The server paints a
//! window-index word there (byte = w*17: high nibble = low nibble =
//! checksum) at every ring's Z offset; a 1x1 client pass decodes it once
//! per frame and the atlas passes early-out the 15/16 of pixels whose
//! window is not the active one — the capture cost stays at ring-0 levels
//! on llvmpipe.
//!
//! DATA: ring idx 0's inner cells (the ring-0 disk footprint) are the
//! ring-0 disk downsampled (max land height / water majority). Every other
//! cell samples ONE real chunk natively (seedgen, the v5.7 machinery): the
//! chunk whose center falls inside the cell. Far cells are therefore real
//! generator data at sparse resolution — the same trade Distant Horizons'
//! distant generation makes — and the march renders them as flat Voxy
//! quads at the ring's cell size.
//!
//! The march samples rings finest-first (ring 0 inside its disk, then the
//! cascade ring whose square contains the point), steps scale with the
//! LOCAL cell size, and the far fog hands off to a DH-style curve that
//! keeps the horizon visible instead of the vanilla 1 km wall.
//!
//! EVERYTHING here is config-gated (`cascade=true`); with the knob off the
//! module is byte-identical to v6.58 (the production pack ships the same
//! 13 entries, the engine never builds the store, no dots are spawned).

use crate::config::Config;
use crate::stream::{Cell, Spawn};
use std::collections::HashMap;

/// Cascade ring count (ring idx 0..5 behind the ring-0 disk).
pub const RINGS: usize = 6;
/// Ring idx 0 cell size in blocks; ring idx k cell = BASE * 4^k.
pub const RING_BASE_CELL: f64 = 16.0;
/// Scaffold slots per axis (64x64 = 4096 slots per window).
pub const SLOT_SIDE: u32 = 64;
/// Scaffold slot pitch in blocks (the transport quantum — independent of
/// the ring cell sizes; the word encoding lives at this scale).
pub const SCAFFOLD_PITCH: f64 = 2.0;
/// Sub-cell quantum of the scaffold word (pitch / 4, like ring 0).
pub const SCAFFOLD_SUB: f64 = SCAFFOLD_PITCH / 4.0;
/// The scaffold re-anchors every this-many-blocks step of the eye. Both
/// sides compute anchor = floor(eye / ANCHOR_GRID) * ANCHOR_GRID.
pub const ANCHOR_GRID: f64 = 128.0;
/// Windows per ring (16 x 64x64 = the full 256x256 grid per 16 laps).
pub const WINDOWS: u32 = 16;
/// Z separation between consecutive rings' dot stacks (blocks).
pub const ZSTRIDE: f64 = 4.0;
/// The beacon slot — never carries cell words (the window broadcast).
pub const BEACON_SLOT: (u32, u32) = (63, 63);
/// Ring idx 0's inner cells per axis (the ring-0 disk footprint at 16 m).
pub fn inner_side(region_blocks: i32) -> u32 {
    ((f64::from(region_blocks) / RING_BASE_CELL).floor().max(1.0)) as u32
}
/// The window index that carries ring cell (cx, cz): the sixteen 64x64
/// windows partition the 256x256 grid — exactly one window per cell.
#[allow(dead_code)] // protocol surface; the streamer uses window_origin directly
pub fn window_of(cx: u32, cz: u32) -> u32 {
    (cx / SLOT_SIDE) + (cz / SLOT_SIDE) * 4
}
/// The 64x64 window's cell-space origin for window w.
pub fn window_origin(w: u32) -> (u32, u32) {
    ((w % 4) * SLOT_SIDE, (w / 4) * SLOT_SIDE)
}
/// The scaffold slot a ring cell rides in its window.
#[allow(dead_code)] // protocol surface; pinned by the windows_partition test
pub fn slot_of(cx: u32, cz: u32) -> (u32, u32) {
    (cx % SLOT_SIDE, cz % SLOT_SIDE)
}
/// The scaffold anchor for an eye coordinate.
pub fn anchor_of(eye: f64) -> f64 {
    (eye / ANCHOR_GRID).floor() * ANCHOR_GRID
}

/// One cascade ring: 256x256 cells at `cell` meters.
pub struct RingStore {
    pub cell: f64,
    pub cells: Vec<Cell>,
}

impl RingStore {
    fn new(cell: f64) -> Self {
        RingStore {
            cell,
            cells: vec![
                Cell {
                    height: 0,
                    water: false,
                    valid: false,
                    provisional: true,
                    rgb: [0; 3],
                };
                256 * 256
            ],
        }
    }
    pub fn get(&self, cx: u32, cz: u32) -> Option<&Cell> {
        if cx >= 256 || cz >= 256 {
            return None;
        }
        self.cells.get((cz * 256 + cx) as usize)
    }
    pub fn set(&mut self, cx: u32, cz: u32, c: Cell) {
        if cx < 256 && cz < 256 {
            self.cells[(cz * 256 + cx) as usize] = c;
        }
    }
    /// Ring coverage in percent (valid cells / total) — the CI fill gate.
    pub fn coverage(&self) -> f64 {
        let valid = self.cells.iter().filter(|c| c.valid).count();
        100.0 * valid as f64 / self.cells.len() as f64
    }
}

/// Aggregate sampled columns into one cascade cell: max land height
/// (silhouette-preserving, like a far Voxy quad), water majority, color
/// from the tallest column. `rgb_of` resolves a column's palette color.
pub fn aggregate(cols: &[Column], sea: i32, rgb_of: &dyn Fn(i16, bool) -> [u8; 3]) -> Option<Cell> {
    if cols.is_empty() {
        return None;
    }
    let total = cols.len() as i32;
    let mut water_cnt = 0i32;
    let mut best: Option<(i16, bool)> = None;
    for c in cols {
        if c.liquid {
            water_cnt += 1;
        }
        let ly = if c.liquid { sea as i16 } else { c.height };
        let better = best.map(|(by, _)| ly > by).unwrap_or(true);
        if better {
            best = Some((ly, c.liquid));
        }
    }
    let (y, liquid) = best?;
    let water = water_cnt * 2 > total;
    let rgb = rgb_of(y, if water { true } else { liquid });
    Some(Cell {
        height: y.clamp(-64, 1023),
        water,
        valid: true,
        provisional: true,
        rgb,
    })
}

/// One sampled surface column (the seedgen helper's payload unpacked).
#[derive(Clone)]
pub struct Column {
    pub wx: i32,
    pub wz: i32,
    pub height: i16,
    pub liquid: bool,
    pub material: String,
    pub biome: String,
}

/// The whole cascade: six rings + the builder cursors + the scaffold
/// streamer state. Lives behind the State mutex; all methods run on the
/// main thread (the seedgen worker writes cells through the same lock).
pub struct Cascade {
    pub rings: Vec<RingStore>,
    /// Builder admission cursor per ring (cell index, row-major).
    cursor: [usize; RINGS],
    /// Ring idx 0 inner cells built so far (the downsample sweep cursor).
    inner_cursor: usize,
    /// Scaffold sweep cursor per ring (slot index within the window).
    sweep: [usize; RINGS],
    /// The lap the sweep cursors belong to (a new lap resets them).
    last_lap: u64,
    /// Scaffold rank cache: slot -> rank, valid for the current anchor.
    rank_cache: HashMap<(u32, u32), usize>,
    /// The anchor the cache was built for.
    anchor: (f64, f64),
    /// Diagnostics: cells sampled per ring since boot.
    pub sampled: [u64; RINGS],
}

impl Cascade {
    pub fn new() -> Self {
        let mut rings = Vec::with_capacity(RINGS);
        for k in 0..RINGS {
            rings.push(RingStore::new(RING_BASE_CELL * 4f64.powi(k as i32)));
        }
        Cascade {
            rings,
            cursor: [0; RINGS],
            inner_cursor: 0,
            sweep: [0; RINGS],
            last_lap: 0,
            rank_cache: HashMap::new(),
            anchor: (f64::NAN, f64::NAN),
            sampled: [0; RINGS],
        }
    }

    pub fn ring_origin(cfg: &Config) -> (f64, f64) {
        let half = f64::from(cfg.region_blocks) * 0.5;
        (
            cfg.center_x.unwrap_or(0.0) - half,
            cfg.center_z.unwrap_or(0.0) - half,
        )
    }

    /// The chunk (cx, cz) whose center falls inside ring cell (ccx, ccz) —
    /// the sample source both the builder enqueues and the worker claims.
    /// The chunk center nearest the cell center wins; the inverse map
    /// (chunk_claim) round-trips it (pinned by test).
    pub fn sample_chunk(origin: (f64, f64), cell: f64, ccx: i64, ccz: i64) -> Option<(i32, i32)> {
        let (ox, oz) = origin;
        let center_x = ox + (ccx as f64 + 0.5) * cell;
        let center_z = oz + (ccz as f64 + 0.5) * cell;
        let base_x = ((center_x - 8.0) / 16.0).floor() as i64;
        let base_z = ((center_z - 8.0) / 16.0).floor() as i64;
        let mut best: Option<(i32, i32, f64)> = None;
        for dx in -1..=1i64 {
            for dz in -1..=1i64 {
                let cx = base_x + dx;
                let cz = base_z + dz;
                let ccx16 = (cx * 16 + 8) as f64;
                let ccz16 = (cz * 16 + 8) as f64;
                let inx = ccx16 >= ox + ccx as f64 * cell && ccx16 < ox + (ccx + 1) as f64 * cell;
                let inz = ccz16 >= oz + ccz as f64 * cell && ccz16 < oz + (ccz + 1) as f64 * cell;
                if !(inx && inz) {
                    continue;
                }
                let d = (ccx16 - center_x).hypot(ccz16 - center_z);
                if best.map(|(_, _, bd)| d < bd).unwrap_or(true) {
                    best = Some((cx as i32, cz as i32, d));
                }
            }
        }
        best.map(|(cx, cz, _)| (cx, cz))
    }

    /// The ring cell a chunk claims per ring (the worker-side inverse map):
    /// the cell whose interval contains the chunk's CENTER point.
    pub fn chunk_claim(origin: (f64, f64), cell: f64, cx: i32, cz: i32) -> (i64, i64) {
        let ccx = (((cx * 16 + 8) as f64 - origin.0) / cell).floor() as i64;
        let ccz = (((cz * 16 + 8) as f64 - origin.1) / cell).floor() as i64;
        (ccx, ccz)
    }

    /// Write one sampled chunk's columns into every ring it claims. Called
    /// from the seedgen worker for every generated chunk (the columns are
    /// the helper's payload grid for that chunk). `rgb_of` maps a winning
    /// column's (height, liquid) to its palette color.
    pub fn ingest_chunk(
        &mut self,
        origin: (f64, f64),
        cx: i32,
        cz: i32,
        cols: &[Column],
        sea: i32,
        rgb_of: &dyn Fn(i16, bool) -> [u8; 3],
    ) {
        if cols.is_empty() {
            return;
        }
        for k in 0..RINGS {
            let cell = self.rings[k].cell;
            let (ccx, ccz) = Self::chunk_claim(origin, cell, cx, cz);
            if ccx < 0 || ccz < 0 || ccx >= 256 || ccz >= 256 {
                continue;
            }
            // Ring idx >= 1 honors the claim only for the cell's OWN sample
            // chunk; ring idx 0 takes any chunk whose columns fall inside
            // (a 16 m cell straddling a chunk border takes the share).
            if k >= 1 && Self::sample_chunk(origin, cell, ccx, ccz) != Some((cx, cz)) {
                continue;
            }
            if self.rings[k]
                .get(ccx as u32, ccz as u32)
                .map(|c| c.valid)
                .unwrap_or(false)
            {
                continue;
            }
            let x0 = (origin.0 + ccx as f64 * cell).floor() as i32;
            let z0 = (origin.1 + ccz as f64 * cell).floor() as i32;
            let x1 = (origin.0 + (ccx + 1) as f64 * cell).floor() as i32;
            let z1 = (origin.1 + (ccz + 1) as f64 * cell).floor() as i32;
            let mine: Vec<Column> = cols
                .iter()
                .filter(|c| c.wx >= x0 && c.wx < x1 && c.wz >= z0 && c.wz < z1)
                .cloned()
                .collect();
            if let Some(c) = aggregate(&mine, sea, rgb_of) {
                self.rings[k].set(ccx as u32, ccz as u32, c);
                self.sampled[k] += 1;
            }
        }
    }

    /// Builder pump: build ring idx 0's inner cells from the ring-0 disk
    /// downsample (all parents valid), then admit chunk samples for the
    /// still-invalid cells of each ring, nearest ring first. `enqueue`
    /// admits a chunk to the seedgen queue. Returns chunks enqueued.
    pub fn pump(
        &mut self,
        stream: &crate::stream::Streamer,
        cfg: &Config,
        admit: usize,
        enqueue: &mut dyn FnMut(i32, i32),
    ) -> usize {
        let origin = Self::ring_origin(cfg);
        let sea = cfg.sea_level;
        let mut enqueued = 0usize;

        // ---- ring idx 0 inner: the ring-0 disk downsample ------------------
        let inner = inner_side(cfg.region_blocks) as usize;
        let parent_side_f = RING_BASE_CELL / f64::from(cfg.cell_size.max(1));
        let parent_side = parent_side_f.round() as usize;
        if parent_side >= 1 && (parent_side_f - parent_side as f64).abs() < 1.0e-6 {
            while self.inner_cursor < inner * inner {
                let ci = self.inner_cursor;
                let ix = (ci % inner) as u32;
                let iz = (ci / inner) as u32;
                if self.rings[0].get(ix, iz).map(|c| c.valid).unwrap_or(false) {
                    self.inner_cursor += 1;
                    continue;
                }
                let px0 = (ci % inner) * parent_side;
                let pz0 = (ci / inner) * parent_side;
                let side = stream.side as usize;
                let mut got: Vec<(i16, bool, [u8; 3])> =
                    Vec::with_capacity(parent_side * parent_side);
                let mut all = true;
                for dz in 0..parent_side {
                    for dx in 0..parent_side {
                        let sx = px0 + dx;
                        let sz = pz0 + dz;
                        if sx >= side || sz >= side {
                            all = false;
                            break;
                        }
                        let c = &stream.cells[sz * side + sx];
                        if !c.valid {
                            all = false;
                            break;
                        }
                        got.push((c.height, c.water, c.rgb));
                    }
                    if !all {
                        break;
                    }
                }
                if !all || got.is_empty() {
                    break; // parents not scanned yet: retry next tick
                }
                self.inner_cursor += 1;
                let total = got.len() as i32;
                let water_cnt = got.iter().filter(|(_, l, _)| *l).count() as i32;
                let water = water_cnt * 2 > total;
                let best = got
                    .iter()
                    .filter(|(_, l, _)| *l == water || water_cnt == 0 || water_cnt == total)
                    .copied()
                    .max_by_key(|(y, l, _)| if *l { sea as i16 } else { *y });
                let Some((y, _l, rgb)) = best else {
                    continue;
                };
                self.rings[0].set(
                    ix,
                    iz,
                    Cell {
                        height: y.clamp(-64, 1023),
                        water,
                        valid: true,
                        provisional: true,
                        rgb,
                    },
                );
                self.sampled[0] += 1;
            }
        }

        // ---- chunk-sample admission: nearest ring first ---------------------
        'rings: for k in 0..RINGS {
            let cell = self.rings[k].cell;
            let inner_cut = if k == 0 { inner as i64 } else { 0 };
            while enqueued < admit {
                let ci = self.cursor[k];
                if ci >= 256 * 256 {
                    continue 'rings;
                }
                self.cursor[k] = ci + 1;
                let ccx = (ci % 256) as i64;
                let ccz = (ci / 256) as i64;
                if ccx < inner_cut && ccz < inner_cut {
                    continue;
                }
                if self.rings[k]
                    .get(ccx as u32, ccz as u32)
                    .map(|c| c.valid)
                    .unwrap_or(false)
                {
                    continue;
                }
                let Some((scx, scz)) = Self::sample_chunk(origin, cell, ccx, ccz) else {
                    continue;
                };
                enqueue(scx, scz);
                enqueued += 1;
            }
            if enqueued >= admit {
                break 'rings;
            }
        }
        enqueued
    }

    /// The transport rank for a scaffold slot: the lowest tier strictly
    /// above BOTH the local surface (+TIER_MARGIN, via tier_rank) AND
    /// (eye_y - 100) — a dot must sit above the terrain (26.3 hides
    /// particles inside blocks) and inside the decode band of the eye that
    /// must strobe it. `local_height` = the loaded-chunk surface probe.
    fn slot_rank(
        &mut self,
        ax: f64,
        az: f64,
        si: u32,
        sj: u32,
        local_height: Option<i16>,
        eye_y: f64,
    ) -> usize {
        let key = (si, sj);
        if self.anchor != (ax, az) {
            self.anchor = (ax, az);
            self.rank_cache.clear();
        }
        if let Some(&r) = self.rank_cache.get(&key) {
            return r;
        }
        let surface_floor = local_height.map(crate::stream::tier_rank).unwrap_or(0);
        let eye_floor = crate::stream::tier_rank((eye_y - 100.0).clamp(-64.0, 1.0e4) as i16);
        let r = surface_floor.max(eye_floor).min(3);
        self.rank_cache.insert(key, r);
        r
    }

    /// Stream one tick of the scaffold into `out`. Returns dots spent.
    ///
    /// `rank_probe(wx, wz)` resolves the LOCAL surface height at a world xz
    /// (the JNI loaded-chunk probe); `None` degrades the slot to rank 0.
    #[allow(clippy::too_many_arguments)]
    pub fn stream_tick(
        &mut self,
        _cfg: &Config,
        tick: u64,
        lap_ticks: u64,
        guard: u64,
        budget: usize,
        eye: [f64; 3],
        look: [f64; 3],
        sky_visible: bool,
        rank_probe: &mut dyn FnMut(i32, i32) -> Option<i16>,
        out: &mut Vec<Spawn>,
    ) -> usize {
        if lap_ticks == 0 || tick % lap_ticks < guard {
            return 0; // lap boundary guard: let old dots die first
        }
        if !sky_visible {
            // A sky-blind view never decodes (the rot channel decays) —
            // return BEFORE the sweep: a refusal must not burn the cursor
            // (the slots stay streamable when the sky comes back).
            return 0;
        }
        let lap = tick / lap_ticks;
        let w = (lap % u64::from(WINDOWS)) as u32;
        if lap != self.last_lap {
            self.last_lap = lap;
            self.sweep = [0; RINGS];
        }
        let (wx, wz) = window_origin(w);
        let ax = anchor_of(eye[0]);
        let az = anchor_of(eye[2]);
        let mut spent = 0usize;

        // ---- the beacon: the window broadcast at every ring's Z offset ----
        let beacon_byte = ((w * 17) & 0xFF) as u16;
        let bx = ax + (BEACON_SLOT.0 as f64 + 0.5) * SCAFFOLD_PITCH;
        let bz = az + (BEACON_SLOT.1 as f64 + 0.5) * SCAFFOLD_PITCH;
        for k in 0..RINGS {
            let rank = self.slot_rank(
                ax,
                az,
                BEACON_SLOT.0,
                BEACON_SLOT.1,
                rank_probe(bx as i32, bz as i32),
                eye[1],
            );
            let zoff = k as f64 * ZSTRIDE;
            let word = beacon_byte << 8;
            // Same band+cone discipline as the cell words: a beacon dot the
            // eye cannot decode is waste (and the test's band invariant).
            if decodable(
                bx,
                crate::stream::HI_TIERS[rank] + zoff,
                bz + zoff,
                eye,
                look,
                sky_visible,
            ) {
                word_dots(word, bx, bz, 4, crate::stream::HI_TIERS[rank], zoff, out);
                spent += 4;
            }
        }

        // ---- the six rings, round robin, slot sweep per lap ----------------
        let tag = (w & 0xF) as u16;
        let slots_total = (SLOT_SIDE * SLOT_SIDE) as usize;
        'budget: while spent < budget {
            let mut progressed = false;
            for k in 0..RINGS {
                if spent >= budget {
                    break 'budget;
                }
                while self.sweep[k] < slots_total {
                    let s = self.sweep[k];
                    self.sweep[k] += 1;
                    let si = (s % SLOT_SIDE as usize) as u32;
                    let sj = (s / SLOT_SIDE as usize) as u32;
                    if (si, sj) == BEACON_SLOT {
                        continue;
                    }
                    let cx = wx + si;
                    let cz = wz + sj;
                    let Some(cell) = self.rings[k].get(cx, cz).cloned() else {
                        continue;
                    };
                    if !cell.valid {
                        continue;
                    }
                    let scx = ax + (si as f64 + 0.5) * SCAFFOLD_PITCH;
                    let scz = az + (sj as f64 + 0.5) * SCAFFOLD_PITCH;
                    let rank =
                        self.slot_rank(ax, az, si, sj, rank_probe(scx as i32, scz as i32), eye[1]);
                    let zoff = k as f64 * ZSTRIDE;
                    // Band + frustum pre-check on the word probe centers (the
                    // per-dot gate repeats it exactly at delivery — this just
                    // keeps refused slots from costing budget).
                    let hi_y = crate::stream::HI_TIERS[rank];
                    let lo_y = crate::stream::LO_TIERS[rank];
                    if !decodable(scx, hi_y + zoff, scz + zoff, eye, look, sky_visible)
                        || !decodable(scx, lo_y + zoff, scz + zoff, eye, look, sky_visible)
                    {
                        continue;
                    }
                    let (hw, cw) = crate::stream::Streamer::encode_words(&cell);
                    // HI word: height byte (bits 8..15) + window tag
                    // (bits 4..7 — dots 4/5 carry word bits 6..5/4..5 via the
                    // same (14 - 2*slot) shifts; the low nibble never paints).
                    let hi = (hw & 0xFF00) | (tag << 4);
                    let lo = cw & 0xFF00;
                    word_dots(hi, scx, scz, 6, hi_y, zoff, out);
                    word_dots(lo, scx, scz, 4, lo_y, zoff, out);
                    spent += 10;
                    progressed = true;
                    break;
                }
            }
            if !progressed {
                break 'budget; // every ring's window swept for this lap
            }
        }
        spent
    }
}

/// The spawn-gate geometry for a word's probe center (mirrors the per-dot
/// gate: band (CULL_RADIUS, ZONE_FAR] + frustum cone; the delivery gate
/// repeats it exactly per dot).
fn decodable(x: f64, y: f64, z: f64, eye: [f64; 3], look: [f64; 3], sky_visible: bool) -> bool {
    if !sky_visible {
        return false;
    }
    let d = [x - eye[0], y - eye[1], z - eye[2]];
    let r2 = d[0] * d[0] + d[1] * d[1] + d[2] * d[2];
    let r = r2.sqrt();
    if r <= crate::stream::CULL_RADIUS || r > crate::stream::ZONE_FAR {
        return false;
    }
    (d[0] * look[0] + d[1] * look[1] + d[2] * look[2]) / r > crate::stream::cos_half_cone()
}

/// The cascade word dots: `slots` dots at Z positions
/// (s - (slots-1)/2) * SCAFFOLD_SUB + zoff, X sub = the 2-bit value,
/// msb-first (slot 0 carries bits 14..15) — the exact geometry
/// cascade_capture.fsh re-projects.
pub fn word_dots(
    word: u16,
    cx: f64,
    cz: f64,
    slots: usize,
    plane_y: f64,
    zoff: f64,
    out: &mut Vec<Spawn>,
) {
    let half = (slots as f64 - 1.0) * 0.5;
    for slot in 0..slots {
        let bits = ((word >> (14 - 2 * slot)) & 3) as f64;
        let dx = (bits - 1.5) * SCAFFOLD_SUB;
        let dz = (slot as f64 - half) * SCAFFOLD_SUB + zoff;
        out.push(Spawn {
            x: cx + dx,
            y: plane_y,
            z: cz + dz,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg() -> Config {
        Config::parse("center_x=1000;center_z=2000")
    }

    #[test]
    fn ring_geometry_quadruples() {
        let c = Cascade::new();
        assert_eq!(c.rings.len(), RINGS);
        assert_eq!(c.rings[0].cell, 16.0);
        assert_eq!(c.rings[1].cell, 64.0);
        assert_eq!(c.rings[5].cell, 16384.0);
        let half_span = c.rings[5].cell * 256.0 / 2.0;
        assert!(half_span >= 2_000_000.0, "half span {half_span}");
    }

    #[test]
    fn windows_partition_the_grid() {
        for cx in [0u32, 1, 63, 64, 127, 128, 255] {
            for cz in [0u32, 64, 191, 255] {
                let w = window_of(cx, cz);
                let (wx, wz) = window_origin(w);
                let (si, sj) = slot_of(cx, cz);
                assert_eq!(wx + si, cx);
                assert_eq!(wz + sj, cz);
                assert!(w < WINDOWS);
            }
        }
    }

    #[test]
    fn sample_chunk_is_the_inverse_of_chunk_claim() {
        for origin in [(0.0, 0.0), (1000.5, -333.25), (-512.0, 4096.0)] {
            for cell in [16.0, 64.0, 16384.0] {
                for ccx in [0i64, 7, 255] {
                    for ccz in [0i64, 128, 255] {
                        let Some((cx, cz)) = Cascade::sample_chunk(origin, cell, ccx, ccz) else {
                            continue;
                        };
                        let (bx, bz) = Cascade::chunk_claim(origin, cell, cx, cz);
                        assert_eq!(
                            (bx, bz),
                            (ccx, ccz),
                            "origin {origin:?} cell {cell}: chunk ({cx},{cz}) must claim ({ccx},{ccz}), got ({bx},{bz})"
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn ingest_builds_cells_from_chunk_columns() {
        let mut c = Cascade::new();
        let origin = (0.0, 0.0);
        let Some((cx, cz)) = Cascade::sample_chunk(origin, 64.0, 0, 0) else {
            panic!("no sample chunk");
        };
        let mut cols = Vec::new();
        for gx in 0..8 {
            for gz in 0..8 {
                cols.push(Column {
                    wx: cx * 16 + gx * 2,
                    wz: cz * 16 + gz * 2,
                    height: 70 + gx as i16,
                    liquid: false,
                    material: "grass_block".into(),
                    biome: "plains".into(),
                });
            }
        }
        let rgb_of = |_: i16, _: bool| [51, 102, 51];
        c.ingest_chunk(origin, cx, cz, &cols, 63, &rgb_of);
        let (ccx, ccz) = Cascade::chunk_claim(origin, 64.0, cx, cz);
        let cell = c.rings[1].get(ccx as u32, ccz as u32).unwrap();
        assert!(cell.valid, "the claimed cell must be built");
        assert_eq!(cell.height, 77); // max land height
        assert!(!cell.water);
        assert_eq!(cell.rgb, [51, 102, 51]);
    }

    #[test]
    fn ingest_water_majority_flips_the_cell() {
        let mut c = Cascade::new();
        let mut cols = Vec::new();
        for gx in 0..8 {
            for gz in 0..8 {
                let liquid = gx < 5; // 40 of 64 columns water
                cols.push(Column {
                    wx: gx * 2,
                    wz: gz * 2,
                    height: if liquid { 60 } else { 80 },
                    liquid,
                    material: if liquid { "water" } else { "grass_block" }.into(),
                    biome: "plains".into(),
                });
            }
        }
        let rgb_of = |_: i16, l: bool| {
            if l {
                crate::palette::surface_color("water", "ocean", true)
            } else {
                [51, 102, 51]
            }
        };
        // Ring idx 0 claims the cell containing the chunk center (8,8) -> 0,0.
        c.ingest_chunk((0.0, 0.0), 0, 0, &cols, 63, &rgb_of);
        let cell = c.rings[0].get(0, 0).unwrap();
        assert!(cell.valid);
        assert!(cell.water, "water majority must flip the cell");
    }

    #[test]
    fn tag_rides_the_height_word_low_nibble() {
        let cell = Cell {
            height: 100,
            water: false,
            valid: true,
            provisional: false,
            rgb: [51, 51, 51],
        };
        let (hw, cw) = crate::stream::Streamer::encode_words(&cell);
        assert_eq!(hw & 0xFF, 0);
        assert_eq!(cw & 0xFF, 0);
        let tag = 9u16;
        let hi = (hw & 0xFF00) | (tag << 4);
        assert_eq!(hi >> 8, 164); // (100 + 64)
        assert_eq!((hi >> 4) & 0xF, tag);
    }

    #[test]
    fn word_dots_geometry_matches_the_shader_contract() {
        // 6-slot HI word: Z positions at (s - 2.5) * 0.5 + zoff, X at
        // (bits - 1.5) * 0.5. Slot 0 carries bits 14..15 (msb-first).
        let mut out = Vec::new();
        word_dots(0b10 << 14, 100.0, 200.0, 6, 120.0, 8.0, &mut out);
        assert_eq!(out.len(), 6);
        assert_eq!(out[0].x, 100.0 + (2.0 - 1.5) * 0.5);
        assert_eq!(out[0].y, 120.0);
        assert_eq!(out[0].z, 200.0 + (0.0 - 2.5) * 0.5 + 8.0);
        // 4-slot LO word mirrors the ring-0 geometry (+zoff only).
        out.clear();
        word_dots(5 << 14, 100.0, 200.0, 4, 80.0, 4.0, &mut out);
        assert_eq!(out.len(), 4);
        assert_eq!(out[3].z, 200.0 + (3.0 - 1.5) * 0.5 + 4.0);
    }

    #[test]
    fn stream_tick_respects_the_lap_guard_budget_and_band() {
        let mut c = Cascade::new();
        for cz in 0..64u32 {
            for cx in 0..64u32 {
                c.rings[0].set(
                    cx,
                    cz,
                    Cell {
                        height: 80,
                        water: false,
                        valid: true,
                        provisional: true,
                        rgb: [51, 51, 51],
                    },
                );
            }
        }
        let cfg = cfg();
        let mut out = Vec::new();
        let mut probe = |_x: i32, _z: i32| -> Option<i16> { Some(70) };
        let eye = [0.0, 70.0, 0.0];
        let look = [0.0, 0.0, 1.0];
        // Guard window: nothing streams.
        let n = c.stream_tick(&cfg, 0, 300, 8, 1400, eye, look, true, &mut probe, &mut out);
        assert_eq!(n, 0, "the lap guard must silence tick 0");
        // Mid-lap: the beacon (24 dots) + budgeted cells, all within budget.
        out.clear();
        let n = c.stream_tick(
            &cfg, 100, 300, 8, 1400, eye, look, true, &mut probe, &mut out,
        );
        assert!(n >= 24, "the beacon must ride every tick ({n})");
        assert!(n <= 1400 + 24, "budget respected ({n})");
        assert_eq!(out.len(), n, "every spent dot is a spawn");
        // Every dot sits within the decode band of the eye.
        for sp in &out {
            let d = (sp.x * sp.x + (sp.y - 70.0) * (sp.y - 70.0) + sp.z * sp.z).sqrt();
            assert!(d <= crate::stream::ZONE_FAR + 1.0, "dot at {d} out of band");
        }
        // Sky-blind: nothing streams.
        out.clear();
        let n = c.stream_tick(
            &cfg, 100, 300, 8, 1400, eye, look, false, &mut probe, &mut out,
        );
        assert_eq!(n, 0, "sky-blind views stream nothing");
        // The sweep advances: a second tick spends MORE cells (cursor moved).
        out.clear();
        let n2 = c.stream_tick(
            &cfg, 101, 300, 8, 1400, eye, look, true, &mut probe, &mut out,
        );
        assert!(n2 > 24, "the sweep cursor must advance (n2={n2})");
    }
}
