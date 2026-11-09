# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 79.462 | 83.257 | 18.663 | 218 | FAIL |
| 01_aerial_west | 58.432 | 74.008 | 19.736 | 251 | FAIL |
| 02_aerial_south | 49.922 | 68.192 | 34.984 | 253 | FAIL |
| 03_aerial_north | 51.996 | 73.001 | 20.094 | 253 | FAIL |
| 04_aerial_east | 40.304 | 49.278 | 52.946 | 255 | FAIL |
| 05_oblique | 90.848 | 96.609 | 2.144 | 189 | FAIL |
| 06_ground_west | 37.141 | 69.877 | 19.301 | 250 | FAIL |
| 07_far_north | 85.392 | 90.260 | 8.111 | 248 | FAIL |
| 08_back_inside | 50.487 | 71.259 | 24.709 | 227 | FAIL |
| 09_after_mesh | 47.878 | 64.081 | 29.631 | 233 | FAIL |
| 10_marker_ray | 48.093 | 64.306 | 36.722 | 251 | FAIL |
| 11_topdown | 30.083 | 77.207 | 6.275 | 208 | FAIL |
| 12_far_north_re | 85.392 | 90.260 | 8.095 | 245 | FAIL |

**GATE: RED — parity below 100%**
