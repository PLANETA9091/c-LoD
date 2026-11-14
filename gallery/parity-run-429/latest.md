# PARITY RUN 429 (run 43603042292, sha 89fae4f) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 79.328 | 84.372 | 18.246 | 207 | FAIL |
| 01_aerial_west | 61.967 | 78.039 | 18.464 | 251 | FAIL |
| 02_aerial_south | 50.576 | 72.292 | 21.233 | 253 | FAIL |
| 03_aerial_north | 55.621 | 78.726 | 14.773 | 253 | FAIL |
| 04_aerial_east | 52.143 | 66.453 | 29.768 | 254 | FAIL |
| 05_oblique | 92.849 | 98.944 | 0.376 | 169 | FAIL |
| 06_ground_west | 38.682 | 71.445 | 24.014 | 248 | FAIL |
| 07_far_north | 85.392 | 90.250 | 8.112 | 248 | FAIL |
| 08_back_inside | 57.496 | 74.758 | 24.481 | 239 | FAIL |
| 09_after_mesh | 51.057 | 64.142 | 29.795 | 233 | FAIL |
| 10_marker_ray | 49.161 | 64.292 | 31.155 | 236 | FAIL |
| 11_topdown | 28.691 | 75.166 | 6.624 | 212 | FAIL |
| 12_far_north_re | 85.392 | 90.254 | 8.112 | 245 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-429/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-429/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-429/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-429/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-429/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-429/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-429/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-429/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-429/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-429/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-429/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-429/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-429/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-429/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
- view probe: [view_probe.md](parity-run-429/view_probe.md) — per-position q/settle state, the v6.26.6 settle-by-laps + dip-probe channel
