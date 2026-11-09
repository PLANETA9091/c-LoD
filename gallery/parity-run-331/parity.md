# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 75.199 | 78.624 | 23.286 | 210 | FAIL |
| 01_aerial_west | 58.933 | 69.489 | 21.466 | 251 | FAIL |
| 02_aerial_south | 48.598 | 65.895 | 32.477 | 253 | FAIL |
| 03_aerial_north | 53.899 | 71.294 | 19.172 | 253 | FAIL |
| 04_aerial_east | 43.637 | 51.772 | 48.357 | 254 | FAIL |
| 05_oblique | 93.547 | 98.554 | 0.708 | 169 | FAIL |
| 06_ground_west | 38.401 | 63.751 | 25.242 | 248 | FAIL |
| 07_far_north | 85.745 | 91.532 | 7.976 | 249 | FAIL |
| 08_back_inside | 60.633 | 74.038 | 24.318 | 239 | FAIL |
| 09_after_mesh | 53.897 | 64.445 | 25.613 | 234 | FAIL |
| 10_marker_ray | 52.359 | 65.651 | 32.241 | 236 | FAIL |
| 11_topdown | 27.529 | 66.935 | 9.169 | 223 | FAIL |
| 12_far_north_re | 85.745 | 91.529 | 7.988 | 243 | FAIL |

**GATE: RED — parity below 100%**
