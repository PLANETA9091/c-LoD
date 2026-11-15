# PARITY RUN 439 (run 43653521855, sha 0d8cb0f) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 79.201 | 84.045 | 18.301 | 209 | FAIL |
| 01_aerial_west | 57.954 | 72.350 | 20.955 | 249 | FAIL |
| 02_aerial_south | 49.285 | 70.458 | 27.633 | 253 | FAIL |
| 03_aerial_north | 54.343 | 74.984 | 15.936 | 253 | FAIL |
| 04_aerial_east | 51.899 | 65.040 | 34.723 | 254 | FAIL |
| 05_oblique | 85.204 | 94.109 | 4.320 | 169 | FAIL |
| 06_ground_west | 49.150 | 80.572 | 13.003 | 250 | FAIL |
| 07_far_north | 85.394 | 90.259 | 8.113 | 248 | FAIL |
| 08_back_inside | 59.115 | 76.361 | 23.147 | 239 | FAIL |
| 09_after_mesh | 48.254 | 57.975 | 35.323 | 233 | FAIL |
| 10_marker_ray | 40.856 | 54.708 | 38.297 | 236 | FAIL |
| 11_topdown | 25.987 | 65.816 | 9.233 | 217 | FAIL |
| 12_far_north_re | 85.394 | 90.262 | 8.106 | 245 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-439/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-439/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-439/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-439/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-439/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-439/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-439/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-439/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-439/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-439/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-439/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-439/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-439/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-439/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
- view probe: [view_probe.md](parity-run-439/view_probe.md) — per-position q/settle state, the v6.26.6 settle-by-laps + dip-probe channel
