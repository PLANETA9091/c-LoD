# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 91.978 | 97.204 | 1.757 | 210 | FAIL |
| 01_aerial_west | 62.505 | 79.912 | 18.037 | 248 | FAIL |
| 02_aerial_south | 66.808 | 86.039 | 9.116 | 253 | FAIL |
| 03_aerial_north | 57.250 | 77.579 | 13.943 | 253 | FAIL |
| 04_aerial_east | 67.600 | 78.502 | 17.032 | 254 | FAIL |
| 05_oblique | 85.327 | 92.763 | 8.665 | 222 | FAIL |
| 06_ground_west | 32.349 | 66.516 | 25.200 | 247 | FAIL |
| 07_far_north | 85.461 | 90.246 | 8.124 | 248 | FAIL |
| 08_back_inside | 56.349 | 73.583 | 24.987 | 239 | FAIL |
| 09_after_mesh | 46.443 | 56.012 | 35.766 | 233 | FAIL |
| 10_marker_ray | 47.539 | 60.682 | 33.657 | 236 | FAIL |
| 11_topdown | 27.205 | 69.746 | 8.292 | 223 | FAIL |
| 12_far_north_re | 85.462 | 90.245 | 8.130 | 245 | FAIL |

**GATE: RED — parity below 100%**
