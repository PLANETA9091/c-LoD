# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 0.000 | 0.002 | 144.696 | 250 | FAIL |
| 01_aerial_west | 0.000 | 0.339 | 95.994 | 251 | FAIL |
| 02_aerial_south | 0.000 | 0.124 | 87.707 | 254 | FAIL |
| 03_aerial_north | 0.000 | 0.124 | 91.493 | 253 | FAIL |
| 04_aerial_east | 0.000 | 0.050 | 131.594 | 254 | FAIL |
| 05_oblique | 0.000 | 0.000 | 87.995 | 254 | FAIL |
| 06_ground_west | 0.000 | 0.802 | 86.860 | 249 | FAIL |
| 07_far_north | 0.000 | 0.011 | 145.532 | 236 | FAIL |
| 08_back_inside | 0.000 | 0.240 | 94.802 | 235 | FAIL |
| 09_after_mesh | 0.000 | 0.497 | 91.445 | 235 | FAIL |
| 10_marker_ray | 0.000 | 0.041 | 122.321 | 241 | FAIL |
| 11_topdown | 0.000 | 0.972 | 33.418 | 218 | FAIL |
| 12_far_north_re | 0.000 | 0.010 | 137.313 | 235 | FAIL |

**GATE: RED — parity below 100%**
