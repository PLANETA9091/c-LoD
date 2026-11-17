# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 65.863 | 75.263 | 27.684 | 220 | FAIL |
| 01_aerial_west | 48.163 | 58.192 | 38.095 | 249 | FAIL |
| 02_aerial_south | 42.019 | 59.585 | 35.404 | 253 | FAIL |
| 03_aerial_north | 47.485 | 63.237 | 25.433 | 253 | FAIL |
| 04_aerial_east | 49.467 | 63.728 | 26.008 | 255 | FAIL |
| 05_oblique | 69.679 | 89.968 | 9.073 | 223 | FAIL |
| 06_ground_west | 31.868 | 63.347 | 21.410 | 255 | FAIL |
| 07_far_north | 85.635 | 90.422 | 8.059 | 248 | FAIL |
| 08_back_inside | 44.203 | 64.475 | 33.131 | 237 | FAIL |
| 09_after_mesh | 34.069 | 48.551 | 48.324 | 231 | FAIL |
| 10_marker_ray | 36.360 | 53.231 | 39.597 | 245 | FAIL |
| 11_topdown | 22.671 | 65.710 | 8.898 | 217 | FAIL |
| 12_far_north_re | 85.486 | 90.390 | 8.243 | 246 | FAIL |

**GATE: RED — parity below 100%**
