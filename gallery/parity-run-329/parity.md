# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 75.175 | 78.655 | 23.624 | 210 | FAIL |
| 01_aerial_west | 58.570 | 70.606 | 19.462 | 251 | FAIL |
| 02_aerial_south | 48.795 | 66.204 | 32.079 | 250 | FAIL |
| 03_aerial_north | 53.973 | 72.184 | 17.986 | 253 | FAIL |
| 04_aerial_east | 43.265 | 51.645 | 47.326 | 254 | FAIL |
| 05_oblique | 92.935 | 97.957 | 0.821 | 169 | FAIL |
| 06_ground_west | 36.892 | 65.082 | 25.310 | 249 | FAIL |
| 07_far_north | 87.147 | 95.747 | 1.992 | 223 | FAIL |
| 08_back_inside | 58.358 | 62.262 | 49.973 | 239 | FAIL |
| 09_after_mesh | 55.183 | 66.228 | 25.258 | 238 | FAIL |
| 10_marker_ray | 44.887 | 63.879 | 31.996 | 236 | FAIL |
| 11_topdown | 27.484 | 66.481 | 9.105 | 223 | FAIL |
| 12_far_north_re | 85.745 | 91.532 | 7.978 | 243 | FAIL |

**GATE: RED — parity below 100%**
