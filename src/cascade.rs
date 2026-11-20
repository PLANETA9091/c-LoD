//! v6.59 THE CASCADE — reconstruction from recovery notes (v659-recovery-notes.md
//! §2 verbatim fragments + the in-conversation reads of the lost v6.59 file).
//!
//! Six rings r0..r5, cell sizes 16/64/256/1024/4096/16384 m, 256×256 cells
//! each. Per lap (LAP_TICKS) the stream serves one 64×64 window of the cell
//! grid: the beacon broadcasts the active window, cell words paint in
//! anchor-local slots, the atlas decodes only that window per lap.
//!
//! RECONSTRUCTED (not verbatim): slot_rank's eye term, rank_probe's data
//! source, the sample/chunk mapping internals — the unit tests pin the
//! observable contracts from the lost file.

use crate::config::Config;
use crate::stream::{
    cos_half_cone, Cell, CULL_RADIUS, HI_TIERS, LO_TIERS, Spawn, ZONE_FAR,
};

pub const RINGS: usize = 6;
pub const GRID: u32 = 256;
pub const SLOT_SIDE: u32 = 64;
pub const WINDOWS: u32 = 16;
pub const LAP_TICKS: u64 = 300;

pub const SCAFFOLD_PITCH: f64 = 2.0;
pub const SCAFFOLD_SUB: f64 = SCAFFOLD_PITCH / 4.0;
pub const ZSTRIDE: f64 = 4.0;
pub const BEACON_SLOT: (u32, u32) = (63, 63);

/// One ingested world column (16.59: from the native generation sampler /
/// loaded-chunk scans). `wx/wz` are world block coords of the column.
pub struct Column {
    pub wx: i32,
    pub wz: i32,
    pub height: i16,
    pub liquid: bool,
    pub material: String,
}

/// The stream word: 16 bits, slot s carries bits (14-2s)..(15-2s) as its X
/// sub-position. HI = 6 dots (height byte bits 8..15 + tag bits 4..7),
/// LO = 4 dots (color byte bits 8..15).
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

pub fn window_origin(w: u32) -> (u32, u32) {
    ((w % 4) * 64, (w / 4) * 64)
}

pub fn window_of(cx: u32, cz: u32) -> u32 {
    (cx >> 6) + (cz >> 6) * 4
}

pub fn slot_of(cx: u32, cz: u32) -> (u32, u32) {
    (cx & 63, cz & 63)
}

/// The 128 m anchor lattice the scaffold slots ride (floor to the grid).
pub fn anchor_of(v: f64) -> f64 {
    (v / 128.0).floor() * 128.0
}

/// The cascade grid origin: the disk center minus half the region span.
pub fn ring_origin(cfg: &Config) -> (f64, f64) {
    let half = f64::from(cfg.region_blocks) * 0.5;
    (
        cfg.center_x.unwrap_or(0.0) - half,
        cfg.center_z.unwrap_or(0.0) - half,
    )
}

pub fn ring_cell(r: usize) -> f64 {
    16.0 * exp2(2 * r as i32)
}

fn exp2(p: i32) -> f64 {
    f64::from(1u32 << p.max(0))
}

/// One ring's 256×256 cell grid. Ring 0 cells are 16 m; ring 5 = 16 km.
pub struct Ring {
    pub cell: f64,
    cells: Vec<Cell>,
}

impl Ring {
    fn new(cell: f64) -> Self {
        Ring {
            cell,
            cells: vec![Cell::default(); (GRID * GRID) as usize],
        }
    }

    pub fn get(&self, cx: u32, cz: u32) -> Option<&Cell> {
        if cx >= GRID || cz >= GRID {
            return None;
        }
        Some(&self.cells[(cz * GRID + cx) as usize])
    }

    /// Cheap scan-cursor skip: has this cell ingested any column?
    pub fn valid_at(&self, cx: u32, cz: u32) -> bool {
        self.get(cx, cz).map(|c| c.valid).unwrap_or(false)
    }

    fn set(&mut self, cx: u32, cz: u32, cell: Cell) -> bool {
        if cx >= GRID || cz >= GRID {
            return false;
        }
        self.cells[(cz * GRID + cx) as usize] = cell;
        true
    }

    /// The ring cell holding world block (wx, wz): grid-indexed, cell sizes
    /// are powers of four so one origin serves every ring.
    fn claim_block(&self, origin: (f64, f64), wx: i32, wz: i32) -> Option<(u32, u32)> {
        let cx = ((f64::from(wx) - origin.0) / self.cell).floor();
        let cz = ((f64::from(wz) - origin.1) / self.cell).floor();
        if cx < 0.0 || cz < 0.0 || cx >= f64::from(GRID) || cz >= f64::from(GRID) {
            return None;
        }
        Some((cx as u32, cz as u32))
    }
}

