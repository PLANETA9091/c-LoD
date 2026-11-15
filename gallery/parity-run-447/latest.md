# PARITY RUN 447 (run 43703052408, sha f49f26c) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 77.091 | 82.238 | 20.658 | 204 | FAIL |
| 01_aerial_west | 57.087 | 72.373 | 21.896 | 249 | FAIL |
| 02_aerial_south | 50.722 | 73.701 | 21.334 | 253 | FAIL |
| 03_aerial_north | 56.012 | 79.029 | 14.514 | 253 | FAIL |
| 04_aerial_east | 50.921 | 66.224 | 29.236 | 254 | FAIL |
| 05_oblique | 86.437 | 93.466 | 4.982 | 172 | FAIL |
| 06_ground_west | 31.967 | 67.799 | 23.223 | 247 | FAIL |
| 07_far_north | 85.394 | 90.261 | 8.099 | 248 | FAIL |
| 08_back_inside | 51.551 | 68.049 | 31.284 | 237 | FAIL |
| 09_after_mesh | 40.607 | 52.800 | 40.595 | 233 | FAIL |
| 10_marker_ray | 37.487 | 55.108 | 42.908 | 236 | FAIL |
| 11_topdown | 28.039 | 72.806 | 7.338 | 223 | FAIL |
| 12_far_north_re | 85.394 | 90.261 | 8.097 | 245 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-447/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-447/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-447/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-447/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-447/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-447/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-447/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-447/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-447/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-447/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-447/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-447/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-447/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-447/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
- view probe: [view_probe.md](parity-run-447/view_probe.md) — per-position q/settle state, the v6.26.6 settle-by-laps + dip-probe channel
