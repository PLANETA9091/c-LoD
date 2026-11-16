# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 77.581 | 82.560 | 20.878 | 204 | FAIL |
| 01_aerial_west | 61.752 | 77.418 | 18.168 | 249 | FAIL |
| 02_aerial_south | 47.823 | 70.204 | 28.030 | 253 | FAIL |
| 03_aerial_north | 51.420 | 72.197 | 19.193 | 249 | FAIL |
| 04_aerial_east | 51.466 | 67.309 | 29.632 | 254 | FAIL |
| 05_oblique | 77.304 | 85.990 | 17.148 | 222 | FAIL |
| 06_ground_west | 45.181 | 77.004 | 16.900 | 250 | FAIL |
| 07_far_north | 85.271 | 90.252 | 8.203 | 248 | FAIL |
| 08_back_inside | 50.098 | 67.831 | 31.022 | 231 | FAIL |
| 09_after_mesh | 38.777 | 54.184 | 40.119 | 236 | FAIL |
| 10_marker_ray | 59.232 | 75.438 | 19.988 | 250 | FAIL |
| 11_topdown | 28.296 | 76.887 | 6.063 | 216 | FAIL |
| 12_far_north_re | 85.278 | 90.246 | 8.278 | 246 | FAIL |

**GATE: RED — parity below 100%**
