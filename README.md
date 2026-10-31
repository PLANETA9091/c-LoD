# c-LoD

**True server-side level-of-distant-detail terrain for Minecraft Java 26.3 ("Wilderness Bound") on Paper — as a [CRUSSTY](https://github.com/PLANETA9091/CRUSSTY) module written in Rust.**

No client mods. **No Display entities. No textures in the pack.** The vanilla client renders the distant world itself, driven entirely by the `/posteffect` command + a one-time shader-only resource pack — every byte of world data streams live as sky-grid dust particles.

```
 ┌─────────────────────────── SERVER (c-LoD module, Rust) ───────────────────────────┐
 │                                                                                   │
 │  heightmap scan (JNI, budgeted per tick)                                          │
 │    SEEDGEN (v5.7–5.9): chunks the server never made generate NATIVELY on our      │
 │      own daemon threads — the vanilla NoiseChunk + MaterialSystem walk runs       │
 │      INSIDE the JVM (v5.8: one JNI call per chunk, was ~1500/column;              │
 │      v5.9: one call per BATCH of chunks, versioned intern tables, identity        │
 │      caches for the per-cell interning, affine-stride y-walk), no tickets,        │
 │      no chunk-system contention, zero TPS impact; cells land provisional          │
 │      and upgrade from the real chunk once the server makes it                     │
 │    FORCED tickets stay as the fallback frontier (gen_budget / gen_wish, v5.5)     │
 │    delta-zone scheduler (v5.4): the stream pays only for decodable cells + edits  │
 │        │                                                                          │
 │        ▼                                                                          │
 │  SKY-GRID BIT-STREAM (src/stream.rs)                                              │
 │    16-bit word = 4 dust particles (X-sub = 2 data bits, Z-sub = slot)             │
 │    HI plane (Y=120) carries the height byte, LO plane (Y=80) the color index      │
 │    2048 particles/tick → a full 256x256-cell disk per ~64-tick lap;               │
 │    cone-priority refill (v5.3.3), stealth spawn gate (v5.3.1)                     │
 │        │                                        │                                 │
 │        ▼                                        ▼                                 │
 │  PlayerPostEffects.add(Key)      resource pack: 2 post chains + GLSL, ZERO PNGs   │
 │  Player.setResourcePack(...) ◄── byte-deterministic SHA-1, pushed ONCE per join   │
 │  (v5.6: the disk arms and warm-scans BEFORE the first player joins)               │
 │  embedded HTTP: GET /clod-pack.zip | /clod-pack.sha1 | /clod-pack.info            │
 └────────────┬─────────────────────────────────────────────────────┬────────────────┘
              │  ClientboundPostEffectsPacket                        │  pack download (once)
              ▼                                        ServerLevel.sendParticles (NMS bridge)
 ┌──────────────────────────── VANILLA CLIENT 26.3 ──────────────────────────────────┐
 │  CAMERA CHANNEL:                                                                  │
 │  sky.fsh override — vanilla sky program + payload: the sky stage is the only       │
 │  one holding the true camera rotation (ModelViewMat) and FOV (ProjMat[1][1]);      │
 │  it encodes az/el/fov into the ALPHA of the leftmost 8 sky columns (RGB is         │
 │  untouched — the channel is invisible on screen).                                  │
 │                                                                                    │
 │  clod:lod_terrain  (every frame, 9 passes)                                         │
 │   1. col_scan    — 640x1 micro-pass: scan the column top-down for the topmost      │
 │                    intact payload row (magic + checksum)                           │
 │   2. rot_solve   — 1x1: decode (az, el, fov); confidence decay only while          │
 │                    fully occluded; reconstructs the day tick from live FogColor    │
 │   3. rot_copy    — ping-pong the rotation target                                   │
 │   4. capture ×4  — project the live dust particles to the screen (depth-strobe:    │
 │                    only geometry-backed dots decode), merge the 16-bit words       │
 │                    into persistent clod_h_a/b + clod_c_a/b (256x256 each)          │
 │   5. dust_mask   — project live sub-positions again → carrier erase mask           │
 │   6. lod_terrain — raymarch the decoded persistent heightfield over sky pixels:    │
 │                    distant terrain, water, snow, fog + inpaint masked pixels       │
 │   7. blit        — composite back into minecraft:main                              │
 │  clod:atmosphere  — depth veil over real terrain: hides the render-distance edge   │
 │                    (depth-only, never needs rotation)                              │
 └────────────────────────────────────────────────────────────────────────────────────┘
```

## Why this works on a vanilla client

Every piece below was verified against the **unobfuscated 26.3 client jar** (Mojang maps are shipped raw since 26.3):

| Fact | Source |
|------|--------|
| `ClientboundPostEffectsPacket(List<Identifier>)` → `LocalPlayer.setActivePostEffects` → `GameRenderer` diffs and runs `PostChain.process(...)` every frame | `ClientCommonPacketListenerImpl`, `GameRenderer`, `net/minecraft/client/renderer/PostChain` |
| Chains load from the server resource pack: `assets/<ns>/post_effect/*.json` + `assets/<ns>/shaders/post/*.fsh` | `PostChainConfig`, vanilla `invert.json` / `entity_outline.json` |
| Post passes can sample the **depth buffer** of `minecraft:main` (`"use_depth_buffer": true`) — the atmosphere veil is depth-only | `PostChainConfig$TargetInput`, `PostPass$TargetInput.getDepthTextureView` |
| Post shaders get the **camera world position** via the `Globals` UBO (`CameraBlockPos` + `CameraOffset`), plus `GameTime` and `ScreenSize` | `include/globals.glsl`, consumer `post/box_blur.fsh` |
| `persistent: true` targets survive across frames → the decoded heightfield accumulates while the client sleeps on it | `PostChainConfig$TargetConfig` |
| Core shaders resolve through the resource system → a server pack can override the **vanilla sky shader** (`pipeline/sky` → shader ids `core/sky`) | `RenderPipelines` (pipeline builder), `ShaderManager` resource loading |
| The sky pipeline binds `Projection` + `DynamicTransforms` UBOs and has **no blending** (`ColorTargetState.DEFAULT`, writeMask 15) — raw RGBA (alpha included) lands in `minecraft:main` | `RenderPipelines` bytecode, `ColorTargetState` |
| 26.3 device depth is `d = depth_near / eyeZ` (reversed-Z, infinite far, sky reads 0.0) — dust particles are opaque and write depth, so the capture strobe accepts only dots backed by geometry | `Minecraft` render globals, verified live on the e2e stand |
| Uniform types `float/int/ivec3/vec2/vec3/vec4/matrix4x4` become `std140` UBOs from JSON; a `vec4` value must be a 4-element list | `UniformValue$Type` |
| Paper API: `Player#postEffects()` → `PlayerPostEffects.add/remove(Key)`, `Player.setResourcePack(UUID,url,sha1,prompt,force)` | Paper 26.3 javadoc |
| Server-side particles: `CraftWorld.getHandle()` → `ServerLevel.sendParticles` + `ParticleTypes.DUST` (`DustParticleOptions.<init>(IF)V`) through a native JNI bridge — no Bukkit reflection per spawn | `org.bukkit.craftbukkit.entity.CraftWorld`, verified: 0 JNI linkage errors |

On 26.3's reversed-Z depth, the sky reads `device depth == 0.0`. The **view basis travels through the frame itself**: the sky shader computes `yaw/pitch/fov` from `ModelViewMat`/`ProjMat` and packs them into the invisible alpha of the leftmost sky columns; the chain scans for the topmost intact row (magic + checksum), so any occlusion self-heals frame-exact — **no sun, no moon, no time-of-day requirement, no warm-up**, and the real per-client FOV (spyglass zoom included) rides along for free.

## The bit-stream in one paragraph

The server wants the client to know a 256×256-cell heightfield/colorfield. It cannot send textures (the pack is pushed once and stays static), so it **broadcasts the data through the only always-on, client-visible channel vanilla Paper gives a module: particles.** Each 16-bit word is painted with 4 dust particles whose X offsets within a cell carry 2 data bits each and whose Z offsets pick the slot; a word on the HI plane (Y=120) is one byte of height, a word on the LO plane (Y=80) one color index (6×6×6 palette, water = 216). The capture pass projects all 16 sub-positions of every live word to the screen, keeps only dots that sit in front of geometry (depth strobe — sky-backed dots would be invisible data anyway), and merges them into persistent ping-pong buffers. The march then raymarches whatever has decoded so far — the LOD grows in as the stream laps the disk. The stealth gate (v5.3.1) spawns particles only where they are on-screen, decodable and erasable: behind the camera, above the horizon band or inside the near dome nothing spawns at all, and the dust mask pass erases the carrier pixels the same frame they are read.

## Install

```bash
crussty install PLANETA9091/c-LoD
# or drop the release bundle into modules/
```

Requires: Paper (or Purpur) 26.3+, Java 25+ runtime, CRUSSTY runtime. The module opens one HTTP port for the resource pack.

## Configuration

Options string passed through the CRUSSTY runtime (`key=value;key=value`). Every knob has a working default — an empty options string produces a functional module:

| Key | Default | Meaning |
|-----|---------|---------|
| `http_port` | `8199` | Port of the embedded pack server (`/clod-pack.zip`, `/clod-pack.sha1`, `/clod-pack.info` — live stream counters) |
| `pack_base_url` | *(auto)* | Public base URL, e.g. `http://play.example.com` |
| `pack_uuid` | fixed | Stable pack UUID (clients re-download only on SHA-1 change — and the SHA-1 never changes: the pack is byte-deterministic and data-free) |
| `pack_prompt` | *(empty)* | Client prompt text |
| `world` | first world | World streamed to the LOD |
| `region_blocks` | `512` | Side length of one streamed disk in blocks (power of two, 256–2048) |
| `cell_size` | `2` | Blocks per LOD cell (power of two, 1–32) |
| `scan_budget` | `4096` | Heightmap cells sampled per engine tick (main-thread JNI budget) |
| `gen_budget` | `16` | v5.5 frontier: new chunk-gen requests admitted per tick (1–256) |
| `gen_wish` | `512` | v5.5 frontier: cap on chunks with an in-flight FORCED ticket (32–4096) |
| `particles_per_tick` | `2048` | Sky-grid pacing: dust particles spawned per tick (64–16384; each cell costs 8 — 4 height + 4 color) |
| `center_x` / `center_z` | *(auto)* | Fixed disk origin in world x/z; empty = the first player's position at arm time |
| `fov` | `70` | Static fallback only — the sky channel measures the real per-client FOV (spyglass zoom works) |
| `lod_near` | `96` | LOD march starts here (keep BELOW the client view distance: real chunks win by depth, LOD fills underneath the chunk edge — no seam) |
| `veil_far` | `512` | Effective maximum distance of the atmosphere veil |
| `depth_near` | `0.05` | 26.3 device-depth near plane (`d = depth_near / eyeZ`) |
| `sea_level` | `63` | Water level |
| `tick_ms` | `1000` | Engine tick interval |
| `effect_terrain` / `effect_atmosphere` | `clod:lod_terrain` / `clod:atmosphere` | Post-effect ids installed into the pack |
| `color_low` / `color_high` / `color_water` / `color_veil` | defaults | Palette, `r,g,b` floats or 0–255 |
| `snow_line` | `0.78` | Snow blend point (normalized elevation) |
| `max_height` | `320` | Terrain ceiling used for ray culling |
| `native_gen` | `true` | SEEDGEN: generate never-generated chunks natively on our own threads (false = legacy FORCED-ticket frontier) |
| `native_threads` | `0` | SEEDGEN worker count; 0 = auto (cores−1, clamped 1–8) |
| `seedgen_batch` | `8` | SEEDGEN v5.9: chunks per JNI batch call (1 = one crossing per chunk, the v5.8 shape) |

Post-effect ids installed by the pack: **`clod:lod_terrain`**, **`clod:atmosphere`** — exactly the identifiers the vanilla `/posteffect` command accepts, so admins can also toggle them by hand:

```
/posteffect add @a clod:lod_terrain
```

## Measured, not promised

All numbers from the real 2-CPU e2e stand (Paper 26.3-207 + genuine vanilla client under Xvfb/llvmpipe), frames archived under `docs/e2e-*/`:

- **Gameplay transparency**: TPS 20.0 under the full 2048-pkt/tick stream (13.3M particles, 0 failed); the stream never touches the world (capture is read-only, the LOD paints only sky-depth pixels, real geometry always wins by depth).
- **Stealth** (v5.3.2 zero-residual protocol): static burst 10/10 frames with **0 carrier pixels**, 75 s spin 0/0/0, teleport hops 0/0/0/0/0, top-down sky-blind view +0 spawns. Worst case from an extreme vantage (Y=280 looking straight into the disk): a sparse scatter of 1–2 px dots.
- **Network**: the spawn gate paces the stream to what the camera can actually decode — 33–800 packets/tick observed vs 2048 constant.
- **Scan throughput** (v5.5 gen-frontier): 35 → 80 cells/s sustained on 2 cores (best 896/s), a warm world re-scans ~49k cells in seconds. A vantage tour accelerates the scan ~15× (teleports load chunks, the ring sweep samples them instantly).
- **Pre-join arm** (v5.6): the disk arms and warm-scans before the first player joins — 32 s ahead of the join, 54874/65536 cells warm at tick 200, 0 carrier pixels on the first horizon frame.
- **SEEDGEN cold-world fill** (v5.7): the full 512×512-cell disk (1024 chunks the server had NEVER generated) filled natively in ~23 s on one worker of a 2-core box — ~2,750 cells/s vs ~80 cells/s for the v5.5 ticket frontier (**~35×**), zero failures, zero tickets, TPS untouched; with `native_threads` workers it scales by cores. v5.8 moved the whole per-chunk noise walk inside the JVM (`java/ClodSeedgen`, one JNI call per chunk instead of ~1500 per column). v5.9 shaves the rest of the overhead: one JNI call per **batch** of chunks (`seedgen_batch`, default 8), intern tables versioned in the payload header (they used to be re-read one JNI call per string every chunk), per-worker identity caches for the per-cell material/biome interning, and an affine-stride y-walk verified per column against the exact v5.8 calls (the preflight proves both paths bit-identical). The per-column v5.7 path stays as an automatic fallback; the CI stand re-measures the fill rate on every master push.
- **One push, forever**: the pack SHA-1 is byte-deterministic across restarts; clients never re-download.

## Development

```bash
cargo test --release          # 61 unit tests (stream/engine/config/packgen/palette/sha1/http/zipw/seedgen)
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
bash scripts/check_glsl.sh    # glslangValidator: every shader + both chain JSONs
```

The full verification pyramid — CI layers, boot gates G1–G10, the real-client harness and the evidence archive — is mapped in [TESTS.md](TESTS.md). CI runs rustfmt, clippy (`-D warnings`), the full test suite, a bytecode-freshness check for the embedded seedgen helper (`java/ClodSeedgen.class` recompiled and diffed against the commit), glslang validation, the pack-integrity assertions (12→13 entries, **zero** `.png`/`textures/`, capture chain present, no texture inputs), release builds on Linux/macOS/Windows, the CRUSSTY module bundle — and an **e2e live-server job** on `master`: a real Paper 26.3 kernel on Temurin 25 with the CRUSSTY runtime built from source, two boots (fresh world + restart) asserting the pack fetch, the gates and byte-exact SHA-1 reproduction.

## Honest limitations (v5.6)

- **The camera channel needs sky in the leftmost screen columns.** Facing a wall of terrain (or being fully indoors) the chain holds its last estimate with a slow decay — the LOD fades out instead of popping, and the atmosphere chain keeps working (it never needs rotation). Any sky row in the leftmost 8 columns re-locks the basis within one frame.
- **A cold world starts nearly empty — for seconds, not minutes.** SEEDGEN (v5.7) fills never-generated chunks natively at ~35× the ticket frontier, and v5.8/v5.9 keep shaving the per-chunk overhead (walk inside the JVM → batched JNI → versioned tables + identity caches + stride math), so a fresh 512-block disk completes in well under a minute on a small VPS. Generated cells are `provisional`: terrain-exact, but trees/carvers/structures only arrive when the server generates the real chunk (the loaded-chunk sampler upgrades them then). The helper degrades gracefully: bytecode that fails to define or throws on the host's Paper build falls back to the per-column v5.7 path automatically.
- **The carrier is physics, not magic.** Dust particles are real entities on the wire: the stealth gate erases them from every conventional viewpoint, but a client parked at an extreme vantage can still spot sparse dots.
- **No client→server feedback channel exists on vanilla 26.3** — the server paces the stream blind (FOV cone + decode band + delta-zone scheduler approximate what a real ACK channel would give).

## License

MIT
