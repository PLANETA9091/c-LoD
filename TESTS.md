# c-LoD — test map

Every layer of the verification pyramid, where it lives, and how to run it.
CI (`.github/workflows/ci.yml`) runs layers 1–4 on every push; the `e2e-live`
job additionally runs layer 5 on `master`, and since v6.4 the `visual` job
runs layer 8 there too — a real vanilla 26.3 client under Xvfb + Mesa
llvmpipe on the runner (no GPU needed, software GL is enough for 640×360).
Layers 6–7 remain the richer local real-client harness.

## 1. Rust unit tests — `cargo test` (87 tests)

| module | count | what they pin |
|--------|------:|---------------|
| `src/stream.rs` | 27 | 16-bit word packing (4 particles/word, X-sub = data bits, Z-sub = slot), HI/LO plane layout, delta-zone scheduling, cone admission, lap coverage; v6.2 rescan due/mark semantics + the dirty edit stream carrying exactly ONE reset electron at the (qx=0,qz=0) probe position; v6.4.2 center-anchored disk (config center -> Streamer origin) streams from an off-center vantage; v6.6 ROUND-ROBIN cone — the frustum budget rotates across ALL views (prefix of each tick's batch belongs to the served view's band+frustum, opposite frustums make the assertion unambiguous), and the per-view cone caches are matched by eye/look geometry — player-order flips reuse them and a 20-view churn burst degrades to transparent rebuilds, never to starved players; v6.6.1 EDIT RECENCY — 150 STALE decodable dirty cells cannot outvote a fresh edit in the hold/release decision (`stale_backlog_yields_the_edit_window`), while a fresh 150-cell wave still streams straight through (`fresh_wave_still_streams_through`); v6.7.1 MULTI-BEAT — the edit episode produces EXACTLY edit_beats+1 releases with a reset electron each (3-beat timeline pinned for the default, `edit_beats_zero_disables_rebeats` restores the v6.6.1 single shot, `edit_beats_scale_the_episode_exactly` pins the K=3 arithmetic and the stop) |
| `src/engine.rs` | 9 | `wish_admit` ledger (dup reject, backlog cap, per-tick budget, v5.5 window parameter), frontier math; v6.6 wire-budget window math (`bytes_per_sec`: honest 0 on empty windows, exact averages at both tick cadences) |
| `src/seedgen.rs` | 12 | SeedQueue dedup/order/close + v5.9 `pop_batch` drain; bit-contract vs `ClodSeedgen.pack()` (height/liquid/ordinals/valid bit, bit budget); v5.9 `chunkBatch` payload parsing (header, statuses, zero-filled failed segments, corruption rejects) |
| `src/config.rs` | 14 | `CLOD_OPTS` parsing + clamps (gen_budget 1..256, gen_wish 32..4096, particles_per_tick, disk geometry, native_threads/seedgen_batch, native_dust/native_trees/native_edits, rescan_ticks 0/off + 5..2400); v6.7 debug_lens bool (production false); v6.7.1 edit_beats 0..8, default 2 (0 = v6.6.1 single shot, 1 = v6.6.2 rebeat, garbage falls back) |
| `src/packgen.rs` | 6 | the pack builds byte-deterministically, 13 entries, ZERO textures/PNGs, capture chain present; v6.2: reset pass precedes the data captures, ClodAtlas/AtlasInfo stay renamed to ClodRegion/RegionInfo; v6.7: the debug lens ships ONLY under debug_lens=true (production 13 entries unchanged SHA, debug 14 with the lens window tokens, both deterministic, SHAs diverge) |
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

## 2c. VOXY visual reference — CI job `voxy-reference` (layer 9)

The user call: the marcher's look must be judged against the REAL Voxy mod,
not against inventions (the homemade "voxy-grade" shading was reverted for
exactly that). The evidence stand (see `docs/voxy-visual-model.md` for the
extracted model with file:line citations):

- `scripts/bench/setup_client_voxy.sh` — vanilla runtime + Fabric loader
  0.20.1 + fabric-api 0.165.0 + sodium 0.9.2 (a hard voxy dependency) +
  voxy 0.2.20-beta, all pinned; deterministic `voxy-config.json`.
- `scripts/bench/run_server_voxy.sh` — PLAIN Paper 26.3 (no c-LoD module: no
  pack push, no dust transport in the reference frames), seed `clodbench2026`
  — the same world the c-LoD visual leg paints.
