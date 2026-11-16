# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 77.635 | 82.576 | 21.000 | 204 | FAIL |
| 01_aerial_west | 58.739 | 73.502 | 20.067 | 254 | FAIL |
| 02_aerial_south | 50.563 | 72.367 | 26.749 | 253 | FAIL |
| 03_aerial_north | 51.914 | 72.889 | 19.434 | 253 | FAIL |
| 04_aerial_east | 50.820 | 65.837 | 35.209 | 254 | FAIL |
| 05_oblique | 88.055 | 96.080 | 3.188 | 177 | FAIL |
| 06_ground_west | 33.373 | 68.035 | 23.150 | 249 | FAIL |
| 07_far_north | 85.278 | 90.243 | 8.271 | 249 | FAIL |
| 08_back_inside | 59.848 | 75.897 | 23.458 | 238 | FAIL |
| 09_after_mesh | 50.316 | 64.628 | 29.844 | 231 | FAIL |
| 10_marker_ray | 43.986 | 59.545 | 40.316 | 238 | FAIL |
| 11_topdown | 29.304 | 76.985 | 6.132 | 216 | FAIL |
| 12_far_north_re | 85.278 | 90.243 | 8.276 | 246 | FAIL |

**GATE: RED — parity below 100%**
