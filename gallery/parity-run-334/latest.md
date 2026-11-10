# PARITY RUN 334 (run 42780508814, sha c0ae5ff) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 75.009 | 78.431 | 23.805 | 210 | FAIL |
| 01_aerial_west | 59.784 | 70.934 | 21.613 | 248 | FAIL |
| 02_aerial_south | 48.364 | 65.631 | 33.326 | 253 | FAIL |
| 03_aerial_north | 54.569 | 72.262 | 18.275 | 253 | FAIL |
| 04_aerial_east | 42.389 | 51.537 | 50.739 | 254 | FAIL |
| 05_oblique | 94.138 | 98.962 | 0.358 | 169 | FAIL |
| 06_ground_west | 32.735 | 62.345 | 26.406 | 249 | FAIL |
| 07_far_north | 85.301 | 90.253 | 8.204 | 248 | FAIL |
| 08_back_inside | 60.559 | 77.221 | 21.328 | 238 | FAIL |
| 09_after_mesh | 54.130 | 65.527 | 27.165 | 237 | FAIL |
| 10_marker_ray | 51.807 | 65.778 | 32.094 | 238 | FAIL |
| 11_topdown | 32.112 | 85.570 | 4.286 | 216 | FAIL |
| 12_far_north_re | 85.298 | 90.234 | 8.282 | 246 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-334/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-334/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-334/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-334/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-334/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-334/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-334/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-334/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-334/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-334/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-334/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-334/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-334/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-334/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
