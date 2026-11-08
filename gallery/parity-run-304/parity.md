# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 12, ours shots: 12, paired: 12, missing (voxy shot with no ours pair): 0
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 01_aerial_west | 52.257 | 52.791 | 26.234 | 255 | FAIL |
| 02_aerial_south | 46.069 | 49.135 | 30.592 | 254 | FAIL |
| 03_aerial_north | 46.289 | 48.973 | 28.850 | 254 | FAIL |
| 04_aerial_east | 73.412 | 75.576 | 12.423 | 253 | FAIL |
| 05_oblique | 57.557 | 58.264 | 19.688 | 143 | FAIL |
| 06_ground_west | 50.743 | 52.078 | 24.427 | 254 | FAIL |
| 07_far_north | 85.392 | 90.258 | 8.109 | 248 | FAIL |
| 08_back_inside | 69.738 | 83.546 | 11.500 | 238 | FAIL |
| 09_after_mesh | 59.266 | 65.177 | 19.151 | 243 | FAIL |
| 10_marker_ray | 69.759 | 71.105 | 15.788 | 254 | FAIL |
| 11_topdown | 30.497 | 76.012 | 6.770 | 223 | FAIL |
| 12_far_north_re | 85.393 | 90.259 | 8.111 | 245 | FAIL |

**GATE: RED — parity below 100%**
