# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 79.125 | 84.201 | 18.339 | 209 | FAIL |
| 01_aerial_west | 57.848 | 72.437 | 20.752 | 251 | FAIL |
| 02_aerial_south | 50.286 | 72.020 | 26.362 | 253 | FAIL |
| 03_aerial_north | 53.772 | 75.079 | 15.945 | 249 | FAIL |
| 04_aerial_east | 51.549 | 64.941 | 35.357 | 254 | FAIL |
| 05_oblique | 84.134 | 94.025 | 4.494 | 172 | FAIL |
| 06_ground_west | 26.281 | 60.300 | 27.189 | 249 | FAIL |
| 07_far_north | 85.786 | 91.599 | 7.877 | 248 | FAIL |
| 08_back_inside | 56.615 | 73.863 | 25.001 | 237 | FAIL |
| 09_after_mesh | 48.434 | 58.620 | 34.724 | 233 | FAIL |
| 10_marker_ray | 44.253 | 58.266 | 35.197 | 236 | FAIL |
| 11_topdown | 27.217 | 70.208 | 8.197 | 223 | FAIL |
| 12_far_north_re | 85.786 | 91.602 | 7.860 | 243 | FAIL |

**GATE: RED — parity below 100%**
