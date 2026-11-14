# PARITY RUN 419 (run 43554961127, sha 0748402) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 79.444 | 84.504 | 18.059 | 210 | FAIL |
| 01_aerial_west | 58.268 | 72.931 | 20.797 | 248 | FAIL |
| 02_aerial_south | 50.933 | 72.273 | 20.855 | 253 | FAIL |
| 03_aerial_north | 54.028 | 74.718 | 16.226 | 254 | FAIL |
| 04_aerial_east | 53.032 | 65.946 | 28.697 | 254 | FAIL |
| 05_oblique | 85.581 | 95.334 | 3.410 | 171 | FAIL |
| 06_ground_west | 26.066 | 60.435 | 26.986 | 249 | FAIL |
| 07_far_north | 85.426 | 90.378 | 8.233 | 249 | FAIL |
| 08_back_inside | 58.189 | 75.370 | 23.778 | 238 | FAIL |
| 09_after_mesh | 48.856 | 58.391 | 35.819 | 237 | FAIL |
| 10_marker_ray | 39.770 | 53.806 | 40.790 | 238 | FAIL |
| 11_topdown | 25.938 | 64.824 | 9.427 | 215 | FAIL |
| 12_far_north_re | 85.426 | 90.380 | 8.238 | 246 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-419/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-419/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-419/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-419/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-419/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-419/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-419/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-419/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-419/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-419/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-419/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-419/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-419/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-419/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
- view probe: [view_probe.md](parity-run-419/view_probe.md) — per-position q/settle state, the v6.26.6 settle-by-laps + dip-probe channel
