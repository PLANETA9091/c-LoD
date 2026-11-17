# PARITY RUN 484 (run 44053352106, sha 34c47d5) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 67.255 | 76.196 | 26.416 | 220 | FAIL |
| 01_aerial_west | 44.418 | 56.350 | 38.596 | 251 | FAIL |
| 02_aerial_south | 37.752 | 56.505 | 37.993 | 251 | FAIL |
| 03_aerial_north | 46.625 | 62.180 | 26.471 | 253 | FAIL |
| 04_aerial_east | 49.590 | 63.360 | 26.359 | 255 | FAIL |
| 05_oblique | 67.144 | 86.908 | 10.826 | 248 | FAIL |
| 06_ground_west | 44.894 | 61.704 | 19.809 | 249 | FAIL |
| 07_far_north | 85.384 | 90.261 | 8.105 | 248 | FAIL |
| 08_back_inside | 43.758 | 64.272 | 32.997 | 227 | FAIL |
| 09_after_mesh | 31.966 | 45.931 | 51.465 | 227 | FAIL |
| 10_marker_ray | 36.582 | 49.416 | 38.417 | 252 | FAIL |
| 11_topdown | 21.031 | 61.701 | 10.346 | 223 | FAIL |
| 12_far_north_re | 85.383 | 90.258 | 8.100 | 245 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-484/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-484/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-484/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-484/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-484/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-484/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-484/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-484/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-484/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-484/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-484/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-484/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-484/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-484/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
- view probe: [view_probe.md](parity-run-484/view_probe.md) — per-position q/settle state, the v6.26.6 settle-by-laps + dip-probe channel
