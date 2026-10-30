# c-LoD — test map

Every layer of the verification pyramid, where it lives, and how to run it.
CI (`.github/workflows/ci.yml`) runs layers 1–4 on every push; the `e2e-live`
job additionally runs layer 5 on `master`. Layers 6–7 are the local
real-client harness (a genuine vanilla 26.3 client under Xvfb — CI has no GPU).

## 1. Rust unit tests — `cargo test` (48 tests)

| module | count | what they pin |
|--------|------:|---------------|
| `src/stream.rs` | 16 | 16-bit word packing (4 particles/word, X-sub = data bits, Z-sub = slot), HI/LO plane layout, delta-zone scheduling, cone admission, lap coverage |
| `src/engine.rs` | 8 | `wish_admit` ledger (dup reject, backlog cap, per-tick budget, v5.5 window parameter), frontier math |
| `src/config.rs` | 6 | `CLOD_OPTS` parsing + clamps (gen_budget 1..256, gen_wish 32..4096, particles_per_tick, disk geometry) |
| `src/packgen.rs` | 5 | the pack builds byte-deterministically, 12 entries, ZERO textures/PNGs, capture chain present |
| `src/palette.rs` | 4 | surface → RGB mapping incl. water/foliage tints |
| `src/sha1.rs` | 4 | SHA-1 correctness (RFC vectors + pack-sized inputs) |
| `src/http.rs` | 2 | embedded pack server routes/metadata |
| `src/zipw.rs` | 3 | zip writer: entry order, stored sizes |

```bash
cargo test --release
```

## 2. Static gates

```bash
cargo fmt --all -- --check          # rustfmt
cargo clippy --all-targets -- -D warnings
bash scripts/check_glsl.sh          # glslangValidator on every shader +
                                    # JSON schema check of both post-chains
```

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
