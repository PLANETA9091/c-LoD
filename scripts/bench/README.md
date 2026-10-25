# Real-client E2E bench (local harness)

The CI `e2e-live` job boots the server stack and asserts gates G1–G7, but only
a real vanilla 26.3 client can prove the *visual* half: pack acceptance, the
post-chain compiling, and the camera channel holding at noon (the exact v4.0
killer scenario). This directory contains the full local harness that does
exactly that. It was used to verify v4.1.0 on a genuine vanilla 26.3 client —
see `docs/e2e-2026-10-20-noon/` for the screenshot matrix.

## What's here

| file | role |
|------|------|
| `setup_client.py` | rebuilds the vanilla 26.3 client runtime from the version manifest (66 libraries + natives extraction + 5147 asset objects) |
| `setup_server.sh` | assembles the CRUSSTY dist root (modules/lod + launcher + runtime + Paper 26.3), mirrors the CI job |
| `boot_server.sh` | boots the stack with a stdin fifo (`/tmp/server.in`), waits for the one-time pack finalize, runs gates G1–G7 |
| `run_client.sh` | Xvfb :99 + vanilla client with `--quickPlayMultiplayer`, forced `preferredGraphicsBackend:opengl`, fast llvmpipe options |
| `libclodgl.c` | **the sRGB shim** — see below |
| `press_key.py` / `click.py` | XTest key/click injection (accepts the server resource-pack prompt headlessly) |
| `run_e2e_shots.sh` | join → accept pack → lock noon → 8-position screenshot matrix (4 yaws + aerial + Veil-outside + back-inside) |
| `run_full_e2e.sh` | one-shot supervisor: boot (or reuse a live finalized server) + client phase + guaranteed cleanup |
| `run_dh_client.sh` | vanilla client **with Fabric 0.20.1 + DH 3.3.4-26.3 + fabric-api** (`mods/` in the client game dir), KnotClient launch, port via `CLOD_DH_PORT` |
| `run_dh_ref.sh` | DH reference one-shot: Fabric server **with the DH server mod** (`~/mc-test/dh-dist`, port 25566, same seed) + corridor seed flight + the same 8-shot matrix against real DH rendering |
| `run_dh_soak.sh` | DH LOD-data soak: park the DH client at the bench camera for 8 min, measure client/server DH sqlite growth |

## The DH reference setup (what we learned about Distant Horizons)

The DH A/B reference uses a **second server** — Fabric + the DH jar — because a
DH 3.3.4 client against a vanilla/Paper server never creates its `DhLevel`:
`DhClientWorld.createClientLevel` waits for the **level key**, and only the DH
*server* mod sends it over the plugin channel (`sendLevelKeys = true`). Without
it the client renders fog only (the `getOrLoadLevel → null` NPE path).

Other DH specifics the bench had to discover:
- `generationMaxChunkRadius` defaults to **0** — distant generation is OFF;
  the client only builds LODs from what the DH server generates/syncs.
- DH config presets override the file at load; values are rewritten on exit —
  tune the toml *before* launch (run_dh_ref.sh does it idempotently).
- `fabric-server.jar` (the loader bootstrap) does **not** forward stdin, but
  per-command fifo writes (`echo "..." > /tmp/fabcin | tail -f … | java`)
  reach the console; a held-open `exec 9>` fd does not deliver.
