# PARITY RUN 338 (run 42844478508, sha 74d6b01) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 88.972 | 92.158 | 4.254 | 210 | FAIL |
| 01_aerial_west | 52.429 | 58.895 | 27.266 | 249 | FAIL |
| 02_aerial_south | 57.621 | 71.041 | 17.145 | 253 | FAIL |
| 03_aerial_north | 52.233 | 65.478 | 17.877 | 249 | FAIL |
| 04_aerial_east | 68.132 | 75.519 | 13.573 | 254 | FAIL |
| 05_oblique | 87.531 | 91.712 | 4.954 | 192 | FAIL |
| 06_ground_west | 46.787 | 62.026 | 19.003 | 249 | FAIL |
| 07_far_north | 85.446 | 90.390 | 8.241 | 249 | FAIL |
| 08_back_inside | 61.646 | 78.239 | 19.847 | 238 | FAIL |
| 09_after_mesh | 54.528 | 65.619 | 25.188 | 237 | FAIL |
| 10_marker_ray | 50.531 | 64.387 | 33.361 | 238 | FAIL |
| 11_topdown | 31.705 | 83.609 | 4.866 | 216 | FAIL |
| 12_far_north_re | 85.446 | 90.395 | 8.229 | 246 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-338/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-338/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-338/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-338/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-338/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-338/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-338/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-338/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-338/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-338/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-338/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-338/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-338/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-338/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
