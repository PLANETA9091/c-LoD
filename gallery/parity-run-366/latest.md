# PARITY RUN 366 (run 42983129404, sha af29a71) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 90.498 | 93.909 | 5.015 | 210 | FAIL |
| 01_aerial_west | 56.136 | 66.681 | 24.299 | 249 | FAIL |
| 02_aerial_south | 65.488 | 81.964 | 10.825 | 253 | FAIL |
| 03_aerial_north | 55.134 | 72.737 | 17.148 | 253 | FAIL |
| 04_aerial_east | 59.401 | 67.573 | 29.158 | 254 | FAIL |
| 05_oblique | 89.874 | 94.795 | 4.057 | 172 | FAIL |
| 06_ground_west | 49.966 | 71.474 | 19.732 | 250 | FAIL |
| 07_far_north | 85.582 | 90.424 | 8.071 | 248 | FAIL |
| 08_back_inside | 58.527 | 74.262 | 22.817 | 239 | FAIL |
| 09_after_mesh | 50.303 | 60.599 | 28.423 | 227 | FAIL |
| 10_marker_ray | 52.036 | 63.882 | 34.937 | 236 | FAIL |
| 11_topdown | 31.643 | 83.346 | 4.848 | 223 | FAIL |
| 12_far_north_re | 85.630 | 90.417 | 8.055 | 245 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-366/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-366/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-366/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-366/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-366/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-366/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-366/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-366/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-366/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-366/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-366/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-366/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-366/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-366/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
- view probe: [view_probe.md](parity-run-366/view_probe.md) — per-position q/settle state, the v6.26.6 settle-by-laps + dip-probe channel
