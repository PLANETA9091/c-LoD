# PARITY RUN 361 (run 42905584845, sha 2cdc8c1) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 90.500 | 93.913 | 4.909 | 210 | FAIL |
| 01_aerial_west | 56.911 | 67.411 | 23.906 | 249 | FAIL |
| 02_aerial_south | 65.475 | 82.170 | 10.286 | 253 | FAIL |
| 03_aerial_north | 56.821 | 74.900 | 15.484 | 253 | FAIL |
| 04_aerial_east | 59.476 | 67.767 | 28.440 | 254 | FAIL |
| 05_oblique | 91.142 | 95.922 | 2.796 | 193 | FAIL |
| 06_ground_west | 42.340 | 62.910 | 26.613 | 249 | FAIL |
| 07_far_north | 85.477 | 90.319 | 8.094 | 248 | FAIL |
| 08_back_inside | 58.702 | 74.080 | 22.669 | 237 | FAIL |
| 09_after_mesh | 50.257 | 61.294 | 27.032 | 231 | FAIL |
| 10_marker_ray | 52.485 | 62.735 | 36.599 | 236 | FAIL |
| 11_topdown | 31.276 | 82.210 | 5.054 | 222 | FAIL |
| 12_far_north_re | 85.477 | 90.316 | 8.107 | 246 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-361/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-361/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-361/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-361/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-361/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-361/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-361/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-361/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-361/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-361/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-361/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-361/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-361/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-361/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
- view probe: [view_probe.md](parity-run-361/view_probe.md) — per-position q/settle state, the v6.26.6 settle-by-laps + dip-probe channel
