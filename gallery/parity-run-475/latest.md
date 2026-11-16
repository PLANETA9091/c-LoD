# PARITY RUN 475 (run 43925174795, sha 5ea03b2) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 67.778 | 75.250 | 30.542 | 212 | FAIL |
| 01_aerial_west | 49.098 | 61.012 | 33.105 | 250 | FAIL |
| 02_aerial_south | 42.143 | 60.419 | 37.614 | 253 | FAIL |
| 03_aerial_north | 49.888 | 69.999 | 21.729 | 253 | FAIL |
| 04_aerial_east | 51.590 | 67.044 | 28.080 | 254 | FAIL |
| 05_oblique | 76.848 | 90.462 | 6.511 | 222 | FAIL |
| 06_ground_west | 37.437 | 67.913 | 18.533 | 249 | FAIL |
| 07_far_north | 85.317 | 90.242 | 8.282 | 249 | FAIL |
| 08_back_inside | 44.203 | 64.883 | 34.698 | 238 | FAIL |
| 09_after_mesh | 32.371 | 46.056 | 50.499 | 231 | FAIL |
| 10_marker_ray | 40.490 | 59.146 | 35.912 | 250 | FAIL |
| 11_topdown | 23.035 | 67.222 | 8.217 | 222 | FAIL |
| 12_far_north_re | 85.317 | 90.238 | 8.285 | 246 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-475/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-475/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-475/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-475/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-475/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-475/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-475/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-475/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-475/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-475/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-475/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-475/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-475/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-475/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
- view probe: [view_probe.md](parity-run-475/view_probe.md) — per-position q/settle state, the v6.26.6 settle-by-laps + dip-probe channel
