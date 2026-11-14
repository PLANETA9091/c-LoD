# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 79.328 | 84.372 | 18.246 | 207 | FAIL |
| 01_aerial_west | 61.967 | 78.039 | 18.464 | 251 | FAIL |
| 02_aerial_south | 50.576 | 72.292 | 21.233 | 253 | FAIL |
| 03_aerial_north | 55.621 | 78.726 | 14.773 | 253 | FAIL |
| 04_aerial_east | 52.143 | 66.453 | 29.768 | 254 | FAIL |
| 05_oblique | 92.849 | 98.944 | 0.376 | 169 | FAIL |
| 06_ground_west | 38.682 | 71.445 | 24.014 | 248 | FAIL |
| 07_far_north | 85.392 | 90.250 | 8.112 | 248 | FAIL |
| 08_back_inside | 57.496 | 74.758 | 24.481 | 239 | FAIL |
| 09_after_mesh | 51.057 | 64.142 | 29.795 | 233 | FAIL |
| 10_marker_ray | 49.161 | 64.292 | 31.155 | 236 | FAIL |
| 11_topdown | 28.691 | 75.166 | 6.624 | 212 | FAIL |
| 12_far_north_re | 85.392 | 90.254 | 8.112 | 245 | FAIL |

**GATE: RED — parity below 100%**
