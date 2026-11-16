# PARITY RUN 472 (run 43910820613, sha cab5e0e) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 75.812 | 80.885 | 22.181 | 212 | FAIL |
| 01_aerial_west | 57.366 | 73.337 | 19.106 | 249 | FAIL |
| 02_aerial_south | 41.754 | 59.573 | 33.323 | 253 | FAIL |
| 03_aerial_north | 41.113 | 61.825 | 24.835 | 253 | FAIL |
| 04_aerial_east | 41.766 | 61.274 | 33.918 | 254 | FAIL |
| 05_oblique | 67.453 | 83.499 | 19.661 | 223 | FAIL |
| 06_ground_west | 31.509 | 47.511 | 33.574 | 250 | FAIL |
| 07_far_north | 85.270 | 90.233 | 8.292 | 249 | FAIL |
| 08_back_inside | 55.123 | 77.380 | 18.398 | 238 | FAIL |
| 09_after_mesh | 49.652 | 71.099 | 22.136 | 237 | FAIL |
| 10_marker_ray | 44.654 | 64.912 | 28.907 | 251 | FAIL |
| 11_topdown | 19.351 | 59.634 | 9.857 | 209 | FAIL |
| 12_far_north_re | 85.270 | 90.239 | 8.274 | 246 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-472/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-472/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-472/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-472/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-472/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-472/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-472/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-472/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-472/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-472/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-472/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-472/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-472/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-472/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
- view probe: [view_probe.md](parity-run-472/view_probe.md) — per-position q/settle state, the v6.26.6 settle-by-laps + dip-probe channel
