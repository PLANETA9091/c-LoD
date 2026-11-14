# PARITY RUN 432 (run 43617071324, sha 9f2c0bf) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 79.546 | 84.625 | 18.089 | 209 | FAIL |
| 01_aerial_west | 56.781 | 70.842 | 21.871 | 250 | FAIL |
| 02_aerial_south | 50.309 | 72.463 | 23.079 | 253 | FAIL |
| 03_aerial_north | 55.510 | 77.780 | 14.684 | 253 | FAIL |
| 04_aerial_east | 51.998 | 65.688 | 32.448 | 254 | FAIL |
| 05_oblique | 84.615 | 93.594 | 4.908 | 172 | FAIL |
| 06_ground_west | 26.182 | 61.336 | 27.187 | 249 | FAIL |
| 07_far_north | 85.278 | 90.242 | 8.275 | 249 | FAIL |
| 08_back_inside | 56.964 | 73.676 | 24.947 | 238 | FAIL |
| 09_after_mesh | 47.144 | 56.717 | 35.674 | 236 | FAIL |
| 10_marker_ray | 40.198 | 54.688 | 39.111 | 238 | FAIL |
| 11_topdown | 26.624 | 67.247 | 8.798 | 216 | FAIL |
| 12_far_north_re | 85.278 | 90.245 | 8.270 | 246 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-432/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-432/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-432/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-432/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-432/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-432/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-432/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-432/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-432/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-432/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-432/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-432/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-432/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-432/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
- view probe: [view_probe.md](parity-run-432/view_probe.md) — per-position q/settle state, the v6.26.6 settle-by-laps + dip-probe channel
