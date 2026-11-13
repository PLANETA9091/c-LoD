# PARITY RUN 392 (run 43398845576, sha 79a4c55) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 91.707 | 97.352 | 1.651 | 201 | FAIL |
| 01_aerial_west | 57.672 | 72.487 | 20.452 | 251 | FAIL |
| 02_aerial_south | 66.434 | 85.048 | 9.749 | 253 | FAIL |
| 03_aerial_north | 56.809 | 76.152 | 14.419 | 253 | FAIL |
| 04_aerial_east | 67.985 | 78.472 | 17.016 | 254 | FAIL |
| 05_oblique | 80.512 | 90.153 | 9.231 | 192 | FAIL |
| 06_ground_west | 27.318 | 61.521 | 27.298 | 249 | FAIL |
| 07_far_north | 85.462 | 90.247 | 8.129 | 249 | FAIL |
| 08_back_inside | 57.844 | 74.687 | 24.274 | 237 | FAIL |
| 09_after_mesh | 47.229 | 56.592 | 36.024 | 233 | FAIL |
| 10_marker_ray | 50.048 | 63.194 | 31.311 | 236 | FAIL |
| 11_topdown | 27.394 | 70.484 | 8.156 | 223 | FAIL |
| 12_far_north_re | 85.462 | 90.251 | 8.126 | 245 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-392/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-392/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-392/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-392/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-392/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-392/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-392/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-392/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-392/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-392/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-392/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-392/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-392/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-392/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
- view probe: [view_probe.md](parity-run-392/view_probe.md) — per-position q/settle state, the v6.26.6 settle-by-laps + dip-probe channel
