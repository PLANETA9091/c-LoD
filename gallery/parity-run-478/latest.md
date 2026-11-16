# PARITY RUN 478 (run 43938927827, sha 47f14eb) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 66.512 | 74.543 | 31.478 | 241 | FAIL |
| 01_aerial_west | 46.253 | 57.739 | 38.681 | 250 | FAIL |
| 02_aerial_south | 38.158 | 54.944 | 41.695 | 253 | FAIL |
| 03_aerial_north | 49.127 | 68.401 | 22.926 | 253 | FAIL |
| 04_aerial_east | 54.004 | 68.850 | 25.344 | 254 | FAIL |
| 05_oblique | 79.577 | 93.934 | 5.102 | 248 | FAIL |
| 06_ground_west | 29.030 | 65.800 | 21.859 | 249 | FAIL |
| 07_far_north | 85.447 | 90.393 | 8.231 | 249 | FAIL |
| 08_back_inside | 46.240 | 66.130 | 32.424 | 238 | FAIL |
| 09_after_mesh | 34.904 | 48.958 | 47.043 | 231 | FAIL |
| 10_marker_ray | 40.245 | 60.658 | 36.257 | 245 | FAIL |
| 11_topdown | 24.901 | 71.501 | 7.155 | 216 | FAIL |
| 12_far_north_re | 85.447 | 90.390 | 8.241 | 246 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-478/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-478/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-478/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-478/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-478/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-478/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-478/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-478/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-478/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-478/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-478/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-478/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-478/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-478/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
- view probe: [view_probe.md](parity-run-478/view_probe.md) — per-position q/settle state, the v6.26.6 settle-by-laps + dip-probe channel
