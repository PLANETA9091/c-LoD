# PARITY RUN 459 (run 43721486448, sha 8342bd9) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 76.924 | 81.779 | 21.259 | 204 | FAIL |
| 01_aerial_west | 57.670 | 71.741 | 22.129 | 251 | FAIL |
| 02_aerial_south | 48.581 | 69.862 | 28.323 | 253 | FAIL |
| 03_aerial_north | 53.976 | 74.924 | 15.896 | 253 | FAIL |
| 04_aerial_east | 48.758 | 61.702 | 39.142 | 254 | FAIL |
| 05_oblique | 87.797 | 96.034 | 3.187 | 169 | FAIL |
| 06_ground_west | 35.913 | 69.924 | 24.134 | 249 | FAIL |
| 07_far_north | 85.317 | 90.243 | 8.274 | 249 | FAIL |
| 08_back_inside | 53.105 | 69.283 | 29.115 | 227 | FAIL |
| 09_after_mesh | 42.798 | 54.483 | 38.212 | 236 | FAIL |
| 10_marker_ray | 37.348 | 53.555 | 43.937 | 238 | FAIL |
| 11_topdown | 28.172 | 72.543 | 7.436 | 215 | FAIL |
| 12_far_north_re | 85.317 | 90.245 | 8.271 | 246 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-459/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-459/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-459/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-459/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-459/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-459/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-459/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-459/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-459/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-459/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-459/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-459/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-459/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-459/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
- view probe: [view_probe.md](parity-run-459/view_probe.md) — per-position q/settle state, the v6.26.6 settle-by-laps + dip-probe channel
