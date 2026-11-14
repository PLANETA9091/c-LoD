# PARITY RUN 424 (run 43580206454, sha 60c22d5) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 79.316 | 84.394 | 18.125 | 210 | FAIL |
| 01_aerial_west | 60.225 | 74.821 | 19.608 | 250 | FAIL |
| 02_aerial_south | 51.022 | 73.262 | 20.860 | 253 | FAIL |
| 03_aerial_north | 53.427 | 74.088 | 16.228 | 253 | FAIL |
| 04_aerial_east | 51.775 | 65.077 | 29.439 | 254 | FAIL |
| 05_oblique | 90.056 | 98.189 | 1.041 | 169 | FAIL |
| 06_ground_west | 29.567 | 63.812 | 25.327 | 248 | FAIL |
| 07_far_north | 85.464 | 90.261 | 8.110 | 248 | FAIL |
| 08_back_inside | 56.059 | 72.140 | 25.019 | 239 | FAIL |
| 09_after_mesh | 49.802 | 59.929 | 34.195 | 236 | FAIL |
| 10_marker_ray | 41.350 | 55.847 | 38.292 | 238 | FAIL |
| 11_topdown | 27.701 | 70.343 | 8.267 | 216 | FAIL |
| 12_far_north_re | 85.317 | 90.243 | 8.284 | 246 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-424/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-424/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-424/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-424/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-424/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-424/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-424/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-424/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-424/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-424/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-424/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-424/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-424/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-424/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
- view probe: [view_probe.md](parity-run-424/view_probe.md) — per-position q/settle state, the v6.26.6 settle-by-laps + dip-probe channel
