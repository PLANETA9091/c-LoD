# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 77.091 | 82.238 | 20.658 | 204 | FAIL |
| 01_aerial_west | 57.087 | 72.373 | 21.896 | 249 | FAIL |
| 02_aerial_south | 50.722 | 73.701 | 21.334 | 253 | FAIL |
| 03_aerial_north | 56.012 | 79.029 | 14.514 | 253 | FAIL |
| 04_aerial_east | 50.921 | 66.224 | 29.236 | 254 | FAIL |
| 05_oblique | 86.437 | 93.466 | 4.982 | 172 | FAIL |
| 06_ground_west | 31.967 | 67.799 | 23.223 | 247 | FAIL |
| 07_far_north | 85.394 | 90.261 | 8.099 | 248 | FAIL |
| 08_back_inside | 51.551 | 68.049 | 31.284 | 237 | FAIL |
| 09_after_mesh | 40.607 | 52.800 | 40.595 | 233 | FAIL |
| 10_marker_ray | 37.487 | 55.108 | 42.908 | 236 | FAIL |
| 11_topdown | 28.039 | 72.806 | 7.338 | 223 | FAIL |
| 12_far_north_re | 85.394 | 90.261 | 8.097 | 245 | FAIL |

**GATE: RED — parity below 100%**
