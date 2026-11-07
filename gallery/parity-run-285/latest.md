# PARITY RUN 285 (run 42411779934, sha 44ca5a7) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 10, ours shots: 10, paired: 10, missing (voxy shot with no ours pair): 0
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 01_aerial_west | 68.274 | 90.230 | 6.378 | 218 | FAIL |
| 02_aerial_south | 70.473 | 91.581 | 7.088 | 248 | FAIL |
| 03_aerial_north | 62.476 | 88.278 | 9.710 | 249 | FAIL |
| 04_aerial_east | 76.161 | 90.197 | 6.442 | 248 | FAIL |
| 05_oblique | 94.824 | 99.762 | 0.144 | 18 | FAIL |
| 06_ground_west | 63.403 | 88.946 | 7.795 | 249 | FAIL |
| 07_far_north | 100.000 | 100.000 | 0.000 | 1 | FAIL |
| 08_back_inside | 70.453 | 84.661 | 18.479 | 227 | FAIL |
| 09_after_mesh | 64.275 | 83.917 | 16.527 | 251 | FAIL |
| 10_marker_ray | 76.091 | 86.347 | 14.189 | 240 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_01_aerial_west.jpg](parity-run-285/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-285/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-285/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-285/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-285/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-285/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-285/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-285/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-285/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-285/tri_10_marker_ray.jpg)
