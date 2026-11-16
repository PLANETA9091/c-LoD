# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 76.666 | 82.312 | 21.120 | 204 | FAIL |
| 01_aerial_west | 61.402 | 76.046 | 18.386 | 254 | FAIL |
| 02_aerial_south | 48.214 | 70.507 | 26.945 | 253 | FAIL |
| 03_aerial_north | 52.828 | 73.241 | 18.982 | 253 | FAIL |
| 04_aerial_east | 51.672 | 66.638 | 32.156 | 254 | FAIL |
| 05_oblique | 79.665 | 88.581 | 13.364 | 221 | FAIL |
| 06_ground_west | 29.855 | 64.367 | 24.454 | 249 | FAIL |
| 07_far_north | 85.292 | 90.270 | 8.280 | 249 | FAIL |
| 08_back_inside | 51.175 | 68.834 | 32.049 | 238 | FAIL |
| 09_after_mesh | 39.531 | 54.582 | 41.702 | 237 | FAIL |
| 10_marker_ray | 56.199 | 71.476 | 25.672 | 238 | FAIL |
| 11_topdown | 28.487 | 76.859 | 6.190 | 216 | FAIL |
| 12_far_north_re | 85.292 | 90.275 | 8.278 | 248 | FAIL |

**GATE: RED — parity below 100%**
