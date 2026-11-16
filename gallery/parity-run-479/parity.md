# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 68.983 | 75.729 | 29.559 | 214 | FAIL |
| 01_aerial_west | 48.034 | 59.796 | 35.798 | 251 | FAIL |
| 02_aerial_south | 40.763 | 61.209 | 36.655 | 253 | FAIL |
| 03_aerial_north | 48.719 | 67.731 | 23.671 | 253 | FAIL |
| 04_aerial_east | 51.536 | 67.143 | 28.030 | 249 | FAIL |
| 05_oblique | 78.732 | 93.745 | 5.880 | 248 | FAIL |
| 06_ground_west | 38.998 | 65.800 | 17.833 | 250 | FAIL |
| 07_far_north | 85.278 | 90.245 | 8.278 | 249 | FAIL |
| 08_back_inside | 45.337 | 64.831 | 34.152 | 231 | FAIL |
| 09_after_mesh | 34.281 | 47.529 | 48.494 | 237 | FAIL |
| 10_marker_ray | 38.502 | 57.105 | 39.713 | 238 | FAIL |
| 11_topdown | 23.497 | 66.893 | 8.459 | 216 | FAIL |
| 12_far_north_re | 85.278 | 90.241 | 8.286 | 246 | FAIL |

**GATE: RED — parity below 100%**