- `scripts/bench/run_client_voxy.sh` — Xvfb + llvmpipe + the sRGB shim +
  KnotClient; `MESA_GL_VERSION_OVERRIDE=4.6` AND
  `MESA_GLSL_VERSION_OVERRIDE=460` (llvmpipe's GLSL compiler caps at 4.50
  while voxy ships `#version 460` compute shaders — verified live).
- `scripts/bench/run_voxy_shots.sh` — join → noon lock → spectator → the
  SAME vantage matrix as the c-LoD leg (center -930,796) →
  `~/mc-test/voxy-shots/`.
- CI: job `voxy-reference` (master + workflow_dispatch), uploads the
  `voxy-reference-shots` artifact; `continue-on-error` while the recipe
  settles (voxy-on-llvmpipe is experimental), NOT part of the strict gate.

Extracted so far from the voxy source: per-quad real voxel light sampled
against the live vanilla lightmap, axis-aligned quad normals, full-frame
SSAO (photon-vision, radius 1.5, pow 2.5), vanilla fog pair + 10% xz fade
(the fog/fade were already byte-exact in our marcher).

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

### v6.7.2 VERDICT FEED — the summary is never auth-walled again

Three verdict cycles in a row (v6.6.1 -> v6.7.1) lost their conclusions to
GitHub API rate limiting on the shared sandbox egress IPs: the artifacts
and job-log endpoints answer `403 rate limit exceeded` for hours at a
time, exactly when the next cron wake-up needs to read them. The `visual`
job now ends with a best-effort step that commits `summary.md` and
`probe_counts.txt` to the **`ci-verdicts` branch** (`permissions:
contents: write`, run-id-named files + `latest.md` + `index.md`). The
branch is plain git: every consumer — cron wake-ups, CI itself, any
machine — reads it over `raw.githubusercontent.com` with a `git fetch`,
NO API quota involved. The step is non-fatal by construction (`exit 0`,
push failures warn only): the feed can never flip the job's real verdict.

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

### v6.5.3 — LEG 2 PASS proved the marker real; the hold→release edit window targets the particle pool

