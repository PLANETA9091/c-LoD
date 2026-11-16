# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 76.610 | 80.776 | 21.782 | 212 | FAIL |
| 01_aerial_west | 50.290 | 69.664 | 20.143 | 251 | FAIL |
| 02_aerial_south | 43.260 | 62.289 | 30.866 | 253 | FAIL |
| 03_aerial_north | 41.632 | 62.348 | 24.141 | 249 | FAIL |
| 04_aerial_east | 44.000 | 63.897 | 30.491 | 254 | FAIL |
| 05_oblique | 62.671 | 82.481 | 19.387 | 223 | FAIL |
| 06_ground_west | 33.187 | 47.485 | 33.335 | 251 | FAIL |
| 07_far_north | 85.409 | 90.315 | 8.113 | 248 | FAIL |
| 08_back_inside | 35.674 | 61.522 | 34.238 | 237 | FAIL |
| 09_after_mesh | 29.100 | 50.172 | 44.783 | 232 | FAIL |
| 10_marker_ray | 49.655 | 68.458 | 27.608 | 251 | FAIL |
| 11_topdown | 18.721 | 60.232 | 10.232 | 216 | FAIL |
| 12_far_north_re | 85.286 | 90.302 | 8.286 | 246 | FAIL |

**GATE: RED — parity below 100%**
