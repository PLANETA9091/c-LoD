# PARITY RUN 405 (run 43503847362, sha b97285d) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 79.504 | 84.643 | 17.773 | 209 | FAIL |
| 01_aerial_west | 62.165 | 78.993 | 18.338 | 248 | FAIL |
| 02_aerial_south | 50.266 | 72.928 | 21.822 | 253 | FAIL |
| 03_aerial_north | 54.112 | 75.611 | 15.849 | 253 | FAIL |
| 04_aerial_east | 51.124 | 65.161 | 30.859 | 254 | FAIL |
| 05_oblique | 91.821 | 98.963 | 0.378 | 169 | FAIL |
| 06_ground_west | 32.065 | 66.453 | 25.930 | 249 | FAIL |
| 07_far_north | 85.767 | 91.812 | 7.800 | 248 | FAIL |
| 08_back_inside | 57.062 | 74.295 | 25.096 | 238 | FAIL |
| 09_after_mesh | 49.907 | 59.804 | 34.543 | 237 | FAIL |
| 10_marker_ray | 41.368 | 56.407 | 38.253 | 238 | FAIL |
| 11_topdown | 28.242 | 72.725 | 7.296 | 216 | FAIL |
| 12_far_north_re | 85.723 | 91.777 | 7.941 | 246 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-405/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-405/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-405/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-405/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-405/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-405/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-405/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-405/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-405/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-405/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-405/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-405/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-405/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-405/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
- view probe: [view_probe.md](parity-run-405/view_probe.md) — per-position q/settle state, the v6.26.6 settle-by-laps + dip-probe channel
