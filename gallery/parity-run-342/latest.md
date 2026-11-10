# PARITY RUN 342 (run 42848159380, sha fb4eb74) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 85.577 | 88.028 | 7.018 | 210 | FAIL |
| 01_aerial_west | 51.816 | 61.283 | 27.288 | 253 | FAIL |
| 02_aerial_south | 56.595 | 68.896 | 18.436 | 253 | FAIL |
| 03_aerial_north | 49.805 | 64.987 | 22.141 | 253 | FAIL |
| 04_aerial_east | 63.760 | 70.295 | 19.842 | 254 | FAIL |
| 05_oblique | 88.515 | 93.507 | 5.098 | 219 | FAIL |
| 06_ground_west | 49.559 | 67.981 | 17.435 | 253 | FAIL |
| 07_far_north | 85.277 | 90.244 | 8.281 | 249 | FAIL |
| 08_back_inside | 60.931 | 77.618 | 20.866 | 238 | FAIL |
| 09_after_mesh | 53.693 | 65.159 | 24.655 | 237 | FAIL |
| 10_marker_ray | 51.651 | 65.549 | 32.646 | 241 | FAIL |
| 11_topdown | 32.212 | 85.774 | 4.252 | 215 | FAIL |
| 12_far_north_re | 85.390 | 90.267 | 8.118 | 245 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-342/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-342/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-342/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-342/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-342/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-342/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-342/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-342/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-342/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-342/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-342/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-342/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-342/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-342/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
