# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 91.490 | 95.196 | 2.630 | 210 | FAIL |
| 01_aerial_west | 59.633 | 70.610 | 21.674 | 248 | FAIL |
| 02_aerial_south | 64.768 | 81.677 | 11.024 | 253 | FAIL |
| 03_aerial_north | 55.164 | 72.960 | 17.323 | 253 | FAIL |
| 04_aerial_east | 68.864 | 77.927 | 16.430 | 254 | FAIL |
| 05_oblique | 93.953 | 98.961 | 0.360 | 169 | FAIL |
| 06_ground_west | 46.134 | 68.682 | 24.731 | 249 | FAIL |
| 07_far_north | 85.462 | 90.252 | 8.123 | 248 | FAIL |
| 08_back_inside | 58.662 | 74.964 | 22.862 | 237 | FAIL |
| 09_after_mesh | 52.177 | 63.110 | 29.077 | 233 | FAIL |
| 10_marker_ray | 44.197 | 61.124 | 37.071 | 236 | FAIL |
| 11_topdown | 31.278 | 81.890 | 5.120 | 223 | FAIL |
| 12_far_north_re | 85.461 | 90.248 | 8.126 | 245 | FAIL |

**GATE: RED — parity below 100%**
