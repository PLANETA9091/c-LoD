# PARITY RUN 390 (run 43384983708, sha cfbab44) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 91.978 | 97.204 | 1.757 | 210 | FAIL |
| 01_aerial_west | 62.505 | 79.912 | 18.037 | 248 | FAIL |
| 02_aerial_south | 66.808 | 86.039 | 9.116 | 253 | FAIL |
| 03_aerial_north | 57.250 | 77.579 | 13.943 | 253 | FAIL |
| 04_aerial_east | 67.600 | 78.502 | 17.032 | 254 | FAIL |
| 05_oblique | 85.327 | 92.763 | 8.665 | 222 | FAIL |
| 06_ground_west | 32.349 | 66.516 | 25.200 | 247 | FAIL |
| 07_far_north | 85.461 | 90.246 | 8.124 | 248 | FAIL |
| 08_back_inside | 56.349 | 73.583 | 24.987 | 239 | FAIL |
| 09_after_mesh | 46.443 | 56.012 | 35.766 | 233 | FAIL |
| 10_marker_ray | 47.539 | 60.682 | 33.657 | 236 | FAIL |
| 11_topdown | 27.205 | 69.746 | 8.292 | 223 | FAIL |
| 12_far_north_re | 85.462 | 90.245 | 8.130 | 245 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-390/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-390/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-390/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-390/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-390/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-390/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-390/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-390/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-390/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-390/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-390/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-390/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-390/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-390/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
- view probe: [view_probe.md](parity-run-390/view_probe.md) — per-position q/settle state, the v6.26.6 settle-by-laps + dip-probe channel