- The DH client needs `libXtst.so.6` (java.awt through the mod's icon path) —
  extracted into `~/mc-test/shim/` (already on `LD_LIBRARY_PATH`).
- DH confirms our ported values independently: `farFogStart 0.4`,
  `farFogMax 1.0`, `enableVanillaFog = false`,
  `grassSideRendering = FADE_TO_DIRT`, `lodShading AUTO`, and DH itself
  *disables vanilla chunk fade-in* exactly like our `terrain.vsh` override.
- On the 4 GB bench the DH server LOD gen runs single-threaded
  (`numberOfThreads = 1`) — a full ±2048 block reference takes multiple soak
  runs (`run_dh_soak.sh` measures the client/server sqlite growth).

## The sRGB shim (why libclodgl.c exists)

Minecraft 26.3's renderpearl defaults to Vulkan and falls back to
`PreferredGraphicsApi.opengl` — the OpenGL backend asks SDL for
`SDL_GL_FRAMEBUFFER_SRGB_CAPABLE=1`. Mesa's dril GLX (Debian 13, used by
Xvfb) exposes **no sRGB-capable visuals**, so `glXChooseFBConfig` matches
nothing and the client dies with `Couldn't find matching GLX visual`.

`libclodgl.c` builds `~/mc-test/shim/libGL.so.1`: SDL dlopens it through
`LD_LIBRARY_PATH`; every symbol forwards to the real libGL, except
`glXChooseFBConfig` / `glXChooseVisual`, which strip the
`GLX_FRAMEBUFFER_SRGB_CAPABLE` (0x20B1/0x20B2) pair — and
`glXGetProcAddress(ARB)`, which hands out the filtered versions. Software
sRGB-less configs are perfectly fine for a test bench.

## Local memory budget (4 GB container)

Paper runs with `-Xms256M -Xmx${CLOD_SERVER_XMX:-700M} ...` (via
`DIST_JAVA_OPTS`); the client runs `-Xmx1200M` at 640×360 with llvmpipe.
Two OOM cliffs found the hard way:
- a **fresh-world scan** at 700M dies in `ThreadLocalFixedDensityBufferCache`
  (java OOM → Moonrise chunk-system error storm → the protocol listener never
  binds). A full local scan (ring order, warm world) needs
  `CLOD_SERVER_XMX=2800M` **while no client is running**.
- the client phase on top of a fat server heap hits the **kernel** OOM killer
  instead (paper killed with 25565 never listening). For shot runs reboot with
  `CLOD_SERVER_XMX=900M` — the atlas snapshot restores and finalizes
  instantly, so the big heap is only needed for the one-time scan.

## Reproduce

```bash
python3 scripts/bench/setup_client.py          # once, ~600 MB downloads
bash   scripts/bench/setup_server.sh           # once, needs liblod.so + CRUSSTY
bash   scripts/bench/run_full_e2e.sh           # boot + client + shots
```

Screenshots land in `~/mc-test/shots/`.

## The Voxy A/B suite (v4.5.2+) — what it took to run Voxy 0.2.20 on the bench

The A/B matrix ends with a real-Voxy baseline (`20_voxy_day … 23_voxy_day_return`).
Voxy is a client-side mod, so its baseline does not need the c-LoD server at all
— that turned out to be the key to fitting it on the 4 GB box at all.

| file | role |
|------|------|
| `run_voxy_client.sh` | vanilla client + Fabric 0.20.1 + **voxy 0.2.20-beta-gl450.jar** + fabric-api + sodium (see below), Xvfb :99, llvmpipe, fresh `servers.dat` + voxy DB + Mesa shader cache every launch |
| `boot_paper_plain.sh` | **plain Paper 26.3** (no CRUSSTY agent/launcher/module) on the same world dir — frees ~300 MB RSS; stdin fifo for the flight `tp`s |
| `run_voxy_full.sh` | phase A: boot + join (server- AND client-side markers) + pixel-based world detector (the Mojang screen is saturated red at the frame center; requires ≥100 s post-join, 2 non-red samples AND a changing frame) + the 4-stop strip flight (spacing 128 ≤ vanilla-RD ring 2×64 — wider spacing leaves HOLES in the band exactly where the shots look) + an F3 diagnostic grab; leaves the client ALIVE |
| `run_voxy_shots.sh` | phase B (separate call; the client may or may not survive between calls — nondeterministic reaper): 45 s mesh settle + the 4 shots at the user camera, then cleanup |
| `run_voxy_join.sh` / `run_voxy_ab.sh` | the phase halves of `run_voxy_full.sh`, kept for reuse |
| `scripts/patch_voxy_gl450.py` (repo root scripts/) | rebuilds `voxy-0.2.20-beta-gl450.jar` from the pristine Modrinth jar |

