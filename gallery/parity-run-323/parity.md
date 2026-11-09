# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 75.753 | 79.204 | 22.550 | 210 | FAIL |
| 01_aerial_west | 59.295 | 70.572 | 21.423 | 251 | FAIL |
| 02_aerial_south | 49.292 | 66.699 | 31.655 | 253 | FAIL |
| 03_aerial_north | 55.555 | 73.812 | 17.177 | 253 | FAIL |
| 04_aerial_east | 43.953 | 53.243 | 48.428 | 254 | FAIL |
| 05_oblique | 93.095 | 98.223 | 0.862 | 163 | FAIL |
| 06_ground_west | 35.783 | 65.516 | 25.297 | 248 | FAIL |
| 07_far_north | 86.536 | 91.039 | 10.206 | 226 | FAIL |
| 08_back_inside | 61.578 | 78.150 | 21.994 | 239 | FAIL |
| 09_after_mesh | 53.437 | 64.368 | 25.800 | 233 | FAIL |
| 10_marker_ray | 71.215 | 81.339 | 16.003 | 236 | FAIL |
| 11_topdown | 27.697 | 67.542 | 9.035 | 212 | FAIL |
| 12_far_north_re | 85.392 | 90.263 | 8.097 | 245 | FAIL |

**GATE: RED — parity below 100%**
