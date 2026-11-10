# PARITY RUN 337 (run 42842375738, sha 7f8a755) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 87.425 | 90.373 | 5.017 | 220 | FAIL |
| 01_aerial_west | 54.233 | 62.381 | 25.965 | 251 | FAIL |
| 02_aerial_south | 62.159 | 76.193 | 10.739 | 251 | FAIL |
| 03_aerial_north | 53.072 | 67.918 | 19.051 | 253 | FAIL |
| 04_aerial_east | 67.592 | 73.211 | 13.665 | 254 | FAIL |
| 05_oblique | 87.797 | 92.898 | 4.342 | 195 | FAIL |
| 06_ground_west | 48.895 | 66.943 | 16.491 | 251 | FAIL |
| 07_far_north | 85.485 | 90.392 | 8.243 | 249 | FAIL |
| 08_back_inside | 60.960 | 77.640 | 20.709 | 238 | FAIL |
| 09_after_mesh | 53.644 | 65.117 | 25.012 | 236 | FAIL |
| 10_marker_ray | 48.437 | 63.122 | 34.954 | 250 | FAIL |
| 11_topdown | 32.180 | 85.922 | 4.218 | 212 | FAIL |
| 12_far_north_re | 85.634 | 90.424 | 8.048 | 245 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-337/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-337/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-337/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-337/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-337/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-337/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-337/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-337/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-337/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-337/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-337/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-337/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-337/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-337/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