### The voxy jar GLSL patch (llvmpipe)

Voxy 0.2.20 ships `#version 460 core` shaders; Mesa 25.0 llvmpipe accepts at
most GLSL 4.50 **even when the GL context reports 4.6** (Mesa's GLSL compiler
caps at 4.50; `MESA_GL_VERSION_OVERRIDE=4.6` only changes what the context
claims). The patch rewrites, byte-exact (constant-pool-safe):

- `#version 460` → `#version 450` in 2 classes (ShaderLoader prepends it to
  every shader; Capabilities probes with one) + 19 shader assets (the loader
  strips the asset version line, patched for external tools);
- `gl_BaseInstance` → `gl_BaseInstanceARB` in 3 vertex shaders (under 450 the
  unsuffixed builtin exists only in 460 core; llvmpipe implements
  `GL_ARB_shader_draw_parameters`, and its device feature report
  `shaderDrawParameters=true` confirms it);
- the quad-subgroup extension uncommented in `quads.frag` (core in 460, an
  extension under 450 — llvmpipe warns it is unsupported in fragment shaders
  but the compile survives);
- the geometry-SSBO floor `512*1024*1024` → `160*1024*1024` in
  RenderResourceReuse (on llvmpipe that buffer is malloc'd RAM; 512 MB + the
  uploads OOM-killed the client three times).

Everything else is 4.5-safe: no uint64 (int64 is used behind
`#extension GL_ARB_gpu_shader_int64`), the KHR subgroup extensions are already
declared in the shaders that need them.

### Memory map of the voxy client on this box (live-verified)

- client RSS peak ≈ 2.3–2.7 GB: heap ≤1 GB (Xmx1024M) + geometry SSBO 160 MB
  + GL textures/buffers + llvm arenas + lmdb dirty pages + netty;
- plain Paper at Xmx400M ≈ 0.8 GB — **the full CRUSSTY stack (+0.3–0.4 GB) +
  voxy does not fit 4 GB** (three OOM-killer kills verified in dmesg);
- `pack_push=0` (new v4.5.2 module option, `CLOD_EXTRA_OPTS=";pack_push=0"`)
  stops the server pushing the c-LoD pack: the 26.3 client AUTO-APPLIES it
  with no decline dialog, the pack's `terrain.vsh` override makes sodium
  reject the terrain shaders and voxy's render restart wedges the world load
  in a restart loop (stuck at ~35 % "Downloading terrain" forever);
- every voxy join creates a NEW world-engine id — old `.voxy/saves` data is
  orphaned; the accumulated stale pyramid fed garbage sizes into the
  AsyncNodeManager scratch-resize loop → a giant llvmpipe memset that
  safepoint-stalls the whole JVM (jcmd attach = connection reset). The client
  script wipes `.voxy/saves` every launch;
- `service_threads` 2 vs 1: irrelevant to the freezes (those were the
  corrupted DB); 2 drains the ingest queue faster.

### Where the A/B stands

The suite runs end-to-end: join → world → ±256-band ingest (75 MB DB) →
flight → shots. The one thing it has NOT produced: the visible voxy LOD band.
The band does not appear with `section_render_distance` 4 or 16 (the render
distance is `sectionRenderDistance × 512` blocks per NormalRenderPipeline, so
2048/8192 — the band is well inside it), with fog on/off paths, after 5 min
mesh settles. Prime suspect: voxy 0.2.20's hierarchical traversal is a
COMPUTE occlusion culler over a Hi-Z pyramid built from the vanilla depth
readback — on llvmpipe the readback/cull chain plausibly rejects every node
(geometry built, never drawn). Verifying that needs the voxy F3 debug line
(`VoxyDebugScreenEntry` registers a 26.3 debug-screen group; it did not
render on this build) or DEBUG-level voxy logs (the 26.3 logconfig.xml did
not pick up a `me.cortex.voxy` logger). Until then the voxy baseline shots
show vanilla + the void beyond RD — documented honestly rather than faked.

