# PARITY RUN 470 (run 43905538094, sha 64957b8) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 77.324 | 82.518 | 20.944 | 209 | FAIL |
| 01_aerial_west | 60.383 | 74.354 | 19.170 | 250 | FAIL |
| 02_aerial_south | 48.541 | 71.234 | 25.229 | 253 | FAIL |
| 03_aerial_north | 50.045 | 70.749 | 20.410 | 254 | FAIL |
| 04_aerial_east | 52.982 | 68.436 | 26.718 | 254 | FAIL |
| 05_oblique | 75.880 | 85.363 | 18.473 | 222 | FAIL |
| 06_ground_west | 29.764 | 64.006 | 22.938 | 249 | FAIL |
| 07_far_north | 85.264 | 90.248 | 8.252 | 249 | FAIL |
| 08_back_inside | 49.544 | 66.294 | 31.554 | 227 | FAIL |
| 09_after_mesh | 39.366 | 53.188 | 40.392 | 237 | FAIL |
| 10_marker_ray | 51.503 | 69.062 | 27.074 | 250 | FAIL |
| 11_topdown | 27.132 | 74.200 | 6.582 | 216 | FAIL |
| 12_far_north_re | 85.278 | 90.243 | 8.280 | 246 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-470/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-470/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-470/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-470/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-470/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-470/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-470/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-470/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-470/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-470/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-470/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-470/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-470/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-470/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
- view probe: [view_probe.md](parity-run-470/view_probe.md) — per-position q/settle state, the v6.26.6 settle-by-laps + dip-probe channel
