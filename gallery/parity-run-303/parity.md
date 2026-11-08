# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 12, ours shots: 12, paired: 12, missing (voxy shot with no ours pair): 0
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 01_aerial_west | 42.483 | 42.732 | 43.371 | 255 | FAIL |
| 02_aerial_south | 34.808 | 37.429 | 42.737 | 255 | FAIL |
| 03_aerial_north | 29.294 | 31.970 | 58.218 | 255 | FAIL |
| 04_aerial_east | 54.098 | 54.548 | 47.312 | 255 | FAIL |
| 05_oblique | 54.608 | 55.315 | 27.215 | 255 | FAIL |
| 06_ground_west | 42.310 | 44.001 | 38.773 | 255 | FAIL |
| 07_far_north | 86.463 | 90.756 | 10.660 | 226 | FAIL |
| 08_back_inside | 55.557 | 62.457 | 38.684 | 255 | FAIL |
| 09_after_mesh | 46.226 | 49.838 | 43.377 | 255 | FAIL |
| 10_marker_ray | 50.783 | 52.135 | 50.412 | 255 | FAIL |
| 11_topdown | 22.838 | 58.947 | 19.546 | 228 | FAIL |
| 12_far_north_re | 85.382 | 90.257 | 8.104 | 245 | FAIL |

**GATE: RED — parity below 100%**
