# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 12, ours shots: 12, paired: 12, missing (voxy shot with no ours pair): 0
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 01_aerial_west | 68.590 | 87.743 | 10.788 | 234 | FAIL |
| 02_aerial_south | 70.510 | 91.777 | 7.067 | 253 | FAIL |
| 03_aerial_north | 61.891 | 86.304 | 10.611 | 253 | FAIL |
| 04_aerial_east | 75.905 | 89.270 | 7.219 | 248 | FAIL |
| 05_oblique | 94.824 | 99.762 | 0.144 | 18 | FAIL |
| 06_ground_west | 63.734 | 90.243 | 7.502 | 254 | FAIL |
| 07_far_north | 85.361 | 85.361 | 20.441 | 249 | FAIL |
| 08_back_inside | 69.849 | 81.977 | 22.129 | 238 | FAIL |
| 09_after_mesh | 64.180 | 83.301 | 17.141 | 234 | FAIL |
| 10_marker_ray | 75.818 | 85.313 | 14.951 | 240 | FAIL |
| 11_topdown | 33.916 | 88.474 | 3.853 | 223 | FAIL |
| 12_far_north_re | 85.399 | 85.586 | 19.867 | 248 | FAIL |

**GATE: RED — parity below 100%**
