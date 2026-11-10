# PARITY RUN 339 (run 42846195606, sha 3da0b16) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 181, paired: 13, missing (voxy shot with no ours pair): 0
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 100.000 | 100.000 | 0.000 | 0 | PASS |
| 01_aerial_west | 0.000 | 10.031 | 111.868 | 255 | FAIL |
| 02_aerial_south | 0.000 | 2.468 | 128.423 | 255 | FAIL |
| 03_aerial_north | 0.000 | 2.514 | 125.390 | 255 | FAIL |
| 04_aerial_east | 0.000 | 12.468 | 140.113 | 255 | FAIL |
| 05_oblique | 0.000 | 1.441 | 111.171 | 255 | FAIL |
| 06_ground_west | 0.000 | 8.469 | 99.467 | 255 | FAIL |
| 07_far_north | 100.000 | 100.000 | 0.000 | 0 | PASS |
| 08_back_inside | 0.000 | 11.577 | 105.456 | 255 | FAIL |
| 09_after_mesh | 100.000 | 100.000 | 0.000 | 0 | PASS |
| 10_marker_ray | 100.000 | 100.000 | 0.000 | 0 | PASS |
| 11_topdown | 100.000 | 100.000 | 0.000 | 0 | PASS |
| 12_far_north_re | 100.000 | 100.000 | 0.000 | 0 | PASS |

- ours shots with no voxy pair (informational): 00_aerial_first_boot, 00_join_state, 00_pack_applied, 00_prompt, 00_tower_pre_OFF, 00_tower_pre_ON, 01_center_north_dwell, 01_horizon_ON, 01_night_first_frame, 01_noon_ground, 01_north_LOD_day, 01_platform_dwell_village_disk

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-339/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-339/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-339/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-339/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-339/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-339/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-339/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-339/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-339/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-339/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-339/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-339/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-339/tri_12_far_north_re.jpg)
