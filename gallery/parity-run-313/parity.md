# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 78.344 | 82.528 | 19.789 | 194 | FAIL |
| 01_aerial_west | 60.354 | 77.002 | 18.049 | 250 | FAIL |
| 02_aerial_south | 48.217 | 65.714 | 35.680 | 248 | FAIL |
| 03_aerial_north | 47.443 | 64.560 | 24.349 | 253 | FAIL |
| 04_aerial_east | 40.279 | 49.047 | 52.090 | 254 | FAIL |
| 05_oblique | 85.036 | 88.395 | 6.318 | 189 | FAIL |
| 06_ground_west | 42.623 | 76.194 | 14.980 | 250 | FAIL |
| 07_far_north | 86.947 | 94.367 | 3.107 | 224 | FAIL |
| 08_back_inside | 51.990 | 72.197 | 25.207 | 232 | FAIL |
| 09_after_mesh | 49.437 | 66.385 | 28.687 | 233 | FAIL |
| 10_marker_ray | 43.970 | 58.337 | 42.201 | 251 | FAIL |
| 11_topdown | 30.479 | 78.623 | 5.892 | 208 | FAIL |
| 12_far_north_re | 85.574 | 90.403 | 8.067 | 245 | FAIL |

**GATE: RED — parity below 100%**
