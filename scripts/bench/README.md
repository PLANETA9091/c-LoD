# Real-client E2E bench (local harness) — now runs in CI

**Everything here runs on GitHub Actions** — the CI `e2e-visual` job boots the
server stack AND a real vanilla 26.3 client under Xvfb on the runner, accepts
the pack headlessly, locks noon, takes the 8-position screenshot matrix and
asserts the visual gates (`scripts/e2e/visual_gates.py`: PNG V1–V5 + client-log
C1–C4). Artifacts (shots + logs) are uploaded on every run — green or red.
The `e2e-live` job covers the server-side gates G1–G7 on all branches too.
This directory remains the local debugging twin of the CI job.

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

Paper runs with `-Xms256M -Xmx700M -XX:MaxMetaspaceSize=256M
-XX:MaxDirectMemorySize=128M` (via `DIST_JAVA_OPTS`), atlas `512`, scan
deadline 45 s; the client runs `-Xmx1200M` at 640×360 with llvmpipe
(`LP_NUM_THREADS=$(nproc)`). The 43 % atlas coverage from the short scan is
deliberate — the deterministic SHA-1 across restarts (`atlas restored`) is
what the run proves.

## Reproduce

```bash
python3 scripts/bench/setup_client.py          # once, ~600 MB downloads
bash   scripts/bench/setup_server.sh           # once, needs liblod.so + CRUSSTY
bash   scripts/bench/run_full_e2e.sh           # boot + client + shots
```

Screenshots land in `~/mc-test/shots/`; gates:
`python3 scripts/e2e/visual_gates.py ~/mc-test/shots /tmp/client.log /tmp/server.log`.

## Env overrides (what the CI job sets)

| var | default | meaning |
|-----|---------|---------|
| `CLOD_CLIENT_ROOT` | `~/mc-test/client` | vanilla client runtime root |
| `CLOD_MANIFEST` / `CLOD_CLIENT_JAR` | repo `analysis/mc263/` | version manifest / client jar |
| `CLOD_JAVA` | `~/mc-test/jdk25/bin/java` | JDK 25+ `java` binary (CI: `$JAVA_HOME/bin/java`) |
| `CLOD_DIST` | `~/mc-test/dist` | CRUSSTY dist root |
| `CLOD_CRUSSTY` | sibling `analysis/crussty` | CRUSSTY checkout |
| `CLOD_PAPER_JAR` | `~/mc-test/dl/paper-26.3.jar` | Paper jar |
| `CLOD_SHOTS` | `~/mc-test/shots` | screenshot output dir |
| `CLOD_SHIM_DIR` | `~/mc-test/shim` | sRGB shim dir (`libGL.so.1`) |
| `CLOD_OPTS` | `scan_deadline_secs=45;…;atlas_size=512` | module options |
| `CLOD_DIST_JAVA_OPTS` | `-Xms256M -Xmx700M …` | Paper JVM heap |
| `CLOD_REPO` / `CLOD_DIST` in `setup_server.sh` | derived | repo / dist roots |
