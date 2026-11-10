# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 77.203 | 80.639 | 18.488 | 220 | FAIL |
| 01_aerial_west | 46.922 | 55.740 | 34.485 | 253 | FAIL |
| 02_aerial_south | 41.441 | 56.213 | 37.800 | 253 | FAIL |
| 03_aerial_north | 53.783 | 70.666 | 17.603 | 249 | FAIL |
| 04_aerial_east | 48.500 | 56.002 | 37.598 | 254 | FAIL |
| 05_oblique | 86.213 | 92.190 | 6.385 | 210 | FAIL |
| 06_ground_west | 36.610 | 62.314 | 22.822 | 254 | FAIL |
| 07_far_north | 85.561 | 90.418 | 8.068 | 248 | FAIL |
| 08_back_inside | 60.912 | 77.518 | 21.161 | 237 | FAIL |
| 09_after_mesh | 53.497 | 65.135 | 24.479 | 233 | FAIL |
| 10_marker_ray | 49.094 | 63.583 | 33.547 | 240 | FAIL |
| 11_topdown | 31.765 | 83.974 | 4.725 | 212 | FAIL |
| 12_far_north_re | 85.561 | 90.423 | 8.058 | 245 | FAIL |

**GATE: RED — parity below 100%**
