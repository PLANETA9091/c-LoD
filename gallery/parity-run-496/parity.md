# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate (v6.58 CREDIT): credit zone = d<=8 (normal chunks, both clients render the same world — agreement there is credited); dispute zone = d>8 (content misses — the LOD queue). PASS at 100% exact, CREDITED at near% >= 80, FAIL below; the job exits 1 on any FAIL/missing shot

| vantage | exact % | credit % (d<=8) | dispute % (d>8) | mean d | max d | verdict |
|---|---|---|---|---|---|---|
| 00_summit_rise | 67.600 | 76.793 | 23.207 | 25.875 | 220 | FAIL |
| 01_aerial_west | 45.762 | 56.507 | 43.493 | 38.721 | 249 | FAIL |
| 02_aerial_south | 38.864 | 56.812 | 43.188 | 37.536 | 251 | FAIL |
| 03_aerial_north | 49.676 | 67.477 | 32.523 | 21.453 | 249 | FAIL |
| 04_aerial_east | 48.966 | 62.346 | 37.654 | 28.275 | 255 | FAIL |
| 05_oblique | 71.295 | 89.539 | 10.461 | 7.554 | 248 | CREDITED |
| 06_ground_west | 33.714 | 64.811 | 35.189 | 19.913 | 255 | FAIL |
| 07_far_north | 85.394 | 90.260 | 9.740 | 8.108 | 248 | CREDITED |
| 08_back_inside | 45.339 | 65.609 | 34.391 | 31.467 | 237 | FAIL |
| 09_after_mesh | 32.730 | 46.041 | 53.959 | 48.922 | 235 | FAIL |
| 10_marker_ray | 33.925 | 47.161 | 52.839 | 41.335 | 255 | FAIL |
| 11_topdown | 22.021 | 61.291 | 38.709 | 10.495 | 223 | FAIL |
| 12_far_north_re | 85.394 | 90.260 | 9.740 | 8.097 | 245 | CREDITED |

**GATE: RED — a pair sits below the credit floor**
