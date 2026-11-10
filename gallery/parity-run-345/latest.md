# PARITY RUN 345 (run 42853146129, sha 82add1a) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 77.312 | 80.372 | 18.391 | 222 | FAIL |
| 01_aerial_west | 46.659 | 57.990 | 34.625 | 249 | FAIL |
| 02_aerial_south | 42.087 | 59.247 | 40.090 | 253 | FAIL |
| 03_aerial_north | 52.781 | 70.206 | 17.336 | 250 | FAIL |
| 04_aerial_east | 47.912 | 57.122 | 37.283 | 254 | FAIL |
| 05_oblique | 85.549 | 90.997 | 8.931 | 209 | FAIL |
| 06_ground_west | 34.668 | 63.703 | 21.051 | 249 | FAIL |
| 07_far_north | 85.461 | 90.252 | 8.111 | 249 | FAIL |
| 08_back_inside | 60.069 | 76.692 | 21.403 | 237 | FAIL |
| 09_after_mesh | 53.868 | 64.895 | 24.710 | 233 | FAIL |
| 10_marker_ray | 48.894 | 63.889 | 33.924 | 240 | FAIL |
| 11_topdown | 31.489 | 82.874 | 4.892 | 212 | FAIL |
| 12_far_north_re | 85.461 | 90.251 | 8.123 | 246 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-345/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-345/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-345/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-345/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-345/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-345/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-345/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-345/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-345/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-345/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-345/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-345/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-345/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-345/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
