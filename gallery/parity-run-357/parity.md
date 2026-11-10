# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 90.426 | 93.829 | 5.082 | 210 | FAIL |
| 01_aerial_west | 58.677 | 69.588 | 21.989 | 250 | FAIL |
| 02_aerial_south | 65.258 | 81.997 | 10.932 | 253 | FAIL |
| 03_aerial_north | 55.330 | 73.060 | 16.675 | 253 | FAIL |
| 04_aerial_east | 59.428 | 68.179 | 28.531 | 254 | FAIL |
| 05_oblique | 93.162 | 97.950 | 1.114 | 169 | FAIL |
| 06_ground_west | 32.393 | 60.943 | 28.608 | 247 | FAIL |
| 07_far_north | 85.390 | 90.249 | 8.131 | 248 | FAIL |
| 08_back_inside | 60.720 | 77.716 | 21.274 | 239 | FAIL |
| 09_after_mesh | 52.239 | 63.810 | 24.732 | 233 | FAIL |
| 10_marker_ray | 49.408 | 63.767 | 34.289 | 236 | FAIL |
| 11_topdown | 31.492 | 82.658 | 5.088 | 223 | FAIL |
| 12_far_north_re | 85.390 | 90.252 | 8.116 | 245 | FAIL |

**GATE: RED — parity below 100%**
