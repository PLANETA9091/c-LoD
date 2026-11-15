# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 79.440 | 84.596 | 18.008 | 209 | FAIL |
| 01_aerial_west | 57.286 | 71.067 | 21.379 | 251 | FAIL |
| 02_aerial_south | 51.466 | 73.030 | 22.105 | 253 | FAIL |
| 03_aerial_north | 54.380 | 75.777 | 15.663 | 253 | FAIL |
| 04_aerial_east | 53.528 | 67.304 | 30.957 | 254 | FAIL |
| 05_oblique | 85.064 | 93.644 | 4.741 | 171 | FAIL |
| 06_ground_west | 30.285 | 65.644 | 26.633 | 249 | FAIL |
| 07_far_north | 85.270 | 90.235 | 8.293 | 249 | FAIL |
| 08_back_inside | 56.751 | 74.173 | 25.422 | 238 | FAIL |
| 09_after_mesh | 47.424 | 56.662 | 37.811 | 236 | FAIL |
| 10_marker_ray | 40.816 | 54.849 | 38.865 | 238 | FAIL |
| 11_topdown | 26.733 | 67.570 | 8.942 | 222 | FAIL |
| 12_far_north_re | 85.270 | 90.238 | 8.288 | 248 | FAIL |

**GATE: RED — parity below 100%**
