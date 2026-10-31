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
 │    LIVE EDITS (v6.2): the rescan wave re-samples stale cells of loaded chunks     │
 │      (cheap height probe → full sample only when the surface moved), so blocks    │
 │      placed after server start reach the LOD without a relog                      │
 │    EVENT PUSH (v6.3): a Bukkit listener (stable PluginManager API) feeds every     │
 │      block change into a ring the engine drains NEXT tick — edits reach the       │
 │      dirty phase in ONE engine tick; the wave stays as the backstop               │
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
              ▼
   v6.0 NATIVE DUST WIRE: the per-tick batch rides ONE vanilla
   ClientboundBundlePacket per player — one frame, one encode, one
   Netty write; particle packets are built once per position and
   cached (src/wire.rs), so the steady state allocates nothing.

 ┌──────────────────────────── VANILLA CLIENT 26.3 ──────────────────────────────────┐
 │  CAMERA CHANNEL:                                                                  │
 │  sky.fsh override — vanilla sky program + payload: the sky stage is the only       │
 │  one holding the true camera rotation (ModelViewMat) and FOV (ProjMat[1][1]);      │
 │  it encodes az/el/fov into the ALPHA of the leftmost 8 sky columns (RGB is         │
 │  untouched — the channel is invisible on screen).                                  │
 │                                                                                    │
 │  clod:lod_terrain  (every frame, 10 passes)                                        │
 │   1. col_scan    — 640x1 micro-pass: scan the column top-down for the topmost      │
 │                    intact payload row (magic + checksum)                           │
 │   2. rot_solve   — 1x1: decode (az, el, fov); confidence decay only while          │
 │                    fully occluded; reconstructs the day tick from live FogColor    │
 │   3. rot_copy    — ping-pong the rotation target                                   │
 │   4. capture ×5  — project the live dust particles to the screen (depth-strobe:    │
 │                    only geometry-backed dots decode), merge the 16-bit words       │
 │                    into persistent clod_h_a/b + clod_c_a/b (256x256 each);        │
 │                    pass 0 rides the v6.2 RESET plane — an edited cell REPLACES    │
 │                    its word once the fresh decode completes (the OR-merge alone   │
 │                    can never lose bits)                                           │
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
| Server-side particles: `CraftPlayer.getHandle()` → `ClientboundLevelParticlesPacket(options, force=true, …)` straight into `ServerGamePacketListenerImpl.send` (live-verified since v5.1); v6.0 wraps the whole per-tick batch into one `ClientboundBundlePacket` per player — vanilla wire, vanilla codec, ctor resolved by a signature ladder with automatic legacy fallback | `net.minecraft.network.protocol.game.ClientboundBundlePacket` |

On 26.3's reversed-Z depth, the sky reads `device depth == 0.0`. The **view basis travels through the frame itself**: the sky shader computes `yaw/pitch/fov` from `ModelViewMat`/`ProjMat` and packs them into the invisible alpha of the leftmost sky columns; the chain scans for the topmost intact row (magic + checksum), so any occlusion self-heals frame-exact — **no sun, no moon, no time-of-day requirement, no warm-up**, and the real per-client FOV (spyglass zoom included) rides along for free.

## The bit-stream in one paragraph

The server wants the client to know a 256×256-cell heightfield/colorfield. It cannot send textures (the pack is pushed once and stays static), so it **broadcasts the data through the only always-on, client-visible channel vanilla Paper gives a module: particles.** Each 16-bit word is painted with 4 dust particles whose X offsets within a cell carry 2 data bits each and whose Z offsets pick the slot; a word on the HI plane (Y=120) is one byte of height, a word on the LO plane (Y=80) one color index (6×6×6 palette, water = 216). The capture pass projects all 16 sub-positions of every live word to the screen, keeps only dots that sit in front of geometry (depth strobe — sky-backed dots would be invisible data anyway), and merges them into persistent ping-pong buffers. The march then raymarches whatever has decoded so far — the LOD grows in as the stream laps the disk. The stealth gate (v5.3.1) spawns particles only where they are on-screen, decodable and erasable: behind the camera, above the horizon band or inside the near dome nothing spawns at all, and the dust mask pass erases the carrier pixels the same frame they are read.

