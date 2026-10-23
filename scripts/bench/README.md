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
