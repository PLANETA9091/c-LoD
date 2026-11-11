# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 90.502 | 93.882 | 4.972 | 210 | FAIL |
| 01_aerial_west | 59.538 | 70.794 | 21.392 | 251 | FAIL |
| 02_aerial_south | 65.725 | 83.178 | 9.885 | 253 | FAIL |
| 03_aerial_north | 53.838 | 72.260 | 16.603 | 253 | FAIL |
| 04_aerial_east | 60.049 | 68.080 | 29.347 | 254 | FAIL |
| 05_oblique | 93.988 | 98.958 | 0.360 | 169 | FAIL |
| 06_ground_west | 35.293 | 64.331 | 25.956 | 249 | FAIL |
| 07_far_north | 85.465 | 90.260 | 8.097 | 248 | FAIL |
| 08_back_inside | 58.540 | 74.377 | 23.037 | 237 | FAIL |
| 09_after_mesh | 49.307 | 59.935 | 28.583 | 233 | FAIL |
| 10_marker_ray | 50.755 | 63.375 | 35.431 | 238 | FAIL |
| 11_topdown | 31.151 | 82.299 | 5.085 | 223 | FAIL |
| 12_far_north_re | 85.464 | 90.260 | 8.098 | 245 | FAIL |

**GATE: RED — parity below 100%**
