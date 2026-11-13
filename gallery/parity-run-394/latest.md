# PARITY RUN 394 (run 43412209932, sha 2448b5b) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 79.557 | 84.608 | 17.958 | 204 | FAIL |
| 01_aerial_west | 61.339 | 77.765 | 18.824 | 251 | FAIL |
| 02_aerial_south | 49.905 | 71.389 | 26.679 | 253 | FAIL |
| 03_aerial_north | 54.324 | 75.729 | 15.538 | 253 | FAIL |
| 04_aerial_east | 51.820 | 64.566 | 34.865 | 254 | FAIL |
| 05_oblique | 91.710 | 98.927 | 0.391 | 169 | FAIL |
| 06_ground_west | 46.879 | 75.803 | 19.308 | 250 | FAIL |
| 07_far_north | 85.408 | 90.254 | 8.116 | 248 | FAIL |
| 08_back_inside | 58.807 | 75.280 | 22.796 | 237 | FAIL |
| 09_after_mesh | 50.903 | 63.198 | 28.707 | 233 | FAIL |
| 10_marker_ray | 41.766 | 58.454 | 37.176 | 236 | FAIL |
| 11_topdown | 27.534 | 71.700 | 7.529 | 223 | FAIL |
| 12_far_north_re | 85.407 | 90.256 | 8.117 | 245 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-394/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-394/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-394/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-394/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-394/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-394/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-394/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-394/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-394/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-394/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-394/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-394/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-394/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-394/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
- view probe: [view_probe.md](parity-run-394/view_probe.md) — per-position q/settle state, the v6.26.6 settle-by-laps + dip-probe channel
