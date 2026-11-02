### c-LoD visual run (-930,796, marker=red_concrete, vantage -1005,150 yaw 270)
- boot + gates OK (05:51:50)
- sky-grid armed
- joined session 1 (05:52:17)
- stream live: cells=65536
- scan complete=1 (cells=cells=65536/65536)
- paint settled=0 (wire 709814p)
- baseline try 1: drift=18 px
- burst_close white px: 2 (renderer alive if > 50)
- burst_far white px: 0 (renderer alive in the marker zone if > 20)
- fill issued: 5x111x5 red_concrete at -930,796 (zone forceloaded ±48)

### /clod-cells AFTER FILL (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 27 0 0x8400 0x3100
123 122 -940 784 71 0 1 [97, 119, 97] 27 0 0x8700 0x3100
124 122 -938 784 68 0 1 [97, 119, 97] 27 0 0x8400 0x3100
125 122 -936 784 62 1 1 [63, 118, 228] 27 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 27 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 27 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 27 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 36 0 0x8000 0x5c00
130 122 -926 784 66 0 1 [134, 183, 131] 36 0 0x8200 0x5c00
131 122 -924 784 75 0 1 [97, 119, 97] 36 0 0x8b00 0x3100
132 122 -922 784 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 122 -920 784 75 0 1 [97, 119, 97] 18446744073709551615 1 0x8b00 0x3100
134 122 -918 784 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x3100
122 123 -942 786 70 0 1 [97, 119, 97] 27 0 0x8600 0x3100
123 123 -940 786 65 0 1 [97, 119, 97] 27 0 0x8100 0x3100
124 123 -938 786 65 0 1 [97, 119, 97] 27 0 0x8100 0x3100
125 123 -936 786 62 1 1 [63, 118, 228] 27 0 0x7e00 0xd800
```
### stream diag after fill:
[c-LoD] stream: tick 4900 65536/65536 cells, lap 4621, cone 97980 zone 640 dirty 1068 (q 11618) ewin 241 rbeat 0p/1 rescan 1221538c/1569e, edits +3313p/2485c/673e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 326 forced 186, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 795083p/2942b fail 0 ~0 kB/s cache 15994/24576 h779089 m15994 e0
[06:00:13 INFO]: [c-LoD] stream: tick 4900 65536/65536 cells, lap 4621, cone 97980 zone 640 dirty 1068 (q 11618) ewin 241 rbeat 0p/1 rescan 1221538c/1569e, edits +3313p/2485c/673e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 326 forced 186, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 795083p/2942b fail 0 ~0 kB/s cache 15994/24576 h779089 m15994 e0
- probe 0: red=22 paint=114 lens_red=0 lens_black=201056 lens_water=0 lens_dark=0 (need red>=150 paint>=300)
- probe 1: red=22 paint=114 lens_red=0 lens_black=201056 lens_water=0 lens_dark=0 (need red>=150 paint>=300)
- probe 2: red=22 paint=114 lens_red=0 lens_black=201056 lens_water=0 lens_dark=0 (need red>=150 paint>=300)
- probe 3: red=22 paint=114 lens_red=0 lens_black=201056 lens_water=0 lens_dark=0 (need red>=150 paint>=300)
- probe 4: red=22 paint=114 lens_red=0 lens_black=201056 lens_water=0 lens_dark=0 (need red>=150 paint>=300)
- probe 5: red=22 paint=114 lens_red=0 lens_black=201056 lens_water=0 lens_dark=0 (need red>=150 paint>=300)
- probe 6: red=22 paint=114 lens_red=0 lens_black=201056 lens_water=0 lens_dark=0 (need red>=150 paint>=300)
- probe 7: red=22 paint=114 lens_red=0 lens_black=201056 lens_water=0 lens_dark=0 (need red>=150 paint>=300)
- probe 8: red=22 paint=114 lens_red=0 lens_black=201056 lens_water=0 lens_dark=0 (need red>=150 paint>=300)
- probe 9: red=22 paint=114 lens_red=0 lens_black=201056 lens_water=0 lens_dark=0 (need red>=150 paint>=300)
- probe 10: red=22 paint=114 lens_red=0 lens_black=201056 lens_water=0 lens_dark=0 (need red>=150 paint>=300)
- probe 11: red=22 paint=114 lens_red=0 lens_black=201056 lens_water=0 lens_dark=0 (need red>=150 paint>=300)
- probe 12: red=22 paint=114 lens_red=0 lens_black=201056 lens_water=0 lens_dark=0 (need red>=150 paint>=300)
- probe 13: red=22 paint=114 lens_red=0 lens_black=201056 lens_water=0 lens_dark=0 (need red>=150 paint>=300)
- probe 14: red=22 paint=114 lens_red=0 lens_black=201056 lens_water=0 lens_dark=0 (need red>=150 paint>=300)
- probe 15: red=22 paint=114 lens_red=0 lens_black=201056 lens_water=0 lens_dark=0 (need red>=150 paint>=300)
- probe 16: red=22 paint=114 lens_red=0 lens_black=201056 lens_water=0 lens_dark=0 (need red>=150 paint>=300)
- probe 17: red=22 paint=114 lens_red=0 lens_black=201056 lens_water=0 lens_dark=0 (need red>=150 paint>=300)
- probe 18: red=22 paint=114 lens_red=0 lens_black=201056 lens_water=0 lens_dark=0 (need red>=150 paint>=300)
- probe 19: red=22 paint=114 lens_red=0 lens_black=201056 lens_water=0 lens_dark=0 (need red>=150 paint>=300)
- probe 20: red=22 paint=114 lens_red=0 lens_black=201056 lens_water=0 lens_dark=0 (need red>=150 paint>=300)
- probe 21: red=22 paint=114 lens_red=0 lens_black=201056 lens_water=0 lens_dark=0 (need red>=150 paint>=300)
- probe 22: red=21 paint=134 lens_red=0 lens_black=201056 lens_water=0 lens_dark=0 (need red>=150 paint>=300)
- probe 23: red=22 paint=120 lens_red=0 lens_black=201056 lens_water=0 lens_dark=0 (need red>=150 paint>=300)
### stream diag after probes:
[06:02:54 INFO]: [c-LoD] stream: tick 6500 65536/65536 cells, lap 4697, cone 100010 zone 1280 dirty 1120 (q 11620) ewin 0 rbeat 0p/0 rescan 1738415c/1643e, edits +4702p/3441c/957e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 326 forced 186, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 816817p/2973b fail 0 ~112 kB/s cache 15999/24576 h800818 m15999 e0
[c-LoD] stream: tick 6600 65536/65536 cells, lap 4797, cone 101345 zone 1280 dirty 1120 (q 11620) ewin 0 rbeat 0p/0 rescan 1772494c/1643e, edits +4703p/3442c/957e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 326 forced 186, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 827450p/2984b fail 0 ~56 kB/s cache 15999/24576 h811451 m15999 e0
[06:03:04 INFO]: [c-LoD] stream: tick 6600 65536/65536 cells, lap 4797, cone 101345 zone 1280 dirty 1120 (q 11620) ewin 0 rbeat 0p/0 rescan 1772494c/1643e, edits +4703p/3442c/957e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 326 forced 186, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 827450p/2984b fail 0 ~56 kB/s cache 15999/24576 h811451 m15999 e0
- joined session 2 (06:03:36)
- relog probe 0: red=0 lens_red=0 lens_black=201056 lens_water=0 lens_dark=0 (need >= 150)
- relog probe 1: red=0 lens_red=0 lens_black=201056 lens_water=0 lens_dark=0 (need >= 150)
- relog probe 2: red=0 lens_red=0 lens_black=201056 lens_water=0 lens_dark=0 (need >= 150)
- relog probe 3: red=0 lens_red=0 lens_black=201056 lens_water=0 lens_dark=0 (need >= 150)
- relog probe 4: red=0 lens_red=0 lens_black=201056 lens_water=0 lens_dark=0 (need >= 150)
- relog probe 5: red=0 lens_red=0 lens_black=201056 lens_water=0 lens_dark=0 (need >= 150)
- relog probe 6: red=0 lens_red=0 lens_black=201056 lens_water=0 lens_dark=0 (need >= 150)
- relog probe 7: red=0 lens_red=0 lens_black=201056 lens_water=0 lens_dark=0 (need >= 150)
- relog probe 8: red=0 lens_red=0 lens_black=201056 lens_water=0 lens_dark=0 (need >= 150)
- relog probe 9: red=0 lens_red=0 lens_black=201056 lens_water=0 lens_dark=0 (need >= 150)
- relog probe 10: red=0 lens_red=0 lens_black=201056 lens_water=0 lens_dark=0 (need >= 150)
- relog probe 11: red=0 lens_red=0 lens_black=201056 lens_water=0 lens_dark=0 (need >= 150)
- **FAIL:** marker lost after relog — stream state is not persistent/deterministic

### /clod-cells AFTER PROBES (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 7 0 0x8400 0x3100
123 122 -940 784 71 0 1 [97, 119, 97] 7 0 0x8700 0x3100
124 122 -938 784 68 0 1 [97, 119, 97] 6 0 0x8400 0x3100
125 122 -936 784 62 1 1 [63, 118, 228] 6 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 5 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 5 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 4 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 4 0 0x8000 0x5c00
130 122 -926 784 66 0 1 [134, 183, 131] 3 0 0x8200 0x5c00
131 122 -924 784 75 0 1 [97, 119, 97] 2 0 0x8b00 0x3100
132 122 -922 784 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 122 -920 784 75 0 1 [97, 119, 97] 18446744073709551615 1 0x8b00 0x3100
134 122 -918 784 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x3100
122 123 -942 786 70 0 1 [97, 119, 97] 8 0 0x8600 0x3100
123 123 -940 786 65 0 1 [97, 119, 97] 7 0 0x8100 0x3100
124 123 -938 786 65 0 1 [97, 119, 97] 7 0 0x8100 0x3100
125 123 -936 786 62 1 1 [63, 118, 228] 6 0 0x7e00 0xd800
```
- edit channel: 0 linkage errors, 0 failed event dispatches (both must be 0)
