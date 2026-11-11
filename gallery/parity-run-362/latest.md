# PARITY RUN 362 (run 42977427480, sha a2b9b73) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 90.433 | 93.851 | 5.079 | 210 | FAIL |
| 01_aerial_west | 58.177 | 69.286 | 22.547 | 249 | FAIL |
| 02_aerial_south | 65.515 | 82.755 | 10.274 | 253 | FAIL |
| 03_aerial_north | 55.592 | 73.221 | 16.752 | 253 | FAIL |
| 04_aerial_east | 59.121 | 68.041 | 28.967 | 254 | FAIL |
| 05_oblique | 92.347 | 97.135 | 1.758 | 169 | FAIL |
| 06_ground_west | 51.058 | 74.059 | 19.673 | 249 | FAIL |
| 07_far_north | 85.561 | 90.425 | 8.048 | 248 | FAIL |
| 08_back_inside | 60.261 | 76.644 | 21.362 | 237 | FAIL |
| 09_after_mesh | 53.042 | 65.156 | 24.781 | 233 | FAIL |
| 10_marker_ray | 51.229 | 64.341 | 34.074 | 238 | FAIL |
| 11_topdown | 32.119 | 85.468 | 4.427 | 223 | FAIL |
| 12_far_north_re | 85.561 | 90.420 | 8.058 | 245 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-362/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-362/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-362/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-362/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-362/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-362/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-362/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-362/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-362/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-362/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-362/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-362/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-362/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-362/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
- view probe: [view_probe.md](parity-run-362/view_probe.md) — per-position q/settle state, the v6.26.6 settle-by-laps + dip-probe channel
