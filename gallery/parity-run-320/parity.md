# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 76.286 | 79.548 | 23.511 | 188 | FAIL |
| 01_aerial_west | 63.240 | 81.299 | 17.451 | 248 | FAIL |
| 02_aerial_south | 52.355 | 71.725 | 30.067 | 253 | FAIL |
| 03_aerial_north | 58.769 | 82.722 | 14.712 | 253 | FAIL |
| 04_aerial_east | 45.587 | 57.172 | 46.658 | 254 | FAIL |
| 05_oblique | 93.790 | 98.729 | 1.117 | 161 | FAIL |
| 06_ground_west | 31.891 | 66.130 | 25.947 | 248 | FAIL |
| 07_far_north | 86.842 | 94.203 | 3.241 | 224 | FAIL |
| 08_back_inside | 63.677 | 67.670 | 49.221 | 239 | FAIL |
| 09_after_mesh | 63.863 | 82.058 | 17.180 | 238 | FAIL |
| 10_marker_ray | 74.710 | 85.010 | 14.744 | 236 | FAIL |
| 11_topdown | 33.181 | 86.563 | 4.223 | 223 | FAIL |
| 12_far_north_re | 85.460 | 90.249 | 8.125 | 245 | FAIL |

**GATE: RED — parity below 100%**
