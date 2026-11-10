# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 76.301 | 79.089 | 19.779 | 210 | FAIL |
| 01_aerial_west | 48.194 | 57.590 | 32.744 | 249 | FAIL |
| 02_aerial_south | 39.843 | 54.690 | 42.100 | 253 | FAIL |
| 03_aerial_north | 50.776 | 65.248 | 20.714 | 253 | FAIL |
| 04_aerial_east | 47.722 | 54.744 | 39.733 | 254 | FAIL |
| 05_oblique | 82.436 | 87.685 | 9.732 | 250 | FAIL |
| 06_ground_west | 48.573 | 66.283 | 19.215 | 249 | FAIL |
| 07_far_north | 85.392 | 90.256 | 8.110 | 248 | FAIL |
| 08_back_inside | 64.449 | 80.178 | 17.352 | 239 | FAIL |
| 09_after_mesh | 58.464 | 68.922 | 22.052 | 231 | FAIL |
| 10_marker_ray | 53.045 | 63.784 | 35.212 | 236 | FAIL |
| 11_topdown | 31.360 | 82.892 | 4.983 | 212 | FAIL |
| 12_far_north_re | 85.393 | 90.259 | 8.112 | 245 | FAIL |

**GATE: RED — parity below 100%**
