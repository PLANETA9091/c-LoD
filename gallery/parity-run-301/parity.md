# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 12, ours shots: 12, paired: 12, missing (voxy shot with no ours pair): 0
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 01_aerial_west | 68.580 | 88.525 | 10.271 | 255 | FAIL |
| 02_aerial_south | 70.707 | 92.399 | 6.883 | 255 | FAIL |
| 03_aerial_north | 62.510 | 88.179 | 9.835 | 249 | FAIL |
| 04_aerial_east | 76.203 | 90.216 | 6.546 | 255 | FAIL |
| 05_oblique | 94.823 | 99.761 | 0.145 | 255 | FAIL |
| 06_ground_west | 63.348 | 89.933 | 7.620 | 255 | FAIL |
| 07_far_north | 97.872 | 97.872 | 3.075 | 255 | FAIL |
| 08_back_inside | 71.377 | 88.609 | 11.090 | 255 | FAIL |
| 09_after_mesh | 64.209 | 84.033 | 16.562 | 255 | FAIL |
| 10_marker_ray | 76.040 | 86.446 | 14.234 | 255 | FAIL |
| 11_topdown | 33.975 | 88.702 | 3.751 | 223 | FAIL |
| 12_far_north_re | 85.574 | 87.054 | 14.509 | 255 | FAIL |

**GATE: RED — parity below 100%**
