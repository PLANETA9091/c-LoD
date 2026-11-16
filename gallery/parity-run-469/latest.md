# PARITY RUN 469 (run 43905470733, sha bafaf97) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 77.607 | 82.850 | 20.565 | 204 | FAIL |
| 01_aerial_west | 60.300 | 75.213 | 17.933 | 250 | FAIL |
| 02_aerial_south | 49.183 | 71.297 | 25.658 | 255 | FAIL |
| 03_aerial_north | 51.238 | 71.272 | 18.903 | 249 | FAIL |
| 04_aerial_east | 51.568 | 66.475 | 29.134 | 255 | FAIL |
| 05_oblique | 75.269 | 85.266 | 16.335 | 222 | FAIL |
| 06_ground_west | 29.488 | 65.732 | 22.082 | 249 | FAIL |
| 07_far_north | 85.281 | 90.313 | 8.231 | 248 | FAIL |
| 08_back_inside | 50.004 | 68.372 | 30.956 | 238 | FAIL |
| 09_after_mesh | 41.308 | 57.137 | 38.828 | 236 | FAIL |
| 10_marker_ray | 55.803 | 70.704 | 23.871 | 238 | FAIL |
| 11_topdown | 27.813 | 76.601 | 6.159 | 216 | FAIL |
| 12_far_north_re | 85.294 | 90.311 | 8.261 | 246 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-469/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-469/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-469/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-469/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-469/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-469/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-469/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-469/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-469/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-469/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-469/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-469/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-469/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-469/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
- view probe: [view_probe.md](parity-run-469/view_probe.md) — per-position q/settle state, the v6.26.6 settle-by-laps + dip-probe channel
