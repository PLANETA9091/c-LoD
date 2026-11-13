# PARITY RUN 399 (run 43467483199, sha df2ae05) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 79.132 | 84.144 | 18.657 | 209 | FAIL |
| 01_aerial_west | 59.851 | 74.180 | 19.664 | 248 | FAIL |
| 02_aerial_south | 50.965 | 73.544 | 22.689 | 253 | FAIL |
| 03_aerial_north | 54.884 | 77.557 | 15.030 | 253 | FAIL |
| 04_aerial_east | 51.479 | 65.579 | 33.257 | 254 | FAIL |
| 05_oblique | 89.276 | 97.417 | 1.302 | 171 | FAIL |
| 06_ground_west | 39.628 | 72.043 | 22.692 | 248 | FAIL |
| 07_far_north | 85.576 | 90.413 | 8.062 | 248 | FAIL |
| 08_back_inside | 57.511 | 73.961 | 24.054 | 237 | FAIL |
| 09_after_mesh | 49.629 | 58.981 | 34.406 | 236 | FAIL |
| 10_marker_ray | 40.253 | 55.405 | 38.466 | 238 | FAIL |
| 11_topdown | 27.317 | 69.663 | 8.430 | 215 | FAIL |
| 12_far_north_re | 85.438 | 90.383 | 8.246 | 246 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-399/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-399/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-399/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-399/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-399/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-399/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-399/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-399/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-399/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-399/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-399/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-399/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-399/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-399/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
- view probe: [view_probe.md](parity-run-399/view_probe.md) — per-position q/settle state, the v6.26.6 settle-by-laps + dip-probe channel
