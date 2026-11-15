# PARITY RUN 458 (run 43717235377, sha 38fda5b) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 77.138 | 82.046 | 20.597 | 206 | FAIL |
| 01_aerial_west | 57.608 | 72.822 | 21.851 | 248 | FAIL |
| 02_aerial_south | 48.483 | 70.246 | 28.369 | 253 | FAIL |
| 03_aerial_north | 55.300 | 76.208 | 14.870 | 253 | FAIL |
| 04_aerial_east | 48.664 | 62.298 | 38.241 | 254 | FAIL |
| 05_oblique | 87.630 | 95.631 | 3.034 | 169 | FAIL |
| 06_ground_west | 34.393 | 69.345 | 24.555 | 248 | FAIL |
| 07_far_north | 85.409 | 90.319 | 8.102 | 248 | FAIL |
| 08_back_inside | 53.892 | 71.273 | 27.636 | 237 | FAIL |
| 09_after_mesh | 44.786 | 56.525 | 36.009 | 233 | FAIL |
| 10_marker_ray | 64.999 | 77.055 | 16.913 | 236 | FAIL |
| 11_topdown | 28.409 | 73.444 | 6.999 | 223 | FAIL |
| 12_far_north_re | 85.280 | 90.309 | 8.256 | 246 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-458/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-458/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-458/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-458/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-458/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-458/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-458/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-458/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-458/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-458/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-458/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-458/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-458/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-458/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
- view probe: [view_probe.md](parity-run-458/view_probe.md) — per-position q/settle state, the v6.26.6 settle-by-laps + dip-probe channel
