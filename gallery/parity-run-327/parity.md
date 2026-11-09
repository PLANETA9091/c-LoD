# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 75.216 | 78.748 | 23.459 | 210 | FAIL |
| 01_aerial_west | 59.776 | 70.928 | 21.627 | 248 | FAIL |
| 02_aerial_south | 48.558 | 66.350 | 31.378 | 253 | FAIL |
| 03_aerial_north | 54.388 | 72.695 | 17.110 | 250 | FAIL |
| 04_aerial_east | 42.536 | 50.448 | 48.230 | 254 | FAIL |
| 05_oblique | 93.911 | 98.959 | 0.360 | 169 | FAIL |
| 06_ground_west | 37.767 | 66.812 | 25.203 | 249 | FAIL |
| 07_far_north | 87.118 | 95.749 | 1.996 | 223 | FAIL |
| 08_back_inside | 57.783 | 61.673 | 49.382 | 239 | FAIL |
| 09_after_mesh | 53.096 | 63.797 | 25.314 | 238 | FAIL |
| 10_marker_ray | 47.475 | 65.269 | 30.828 | 237 | FAIL |
| 11_topdown | 27.628 | 67.320 | 9.123 | 223 | FAIL |
| 12_far_north_re | 85.750 | 91.682 | 7.761 | 245 | FAIL |

**GATE: RED — parity below 100%**
