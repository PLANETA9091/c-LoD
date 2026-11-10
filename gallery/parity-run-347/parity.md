# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 71.130 | 74.805 | 25.042 | 217 | FAIL |
| 01_aerial_west | 39.661 | 51.538 | 39.163 | 250 | FAIL |
| 02_aerial_south | 41.068 | 57.812 | 42.364 | 253 | FAIL |
| 03_aerial_north | 47.227 | 66.124 | 24.157 | 250 | FAIL |
| 04_aerial_east | 43.882 | 52.358 | 48.446 | 254 | FAIL |
| 05_oblique | 81.672 | 87.627 | 12.708 | 200 | FAIL |
| 06_ground_west | 45.530 | 65.200 | 23.769 | 249 | FAIL |
| 07_far_north | 85.368 | 90.269 | 8.119 | 248 | FAIL |
| 08_back_inside | 60.825 | 77.577 | 20.876 | 237 | FAIL |
| 09_after_mesh | 54.964 | 65.883 | 26.552 | 233 | FAIL |
| 10_marker_ray | 44.079 | 63.049 | 33.123 | 236 | FAIL |
| 11_topdown | 32.118 | 85.509 | 4.342 | 212 | FAIL |
| 12_far_north_re | 85.392 | 90.260 | 8.109 | 245 | FAIL |

**GATE: RED — parity below 100%**
