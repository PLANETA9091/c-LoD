# PARITY RUN 393 (run 43405588914, sha e11728a) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 92.531 | 97.470 | 1.573 | 201 | FAIL |
| 01_aerial_west | 57.852 | 72.427 | 20.603 | 251 | FAIL |
| 02_aerial_south | 66.406 | 85.339 | 9.477 | 253 | FAIL |
| 03_aerial_north | 57.191 | 77.063 | 14.211 | 253 | FAIL |
| 04_aerial_east | 68.003 | 78.512 | 17.027 | 254 | FAIL |
| 05_oblique | 78.488 | 88.516 | 11.861 | 222 | FAIL |
| 06_ground_west | 31.874 | 67.194 | 25.576 | 249 | FAIL |
| 07_far_north | 85.410 | 90.322 | 8.103 | 248 | FAIL |
| 08_back_inside | 56.360 | 73.567 | 24.628 | 237 | FAIL |
| 09_after_mesh | 47.321 | 56.882 | 36.522 | 233 | FAIL |
| 10_marker_ray | 51.973 | 63.567 | 30.262 | 236 | FAIL |
| 11_topdown | 27.887 | 71.116 | 7.867 | 217 | FAIL |
| 12_far_north_re | 85.409 | 90.316 | 8.123 | 246 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-393/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-393/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-393/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-393/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-393/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-393/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-393/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-393/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-393/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-393/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-393/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-393/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-393/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-393/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
- view probe: [view_probe.md](parity-run-393/view_probe.md) — per-position q/settle state, the v6.26.6 settle-by-laps + dip-probe channel