## Module options added in v4.9.0 (bench-relevant)

- `slice_depth=24` — how many blocks below the surface the slice walk reads
  (ChunkSnapshot, JNI-cheap: the whole 1024-atlas rescan with slices took
  ~8.5 min on the 2-worker box vs ~6.5 min without, ok=64262 err=0).
  0 disables the DH banded walls (single-band fallback).
- `earth_curve_ratio=0` — DH earthCurveRatio (0 = off, DH default). The
  bench verified 400 (subtle, ~1 block at the vanilla horizon) and
  5000 (DH max, dramatic: docs/e2e-2026-10-25-v49-slices/zoom/
  curvature_ratio5000_south.png). The pack SHA-1 changes with the value
  (the uniform rides the chain JSON) — determinism per config holds.
- Atlas snapshot format: CLODAT04 (blocked [cells][slice table] layout).
  A CLODAT03 snapshot is stale — the scan re-runs once after an upgrade.
- Pack size: 1.88 MB (was ~1.5 MB) — the slice strip PNG compresses well.

## Fresh-sandbox rebuild checklist (2026-10-25)

The sandbox resets `~/mc-test` between long gaps. Rebuild order that worked:
1. `scripts/rustup-init.sh -y --profile minimal` + JDK25 (`~/tools/jdk25`, symlink `~/mc-test/jdk25`).
2. `cp <paper.jar> ~/mc-test/dl/`, copy the crussty tree to `~/mc-test/crussty`, build runtime, `setup_server.sh`.
3. `setup_client.py` is resumable (66 libs + 5147 assets, ~3 min on the 2-core box).
4. Build the shim: `gcc -shared -fPIC -O2 -o ~/mc-test/shim/libGL.so.1 scripts/bench/libclodgl.c -ldl`.
5. **`pip3 install python-xlib`** — the fresh venv lacks it, and `click.py` then dies
   with ModuleNotFoundError while the harness keeps going: both "Yes" clicks miss,
   the pack is never accepted, and the client log ends with
   `Requested post effect does not exist: clod:lod_terrain` (the signature of a
   pack-less run — 2026-10-24-2045 evidence is the last good matrix, the next
   local run failed exactly this way until xlib was installed).
6. Fresh world needs `CLOD_SERVER_XMX=2800M` for the first scan (~13 min boot+scan
   on this box); afterwards reboot at the default 900M — the atlas snapshot
   restores and finalizes instantly.
7. `deep_zoom.py <shot> <out> <x0> <y0> <x1> <y1> [scale]` — NEAREST deep-zoom of
   any screenshot region (the letterboxed 1280x720 client frame uses fractions of
   the FULL frame, bars included).

## DH reference A/B on the 4 GB box — 2026-10-25 findings (fresh sandbox)

The 10-24 reference matrix (`docs/e2e-2026-10-24-v43-dh/`) remains the DH
evidence record; reproducing it on a fresh 4 GB sandbox hit five walls,
all diagnosed:

1. **`libXtst.so.6`** — the DH client touches java.awt (icon path) and dies
   with UnsatisfiedLinkError without it. Extracted into `~/mc-test/shim/`
   (see the rebuild checklist above: `apt-get download libxtst6`, unpack the
   deb, copy `libXtst.so.6*` into the shim dir).
2. **The y=240 spawn trap** — the client respawns at the previous logout
   position. The aerial shot's logout leaves the player at y=240 looking at
   bare sky; every frame until the next `tp` is pure sky. A sky-colored frame
   with a moving sun = camera in the air, not a renderer bug.
3. **Post-tp settle** — after ANY `tp`, llvmpipe needs 60-75 s before vanilla
   chunks mesh AND DH rebuilds its LOD ring. The 8 s waits in the shot loop
   are only enough when the client is already parked at the camera for
   minutes.
