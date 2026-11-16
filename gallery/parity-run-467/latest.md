# PARITY RUN 467 (run 43903072188, sha b5bcff3) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 77.581 | 82.560 | 20.878 | 204 | FAIL |
| 01_aerial_west | 61.752 | 77.418 | 18.168 | 249 | FAIL |
| 02_aerial_south | 47.823 | 70.204 | 28.030 | 253 | FAIL |
| 03_aerial_north | 51.420 | 72.197 | 19.193 | 249 | FAIL |
| 04_aerial_east | 51.466 | 67.309 | 29.632 | 254 | FAIL |
| 05_oblique | 77.304 | 85.990 | 17.148 | 222 | FAIL |
| 06_ground_west | 45.181 | 77.004 | 16.900 | 250 | FAIL |
| 07_far_north | 85.271 | 90.252 | 8.203 | 248 | FAIL |
| 08_back_inside | 50.098 | 67.831 | 31.022 | 231 | FAIL |
| 09_after_mesh | 38.777 | 54.184 | 40.119 | 236 | FAIL |
| 10_marker_ray | 59.232 | 75.438 | 19.988 | 250 | FAIL |
| 11_topdown | 28.296 | 76.887 | 6.063 | 216 | FAIL |
| 12_far_north_re | 85.278 | 90.246 | 8.278 | 246 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-467/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-467/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-467/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-467/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-467/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-467/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-467/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-467/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-467/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-467/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-467/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-467/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-467/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-467/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
- view probe: [view_probe.md](parity-run-467/view_probe.md) — per-position q/settle state, the v6.26.6 settle-by-laps + dip-probe channel
