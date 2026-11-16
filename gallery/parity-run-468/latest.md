# PARITY RUN 468 (run 43905228620, sha ec6d951) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 77.220 | 82.478 | 21.095 | 218 | FAIL |
| 01_aerial_west | 62.396 | 79.031 | 17.714 | 248 | FAIL |
| 02_aerial_south | 49.213 | 72.408 | 25.682 | 253 | FAIL |
| 03_aerial_north | 52.023 | 73.024 | 18.798 | 253 | FAIL |
| 04_aerial_east | 53.121 | 69.350 | 28.031 | 254 | FAIL |
| 05_oblique | 76.600 | 84.795 | 17.355 | 222 | FAIL |
| 06_ground_west | 33.361 | 67.410 | 23.069 | 249 | FAIL |
| 07_far_north | 85.238 | 90.296 | 8.268 | 249 | FAIL |
| 08_back_inside | 51.386 | 69.327 | 26.517 | 226 | FAIL |
| 09_after_mesh | 40.367 | 54.540 | 39.577 | 237 | FAIL |
| 10_marker_ray | 47.794 | 65.900 | 29.848 | 238 | FAIL |
| 11_topdown | 27.637 | 75.309 | 6.330 | 216 | FAIL |
| 12_far_north_re | 85.238 | 90.293 | 8.283 | 246 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-468/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-468/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-468/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-468/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-468/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-468/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-468/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-468/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-468/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-468/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-468/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-468/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-468/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-468/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
- view probe: [view_probe.md](parity-run-468/view_probe.md) — per-position q/settle state, the v6.26.6 settle-by-laps + dip-probe channel