**Live edits (v6.2).** The OR-merge that makes partial decodes self-heal can never *lose* bits — a re-streamed changed word would corrupt against the old one. So the server watches the world: a rescan wave (`rescan_ticks`, default 40) re-samples the loaded chunks' cells (one cheap `getHighestBlockYAt` probe per cell; the full color sample only when the surface actually moved), a changed sample marks the cell dirty, and the dirty stream carries one extra **reset electron** on a third plane (Y=100). The capture's reset pass flags that cell for the frame; a flagged cell **replaces** its stored word the first time the fresh decode completes (all four slots strobed). A placed tower, a mined hill, a burned forest — all of it reaches the LOD in seconds, no relog, no repack. v6.3 tightens detection to **one engine tick**: a Bukkit event listener (13 block-event types, stable `PluginManager.registerEvents` surface, no NMS internals) pushes every changed column into a ring the engine drains before the stream step, so the edit's dirty cell rides the very next batch with its reset electron. The rescan wave stays as the backstop (and the sole detector on plugin-less servers).

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
| `rescan_ticks` | `40` | v6.2 LIVE EDITS: ticks between rescan waves; the wave re-samples stale cells of loaded chunks (0 = off, the v6.1 paint-once behavior) |
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
| `native_dust` | `true` | v6.0 NATIVE DUST WIRE: the per-tick dust batch rides ONE `ClientboundBundlePacket` per player built from cached packets (false = the v5 per-particle path) |
| `native_trees` | `true` | v6.1 SEEDGEN TREES: the real vanilla VEGETAL_DECORATION pass (trees, leaves — exact `addVanillaDecorations` recipe) runs on top of the native walk, so never-generated forest chunks stream with tree tops from the seed alone (false = terrain-only v5.9 output) |
| `native_edits` | `true` | v6.3 EVENT PUSH: block events feed the dirty phase within ONE engine tick (needs ≥1 enabled plugin as the registration host; without one the rescan wave stays the detector) |

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
- **Native dust wire** (v6.0): the stream itself stopped paying per-particle Java tax. The legacy path crossed the JNI boundary **four times per particle per player** (getHandle + connection field + NMS record constructor + vanilla send → one Netty write each). The v6.0 wire builds each unique particle packet **once** (positions are exact lattice points — `src/wire.rs` keys them by their exact i64 lattice coordinates), caches it as a global ref, and ships the whole gated batch as **one `ClientboundBundlePacket` per player** — a single vanilla frame, a single encode, a single Netty write per player per tick. Steady state after the first lap: ~100% cache hits, **zero Java objects per particle**; the client decodes byte-identical packets (same constructor, same arguments), only the envelope changed. Delivery keeps the full v5.3.1 stealth gate, chunked at 1024 packets per bundle, and falls back to the per-particle path automatically (config `native_dust`, or ≥3 consecutive bundle failures for a player).
- **Seedgen trees** (v6.1): the last fidelity gap of native generation closed — forest chunks the server NEVER generated now stream with tree tops straight from the seed. After the terrain walk the helper replays the exact vanilla `addVanillaDecorations` recipe for `VEGETAL_DECORATION` (`setDecorationSeed(seed, originX, originZ)` → per feature `setFeatureSeed(decorated, globalIdx, step)` → the real `FeaturePlacer.placeWithBiomeCheck`) against a dynamic-proxy `WorldGenLevel` backed by the walked terrain: `setBlock` records motion-blocking writes through the REAL vanilla heightmap predicates into live per-block tops, `getHeight`/`getBlockState`/`getBiome` answer from the model, the biome filter runs on the real `BiomeManager`. Because every feature is freshly seeded per chunk, running only our chunk's biomes is bit-equivalent to vanilla's 3×3 palette union. Cell cost: a few percent of the noise walk (forest ~1–3 ms/chunk); any throw degrades that chunk to the terrain-only output (the v5.9 cells, proven bit-identical by the preflight).
- **Live edits** (v6.2): blocks placed after server start reach the LOD **without a relog**. The scan disk used to be paint-once — a valid cell was never re-sampled, and the client's OR-merge could not even *express* a decrease. Now the rescan wave re-waves the disk every `rescan_ticks` (40 by default): only loaded chunks are visited (nothing can change in an unticked chunk, so those cells cost one cached probe per wave), the cheap height probe short-circuits nearly every steady-state cell, and a moved surface pays one full color sample. The changed words ride the dirty phase with a reset electron; the client replaces the cell word atomically on the first complete decode. Detection latency ≤ `rescan_ticks` (≤ 2 s at the default), propagation is immediate once the cell is decodable.
- **Event push** (v6.3): edit detection dropped from the rescan wave's `rescan_ticks` to ≤ 1 engine tick. A runtime-defined listener class (the seedgen defineClass pattern, bytecode compiled against the JDK + the preflight mock tree) registers 13 block-event handlers through the STABLE Bukkit plugin API and packs every affected column (x: 26 bits | z: 26 bits) into an 8192-slot ring; the engine drains it before the stream step, re-samples those columns immediately and the dirty phase streams them in the same batch. Registration needs one enabled plugin as the host; 31-check preflight proves the ladder (hard-fail/pending/registered), the world guard, the packing and the multi-block payloads against mocks — and on any failure the channel degrades to the wave, never the other way round.
- **One push, forever**: the pack SHA-1 is byte-deterministic across restarts; clients never re-download.