Three runs closed this iteration:

  - **#204 (41298864989): LEG 2 PASS, failures=0** — `marker STILL visible
    after relog (probe 0)`. A FRESH client (empty buffers, empty particle
    pool) re-decodes the marker footprint instantly and it survives the
    session change. Transport, encode, strobe, paint — the product chain
    is correct end to end.
  - **#205 (41300574740): the first window draft REGRESSED leg 2** (paint=0
    everywhere — the fresh client never got its initial paint). The draft
    armed the window on ANY decodable dirty stream, and the boot-era
    backlog (~13k provisional-upgrade cells) kept arming it, starving the
    cone/zone phases that are a fresh client's ONLY paint source.
  - **#206 (41302289900): the gates fixed the regression (LEG 2 PASS
    again) but leg 1 stayed red** — the window armed AFTER the edit batch
    had already gone out into the still-saturated pool. The silence came
    too late: the dots lived and died unrendered.

The measured killer (leg 1, #202-#206): the steady-state refresh stream
(cone 3 s + zone 30 s, thousands of dots/tick) saturates the vanilla
client's particle pool; new spawns are REJECTED once the cap is hit, so
the edit's reset electron + fresh dots never render on a long-lived client
(#203's permanent sky-hole = the same saturation plus one phantom-complete
zero-replace whose recovery dots were then rejected).

**The final v6.5.3 sequence for a GENUINE edit — HOLD, RELEASE, POST:**

  - **HOLD (EDIT_HOLD_TICKS = 60, 6 s):** the decision scan (idle ticks
    only) counts decodable dirty cells; a SMALL batch (≤ 100 — a wave
    breaks the scan early) for a SETTLED audience (every view older than
    FRESH_CLIENT_TICKS = 600 ticks) silences the wire FIRST — the pool
    drains while the edit batch WAITS;
  - **RELEASE:** the hold's last tick streams the batch into the drained
    pool (reset electrons ride along);
  - **POST (EDIT_WINDOW_TICKS = 200, 20 s):** silence covers the released
    dots' lifetime so nothing competes with their render.

Waves (the boot-time provisional upgrades, a forceload — the decodable
count exceeds the cap or the scan's 100 ceiling) and fresh clients stream
straight through, never hold: waves feed the initial paint, and cone/zone
are a fresh client's only paint source. `ewin` rides the stream diag line;
`stream_dirty` factors the release loop shared by the hold, the fresh and
the idle paths.

Covered by `edit_window_silences_steady_state_and_recovers` (the full
three-beat timeline: arm, silent hold, in-tick release with the reset
electron, silent post, expiry, steady state resumed; fresh audience never
arms; a 400-cell wave never arms) plus the updated hold-aware edits of
`changed_cell_streams_first`, `dirty_edit_stream_carries_reset_electron`
and `dirty_cell_out_of_zone_waits_for_a_view`. 77 Rust tests, fmt/clippy
green.

### v6.5.4 — run #207 verdict: the hold→release sequence worked end to end; the last unknown is client-side

Run 41304650106 (the full v6.5.3) is the cleanest evidence yet, from the
full server.log timeline:

  - tick 4700 (ewin 0, q 10274) → **the fill (12:13:42)** → tick 4800
    (**ewin 260 — the FULL arm, hold+post, on the very fill**) → the hold
    froze the wire (792610 → 792799 = exactly the marker's 21 cells × 9
    dots) → **the release (~tick 4850): dirty_streams 1266 → 1287** — 21
    cells with their reset electrons went out into the DRAINED pool → the
    post window ran (wire frozen through tick 5000) → a second small edit
    re-armed the window at tick 8100 on its own. The scheduler did
    everything right.
  - The client received the packets (level_particles IN, 54 bytes each).
    And the result is STILL the sky hole: the marker strip flipped from
    painted ocean (32,45,76) to vanilla sky (176,204,255), paint=3679 px
    stable, on the watched client AND on a fresh relog decode. leg2 is
    also flaky across runs (#204/#206 PASS, #205/#207 FAIL) — a harness
    timing question (the relog probe window), separate from leg 1.

So the remaining unknown is now razor-thin: the server sends the right
words (hword 0xEA00 = height 170, cword 0x0200 = red concrete — pinned by
the debug_window unit test), the dots reach the client, and every client —
including one decoding from scratch through cone/zone, with no edit
protocol involved — decodes THOSE cells to height 0 while decoding their
ocean predecessors (hword 0x7E00) and all neighbors correctly. The decode
math treats the new word differently: the next iteration must instrument
the capture itself (a CI-only debug overlay reading clod_h back onto the
screen, or a local bench reproduction with a buffer readback) and find
which step turns 0xEA00 into 0 — the strobe positions, the reset/replace
gate, or the merge. Deliberately NOT patched blind — the same ledger rule
that held for the fog.

### v6.6 — ALL-LOD fairness + the network budget

The atlas is confirmed dead (packgen's ClodAtlas guard, zero-texture zip,
pack pushed once per join) and the stream is the only data path. v6.6
makes the stream fair for every client and measurable on the wire:

  - ROUND-ROBIN CONE (stream.rs): the frustum-priority phase served
    `views[0]` only — N-1 players waited on the 60 s zone sweep for turn
    refills. The cone budget now rotates across the whole audience: one
    view owns the half budget per tick (single-player fill speed
    unchanged), the cursor hands it to the next view every tick, and
    every tracked view keeps its own near-first cone cache matched by
    eye/look drift (<= 8 blocks / <= ~5 deg) — player-list order can
    never invalidate a cache, and a 16-cache cap survives spectator
    churn with transparent rebuilds. Tests:
    `cone_budget_rotates_round_robin_across_views` (opposite frustums
    make the per-tick prefix assertion unambiguous) and
    `cone_caches_survive_order_flips_and_view_churn` (order flips reuse
    caches; 20 one-off eyes; the tracked views keep cone service).
  - NETWORK BUDGET (engine.rs + jni.rs): `WIRE_BYTES` counts estimated
    payload bytes since boot — delivered packets × the measured
    level_particles cost (54 B, run 41304650106 ledger) + per-chunk
    bundle envelopes (16 B) — in BOTH the bundle wire and the legacy
    fallback. `stream_diag` turns the 100-tick delta into an average
    `~N kB/s` right on the wire line; `bytes_per_sec` is pure and
    unit-tested (`bytes_per_sec_window_math`). 80 Rust tests, fmt/clippy
    green.

### v6.6.1 — the CI visual verdict and the recency fix

Run 41325368226 (v6.6) delivered the sharpest evidence yet, and a
verdict: 10/11 jobs green; the `visual` job FAILED leg 1 but PASSED
leg 2 — and the two halves finally separate the failure:

  - LEG 2 (fresh client, red=167 px ≥ 150): the relog decode PAINTS the
    red column (visible in `13_after_relog_probe0.png` — a fogged red
    column against sky). The streamed words are CORRECT; the v6.5.4
    "decodes to 0 on every client" pathology is GONE.
  - LEG 1 (long-lived client, red=17 px × 24 probes over 3 min): the
    live edit never recolors the marker. The before/after frames show
    only the transport's own dot strip over the ocean — the capture
    buffers keep the pre-edit words (OR-merge cannot lose bits; the
    reset-electron replace never rendered).
  - Root cause (from the diag numbers): the dirty queue still held the
    boot-era backlog (q 11942 at tick 6500, dirty stuck at 1061, edits
    +4658p/3400c/986e) — the queue-wide hold/release decision counted
    its decodable members (> 100) and classified the genuine ~25-cell
    marker edit as a WAVE. No hold, no drain, no silence: the edit dots
    went out into the running refresh stream (the #204 saturation
    mechanism) and were never rendered.

The fix — v6.6.1 EDIT RECENCY: `set_cell` stamps `dirty_since` (the
engine tick of the write); the hold/release decision counts only cells
younger than RECENT_EDIT_TICKS = 200 (20 s at 10 tps). Boot backlog is
wave material by definition (it already waited minutes) and keeps
streaming straight through; a genuine edit re-arms the window even with
a 12k backlog. Seedgen fills stamp 0 (never genuine edits); the engine
scan path stamps the live tick. Tests:
`stale_backlog_yields_the_edit_window` (150 stale decodable cells + 1
fresh edit -> the hold ARMS, the edit releases with its reset electron,
the backlog stays queued) and `fresh_wave_still_streams_through` (150
FRESH decodable cells -> no hold, immediate streaming). The harness now
dumps `stream: tick` diag lines right after the fill and after the
probe loop, so the next run's summary shows ewin/dirty/q on both ends
of the edit. 82 Rust tests, fmt/clippy green.

### v6.6.2 — the sparse-word fragility and the rebeat

Run 41328418191 (v6.6.1) moved the verdict: the recency fix WORKED (the
window armed at the fill — recomputing the 10-tps clock makes tick 6700's
ewin 0 the normal post-expiry state), but the marker STILL rendered
wrong — and this time on BOTH legs, with a shape/color split that finally
exposed the transport's single-point fragility:

  - The fresh relog client painted the marker as a TALL column in the
    WRONG color (dark navy ≈ index 0 fogged) — the HEIGHT word decoded,
    the COLOR word did not.
  - RED_CONCRETE = color index 2 = cword 0x0200 = exactly ONE active
    slot-dot of four (the height word 0xEA00 has three — loss-tolerant
    under OR-merge). Lose that one dot (pool rejection, strobe, fog) and
    the client decodes color 0 (black) -> fogged navy — on any client,
    fresh ones included (run 1 decoded red=167, run 2 missed the same
    dot: a coin flip per run).
  - A refresh lap without a complete decode OR-merges the old ocean bits
    back over the partial new word — the wrong color persists.

The engine-side mitigation — v6.6.2 REBEAT: the hold/release path
schedules ONE retry for every streamed edit cell (REBEAT_DELAY_TICKS=40
after the release); the rebeat re-dirties them (fresh stamp), cuts the
post silence, and reruns the whole sequence — the client gets a second
complete-decode chance with a fresh reset electron. `pending_rebeat`
keeps the retry a retry (the rebeat release never re-schedules). Test:
the edit-window test now pins the full two-beat timeline (second release
carries the reset, dirty_pending returns to 0, the window expires, the
rebeat does not loop). The harness copies its summary (server belief,
diag dumps, verdicts) into the artifact as `summary.md` —
GITHUB_STEP_SUMMARY is not API-downloadable. 82 Rust tests, fmt/clippy
green. Backlog: instrument the client capture (the ledger's frontier) to
make the per-dot loss observable and the replace protocol verifiable.

### v6.7.0 — the capture lens (client-side observability)

The v6.6.x visual runs could say WHAT failed (red=17) but not WHY: the
client's capture state was a black box. The lens makes the decode state
VISIBLE: a CI-only final pass (debug_lens=true) draws the persistent
capture targets onto the screen as two panels — the color lattice (idx 2
= exactly (102,0,0), water blue, idx 0 black, unmarked dark gray) and
the height bytes (gray, unmarked dark red). A screenshot of the lens IS
the client's committed belief; the lost-single-dot fragility shows as
lens_black with a live height panel. Production pack stays
byte-identical (13 entries, same SHA — test-guarded); the debug pack is
14 entries with the lens window tokens. Harness: lens_metrics() counts
four pixel signatures inside the fixed panel rects only; server belief
(/clod-cells), stream diag (ewin/dirty/q) and client belief (lens_*) ride
every probe line of both legs. 84 Rust tests at the time, fmt/clippy
green.

### v6.7.1 — MULTI-BEAT: the bounded retry sequence

The v6.6.2 rebeat was ONE retry: p and 1-(1-p)^2 measured on runs 1-2
(the same sparse dot lost twice would still kill the color). v6.7.1
generalizes the retry into a bounded EPISODE: `edit_beats` (CLOD_OPTS,
0..=8, default 2) retry beats after the genuine release — three total
complete-decode chances at the default, miss probability (1-p)^K. Every
beat re-runs hold→release with a fresh reset electron, so a
phantom-complete partial write (the reset landed, the sparse color dot
did not) is repaired by the NEXT beat's reset + complete decode. The
episode counter (`rebeat_beats_left`) is set at the genuine release
(edit_beats - 1 — its own entries are retry 1 of K) and consumed one per
continuation release, so the retries can never loop; a new genuine edit
resets the episode. The stream diag line grew the `rbeat Np/M` token
(pending entries / beats left) so ops see the retry sequence live.
Tests +3: the edit-window test pins the full THREE-beat timeline (each
release carries its reset electron, the counter spends, no fourth beat),
`edit_beats_zero_disables_rebeats` restores the v6.6.1 single-shot edit
exactly, `edit_beats_scale_the_episode_exactly` pins the K=3 arithmetic
(exactly 4 releases, then silence). 87 Rust tests, fmt/clippy green.
The lens verdict from the next visual run decides the next layer: if
lens_black persists at a correct stream across three beats, the fix
moves to the client (strobe inversion for rare codes / replace-electron
acknowledgment).

v6.7.2 VERDICT FEED (harness/CI): the visual job's final best-effort step
commits summary.md + probe_counts.txt to the ci-verdicts branch
(run-<id>.md, latest.md, index.md) — verdicts live in git, readable over
raw CDN with NO API quota (three earlier verdicts drowned in 403s). The
step is non-fatal by construction (exit 0, push failure = WARN): the feed
cannot flip the job's real verdict. Production pack untouched.

v6.7.3 RENDERER PREFLIGHT (harness, run 41396294510 verdict): the burst
gates were advisory — a client renderer dead from boot (close=2 white px,
paint=114, lens all-zero over BOTH panels) burned the 12-minute schedule
and the verdict then blamed the transport. Now: burst counts land in shell
vars; a weak CLOSE burst gets one 20 s settle + retry, and a second miss
aborts the leg as RENDERER DEAD / run INVALID with the client log tail in
the summary (the feed publishes it — `if: always()`). lens_metrics grew
the ALL-ZERO classifier (lens_zero=1: >= 90% black in EACH panel =
capture never committed — a dead pipeline, NOT the cell-LOCAL dot-loss
signature); the probe loops print the "dead pipeline, transport N/A"
verdict line once. Harness self-tested on synthetic panels (all-zero /
dot-loss / bare scene). Rust code untouched: 87 tests, fmt/clippy green.
The real transport verdict (leg 1, marker red) is still pending a run
with a LIVE renderer — the lens question stands.

v6.8.1 EVIDENCE TO PRODUCTION (harness/CI/shader): voxy-reference shots
publish to the voxy-ref branch via scripts/bench/publish_voxy_feed.sh —
raw CDN, no API quota (the artifacts endpoint kept 403-ing the sandbox
egress IPs; the publisher is self-tested against a scratch remote: prune
ordering + GITHUB_SHA-under-set-u bugs were caught BEFORE CI). The marcher
ships the cited SSAO port of voxy post/ssao.comp (BETTER path): 12 steps,
golden-ratio r1 with the constant dither 0.5, radius 1.5, falloff
1/(1+len/r), pow(clamp(1-ao/steps), 2.5), land hits only (the bit6 gate
mapping, RenderDataFactory.java:262), applied before fog (the
NormalRenderPipeline order). Every constant carries its ssao.comp line
number in the shader source — the no-invented-constants rule from the
rejected v7.0 attempt. Shader gate: all 5 post shaders + full chain check
green; numeric port sanity (flat=1.0, valley 0.476 < cliff 0.721, void
frontier 1.0) in scripts/ssao_port_sanity.py. Rust code untouched: 87
tests, fmt/clippy green.

v6.8.2 RENDERER FORENSICS (harness/CI/shader-chain): the debug lens pass
used to read minecraft:main while writing minecraft:main — a framebuffer
feedback loop (GL undefined behavior); the lens REPLACED the blit pass
(its passthrough IS a pure copy), so the debug chain is 12->11 passes,
In <- clod_swap, zero read-while-write (structurally checked: no pass
reads its own output). The renderer preflight now disambiguates: burst
weak + lens_zero=1 (capture never committed the whole session) = RENDERER
DEAD, abort with client-log forensics; burst weak + lens_zero=0 = the
v6.5.3 saturated-pool state, renderer ALIVE, the run continues (the
v6.7.3 >50 threshold was never validated against a healthy steady-state
run). Client-log forensics (pipeline/particle/atlas grep, packet spam
filtered) ride the verdict feed always. The voxy feed publishes a failure
marker (log tails) on zero-shot runs — a failed leg is no longer
indistinguishable from "hasn't run yet". Rust code untouched: 87 tests,
fmt/clippy green.

v6.8.3 EYES IN THE FEED (harness/CI): the verdicts stop being blind.
(1) The ci-verdicts branch carries four downscaled 640 px JPEGs per run
(burst / far / baseline / last probe, scripts/bench/jpg_to_feed.py) plus
the GL stack version (xvfb-run glxinfo) and the client's own LWJGL/OpenGL/
Mesa lines in the forensics grep — three RENDERER DEAD verdicts
(41449086366 / 41454021639 / 41454056043) were judged on white-px
counters alone; a picture separates a dead GL from a live scene with no
particles instantly. (2) The voxy feed publishes the failure marker with
a per-step trace on EVERY zero-shot outcome: run 41454056043 died 76 s
in — BEFORE run_voxy_shots.sh's mkdir — and the old early-exit left no
branch at all, indistinguishable from "hasn't run yet"; each voxy step
now appends start/ok lines to ~/mc-test/voxy-steps.log and the marker
carries it (publisher self-tested on a scratch remote: missing dir,
empty dir, success + prune-5 — 12/12). (3) Death-window forensics
recorded: the last live run is v6.6.2 (paint=3679), the first dead one
v6.7.2 (paint=114, close=2); the v6.6.2->v6.7.0 diff touches no
production-pack byte (packgen pins the 13-entry artifact by test) — the
prime suspect moves to the runner image (Mesa/llvmpipe bump), which the
new GL line will name. Rust code untouched: 87 tests, fmt/clippy green.

v6.8.4 FROZEN-FRAME CONTROL ARM (harness/CI): the v6.8.3 screenshots
settled the question the counters couldn't — run 41464836708's burst and
baseline JPEGs are IDENTICAL to the pixel: the pack client's frame froze
mid-boot with LIVE lens panels (lens frames, dark lattice, a red word
strip already committed to the color lattice), a slice of sky and 95%
black. "RENDERER DEAD" was actually a dead frame SWAP after an early
frame. The differential arm: visual-control CI job — the SAME client
against PLAIN Paper (boot_server.sh CLOD_CONTROL=1: no CRUSSTY launcher,
no module, no pack; eula/properties written by the job), the SAME white-
px burst counter (now the white_count() function shared by both arms),
plus a 12 s frame-motion probe. CONTROL PASS (pack-free client draws) ->
the pack chain kills the swap; CONTROL FAIL -> the environment (Xvfb/
llvmpipe/window) is the killer and the pack is exonerated. The control
publisher (publish_control_feed.sh) retries once on a rejected push —
the main visual publisher races it on the same branch (self-tested on a
scratch remote 9/9: normal, racer-on-top, prune-3). The voxy setup
traces its internal steps into the same steps.log the failure marker
carries (the 2-second fabric-layer death now names its stage). 87 tests,
fmt/clippy green.

v6.8.5 STEP TRACES CLOSE BOTH DEATHS (CI): the v6.8.4 traces earned their
keep on their first run (#225, 41468940410). (1) The voxy failure marker
named its 2-second fabric-layer killer precisely: "running fabric
installer" and nothing after — the setup script fell back to
~/tools/jdk25/bin/java because the fabric-layer step never exported
CLOD_JAVA; the path does not exist on the runner, exit 127, set -e end.
Fix: the step exports CLOD_JAVA="$JAVA_HOME/bin/java". (2) The control
job died BEFORE its own step (no summary.md -> the publisher would have
been silent) — every control step now traces into ~/mc-test/control-
steps.log and the publisher publishes a FAILURE marker (trace tail) when
the summary never appears, closing the last silent-hole variant. 87
tests, fmt/clippy green.

v6.8.6 EVERY SILENT DEATH NAMED (CI): run #226 (41472301678) proved the
traces pay for themselves AND exposed two deaths they could not name.
(1) The voxy leg died in <=1 s after "voxy leg start" — fabric layer
finally OK (v6.8.5 fix worked), but the leg's own stdout went only to
the API-gated step log: the feed saw start-then-nothing. The leg now
tees everything into voxy-leg.log, traces every stage into
voxy-steps.log, and run_server_voxy.sh's Paper download is
--fail --retry 3 with the API's own words on any refusal (the bare
`curl -s | python3` pipeline was the only untraced instant-death
candidate before voxy_server.log exists). (2) The control pulse died in
8.5 untraced minutes — its GITHUB_STEP_SUMMARY lands in the shots dir
only at the end; the leg now tees into control-leg.log and traces
sanity/boot/client/probes; the setup_client step got its start/ok trace.
Both failure markers carry the leg logs (publisher self-tested on a
scratch remote: voxy marker + leg.log in run dir; control marker +
leg.log). 87 tests, fmt/clippy green.

v6.8.7 WHITE-LIST DEFAULT, BYTE-EXACT (CI): run #227's traces named both
deaths. (1) voxy leg: the leg.log showed the real <=1 s killer — NOT the
curl pipeline, but a self-copy `cp -f "$D/paper-26.3.jar" ./paper-26.3.jar`
(the download path IS the cwd; GNU cp rc=1 under set -e) — guarded with
an `-ef` test. (2) control arm: boot ok, client booted, handshake+hello
-> instant login_disconnect (43 bytes) -> join never happened. The new
protocol probe (scripts/bench/ctl_login_probe.py) reproduced it locally
against the EXACT control dist (Paper 26.3 build 238 + the same 5-line
server.properties + Java 25): the server's own words are
"You are not whitelisted on this server!" — Paper 26.3's FIRST BOOT
appends white-list=true to a minimal server.properties (the historical
default was false). Fix validated by the same probe: with an explicit
white-list=false the login proceeds (SetCompression response). Every CI
dist now writes white-list=false + enforce-secure-profile=false; the
control boot fingerprints join-critical props into the leg log; control
failure markers carry the server's disconnect lines. 87 tests.

v6.9.0 RENDERER-DEAD MYTH BUSTED BY PIXEL FORENSICS (CI): the #228 artifact
(client.log 144 MB + burst PNGs) shows a HEALTHY client — no GL errors, no
"Failed to apply post effect" warns, tick loop alive to the last line. The
window forensics: MC runs 640x360 at (320,176) inside the 1280x720 Xvfb
(--width 640 since the v4.1 bench), so the v6.8.x debug-lens panels rendered
at screen x 632..952 and the harness lens_metrics (PX0=952, rows 392..712)
measured the LETTERBOX — lens_zero since v6.7.3 was a letterbox artifact
(the panels are genuinely empty, so the verdict was right for the wrong
reason). The burst white-count died because the color panel covers the dust
column zone (x 634..668, measured on the control arm) since v6.8.2 — the
lens era IS the renderer-dead era. Also verified in 26.3 bytecode:
GameRenderer.update prepends minecraft:end_of_frame to every requested list
(a missing optional hook, harmless warn); preparePostEffects only closes
persistent targets when chains LEAVE the list (in-game the list persists —
the cross-frame accumulation design is valid; the every-frame close happens
only in the !shouldRenderLevel branch, i.e. menus). Fixes: run_client.sh
--width 1280 --height 720 (documented geometry, burst zone uncovered, one
ruler for both arms); debug_lens.fsh stage-eye strips (rot conf / scan
payload rows / carrier mask hits) under the color panel's top frame; the
debug chain feeds Scan/Rot/Mask to the lens; lens_metrics locates panels by
their green frames (window-size-proof). Production pack bytes untouched
(the lens ships only in the debug chain). 87 tests.

v6.9.1 WINDOW-FIX FALLOUT NAMED AND GUARDED (CI #229): the 1280x720 window
moved the pack prompt's Proceed button to y 420..475; the fixed click
(561,385) landed ABOVE it, the client sat on the prompt the whole session,
and the burst preflight measured the prompt's own white text as "5405 white
px renderer alive" — a menu passed the renderer gate. Fixes: the prompt
click is DYNAMIC (locate the Proceed label's white pixels in the button
band y 410..480, x 120..700, click its center; the yellow warning only
gates when to click — verified on the #229 prompt PNG: click lands at
(406,445), dead center); a PROMPT GUARD after the close burst fails the run
immediately ("PACK NEVER ACCEPTED") instead of measuring a menu for 12
minutes; numpy joins the CI pip deps (the v6.9.0 lens_metrics locator needs
it — on #229 every lens metric printed "?" from the numpy ImportError, the
stage eyes never reached the feed). 87 tests, fmt/clippy green.

v6.9.2 STAGE EYES v2 (CI #231): the verdict finally had eyes — rot=ok,
scan=15, lens_zero=1, while the burst PNG shows the carrier column PLAINLY
VISIBLE (4268 green px) and the relative strobe arm fires on 399/400 of the
actual dot pixels measured off the artifact. The hue test and the camera
decode are innocent; the residual suspects are the projection inputs. The
four lens strips became BUCKET PRINTERS: CAM (the Globals CameraBlockPos
the capture sees, cell-space buckets over the atlas), ROT (az/el buckets),
HITS (clod_h_b.g strobe-fired cells, 32x32 grid), MASK (clod_mask marks,
dense 64x36 grid — v6.9.0's 32x18 missed 2-3px discs). The failure marker
decodes the buckets against the known vantage (cam ~8, az ~0, el ~3) and
names: Globals-camera-wrong / az-wrong / strobe-never-fires / merge-dead.
#231 also taught: the dense carrier column survives the eraser because the
self-repairing fill copies dot colors from dot neighbors (isolated-dot
assumption) — cosmetic. Harness decode verified on synthetic panels
(cam_x_b=8 cam_z_b=9 az_b=0 el_b=3 strobe_hits=100 mask_hits=40). 87 tests,
fmt/clippy green, glsl 15/15.

v6.9.3 UNIFORM-BLOCK GATE (CI #232): the #232 run died BEFORE the verdict —
the client's own words in client.log:
  ShaderCompileException: Unable to find shader defined uniform (ClodGrid)
  -> Failed to compile post processing pipeline clod:lod_terrain/10
  -> (recovery) -> "Stepan lost connection: Disconnected"
The v6.9.2 lens shader gained the ClodGrid/ClodStream blocks while the debug
chain's lens pass still supplied only ClodLens — the sampler-map gate cannot
see uniform blocks. Fixes: the lens pass supplies ClodGrid+ClodStream; a new
STATIC gate (uniform blocks map) requires every shader-defined std140 block
(minus the client-bound vanilla set) to appear in the pass uniforms and
vice versa — negative-tested (removing ClodGrid from the JSON FAILs the
gate). Also fixed the ancient latent assert_online bug: it read $tag (never
set) instead of $1 — run #232 was the FIRST time the assert fired (the
client left mid-test because of the compile-death disconnect) and set -u
killed the script on "line 111: tag: unbound variable", hiding the real
reason. Confirmed working in #232's log: the dynamic prompt click hit
(406,445) dead center and the pack was accepted. 87 tests.
