# PARITY RUN 305 (run 42511241270, sha 008d800) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 12, ours shots: 12, paired: 12, missing (voxy shot with no ours pair): 0
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 01_aerial_west | 51.500 | 52.710 | 25.179 | 254 | FAIL |
| 02_aerial_south | 45.326 | 48.558 | 31.647 | 254 | FAIL |
| 03_aerial_north | 45.880 | 48.757 | 20.259 | 254 | FAIL |
| 04_aerial_east | 73.373 | 73.419 | 14.020 | 253 | FAIL |
| 05_oblique | 57.557 | 58.264 | 19.924 | 143 | FAIL |
| 06_ground_west | 43.437 | 51.829 | 23.720 | 254 | FAIL |
| 07_far_north | 86.632 | 91.582 | 8.949 | 226 | FAIL |
| 08_back_inside | 66.719 | 70.615 | 42.578 | 243 | FAIL |
| 09_after_mesh | 60.650 | 69.429 | 20.249 | 230 | FAIL |
| 10_marker_ray | 71.350 | 75.993 | 17.319 | 240 | FAIL |
| 11_topdown | 29.248 | 77.859 | 6.468 | 223 | FAIL |
| 12_far_north_re | 85.464 | 90.258 | 8.111 | 245 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_01_aerial_west.jpg](parity-run-305/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-305/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-305/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-305/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-305/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-305/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-305/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-305/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-305/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-305/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-305/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-305/tri_12_far_north_re.jpg)
