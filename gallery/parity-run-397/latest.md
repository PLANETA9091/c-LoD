# PARITY RUN 397 (run 43458673355, sha c685b2d) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 78.901 | 83.571 | 18.920 | 204 | FAIL |
| 01_aerial_west | 61.852 | 77.819 | 18.490 | 249 | FAIL |
| 02_aerial_south | 49.984 | 71.774 | 26.693 | 253 | FAIL |
| 03_aerial_north | 54.358 | 75.996 | 15.350 | 253 | FAIL |
| 04_aerial_east | 51.117 | 65.006 | 36.338 | 254 | FAIL |
| 05_oblique | 91.961 | 98.959 | 0.378 | 169 | FAIL |
| 06_ground_west | 33.601 | 67.957 | 24.847 | 249 | FAIL |
| 07_far_north | 85.750 | 91.522 | 8.011 | 248 | FAIL |
| 08_back_inside | 55.775 | 72.818 | 25.569 | 237 | FAIL |
| 09_after_mesh | 0.000 | 0.247 | 99.126 | 245 | FAIL |
| 10_marker_ray | 39.973 | 54.506 | 39.288 | 236 | FAIL |
| 11_topdown | 26.292 | 67.191 | 8.990 | 223 | FAIL |
| 12_far_north_re | 85.751 | 91.530 | 7.979 | 243 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-397/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-397/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-397/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-397/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-397/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-397/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-397/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-397/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-397/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-397/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-397/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-397/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-397/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-397/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
- view probe: [view_probe.md](parity-run-397/view_probe.md) — per-position q/settle state, the v6.26.6 settle-by-laps + dip-probe channel
