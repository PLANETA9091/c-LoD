# PARITY RUN 351 (run 42861055712, sha 134f705) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 79.385 | 81.785 | 15.712 | 220 | FAIL |
| 01_aerial_west | 53.290 | 60.284 | 26.141 | 249 | FAIL |
| 02_aerial_south | 58.079 | 71.100 | 18.439 | 253 | FAIL |
| 03_aerial_north | 49.656 | 63.015 | 21.031 | 253 | FAIL |
| 04_aerial_east | 64.628 | 72.591 | 16.397 | 254 | FAIL |
| 05_oblique | 84.482 | 87.953 | 8.183 | 192 | FAIL |
| 06_ground_west | 50.235 | 66.217 | 16.856 | 249 | FAIL |
| 07_far_north | 85.963 | 91.889 | 7.942 | 248 | FAIL |
| 08_back_inside | 59.444 | 75.866 | 22.766 | 237 | FAIL |
| 09_after_mesh | 51.472 | 62.579 | 26.764 | 227 | FAIL |
| 10_marker_ray | 49.752 | 64.008 | 33.999 | 236 | FAIL |
| 11_topdown | 32.257 | 86.003 | 4.208 | 212 | FAIL |
| 12_far_north_re | 86.054 | 91.887 | 7.924 | 245 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-351/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-351/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-351/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-351/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-351/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-351/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-351/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-351/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-351/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-351/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-351/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-351/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-351/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-351/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
