# PARITY RUN 367 (run 42984087245, sha c79d21c) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 90.621 | 94.034 | 4.831 | 210 | FAIL |
| 01_aerial_west | 56.969 | 67.930 | 23.591 | 251 | FAIL |
| 02_aerial_south | 65.110 | 81.852 | 10.770 | 253 | FAIL |
| 03_aerial_north | 56.829 | 74.865 | 15.396 | 253 | FAIL |
| 04_aerial_east | 60.438 | 68.112 | 28.442 | 254 | FAIL |
| 05_oblique | 90.580 | 95.374 | 3.529 | 171 | FAIL |
| 06_ground_west | 35.660 | 63.150 | 26.325 | 249 | FAIL |
| 07_far_north | 85.555 | 90.426 | 8.060 | 248 | FAIL |
| 08_back_inside | 59.978 | 76.842 | 21.870 | 239 | FAIL |
| 09_after_mesh | 51.748 | 62.742 | 28.190 | 231 | FAIL |
| 10_marker_ray | 51.703 | 65.014 | 32.891 | 237 | FAIL |
| 11_topdown | 31.657 | 83.057 | 4.813 | 223 | FAIL |
| 12_far_north_re | 85.561 | 90.423 | 8.044 | 245 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-367/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-367/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-367/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-367/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-367/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-367/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-367/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-367/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-367/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-367/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-367/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-367/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-367/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-367/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
- view probe: [view_probe.md](parity-run-367/view_probe.md) — per-position q/settle state, the v6.26.6 settle-by-laps + dip-probe channel
