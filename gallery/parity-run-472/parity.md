# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 75.812 | 80.885 | 22.181 | 212 | FAIL |
| 01_aerial_west | 57.366 | 73.337 | 19.106 | 249 | FAIL |
| 02_aerial_south | 41.754 | 59.573 | 33.323 | 253 | FAIL |
| 03_aerial_north | 41.113 | 61.825 | 24.835 | 253 | FAIL |
| 04_aerial_east | 41.766 | 61.274 | 33.918 | 254 | FAIL |
| 05_oblique | 67.453 | 83.499 | 19.661 | 223 | FAIL |
| 06_ground_west | 31.509 | 47.511 | 33.574 | 250 | FAIL |
| 07_far_north | 85.270 | 90.233 | 8.292 | 249 | FAIL |
| 08_back_inside | 55.123 | 77.380 | 18.398 | 238 | FAIL |
| 09_after_mesh | 49.652 | 71.099 | 22.136 | 237 | FAIL |
| 10_marker_ray | 44.654 | 64.912 | 28.907 | 251 | FAIL |
| 11_topdown | 19.351 | 59.634 | 9.857 | 209 | FAIL |
| 12_far_north_re | 85.270 | 90.239 | 8.274 | 246 | FAIL |

**GATE: RED — parity below 100%**