## Development

```bash
cargo test --release          # 72 unit tests (stream/engine/config/packgen/palette/sha1/http/zipw/seedgen/wire)
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
bash scripts/check_glsl.sh    # glslangValidator: every shader + both chain JSONs
```

The full verification pyramid — CI layers, boot gates G1–G10, the real-client harness and the evidence archive — is mapped in [TESTS.md](TESTS.md). CI runs rustfmt, clippy (`-D warnings`), the full test suite, a bytecode-freshness check for the embedded seedgen helper (`java/ClodSeedgen.class` recompiled and diffed against the commit), glslang validation, the pack-integrity assertions (12→13 entries, **zero** `.png`/`textures/`, capture chain present, no texture inputs), release builds on Linux/macOS/Windows, the CRUSSTY module bundle — and an **e2e live-server job** on `master`: a real Paper 26.3 kernel on Temurin 25 with the CRUSSTY runtime built from source, two boots (fresh world + restart) asserting the pack fetch, the gates and byte-exact SHA-1 reproduction.

## Honest limitations (v6.3)

- **The camera channel needs sky in the leftmost screen columns.** Facing a wall of terrain (or being fully indoors) the chain holds its last estimate with a slow decay — the LOD fades out instead of popping, and the atmosphere chain keeps working (it never needs rotation). Any sky row in the leftmost 8 columns re-locks the basis within one frame.
- **A cold world starts nearly empty — for seconds, not minutes.** SEEDGEN (v5.7) fills never-generated chunks natively at ~35× the ticket frontier, and v5.8/v5.9 keep shaving the per-chunk overhead (walk inside the JVM → batched JNI → versioned tables + identity caches + stride math), so a fresh 512-block disk completes in well under a minute on a small VPS. Generated cells are `provisional`: terrain-exact, but trees/carvers/structures only arrive when the server generates the real chunk (the v6.2 rescan wave or the loaded-chunk sampler upgrades them then). The helper degrades gracefully: bytecode that fails to define or throws on the host's Paper build falls back to the per-column v5.7 path automatically.
- **Edits propagate in seconds, not frames, and only where someone can decode them.** With the event channel live (v6.3, needs ≥1 enabled plugin) detection is ≤ 1 engine tick (`tick_ms` is the floor); otherwise it waits for the rescan wave (≤ `rescan_ticks`); the dirty stream needs a decodable viewer (the persistent buffers hold the words otherwise), and the word replacement needs one complete 16-dot decode — a half-occluded cell heals on a later lap instead. A same-height top-block material swap (wool→concrete at one Y, no height change) stays unpainted until the height moves: the wave's cheap probe compares heights only.
- **The carrier is physics, not magic.** Dust particles are real entities on the wire: the stealth gate erases them from every conventional viewpoint, but a client parked at an extreme vantage can still spot sparse dots (and each edit ships one extra reset electron for ~1 s).
- **The bundle wire leans on vanilla bundle decoding.** If a future 26.3.x changes `ClientboundBundlePacket` (name, constructor shape, or client-side bundle guards), the wire surface stops resolving and delivery falls back to the proven per-particle path automatically — the LOD keeps working at the v5 cost profile.
- **No client→server feedback channel exists on vanilla 26.3** — the server paces the stream blind (FOV cone + decode band + delta-zone scheduler approximate what a real ACK channel would give).

## License

MIT
