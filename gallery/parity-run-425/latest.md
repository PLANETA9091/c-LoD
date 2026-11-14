# PARITY RUN 425 (run 43584846332, sha ba89c57) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 79.237 | 84.321 | 18.181 | 210 | FAIL |
| 01_aerial_west | 61.234 | 77.582 | 18.769 | 249 | FAIL |
| 02_aerial_south | 50.002 | 72.503 | 26.344 | 253 | FAIL |
| 03_aerial_north | 53.921 | 74.959 | 16.059 | 253 | FAIL |
| 04_aerial_east | 52.033 | 65.292 | 33.911 | 254 | FAIL |
| 05_oblique | 92.115 | 98.970 | 0.374 | 169 | FAIL |
| 06_ground_west | 47.815 | 75.163 | 19.466 | 249 | FAIL |
| 07_far_north | 85.454 | 91.510 | 8.165 | 249 | FAIL |
| 08_back_inside | 54.783 | 71.583 | 26.747 | 238 | FAIL |
| 09_after_mesh | 44.093 | 54.394 | 39.003 | 237 | FAIL |
| 10_marker_ray | 41.706 | 55.959 | 37.371 | 238 | FAIL |
| 11_topdown | 25.835 | 66.496 | 8.967 | 216 | FAIL |
| 12_far_north_re | 85.454 | 91.510 | 8.177 | 246 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-425/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-425/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-425/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-425/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-425/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-425/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-425/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-425/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-425/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-425/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-425/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-425/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-425/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-425/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
- view probe: [view_probe.md](parity-run-425/view_probe.md) — per-position q/settle state, the v6.26.6 settle-by-laps + dip-probe channel
