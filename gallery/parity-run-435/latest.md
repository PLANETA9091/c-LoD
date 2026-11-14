# PARITY RUN 435 (run 43633147840, sha 5e309e2) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 79.226 | 84.282 | 18.297 | 204 | FAIL |
| 01_aerial_west | 60.508 | 76.473 | 19.094 | 254 | FAIL |
| 02_aerial_south | 51.446 | 73.642 | 21.943 | 253 | FAIL |
| 03_aerial_north | 53.760 | 75.272 | 15.884 | 253 | FAIL |
| 04_aerial_east | 52.276 | 65.829 | 32.176 | 254 | FAIL |
| 05_oblique | 89.991 | 97.682 | 1.230 | 177 | FAIL |
| 06_ground_west | 38.054 | 71.641 | 23.687 | 249 | FAIL |
| 07_far_north | 85.266 | 90.245 | 8.250 | 248 | FAIL |
| 08_back_inside | 57.088 | 74.752 | 24.592 | 238 | FAIL |
| 09_after_mesh | 48.709 | 61.228 | 31.268 | 237 | FAIL |
| 10_marker_ray | 40.537 | 57.541 | 38.026 | 238 | FAIL |
| 11_topdown | 27.522 | 71.305 | 7.562 | 215 | FAIL |
| 12_far_north_re | 85.278 | 90.243 | 8.281 | 246 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-435/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-435/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-435/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-435/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-435/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-435/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-435/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-435/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-435/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-435/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-435/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-435/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-435/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-435/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
- view probe: [view_probe.md](parity-run-435/view_probe.md) — per-position q/settle state, the v6.26.6 settle-by-laps + dip-probe channel
