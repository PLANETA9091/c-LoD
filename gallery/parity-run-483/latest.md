# PARITY RUN 483 (run 44044880666, sha 3bc7ebd) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 65.863 | 75.263 | 27.684 | 220 | FAIL |
| 01_aerial_west | 48.163 | 58.192 | 38.095 | 249 | FAIL |
| 02_aerial_south | 42.019 | 59.585 | 35.404 | 253 | FAIL |
| 03_aerial_north | 47.485 | 63.237 | 25.433 | 253 | FAIL |
| 04_aerial_east | 49.467 | 63.728 | 26.008 | 255 | FAIL |
| 05_oblique | 69.679 | 89.968 | 9.073 | 223 | FAIL |
| 06_ground_west | 31.868 | 63.347 | 21.410 | 255 | FAIL |
| 07_far_north | 85.635 | 90.422 | 8.059 | 248 | FAIL |
| 08_back_inside | 44.203 | 64.475 | 33.131 | 237 | FAIL |
| 09_after_mesh | 34.069 | 48.551 | 48.324 | 231 | FAIL |
| 10_marker_ray | 36.360 | 53.231 | 39.597 | 245 | FAIL |
| 11_topdown | 22.671 | 65.710 | 8.898 | 217 | FAIL |
| 12_far_north_re | 85.486 | 90.390 | 8.243 | 246 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-483/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-483/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-483/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-483/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-483/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-483/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-483/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-483/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-483/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-483/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-483/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-483/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-483/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-483/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
- view probe: [view_probe.md](parity-run-483/view_probe.md) — per-position q/settle state, the v6.26.6 settle-by-laps + dip-probe channel
