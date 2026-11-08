# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 12, ours shots: 12, paired: 12, missing (voxy shot with no ours pair): 0
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 01_aerial_west | 68.656 | 88.003 | 10.851 | 234 | FAIL |
| 02_aerial_south | 70.618 | 92.116 | 7.061 | 253 | FAIL |
| 03_aerial_north | 62.149 | 86.951 | 10.440 | 253 | FAIL |
| 04_aerial_east | 76.142 | 90.115 | 7.065 | 248 | FAIL |
| 05_oblique | 94.824 | 99.762 | 0.144 | 18 | FAIL |
| 06_ground_west | 63.778 | 90.206 | 7.715 | 254 | FAIL |
| 07_far_north | 97.873 | 97.873 | 3.074 | 245 | FAIL |
| 08_back_inside | 69.918 | 82.244 | 22.637 | 238 | FAIL |
| 09_after_mesh | 64.265 | 83.604 | 17.149 | 234 | FAIL |
| 10_marker_ray | 75.835 | 85.369 | 14.962 | 240 | FAIL |
| 11_topdown | 34.485 | 90.730 | 3.227 | 223 | FAIL |
| 12_far_north_re | 85.404 | 85.610 | 19.337 | 249 | FAIL |

**GATE: RED — parity below 100%**
