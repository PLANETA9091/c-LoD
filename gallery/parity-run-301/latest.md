# PARITY RUN 301 (run 42499555944, sha 927f1e9) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 12, ours shots: 12, paired: 12, missing (voxy shot with no ours pair): 0
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 01_aerial_west | 68.580 | 88.525 | 10.271 | 255 | FAIL |
| 02_aerial_south | 70.707 | 92.399 | 6.883 | 255 | FAIL |
| 03_aerial_north | 62.510 | 88.179 | 9.835 | 249 | FAIL |
| 04_aerial_east | 76.203 | 90.216 | 6.546 | 255 | FAIL |
| 05_oblique | 94.823 | 99.761 | 0.145 | 255 | FAIL |
| 06_ground_west | 63.348 | 89.933 | 7.620 | 255 | FAIL |
| 07_far_north | 97.872 | 97.872 | 3.075 | 255 | FAIL |
| 08_back_inside | 71.377 | 88.609 | 11.090 | 255 | FAIL |
| 09_after_mesh | 64.209 | 84.033 | 16.562 | 255 | FAIL |
| 10_marker_ray | 76.040 | 86.446 | 14.234 | 255 | FAIL |
| 11_topdown | 33.975 | 88.702 | 3.751 | 223 | FAIL |
| 12_far_north_re | 85.574 | 87.054 | 14.509 | 255 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_01_aerial_west.jpg](parity-run-301/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-301/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-301/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-301/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-301/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-301/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-301/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-301/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-301/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-301/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-301/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-301/tri_12_far_north_re.jpg)
