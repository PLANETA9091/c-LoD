# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 79.557 | 84.608 | 17.958 | 204 | FAIL |
| 01_aerial_west | 61.339 | 77.765 | 18.824 | 251 | FAIL |
| 02_aerial_south | 49.905 | 71.389 | 26.679 | 253 | FAIL |
| 03_aerial_north | 54.324 | 75.729 | 15.538 | 253 | FAIL |
| 04_aerial_east | 51.820 | 64.566 | 34.865 | 254 | FAIL |
| 05_oblique | 91.710 | 98.927 | 0.391 | 169 | FAIL |
| 06_ground_west | 46.879 | 75.803 | 19.308 | 250 | FAIL |
| 07_far_north | 85.408 | 90.254 | 8.116 | 248 | FAIL |
| 08_back_inside | 58.807 | 75.280 | 22.796 | 237 | FAIL |
| 09_after_mesh | 50.903 | 63.198 | 28.707 | 233 | FAIL |
| 10_marker_ray | 41.766 | 58.454 | 37.176 | 236 | FAIL |
| 11_topdown | 27.534 | 71.700 | 7.529 | 223 | FAIL |
| 12_far_north_re | 85.407 | 90.256 | 8.117 | 245 | FAIL |

**GATE: RED — parity below 100%**
