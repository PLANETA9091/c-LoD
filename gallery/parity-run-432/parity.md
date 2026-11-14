# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 79.546 | 84.625 | 18.089 | 209 | FAIL |
| 01_aerial_west | 56.781 | 70.842 | 21.871 | 250 | FAIL |
| 02_aerial_south | 50.309 | 72.463 | 23.079 | 253 | FAIL |
| 03_aerial_north | 55.510 | 77.780 | 14.684 | 253 | FAIL |
| 04_aerial_east | 51.998 | 65.688 | 32.448 | 254 | FAIL |
| 05_oblique | 84.615 | 93.594 | 4.908 | 172 | FAIL |
| 06_ground_west | 26.182 | 61.336 | 27.187 | 249 | FAIL |
| 07_far_north | 85.278 | 90.242 | 8.275 | 249 | FAIL |
| 08_back_inside | 56.964 | 73.676 | 24.947 | 238 | FAIL |
| 09_after_mesh | 47.144 | 56.717 | 35.674 | 236 | FAIL |
| 10_marker_ray | 40.198 | 54.688 | 39.111 | 238 | FAIL |
| 11_topdown | 26.624 | 67.247 | 8.798 | 216 | FAIL |
| 12_far_north_re | 85.278 | 90.245 | 8.270 | 246 | FAIL |

**GATE: RED — parity below 100%**
