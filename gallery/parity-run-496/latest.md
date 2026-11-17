# PARITY RUN 496 (run 44087716273, sha 7f01495) — gate: False

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

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-496/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-496/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-496/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-496/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-496/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-496/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-496/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-496/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-496/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-496/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-496/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-496/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-496/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-496/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
- view probe: [view_probe.md](parity-run-496/view_probe.md) — per-position q/settle state, the v6.26.6 settle-by-laps + dip-probe channel
