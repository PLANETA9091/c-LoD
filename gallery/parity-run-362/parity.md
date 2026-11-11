# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 90.433 | 93.851 | 5.079 | 210 | FAIL |
| 01_aerial_west | 58.177 | 69.286 | 22.547 | 249 | FAIL |
| 02_aerial_south | 65.515 | 82.755 | 10.274 | 253 | FAIL |
| 03_aerial_north | 55.592 | 73.221 | 16.752 | 253 | FAIL |
| 04_aerial_east | 59.121 | 68.041 | 28.967 | 254 | FAIL |
| 05_oblique | 92.347 | 97.135 | 1.758 | 169 | FAIL |
| 06_ground_west | 51.058 | 74.059 | 19.673 | 249 | FAIL |
| 07_far_north | 85.561 | 90.425 | 8.048 | 248 | FAIL |
| 08_back_inside | 60.261 | 76.644 | 21.362 | 237 | FAIL |
| 09_after_mesh | 53.042 | 65.156 | 24.781 | 233 | FAIL |
| 10_marker_ray | 51.229 | 64.341 | 34.074 | 238 | FAIL |
| 11_topdown | 32.119 | 85.468 | 4.427 | 223 | FAIL |
| 12_far_north_re | 85.561 | 90.420 | 8.058 | 245 | FAIL |

**GATE: RED — parity below 100%**
