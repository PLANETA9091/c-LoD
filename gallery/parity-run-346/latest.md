# PARITY RUN 346 (run 42853527782, sha 469e24a) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 84.165 | 86.896 | 8.095 | 219 | FAIL |
| 01_aerial_west | 55.806 | 65.951 | 24.859 | 251 | FAIL |
| 02_aerial_south | 61.261 | 77.473 | 13.632 | 253 | FAIL |
| 03_aerial_north | 51.118 | 65.367 | 20.913 | 254 | FAIL |
| 04_aerial_east | 63.478 | 70.756 | 19.673 | 254 | FAIL |
| 05_oblique | 88.311 | 92.906 | 5.480 | 192 | FAIL |
| 06_ground_west | 38.085 | 65.604 | 16.720 | 249 | FAIL |
| 07_far_north | 85.337 | 90.278 | 8.118 | 248 | FAIL |
| 08_back_inside | 60.956 | 77.884 | 20.980 | 237 | FAIL |
| 09_after_mesh | 53.031 | 64.709 | 25.930 | 232 | FAIL |
| 10_marker_ray | 52.192 | 64.802 | 33.634 | 236 | FAIL |
| 11_topdown | 31.837 | 84.310 | 4.654 | 212 | FAIL |
| 12_far_north_re | 85.417 | 90.259 | 8.100 | 245 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-346/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-346/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-346/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-346/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-346/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-346/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-346/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-346/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-346/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-346/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-346/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-346/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-346/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-346/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
