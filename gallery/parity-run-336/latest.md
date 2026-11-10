# PARITY RUN 336 (run 42789151568, sha 541d08f) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 75.362 | 78.779 | 23.128 | 210 | FAIL |
| 01_aerial_west | 57.853 | 68.029 | 23.104 | 248 | FAIL |
| 02_aerial_south | 49.075 | 66.189 | 31.780 | 253 | FAIL |
| 03_aerial_north | 53.970 | 71.362 | 18.457 | 253 | FAIL |
| 04_aerial_east | 43.872 | 52.597 | 48.426 | 254 | FAIL |
| 05_oblique | 91.712 | 96.754 | 1.905 | 169 | FAIL |
| 06_ground_west | 42.541 | 62.452 | 26.457 | 249 | FAIL |
| 07_far_north | 85.557 | 91.594 | 7.971 | 249 | FAIL |
| 08_back_inside | 59.265 | 74.840 | 22.113 | 238 | FAIL |
| 09_after_mesh | 53.127 | 63.663 | 28.353 | 237 | FAIL |
| 10_marker_ray | 58.246 | 70.245 | 26.639 | 238 | FAIL |
| 11_topdown | 31.450 | 82.951 | 4.964 | 216 | FAIL |
| 12_far_north_re | 85.523 | 91.583 | 8.057 | 246 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-336/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-336/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-336/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-336/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-336/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-336/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-336/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-336/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-336/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-336/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-336/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-336/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-336/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-336/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
