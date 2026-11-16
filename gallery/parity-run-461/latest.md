# PARITY RUN 461 (run 43883915618, sha 9a2a713) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 77.635 | 82.576 | 21.000 | 204 | FAIL |
| 01_aerial_west | 58.739 | 73.502 | 20.067 | 254 | FAIL |
| 02_aerial_south | 50.563 | 72.367 | 26.749 | 253 | FAIL |
| 03_aerial_north | 51.914 | 72.889 | 19.434 | 253 | FAIL |
| 04_aerial_east | 50.820 | 65.837 | 35.209 | 254 | FAIL |
| 05_oblique | 88.055 | 96.080 | 3.188 | 177 | FAIL |
| 06_ground_west | 33.373 | 68.035 | 23.150 | 249 | FAIL |
| 07_far_north | 85.278 | 90.243 | 8.271 | 249 | FAIL |
| 08_back_inside | 59.848 | 75.897 | 23.458 | 238 | FAIL |
| 09_after_mesh | 50.316 | 64.628 | 29.844 | 231 | FAIL |
| 10_marker_ray | 43.986 | 59.545 | 40.316 | 238 | FAIL |
| 11_topdown | 29.304 | 76.985 | 6.132 | 216 | FAIL |
| 12_far_north_re | 85.278 | 90.243 | 8.276 | 246 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-461/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-461/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-461/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-461/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-461/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-461/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-461/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-461/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-461/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-461/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-461/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-461/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-461/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-461/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
- view probe: [view_probe.md](parity-run-461/view_probe.md) — per-position q/settle state, the v6.26.6 settle-by-laps + dip-probe channel
