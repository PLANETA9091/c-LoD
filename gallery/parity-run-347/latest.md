# PARITY RUN 347 (run 42853768076, sha 3d43ca5) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 71.130 | 74.805 | 25.042 | 217 | FAIL |
| 01_aerial_west | 39.661 | 51.538 | 39.163 | 250 | FAIL |
| 02_aerial_south | 41.068 | 57.812 | 42.364 | 253 | FAIL |
| 03_aerial_north | 47.227 | 66.124 | 24.157 | 250 | FAIL |
| 04_aerial_east | 43.882 | 52.358 | 48.446 | 254 | FAIL |
| 05_oblique | 81.672 | 87.627 | 12.708 | 200 | FAIL |
| 06_ground_west | 45.530 | 65.200 | 23.769 | 249 | FAIL |
| 07_far_north | 85.368 | 90.269 | 8.119 | 248 | FAIL |
| 08_back_inside | 60.825 | 77.577 | 20.876 | 237 | FAIL |
| 09_after_mesh | 54.964 | 65.883 | 26.552 | 233 | FAIL |
| 10_marker_ray | 44.079 | 63.049 | 33.123 | 236 | FAIL |
| 11_topdown | 32.118 | 85.509 | 4.342 | 212 | FAIL |
| 12_far_north_re | 85.392 | 90.260 | 8.109 | 245 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-347/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-347/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-347/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-347/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-347/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-347/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-347/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-347/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-347/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-347/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-347/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-347/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-347/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-347/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
