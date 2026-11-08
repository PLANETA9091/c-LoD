# PARITY RUN 302 (run 42503115222, sha 797583a) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 12, ours shots: 12, paired: 12, missing (voxy shot with no ours pair): 0
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 01_aerial_west | 54.693 | 70.753 | 31.515 | 255 | FAIL |
| 02_aerial_south | 56.230 | 73.453 | 28.338 | 255 | FAIL |
| 03_aerial_north | 44.971 | 68.659 | 42.185 | 255 | FAIL |
| 04_aerial_east | 57.804 | 69.725 | 39.572 | 255 | FAIL |
| 05_oblique | 74.404 | 77.631 | 15.205 | 255 | FAIL |
| 06_ground_west | 52.517 | 73.700 | 23.991 | 255 | FAIL |
| 07_far_north | 72.205 | 72.205 | 48.845 | 255 | FAIL |
| 08_back_inside | 57.152 | 67.303 | 37.113 | 255 | FAIL |
| 09_after_mesh | 50.813 | 66.911 | 36.172 | 255 | FAIL |
| 10_marker_ray | 57.130 | 67.293 | 45.963 | 255 | FAIL |
| 11_topdown | 25.631 | 70.090 | 15.799 | 223 | FAIL |
| 12_far_north_re | 63.339 | 63.340 | 60.228 | 255 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_01_aerial_west.jpg](parity-run-302/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-302/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-302/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-302/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-302/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-302/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-302/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-302/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-302/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-302/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-302/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-302/tri_12_far_north_re.jpg)
