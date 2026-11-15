# PARITY RUN 448 (run 43703199751, sha af7a438) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 77.364 | 82.485 | 19.980 | 209 | FAIL |
| 01_aerial_west | 55.396 | 70.118 | 23.236 | 251 | FAIL |
| 02_aerial_south | 47.909 | 69.745 | 29.272 | 253 | FAIL |
| 03_aerial_north | 54.331 | 76.004 | 15.966 | 253 | FAIL |
| 04_aerial_east | 46.844 | 61.149 | 40.623 | 254 | FAIL |
| 05_oblique | 84.283 | 93.481 | 5.977 | 172 | FAIL |
| 06_ground_west | 28.937 | 65.023 | 23.982 | 248 | FAIL |
| 07_far_north | 85.771 | 91.601 | 7.871 | 248 | FAIL |
| 08_back_inside | 52.495 | 70.168 | 28.719 | 237 | FAIL |
| 09_after_mesh | 41.506 | 51.785 | 42.960 | 236 | FAIL |
| 10_marker_ray | 40.834 | 54.845 | 41.240 | 238 | FAIL |
| 11_topdown | 27.197 | 69.553 | 8.380 | 216 | FAIL |
| 12_far_north_re | 85.535 | 91.580 | 8.062 | 246 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-448/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-448/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-448/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-448/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-448/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-448/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-448/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-448/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-448/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-448/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-448/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-448/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-448/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-448/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
- view probe: [view_probe.md](parity-run-448/view_probe.md) — per-position q/settle state, the v6.26.6 settle-by-laps + dip-probe channel
