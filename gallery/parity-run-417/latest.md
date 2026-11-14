# PARITY RUN 417 (run 43546720527, sha eaf8a0d) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 79.125 | 84.201 | 18.339 | 209 | FAIL |
| 01_aerial_west | 57.848 | 72.437 | 20.752 | 251 | FAIL |
| 02_aerial_south | 50.286 | 72.020 | 26.362 | 253 | FAIL |
| 03_aerial_north | 53.772 | 75.079 | 15.945 | 249 | FAIL |
| 04_aerial_east | 51.549 | 64.941 | 35.357 | 254 | FAIL |
| 05_oblique | 84.134 | 94.025 | 4.494 | 172 | FAIL |
| 06_ground_west | 26.281 | 60.300 | 27.189 | 249 | FAIL |
| 07_far_north | 85.786 | 91.599 | 7.877 | 248 | FAIL |
| 08_back_inside | 56.615 | 73.863 | 25.001 | 237 | FAIL |
| 09_after_mesh | 48.434 | 58.620 | 34.724 | 233 | FAIL |
| 10_marker_ray | 44.253 | 58.266 | 35.197 | 236 | FAIL |
| 11_topdown | 27.217 | 70.208 | 8.197 | 223 | FAIL |
| 12_far_north_re | 85.786 | 91.602 | 7.860 | 243 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-417/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-417/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-417/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-417/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-417/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-417/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-417/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-417/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-417/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-417/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-417/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-417/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-417/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-417/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
- view probe: [view_probe.md](parity-run-417/view_probe.md) — per-position q/settle state, the v6.26.6 settle-by-laps + dip-probe channel
