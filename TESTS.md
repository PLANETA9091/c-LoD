# c-LoD — test map

Every layer of the verification pyramid, where it lives, and how to run it.
CI (`.github/workflows/ci.yml`) runs layers 1–4 on every push; the `e2e-live`
job additionally runs layer 5 on `master`, and since v6.4 the `visual` job
runs layer 8 there too — a real vanilla 26.3 client under Xvfb + Mesa
llvmpipe on the runner (no GPU needed, software GL is enough for 640×360).
Layers 6–7 remain the richer local real-client harness.

## 1. Rust unit tests — `cargo test` (73 tests)

| module | count | what they pin |
|--------|------:|---------------|
| `src/stream.rs` | 18 | 16-bit word packing (4 particles/word, X-sub = data bits, Z-sub = slot), HI/LO plane layout, delta-zone scheduling, cone admission, lap coverage; v6.2 rescan due/mark semantics + the dirty edit stream carrying exactly ONE reset electron at the (qx=0,qz=0) probe position |
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
