# PARITY RUN 304 (run 42508555616, sha 20bfd36) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 12, ours shots: 12, paired: 12, missing (voxy shot with no ours pair): 0
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 01_aerial_west | 52.257 | 52.791 | 26.234 | 255 | FAIL |
| 02_aerial_south | 46.069 | 49.135 | 30.592 | 254 | FAIL |
| 03_aerial_north | 46.289 | 48.973 | 28.850 | 254 | FAIL |
| 04_aerial_east | 73.412 | 75.576 | 12.423 | 253 | FAIL |
| 05_oblique | 57.557 | 58.264 | 19.688 | 143 | FAIL |
| 06_ground_west | 50.743 | 52.078 | 24.427 | 254 | FAIL |
| 07_far_north | 85.392 | 90.258 | 8.109 | 248 | FAIL |
| 08_back_inside | 69.738 | 83.546 | 11.500 | 238 | FAIL |
| 09_after_mesh | 59.266 | 65.177 | 19.151 | 243 | FAIL |
| 10_marker_ray | 69.759 | 71.105 | 15.788 | 254 | FAIL |
| 11_topdown | 30.497 | 76.012 | 6.770 | 223 | FAIL |
| 12_far_north_re | 85.393 | 90.259 | 8.111 | 245 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_01_aerial_west.jpg](parity-run-304/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-304/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-304/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-304/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-304/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-304/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-304/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-304/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-304/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-304/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-304/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-304/tri_12_far_north_re.jpg)
