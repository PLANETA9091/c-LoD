# PARITY RUN 482 (run 44043887160, sha e01f8ea) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 67.924 | 74.745 | 30.772 | 218 | FAIL |
| 01_aerial_west | 57.861 | 72.005 | 23.016 | 248 | FAIL |
| 02_aerial_south | 48.061 | 67.925 | 28.536 | 253 | FAIL |
| 03_aerial_north | 48.706 | 67.760 | 22.272 | 253 | FAIL |
| 04_aerial_east | 52.544 | 68.611 | 26.926 | 254 | FAIL |
| 05_oblique | 82.285 | 93.552 | 4.878 | 248 | FAIL |
| 06_ground_west | 35.215 | 69.770 | 16.681 | 250 | FAIL |
| 07_far_north | 85.269 | 90.236 | 8.289 | 249 | FAIL |
| 08_back_inside | 44.426 | 65.539 | 34.036 | 238 | FAIL |
| 09_after_mesh | 34.012 | 48.243 | 49.560 | 237 | FAIL |
| 10_marker_ray | 42.062 | 59.564 | 34.161 | 250 | FAIL |
| 11_topdown | 23.038 | 66.706 | 8.369 | 216 | FAIL |
| 12_far_north_re | 85.269 | 90.234 | 8.289 | 246 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-482/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-482/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-482/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-482/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-482/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-482/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-482/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-482/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-482/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-482/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-482/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-482/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-482/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-482/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
- view probe: [view_probe.md](parity-run-482/view_probe.md) — per-position q/settle state, the v6.26.6 settle-by-laps + dip-probe channel
