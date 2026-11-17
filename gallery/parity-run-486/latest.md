# PARITY RUN 486 (run 44061200730, sha 62fe1d7) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 66.566 | 76.002 | 27.264 | 218 | FAIL |
| 01_aerial_west | 48.749 | 57.539 | 36.974 | 251 | FAIL |
| 02_aerial_south | 43.964 | 61.084 | 33.124 | 253 | FAIL |
| 03_aerial_north | 45.533 | 59.133 | 27.421 | 255 | FAIL |
| 04_aerial_east | 50.529 | 64.644 | 26.444 | 254 | FAIL |
| 05_oblique | 64.526 | 83.824 | 11.590 | 222 | FAIL |
| 06_ground_west | 45.209 | 58.824 | 21.654 | 251 | FAIL |
| 07_far_north | 85.486 | 90.396 | 8.226 | 249 | FAIL |
| 08_back_inside | 40.556 | 62.135 | 34.053 | 227 | FAIL |
| 09_after_mesh | 29.570 | 44.987 | 53.122 | 232 | FAIL |
| 10_marker_ray | 38.873 | 50.922 | 37.257 | 251 | FAIL |
| 11_topdown | 20.025 | 58.959 | 10.894 | 223 | FAIL |
| 12_far_north_re | 85.486 | 90.390 | 8.245 | 246 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-486/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-486/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-486/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-486/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-486/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-486/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-486/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-486/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-486/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-486/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-486/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-486/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-486/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-486/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
- view probe: [view_probe.md](parity-run-486/view_probe.md) — per-position q/settle state, the v6.26.6 settle-by-laps + dip-probe channel
