# PARITY RUN 364 (run 42981503991, sha 37bbea0) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 90.111 | 90.122 | 11.596 | 219 | FAIL |
| 01_aerial_west | 46.218 | 46.240 | 70.903 | 250 | FAIL |
| 02_aerial_south | 37.664 | 37.688 | 74.592 | 248 | FAIL |
| 03_aerial_north | 38.473 | 68.658 | 17.063 | 249 | FAIL |
| 04_aerial_east | 58.523 | 59.893 | 29.454 | 248 | FAIL |
| 05_oblique | 20.139 | 29.678 | 120.757 | 223 | FAIL |
| 06_ground_west | 25.852 | 49.525 | 27.958 | 249 | FAIL |
| 07_far_north | 84.879 | 85.242 | 9.365 | 254 | FAIL |
| 08_back_inside | 46.286 | 60.645 | 23.806 | 237 | FAIL |
| 09_after_mesh | 38.622 | 56.501 | 27.727 | 231 | FAIL |
| 10_marker_ray | 46.761 | 52.974 | 33.819 | 236 | FAIL |
| 11_topdown | 0.025 | 46.057 | 9.709 | 223 | FAIL |
| 12_far_north_re | 84.879 | 85.242 | 9.372 | 254 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-364/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-364/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-364/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-364/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-364/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-364/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-364/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-364/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-364/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-364/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-364/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-364/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-364/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-364/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
- view probe: [view_probe.md](parity-run-364/view_probe.md) — per-position q/settle state, the v6.26.6 settle-by-laps + dip-probe channel
