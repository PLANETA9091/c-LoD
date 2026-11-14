# PARITY RUN 431 (run 43612603919, sha 14ab34b) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 79.200 | 84.123 | 18.475 | 210 | FAIL |
| 01_aerial_west | 58.854 | 72.202 | 20.544 | 251 | FAIL |
| 02_aerial_south | 49.403 | 70.230 | 27.224 | 253 | FAIL |
| 03_aerial_north | 53.699 | 74.348 | 16.054 | 253 | FAIL |
| 04_aerial_east | 51.953 | 64.867 | 34.561 | 254 | FAIL |
| 05_oblique | 86.520 | 96.525 | 2.321 | 169 | FAIL |
| 06_ground_west | 27.192 | 61.159 | 26.189 | 249 | FAIL |
| 07_far_north | 85.481 | 90.280 | 8.100 | 248 | FAIL |
| 08_back_inside | 56.491 | 72.825 | 24.637 | 238 | FAIL |
| 09_after_mesh | 47.815 | 56.828 | 36.549 | 233 | FAIL |
| 10_marker_ray | 40.226 | 54.464 | 38.777 | 236 | FAIL |
| 11_topdown | 26.223 | 66.787 | 8.978 | 223 | FAIL |
| 12_far_north_re | 85.481 | 90.280 | 8.106 | 245 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-431/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-431/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-431/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-431/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-431/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-431/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-431/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-431/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-431/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-431/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-431/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-431/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-431/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-431/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
- view probe: [view_probe.md](parity-run-431/view_probe.md) — per-position q/settle state, the v6.26.6 settle-by-laps + dip-probe channel
