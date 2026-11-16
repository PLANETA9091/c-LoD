# PARITY RUN 481 (run 43956152206, sha ee22016) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 67.189 | 74.282 | 31.400 | 218 | FAIL |
| 01_aerial_west | 56.696 | 69.006 | 24.849 | 250 | FAIL |
| 02_aerial_south | 49.272 | 70.016 | 25.118 | 253 | FAIL |
| 03_aerial_north | 49.372 | 69.352 | 21.145 | 253 | FAIL |
| 04_aerial_east | 53.532 | 69.450 | 24.200 | 249 | FAIL |
| 05_oblique | 80.738 | 95.039 | 4.432 | 248 | FAIL |
| 06_ground_west | 37.796 | 67.537 | 16.797 | 251 | FAIL |
| 07_far_north | 85.535 | 91.581 | 8.058 | 249 | FAIL |
| 08_back_inside | 44.973 | 65.883 | 33.692 | 231 | FAIL |
| 09_after_mesh | 33.059 | 46.170 | 50.704 | 237 | FAIL |
| 10_marker_ray | 40.660 | 56.856 | 36.227 | 248 | FAIL |
| 11_topdown | 22.961 | 66.570 | 8.710 | 216 | FAIL |
| 12_far_north_re | 85.535 | 91.578 | 8.063 | 246 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-481/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-481/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-481/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-481/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-481/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-481/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-481/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-481/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-481/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-481/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-481/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-481/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-481/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-481/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
- view probe: [view_probe.md](parity-run-481/view_probe.md) — per-position q/settle state, the v6.26.6 settle-by-laps + dip-probe channel
