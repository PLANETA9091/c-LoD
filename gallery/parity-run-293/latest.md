# PARITY RUN 293 (run 42480623231, sha 4d6e4fa) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 12, ours shots: 12, paired: 12, missing (voxy shot with no ours pair): 0
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 01_aerial_west | 68.656 | 88.003 | 10.851 | 234 | FAIL |
| 02_aerial_south | 70.618 | 92.116 | 7.061 | 253 | FAIL |
| 03_aerial_north | 62.149 | 86.951 | 10.440 | 253 | FAIL |
| 04_aerial_east | 76.142 | 90.115 | 7.065 | 248 | FAIL |
| 05_oblique | 94.824 | 99.762 | 0.144 | 18 | FAIL |
| 06_ground_west | 63.778 | 90.206 | 7.715 | 254 | FAIL |
| 07_far_north | 97.873 | 97.873 | 3.074 | 245 | FAIL |
| 08_back_inside | 69.918 | 82.244 | 22.637 | 238 | FAIL |
| 09_after_mesh | 64.265 | 83.604 | 17.149 | 234 | FAIL |
| 10_marker_ray | 75.835 | 85.369 | 14.962 | 240 | FAIL |
| 11_topdown | 34.485 | 90.730 | 3.227 | 223 | FAIL |
| 12_far_north_re | 85.404 | 85.610 | 19.337 | 249 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_01_aerial_west.jpg](parity-run-293/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-293/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-293/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-293/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-293/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-293/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-293/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-293/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-293/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-293/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-293/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-293/tri_12_far_north_re.jpg)
