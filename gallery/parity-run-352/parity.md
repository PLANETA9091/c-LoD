# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 12, missing (voxy shot with no ours pair): 1
- ours pair sources: . = 12
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 67.854 | 71.192 | 31.472 | 216 | FAIL |
| 01_aerial_west | 44.988 | 56.619 | 35.463 | 250 | FAIL |
| 02_aerial_south | 39.689 | 55.259 | 41.646 | 253 | FAIL |
| 03_aerial_north | 36.404 | 52.181 | 41.514 | 253 | FAIL |
| 04_aerial_east | 30.706 | 39.073 | 65.490 | 254 | FAIL |
| 05_oblique | 78.973 | 83.891 | 18.899 | 201 | FAIL |
| 06_ground_west | 46.946 | 68.109 | 20.032 | 250 | FAIL |
| 07_far_north | 85.420 | 90.404 | 8.165 | 248 | FAIL |
| 08_back_inside | 60.383 | 76.033 | 20.848 | 239 | FAIL |
| 09_after_mesh | 53.056 | 63.645 | 24.826 | 229 | FAIL |
| 10_marker_ray | 52.970 | 65.110 | 33.626 | 236 | FAIL |
| 11_topdown | 0.079 | 51.840 | 8.737 | 193 | FAIL |
| 12_far_north_re | — | — | — | — | FAIL (no ours shot) |

- ours shots with no voxy pair (informational): night_12_far_north_re

**GATE: RED — parity below 100%**
