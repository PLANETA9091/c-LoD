# PARITY RUN 421 (run 43565424194, sha f686db1) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 79.254 | 84.369 | 18.429 | 207 | FAIL |
| 01_aerial_west | 58.656 | 72.831 | 20.816 | 251 | FAIL |
| 02_aerial_south | 50.185 | 71.299 | 21.670 | 253 | FAIL |
| 03_aerial_north | 55.078 | 76.984 | 15.544 | 253 | FAIL |
| 04_aerial_east | 52.007 | 65.687 | 30.029 | 254 | FAIL |
| 05_oblique | 86.749 | 95.991 | 3.059 | 171 | FAIL |
| 06_ground_west | 26.960 | 61.464 | 26.079 | 248 | FAIL |
| 07_far_north | 85.562 | 90.422 | 8.059 | 248 | FAIL |
| 08_back_inside | 57.322 | 74.939 | 23.990 | 239 | FAIL |
| 09_after_mesh | 50.068 | 59.797 | 34.789 | 233 | FAIL |
| 10_marker_ray | 44.993 | 57.772 | 35.533 | 236 | FAIL |
| 11_topdown | 27.984 | 71.262 | 7.957 | 222 | FAIL |
| 12_far_north_re | 85.563 | 90.424 | 8.047 | 245 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-421/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-421/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-421/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-421/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-421/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-421/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-421/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-421/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-421/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-421/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-421/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-421/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-421/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-421/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
- view probe: [view_probe.md](parity-run-421/view_probe.md) — per-position q/settle state, the v6.26.6 settle-by-laps + dip-probe channel
