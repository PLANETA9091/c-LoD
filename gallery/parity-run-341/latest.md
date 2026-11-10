# PARITY RUN 341 (run 42847767865, sha a96cd01) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 78.899 | 82.349 | 16.319 | 220 | FAIL |
| 01_aerial_west | 54.194 | 63.885 | 26.028 | 250 | FAIL |
| 02_aerial_south | 43.854 | 58.778 | 36.993 | 253 | FAIL |
| 03_aerial_north | 50.650 | 67.342 | 19.647 | 253 | FAIL |
| 04_aerial_east | 47.907 | 55.523 | 39.372 | 254 | FAIL |
| 05_oblique | 87.621 | 93.134 | 4.491 | 192 | FAIL |
| 06_ground_west | 37.315 | 64.019 | 17.510 | 249 | FAIL |
| 07_far_north | 85.296 | 90.232 | 8.299 | 249 | FAIL |
| 08_back_inside | 60.385 | 77.260 | 21.348 | 238 | FAIL |
| 09_after_mesh | 54.084 | 65.484 | 27.323 | 237 | FAIL |
| 10_marker_ray | 47.109 | 63.404 | 34.021 | 238 | FAIL |
| 11_topdown | 31.731 | 83.354 | 4.732 | 214 | FAIL |
| 12_far_north_re | 85.457 | 90.249 | 8.120 | 245 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-341/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-341/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-341/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-341/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-341/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-341/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-341/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-341/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-341/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-341/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-341/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-341/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-341/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-341/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
