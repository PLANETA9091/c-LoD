# PARITY RUN 307 (run 42522828042, sha 8b7cf42) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 12, ours shots: 12, paired: 12, missing (voxy shot with no ours pair): 0
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 01_aerial_west | 51.762 | 55.956 | 16.215 | 230 | FAIL |
| 02_aerial_south | 56.546 | 67.463 | 15.055 | 253 | FAIL |
| 03_aerial_north | 46.089 | 51.592 | 21.173 | 253 | FAIL |
| 04_aerial_east | 72.785 | 74.841 | 10.556 | 248 | FAIL |
| 05_oblique | 79.008 | 79.947 | 10.222 | 106 | FAIL |
| 06_ground_west | 44.232 | 58.199 | 12.709 | 249 | FAIL |
| 07_far_north | 86.618 | 91.825 | 8.886 | 226 | FAIL |
| 08_back_inside | 59.091 | 63.685 | 11.827 | 226 | FAIL |
| 09_after_mesh | 50.173 | 54.397 | 20.675 | 224 | FAIL |
| 10_marker_ray | 69.895 | 71.921 | 12.549 | 220 | FAIL |
| 11_topdown | 34.008 | 89.183 | 3.631 | 223 | FAIL |
| 12_far_north_re | 85.391 | 90.254 | 8.119 | 245 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_01_aerial_west.jpg](parity-run-307/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-307/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-307/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-307/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-307/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-307/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-307/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-307/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-307/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-307/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-307/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-307/tri_12_far_north_re.jpg)
