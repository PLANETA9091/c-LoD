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
