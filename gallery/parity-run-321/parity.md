# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 75.329 | 78.767 | 23.087 | 210 | FAIL |
| 01_aerial_west | 59.700 | 70.835 | 21.579 | 248 | FAIL |
| 02_aerial_south | 48.934 | 66.393 | 32.298 | 253 | FAIL |
| 03_aerial_north | 55.661 | 74.020 | 16.861 | 253 | FAIL |
| 04_aerial_east | 44.225 | 53.661 | 49.212 | 254 | FAIL |
| 05_oblique | 94.182 | 98.962 | 0.357 | 169 | FAIL |
| 06_ground_west | 58.253 | 79.427 | 13.062 | 249 | FAIL |
| 07_far_north | 86.696 | 91.211 | 10.074 | 226 | FAIL |
| 08_back_inside | 64.884 | 80.314 | 19.917 | 239 | FAIL |
| 09_after_mesh | 59.629 | 70.714 | 21.728 | 233 | FAIL |
| 10_marker_ray | 70.991 | 81.369 | 15.961 | 236 | FAIL |
| 11_topdown | 27.765 | 67.748 | 8.802 | 223 | FAIL |
| 12_far_north_re | 85.561 | 90.428 | 8.040 | 245 | FAIL |

**GATE: RED — parity below 100%**
