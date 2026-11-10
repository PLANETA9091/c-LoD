# PARITY RUN 340 (run 42846744383, sha a5b696d) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 76.852 | 80.387 | 18.656 | 220 | FAIL |
| 01_aerial_west | 48.515 | 58.004 | 31.915 | 251 | FAIL |
| 02_aerial_south | 42.400 | 58.838 | 39.022 | 253 | FAIL |
| 03_aerial_north | 53.469 | 69.146 | 17.345 | 253 | FAIL |
| 04_aerial_east | 49.674 | 58.207 | 35.366 | 254 | FAIL |
| 05_oblique | 83.283 | 87.476 | 10.522 | 219 | FAIL |
| 06_ground_west | 46.479 | 64.418 | 21.093 | 251 | FAIL |
| 07_far_north | 85.316 | 90.245 | 8.269 | 249 | FAIL |
| 08_back_inside | 61.018 | 77.629 | 20.648 | 238 | FAIL |
| 09_after_mesh | 54.043 | 65.259 | 23.767 | 237 | FAIL |
| 10_marker_ray | 49.855 | 64.265 | 33.687 | 236 | FAIL |
| 11_topdown | 31.105 | 81.640 | 5.332 | 215 | FAIL |
| 12_far_north_re | 85.411 | 90.276 | 8.122 | 245 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-340/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-340/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-340/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-340/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-340/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-340/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-340/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-340/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-340/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-340/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-340/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-340/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-340/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-340/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
