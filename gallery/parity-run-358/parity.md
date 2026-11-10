# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 90.412 | 93.829 | 5.138 | 210 | FAIL |
| 01_aerial_west | 57.729 | 68.340 | 23.075 | 250 | FAIL |
| 02_aerial_south | 64.970 | 81.429 | 11.153 | 253 | FAIL |
| 03_aerial_north | 55.217 | 73.026 | 16.998 | 253 | FAIL |
| 04_aerial_east | 58.974 | 67.264 | 29.574 | 254 | FAIL |
| 05_oblique | 91.900 | 96.737 | 1.819 | 169 | FAIL |
| 06_ground_west | 50.914 | 72.242 | 18.456 | 250 | FAIL |
| 07_far_north | 85.392 | 90.261 | 8.094 | 248 | FAIL |
| 08_back_inside | 61.660 | 77.569 | 20.288 | 239 | FAIL |
| 09_after_mesh | 55.688 | 66.507 | 25.760 | 233 | FAIL |
| 10_marker_ray | 49.762 | 62.861 | 35.542 | 236 | FAIL |
| 11_topdown | 31.366 | 82.285 | 4.968 | 223 | FAIL |
| 12_far_north_re | 85.392 | 90.261 | 8.095 | 245 | FAIL |

**GATE: RED — parity below 100%**