/// Cascade diagnostics (the cdiag tail of the engine's stream heartbeat).
pub struct CDiag {
    pub ticks: u64,
    pub beacon_ok: u64,
    pub slots_examined: u64,
    pub cells_valid: u64,
    pub dec_sky: u64,
    pub dec_band: u64,
    pub dec_cone: u64,
    pub dec_ok: u64,
}

impl CDiag {
    pub fn line(&self) -> String {
        format!(
            "t{} bk{} sl{} cv{} sky{} band{} cone{} ok{}",
            self.ticks,
            self.beacon_ok,
            self.slots_examined,
            self.cells_valid,
            self.dec_sky,
            self.dec_band,
            self.dec_cone,
            self.dec_ok
        )
    }
}

/// The refusal reason for a word's probe center: 0 = decodable,
/// 1 = sky-blind, 2 = out of the decode band, 3 = outside the frustum cone.
fn dec_ok_reason(x: f64, y: f64, z: f64, eye: [f64; 3], look: [f64; 3], sky_visible: bool) -> u8 {
    if !sky_visible {
        return 1;
    }
    let d = [x - eye[0], y - eye[1], z - eye[2]];
    let d2 = d[0] * d[0] + d[1] * d[1] + d[2] * d[2];
    if d2 <= CULL_RADIUS * CULL_RADIUS || d2 > ZONE_FAR * ZONE_FAR {
        return 2;
    }
    let inv = 1.0 / d2.sqrt();
    if (d[0] * look[0] + d[1] * look[1] + d[2] * look[2]) * inv <= cos_half_cone() {
        return 3;
    }
    0
}

pub struct Cascade {
    pub rings: Vec<Ring>,
    pub sweep: [u32; RINGS],
    pub diag: CDiag,
    pub ticks: u64,
    /// The engine's ring-scan cursor (ring-0 cell index 0..65536).
    pub scan_cursor: u32,
    last_lap: u64,
}

impl Cascade {
    pub fn new() -> Self {
        Cascade {
            rings: (0..RINGS)
                .map(|r| Ring::new(16.0 * exp2(2 * r as i32)))
                .collect(),
            sweep: [0; RINGS],
            diag: CDiag {
                ticks: 0,
                beacon_ok: 0,
                slots_examined: 0,
                cells_valid: 0,
                dec_sky: 0,
                dec_band: 0,
                dec_cone: 0,
                dec_ok: 0,
            },
            ticks: 0,
            scan_cursor: 0,
            last_lap: u64::MAX,
        }
    }

    /// The lane coverage line: ingested-cell percentages for rings 0/1, the
    /// fill gate the probe lane reads off /clod-pack.info.
    pub fn coverage_line(&self) -> String {
        let pct = |r: usize| -> f64 {
            let valid = self.rings[r]
                .cells
                .iter()
                .filter(|c| c.valid)
                .count() as f64;
            valid * 100.0 / f64::from(GRID * GRID)
        };
        format!("cascade r0={:.1}% r1={:.1}%", pct(0), pct(1))
    }

    /// The chunk whose center falls inside ring cell (ccx, ccz) — the chunk
    /// that claims the cell when its columns ingest. Mapped by the cell
    /// center: chunk_claim maps the chunk center back, and the chunk center
    /// never drifts more than 8 m from the cell center, so the inverse
    /// round-trips for every cell size ≥ 16.
    pub fn sample_chunk(origin: (f64, f64), cell: f64, ccx: i64, ccz: i64) -> Option<(i32, i32)> {
        if ccx < 0 || ccz < 0 || ccx >= i64::from(GRID) || ccz >= i64::from(GRID) {
            return None;
        }
        let wx = origin.0 + (ccx as f64 + 0.5) * cell;
        let wz = origin.1 + (ccz as f64 + 0.5) * cell;
        Some(((wx / 16.0).floor() as i32, (wz / 16.0).floor() as i32))
    }

    /// The ring cell claiming the chunk (cx, cz): by the chunk center.
    pub fn chunk_claim(origin: (f64, f64), cell: f64, cx: i32, cz: i32) -> (i64, i64) {
        let mid = (f64::from(cx) * 16.0 + 8.0 - origin.0) / cell;
        let midz = (f64::from(cz) * 16.0 + 8.0 - origin.1) / cell;
        (mid.floor() as i64, midz.floor() as i64)
    }

