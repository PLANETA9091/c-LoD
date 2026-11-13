# PARITY RUN 406 (run 43512539193, sha 6b7888b) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 79.109 | 84.253 | 18.210 | 209 | FAIL |
| 01_aerial_west | 58.204 | 72.737 | 20.427 | 254 | FAIL |
| 02_aerial_south | 49.787 | 71.948 | 26.583 | 253 | FAIL |
| 03_aerial_north | 54.752 | 76.236 | 14.990 | 253 | FAIL |
| 04_aerial_east | 51.552 | 65.169 | 34.615 | 254 | FAIL |
| 05_oblique | 85.518 | 94.814 | 3.797 | 169 | FAIL |
| 06_ground_west | 33.882 | 67.766 | 25.083 | 249 | FAIL |
| 07_far_north | 85.537 | 91.663 | 7.937 | 249 | FAIL |
| 08_back_inside | 56.732 | 74.170 | 24.606 | 238 | FAIL |
| 09_after_mesh | 48.064 | 58.345 | 35.283 | 233 | FAIL |
| 10_marker_ray | 44.848 | 57.665 | 35.294 | 238 | FAIL |
| 11_topdown | 27.212 | 68.978 | 8.317 | 216 | FAIL |
| 12_far_north_re | 85.537 | 91.661 | 7.942 | 246 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-406/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-406/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-406/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-406/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-406/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-406/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-406/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-406/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-406/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-406/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-406/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-406/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-406/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-406/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
- view probe: [view_probe.md](parity-run-406/view_probe.md) — per-position q/settle state, the v6.26.6 settle-by-laps + dip-probe channel
