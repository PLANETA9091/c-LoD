# PARITY RUN 400 (run 43471448559, sha afffc7d) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 79.482 | 84.632 | 18.136 | 210 | FAIL |
| 01_aerial_west | 57.852 | 72.230 | 20.762 | 248 | FAIL |
| 02_aerial_south | 51.046 | 73.462 | 20.841 | 253 | FAIL |
| 03_aerial_north | 53.826 | 74.505 | 15.931 | 253 | FAIL |
| 04_aerial_east | 51.904 | 66.143 | 29.581 | 254 | FAIL |
| 05_oblique | 84.085 | 93.816 | 4.356 | 169 | FAIL |
| 06_ground_west | 27.907 | 62.970 | 26.582 | 249 | FAIL |
| 07_far_north | 85.659 | 91.752 | 7.999 | 249 | FAIL |
| 08_back_inside | 55.983 | 73.376 | 25.462 | 237 | FAIL |
| 09_after_mesh | 45.865 | 55.507 | 37.365 | 233 | FAIL |
| 10_marker_ray | 40.402 | 55.107 | 38.683 | 238 | FAIL |
| 11_topdown | 26.422 | 67.368 | 8.807 | 223 | FAIL |
| 12_far_north_re | 85.659 | 91.750 | 8.005 | 246 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-400/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-400/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-400/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-400/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-400/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-400/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-400/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-400/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-400/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-400/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-400/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-400/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-400/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-400/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
- view probe: [view_probe.md](parity-run-400/view_probe.md) — per-position q/settle state, the v6.26.6 settle-by-laps + dip-probe channel
