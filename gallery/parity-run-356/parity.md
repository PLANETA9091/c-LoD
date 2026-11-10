# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 90.336 | 93.811 | 5.093 | 210 | FAIL |
| 01_aerial_west | 59.782 | 71.045 | 21.573 | 248 | FAIL |
| 02_aerial_south | 65.296 | 82.310 | 10.451 | 253 | FAIL |
| 03_aerial_north | 55.339 | 73.171 | 16.408 | 253 | FAIL |
| 04_aerial_east | 59.339 | 68.491 | 27.473 | 254 | FAIL |
| 05_oblique | 94.182 | 98.962 | 0.357 | 169 | FAIL |
| 06_ground_west | 32.645 | 63.771 | 25.786 | 248 | FAIL |
| 07_far_north | 85.417 | 90.260 | 8.095 | 248 | FAIL |
| 08_back_inside | 60.691 | 77.268 | 20.742 | 237 | FAIL |
| 09_after_mesh | 53.017 | 63.822 | 24.509 | 228 | FAIL |
| 10_marker_ray | 48.979 | 63.378 | 34.586 | 236 | FAIL |
| 11_topdown | 31.682 | 83.494 | 4.805 | 223 | FAIL |
| 12_far_north_re | 85.416 | 90.259 | 8.110 | 245 | FAIL |

**GATE: RED — parity below 100%**
