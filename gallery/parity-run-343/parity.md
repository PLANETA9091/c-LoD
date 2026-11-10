# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 86.620 | 89.689 | 5.386 | 210 | FAIL |
| 01_aerial_west | 52.346 | 60.535 | 28.913 | 252 | FAIL |
| 02_aerial_south | 56.446 | 68.763 | 19.567 | 253 | FAIL |
| 03_aerial_north | 52.507 | 69.211 | 18.735 | 253 | FAIL |
| 04_aerial_east | 66.379 | 75.152 | 15.216 | 254 | FAIL |
| 05_oblique | 88.497 | 93.933 | 4.802 | 220 | FAIL |
| 06_ground_west | 34.539 | 60.527 | 20.505 | 249 | FAIL |
| 07_far_north | 85.533 | 91.581 | 8.056 | 249 | FAIL |
| 08_back_inside | 61.091 | 77.898 | 20.633 | 238 | FAIL |
| 09_after_mesh | 53.708 | 65.226 | 24.407 | 237 | FAIL |
| 10_marker_ray | 49.599 | 64.498 | 33.867 | 240 | FAIL |
| 11_topdown | 32.020 | 85.250 | 4.354 | 212 | FAIL |
| 12_far_north_re | 85.769 | 91.600 | 7.866 | 243 | FAIL |

**GATE: RED — parity below 100%**
