# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 76.972 | 80.247 | 22.449 | 189 | FAIL |
| 01_aerial_west | 62.108 | 78.666 | 17.670 | 248 | FAIL |
| 02_aerial_south | 52.858 | 73.423 | 29.594 | 253 | FAIL |
| 03_aerial_north | 59.255 | 83.273 | 13.950 | 253 | FAIL |
| 04_aerial_east | 42.479 | 51.521 | 47.019 | 254 | FAIL |
| 05_oblique | 93.949 | 99.135 | 0.690 | 166 | FAIL |
| 06_ground_west | 42.883 | 73.130 | 23.949 | 249 | FAIL |
| 07_far_north | 86.376 | 89.708 | 12.419 | 226 | FAIL |
| 08_back_inside | 63.409 | 79.916 | 21.514 | 239 | FAIL |
| 09_after_mesh | 57.498 | 74.073 | 21.976 | 233 | FAIL |
| 10_marker_ray | 74.747 | 85.035 | 14.723 | 236 | FAIL |
| 11_topdown | 32.894 | 85.121 | 4.600 | 223 | FAIL |
| 12_far_north_re | 85.406 | 90.252 | 8.111 | 245 | FAIL |

**GATE: RED — parity below 100%**
