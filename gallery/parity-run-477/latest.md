# PARITY RUN 477 (run 43933786416, sha a5e57b5) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 68.308 | 75.036 | 30.631 | 241 | FAIL |
| 01_aerial_west | 54.216 | 65.405 | 28.650 | 251 | FAIL |
| 02_aerial_south | 47.218 | 66.245 | 29.534 | 253 | FAIL |
| 03_aerial_north | 51.360 | 71.643 | 19.620 | 253 | FAIL |
| 04_aerial_east | 53.806 | 69.059 | 25.366 | 254 | FAIL |
| 05_oblique | 76.866 | 90.987 | 7.760 | 248 | FAIL |
| 06_ground_west | 39.008 | 65.621 | 19.590 | 250 | FAIL |
| 07_far_north | 85.269 | 90.234 | 8.295 | 249 | FAIL |
| 08_back_inside | 44.224 | 64.558 | 35.445 | 227 | FAIL |
| 09_after_mesh | 32.708 | 46.394 | 51.773 | 236 | FAIL |
| 10_marker_ray | 38.848 | 55.466 | 40.072 | 253 | FAIL |
| 11_topdown | 22.923 | 66.362 | 8.662 | 216 | FAIL |
| 12_far_north_re | 85.269 | 90.237 | 8.278 | 246 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-477/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-477/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-477/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-477/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-477/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-477/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-477/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-477/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-477/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-477/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-477/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-477/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-477/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-477/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
- view probe: [view_probe.md](parity-run-477/view_probe.md) — per-position q/settle state, the v6.26.6 settle-by-laps + dip-probe channel
