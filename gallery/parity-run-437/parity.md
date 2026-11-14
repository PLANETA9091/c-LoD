# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 78.738 | 83.786 | 19.613 | 210 | FAIL |
| 01_aerial_west | 58.197 | 71.859 | 20.842 | 249 | FAIL |
| 02_aerial_south | 50.560 | 72.246 | 22.569 | 253 | FAIL |
| 03_aerial_north | 53.527 | 74.313 | 15.777 | 253 | FAIL |
| 04_aerial_east | 50.999 | 64.691 | 33.417 | 254 | FAIL |
| 05_oblique | 85.922 | 95.769 | 2.972 | 169 | FAIL |
| 06_ground_west | 25.867 | 58.884 | 28.216 | 249 | FAIL |
| 07_far_north | 85.883 | 91.802 | 7.738 | 248 | FAIL |
| 08_back_inside | 58.708 | 74.847 | 22.911 | 239 | FAIL |
| 09_after_mesh | 50.898 | 59.918 | 33.168 | 233 | FAIL |
| 10_marker_ray | 37.888 | 52.520 | 41.598 | 236 | FAIL |
| 11_topdown | 26.240 | 65.482 | 9.312 | 223 | FAIL |
| 12_far_north_re | 85.884 | 91.800 | 7.743 | 245 | FAIL |

**GATE: RED — parity below 100%**
