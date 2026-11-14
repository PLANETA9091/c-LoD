# PARITY RUN 428 (run 43598118713, sha 1db69d4) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 79.059 | 83.649 | 18.343 | 210 | FAIL |
| 01_aerial_west | 58.027 | 71.953 | 21.144 | 250 | FAIL |
| 02_aerial_south | 49.120 | 70.214 | 27.979 | 253 | FAIL |
| 03_aerial_north | 55.169 | 77.633 | 14.630 | 253 | FAIL |
| 04_aerial_east | 51.932 | 65.936 | 34.397 | 254 | FAIL |
| 05_oblique | 86.570 | 95.743 | 2.996 | 169 | FAIL |
| 06_ground_west | 34.099 | 68.354 | 24.627 | 247 | FAIL |
| 07_far_north | 85.467 | 90.261 | 8.097 | 248 | FAIL |
| 08_back_inside | 57.134 | 73.501 | 24.406 | 239 | FAIL |
| 09_after_mesh | 50.857 | 60.345 | 34.039 | 236 | FAIL |
| 10_marker_ray | 41.404 | 55.655 | 38.591 | 238 | FAIL |
| 11_topdown | 27.616 | 69.969 | 8.386 | 216 | FAIL |
| 12_far_north_re | 85.317 | 90.245 | 8.277 | 246 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-428/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-428/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-428/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-428/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-428/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-428/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-428/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-428/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-428/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-428/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-428/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-428/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-428/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-428/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
- view probe: [view_probe.md](parity-run-428/view_probe.md) — per-position q/settle state, the v6.26.6 settle-by-laps + dip-probe channel
