# PARITY RUN 463 (run 43886024492, sha 1d1c3d4) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 77.314 | 82.252 | 21.483 | 204 | FAIL |
| 01_aerial_west | 57.729 | 72.621 | 20.607 | 249 | FAIL |
| 02_aerial_south | 50.134 | 72.711 | 25.699 | 253 | FAIL |
| 03_aerial_north | 54.715 | 76.946 | 17.758 | 253 | FAIL |
| 04_aerial_east | 49.477 | 64.612 | 36.360 | 254 | FAIL |
| 05_oblique | 85.844 | 93.364 | 5.378 | 177 | FAIL |
| 06_ground_west | 33.942 | 68.879 | 23.053 | 250 | FAIL |
| 07_far_north | 85.269 | 90.236 | 8.282 | 249 | FAIL |
| 08_back_inside | 53.716 | 70.835 | 29.611 | 238 | FAIL |
| 09_after_mesh | 45.854 | 61.588 | 35.318 | 237 | FAIL |
| 10_marker_ray | 46.235 | 60.493 | 40.311 | 238 | FAIL |
| 11_topdown | 30.505 | 81.256 | 5.013 | 223 | FAIL |
| 12_far_north_re | 85.269 | 90.233 | 8.290 | 246 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-463/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-463/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-463/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-463/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-463/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-463/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-463/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-463/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-463/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-463/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-463/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-463/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-463/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-463/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
- view probe: [view_probe.md](parity-run-463/view_probe.md) — per-position q/settle state, the v6.26.6 settle-by-laps + dip-probe channel
