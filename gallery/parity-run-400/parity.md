# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 79.482 | 84.632 | 18.136 | 210 | FAIL |
| 01_aerial_west | 57.852 | 72.230 | 20.762 | 248 | FAIL |
| 02_aerial_south | 51.046 | 73.462 | 20.841 | 253 | FAIL |
| 03_aerial_north | 53.826 | 74.505 | 15.931 | 253 | FAIL |
| 04_aerial_east | 51.904 | 66.143 | 29.581 | 254 | FAIL |
| 05_oblique | 84.085 | 93.816 | 4.356 | 169 | FAIL |
| 06_ground_west | 27.907 | 62.970 | 26.582 | 249 | FAIL |
| 07_far_north | 85.659 | 91.752 | 7.999 | 249 | FAIL |
| 08_back_inside | 55.983 | 73.376 | 25.462 | 237 | FAIL |
| 09_after_mesh | 45.865 | 55.507 | 37.365 | 233 | FAIL |
| 10_marker_ray | 40.402 | 55.107 | 38.683 | 238 | FAIL |
| 11_topdown | 26.422 | 67.368 | 8.807 | 223 | FAIL |
| 12_far_north_re | 85.659 | 91.750 | 8.005 | 246 | FAIL |

**GATE: RED — parity below 100%**