    /// Ingest world columns into every ring whose cell they fall into.
    pub fn ingest_columns(&mut self, origin: (f64, f64), cols: &[Column]) {
        for col in cols {
            for ring in self.rings.iter_mut() {
                if let Some((cx, cz)) = ring.claim_block(origin, col.wx, col.wz) {
                    let cell = Cell {
                        height: col.height,
                        water: col.liquid,
                        valid: true,
                        provisional: false,
                        rgb: column_rgb(&col.material),
                    };
                    ring.set(cx, cz, cell);
                }
            }
        }
    }

    /// The surface probe for a rank decision: ring 0's cell at the world
    /// block, when valid.
    pub fn rank_probe(&self, origin: (f64, f64), x: i32, z: i32) -> Option<i16> {
        let r0 = &self.rings[0];
        let cx = ((f64::from(x) - origin.0) / r0.cell).floor();
        let cz = ((f64::from(z) - origin.1) / r0.cell).floor();
        if cx < 0.0 || cz < 0.0 {
            return None;
        }
        r0.get(cx as u32, cz as u32)
            .filter(|c| c.valid)
            .map(|c| c.height)
    }

    /// RECONSTRUCTED: the rank plane must clear the surface and stay above
    /// the eye-40 line (a plane below it cannot carry over the terrain).
    pub fn slot_rank(
        &self,
        _origin: (f64, f64),
        si: u32,
        sj: u32,
        probe: Option<i16>,
        eye_y: f64,
    ) -> usize {
        let _ = (si, sj);
        let h = probe.map(f64::from).unwrap_or(eye_y);
        let h = h.max(eye_y - 40.0);
        LO_TIERS.iter().position(|&lo| lo > h).unwrap_or(3)
    }

