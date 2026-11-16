# PARITY RUN 479 (run 43945096947, sha 9a5f266) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 68.983 | 75.729 | 29.559 | 214 | FAIL |
| 01_aerial_west | 48.034 | 59.796 | 35.798 | 251 | FAIL |
| 02_aerial_south | 40.763 | 61.209 | 36.655 | 253 | FAIL |
| 03_aerial_north | 48.719 | 67.731 | 23.671 | 253 | FAIL |
| 04_aerial_east | 51.536 | 67.143 | 28.030 | 249 | FAIL |
| 05_oblique | 78.732 | 93.745 | 5.880 | 248 | FAIL |
| 06_ground_west | 38.998 | 65.800 | 17.833 | 250 | FAIL |
| 07_far_north | 85.278 | 90.245 | 8.278 | 249 | FAIL |
| 08_back_inside | 45.337 | 64.831 | 34.152 | 231 | FAIL |
| 09_after_mesh | 34.281 | 47.529 | 48.494 | 237 | FAIL |
| 10_marker_ray | 38.502 | 57.105 | 39.713 | 238 | FAIL |
| 11_topdown | 23.497 | 66.893 | 8.459 | 216 | FAIL |
| 12_far_north_re | 85.278 | 90.241 | 8.286 | 246 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-479/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-479/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-479/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-479/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-479/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-479/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-479/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-479/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-479/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-479/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-479/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-479/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-479/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-479/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
- view probe: [view_probe.md](parity-run-479/view_probe.md) — per-position q/settle state, the v6.26.6 settle-by-laps + dip-probe channel
