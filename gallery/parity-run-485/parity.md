# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 66.833 | 76.138 | 27.584 | 241 | FAIL |
| 01_aerial_west | 55.967 | 63.954 | 28.736 | 251 | FAIL |
| 02_aerial_south | 47.218 | 65.426 | 25.884 | 253 | FAIL |
| 03_aerial_north | 47.739 | 63.923 | 24.322 | 253 | FAIL |
| 04_aerial_east | 50.146 | 65.034 | 27.136 | 255 | FAIL |
| 05_oblique | 69.954 | 89.009 | 8.005 | 251 | FAIL |
| 06_ground_west | 43.122 | 58.608 | 21.979 | 255 | FAIL |
| 07_far_north | 85.575 | 90.404 | 8.055 | 248 | FAIL |
| 08_back_inside | 66.302 | 82.350 | 16.622 | 239 | FAIL |
| 09_after_mesh | 56.860 | 67.301 | 25.423 | 231 | FAIL |
| 10_marker_ray | 42.411 | 56.231 | 30.231 | 253 | FAIL |
| 11_topdown | 25.451 | 60.659 | 11.071 | 212 | FAIL |
| 12_far_north_re | 85.426 | 90.380 | 8.235 | 246 | FAIL |

**GATE: RED — parity below 100%**
