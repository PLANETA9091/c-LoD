# PARITY RUN 436 (run 43638390398, sha da7f25c) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 79.473 | 84.576 | 18.094 | 209 | FAIL |
| 01_aerial_west | 58.120 | 72.471 | 21.177 | 248 | FAIL |
| 02_aerial_south | 49.780 | 71.770 | 27.288 | 253 | FAIL |
| 03_aerial_north | 53.250 | 73.731 | 16.729 | 253 | FAIL |
| 04_aerial_east | 51.692 | 65.228 | 35.731 | 254 | FAIL |
| 05_oblique | 85.754 | 95.320 | 3.793 | 171 | FAIL |
| 06_ground_west | 26.100 | 60.664 | 27.465 | 248 | FAIL |
| 07_far_north | 85.787 | 91.602 | 7.871 | 248 | FAIL |
| 08_back_inside | 57.834 | 75.171 | 23.802 | 239 | FAIL |
| 09_after_mesh | 50.658 | 63.124 | 29.729 | 234 | FAIL |
| 10_marker_ray | 41.046 | 58.199 | 37.659 | 237 | FAIL |
| 11_topdown | 28.349 | 73.792 | 6.935 | 215 | FAIL |
| 12_far_north_re | 85.535 | 91.583 | 8.044 | 246 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-436/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-436/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-436/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-436/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-436/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-436/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-436/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-436/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-436/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-436/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-436/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-436/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-436/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-436/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
- view probe: [view_probe.md](parity-run-436/view_probe.md) — per-position q/settle state, the v6.26.6 settle-by-laps + dip-probe channel
