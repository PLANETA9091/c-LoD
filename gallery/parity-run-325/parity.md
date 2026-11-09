# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 74.628 | 78.192 | 24.155 | 210 | FAIL |
| 01_aerial_west | 58.067 | 69.755 | 20.474 | 249 | FAIL |
| 02_aerial_south | 48.457 | 66.057 | 32.617 | 253 | FAIL |
| 03_aerial_north | 53.955 | 72.175 | 17.232 | 249 | FAIL |
| 04_aerial_east | 42.476 | 50.651 | 49.382 | 248 | FAIL |
| 05_oblique | 92.561 | 97.606 | 1.598 | 171 | FAIL |
| 06_ground_west | 57.445 | 78.890 | 13.224 | 249 | FAIL |
| 07_far_north | 86.768 | 94.237 | 3.244 | 224 | FAIL |
| 08_back_inside | 61.487 | 65.523 | 49.937 | 239 | FAIL |
| 09_after_mesh | 59.238 | 71.269 | 18.859 | 238 | FAIL |
| 10_marker_ray | 46.725 | 63.948 | 32.625 | 238 | FAIL |
| 11_topdown | 27.716 | 67.034 | 8.911 | 223 | FAIL |
| 12_far_north_re | 85.392 | 90.261 | 8.097 | 245 | FAIL |

**GATE: RED — parity below 100%**
