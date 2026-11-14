# PARITY RUN 437 (run 43643573654, sha 31f6d49) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 78.738 | 83.786 | 19.613 | 210 | FAIL |
| 01_aerial_west | 58.197 | 71.859 | 20.842 | 249 | FAIL |
| 02_aerial_south | 50.560 | 72.246 | 22.569 | 253 | FAIL |
| 03_aerial_north | 53.527 | 74.313 | 15.777 | 253 | FAIL |
| 04_aerial_east | 50.999 | 64.691 | 33.417 | 254 | FAIL |
| 05_oblique | 85.922 | 95.769 | 2.972 | 169 | FAIL |
| 06_ground_west | 25.867 | 58.884 | 28.216 | 249 | FAIL |
| 07_far_north | 85.883 | 91.802 | 7.738 | 248 | FAIL |
| 08_back_inside | 58.708 | 74.847 | 22.911 | 239 | FAIL |
| 09_after_mesh | 50.898 | 59.918 | 33.168 | 233 | FAIL |
| 10_marker_ray | 37.888 | 52.520 | 41.598 | 236 | FAIL |
| 11_topdown | 26.240 | 65.482 | 9.312 | 223 | FAIL |
| 12_far_north_re | 85.884 | 91.800 | 7.743 | 245 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-437/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-437/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-437/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-437/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-437/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-437/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-437/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-437/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-437/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-437/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-437/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-437/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-437/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-437/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
- view probe: [view_probe.md](parity-run-437/view_probe.md) — per-position q/settle state, the v6.26.6 settle-by-laps + dip-probe channel
