# PARITY RUN 335 (run 42784934872, sha 352e069) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 74.743 | 78.070 | 24.256 | 210 | FAIL |
| 01_aerial_west | 58.263 | 68.587 | 23.071 | 248 | FAIL |
| 02_aerial_south | 49.331 | 66.154 | 31.937 | 253 | FAIL |
| 03_aerial_north | 54.857 | 72.870 | 18.125 | 253 | FAIL |
| 04_aerial_east | 43.487 | 52.270 | 49.257 | 254 | FAIL |
| 05_oblique | 92.685 | 97.608 | 1.160 | 169 | FAIL |
| 06_ground_west | 53.350 | 73.713 | 16.113 | 249 | FAIL |
| 07_far_north | 85.314 | 90.260 | 8.186 | 248 | FAIL |
| 08_back_inside | 58.158 | 74.867 | 23.690 | 238 | FAIL |
| 09_after_mesh | 50.067 | 60.669 | 31.575 | 237 | FAIL |
| 10_marker_ray | 48.932 | 63.166 | 35.490 | 238 | FAIL |
| 11_topdown | 31.684 | 83.227 | 4.870 | 216 | FAIL |
| 12_far_north_re | 85.316 | 90.243 | 8.281 | 246 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-335/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-335/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-335/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-335/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-335/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-335/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-335/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-335/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-335/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-335/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-335/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-335/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-335/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-335/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
