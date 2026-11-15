# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 79.063 | 84.210 | 18.197 | 204 | FAIL |
| 01_aerial_west | 58.213 | 72.445 | 20.569 | 251 | FAIL |
| 02_aerial_south | 49.457 | 70.963 | 27.152 | 253 | FAIL |
| 03_aerial_north | 53.987 | 75.567 | 15.449 | 249 | FAIL |
| 04_aerial_east | 50.756 | 64.212 | 36.208 | 254 | FAIL |
| 05_oblique | 86.144 | 95.942 | 2.739 | 169 | FAIL |
| 06_ground_west | 25.830 | 59.561 | 26.257 | 249 | FAIL |
| 07_far_north | 85.392 | 90.254 | 8.107 | 248 | FAIL |
| 08_back_inside | 55.795 | 71.808 | 25.538 | 237 | FAIL |
| 09_after_mesh | 46.839 | 56.295 | 38.132 | 236 | FAIL |
| 10_marker_ray | 38.977 | 52.487 | 41.314 | 238 | FAIL |
| 11_topdown | 26.135 | 65.816 | 9.358 | 216 | FAIL |
| 12_far_north_re | 85.269 | 90.234 | 8.295 | 246 | FAIL |

**GATE: RED — parity below 100%**
