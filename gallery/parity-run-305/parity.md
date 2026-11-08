# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 12, ours shots: 12, paired: 12, missing (voxy shot with no ours pair): 0
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 01_aerial_west | 51.500 | 52.710 | 25.179 | 254 | FAIL |
| 02_aerial_south | 45.326 | 48.558 | 31.647 | 254 | FAIL |
| 03_aerial_north | 45.880 | 48.757 | 20.259 | 254 | FAIL |
| 04_aerial_east | 73.373 | 73.419 | 14.020 | 253 | FAIL |
| 05_oblique | 57.557 | 58.264 | 19.924 | 143 | FAIL |
| 06_ground_west | 43.437 | 51.829 | 23.720 | 254 | FAIL |
| 07_far_north | 86.632 | 91.582 | 8.949 | 226 | FAIL |
| 08_back_inside | 66.719 | 70.615 | 42.578 | 243 | FAIL |
| 09_after_mesh | 60.650 | 69.429 | 20.249 | 230 | FAIL |
| 10_marker_ray | 71.350 | 75.993 | 17.319 | 240 | FAIL |
| 11_topdown | 29.248 | 77.859 | 6.468 | 223 | FAIL |
| 12_far_north_re | 85.464 | 90.258 | 8.111 | 245 | FAIL |

**GATE: RED — parity below 100%**