4. **Client-side distant generation does not run against a DH server** —
   DH 3.3.4 routes ALL gen through the server when the server mod is present:
   the client spawns no `DH-World Gen Queue Thread` at all (jcmd-verified),
   regardless of `generationMaxChunkRadius`. The data path is: server
   `DistantHorizons.sqlite` (world/dimensions/.../data/) grows via
   `DH-World Gen Queue Thread`, then `DH-FullDataPayloadSender` syncs into
   the client's `Distant_Horizons_server_data/` sqlite (visible FullData
   rows at DetailLevel 0, PosX/PosZ = 64-block section coords, e.g. camera
   (-929, 796) → section (-15, 12)).
5. **Memory** — fabric server 600M + DH client 700M + LP_NUM_THREADS=2
   survives; the old 1500M/900M pair OOM-kills the client at ~1.9 GB RSS.

Even with server gen complete and synced, the 4 GB box did not reproduce the
10-24 band in the shot window (the box that produced that evidence was less
memory-pressured). If a future attempt is needed: boot the stack, park the
client at the camera for 5+ minutes with `vanillaFadeMode = "NONE"`,
then shoot with 10-15 s waits — and verify the band in a probe frame BEFORE
burning the shot budget.

### Fresh-sandbox rebuild notes (2026-10-25 v4.10.0 cycle)

Found while rebuilding the bench from scratch a second time (the sandbox
reset reverted c-LoD to the v4.1-era clone; the toolchain and ~/mc-test were
wiped):

- **Recovery order**: `rustup-init.sh -y --profile minimal` (watch for an
  interrupted install — `rustup component add rust-std` then fails with
  "missing manifest"; fix = `rustup toolchain uninstall stable` + reinstall),
  Temurin 25 via the adoptium API → `~/tools/jdk25` (run_client.sh honours
  `CLOD_JAVA`), glslang 16.6.0 from the Khronos release (the same URL the CI
  uses) → `/tmp/bin`, `setup_client.py` (66 libs + 5147 assets, ~5 min),
  `setup_server.sh` with `CLOD_CRUSSTY=/home/z/my-project/analysis/crussty`,
  shim rebuild, `pip3 install python-xlib` (click.py dies silently without
  it — the pack is never accepted).
- **Background processes do NOT survive the tool-call boundary** in this
  sandbox (the reaper kills the process group; `setsid nohup` does not save
  them — `oom_kill 0` proves it is not the OOM killer). Everything long runs
  in the FOREGROUND within one ≤590 s call: boot+scan+finalize fits
  (~9 min at `CLOD_SERVER_XMX=1800M`; the fresh-world scan at 700 M dies).
- **26.3 console syntax changed**: `gamerule doDaylightCycle false` now
  answers `Incorrect argument ... <--[HERE]` (the rule name is rejected) —
  cosmetic for the bench (the daylight drifts slowly); `gamemode <mode>
  <player>` also fails from the console → use `defaultgamemode spectator`
  (works) + `gamemode=spectator`/`force-gamemode=true` in
  server.properties (Paper honours them at boot).
- **The client dies client-side within the first second of the world join**
  on the fresh sandbox (spawn-chunk race; no server death message — the
  server keeps it a spectator with Health 0). A click on the death screen's
  Respawn button (screen 640,352) revives it; the run_e2e_shots.sh does it
  automatically now.
- **The pack prompt clicks never register** on this Xvfb/SDL3 build (the
  XTest cursor moves, the buttons ignore the clicks). Auto-accept instead:
  patch `acceptTextures=1b` into the client's `servers.dat` (nbtlib one-
  liner, documented in the v4.10 work) — the pack applies at join with no
  dialog. Caveat: the pack reload races the world load once; the client may
  show the death screen — the Respawn click above covers it, and the second
  join reuses the cached pack (`client/downloads/`).
