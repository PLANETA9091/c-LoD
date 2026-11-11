# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 90.302 | 93.651 | 5.162 | 210 | FAIL |
| 01_aerial_west | 59.383 | 70.419 | 21.716 | 248 | FAIL |
| 02_aerial_south | 65.742 | 83.065 | 10.222 | 253 | FAIL |
| 03_aerial_north | 54.216 | 72.561 | 15.874 | 253 | FAIL |
| 04_aerial_east | 60.058 | 68.187 | 28.780 | 254 | FAIL |
| 05_oblique | 93.856 | 98.636 | 0.431 | 169 | FAIL |
| 06_ground_west | 32.256 | 64.064 | 26.126 | 249 | FAIL |
| 07_far_north | 85.406 | 90.254 | 8.119 | 248 | FAIL |
| 08_back_inside | 58.931 | 75.726 | 22.359 | 237 | FAIL |
| 09_after_mesh | 49.597 | 60.294 | 30.485 | 233 | FAIL |
| 10_marker_ray | 47.005 | 63.078 | 34.191 | 236 | FAIL |
| 11_topdown | 32.161 | 85.629 | 4.296 | 223 | FAIL |
| 12_far_north_re | 85.406 | 90.248 | 8.128 | 245 | FAIL |

**GATE: RED — parity below 100%**
