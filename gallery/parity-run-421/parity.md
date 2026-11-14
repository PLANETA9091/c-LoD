# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 79.254 | 84.369 | 18.429 | 207 | FAIL |
| 01_aerial_west | 58.656 | 72.831 | 20.816 | 251 | FAIL |
| 02_aerial_south | 50.185 | 71.299 | 21.670 | 253 | FAIL |
| 03_aerial_north | 55.078 | 76.984 | 15.544 | 253 | FAIL |
| 04_aerial_east | 52.007 | 65.687 | 30.029 | 254 | FAIL |
| 05_oblique | 86.749 | 95.991 | 3.059 | 171 | FAIL |
| 06_ground_west | 26.960 | 61.464 | 26.079 | 248 | FAIL |
| 07_far_north | 85.562 | 90.422 | 8.059 | 248 | FAIL |
| 08_back_inside | 57.322 | 74.939 | 23.990 | 239 | FAIL |
| 09_after_mesh | 50.068 | 59.797 | 34.789 | 233 | FAIL |
| 10_marker_ray | 44.993 | 57.772 | 35.533 | 236 | FAIL |
| 11_topdown | 27.984 | 71.262 | 7.957 | 222 | FAIL |
| 12_far_north_re | 85.563 | 90.424 | 8.047 | 245 | FAIL |

**GATE: RED — parity below 100%**
