# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 77.331 | 82.152 | 20.434 | 210 | FAIL |
| 01_aerial_west | 56.162 | 71.133 | 22.522 | 250 | FAIL |
| 02_aerial_south | 49.145 | 70.912 | 27.117 | 253 | FAIL |
| 03_aerial_north | 55.733 | 78.663 | 14.924 | 253 | FAIL |
| 04_aerial_east | 49.307 | 63.744 | 37.935 | 254 | FAIL |
| 05_oblique | 84.852 | 92.528 | 6.585 | 172 | FAIL |
| 06_ground_west | 33.929 | 68.705 | 25.157 | 249 | FAIL |
| 07_far_north | 85.299 | 90.240 | 8.283 | 249 | FAIL |
| 08_back_inside | 52.352 | 69.877 | 28.220 | 231 | FAIL |
| 09_after_mesh | 40.595 | 54.296 | 39.367 | 237 | FAIL |
| 10_marker_ray | 39.299 | 57.406 | 41.403 | 238 | FAIL |
| 11_topdown | 28.628 | 75.166 | 6.489 | 216 | FAIL |
| 12_far_north_re | 85.299 | 90.238 | 8.288 | 246 | FAIL |

**GATE: RED — parity below 100%**
