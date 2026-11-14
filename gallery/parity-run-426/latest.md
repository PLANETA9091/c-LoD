# PARITY RUN 426 (run 43589464420, sha d235615) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 79.252 | 84.411 | 18.084 | 209 | FAIL |
| 01_aerial_west | 58.490 | 72.270 | 20.749 | 251 | FAIL |
| 02_aerial_south | 50.566 | 72.416 | 21.884 | 253 | FAIL |
| 03_aerial_north | 53.226 | 74.041 | 16.107 | 253 | FAIL |
| 04_aerial_east | 51.768 | 65.296 | 30.668 | 254 | FAIL |
| 05_oblique | 87.147 | 96.401 | 2.501 | 169 | FAIL |
| 06_ground_west | 42.949 | 74.266 | 19.002 | 249 | FAIL |
| 07_far_north | 85.283 | 90.265 | 8.283 | 249 | FAIL |
| 08_back_inside | 56.782 | 72.772 | 24.335 | 238 | FAIL |
| 09_after_mesh | 48.321 | 57.724 | 35.790 | 237 | FAIL |
| 10_marker_ray | 40.064 | 54.222 | 40.172 | 238 | FAIL |
| 11_topdown | 26.241 | 66.534 | 9.126 | 216 | FAIL |
| 12_far_north_re | 85.283 | 90.265 | 8.297 | 246 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-426/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-426/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-426/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-426/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-426/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-426/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-426/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-426/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-426/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-426/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-426/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-426/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-426/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-426/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
- view probe: [view_probe.md](parity-run-426/view_probe.md) — per-position q/settle state, the v6.26.6 settle-by-laps + dip-probe channel
