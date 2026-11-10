# PARITY RUN 360 (run 42898137463, sha b037d69) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 90.316 | 93.732 | 5.256 | 210 | FAIL |
| 01_aerial_west | 56.225 | 67.149 | 23.785 | 249 | FAIL |
| 02_aerial_south | 65.725 | 82.888 | 10.314 | 253 | FAIL |
| 03_aerial_north | 54.687 | 72.390 | 17.753 | 253 | FAIL |
| 04_aerial_east | 59.089 | 68.131 | 28.957 | 254 | FAIL |
| 05_oblique | 89.113 | 93.920 | 4.817 | 172 | FAIL |
| 06_ground_west | 50.279 | 71.958 | 19.820 | 249 | FAIL |
| 07_far_north | 85.479 | 90.277 | 8.106 | 248 | FAIL |
| 08_back_inside | 59.737 | 75.230 | 22.090 | 237 | FAIL |
| 09_after_mesh | 51.827 | 62.402 | 28.903 | 233 | FAIL |
| 10_marker_ray | 52.985 | 66.250 | 31.188 | 236 | FAIL |
| 11_topdown | 31.688 | 83.390 | 4.874 | 223 | FAIL |
| 12_far_north_re | 85.479 | 90.274 | 8.115 | 245 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-360/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-360/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-360/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-360/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-360/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-360/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-360/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-360/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-360/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-360/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-360/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-360/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-360/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-360/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
- view probe: [view_probe.md](parity-run-360/view_probe.md) — per-position q/settle state, the v6.26.6 settle-by-laps + dip-probe channel
