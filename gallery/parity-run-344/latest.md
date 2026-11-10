# PARITY RUN 344 (run 42852191039, sha 5db7321) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 77.891 | 81.848 | 17.789 | 242 | FAIL |
| 01_aerial_west | 46.945 | 56.682 | 33.796 | 249 | FAIL |
| 02_aerial_south | 41.550 | 56.657 | 39.286 | 253 | FAIL |
| 03_aerial_north | 55.226 | 71.362 | 16.007 | 253 | FAIL |
| 04_aerial_east | 51.414 | 58.483 | 35.307 | 254 | FAIL |
| 05_oblique | 85.435 | 91.174 | 8.686 | 218 | FAIL |
| 06_ground_west | 41.095 | 62.152 | 23.592 | 249 | FAIL |
| 07_far_north | 85.461 | 90.252 | 8.117 | 248 | FAIL |
| 08_back_inside | 61.126 | 77.736 | 20.526 | 237 | FAIL |
| 09_after_mesh | 54.192 | 65.433 | 26.295 | 233 | FAIL |
| 10_marker_ray | 53.070 | 66.823 | 30.811 | 236 | FAIL |
| 11_topdown | 31.777 | 83.532 | 4.772 | 212 | FAIL |
| 12_far_north_re | 85.461 | 90.251 | 8.110 | 245 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-344/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-344/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-344/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-344/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-344/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-344/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-344/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-344/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-344/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-344/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-344/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-344/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-344/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-344/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
