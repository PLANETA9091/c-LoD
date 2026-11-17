# PARITY RUN 498 (run 44101655704, sha 865629d) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate (v6.58 CREDIT): credit zone = d<=8 (normal chunks, both clients render the same world — agreement there is credited); dispute zone = d>8 (content misses — the LOD queue). PASS at 100% exact, CREDITED at near% >= 80, FAIL below; the job exits 1 on any FAIL/missing shot

| vantage | exact % | credit % (d<=8) | dispute % (d>8) | mean d | max d | verdict |
|---|---|---|---|---|---|---|
| 00_summit_rise | 66.176 | 75.884 | 24.116 | 27.828 | 241 | FAIL |
| 01_aerial_west | 58.363 | 69.134 | 30.866 | 22.430 | 249 | FAIL |
| 02_aerial_south | 48.833 | 66.352 | 33.648 | 26.126 | 255 | FAIL |
| 03_aerial_north | 48.942 | 66.608 | 33.392 | 21.844 | 253 | FAIL |
| 04_aerial_east | 50.006 | 63.371 | 36.629 | 26.405 | 255 | FAIL |
| 05_oblique | 75.893 | 91.085 | 8.915 | 8.218 | 224 | CREDITED |
| 06_ground_west | 44.136 | 64.109 | 35.891 | 17.940 | 249 | FAIL |
| 07_far_north | 85.787 | 91.600 | 8.400 | 7.872 | 248 | CREDITED |
| 08_back_inside | 44.315 | 65.461 | 34.539 | 32.873 | 237 | FAIL |
| 09_after_mesh | 32.113 | 46.702 | 53.298 | 51.519 | 228 | FAIL |
| 10_marker_ray | 37.881 | 51.488 | 48.512 | 35.024 | 255 | FAIL |
| 11_topdown | 21.143 | 60.506 | 39.494 | 10.709 | 212 | FAIL |
| 12_far_north_re | 85.787 | 91.599 | 8.401 | 7.870 | 243 | CREDITED |

**GATE: RED — a pair sits below the credit floor**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-498/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-498/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-498/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-498/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-498/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-498/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-498/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-498/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-498/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-498/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-498/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-498/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-498/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-498/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
- view probe: [view_probe.md](parity-run-498/view_probe.md) — per-position q/settle state, the v6.26.6 settle-by-laps + dip-probe channel
