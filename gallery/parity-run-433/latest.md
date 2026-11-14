# PARITY RUN 433 (run 43622435155, sha 72f281d) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 79.145 | 84.185 | 18.280 | 209 | FAIL |
| 01_aerial_west | 59.658 | 74.443 | 19.883 | 250 | FAIL |
| 02_aerial_south | 49.995 | 71.531 | 27.041 | 253 | FAIL |
| 03_aerial_north | 53.511 | 74.203 | 16.060 | 249 | FAIL |
| 04_aerial_east | 52.686 | 66.332 | 33.142 | 254 | FAIL |
| 05_oblique | 88.278 | 97.178 | 1.491 | 169 | FAIL |
| 06_ground_west | 27.664 | 61.413 | 27.582 | 249 | FAIL |
| 07_far_north | 85.292 | 90.273 | 8.272 | 249 | FAIL |
| 08_back_inside | 57.499 | 74.763 | 24.284 | 227 | FAIL |
| 09_after_mesh | 50.243 | 61.090 | 32.869 | 237 | FAIL |
| 10_marker_ray | 46.846 | 60.903 | 33.316 | 238 | FAIL |
| 11_topdown | 28.020 | 71.467 | 7.841 | 215 | FAIL |
| 12_far_north_re | 85.292 | 90.268 | 8.289 | 246 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-433/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-433/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-433/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-433/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-433/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-433/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-433/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-433/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-433/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-433/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-433/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-433/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-433/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-433/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
- view probe: [view_probe.md](parity-run-433/view_probe.md) — per-position q/settle state, the v6.26.6 settle-by-laps + dip-probe channel
