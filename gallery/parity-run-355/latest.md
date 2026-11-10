# PARITY RUN 355 (run 42880170656, sha 22c9b2f) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 87.314 | 89.809 | 6.009 | 210 | FAIL |
| 01_aerial_west | 55.436 | 65.098 | 24.400 | 249 | FAIL |
| 02_aerial_south | 56.469 | 73.036 | 15.929 | 249 | FAIL |
| 03_aerial_north | 51.864 | 65.685 | 21.685 | 249 | FAIL |
| 04_aerial_east | 61.771 | 68.195 | 18.984 | 250 | FAIL |
| 05_oblique | 86.373 | 91.022 | 7.614 | 190 | FAIL |
| 06_ground_west | 38.455 | 66.694 | 16.889 | 251 | FAIL |
| 07_far_north | 85.561 | 90.420 | 8.066 | 248 | FAIL |
| 08_back_inside | 60.389 | 76.263 | 21.290 | 237 | FAIL |
| 09_after_mesh | 52.879 | 62.997 | 28.402 | 233 | FAIL |
| 10_marker_ray | 51.358 | 64.526 | 33.592 | 236 | FAIL |
| 11_topdown | 31.652 | 83.410 | 4.850 | 217 | FAIL |
| 12_far_north_re | 85.561 | 90.424 | 8.052 | 245 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-355/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-355/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-355/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-355/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-355/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-355/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-355/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-355/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-355/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-355/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-355/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-355/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-355/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-355/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
- view probe: [view_probe.md](parity-run-355/view_probe.md) — per-position q/settle state, the v6.26.6 settle-by-laps + dip-probe channel
