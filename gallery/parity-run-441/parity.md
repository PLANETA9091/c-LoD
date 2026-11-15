# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 79.681 | 84.755 | 17.726 | 209 | FAIL |
| 01_aerial_west | 61.429 | 76.815 | 18.944 | 248 | FAIL |
| 02_aerial_south | 50.352 | 72.729 | 21.391 | 253 | FAIL |
| 03_aerial_north | 53.714 | 74.601 | 15.961 | 253 | FAIL |
| 04_aerial_east | 50.929 | 63.966 | 30.436 | 254 | FAIL |
| 05_oblique | 92.340 | 98.981 | 0.370 | 169 | FAIL |
| 06_ground_west | 33.984 | 66.434 | 27.623 | 249 | FAIL |
| 07_far_north | 85.269 | 90.236 | 8.289 | 249 | FAIL |
| 08_back_inside | 54.396 | 70.354 | 27.613 | 238 | FAIL |
| 09_after_mesh | 46.041 | 55.715 | 38.995 | 237 | FAIL |
| 10_marker_ray | 39.641 | 54.644 | 40.330 | 238 | FAIL |
| 11_topdown | 26.571 | 68.372 | 8.503 | 215 | FAIL |
| 12_far_north_re | 85.269 | 90.236 | 8.282 | 248 | FAIL |

**GATE: RED — parity below 100%**
