# c-LoD visual verdicts — index

(bot layer, ci-verdict-bot; the CI verdict-feed step (v6.7.2) writes the raw per-run entries — see `run-42355686504*`; this index keeps the round structure)

## Verdicts of record

- **run 42355686504 (#253) 5b0b4f1 2026-11-07** — THE FIRST INSTRUMENTED RUN (bot-dispatched). CI entry: `run-42355686504.md` / `-probes.txt` / `-*.jpg` (committed by the feed step, 1411ab2). Bot verdict: **`run-42355686504-verdict.md`** — 21 cells measured; VERDICT [C]: the marker band (75-105 m) is client-culled (vanilla burst 0 at the marker zone on the HEALTHY leg, NEAR 524 alive); packet shape exonerated (per-dot grid 692/715/765); carrier/eraser resolved (raw 458, packed zeros = eraser-by-design, twins equal); leg-2 scene death persists (separate class). Fix menu unblocked (G3).
- **run 42358652148 (#255) 5e36f36 2026-11-07** — the v6.12 WALK run (bot-dispatched): **cut ≈ 50 m, IDENTICAL on both legs**; full [C] replication. Bot verdict: **`run-42358652148-verdict-walk.md`**.
- run 42076419845 (#250) 6b0b599 2026-11-05 — pre-chain forensics: the arm-race correction, stale-constants, packflow/bytecode notes (`run-250-*.md`, `run-42076419845*.md`).
- run 42063218460 (#248) d4ff522 2026-11-05 — pre-chain baseline audit (`run-248-250-*.md`, `run-42063218460*.md`).
- runs #246/#247 (4191…) — the scene-gate correction, the carrier false-positive overturn (v6.10.0 commit message on master).
- run-249-251-build-only.md — the build-only runs (no visual scenario).

## Instruments (apply order)

1. `instrument-proposal-leg1-dust-type-matrix.patch` (bfbed5c, +74)
2. `instrument-addendum-endrod-planes.patch` (8b16c22, +22)
3. `instrument-addendum-cached-pack-control.patch` (0184784, +52)
4. `instrument-addendum-control-type-matrix.patch` (1a0c871, +28)
5. `instrument-addendum-plane-type-cross.patch` (900d42b, +36)
6. `instrument-addendum-villager-cells.patch` (9da8895, +~44)
7. `instrument-addendum-stream-shape-cells.patch` (40a3ab2, +112)
8. `instrument-addendum-band-edge-walk.patch` (HEAD, +46 — the graded 30-80 m render-radius walk)
- **ONE-COMMAND:** `instrument-chain-full-v6.12.patch` (+431 vs 35eb53b, sha1 af25c1fb byte-verified; v6.11 (+385, d93576f0) kept for history).
- Notes: `instrument-proposal-leg1-dust-type-matrix.md`, `instrument-addendum-plane-type-cross.md`, `instrument-addendum-stream-shape-cells.md`, `instrument-addendum-band-edge-walk.md`, `instrument-addendum-cached-pack-control.md`, `instrument-addendum-control-type-matrix.md`.

## Reference & automation

- `dossier-leg2-dot-death.md` — the stage map S0-S6, suspect table A-J (A/type/eraser measured-resolved; [B] queued behind the band fix), the G3 fix menu.
- `triage.py` (v6.11.1) — the verdict automator: both log formats, VERDICT [C] + the walk's cut, 6 selftest fixtures green.
- `correction-carrier-is-villager.md` — the carrier correction (happy_villager, not dust).
- `roadmap.md` — goals G0-G4 (G0 CLOSED, G1 answered, G2 closed, G3 unblocked, G4 queued), round log.

## How to re-run (the repeatable G0 path)

```bash
git apply instrument-chain-full-v6.12.patch   # onto fix/leg2-arm-race @ 35eb53b (or the already-instrumented branch)
git push                                      # or: workflow_dispatch ci.yml on the branch
python3 triage.py <run summary.md>            # the verdict, automatic
```
