# PARITY RUN 430 (run 43607297032, sha 7c6f7a1) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 79.623 | 84.679 | 17.851 | 210 | FAIL |
| 01_aerial_west | 57.434 | 72.362 | 21.197 | 250 | FAIL |
| 02_aerial_south | 50.272 | 72.222 | 21.386 | 253 | FAIL |
| 03_aerial_north | 53.659 | 74.627 | 16.324 | 253 | FAIL |
| 04_aerial_east | 50.784 | 64.071 | 30.599 | 254 | FAIL |
| 05_oblique | 83.630 | 93.645 | 4.642 | 169 | FAIL |
| 06_ground_west | 26.615 | 62.389 | 26.543 | 249 | FAIL |
| 07_far_north | 85.299 | 90.234 | 8.278 | 249 | FAIL |
| 08_back_inside | 56.620 | 73.931 | 25.033 | 238 | FAIL |
| 09_after_mesh | 46.735 | 56.306 | 37.329 | 237 | FAIL |
| 10_marker_ray | 40.589 | 54.564 | 38.946 | 238 | FAIL |
| 11_topdown | 26.724 | 67.665 | 8.704 | 216 | FAIL |
| 12_far_north_re | 85.299 | 90.237 | 8.284 | 246 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-430/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-430/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-430/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-430/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-430/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-430/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-430/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-430/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-430/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-430/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-430/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-430/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-430/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-430/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
- view probe: [view_probe.md](parity-run-430/view_probe.md) — per-position q/settle state, the v6.26.6 settle-by-laps + dip-probe channel
