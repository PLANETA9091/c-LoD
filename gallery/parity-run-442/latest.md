# PARITY RUN 442 (run 43700503327, sha 5265bbc) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 79.440 | 84.596 | 18.008 | 209 | FAIL |
| 01_aerial_west | 57.286 | 71.067 | 21.379 | 251 | FAIL |
| 02_aerial_south | 51.466 | 73.030 | 22.105 | 253 | FAIL |
| 03_aerial_north | 54.380 | 75.777 | 15.663 | 253 | FAIL |
| 04_aerial_east | 53.528 | 67.304 | 30.957 | 254 | FAIL |
| 05_oblique | 85.064 | 93.644 | 4.741 | 171 | FAIL |
| 06_ground_west | 30.285 | 65.644 | 26.633 | 249 | FAIL |
| 07_far_north | 85.270 | 90.235 | 8.293 | 249 | FAIL |
| 08_back_inside | 56.751 | 74.173 | 25.422 | 238 | FAIL |
| 09_after_mesh | 47.424 | 56.662 | 37.811 | 236 | FAIL |
| 10_marker_ray | 40.816 | 54.849 | 38.865 | 238 | FAIL |
| 11_topdown | 26.733 | 67.570 | 8.942 | 222 | FAIL |
| 12_far_north_re | 85.270 | 90.238 | 8.288 | 248 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-442/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-442/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-442/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-442/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-442/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-442/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-442/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-442/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-442/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-442/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-442/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-442/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-442/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-442/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
- view probe: [view_probe.md](parity-run-442/view_probe.md) — per-position q/settle state, the v6.26.6 settle-by-laps + dip-probe channel
