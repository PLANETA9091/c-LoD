# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 79.504 | 84.643 | 17.773 | 209 | FAIL |
| 01_aerial_west | 62.165 | 78.993 | 18.338 | 248 | FAIL |
| 02_aerial_south | 50.266 | 72.928 | 21.822 | 253 | FAIL |
| 03_aerial_north | 54.112 | 75.611 | 15.849 | 253 | FAIL |
| 04_aerial_east | 51.124 | 65.161 | 30.859 | 254 | FAIL |
| 05_oblique | 91.821 | 98.963 | 0.378 | 169 | FAIL |
| 06_ground_west | 32.065 | 66.453 | 25.930 | 249 | FAIL |
| 07_far_north | 85.767 | 91.812 | 7.800 | 248 | FAIL |
| 08_back_inside | 57.062 | 74.295 | 25.096 | 238 | FAIL |
| 09_after_mesh | 49.907 | 59.804 | 34.543 | 237 | FAIL |
| 10_marker_ray | 41.368 | 56.407 | 38.253 | 238 | FAIL |
| 11_topdown | 28.242 | 72.725 | 7.296 | 216 | FAIL |
| 12_far_north_re | 85.723 | 91.777 | 7.941 | 246 | FAIL |

**GATE: RED — parity below 100%**
