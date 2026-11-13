# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 91.707 | 97.352 | 1.651 | 201 | FAIL |
| 01_aerial_west | 57.672 | 72.487 | 20.452 | 251 | FAIL |
| 02_aerial_south | 66.434 | 85.048 | 9.749 | 253 | FAIL |
| 03_aerial_north | 56.809 | 76.152 | 14.419 | 253 | FAIL |
| 04_aerial_east | 67.985 | 78.472 | 17.016 | 254 | FAIL |
| 05_oblique | 80.512 | 90.153 | 9.231 | 192 | FAIL |
| 06_ground_west | 27.318 | 61.521 | 27.298 | 249 | FAIL |
| 07_far_north | 85.462 | 90.247 | 8.129 | 249 | FAIL |
| 08_back_inside | 57.844 | 74.687 | 24.274 | 237 | FAIL |
| 09_after_mesh | 47.229 | 56.592 | 36.024 | 233 | FAIL |
| 10_marker_ray | 50.048 | 63.194 | 31.311 | 236 | FAIL |
| 11_topdown | 27.394 | 70.484 | 8.156 | 223 | FAIL |
| 12_far_north_re | 85.462 | 90.251 | 8.126 | 245 | FAIL |

**GATE: RED — parity below 100%**
