# PARITY RUN 422 (run 43570219403, sha c93d0df) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 79.423 | 84.485 | 17.956 | 205 | FAIL |
| 01_aerial_west | 60.687 | 75.858 | 19.236 | 248 | FAIL |
| 02_aerial_south | 49.903 | 71.703 | 26.949 | 253 | FAIL |
| 03_aerial_north | 54.078 | 75.094 | 15.749 | 253 | FAIL |
| 04_aerial_east | 52.071 | 65.261 | 34.672 | 255 | FAIL |
| 05_oblique | 89.834 | 97.985 | 0.960 | 169 | FAIL |
| 06_ground_west | 45.535 | 77.029 | 18.733 | 249 | FAIL |
| 07_far_north | 85.292 | 90.238 | 8.262 | 249 | FAIL |
| 08_back_inside | 55.824 | 73.020 | 26.347 | 238 | FAIL |
| 09_after_mesh | 45.836 | 55.426 | 38.314 | 237 | FAIL |
| 10_marker_ray | 40.632 | 54.853 | 38.256 | 238 | FAIL |
| 11_topdown | 25.994 | 66.414 | 8.945 | 216 | FAIL |
| 12_far_north_re | 85.299 | 90.235 | 8.296 | 246 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-422/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-422/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-422/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-422/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-422/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-422/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-422/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-422/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-422/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-422/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-422/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-422/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-422/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-422/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
- view probe: [view_probe.md](parity-run-422/view_probe.md) — per-position q/settle state, the v6.26.6 settle-by-laps + dip-probe channel
