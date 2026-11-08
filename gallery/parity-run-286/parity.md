# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 11, ours shots: 11, paired: 11, missing (voxy shot with no ours pair): 0
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 01_aerial_west | 68.996 | 89.437 | 9.965 | 233 | FAIL |
| 02_aerial_south | 70.163 | 90.827 | 7.251 | 253 | FAIL |
| 03_aerial_north | 62.147 | 86.980 | 10.398 | 253 | FAIL |
| 04_aerial_east | 75.747 | 89.149 | 7.090 | 248 | FAIL |
| 05_oblique | 94.824 | 99.762 | 0.144 | 18 | FAIL |
| 06_ground_west | 63.752 | 90.615 | 7.415 | 249 | FAIL |
| 07_far_north | 85.386 | 88.278 | 9.946 | 251 | FAIL |
| 08_back_inside | 71.582 | 89.959 | 9.817 | 238 | FAIL |
| 09_after_mesh | 64.577 | 84.924 | 16.325 | 240 | FAIL |
| 10_marker_ray | 75.903 | 85.343 | 14.895 | 240 | FAIL |
| 11_topdown | 33.700 | 87.556 | 3.880 | 223 | FAIL |

**GATE: RED — parity below 100%**
