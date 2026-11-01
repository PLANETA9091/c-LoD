# c-LoD — test map

Every layer of the verification pyramid, where it lives, and how to run it.
CI (`.github/workflows/ci.yml`) runs layers 1–4 on every push; the `e2e-live`
job additionally runs layer 5 on `master`, and since v6.4 the `visual` job
runs layer 8 there too — a real vanilla 26.3 client under Xvfb + Mesa
llvmpipe on the runner (no GPU needed, software GL is enough for 640×360).
Layers 6–7 remain the richer local real-client harness.

## 1. Rust unit tests — `cargo test` (74 tests)

| module | count | what they pin |
|--------|------:|---------------|
| `src/stream.rs` | 19 | 16-bit word packing (4 particles/word, X-sub = data bits, Z-sub = slot), HI/LO plane layout, delta-zone scheduling, cone admission, lap coverage; v6.2 rescan due/mark semantics + the dirty edit stream carrying exactly ONE reset electron at the (qx=0,qz=0) probe position; v6.4.2 center-anchored disk (config center -> Streamer origin) streams from an off-center vantage |
| `src/engine.rs` | 8 | `wish_admit` ledger (dup reject, backlog cap, per-tick budget, v5.5 window parameter), frontier math |
| `src/seedgen.rs` | 12 | SeedQueue dedup/order/close + v5.9 `pop_batch` drain; bit-contract vs `ClodSeedgen.pack()` (height/liquid/ordinals/valid bit, bit budget); v5.9 `chunkBatch` payload parsing (header, statuses, zero-filled failed segments, corruption rejects) |
| `src/config.rs` | 11 | `CLOD_OPTS` parsing + clamps (gen_budget 1..256, gen_wish 32..4096, particles_per_tick, disk geometry, native_threads/seedgen_batch, native_dust/native_trees/native_edits, rescan_ticks 0/off + 5..2400) |
| `src/packgen.rs` | 5 | the pack builds byte-deterministically, 13 entries, ZERO textures/PNGs, capture chain present; v6.2: reset pass precedes the data captures, ClodAtlas/AtlasInfo stay renamed to ClodRegion/RegionInfo |
| `src/palette.rs` | 5 | surface → RGB mapping incl. water/foliage tints; v6.4 concrete family carries the 16 dye MapColors (the CI visual marker RED_CONCRETE = 142,33,33 is pinned here) |
| `src/sha1.rs` | 4 | SHA-1 correctness (RFC vectors + pack-sized inputs) |
| `src/http.rs` | 2 | embedded pack server routes/metadata |
| `src/edits.rs` | 2 | v6.3 event push: ring unpack mirrors `ClodEdits.push` bit-for-bit (26-bit two's complement x/z, negatives, edges); ring drain segments (wrap, exact-capacity, masked tail) |
| `src/zipw.rs` | 3 | zip writer: entry order, stored sizes |
| `src/wire.rs` | 3 | dust wire quantization: exact + collision-free, half-block handling, cold-eviction bounds |

```bash
cargo test --release
```

## 2. Static gates

```bash
cargo fmt --all -- --check          # rustfmt
cargo clippy --all-targets -- -D warnings
bash scripts/check_glsl.sh          # glslangValidator on every shader +
                                    # JSON schema check of both post-chains
bash scripts/java/build.sh          # javac --release 17: the embedded seedgen
                                    # helper must rebuild to the committed bytes
                                    # (CI `javahelper` diffs the SHA-1)
```

## 2b. SEEDGEN helper preflight — `scripts/java/preflight/run.sh`

The v5.8–v6.1 Java helper against a mock `net.minecraft` hierarchy — no Paper
needed. 61 terrain checks per run — plus, since v6.3, a 31-check `PreflightEdits`
run (single JVM) for the event-push helper: the registration ladder (Bukkit
missing → -1, no enabled plugin → 0 pending, first enabled plugin wins), the
13-handler surface, the world identity guard, the 26-bit two's-complement
ring packing with negative coordinates, monotonic head across a full wrap,
and the multi-block payloads (explosions, pistons, structure growth, liquid
from-to). Then the original v5.8–v6.1 run: 61 checks per run × two JVMs: `chunk()` vs `chunkBatch()` bit-parity,
batch structure (statuses, zero-filled throwing segments, `n_ok`), `tabVer`
monotonicity, buffer/NoiseChunk close discipline, void semantics — the
affine-stride fast path vs a non-affine (Fisher–Yates) layout falling back,
both producing IDENTICAL cells (hash-compared across the two JVMs:
`COLUMNS fast=1604 slow=0` vs `fast=0 slow=1604`, same `CELLS_SHA`) — and,
since v6.1, a full SEEDGEN TREES stage: the helper is re-`init()`ed with a
mock feature pipeline (mock `ChunkGenerator.featuresPerStep`,
`FeatureSorter$StepFeatureData`, `WorldgenRandom` with the REAL seed recipes,
`FeaturePlacer` planting deterministic mock trees through the REAL proxy
surface) and must resolve, patch tree tops into the packed cells (log- and
leaf-topped columns), keep void cells void, stay deterministic, and produce
layout-independent `TREE_SHA` — with the terrain-only `CELLS_SHA` unchanged
from the v5.9 ground truth.

## 3. Pack integrity (in CI `glsl` job)

```bash
cargo test -p lod pack_builds -- --nocapture
```

Builds the REAL pack zip and asserts: 12 entries, no `.png`/`textures/`,
`clod:post/capture` + `clod_h_b` + `clod_c_b` in `lod_terrain.json`, no
`location` (texture) inputs — the three bans (no Display entities, no
runtime repack, one push per join) enforced at the artifact level.

## 4. Boot gates — `scripts/e2e/gates.py`

Hard assertions over a real boot log (used by CI `e2e-live` and by the local
harness): G1 module injected · G2 runtime admitted · G3 kernel ready ·
G4 JNI+NMS surface resolved · G5 streaming world resolved · G6 shader-only
pack finalized once · G7 zero pack rebuilds · G8 sky-grid stream armed ·
G9 dust particles spawned through the NMS bridge · G10 zero JNI linkage
errors.

```bash
python3 scripts/e2e/gates.py <boot-log>
```

## 5. CI `e2e-live` — real Paper 26.3 + CRUSSSTY boot ×2

Boots the actual server stack on the runner (Paper 26.3 + CRUSSSTY runtime +
the module), twice: fresh world (pack finalize + arm + HTTP pack fetch +
zip assertions) and restart (SHA-1 must reproduce byte-exactly — clients
never re-download). Since v5.6 the disk arms pre-join, so headless CI
exercises the full boot path with no player.

## 6. Real-client harness — `scripts/bench/` (local, Xvfb + llvmpipe)

The visual half: a genuine vanilla 26.3 client joins, accepts the pack,
locks noon, and the harness takes screenshot matrices and runs A/B diffs.
Entry points: `run_full_e2e.sh` (supervisor), `run_e2e_shots.sh`
(8-position matrix), `run_v532_stealth.sh` (stealth protocol:
static burst → 75 s spin → hops → top-down, counts carrier pixels),
`run_frontier_leg.sh` (fresh-world scan-rate leg), `scan_rate.py`
(frontier rate parser), `ab_diff.py` (clean A/B composite diff with a
thermal grid), `restore_env.sh` (one-command toolchain recovery).
Details and stand quirks: `scripts/bench/README.md`.

## 7. Evidence archive — `docs/e2e-*/`

Every verified milestone ships its frames + logs: noon matrix (v4.1),
stream decode (v5.0.2/5.0.4), smooth-move (v5.2.0), transparency (v5.2.2),
zero-residual stealth (v5.3.2), cone refill (v5.3.3), delta-zone (v5.4),
gen-frontier scan-rate (v5.5). Each directory has a `WORKLOG.md` with the
exact numbers measured on that run.

## 8. CI `visual` — the live-edit proof rendered by a real client (v6.4)

The GitHub Actions job `visual` (master + workflow_dispatch) assembles the
stack on the runner, rebuilds the vanilla 26.3 client runtime from the
committed version manifest (`scripts/bench/26.3.json` — libraries, natives
and ~5k assets straight from Mojang, client.jar sha1-verified), builds the
sRGB GLX shim, then runs `scripts/bench/run_ci_visual.sh`:

- **leg 1 (no relog):** the client joins, accepts the pack, noon is locked,
  the camera is `tp`-pinned 200 blocks west of the disk center — outside the
  64-block vanilla chunk radius (server view-distance 4), inside the LOD
  disk. A stable baseline pair of shots is taken (drift < 1500 px,
  retried — the scan keeps painting late cells), then a `/fill` of a
  5×5×101 RED_CONCRETE column fires at the disk center **while the client
  watches**. The probe loop diffs live shots against the baseline
  (`visual_diff.py --probe red`) for up to 120 s; the assertion is both the
  paint delta (≥ 300 px) AND ≥ 150 RED_CONCRETE-colored pixels (fog-aware
  thresholds: r ≥ 110, r−g ≥ 30). It must pass while the join counter stays
  at 1 — no relog, no reconnect, one session.
- **leg 2 (persistence):** the client is kicked, rejoins, the pack is
  re-accepted, and the same marker probe must pass again — the streamed
  disk state survived the session change.

Screenshots, diff composites and both logs upload as the `visual-evidence`
artifact on every run. The palette counterpart of the probe is pinned by the
`concrete_family_carries_the_dye` unit test.

### v6.4.9 status — the honest gate ledger (runs 41241839232..41260665463)

The job's hard-earned guardrails, each proven by a failed run:

| Guardrail | Run that forced it |
| --- | --- |
| baseline waits for `cells=X/Y` scan completion + 6 tries (v6.4.4) | 41237303358 — wire-quiet fired mid-wave |
| `defineClass` resolved on `java.lang.ClassLoader`, not `Class` (v6.4.5) | 41237303358 — EV_FAILED x600 blind |
| `ClodHost` plugin (paper-api + adventure-api + adventure-key, NOT the paperclip bootstrap) + hard `listener registered` gate (v6.4.6/6.4.6.2) | 41244761570/41247538200 |
| `assert_online` around every shot — a menu screenshot is never evidence (v6.4.6) | 41241839232 — both green legs were the autumn-PANORAMA: the client disconnected 2-19 s after the pack push and the red detector fed on the foliage |
| renderpearl rule: every declared sampler must be in the pass inputs — `ResetSampler` on the mode-2 capture pass; static gate in check_glsl.sh (v6.4.7) | 41248401337 — `ShaderCompileException: Unable to find shader defined uniform (ResetSampler)`, client died mid-reload |
| client renderer/network loggers on DEBUG via log4j2-ci.xml (v6.4.6) | every INFO-only run named nothing |
| `time set noon` (26.3 tick scale makes 6000 night; `doDaylightCycle` is not a 26.3 gamerule) (v6.4.8) | 41253590698 — starfield shots |
| console `particle` bursts close/at-marker in the artifact (v6.4.9) | 41256576023 — 446k packets accepted, 2 greenish px |

**Proven ON the CI client (run 41260665463):** the client lives 11 minutes
to OUR kick (the disconnect class is dead), pipelines compile silently
(zero ShaderCompileException), the event push registers (e2e-live is green
on both boots — `edits +0p`, EV_LIVE), the stream delivers (1.5M particles,
2.5M rescanned cells, the disk carries the marker), and the 26.3/llvmpipe
particle renderer DOES draw the villager carrier (the console burst is a
visible particle column at both 20 and 75 blocks).

**Open (the layer-8 gap):** the capture strobe still scores zero — the
day-lit carriers are fog-washed toward the sky blue (measured core color
(0.46, 0.54, 0.70), b-dominant, while the strobe requires g > b + 0.08),
which points at the rot-channel fog payload (sky-override group B) not
reaching the capture pass on llvmpipe. The next iteration must either
verify/repair the sky-override on renderpearl or make the strobe
fog-robust — deliberately NOT patched blind: the strobe is a product
contract (5-tap cross, v5.3.1), not a CI knob.

### v6.5 — the fog-invariant strobe arm (the gap closed by physics, not loosening)

The ledger's second sanctioned path, taken without touching the 5-tap cross
contract. Fog is a per-depth lerp toward ONE color, so it scales the dot and
its backdrop identically: the absolute hue of a fogged carrier dies
(measured core g−b = −0.16, the sky wins) but the (g−b) CONTRAST between the
dot and its own backdrop survives as contrast × (1−f). capture.fsh and
dust_mask.fsh now run BOTH arms:

  - absolute (v5.3.1, unchanged): the un-fogged color must beat
    g > 0.40, g > r + 0.08, g > b + 0.08 — precise when the group B fog
    payload is alive;
  - relative (v6.5, fog-invariant): the pixel's (g−b) must beat the mean
    (g−b) of the 4 diagonal texels ~3 px out by 0.06, plus g > r + 0.02 to
    exclude neutrals/shadows. Sky fails on contrast ≈ 0 (the ring IS the
    backdrop), foliage fails the same way, one neighboring dot in a diagonal
    costs only a quarter of its contrast (the mean).

Numeric check on the measured CI core (0.46, 0.54, 0.70): absolute arm FAILS
both sub-gates (the blindness), relative arm +0.190 > 0.06 PASSES. Villager
green swept under fog f = 0..0.85 stays above the margin everywhere
(0.75 → 0.11), far past the server decode band. Validated by glslang 16.6.0
(all 11 shaders + the renderpearl sampler rule). What the next visual run
must show: the strobe score leaves zero with the SAME dot geometry and the
SAME 5-tap cross — if it does not, the fog was never the killer and the
group-B repair path reopens with the un-fog now provably not the bottleneck.

### v6.5.1 — run #202 verdict: the strobe fix WORKED; it exposed the real killer

Run 41293241318 (v6.5) answered the ledger's verdict rule:

  - **The fog WAS the killer, and the dome now paints.** For the first time
    on CI the screen shows non-sky content below the horizon (~60k px at
    the vantage where vanilla terrain cannot reach — view-distance 4 = 64
    blocks, the camera sits at y=150). In run #201 the same shots were
    100% sky (red=0, green=0, sky=230400 per frame). The decode works.
  - The relative arm ALSO proved itself on the eraser side: the probe's
    own console burst of happy_villager particles dropped to 6 px — the
    color net now eats ANY green-excess dot, exactly its product job. The
    probe moved to `minecraft:end_rod` + a bright-white count so the
    renderer probe stops measuring the eraser.
  - **The legs still failed — root cause #2, named by the JVM:**
    `IncompatibleClassChangeError: Found interface org.bukkit.block.Block,
    but class was expected` in `ClodEdits.pushBlock` — 2405 failed event
    dispatches, the marker fill created ZERO edit events (`1127e` before
    and after the fill), so the marker cells were never re-streamed and
    there was nothing new to decode. The mock tree declared
    `org.bukkit.block.Block` / `BlockState` as final classes; in Bukkit
    both are interfaces, javac emitted `invokevirtual`, the JVM refused
    the dispatch. (Boot gates missed it: the boot log window ends before
    the first fluid tick.)

Fixes, each verified locally:

  - mock `Block` / `BlockState` are now shape-exact interfaces; the
    preflight doubles live in `MockBlock` / `MockBlockState`; rebuilt
    `ClodEdits.class` emits `invokeinterface` (javac 21 — the CI freshness
    toolchain; ClodSeedgen classes reproduce byte-identically);
  - the edits preflight passes: 61 terrain checks + `EDIT_PUSH checks=31
    failed=0` (registration ladder, world guard, ring packing, wrap);
  - `gates.py` G10 counts `IncompatibleClassChangeError` and
    `Could not pass event`; the visual verdict gates both counters over
    the FULL session log (the boot window ends before the first fluid
    tick, which is how #202 slipped through).

### v6.5.2 — run #203 verdict: the edit channel is ALIVE; the marker paints as a sky hole

Run 41295840160 (v6.5.1) settled the edit channel and isolated the next
layer precisely:

  - **The v6.5.1 fix works on CI.** `edits +5611p/4130c/1047e drop 0`
    (in #202: `+0p` forever) — the fill registered 1047 edit events, the
    dirty phase pushed 5611 particles, zero drops; G10's new counters
    stayed clean. The renderer probe is green again (end_rod burst
    144 px close; far-probe 0 is a sensitivity question — end_rod sparks
    are sub-pixel at 75 m — tracked separately).
  - **The marker painted as a SKY HOLE, not red.** The strip through the
    marker (x 628..653, exactly the 5-block footprint at 75 m) flipped
    from painted ocean (32,45,76) to vanilla sky (176,204,255) — stable
    across all 24 edit probes AND all 12 relog probes, while the dome
    around it repainted fully after the relog (255k px). So a FRESH
    client decode also leaves the marker footprint empty: the decode
    pipeline cannot reproduce the server's post-fill state there.
  - **The forensics moved server-side.** New `debug_cell_x/z` CLOD_OPTS
    arm a `/clod-cells` window (one RwLock, refreshed on the 100-tick
    diag beat): per-cell height/water/valid/rgb + the exact hword/cword
    the streamer would send + last-stream age + dirty flag. The visual
    script now dumps the window AFTER the fill and AFTER the probes into
    the step summary. Next-run discriminator: if the server shows
    height=170 valid hword=0xea00 while the screen stays sky, the
    failure is client-side decode of those dots; if the server shows
    ocean/stale words, the edit never reached the disk store. Either
    answer closes a whole branch of speculation.
  - Coverage: 76 Rust tests (debug_cell knob + the debug_window mapping,
    words, age=MAX for never-streamed, out-of-disk clamp), fmt/clippy
    green, the http route tested (503 until armed, verbatim body after).

### v6.5.3 — run #204 verdict: LEG 2 PASS — the marker is REAL; the killer was the particle pool

Run 41298864989 delivered the breakthrough the sky-hole forensics were
built for:

  - **LEG 2 PASS, failures=0** — `marker STILL visible after relog
    (probe 0)`. A FRESH client (empty buffers, empty particle pool)
    re-decodes the marker footprint instantly and it survives the session
    change. The transport, the encode, the strobe, the paint — the whole
    product chain is correct.
  - **Leg 1 still missed it: the long-lived client never got the edit.**
    The steady-state refresh stream (cone every 3 s + zone every 30 s,
    thousands of dots/tick) saturates the vanilla client's particle pool;
    new spawns are REJECTED once the cap is hit. The edit's reset
    electron + fresh dots arrived into a full pool and never rendered —
    the marker became undeliverable precisely on the client that had been
    watching for it. (#203's sky-hole was the same saturation plus one
    unlucky frame: a phantom-complete decode replaced the cell with zero
    and the recovery dots were then rejected — hence the permanent hole.)
  - **The fix: the EDIT WINDOW (v6.5.3).** When a decodable dirty cell
    streams, the cone/zone phases go silent for EDIT_WINDOW_TICKS (200 =
    10 s) — the client's pool drains (dots live 3-5 s) and the edit batch
    renders into free slots. While the window is armed the WHOLE wire
    budget serves dirty cells (was: half). The window re-arms only while
    decodable dirty cells actually stream, so an undecodable backlog
    (the boot-era provisional upgrades, ~13k cells) can never starve the
    steady state. `ewin` rides the stream diag line.
  - Covered by `edit_window_silences_steady_state_and_recovers`: the edit
    batch carries the reset electron (y == RESET_Y), the window arms,
    the steady state stays silent inside it, and cone streaming resumes
    after expiry. 77 Rust tests, fmt/clippy green.

What the next visual run must show: leg 1 catches the marker within its
first probes (the edit lands in one tick + one window), leg 2 keeps
passing, and the sky-hole class is closed for good.
