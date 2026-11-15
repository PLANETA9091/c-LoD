# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 77.138 | 82.046 | 20.597 | 206 | FAIL |
| 01_aerial_west | 57.608 | 72.822 | 21.851 | 248 | FAIL |
| 02_aerial_south | 48.483 | 70.246 | 28.369 | 253 | FAIL |
| 03_aerial_north | 55.300 | 76.208 | 14.870 | 253 | FAIL |
| 04_aerial_east | 48.664 | 62.298 | 38.241 | 254 | FAIL |
| 05_oblique | 87.630 | 95.631 | 3.034 | 169 | FAIL |
| 06_ground_west | 34.393 | 69.345 | 24.555 | 248 | FAIL |
| 07_far_north | 85.409 | 90.319 | 8.102 | 248 | FAIL |
| 08_back_inside | 53.892 | 71.273 | 27.636 | 237 | FAIL |
| 09_after_mesh | 44.786 | 56.525 | 36.009 | 233 | FAIL |
| 10_marker_ray | 64.999 | 77.055 | 16.913 | 236 | FAIL |
| 11_topdown | 28.409 | 73.444 | 6.999 | 223 | FAIL |
| 12_far_north_re | 85.280 | 90.309 | 8.256 | 246 | FAIL |

**GATE: RED — parity below 100%**
