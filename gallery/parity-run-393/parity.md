# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 92.531 | 97.470 | 1.573 | 201 | FAIL |
| 01_aerial_west | 57.852 | 72.427 | 20.603 | 251 | FAIL |
| 02_aerial_south | 66.406 | 85.339 | 9.477 | 253 | FAIL |
| 03_aerial_north | 57.191 | 77.063 | 14.211 | 253 | FAIL |
| 04_aerial_east | 68.003 | 78.512 | 17.027 | 254 | FAIL |
| 05_oblique | 78.488 | 88.516 | 11.861 | 222 | FAIL |
| 06_ground_west | 31.874 | 67.194 | 25.576 | 249 | FAIL |
| 07_far_north | 85.410 | 90.322 | 8.103 | 248 | FAIL |
| 08_back_inside | 56.360 | 73.567 | 24.628 | 237 | FAIL |
| 09_after_mesh | 47.321 | 56.882 | 36.522 | 233 | FAIL |
| 10_marker_ray | 51.973 | 63.567 | 30.262 | 236 | FAIL |
| 11_topdown | 27.887 | 71.116 | 7.867 | 217 | FAIL |
| 12_far_north_re | 85.409 | 90.316 | 8.123 | 246 | FAIL |

**GATE: RED — parity below 100%**
