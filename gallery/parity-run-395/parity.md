# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 79.066 | 84.149 | 18.576 | 209 | FAIL |
| 01_aerial_west | 58.701 | 73.295 | 20.312 | 251 | FAIL |
| 02_aerial_south | 49.796 | 71.158 | 26.934 | 253 | FAIL |
| 03_aerial_north | 53.617 | 74.183 | 16.272 | 249 | FAIL |
| 04_aerial_east | 51.628 | 63.680 | 35.043 | 254 | FAIL |
| 05_oblique | 86.789 | 96.457 | 2.137 | 169 | FAIL |
| 06_ground_west | 26.665 | 60.805 | 26.647 | 248 | FAIL |
| 07_far_north | 85.418 | 90.260 | 8.108 | 248 | FAIL |
| 08_back_inside | 56.013 | 72.824 | 24.969 | 239 | FAIL |
| 09_after_mesh | 47.728 | 57.121 | 36.503 | 234 | FAIL |
| 10_marker_ray | 40.245 | 53.834 | 39.639 | 236 | FAIL |
| 11_topdown | 25.785 | 64.617 | 9.673 | 223 | FAIL |
| 12_far_north_re | 85.419 | 90.260 | 8.109 | 245 | FAIL |

**GATE: RED — parity below 100%**
