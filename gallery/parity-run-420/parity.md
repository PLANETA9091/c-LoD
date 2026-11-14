# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 78.638 | 83.770 | 19.106 | 210 | FAIL |
| 01_aerial_west | 58.763 | 73.058 | 19.998 | 254 | FAIL |
| 02_aerial_south | 51.523 | 73.773 | 21.656 | 253 | FAIL |
| 03_aerial_north | 54.167 | 75.610 | 15.273 | 253 | FAIL |
| 04_aerial_east | 51.810 | 65.845 | 32.339 | 254 | FAIL |
| 05_oblique | 86.950 | 96.193 | 2.435 | 169 | FAIL |
| 06_ground_west | 42.827 | 75.559 | 19.058 | 249 | FAIL |
| 07_far_north | 85.461 | 90.245 | 8.129 | 248 | FAIL |
| 08_back_inside | 55.058 | 71.707 | 26.866 | 237 | FAIL |
| 09_after_mesh | 45.104 | 55.045 | 38.646 | 233 | FAIL |
| 10_marker_ray | 40.136 | 55.004 | 38.784 | 236 | FAIL |
| 11_topdown | 25.990 | 66.832 | 8.901 | 217 | FAIL |
| 12_far_north_re | 85.461 | 90.247 | 8.121 | 245 | FAIL |

**GATE: RED — parity below 100%**
