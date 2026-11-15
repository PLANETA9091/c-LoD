# PARITY RUN 441 (run 43700003006, sha c4823e0) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 79.681 | 84.755 | 17.726 | 209 | FAIL |
| 01_aerial_west | 61.429 | 76.815 | 18.944 | 248 | FAIL |
| 02_aerial_south | 50.352 | 72.729 | 21.391 | 253 | FAIL |
| 03_aerial_north | 53.714 | 74.601 | 15.961 | 253 | FAIL |
| 04_aerial_east | 50.929 | 63.966 | 30.436 | 254 | FAIL |
| 05_oblique | 92.340 | 98.981 | 0.370 | 169 | FAIL |
| 06_ground_west | 33.984 | 66.434 | 27.623 | 249 | FAIL |
| 07_far_north | 85.269 | 90.236 | 8.289 | 249 | FAIL |
| 08_back_inside | 54.396 | 70.354 | 27.613 | 238 | FAIL |
| 09_after_mesh | 46.041 | 55.715 | 38.995 | 237 | FAIL |
| 10_marker_ray | 39.641 | 54.644 | 40.330 | 238 | FAIL |
| 11_topdown | 26.571 | 68.372 | 8.503 | 215 | FAIL |
| 12_far_north_re | 85.269 | 90.236 | 8.282 | 248 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-441/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-441/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-441/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-441/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-441/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-441/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-441/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-441/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-441/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-441/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-441/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-441/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-441/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-441/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
- view probe: [view_probe.md](parity-run-441/view_probe.md) — per-position q/settle state, the v6.26.6 settle-by-laps + dip-probe channel
