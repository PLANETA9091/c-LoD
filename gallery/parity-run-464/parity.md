# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 78.005 | 82.771 | 20.208 | 204 | FAIL |
| 01_aerial_west | 61.354 | 75.801 | 18.486 | 249 | FAIL |
| 02_aerial_south | 48.334 | 70.485 | 27.125 | 253 | FAIL |
| 03_aerial_north | 52.286 | 72.403 | 19.908 | 253 | FAIL |
| 04_aerial_east | 53.224 | 67.796 | 30.656 | 254 | FAIL |
| 05_oblique | 91.714 | 98.415 | 1.031 | 177 | FAIL |
| 06_ground_west | 32.805 | 64.522 | 27.668 | 248 | FAIL |
| 07_far_north | 85.269 | 90.258 | 8.178 | 248 | FAIL |
| 08_back_inside | 65.157 | 80.021 | 17.565 | 238 | FAIL |
| 09_after_mesh | 60.029 | 75.053 | 19.524 | 237 | FAIL |
| 10_marker_ray | 51.127 | 68.472 | 30.773 | 238 | FAIL |
| 11_topdown | 28.782 | 74.669 | 6.756 | 216 | FAIL |
| 12_far_north_re | 85.278 | 90.245 | 8.275 | 246 | FAIL |

**GATE: RED — parity below 100%**
