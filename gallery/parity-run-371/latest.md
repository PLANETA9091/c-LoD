# PARITY RUN 371 (run 43008416927, sha 6841ea4) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 91.490 | 95.196 | 2.630 | 210 | FAIL |
| 01_aerial_west | 59.633 | 70.610 | 21.674 | 248 | FAIL |
| 02_aerial_south | 64.768 | 81.677 | 11.024 | 253 | FAIL |
| 03_aerial_north | 55.164 | 72.960 | 17.323 | 253 | FAIL |
| 04_aerial_east | 68.864 | 77.927 | 16.430 | 254 | FAIL |
| 05_oblique | 93.953 | 98.961 | 0.360 | 169 | FAIL |
| 06_ground_west | 46.134 | 68.682 | 24.731 | 249 | FAIL |
| 07_far_north | 85.462 | 90.252 | 8.123 | 248 | FAIL |
| 08_back_inside | 58.662 | 74.964 | 22.862 | 237 | FAIL |
| 09_after_mesh | 52.177 | 63.110 | 29.077 | 233 | FAIL |
| 10_marker_ray | 44.197 | 61.124 | 37.071 | 236 | FAIL |
| 11_topdown | 31.278 | 81.890 | 5.120 | 223 | FAIL |
| 12_far_north_re | 85.461 | 90.248 | 8.126 | 245 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-371/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-371/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-371/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-371/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-371/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-371/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-371/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-371/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-371/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-371/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-371/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-371/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-371/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-371/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
- view probe: [view_probe.md](parity-run-371/view_probe.md) — per-position q/settle state, the v6.26.6 settle-by-laps + dip-probe channel
