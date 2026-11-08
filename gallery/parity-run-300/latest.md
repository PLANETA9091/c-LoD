# PARITY RUN 300 (run 42493579041, sha 0451883) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 12, ours shots: 12, paired: 12, missing (voxy shot with no ours pair): 0
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 01_aerial_west | 68.491 | 87.264 | 10.934 | 255 | FAIL |
| 02_aerial_south | 70.319 | 91.201 | 7.324 | 255 | FAIL |
| 03_aerial_north | 61.653 | 85.851 | 10.727 | 253 | FAIL |
| 04_aerial_east | 75.636 | 88.426 | 7.390 | 255 | FAIL |
| 05_oblique | 94.823 | 99.761 | 0.145 | 255 | FAIL |
| 06_ground_west | 63.481 | 89.321 | 7.835 | 255 | FAIL |
| 07_far_north | 91.891 | 91.891 | 12.175 | 255 | FAIL |
| 08_back_inside | 67.438 | 71.674 | 38.213 | 255 | FAIL |
| 09_after_mesh | 64.046 | 82.775 | 17.296 | 255 | FAIL |
| 10_marker_ray | 75.847 | 85.281 | 14.947 | 255 | FAIL |
| 11_topdown | 33.840 | 88.392 | 3.919 | 223 | FAIL |
| 12_far_north_re | 85.178 | 85.366 | 20.296 | 255 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_01_aerial_west.jpg](parity-run-300/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-300/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-300/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-300/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-300/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-300/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-300/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-300/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-300/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-300/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-300/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-300/tri_12_far_north_re.jpg)
