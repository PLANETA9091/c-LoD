# PARITY RUN 358 (run 42892029611, sha 2911fec) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 90.412 | 93.829 | 5.138 | 210 | FAIL |
| 01_aerial_west | 57.729 | 68.340 | 23.075 | 250 | FAIL |
| 02_aerial_south | 64.970 | 81.429 | 11.153 | 253 | FAIL |
| 03_aerial_north | 55.217 | 73.026 | 16.998 | 253 | FAIL |
| 04_aerial_east | 58.974 | 67.264 | 29.574 | 254 | FAIL |
| 05_oblique | 91.900 | 96.737 | 1.819 | 169 | FAIL |
| 06_ground_west | 50.914 | 72.242 | 18.456 | 250 | FAIL |
| 07_far_north | 85.392 | 90.261 | 8.094 | 248 | FAIL |
| 08_back_inside | 61.660 | 77.569 | 20.288 | 239 | FAIL |
| 09_after_mesh | 55.688 | 66.507 | 25.760 | 233 | FAIL |
| 10_marker_ray | 49.762 | 62.861 | 35.542 | 236 | FAIL |
| 11_topdown | 31.366 | 82.285 | 4.968 | 223 | FAIL |
| 12_far_north_re | 85.392 | 90.261 | 8.095 | 245 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-358/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-358/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-358/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-358/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-358/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-358/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-358/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-358/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-358/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-358/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-358/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-358/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-358/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-358/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
- view probe: [view_probe.md](parity-run-358/view_probe.md) — per-position q/settle state, the v6.26.6 settle-by-laps + dip-probe channel
