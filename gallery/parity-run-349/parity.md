# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 226, paired: 13, missing (voxy shot with no ours pair): 0
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 100.000 | 100.000 | 0.000 | 0 | PASS |
| 01_aerial_west | 0.000 | 10.031 | 111.874 | 255 | FAIL |
| 02_aerial_south | 0.000 | 2.468 | 128.405 | 255 | FAIL |
| 03_aerial_north | 0.000 | 2.525 | 125.331 | 255 | FAIL |
| 04_aerial_east | 0.000 | 12.468 | 140.026 | 255 | FAIL |
| 05_oblique | 0.000 | 1.441 | 111.171 | 255 | FAIL |
| 06_ground_west | 0.000 | 8.469 | 99.464 | 255 | FAIL |
| 07_far_north | 100.000 | 100.000 | 0.000 | 0 | PASS |
| 08_back_inside | 0.000 | 11.577 | 105.457 | 255 | FAIL |
| 09_after_mesh | 100.000 | 100.000 | 0.000 | 0 | PASS |
| 10_marker_ray | 100.000 | 100.000 | 0.000 | 0 | PASS |
| 11_topdown | 100.000 | 100.000 | 0.000 | 0 | PASS |
| 12_far_north_re | 100.000 | 100.000 | 0.000 | 0 | PASS |

- ours shots with no voxy pair (informational): 00_aerial_first_boot, 00_join_state, 00_pack_applied, 00_prompt, 00_tower_pre_OFF, 00_tower_pre_ON, 01_center_north_dwell, 01_horizon_ON, 01_night_first_frame, 01_noon_ground, 01_north_LOD_day, 01_platform_dwell_village_disk

**GATE: RED — parity below 100%**
