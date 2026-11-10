# PARITY RUN 353 (run 42868053369, sha 4549ef2) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 76.301 | 79.089 | 19.779 | 210 | FAIL |
| 01_aerial_west | 48.194 | 57.590 | 32.744 | 249 | FAIL |
| 02_aerial_south | 39.843 | 54.690 | 42.100 | 253 | FAIL |
| 03_aerial_north | 50.776 | 65.248 | 20.714 | 253 | FAIL |
| 04_aerial_east | 47.722 | 54.744 | 39.733 | 254 | FAIL |
| 05_oblique | 82.436 | 87.685 | 9.732 | 250 | FAIL |
| 06_ground_west | 48.573 | 66.283 | 19.215 | 249 | FAIL |
| 07_far_north | 85.392 | 90.256 | 8.110 | 248 | FAIL |
| 08_back_inside | 64.449 | 80.178 | 17.352 | 239 | FAIL |
| 09_after_mesh | 58.464 | 68.922 | 22.052 | 231 | FAIL |
| 10_marker_ray | 53.045 | 63.784 | 35.212 | 236 | FAIL |
| 11_topdown | 31.360 | 82.892 | 4.983 | 212 | FAIL |
| 12_far_north_re | 85.393 | 90.259 | 8.112 | 245 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-353/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-353/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-353/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-353/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-353/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-353/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-353/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-353/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-353/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-353/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-353/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-353/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-353/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-353/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
- view probe: [view_probe.md](parity-run-353/view_probe.md) — per-position q/settle state, the v6.26.6 settle-by-laps + dip-probe channel
