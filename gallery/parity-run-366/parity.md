# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 90.498 | 93.909 | 5.015 | 210 | FAIL |
| 01_aerial_west | 56.136 | 66.681 | 24.299 | 249 | FAIL |
| 02_aerial_south | 65.488 | 81.964 | 10.825 | 253 | FAIL |
| 03_aerial_north | 55.134 | 72.737 | 17.148 | 253 | FAIL |
| 04_aerial_east | 59.401 | 67.573 | 29.158 | 254 | FAIL |
| 05_oblique | 89.874 | 94.795 | 4.057 | 172 | FAIL |
| 06_ground_west | 49.966 | 71.474 | 19.732 | 250 | FAIL |
| 07_far_north | 85.582 | 90.424 | 8.071 | 248 | FAIL |
| 08_back_inside | 58.527 | 74.262 | 22.817 | 239 | FAIL |
| 09_after_mesh | 50.303 | 60.599 | 28.423 | 227 | FAIL |
| 10_marker_ray | 52.036 | 63.882 | 34.937 | 236 | FAIL |
| 11_topdown | 31.643 | 83.346 | 4.848 | 223 | FAIL |
| 12_far_north_re | 85.630 | 90.417 | 8.055 | 245 | FAIL |

**GATE: RED — parity below 100%**
