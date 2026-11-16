# PARITY RUN 465 (run 43898366097, sha ea3ac08) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 76.666 | 82.312 | 21.120 | 204 | FAIL |
| 01_aerial_west | 61.402 | 76.046 | 18.386 | 254 | FAIL |
| 02_aerial_south | 48.214 | 70.507 | 26.945 | 253 | FAIL |
| 03_aerial_north | 52.828 | 73.241 | 18.982 | 253 | FAIL |
| 04_aerial_east | 51.672 | 66.638 | 32.156 | 254 | FAIL |
| 05_oblique | 79.665 | 88.581 | 13.364 | 221 | FAIL |
| 06_ground_west | 29.855 | 64.367 | 24.454 | 249 | FAIL |
| 07_far_north | 85.292 | 90.270 | 8.280 | 249 | FAIL |
| 08_back_inside | 51.175 | 68.834 | 32.049 | 238 | FAIL |
| 09_after_mesh | 39.531 | 54.582 | 41.702 | 237 | FAIL |
| 10_marker_ray | 56.199 | 71.476 | 25.672 | 238 | FAIL |
| 11_topdown | 28.487 | 76.859 | 6.190 | 216 | FAIL |
| 12_far_north_re | 85.292 | 90.275 | 8.278 | 248 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-465/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-465/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-465/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-465/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-465/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-465/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-465/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-465/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-465/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-465/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-465/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-465/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-465/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-465/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
- view probe: [view_probe.md](parity-run-465/view_probe.md) — per-position q/settle state, the v6.26.6 settle-by-laps + dip-probe channel
