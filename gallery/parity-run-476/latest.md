# PARITY RUN 476 (run 43926875267, sha 9d51ee6) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 68.327 | 75.649 | 29.998 | 210 | FAIL |
| 01_aerial_west | 44.489 | 56.900 | 37.509 | 251 | FAIL |
| 02_aerial_south | 40.073 | 57.046 | 39.209 | 253 | FAIL |
| 03_aerial_north | 50.222 | 69.689 | 21.243 | 250 | FAIL |
| 04_aerial_east | 53.898 | 68.907 | 26.136 | 254 | FAIL |
| 05_oblique | 76.910 | 91.847 | 6.033 | 250 | FAIL |
| 06_ground_west | 35.812 | 64.302 | 19.631 | 249 | FAIL |
| 07_far_north | 85.317 | 90.247 | 8.270 | 249 | FAIL |
| 08_back_inside | 45.576 | 66.259 | 32.554 | 238 | FAIL |
| 09_after_mesh | 34.097 | 47.931 | 47.810 | 237 | FAIL |
| 10_marker_ray | 41.364 | 60.494 | 34.299 | 238 | FAIL |
| 11_topdown | 23.322 | 67.602 | 8.140 | 216 | FAIL |
| 12_far_north_re | 85.317 | 90.243 | 8.281 | 246 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-476/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-476/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-476/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-476/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-476/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-476/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-476/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-476/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-476/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-476/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-476/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-476/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-476/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-476/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
- view probe: [view_probe.md](parity-run-476/view_probe.md) — per-position q/settle state, the v6.26.6 settle-by-laps + dip-probe channel
