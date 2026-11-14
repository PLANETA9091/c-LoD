# PARITY RUN 427 (run 43594124037, sha cb3b82c) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 79.274 | 84.390 | 18.267 | 209 | FAIL |
| 01_aerial_west | 58.828 | 72.577 | 20.314 | 251 | FAIL |
| 02_aerial_south | 50.183 | 72.117 | 21.497 | 253 | FAIL |
| 03_aerial_north | 55.415 | 77.444 | 14.802 | 253 | FAIL |
| 04_aerial_east | 52.438 | 65.985 | 29.007 | 254 | FAIL |
| 05_oblique | 87.742 | 96.753 | 1.779 | 171 | FAIL |
| 06_ground_west | 26.146 | 60.524 | 26.763 | 249 | FAIL |
| 07_far_north | 85.462 | 90.250 | 8.127 | 248 | FAIL |
| 08_back_inside | 58.074 | 73.633 | 23.408 | 239 | FAIL |
| 09_after_mesh | 50.320 | 62.129 | 29.602 | 233 | FAIL |
| 10_marker_ray | 41.516 | 58.579 | 36.859 | 236 | FAIL |
| 11_topdown | 27.484 | 71.414 | 7.496 | 217 | FAIL |
| 12_far_north_re | 85.462 | 90.253 | 8.120 | 245 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-427/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-427/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-427/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-427/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-427/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-427/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-427/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-427/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-427/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-427/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-427/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-427/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-427/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-427/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
- view probe: [view_probe.md](parity-run-427/view_probe.md) — per-position q/settle state, the v6.26.6 settle-by-laps + dip-probe channel
