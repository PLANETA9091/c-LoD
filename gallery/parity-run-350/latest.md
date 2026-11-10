# PARITY RUN 350 (run 42860865876, sha 5b1a99c) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 78.428 | 81.492 | 16.639 | 220 | FAIL |
| 01_aerial_west | 51.538 | 58.858 | 26.648 | 249 | FAIL |
| 02_aerial_south | 45.307 | 60.461 | 34.472 | 253 | FAIL |
| 03_aerial_north | 54.281 | 70.014 | 15.963 | 253 | FAIL |
| 04_aerial_east | 49.368 | 56.200 | 37.136 | 254 | FAIL |
| 05_oblique | 90.104 | 95.701 | 3.472 | 240 | FAIL |
| 06_ground_west | 45.121 | 61.216 | 18.817 | 250 | FAIL |
| 07_far_north | 85.307 | 90.245 | 8.254 | 248 | FAIL |
| 08_back_inside | 60.484 | 77.057 | 20.982 | 239 | FAIL |
| 09_after_mesh | 53.300 | 63.736 | 27.578 | 233 | FAIL |
| 10_marker_ray | 44.341 | 62.227 | 34.009 | 236 | FAIL |
| 11_topdown | 31.312 | 81.925 | 5.127 | 212 | FAIL |
| 12_far_north_re | 85.465 | 90.255 | 8.116 | 245 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-350/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-350/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-350/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-350/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-350/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-350/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-350/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-350/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-350/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-350/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-350/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-350/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-350/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-350/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
