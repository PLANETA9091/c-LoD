# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 75.150 | 78.697 | 23.496 | 210 | FAIL |
| 01_aerial_west | 58.770 | 70.275 | 19.890 | 248 | FAIL |
| 02_aerial_south | 48.723 | 66.189 | 32.299 | 253 | FAIL |
| 03_aerial_north | 54.120 | 72.356 | 17.532 | 253 | FAIL |
| 04_aerial_east | 42.181 | 50.105 | 49.581 | 254 | FAIL |
| 05_oblique | 93.208 | 98.225 | 0.878 | 163 | FAIL |
| 06_ground_west | 35.301 | 65.086 | 25.349 | 249 | FAIL |
| 07_far_north | 86.611 | 90.922 | 9.995 | 225 | FAIL |
| 08_back_inside | 61.487 | 65.523 | 49.937 | 239 | FAIL |
| 09_after_mesh | 59.320 | 71.255 | 19.048 | 238 | FAIL |
| 10_marker_ray | 46.944 | 64.239 | 31.930 | 250 | FAIL |
| 11_topdown | 27.765 | 67.591 | 8.947 | 223 | FAIL |
| 12_far_north_re | 85.923 | 91.817 | 7.744 | 245 | FAIL |

**GATE: RED — parity below 100%**
