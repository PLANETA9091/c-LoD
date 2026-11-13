# PARITY RUN 395 (run 43418738209, sha 35e27af) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 79.066 | 84.149 | 18.576 | 209 | FAIL |
| 01_aerial_west | 58.701 | 73.295 | 20.312 | 251 | FAIL |
| 02_aerial_south | 49.796 | 71.158 | 26.934 | 253 | FAIL |
| 03_aerial_north | 53.617 | 74.183 | 16.272 | 249 | FAIL |
| 04_aerial_east | 51.628 | 63.680 | 35.043 | 254 | FAIL |
| 05_oblique | 86.789 | 96.457 | 2.137 | 169 | FAIL |
| 06_ground_west | 26.665 | 60.805 | 26.647 | 248 | FAIL |
| 07_far_north | 85.418 | 90.260 | 8.108 | 248 | FAIL |
| 08_back_inside | 56.013 | 72.824 | 24.969 | 239 | FAIL |
| 09_after_mesh | 47.728 | 57.121 | 36.503 | 234 | FAIL |
| 10_marker_ray | 40.245 | 53.834 | 39.639 | 236 | FAIL |
| 11_topdown | 25.785 | 64.617 | 9.673 | 223 | FAIL |
| 12_far_north_re | 85.419 | 90.260 | 8.109 | 245 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-395/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-395/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-395/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-395/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-395/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-395/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-395/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-395/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-395/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-395/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-395/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-395/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-395/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-395/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
- view probe: [view_probe.md](parity-run-395/view_probe.md) — per-position q/settle state, the v6.26.6 settle-by-laps + dip-probe channel
