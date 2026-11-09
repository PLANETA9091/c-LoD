# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 76.761 | 79.996 | 23.101 | 188 | FAIL |
| 01_aerial_west | 61.653 | 78.469 | 18.224 | 248 | FAIL |
| 02_aerial_south | 53.045 | 73.502 | 30.045 | 253 | FAIL |
| 03_aerial_north | 59.287 | 83.049 | 13.967 | 253 | FAIL |
| 04_aerial_east | 44.860 | 56.478 | 47.364 | 254 | FAIL |
| 05_oblique | 92.976 | 97.924 | 1.483 | 169 | FAIL |
| 06_ground_west | 60.684 | 86.756 | 11.668 | 250 | FAIL |
| 07_far_north | 86.662 | 94.059 | 3.329 | 224 | FAIL |
| 08_back_inside | 64.817 | 81.863 | 20.276 | 239 | FAIL |
| 09_after_mesh | 57.545 | 74.315 | 21.311 | 231 | FAIL |
| 10_marker_ray | 55.835 | 69.564 | 30.616 | 236 | FAIL |
| 11_topdown | 33.422 | 87.308 | 4.079 | 223 | FAIL |
| 12_far_north_re | 85.229 | 90.142 | 8.132 | 245 | FAIL |

**GATE: RED — parity below 100%**
