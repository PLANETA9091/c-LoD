# c-LoD v6.59 cascade — recovery notes (written from agent context after the sandbox recycle)

Date: 2026-11-20. The sandbox recycle wiped the local git lineage v6.11→v6.59.8y
(commits 9971e5a…d4fe696 are gone from objects/reflog), the untracked sources
`src/cascade.rs` (~1200 lines) and `src/stream.rs`, the runtime stand
`/home/z/mc-test` (paper 26.3, launcher.jar, world), `/home/z/tools` (jdk25, cargo,
glslang) and `c-LoD/tmp549, tmp550/` run artifacts. `download/` survived
(cascade-evidence/ = 18 files). GitHub PLANETA9091/c-LoD only has the ancient
v6.10 lineage; the local tree now sits on `ci: roll 10` (7c6f7a18f, another
branch of the same project's history — do not delete).

This file reconstructs everything the agent context still holds verbatim, so a
future session can rebuild the v6.59 cascade line.

## 1. Architecture recap (v6.59 THE CASCADE)

Goal: render distant terrain from a stat screen (no chunks streamed to the
client) by painting GREEN DUST particles (happy_villager / dust carrier) in
world space near the camera; the client's post-chain (the "march") projects,
detects, decodes words into a persistent atlas, and renders terrain from it.

- Server (Rust module `liblod.so` loaded by `launcher.jar` inside dist/):
  cascade rings r0..r5, cell sizes 16/64/256/1024/4096/16384 m, grid 256×256
  cells per ring (4096 m span at r0). ring_origin(cfg) = (center_x −
  region_blocks/2, center_z − region_blocks/2); live config
  `region_blocks=512;cell_size=2;tick_ms=100;particles_per_tick=2048;
  scan_budget=4096;http_port=8199;center_x=-930;center_z=796`.
  So ring origin = (−1186, 540) — NOT (−2978,−1252): the earlier window-11
  world-square calc used 4096 and is WRONG; redo with 512.
- Stream: per lap (300 ticks), lap%16 = window w; window_origin(w) =
  ((w%4)*64, (w/4)*64); windows partition the 256×256 cell grid (16 windows
  of 64×64). Beacon broadcasts w; cell words are painted in anchor-local
  slots; the atlas decodes only the broadcast window each lap (early-out
  `rwin.r*16 != wExp`).
- Anchor: `anchor = floor(eye/128)*128` (128 m cells); slots at
  anchor + (si+0.5)*2 m for si=0..63 (BEACON_SLOT = (63,63) → ctr =
  anchor + (63.5, 63.5)*2).
- Planes: LO_TIERS [80, 140, 173, 193], HI_TIERS [120, 180, 175, 192.5]
  (height word on HI_TIERS[rank], color word LO_TIERS[rank], RS (edit
  reset) planes between them, LO = HI − 40 per comment).
- zoff = ringIdx * ZSTRIDE (ZSTRIDE = 4.0 m) — rings separate in Z.
- Word: 16 bits = 8 slot-pairs; server paints 6 dots for HI
  ((hw&0xFF00)|(tag<<4), slots 0..5 carry bits 14..3), 4 dots for LO
  (cw&0xFF00). Dot for slot s: X = cx + (bits−1.5)*SCAFFOLD_SUB,
  Y = plane, Z = cz + (s−(slots−1)/2)*SCAFFOLD_SUB + zoff,
  SCAFFOLD_PITCH = 2.0, SCAFFOLD_SUB = 0.5 (pitch/4).
- encode_words(cell): h = (i32(cell.height) + 64).clamp(0,255) as u16;
  (h << 8, cell.color_index() << 8). The march reads h0 = byte − 64 — the
  ±64 is symmetric.
- Beacon byte = (w*17)&0xFF (hi nibble == lo nibble checksum), painted as
  4-slot word at beacon slot on HI_TIERS[rank] planes with per-ring zoffs
  k*4, k=0..5.
- Client post chain (`assets/post_effect/lod_terrain*.json` + `assets/glsl/`):
  capture passes: 3.0 beacon (1×1) → probe 6 zoffs × 4 tiers, byte = w>>8,
  checksum hi==lo → writes rwin (w/16, conf, bestSeen/16, beaconHits/255).
  0.0 HI atlas (2048×256, 256 px per ring): 6-slot decode, tag gate
  ((w>>4)&0xF == wExp), merge: complete → replace+mark, partial → OR into
  unmarked. 1.0 LO atlas: 4-slot, vouched by height twin alpha.
- The march (lod_terrain.fsh) reads the atlas (ColBuf/HeightSampler),
  h0 = byte − 64, 2×2 valid-neighbour average, renders the horizon wall.

## 2. Verbatim code fragments (from the lost files, as seen in context)

### src/cascade.rs — word_dots
```rust
pub const SCAFFOLD_PITCH: f64 = 2.0;
pub const SCAFFOLD_SUB: f64 = SCAFFOLD_PITCH / 4.0;
pub const ZSTRIDE: f64 = 4.0;
pub const BEACON_SLOT: (u32, u32) = (63, 63);

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
```

### src/stream.rs — tiers + encode + word_spawns (4-slot)
```rust
pub const LO_TIERS: [f64; 4] = [80.0, 140.0, 173.0, 193.0];
pub const HI_TIERS: [f64; 4] = [120.0, 180.0, 175.0, 192.5];

pub fn encode_words(cell: &Cell) -> (u16, u16) {
    let h = (i32::from(cell.height) + 64).clamp(0, 255) as u16;
    (h << 8, cell.color_index() << 8)
}

pub fn word_spawns(word: u16, cx: f64, cz: f64, sub: f64, plane_y: f64, out: &mut Vec<Spawn>) {
    for slot in 0..4usize {
        let bits = ((word >> (14 - 2 * slot)) & 3) as f64;
        let dx = (bits - 1.5) * sub;
        let dz = (slot as f64 - 1.5) * sub;
        out.push(Spawn { x: cx + dx, y: plane_y, z: cz + dz });
    }
}
```

### src/cascade.rs — stream_tick core loop (beacon + six rings)
```rust
let ax = anchor_of(eye[0]);
let az = anchor_of(eye[2]);
let mut spent = 0usize;

// ---- the beacon: the window broadcast at every ring's Z offset ----
let beacon_byte = ((w * 17) & 0xFF) as u16;
let bx = ax + (BEACON_SLOT.0 as f64 + 0.5) * SCAFFOLD_PITCH;
let bz = az + (BEACON_SLOT.1 as f64 + 0.5) * SCAFFOLD_PITCH;
for k in 0..RINGS {
    let rank = self.slot_rank(ax, az, BEACON_SLOT.0, BEACON_SLOT.1,
        rank_probe(bx as i32, bz as i32), eye[1]);
    let zoff = k as f64 * ZSTRIDE;
    let word = beacon_byte << 8;
    if decodable(bx, HI_TIERS[rank] + zoff, bz + zoff, eye, look, sky_visible) {
        word_dots(word, bx, bz, 4, HI_TIERS[rank], zoff, out);
        spent += 4;
    }
}

// ---- the six rings, round robin, slot sweep per lap ----
let tag = (w & 0xF) as u16;
let slots_total = (SLOT_SIDE * SLOT_SIDE) as usize;
'budget: while spent < budget {
    let mut progressed = false;
    for k in 0..RINGS {
        if spent >= budget { break 'budget; }
        while self.sweep[k] < slots_total {
            let s = self.sweep[k];
            self.sweep[k] += 1;
            let si = (s % SLOT_SIDE as usize) as u32;
            let sj = (s / SLOT_SIDE as usize) as u32;
            if (si, sj) == BEACON_SLOT { continue; }
            self.diag.slots_examined += 1;
            let cx = wx + si;
            let cz = wz + sj;
            let Some(cell) = self.rings[k].get(cx, cz).cloned() else { continue };
            if !cell.valid { continue; }
            self.diag.cells_valid += 1;
            let scx = ax + (si as f64 + 0.5) * SCAFFOLD_PITCH;
            let scz = az + (sj as f64 + 0.5) * SCAFFOLD_PITCH;
            let rank = self.slot_rank(ax, az, si, sj,
                rank_probe(scx as i32, scz as i32), eye[1]);
            let zoff = k as f64 * ZSTRIDE;
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
            if d_hi > 0 || d_lo > 0 { continue; }
            let (hw, cw) = Streamer::encode_words(&cell);
            // HI word: height byte (bits 8..15) + window tag (bits 4..7)
            let hi = (hw & 0xFF00) | (tag << 4);
            let lo = cw & 0xFF00;
            word_dots(hi, scx, scz, 6, hi_y, zoff, out);
            word_dots(lo, scx, scz, 4, lo_y, zoff, out);
            spent += 10;
            progressed = true;
            break;
        }
    }
    if !progressed { break 'budget; }
}
```
NOTE the known server-side inconsistency: the decodable PRE-GATE tests
(y = tier + zoff, z = scz + zoff) while word_dots paints (y = tier,
z = slot-offset + zoff). The gate is stricter; it only saves budget.

### src/cascade.rs — CDiag
```rust
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
        format!("t{} bk{} sl{} cv{} sky{} band{} cone{} ok{}",
            self.ticks, self.beacon_ok, self.slots_examined, self.cells_valid,
            self.dec_sky, self.dec_band, self.dec_cone, self.dec_ok)
    }
}
```

### src/cascade.rs — dec_ok_reason (start)
```rust
fn dec_ok_reason(x: f64, y: f64, z: f64, eye: [f64; 3], look: [f64; 3], sky_visible: bool) -> u8 {
    if !sky_visible { return 1; }
    let d = [x - eye[0], y - eye[1], z - eye[2]];
    // (band check: 3D distance in [CULL_RADIUS, ZONE_FAR]; cone check: dot
    // with look vs cos_half_cone; 0 = decodable, 1 = sky-blind,
    // 2 = out of the decode band, 3 = outside the frustum cone)
```

### tests (cascade.rs, must keep)
- ring_geometry_quadruples: c.rings.len() == RINGS; rings[0].cell == 16.0;
  rings[1].cell == 64.0; rings[5].cell == 16384.0; half span ≥ 2_000_000.
- windows_partition_the_grid: for (cx,cz) in samples: (wx,wz) = window_origin(w);
  (si,sj) = slot_of(cx,cz); wx+si == cx; wz+sj == cz; w < WINDOWS.
- sample_chunk_is_the_inverse_of_chunk_claim: origins [(0,0),(1000.5,-333.25),
  (-512,4096)] × cells [16,64,16384] × (ccx,ccz) in [(0,0),(7,128),(255,255)]:
  chunk_claim(origin, cell, sample_chunk(origin, cell, ccx, ccz)) == (ccx,ccz).
- ingest_builds_cells_from_chunk_columns: Column { wx, wz, height: 70+gx,
  liquid: false, material: "grass_block" }.
- word_dots_geometry_matches_the_shader_contract (line 845 in v6.59).

### src/cascade.rs — ring_origin
```rust
pub fn ring_origin(cfg: &Config) -> (f64, f64) {
    let half = f64::from(cfg.region_blocks) * 0.5;
    (cfg.center_x.unwrap_or(0.0) - half, cfg.center_z.unwrap_or(0.0) - half)
}
```
Config has `pub region_blocks: i32` (default 512 in v6.59 config parse? —
the CURRENT v6.10 tree's config.rs has region_blocks default 512; the v6.59
lane set it via CLOD_OPTS region_blocks=512; ring cell sizes were hardcoded
16/64/…/16384 independent of region_blocks).

### assets/glsl/cascade_capture.fsh — probe_cword (complete, verbatim)
See recovery-code-fragments.md — the full 367-line shader was read in
context; key parts:
```glsl
void probe_cword(vec2 slotCtr, float planeY, float zoff, int slots, float sub,
                 float depthNear, out uint word, out bool anyHit, out bool allSlots,
                 out float seen) {
    // az/el/fov from RotSampler(0,0): az = r*2PI, el = g*PI - PI/2,
    // fov = 5 + a*115; F/R/U basis; cam = CameraBlockPos + CameraOffset;
    // fog from RotSampler(1,0)/(0,1).
    for (int qz = 0; qz < slots; ++qz) {
        for (int qx = 0; qx < 4; ++qx) {
            vec3 P = vec3(slotCtr.x + (float(qx) - 1.5) * sub, planeY,
                slotCtr.y + (float(qz) - (float(slots)-1.0)*0.5) * sub + zoff);
            // project; skip if z<2 or |ndc|>1; seen += 1 per frustum hit
            // 4-tap corner ring (±3 px) for ringGB/ringLum
            // dust detect at sc + 0.75px cross: delta=(raw.g-raw.b)-ringGB>0.06,
            // absolute: pc.g>0.40 && pc.g>pc.r+0.08 && pc.g>pc.b+0.08,
            // relative: delta && raw.g>raw.r+0.02,
            // mirror: depthNear>0 && (ringLum-lum)>0.06 && |g-b|<=0.08 && |g-r|<=0.08 && zScene>=z
            if (dust) { word |= uint(qx) << (14 - 2 * qz); rowHit=true; anyHit=true; }
        }
        if (rowHit) rowComplete += 1.0;
    }
    allSlots = rowComplete >= float(slots);
}
```
slot_center:
```glsl
vec2 slot_center(vec2 pxCell, out int ringIdx, out float wExpected) {
    ringIdx = int(floor(pxCell.x / 256.0));
    float cx = mod(pxCell.x, 256.0);
    float cz = pxCell.y;
    wExpected = floor(cx / 64.0) + floor(cz / 64.0) * 4.0;
    vec2 slot = vec2(mod(cx, 64.0), mod(cz, 64.0));
    vec2 anchor = floor(vec2(CameraBlockPos) / 128.0) * 128.0;
    return anchor + (slot + 0.5) * 2.0;
}
```
beacon mode (mode >= 2.5): ctr = anchor + (63.5,63.5)*2; probe k=0..5 (zoff
k*4) × r=0..3 tiers, slots=4; byte = w>>8; checksum hi_nib==lo_nib →
wOut, conf=1; fragColor = vec4(wOut/16, conf, bestSeen/16, hits/255).
atlas modes: zoff = ringIdx*4; early-out on rwin (g>0.5, r*16 != wExp →
passthrough prev); blind camera passthrough when rotP.b < 0.25;
hiPass slots=6 tierY(TiersHi,r), else slots=4 tierY(TiersLo,r);
HI honesty: tag == wExp; LO honesty: HeightRef alpha >= 0.5;
merge: allSlots → outByte=fresh, mark=1; else anyHit&&!marked → outByte |=
fresh; fragColor = (outByte/255, eraseFlag, fresh/255, mark).

### lod_terrain.fsh — ground_at0 (verbatim core)
```glsl
float h0 = h.r * 255.0 - 64.0;
// 2x2 cell average over valid neighbours (nb = (1,0),(0,1),(1,1)),
// data frontier stays exact; water flag from ColBuf (a>=0.5, r*255==216).
```
March comment ledger: surface height; water FLAG in the color word (idx 216);
v6.21.0 the COLOR word no longer gates the GEOMETRY; mid-band 80..120 m
decodes height word only; water stays color-gated; census: x=309.r = MAX raw
height byte over the band, x=310.r = MAX color-word ALPHA (254 = decoded).

## 3. Run evidence & verdicts (#543 → #550)

- #543: full fill r0=100% in ~10 min, atlas 8.6%, marsh alive conf=0.63,
  frames shot live. Wall = dark-green fallback gradient (empty atlas cells
  painted fallback).
- #544: lap=40 broke the beacon (23x slots ok server-side, atlas 0%) →
  lap=300 kept forever after; the lap must outlive the dust lifetime
  (~60-80 ticks).
- #547: SWEEP_WAIT=1500; atlas 71.4% (r1H 71.4/r2H 71.0/r1C 70.2/r3H 71.4),
  mx=mn=186, beacon w=11 (11*17=187=0xBB → checksum OK), seen=72%.
  Frames NOT shot (OOM killed the box before).
- #549 (client 512M): client froze on keep-alive at 20:18:02 during the pack
  apply; server kicked at 20:18:44; every shot = Connection Lost; the strip
  kept accumulating from the frozen last frame (conf 0.63→0.06 = dead-camera
  marker). Also paper 384M starved chunk gen (fill 47.5% in 12 min).
  FIX APPLIED: client -Xmx768M -XX:MaxDirectMemorySize=256M (run_client.sh:115),
  paper -Xmx512M (boot_server.sh DIST_JAVA_OPTS), launcher stays -Xmx384M.
- #550 (v6.59.8x): fill 100/28.3 in 13 min, sweep 25 min, ZERO disconnects,
  GATE GREEN (r0=100.0 floor 90, r1=28.3 floor 25), atlas 71.4% all slices
  (r1C=70.2%), beacon w=11, seen=72%, conf=0.73 ALIVE. Frames shot on a live
  world — but the shot vantage (CX,150,CZ,yaw0,pitch0) looked over the
  valley: sky in frame, trees at the edges. Vantage FIXED to
  (CX,165,CZ,0,30) in run_cascade_probe.sh line 240 (pushed as v6.59.8y).
- Strip layout (monitor 640×360, row h−3, x=200..208): r1H, r2H, rot(conf),
  r1C, r3H, rwin(w), census, seen. Decode: mark%=r/255, mx=g, mn=b;
  rot conf=b/255; rwin w=r*16/255 (floor(r*16+0.5) with +0.5 rwin offset);
  seen=r/255.
- The constant-186 question: mx=mn=186 (=0xBA = height 122 m under ±64) at
  71% marked, LIVE camera. Two hypotheses: (a) window 11's cells really are
  a flat 122 m plain → everything works, only frames needed; (b) the decode
  (or the ingest!) is constant. #549's strip mx=39/mn=13 was the frozen-frame
  artifact, NOT live decode. The atlas accumulates across all 16 windows
  (rwin early-out decodes one window per lap, ~100 laps per sweep), so 186
  everywhere at 71% leans (b)-ish unless the whole 4-km grid is 122 m.
  NEXT VERDICT PATH (no cargo needed): graceful-stop the server (echo stop >
  /tmp/server.in) so Paper writes overworld regions to
  dist/world/region/*.mca (they NEVER hit disk while running — checked),
  parse heightmaps MOTION_BLOCKING (36 longs, 9-bit packed, 256 values,
  surface = value−1) with a hand-rolled .mca/NBT parser (anvil-parser2 pip
  install does NOT survive recycles), map cells via ring_origin = center −
  256 (region_blocks=512!), print per-window min/max/mean over the 16
  windows. If window 6 (the vantage ocean, world ≈ (−336,−128) area) shows
  ~63 m while the atlas says 186 → bug confirmed (ingest or decode).
- cargo vanished from the sandbox twice; rustup install did NOT happen.
  The planned module endpoint (window heights on HTTP 8199, engine.rs) was
  never built.

## 4. Harness recipes (lost, but reconstructible)

- boot_server.sh: Xvfb :99 already needed by client; DIST_JAVA_OPTS
  "-Xms256M -Xmx512M -XX:MaxMetaspaceSize=256M -XX:MaxDirectMemorySize=128M
  -XX:+UseG1GC"; two boot paths (paper-26.3.jar direct / launcher.jar with
  CLOD_OPTS above; console via `tail -f /tmp/server.in | java -jar … >
  /tmp/server.log`); send commands with `echo stop > /tmp/server.in`.
- run_client.sh: Xvfb :99 (640×360), xdotool-driven join, offline profile
  "Stepan", `-Xmx768M -XX:MaxDirectMemorySize=256M`; F1 toggles HUD;
  strip read via ffmpeg x11grab frame + PIL.
- run_cascade_probe.sh (the lane): boot → pack prompt accept loop (cached
  pack auto-accept note) → fill loop R0_FLOOR=90 R1_FLOOR=25, DEADLINE 12
  min, cov via curl http://127.0.0.1:8199/clod-pack.info (parses
  `cascade r0=..% r1=..%`) → sweep vantage tp (SWEEP_X=-336 SWEEP_Y=113
  SWEEP_Z=-128 yaw 0 pitch 8), SWEEP_WAIT (s) → march-alive poll (strip
  x=202 vs hotbar) → shots (aerial, aerial_zoom, burst, horizon,
  horizon_zoom; shot vantage tp $CX 165 $CZ 0 30 FIXED) → GATE → pkill.
  center: CX=-930 CZ=796. F1 = key 67 via press_key.py (xdotool).
- press_key.py: xdotool key <code> wrapper (F1 = 67).
- w11_check.sh: gamemode spectator + tp 500 200 1800 / 800 200 2100 + two
  ffmpeg shots (saved in scripts/bench/, committed as v6.59.8y).
- Box memory ledger: client 768M + paper 512M + launcher 384M + system ≈
  2.9G of 4G — keep ~1G headroom; llvmpipe rendering is CPU-bound.
- Evidence dir: /home/z/my-project/download/cascade-evidence/ (18 files from
  #549/#550: cascade_aerial_537 (pack-prompt era), cascade_aerial_550 (sky
  over valley, live world), mon_s2.png (strip 71%), w11_check*.png etc).

## 5. Reconstruction plan (next sessions)

1. Environment: reinstall jdk25 (tools/), rustup/cargo, glslang (shader
   validation), paper 26.3 jar; rebuild mc-test stand (dist/, launcher.jar
   via build.sh + java/ sources — java/ survived in the repo tree).
2. Reconstruct src/cascade.rs + src/stream.rs from §2 fragments + the
   v6.10-lineage files as skeleton (engine.rs http server exists in tree;
   Cell/Column types live in stream.rs — must be rebuilt around encode_words
   and the ingest tests). The unit tests in §2 pin the geometry.
3. Rebuild assets/glsl v6.59 shaders: cascade_capture.fsh is nearly complete
   in context (§2); lod_terrain.fsh (1195 lines) only partially — the march
   needs a functional rewrite: ground_at0 + the cascade capture JSONs
   (post_effect/lod_terrain_cascade.json etc) + clod grid/tiers/stream UBOs.
4. Rebuild the lane scripts (§4 recipes), re-run #551 with the FIXED
   vantage (165/30) — the deliverable frames.
5. Verdict on 186 via §3 graceful-stop + .mca parse (no cargo needed — do
   it FIRST on the fresh stand).
