# PARITY RUN 368 (run 42995947323, sha 7b59c92) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 90.502 | 93.882 | 4.972 | 210 | FAIL |
| 01_aerial_west | 59.538 | 70.794 | 21.392 | 251 | FAIL |
| 02_aerial_south | 65.725 | 83.178 | 9.885 | 253 | FAIL |
| 03_aerial_north | 53.838 | 72.260 | 16.603 | 253 | FAIL |
| 04_aerial_east | 60.049 | 68.080 | 29.347 | 254 | FAIL |
| 05_oblique | 93.988 | 98.958 | 0.360 | 169 | FAIL |
| 06_ground_west | 35.293 | 64.331 | 25.956 | 249 | FAIL |
| 07_far_north | 85.465 | 90.260 | 8.097 | 248 | FAIL |
| 08_back_inside | 58.540 | 74.377 | 23.037 | 237 | FAIL |
| 09_after_mesh | 49.307 | 59.935 | 28.583 | 233 | FAIL |
| 10_marker_ray | 50.755 | 63.375 | 35.431 | 238 | FAIL |
| 11_topdown | 31.151 | 82.299 | 5.085 | 223 | FAIL |
| 12_far_north_re | 85.464 | 90.260 | 8.098 | 245 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-368/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-368/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-368/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-368/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-368/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-368/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-368/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-368/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-368/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-368/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-368/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-368/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-368/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-368/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
- view probe: [view_probe.md](parity-run-368/view_probe.md) — per-position q/settle state, the v6.26.6 settle-by-laps + dip-probe channel
