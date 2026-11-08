# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 12, ours shots: 12, paired: 12, missing (voxy shot with no ours pair): 0
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 01_aerial_west | 68.537 | 87.441 | 10.934 | 234 | FAIL |
| 02_aerial_south | 70.571 | 91.923 | 7.105 | 253 | FAIL |
| 03_aerial_north | 62.100 | 86.910 | 10.470 | 253 | FAIL |
| 04_aerial_east | 75.815 | 89.643 | 7.093 | 248 | FAIL |
| 05_oblique | 94.824 | 99.762 | 0.144 | 18 | FAIL |
| 06_ground_west | 63.496 | 89.258 | 7.921 | 254 | FAIL |
| 07_far_north | 92.012 | 92.012 | 11.778 | 248 | FAIL |
| 08_back_inside | 70.354 | 84.104 | 20.550 | 238 | FAIL |
| 09_after_mesh | 64.097 | 82.932 | 17.294 | 234 | FAIL |
| 10_marker_ray | 75.913 | 85.445 | 14.901 | 240 | FAIL |
| 11_topdown | 34.382 | 90.105 | 3.267 | 223 | FAIL |
| 12_far_north_re | 85.408 | 85.608 | 19.344 | 249 | FAIL |

**GATE: RED — parity below 100%**