    /// The stream tick: beacon broadcast for window w at every ring's Z
    /// offset, then the six rings' round-robin slot sweep for the same
    /// window. Verbatim from the v6.59 core loop (recovery notes §2).
    pub fn stream_tick(
        &mut self,
        origin: (f64, f64),
        w: u32,
        eye: [f64; 3],
        look: [f64; 3],
        budget: usize,
        sky_visible: bool,
        out: &mut Vec<Spawn>,
    ) -> usize {
        let ax = anchor_of(eye[0]);
        let az = anchor_of(eye[2]);
        let mut spent = 0usize;
        self.ticks += 1;
        self.diag.ticks = self.ticks;
        // Lap boundary: reset the per-ring slot sweeps for the next window.
        let lap = self.ticks / LAP_TICKS;
        if lap != self.last_lap {
            self.last_lap = lap;
            self.sweep = [0; RINGS];
        }

        // ---- the beacon: the window broadcast at every ring's Z offset ----
        let beacon_byte = ((w * 17) & 0xFF) as u16;
        let bx = ax + (BEACON_SLOT.0 as f64 + 0.5) * SCAFFOLD_PITCH;
        let bz = az + (BEACON_SLOT.1 as f64 + 0.5) * SCAFFOLD_PITCH;
        for k in 0..RINGS {
            let rank = self.slot_rank(
                origin,
                BEACON_SLOT.0,
                BEACON_SLOT.1,
                self.rank_probe(origin, bx as i32, bz as i32),
                eye[1],
            );
            let zoff = k as f64 * ZSTRIDE;
            let word = beacon_byte << 8;
            if dec_ok_reason(bx, HI_TIERS[rank] + zoff, bz + zoff, eye, look, sky_visible) == 0 {
                self.diag.beacon_ok += 1;
                word_dots(word, bx, bz, 4, HI_TIERS[rank], zoff, out);
                spent += 4;
            }
        }

        // ---- the six rings, round robin, slot sweep per lap ----
        let tag = (w & 0xF) as u16;
        let slots_total = SLOT_SIDE * SLOT_SIDE;
        let (wx, wz) = window_origin(w);
        'budget: while spent < budget {
            let mut progressed = false;
            for k in 0..RINGS {
                if spent >= budget {
                    break 'budget;
                }
                while self.sweep[k] < slots_total {
                    let s = self.sweep[k];
                    self.sweep[k] += 1;
                    let si = s % SLOT_SIDE;
                    let sj = s / SLOT_SIDE;
                    if (si, sj) == BEACON_SLOT {
                        continue;
                    }
                    self.diag.slots_examined += 1;
                    let cx = wx + si;
                    let cz = wz + sj;
                    let Some(cell) = self.rings[k].get(cx, cz).cloned() else {
                        continue;
                    };
                    if !cell.valid {
                        continue;
                    }
                    self.diag.cells_valid += 1;
                    let scx = ax + (si as f64 + 0.5) * SCAFFOLD_PITCH;
                    let scz = az + (sj as f64 + 0.5) * SCAFFOLD_PITCH;
                    let rank = self.slot_rank(
                        origin,
                        si,
                        sj,
                        self.rank_probe(origin, scx as i32, scz as i32),
                        eye[1],
                    );
                    let zoff = k as f64 * ZSTRIDE;
                    // Band + frustum pre-check on the word probe centers (the
                    // per-dot gate repeats it exactly at delivery — this just
                    // keeps refused slots from costing budget). Known v6.59
                    // inconsistency: the gate tests (y = tier + zoff,
                    // z = scz + zoff) while word_dots paints (y = tier,
                    // z = slot-offset + zoff); the gate is only a budget saver.
                    let hi_y = HI_TIERS[rank];
                    let lo_y = LO_TIERS[rank];
                    let d_hi = dec_ok_reason(scx, hi_y + zoff, scz + zoff, eye, look, sky_visible);
                    let d_lo = dec_ok_reason(scx, lo_y + zoff, scz + zoff, eye, look, sky_visible);
                    match d_hi.max(d_lo) {
                        1 => self.diag.dec_sky += 1,
                        2 => self.diag.dec_band += 1,
                        3 => self.diag.dec_cone += 1,
                        _ => self.diag.dec_ok += 1,
                    }
                    if d_hi > 0 || d_lo > 0 {
                        continue;
                    }
                    let (hw, cw) = crate::stream::Streamer::encode_words(&cell);
                    // HI word: height byte (bits 8..15) + window tag
                    // (bits 4..7); the low nibble never paints.
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

    /// Lap bookkeeping: sweep reset at lap boundaries. w = lap % 16.
    pub fn window(&self) -> u32 {
        ((self.ticks / LAP_TICKS) % WINDOWS as u64) as u32
    }

    pub fn reset_sweep(&mut self) {
        self.sweep = [0; RINGS];
    }
}

/// Coarse albedo for an ingested material (water keeps its lattice index
/// through Cell::water; land uses the palette-lattice coarse classes).
fn column_rgb(material: &str) -> [u8; 3] {
    match material {
        "grass_block" => [98, 146, 74],
        "stone" => [125, 125, 125],
        "sand" => [219, 207, 163],
        "snow" | "snow_block" | "powder_snow" => [249, 255, 254],
        "water" => [63, 118, 228],
        _ => [112, 112, 112],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
                });
            }
        }
        c.ingest_columns(origin, &cols);
        // The 16 m chunk spans one ring-0 cell; the 8×8 columns (2 m apart)
        // all land in it and the LAST one wins — height 70+7 = 77.
        let cell0 = c.rings[0].get((cx * 16 / 16) as u32, (cz * 16 / 16) as u32).expect("r0 cell");
        assert!(cell0.valid);
        assert_eq!(cell0.height, 77);
        assert!(!cell0.water);
        // Ring 1 (64 m cells): the same chunk lands in cell (0,0) too.
        let cell1 = c.rings[1].get(0, 0).expect("r1 cell");
        assert!(cell1.valid);
        assert_eq!(cell1.height, 77);
    }

    #[test]
    fn word_dots_geometry_matches_the_shader_contract() {
        // Shader probe (cascade_capture.fsh): P = (slotCtr.x + (qx-1.5)*sub,
        // planeY, slotCtr.y + (qz - (slots-1)/2)*sub + zoff) and
        // word |= qx << (14 - 2*qz) — painting must invert to the word.
        let mut out = Vec::new();
        let word: u16 = 0xBAB0; // height byte 0xBA (bits 8..15), tag 0xB (bits 4..7)
        word_dots(word, 100.0, 200.0, 6, 120.0, 8.0, &mut out);
        assert_eq!(out.len(), 6);
        let mut decoded = 0u16;
        for (qz, p) in out.iter().enumerate() {
            let qx = ((p.x - 100.0) / SCAFFOLD_SUB + 1.5).round() as u16;
            assert!((0..4).contains(&qx), "qx {qx} out of the sub lattice");
            let z_expect = 200.0 + (qz as f64 - 2.5) * SCAFFOLD_SUB + 8.0;
            assert!((p.y - 120.0).abs() < 1e-9, "slot {qz} y {}", p.y);
            assert!((p.z - z_expect).abs() < 1e-9, "slot {qz} z {} vs {z_expect}", p.z);
            decoded |= qx << (14 - 2 * qz);
        }
        assert_eq!(decoded, word);
    }

    #[test]
    fn beacon_byte_checksum_roundtrips() {
        // Beacon byte = (w*17)&0xFF: hi nibble == lo nibble, decoded by the
        // beacon pass as w = hi nibble. Every window 0..15 must round-trip.
        for w in 0..16u32 {
            let byte = ((w * 17) & 0xFF) as u8;
            assert_eq!(byte >> 4, byte & 0xF, "window {w}");
            assert_eq!(byte >> 4, w as u8);
        }
    }
}
