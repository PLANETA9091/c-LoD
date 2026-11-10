# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 87.425 | 90.373 | 5.017 | 220 | FAIL |
| 01_aerial_west | 54.233 | 62.381 | 25.965 | 251 | FAIL |
| 02_aerial_south | 62.159 | 76.193 | 10.739 | 251 | FAIL |
| 03_aerial_north | 53.072 | 67.918 | 19.051 | 253 | FAIL |
| 04_aerial_east | 67.592 | 73.211 | 13.665 | 254 | FAIL |
| 05_oblique | 87.797 | 92.898 | 4.342 | 195 | FAIL |
| 06_ground_west | 48.895 | 66.943 | 16.491 | 251 | FAIL |
| 07_far_north | 85.485 | 90.392 | 8.243 | 249 | FAIL |
| 08_back_inside | 60.960 | 77.640 | 20.709 | 238 | FAIL |
| 09_after_mesh | 53.644 | 65.117 | 25.012 | 236 | FAIL |
| 10_marker_ray | 48.437 | 63.122 | 34.954 | 250 | FAIL |
| 11_topdown | 32.180 | 85.922 | 4.218 | 212 | FAIL |
| 12_far_north_re | 85.634 | 90.424 | 8.048 | 245 | FAIL |

**GATE: RED — parity below 100%**
