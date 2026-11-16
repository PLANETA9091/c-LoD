# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 67.778 | 75.250 | 30.542 | 212 | FAIL |
| 01_aerial_west | 49.098 | 61.012 | 33.105 | 250 | FAIL |
| 02_aerial_south | 42.143 | 60.419 | 37.614 | 253 | FAIL |
| 03_aerial_north | 49.888 | 69.999 | 21.729 | 253 | FAIL |
| 04_aerial_east | 51.590 | 67.044 | 28.080 | 254 | FAIL |
| 05_oblique | 76.848 | 90.462 | 6.511 | 222 | FAIL |
| 06_ground_west | 37.437 | 67.913 | 18.533 | 249 | FAIL |
| 07_far_north | 85.317 | 90.242 | 8.282 | 249 | FAIL |
| 08_back_inside | 44.203 | 64.883 | 34.698 | 238 | FAIL |
| 09_after_mesh | 32.371 | 46.056 | 50.499 | 231 | FAIL |
| 10_marker_ray | 40.490 | 59.146 | 35.912 | 250 | FAIL |
| 11_topdown | 23.035 | 67.222 | 8.217 | 222 | FAIL |
| 12_far_north_re | 85.317 | 90.238 | 8.285 | 246 | FAIL |

**GATE: RED — parity below 100%**
