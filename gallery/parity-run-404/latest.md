# PARITY RUN 404 (run 43494086594, sha adb1004) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 79.213 | 84.017 | 18.409 | 209 | FAIL |
| 01_aerial_west | 60.162 | 75.544 | 19.363 | 248 | FAIL |
| 02_aerial_south | 51.191 | 73.435 | 21.301 | 253 | FAIL |
| 03_aerial_north | 54.215 | 75.358 | 15.463 | 253 | FAIL |
| 04_aerial_east | 51.597 | 64.962 | 31.108 | 254 | FAIL |
| 05_oblique | 88.714 | 96.774 | 2.134 | 169 | FAIL |
| 06_ground_west | 29.542 | 64.328 | 26.474 | 249 | FAIL |
| 07_far_north | 85.278 | 90.243 | 8.280 | 249 | FAIL |
| 08_back_inside | 57.494 | 74.712 | 24.335 | 238 | FAIL |
| 09_after_mesh | 46.603 | 56.729 | 35.683 | 237 | FAIL |
| 10_marker_ray | 39.498 | 54.339 | 39.536 | 238 | FAIL |
| 11_topdown | 27.256 | 69.434 | 8.210 | 222 | FAIL |
| 12_far_north_re | 85.278 | 90.240 | 8.283 | 246 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-404/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-404/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-404/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-404/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-404/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-404/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-404/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-404/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-404/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-404/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-404/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-404/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-404/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-404/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
- view probe: [view_probe.md](parity-run-404/view_probe.md) — per-position q/settle state, the v6.26.6 settle-by-laps + dip-probe channel
