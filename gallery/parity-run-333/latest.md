# PARITY RUN 333 (run 42775766094, sha c651676) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 74.733 | 78.084 | 23.957 | 210 | FAIL |
| 01_aerial_west | 58.701 | 69.528 | 21.838 | 248 | FAIL |
| 02_aerial_south | 48.373 | 65.114 | 33.184 | 253 | FAIL |
| 03_aerial_north | 54.426 | 72.318 | 18.187 | 253 | FAIL |
| 04_aerial_east | 42.952 | 52.082 | 50.464 | 254 | FAIL |
| 05_oblique | 92.596 | 97.424 | 1.765 | 169 | FAIL |
| 06_ground_west | 29.507 | 58.139 | 27.800 | 249 | FAIL |
| 07_far_north | 85.277 | 90.247 | 8.264 | 249 | FAIL |
| 08_back_inside | 61.205 | 77.889 | 20.565 | 238 | FAIL |
| 09_after_mesh | 53.812 | 65.053 | 25.896 | 237 | FAIL |
| 10_marker_ray | 51.085 | 64.637 | 33.899 | 238 | FAIL |
| 11_topdown | 32.284 | 85.779 | 4.221 | 216 | FAIL |
| 12_far_north_re | 85.277 | 90.243 | 8.282 | 246 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-333/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-333/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-333/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-333/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-333/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-333/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-333/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-333/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-333/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-333/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-333/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-333/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-333/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-333/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
