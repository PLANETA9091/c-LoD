# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 79.226 | 84.282 | 18.297 | 204 | FAIL |
| 01_aerial_west | 60.508 | 76.473 | 19.094 | 254 | FAIL |
| 02_aerial_south | 51.446 | 73.642 | 21.943 | 253 | FAIL |
| 03_aerial_north | 53.760 | 75.272 | 15.884 | 253 | FAIL |
| 04_aerial_east | 52.276 | 65.829 | 32.176 | 254 | FAIL |
| 05_oblique | 89.991 | 97.682 | 1.230 | 177 | FAIL |
| 06_ground_west | 38.054 | 71.641 | 23.687 | 249 | FAIL |
| 07_far_north | 85.266 | 90.245 | 8.250 | 248 | FAIL |
| 08_back_inside | 57.088 | 74.752 | 24.592 | 238 | FAIL |
| 09_after_mesh | 48.709 | 61.228 | 31.268 | 237 | FAIL |
| 10_marker_ray | 40.537 | 57.541 | 38.026 | 238 | FAIL |
| 11_topdown | 27.522 | 71.305 | 7.562 | 215 | FAIL |
| 12_far_north_re | 85.278 | 90.243 | 8.281 | 246 | FAIL |

**GATE: RED — parity below 100%**
