# PARITY RUN 402 (run 43480758986, sha a198140) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 79.696 | 84.757 | 17.832 | 209 | FAIL |
| 01_aerial_west | 61.058 | 77.341 | 18.913 | 251 | FAIL |
| 02_aerial_south | 49.448 | 71.002 | 27.761 | 253 | FAIL |
| 03_aerial_north | 53.886 | 74.955 | 15.929 | 253 | FAIL |
| 04_aerial_east | 52.252 | 65.300 | 34.863 | 254 | FAIL |
| 05_oblique | 89.324 | 97.184 | 1.655 | 169 | FAIL |
| 06_ground_west | 29.869 | 63.725 | 26.578 | 249 | FAIL |
| 07_far_north | 85.468 | 90.382 | 8.254 | 249 | FAIL |
| 08_back_inside | 57.265 | 75.152 | 24.496 | 231 | FAIL |
| 09_after_mesh | 49.722 | 59.176 | 35.656 | 236 | FAIL |
| 10_marker_ray | 42.125 | 55.294 | 38.075 | 238 | FAIL |
| 11_topdown | 26.977 | 67.733 | 9.039 | 222 | FAIL |
| 12_far_north_re | 85.468 | 90.383 | 8.254 | 246 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-402/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-402/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-402/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-402/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-402/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-402/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-402/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-402/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-402/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-402/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-402/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-402/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-402/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-402/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
- view probe: [view_probe.md](parity-run-402/view_probe.md) — per-position q/settle state, the v6.26.6 settle-by-laps + dip-probe channel
