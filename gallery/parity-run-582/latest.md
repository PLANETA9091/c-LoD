# PARITY RUN 582 (run 45006904647, sha a226317) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate (v6.58 CREDIT): credit zone = d<=8 (normal chunks, both clients render the same world — agreement there is credited); dispute zone = d>8 (content misses — the LOD queue). PASS at 100% exact, CREDITED at near% >= 80, FAIL below; the job exits 1 on any FAIL/missing shot

| vantage | exact % | credit % (d<=8) | dispute % (d>8) | mean d | max d | verdict |
|---|---|---|---|---|---|---|
| 00_summit_rise | 52.374 | 56.023 | 43.977 | 17.777 | 255 | FAIL |
| 01_aerial_west | 5.486 | 15.165 | 84.835 | 50.362 | 255 | FAIL |
| 02_aerial_south | 6.748 | 24.002 | 75.998 | 40.594 | 255 | FAIL |
| 03_aerial_north | 7.112 | 22.129 | 77.871 | 41.134 | 255 | FAIL |
| 04_aerial_east | 10.267 | 16.946 | 83.054 | 46.540 | 255 | FAIL |
| 05_oblique | 85.057 | 89.048 | 10.952 | 7.241 | 255 | CREDITED |
| 06_ground_west | 5.441 | 18.805 | 81.195 | 38.371 | 255 | FAIL |
| 07_far_north | 61.519 | 66.452 | 33.548 | 16.664 | 255 | FAIL |
| 08_back_inside | 6.653 | 13.438 | 86.562 | 43.297 | 255 | FAIL |
| 09_after_mesh | 5.505 | 15.399 | 84.601 | 46.605 | 255 | FAIL |
| 10_marker_ray | 56.860 | 68.489 | 31.511 | 21.060 | 255 | FAIL |
| 11_topdown | 2.629 | 25.048 | 74.952 | 25.797 | 228 | FAIL |
| 12_far_north_re | 60.256 | 65.184 | 34.816 | 17.051 | 255 | FAIL |

**GATE: RED — a pair sits below the credit floor**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-582/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-582/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-582/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-582/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-582/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-582/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-582/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-582/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-582/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-582/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-582/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-582/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-582/tri_12_far_north_re.jpg)
