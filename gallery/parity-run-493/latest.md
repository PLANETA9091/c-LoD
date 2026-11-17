# PARITY RUN 493 (run 44080209719, sha 3242331) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 67.941 | 77.036 | 26.017 | 241 | FAIL |
| 01_aerial_west | 47.850 | 60.040 | 34.025 | 249 | FAIL |
| 02_aerial_south | 39.708 | 56.464 | 36.337 | 255 | FAIL |
| 03_aerial_north | 46.855 | 63.022 | 26.417 | 255 | FAIL |
| 04_aerial_east | 50.852 | 64.501 | 24.294 | 255 | FAIL |
| 05_oblique | 73.674 | 88.562 | 10.089 | 222 | FAIL |
| 06_ground_west | 41.380 | 64.505 | 16.938 | 251 | FAIL |
| 07_far_north | 85.394 | 90.259 | 8.109 | 248 | FAIL |
| 08_back_inside | 42.435 | 64.396 | 33.925 | 237 | FAIL |
| 09_after_mesh | 31.943 | 47.465 | 48.587 | 233 | FAIL |
| 10_marker_ray | 40.888 | 59.084 | 34.979 | 240 | FAIL |
| 11_topdown | 22.800 | 68.151 | 7.992 | 223 | FAIL |
| 12_far_north_re | 85.394 | 90.261 | 8.101 | 245 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-493/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-493/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-493/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-493/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-493/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-493/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-493/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-493/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-493/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-493/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-493/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-493/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-493/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-493/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
- view probe: [view_probe.md](parity-run-493/view_probe.md) — per-position q/settle state, the v6.26.6 settle-by-laps + dip-probe channel
