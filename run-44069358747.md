- phase: 0 sanity (10:57:07)
### c-LoD visual run (-930,796, marker=red_concrete, vantage -1005,150 yaw 270)
- GL:     Vendor: Mesa (0xffffffff);    Device: llvmpipe (LLVM 21.1.8, 256 bits) (0xffffffff);OpenGL renderer string: llvmpipe (LLVM 21.1.8, 256 bits);OpenGL core profile version string: 4.5 (Core Profile) Mesa 26.0.8-1ubuntu0.3;
- boot + gates OK (10:57:25)
- phase: 1 server booted (10:57:25)
- sky-grid armed
- phase: 2 leg-1 client joined (10:57:43)
- joined session 1 (10:57:43)
- stream live: cells=65536
- phase: 3 scan populated (10:58:05)
- terrain range: 2 terraced mountains at (CX-4,CZ-20)/(CX-2,CZ+16), tops 185/190, grass terraces + stone caps, 66..82 m from the eye (inside the 83.5 m band, chunk edge 64 m)
- scan complete=1 (cells=cells=65536/65536)
- paint settled=1 (wire 549126p)
- baseline try 1: drift=1806 px
- baseline try 2: drift=4309 px
- baseline try 3: drift=1431 px
- lod_silhouette_cols=46 (LOD paint ON SCREEN at 75 m if > 20 — the march-drawn marker column; vanilla cannot render there, chunk edge 64 m; #255 re-measures 49, the ZONE_FAR=48 regression #256 and the control read 0)
- 8932 terrain-green px in the sky region (LOD MOUNTAINS on screen if > 5000; the range paints ColLow-green on low terraces via the fallback palette; flat-world baselines and the pack-free control read ~0)
- burst_close white px: 510 (renderer alive if > 50)
- burst_far white px: 368 (renderer alive in the marker zone if > 20)
- cut walk 40 m: 80 white px (> 20 = rendered)
- cut walk 60 m: 0 white px (> 20 = rendered)
- cut walk 100 m: 0 white px (> 20 = rendered)
- measured far-render cut: 40 m (G4 flap instrument; the decode proof needs >= 83 m from the y=151.6 eye)
- fill issued: 5x111x5 red_concrete at -930,796 (zone forceloaded ±48)
- phase: 4 marker filled (11:04:43)

### /clod-cells AFTER FILL (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 203 0 0x8400 0x2000
123 122 -940 784 71 0 1 [97, 119, 97] 203 0 0x8700 0x2000
124 122 -938 784 68 0 1 [97, 119, 97] 202 0 0x8400 0x2000
125 122 -936 784 62 1 1 [63, 118, 228] 242 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 241 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 240 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 240 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 532 0 0x8000 0x4100
130 122 -926 784 66 0 1 [134, 183, 131] 532 0 0x8200 0x4100
131 122 -924 784 75 0 1 [97, 119, 97] 532 0 0x8b00 0x2000
132 122 -922 784 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 122 -920 784 75 0 1 [97, 119, 97] 18446744073709551615 1 0x8b00 0x2000
134 122 -918 784 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x2000
122 123 -942 786 70 0 1 [97, 119, 97] 203 0 0x8600 0x2000
123 123 -940 786 65 0 1 [97, 119, 97] 203 0 0x8100 0x2000
124 123 -938 786 65 0 1 [97, 119, 97] 202 0 0x8100 0x2000
125 123 -936 786 62 1 1 [63, 118, 228] 242 0 0x7e00 0xd800
126 123 -934 786 62 1 1 [63, 118, 228] 241 0 0x7e00 0xd800
127 123 -932 786 62 1 1 [63, 118, 228] 241 0 0x7e00 0xd800
128 123 -930 786 62 1 1 [63, 118, 228] 240 0 0x7e00 0xd800
129 123 -928 786 73 0 1 [97, 119, 97] 532 0 0x8900 0x2000
130 123 -926 786 67 0 1 [134, 183, 131] 532 0 0x8300 0x4100
131 123 -924 786 69 0 1 [134, 183, 131] 532 0 0x8500 0x4100
132 123 -922 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 123 -920 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 123 -918 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
122 124 -942 788 72 0 1 [97, 119, 97] 203 0 0x8800 0x2000
123 124 -940 788 70 0 1 [97, 119, 97] 203 0 0x8600 0x2000
124 124 -938 788 68 0 1 [97, 119, 97] 202 0 0x8400 0x2000
125 124 -936 788 62 1 1 [63, 118, 228] 242 0 0x7e00 0xd800
126 124 -934 788 62 1 1 [63, 118, 228] 241 0 0x7e00 0xd800
127 124 -932 788 62 1 1 [63, 118, 228] 241 0 0x7e00 0xd800
128 124 -930 788 73 0 1 [97, 119, 97] 200 0 0x8900 0x2000
129 124 -928 788 76 0 1 [97, 119, 97] 532 0 0x8c00 0x2000
130 124 -926 788 73 0 1 [97, 119, 97] 532 0 0x8900 0x2000
131 124 -924 788 69 0 1 [134, 183, 131] 532 0 0x8500 0x4100
132 124 -922 788 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 124 -920 788 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 124 -918 788 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x2000
122 125 -942 790 74 0 1 [97, 119, 97] 203 0 0x8a00 0x2000
123 125 -940 790 73 0 1 [97, 119, 97] 203 0 0x8900 0x2000
124 125 -938 790 71 0 1 [97, 119, 97] 202 0 0x8700 0x2000
125 125 -936 790 62 1 1 [63, 118, 228] 242 0 0x7e00 0xd800
126 125 -934 790 62 1 1 [63, 118, 228] 241 0 0x7e00 0xd800
127 125 -932 790 62 1 1 [63, 118, 228] 241 0 0x7e00 0xd800
128 125 -930 790 62 1 1 [63, 118, 228] 240 0 0x7e00 0xd800
129 125 -928 790 73 0 1 [97, 119, 97] 636 0 0x8900 0x2000
130 125 -926 790 68 0 1 [134, 183, 131] 532 0 0x8400 0x4100
131 125 -924 790 69 0 1 [134, 183, 131] 532 0 0x8500 0x4100
132 125 -922 790 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
133 125 -920 790 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 125 -918 790 70 0 1 [134, 183, 131] 18446744073709551615 1 0x8600 0x4100
122 126 -942 792 74 0 1 [97, 119, 97] 203 0 0x8a00 0x2000
123 126 -940 792 73 0 1 [97, 119, 97] 203 0 0x8900 0x2000
124 126 -938 792 68 0 1 [97, 119, 97] 202 0 0x8400 0x2000
125 126 -936 792 62 1 1 [63, 118, 228] 242 0 0x7e00 0xd800
126 126 -934 792 62 1 1 [63, 118, 228] 241 0 0x7e00 0xd800
127 126 -932 792 62 1 1 [63, 118, 228] 241 0 0x7e00 0xd800
128 126 -930 792 62 1 1 [63, 118, 228] 240 0 0x7e00 0xd800
129 126 -928 792 67 0 1 [134, 183, 131] 200 0 0x8300 0x4100
130 126 -926 792 74 0 1 [97, 119, 97] 532 0 0x8a00 0x2000
131 126 -924 792 77 0 1 [97, 119, 97] 532 0 0x8d00 0x2000
132 126 -922 792 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 126 -920 792 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3200
134 126 -918 792 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x5000
122 127 -942 794 67 0 1 [134, 183, 131] 203 0 0x8300 0x4100
123 127 -940 794 68 0 1 [134, 183, 131] 203 0 0x8400 0x4100
124 127 -938 794 65 0 1 [134, 183, 131] 202 0 0x8100 0x4100
125 127 -936 794 62 1 1 [63, 118, 228] 242 0 0x7e00 0xd800
126 127 -934 794 62 1 1 [63, 118, 228] 241 0 0x7e00 0xd800
127 127 -932 794 170 0 1 [142, 33, 33] 241 1 0xea00 0x0a00
128 127 -930 794 170 0 1 [142, 33, 33] 240 1 0xea00 0x0a00
129 127 -928 794 170 0 1 [142, 33, 33] 200 1 0xea00 0x0a00
130 127 -926 794 74 0 1 [97, 119, 97] 532 0 0x8a00 0x2000
131 127 -924 794 77 0 1 [97, 119, 97] 532 0 0x8d00 0x2000
132 127 -922 794 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 127 -920 794 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3200
134 127 -918 794 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x5000
122 128 -942 796 68 0 1 [134, 183, 131] 203 0 0x8400 0x4100
123 128 -940 796 72 0 1 [97, 119, 97] 203 0 0x8800 0x2000
124 128 -938 796 75 0 1 [97, 119, 97] 202 0 0x8b00 0x2000
125 128 -936 796 72 0 1 [97, 119, 97] 202 0 0x8800 0x2000
126 128 -934 796 62 1 1 [63, 118, 228] 241 0 0x7e00 0xd800
127 128 -932 796 170 0 1 [142, 33, 33] 241 1 0xea00 0x0a00
128 128 -930 796 170 0 1 [142, 33, 33] 240 1 0xea00 0x0a00
129 128 -928 796 170 0 1 [142, 33, 33] 200 1 0xea00 0x0a00
130 128 -926 796 68 0 1 [134, 183, 131] 532 0 0x8400 0x4100
131 128 -924 796 74 0 1 [97, 119, 97] 532 0 0x8a00 0x2000
132 128 -922 796 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 128 -920 796 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
134 128 -918 796 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 129 -942 798 69 0 1 [134, 183, 131] 203 0 0x8500 0x4100
123 129 -940 798 75 0 1 [97, 119, 97] 203 0 0x8b00 0x2000
124 129 -938 798 78 0 1 [97, 119, 97] 204 0 0x8e00 0x2000
125 129 -936 798 75 0 1 [97, 119, 97] 202 0 0x8b00 0x2000
126 129 -934 798 62 1 1 [63, 118, 228] 241 0 0x7e00 0xd800
127 129 -932 798 170 0 1 [142, 33, 33] 241 1 0xea00 0x0a00
128 129 -930 798 170 0 1 [142, 33, 33] 240 1 0xea00 0x0a00
129 129 -928 798 170 0 1 [142, 33, 33] 200 1 0xea00 0x0a00
130 129 -926 798 71 0 1 [97, 119, 97] 532 0 0x8700 0x2000
131 129 -924 798 69 0 1 [134, 183, 131] 532 0 0x8500 0x4100
132 129 -922 798 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 129 -920 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 129 -918 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 130 -942 800 70 0 1 [134, 183, 131] 203 0 0x8600 0x4100
123 130 -940 800 76 0 1 [97, 119, 97] 203 0 0x8c00 0x2000
124 130 -938 800 76 0 1 [97, 119, 97] 202 0 0x8c00 0x2000
125 130 -936 800 72 0 1 [97, 119, 97] 202 0 0x8800 0x2000
126 130 -934 800 62 1 1 [63, 118, 228] 241 0 0x7e00 0xd800
127 130 -932 800 62 1 1 [63, 118, 228] 241 0 0x7e00 0xd800
128 130 -930 800 62 1 1 [63, 118, 228] 240 0 0x7e00 0xd800
129 130 -928 800 71 0 1 [97, 119, 97] 200 0 0x8700 0x2000
130 130 -926 800 71 0 1 [97, 119, 97] 532 0 0x8700 0x2000
131 130 -924 800 69 0 1 [134, 183, 131] 532 0 0x8500 0x4100
132 130 -922 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
133 130 -920 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 130 -918 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 131 -942 802 78 0 1 [97, 119, 97] 205 0 0x8e00 0x2000
123 131 -940 802 77 0 1 [97, 119, 97] 203 0 0x8d00 0x2000
124 131 -938 802 77 0 1 [97, 119, 97] 202 0 0x8d00 0x2000
125 131 -936 802 66 0 1 [134, 183, 131] 202 0 0x8200 0x4100
126 131 -934 802 62 1 1 [63, 118, 228] 241 0 0x7e00 0xd800
127 131 -932 802 62 1 1 [63, 118, 228] 241 0 0x7e00 0xd800
128 131 -930 802 62 1 1 [63, 118, 228] 240 0 0x7e00 0xd800
129 131 -928 802 62 1 1 [63, 118, 228] 574 0 0x7e00 0xd800
130 131 -926 802 66 0 1 [134, 183, 131] 532 0 0x8200 0x4100
131 131 -924 802 68 0 1 [134, 183, 131] 532 0 0x8400 0x4100
132 131 -922 802 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 131 -920 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 131 -918 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 132 -942 804 79 0 1 [97, 119, 97] 205 0 0x8f00 0x2000
123 132 -940 804 76 0 1 [97, 119, 97] 203 0 0x8c00 0x2000
124 132 -938 804 76 0 1 [97, 119, 97] 202 0 0x8c00 0x2000
125 132 -936 804 76 0 1 [97, 119, 97] 202 0 0x8c00 0x2000
126 132 -934 804 62 1 1 [63, 118, 228] 241 0 0x7e00 0xd800
127 132 -932 804 62 1 1 [63, 118, 228] 241 0 0x7e00 0xd800
128 132 -930 804 62 1 1 [63, 118, 228] 240 0 0x7e00 0xd800
129 132 -928 804 62 1 1 [63, 118, 228] 574 0 0x7e00 0xd800
130 132 -926 804 65 0 1 [134, 183, 131] 532 0 0x8100 0x4100
131 132 -924 804 77 0 1 [97, 119, 97] 532 0 0x8d00 0x2000
132 132 -922 804 79 0 1 [97, 119, 97] 532 0 0x8f00 0x2000
133 132 -920 804 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 132 -918 804 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x2000
122 133 -942 806 78 0 1 [97, 119, 97] 205 0 0x8e00 0x2000
123 133 -940 806 69 0 1 [134, 183, 131] 203 0 0x8500 0x4100
124 133 -938 806 155 0 1 [134, 183, 131] 203 0 0xdb00 0x4100
125 133 -936 806 155 0 1 [134, 183, 131] 203 0 0xdb00 0x4100
126 133 -934 806 155 0 1 [134, 183, 131] 202 0 0xdb00 0x4100
127 133 -932 806 155 0 1 [134, 183, 131] 202 0 0xdb00 0x4100
128 133 -930 806 155 0 1 [134, 183, 131] 201 0 0xdb00 0x4100
129 133 -928 806 155 0 1 [134, 183, 131] 200 0 0xdb00 0x4100
130 133 -926 806 155 0 1 [134, 183, 131] 200 0 0xdb00 0x4100
131 133 -924 806 67 0 1 [134, 183, 131] 532 0 0x8300 0x4100
132 133 -922 806 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 133 -920 806 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 133 -918 806 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x2000
122 134 -942 808 79 0 1 [97, 119, 97] 205 0 0x8f00 0x2000
123 134 -940 808 68 0 1 [134, 183, 131] 203 0 0x8400 0x4100
124 134 -938 808 155 0 1 [134, 183, 131] 203 0 0xdb00 0x4100
125 134 -936 808 180 0 1 [134, 183, 131] 200 0 0xf400 0x4100
126 134 -934 808 180 0 1 [134, 183, 131] 200 0 0xf400 0x4100
127 134 -932 808 180 0 1 [134, 183, 131] 532 0 0xf400 0x4100
128 134 -930 808 180 0 1 [134, 183, 131] 533 0 0xf400 0x4100
129 134 -928 808 180 0 1 [134, 183, 131] 533 0 0xf400 0x4100
130 134 -926 808 155 0 1 [134, 183, 131] 200 0 0xdb00 0x4100
131 134 -924 808 67 0 1 [134, 183, 131] 533 0 0x8300 0x4100
132 134 -922 808 68 0 1 [134, 183, 131] 18446744073709551615 1 0x8400 0x4100
133 134 -920 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 134 -918 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
```
### stream diag after fill:
[c-LoD] stream: tick 4300 65536/65536 cells, lap 764, cone 24106 zone 1645 dirty 928 (q 11807) ewin 232 rbeat 0p/0 rearm 0c/0t rescan 1109427c/2567e, edits +5908p/4455c/600e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 330 forced 182, native 65536c/1090ch fail 0 (3w q 0) helper on, trees 1090ch/fail 0, wire bundle 697521p/2136b fail 0 ~0 kB/s cache 13473/24576 h684048 m13473 e0, srbeat 60800s/0q drop 63stale/106undec, ereb 48371s/89q, sweep 0r/90d
[11:04:44 INFO]: [c-LoD] stream: tick 4300 65536/65536 cells, lap 764, cone 24106 zone 1645 dirty 928 (q 11807) ewin 232 rbeat 0p/0 rearm 0c/0t rescan 1109427c/2567e, edits +5908p/4455c/600e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 330 forced 182, native 65536c/1090ch fail 0 (3w q 0) helper on, trees 1090ch/fail 0, wire bundle 697521p/2136b fail 0 ~0 kB/s cache 13473/24576 h684048 m13473 e0, srbeat 60800s/0q drop 63stale/106undec, ereb 48371s/89q, sweep 0r/90d
- carrier ladder y=155: 23888 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=165: 23888 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=168: 23888 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=172: 23888 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=176: 23888 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=180: 25920 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=185: 25524 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- probe 0: red=0 paint=7671 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=16 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 1: red=0 paint=7691 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=16 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 2: red=0 paint=8225 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=16 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 3: red=0 paint=9191 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=16 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 4: red=0 paint=9981 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=16 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 5: red=0 paint=10403 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=16 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 6: red=0 paint=10221 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=16 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 7: red=0 paint=11637 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=16 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 8: red=0 paint=9931 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=16 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 9: red=0 paint=10088 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=16 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 10: red=0 paint=9907 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=16 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 11: red=0 paint=9907 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=16 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 12: red=0 paint=9907 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=16 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 13: red=0 paint=9907 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=16 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 14: red=0 paint=11370 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=16 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 15: red=0 paint=9990 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=16 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 16: red=0 paint=10812 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=16 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 17: red=0 paint=11912 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=16 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 18: red=0 paint=11687 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=16 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 19: red=0 paint=11340 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=16 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 20: red=0 paint=11227 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=16 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 21: red=0 paint=11707 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=16 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 22: red=0 paint=11663 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=16 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 23: red=0 paint=12565 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=16 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 24: red=0 paint=11380 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=16 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 25: red=0 paint=11495 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=16 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 26: red=0 paint=12008 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=16 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 27: red=0 paint=12173 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=16 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 28: red=0 paint=13191 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=16 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 29: red=0 paint=11756 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=16 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 30: red=0 paint=12693 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=16 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 31: red=0 paint=13483 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=16 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 32: red=0 paint=13213 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=16 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 33: red=0 paint=12477 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=16 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 34: red=0 paint=12824 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=16 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 35: red=0 paint=14748 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=16 mask_hits=30 (need red>=60 paint>=300 lod>20)
- phase: 5 leg-1 probes done (11:10:23)
- **FAIL:** leg-1 never saw the marker in 24 probes (red < 60 everywhere) — the edit never reached the LOD paint; this failure was SILENT before v6.31.1
### stream diag after probes:
[c-LoD] stream: tick 7500 65536/65536 cells, lap 968, cone 51152 zone 3730 dirty 961 (q 11836) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 2143367c/2630e, edits +7507p/5590c/977e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 330 forced 182, native 65536c/1090ch fail 0 (3w q 0) helper on, trees 1090ch/fail 0, wire bundle 1064066p/2711b fail 0 ~73 kB/s cache 13565/24576 h1050501 m13565 e0, srbeat 77754s/0q drop 69stale/113undec, ereb 49531s/0q, sweep 0r/32d
[c-LoD] stream: tick 7600 65536/65536 cells, lap 977, cone 52304 zone 3730 dirty 961 (q 11836) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 2175516c/2630e, edits +7507p/5590c/977e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 330 forced 182, native 65536c/1090ch fail 0 (3w q 0) helper on, trees 1090ch/fail 0, wire bundle 1078089p/2729b fail 0 ~73 kB/s cache 13565/24576 h1064524 m13565 e0, srbeat 78367s/0q drop 69stale/113undec, ereb 49531s/0q, sweep 0r/41d
[11:10:14 INFO]: [c-LoD] stream: tick 7600 65536/65536 cells, lap 977, cone 52304 zone 3730 dirty 961 (q 11836) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 2175516c/2630e, edits +7507p/5590c/977e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 330 forced 182, native 65536c/1090ch fail 0 (3w q 0) helper on, trees 1090ch/fail 0, wire bundle 1078089p/2729b fail 0 ~73 kB/s cache 13565/24576 h1064524 m13565 e0, srbeat 78367s/0q drop 69stale/113undec, ereb 49531s/0q, sweep 0r/41d
- parity debug tour: 13 vantages (session-1 client, DEBUG pack — evidence only, the gate pairs the §7b world session)
- parity debug tour done: 13 shots in parity_dbg/ (the §7b world session feeds the gate)
- joined session 2 (11:13:11)
- leg-2 baseline scene px (x<940): 0 (0 = the relog client paints no world at all)
- leg-2 renderer probe NEAR (20 m) white px: 538 raw / 538 changed-vs-baseline (> 20 = the client renders particles)
- leg-2 renderer probe FAR (80 m, marker zone) white px: 575 raw / 574 changed-vs-baseline (> 20 = no distance cut; ~0 with NEAR alive = particles beyond the client chunk radius do not render)
- leg-2 carrier probe green dust (80 m): changed-green px 0 / total changed 188 vs baseline (> 100 green = a stable green carrier exists; total > 0 with green ~0 = the dust renders but fog washes it — a ruler problem, not a carrier problem)
- phase: 8 leg-2 relog probes (11:18:34)
- LEG-2 SCENE DEAD: the relog client paints ZERO scene pixels (x<940) — pure sky + lens panels; the fresh-session capture chain never commits (renderer-dead class, #227/#228); these probes measure a worldless client, not the transport
- relog probe 0: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=0 mask_hits=0 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 1: red=0 scene=135820 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=7 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 2: red=0 scene=149489 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=2 hits_hi=8 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 3: red=7075 scene=149885 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=2 hits_hi=8 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- **LEG 2 PASS** (marker persisted over relog)
- phase: 9 leg-2 done (11:19:02)

### client log forensics (pipeline/particle/atlas — v6.8.2)
```
[11:13:02] [main/DEBUG]: Loading LWJGL system
WARNING: java.lang.System::load has been called by org.lwjgl.system.Library$$Lambda/0x000000003e1e3d60 in an unnamed module (file:/home/runner/mc-test/client/libraries/org/lwjgl/lwjgl/3.4.3/lwjgl-3.4.3.jar)
WARNING: sun.misc.Unsafe::objectFieldOffset has been called by org.lwjgl.system.MemoryBackendUnsafeLegacy (file:/home/runner/mc-test/client/libraries/org/lwjgl/lwjgl/3.4.3/lwjgl-3.4.3.jar)
WARNING: Please consider reporting this to the maintainers of class org.lwjgl.system.MemoryBackendUnsafeLegacy
[11:13:02] [main/DEBUG]: Loading shaderc
[11:13:02] [main/DEBUG]: Loading OpenGL
[11:13:06] [Render thread/ERROR]: Error parsing option value 4 for option Simulation Distance: Value 4 outside of range [5:33]
[11:13:06] [Render thread/INFO]: Backend library: LWJGL version 3.4.3+4
[11:13:06] [Render thread/INFO]: Using graphics backend OpenGL, using drivers: 4.5 (Core Profile) Mesa 26.0.8-1ubuntu0.3
[11:13:06] [Render thread/INFO]: Using graphics device: llvmpipe (LLVM 21.1.8, 256 bits) (Mesa)
[11:13:06] [Render thread/INFO]: Using graphics device extensions: GL_ARB_multi_draw_indirect, GL_ARB_buffer_storage, GL_ARB_shader_draw_parameters, GL_ARB_base_instance, GL_KHR_debug, GL_ARB_draw_indirect, GL_ARB_clip_control, GL_ARB_vertex_attrib_binding, GL_ARB_direct_state_access, GL_EXT_texture_filter_anisotropic
[11:13:06] [Download-2/ERROR]: Failed to fetch user properties
com.mojang.authlib.exceptions.InvalidCredentialsException: Status: 401
	at com.mojang.authlib.exceptions.MinecraftClientHttpException.toAuthenticationException(MinecraftClientHttpException.java:70)
Caused by: MinecraftClientHttpException[type=HTTP_ERROR, status=401, response=ErrorResponse[path=/player/attributes, error=null, errorMessage=null, details=null]]
	at com.mojang.authlib.minecraft.client.MinecraftClient.buildHttpException(MinecraftClient.java:170)
[11:13:06] [Render thread/INFO]: Reloading ResourceManager: vanilla, vanilla
[11:13:06] [Download-1/INFO]: Could not authorize you against Realms server: java.lang.RuntimeException: Failed to parse into SignedJWT: 0
[11:13:06] [Download-1/ERROR]: Failed to fetch Realms feature flags
com.mojang.realmsclient.exception.RealmsServiceException: Realms authentication error with message 'java.lang.RuntimeException: Failed to parse into SignedJWT: 0'
[11:13:08] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[11:13:08] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[11:13:08] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[11:13:08] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[11:13:08] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[11:13:08] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[11:13:08] [Render thread/INFO]: Created: 2048x2048x4 minecraft:textures/atlas/blocks.png-atlas
[11:13:09] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[11:13:09] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[11:13:09] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[11:13:09] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[11:13:09] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
[11:13:15] [Render thread/INFO]: Reloading ResourceManager: vanilla, vanilla, server/00000000/6b1f2f7e-9c0d-4a4b-8b0a-b10d1b4ab100
[11:13:17] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[11:13:17] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[11:13:17] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[11:13:17] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[11:13:17] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[11:13:17] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[11:13:17] [Render thread/INFO]: Created: 2048x2048x4 minecraft:textures/atlas/blocks.png-atlas
[11:13:17] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[11:13:17] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[11:13:17] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[11:13:17] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[11:13:17] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
```

### /clod-cells AFTER PROBES (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 11 0 0x8400 0x2000
123 122 -940 784 71 0 1 [97, 119, 97] 11 0 0x8700 0x2000
124 122 -938 784 68 0 1 [97, 119, 97] 10 0 0x8400 0x2000
125 122 -936 784 62 1 1 [63, 118, 228] 50 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 49 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 48 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 48 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 227 0 0x8000 0x4100
130 122 -926 784 66 0 1 [134, 183, 131] 227 0 0x8200 0x4100
131 122 -924 784 75 0 1 [97, 119, 97] 227 0 0x8b00 0x2000
132 122 -922 784 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 122 -920 784 75 0 1 [97, 119, 97] 18446744073709551615 1 0x8b00 0x2000
134 122 -918 784 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x2000
122 123 -942 786 70 0 1 [97, 119, 97] 11 0 0x8600 0x2000
123 123 -940 786 65 0 1 [97, 119, 97] 11 0 0x8100 0x2000
124 123 -938 786 65 0 1 [97, 119, 97] 10 0 0x8100 0x2000
125 123 -936 786 62 1 1 [63, 118, 228] 50 0 0x7e00 0xd800
126 123 -934 786 62 1 1 [63, 118, 228] 49 0 0x7e00 0xd800
127 123 -932 786 62 1 1 [63, 118, 228] 49 0 0x7e00 0xd800
128 123 -930 786 62 1 1 [63, 118, 228] 48 0 0x7e00 0xd800
129 123 -928 786 73 0 1 [97, 119, 97] 227 0 0x8900 0x2000
130 123 -926 786 67 0 1 [134, 183, 131] 227 0 0x8300 0x4100
131 123 -924 786 69 0 1 [134, 183, 131] 227 0 0x8500 0x4100
132 123 -922 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 123 -920 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 123 -918 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
122 124 -942 788 72 0 1 [97, 119, 97] 11 0 0x8800 0x2000
123 124 -940 788 70 0 1 [97, 119, 97] 11 0 0x8600 0x2000
124 124 -938 788 68 0 1 [97, 119, 97] 10 0 0x8400 0x2000
125 124 -936 788 62 1 1 [63, 118, 228] 50 0 0x7e00 0xd800
126 124 -934 788 62 1 1 [63, 118, 228] 49 0 0x7e00 0xd800
127 124 -932 788 62 1 1 [63, 118, 228] 49 0 0x7e00 0xd800
128 124 -930 788 73 0 1 [97, 119, 97] 8 0 0x8900 0x2000
129 124 -928 788 76 0 1 [97, 119, 97] 227 0 0x8c00 0x2000
130 124 -926 788 73 0 1 [97, 119, 97] 227 0 0x8900 0x2000
131 124 -924 788 69 0 1 [134, 183, 131] 227 0 0x8500 0x4100
132 124 -922 788 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 124 -920 788 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 124 -918 788 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x2000
122 125 -942 790 74 0 1 [97, 119, 97] 11 0 0x8a00 0x2000
123 125 -940 790 73 0 1 [97, 119, 97] 11 0 0x8900 0x2000
124 125 -938 790 71 0 1 [97, 119, 97] 10 0 0x8700 0x2000
125 125 -936 790 62 1 1 [63, 118, 228] 50 0 0x7e00 0xd800
126 125 -934 790 62 1 1 [63, 118, 228] 49 0 0x7e00 0xd800
127 125 -932 790 62 1 1 [63, 118, 228] 49 0 0x7e00 0xd800
128 125 -930 790 62 1 1 [63, 118, 228] 48 0 0x7e00 0xd800
129 125 -928 790 73 0 1 [97, 119, 97] 227 0 0x8900 0x2000
130 125 -926 790 68 0 1 [134, 183, 131] 227 0 0x8400 0x4100
131 125 -924 790 69 0 1 [134, 183, 131] 227 0 0x8500 0x4100
132 125 -922 790 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
133 125 -920 790 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 125 -918 790 70 0 1 [134, 183, 131] 18446744073709551615 1 0x8600 0x4100
122 126 -942 792 74 0 1 [97, 119, 97] 11 0 0x8a00 0x2000
123 126 -940 792 73 0 1 [97, 119, 97] 11 0 0x8900 0x2000
124 126 -938 792 68 0 1 [97, 119, 97] 10 0 0x8400 0x2000
125 126 -936 792 62 1 1 [63, 118, 228] 50 0 0x7e00 0xd800
126 126 -934 792 62 1 1 [63, 118, 228] 49 0 0x7e00 0xd800
127 126 -932 792 62 1 1 [63, 118, 228] 49 0 0x7e00 0xd800
128 126 -930 792 62 1 1 [63, 118, 228] 48 0 0x7e00 0xd800
129 126 -928 792 67 0 1 [134, 183, 131] 227 0 0x8300 0x4100
130 126 -926 792 74 0 1 [97, 119, 97] 227 0 0x8a00 0x2000
131 126 -924 792 77 0 1 [97, 119, 97] 227 0 0x8d00 0x2000
132 126 -922 792 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 126 -920 792 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3200
134 126 -918 792 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x5000
122 127 -942 794 67 0 1 [134, 183, 131] 11 0 0x8300 0x4100
123 127 -940 794 68 0 1 [134, 183, 131] 11 0 0x8400 0x4100
124 127 -938 794 65 0 1 [134, 183, 131] 10 0 0x8100 0x4100
125 127 -936 794 62 1 1 [63, 118, 228] 50 0 0x7e00 0xd800
126 127 -934 794 62 1 1 [63, 118, 228] 49 0 0x7e00 0xd800
127 127 -932 794 170 0 1 [142, 33, 33] 9 0 0xea00 0x0a00
128 127 -930 794 170 0 1 [142, 33, 33] 8 0 0xea00 0x0a00
129 127 -928 794 170 0 1 [142, 33, 33] 227 0 0xea00 0x0a00
130 127 -926 794 74 0 1 [97, 119, 97] 227 0 0x8a00 0x2000
131 127 -924 794 77 0 1 [97, 119, 97] 227 0 0x8d00 0x2000
132 127 -922 794 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 127 -920 794 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3200
134 127 -918 794 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x5000
122 128 -942 796 68 0 1 [134, 183, 131] 11 0 0x8400 0x4100
123 128 -940 796 72 0 1 [97, 119, 97] 11 0 0x8800 0x2000
124 128 -938 796 75 0 1 [97, 119, 97] 10 0 0x8b00 0x2000
125 128 -936 796 72 0 1 [97, 119, 97] 10 0 0x8800 0x2000
126 128 -934 796 62 1 1 [63, 118, 228] 49 0 0x7e00 0xd800
127 128 -932 796 170 0 1 [142, 33, 33] 9 0 0xea00 0x0a00
128 128 -930 796 170 0 1 [142, 33, 33] 8 0 0xea00 0x0a00
129 128 -928 796 170 0 1 [142, 33, 33] 8 0 0xea00 0x0a00
130 128 -926 796 68 0 1 [134, 183, 131] 227 0 0x8400 0x4100
131 128 -924 796 74 0 1 [97, 119, 97] 227 0 0x8a00 0x2000
132 128 -922 796 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 128 -920 796 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
134 128 -918 796 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 129 -942 798 69 0 1 [134, 183, 131] 11 0 0x8500 0x4100
123 129 -940 798 75 0 1 [97, 119, 97] 11 0 0x8b00 0x2000
124 129 -938 798 78 0 1 [97, 119, 97] 12 0 0x8e00 0x2000
125 129 -936 798 75 0 1 [97, 119, 97] 10 0 0x8b00 0x2000
126 129 -934 798 62 1 1 [63, 118, 228] 49 0 0x7e00 0xd800
127 129 -932 798 170 0 1 [142, 33, 33] 9 0 0xea00 0x0a00
128 129 -930 798 170 0 1 [142, 33, 33] 8 0 0xea00 0x0a00
129 129 -928 798 170 0 1 [142, 33, 33] 227 0 0xea00 0x0a00
130 129 -926 798 71 0 1 [97, 119, 97] 227 0 0x8700 0x2000
131 129 -924 798 69 0 1 [134, 183, 131] 227 0 0x8500 0x4100
132 129 -922 798 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 129 -920 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 129 -918 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 130 -942 800 70 0 1 [134, 183, 131] 11 0 0x8600 0x4100
123 130 -940 800 76 0 1 [97, 119, 97] 11 0 0x8c00 0x2000
124 130 -938 800 76 0 1 [97, 119, 97] 10 0 0x8c00 0x2000
125 130 -936 800 72 0 1 [97, 119, 97] 10 0 0x8800 0x2000
126 130 -934 800 62 1 1 [63, 118, 228] 49 0 0x7e00 0xd800
127 130 -932 800 62 1 1 [63, 118, 228] 49 0 0x7e00 0xd800
128 130 -930 800 62 1 1 [63, 118, 228] 48 0 0x7e00 0xd800
129 130 -928 800 71 0 1 [97, 119, 97] 227 0 0x8700 0x2000
130 130 -926 800 71 0 1 [97, 119, 97] 227 0 0x8700 0x2000
131 130 -924 800 69 0 1 [134, 183, 131] 227 0 0x8500 0x4100
132 130 -922 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
133 130 -920 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 130 -918 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 131 -942 802 78 0 1 [97, 119, 97] 13 0 0x8e00 0x2000
123 131 -940 802 77 0 1 [97, 119, 97] 11 0 0x8d00 0x2000
124 131 -938 802 77 0 1 [97, 119, 97] 10 0 0x8d00 0x2000
125 131 -936 802 66 0 1 [134, 183, 131] 10 0 0x8200 0x4100
126 131 -934 802 62 1 1 [63, 118, 228] 49 0 0x7e00 0xd800
127 131 -932 802 62 1 1 [63, 118, 228] 49 0 0x7e00 0xd800
128 131 -930 802 62 1 1 [63, 118, 228] 48 0 0x7e00 0xd800
129 131 -928 802 62 1 1 [63, 118, 228] 267 0 0x7e00 0xd800
130 131 -926 802 66 0 1 [134, 183, 131] 227 0 0x8200 0x4100
131 131 -924 802 68 0 1 [134, 183, 131] 227 0 0x8400 0x4100
132 131 -922 802 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 131 -920 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 131 -918 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 132 -942 804 79 0 1 [97, 119, 97] 13 0 0x8f00 0x2000
123 132 -940 804 76 0 1 [97, 119, 97] 11 0 0x8c00 0x2000
124 132 -938 804 76 0 1 [97, 119, 97] 10 0 0x8c00 0x2000
125 132 -936 804 76 0 1 [97, 119, 97] 10 0 0x8c00 0x2000
126 132 -934 804 62 1 1 [63, 118, 228] 49 0 0x7e00 0xd800
127 132 -932 804 62 1 1 [63, 118, 228] 49 0 0x7e00 0xd800
128 132 -930 804 62 1 1 [63, 118, 228] 48 0 0x7e00 0xd800
129 132 -928 804 62 1 1 [63, 118, 228] 267 0 0x7e00 0xd800
130 132 -926 804 65 0 1 [134, 183, 131] 227 0 0x8100 0x4100
131 132 -924 804 77 0 1 [97, 119, 97] 227 0 0x8d00 0x2000
132 132 -922 804 79 0 1 [97, 119, 97] 227 0 0x8f00 0x2000
133 132 -920 804 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 132 -918 804 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x2000
122 133 -942 806 78 0 1 [97, 119, 97] 13 0 0x8e00 0x2000
123 133 -940 806 69 0 1 [134, 183, 131] 11 0 0x8500 0x4100
124 133 -938 806 155 0 1 [134, 183, 131] 11 0 0xdb00 0x4100
125 133 -936 806 155 0 1 [134, 183, 131] 11 0 0xdb00 0x4100
126 133 -934 806 155 0 1 [134, 183, 131] 10 0 0xdb00 0x4100
127 133 -932 806 155 0 1 [134, 183, 131] 10 0 0xdb00 0x4100
128 133 -930 806 155 0 1 [134, 183, 131] 9 0 0xdb00 0x4100
129 133 -928 806 155 0 1 [134, 183, 131] 8 0 0xdb00 0x4100
130 133 -926 806 155 0 1 [134, 183, 131] 8 0 0xdb00 0x4100
131 133 -924 806 67 0 1 [134, 183, 131] 227 0 0x8300 0x4100
132 133 -922 806 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 133 -920 806 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 133 -918 806 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x2000
122 134 -942 808 79 0 1 [97, 119, 97] 13 0 0x8f00 0x2000
123 134 -940 808 68 0 1 [134, 183, 131] 11 0 0x8400 0x4100
124 134 -938 808 155 0 1 [134, 183, 131] 11 0 0xdb00 0x4100
125 134 -936 808 180 0 1 [134, 183, 131] 8 0 0xf400 0x4100
126 134 -934 808 180 0 1 [134, 183, 131] 8 0 0xf400 0x4100
127 134 -932 808 180 0 1 [134, 183, 131] 227 0 0xf400 0x4100
128 134 -930 808 180 0 1 [134, 183, 131] 227 0 0xf400 0x4100
129 134 -928 808 180 0 1 [134, 183, 131] 227 0 0xf400 0x4100
130 134 -926 808 155 0 1 [134, 183, 131] 8 0 0xdb00 0x4100
131 134 -924 808 67 0 1 [134, 183, 131] 227 0 0x8300 0x4100
132 134 -922 808 68 0 1 [134, 183, 131] 18446744073709551615 1 0x8400 0x4100
133 134 -920 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 134 -918 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
```
- edit channel: 0 linkage errors, 0 failed event dispatches (both must be 0)
- phase: 6 parity world boot (11:19:02)
- parity world: view-distance=8 (the voxy leg's server value)
- parity world: joined session 3 (11:19:36)
- parity world: scan populated (cells=65536; the march paints from these buffers)
- parity world tour: 13 vantages (production pack, vd 8, shared structures)
- coverage sweeps: 72 eye passes = 36 positions (12 az x r144/200 + 12 off-az x r172, y=160) x 2 looks (outward+inward), settle-by-laps >= 6 s (q stable 3x1s), deadline 12 s/pos, no shots — the v6.26.0 data-coverage lever + v6.26.6 lottery kill; probe rides parity/view_probe.md
- parity drain-settle TIMEOUT: queue 19613 -> 19451 > 11500 after 150s — continuing anyway; the queue did not drain without eye proximity (priority-lane evidence, the 01/05/10 dip attribution reads from the tour deltas)
- pre-tour pass done: 13 vantages x 10s (the tour now reads the second OR-merge lap; the rim/dip transport race reads from the tour deltas)
- parity world tour done: 13 shots, 0 night-guard renames in parity/ (the gate pairs voxy/<name>.png vs parity/<name>.png)
- phase: 7 parity tour done (11:41:20)
- sparse re-beat channel at tour end: streamed=138975 q=0 dropped_stale=85 dropped_undecodable=10494 (frac=8 boot; undecodable-dominant = the rim words die in their decodability window, stale-dominant = edit churn owns the cells)
### parity world stream counters (session 3, tail + maxima)
[c-LoD] stream: tick 13100 65536/65536 cells, lap 1255, cone 66099 zone 25401 dirty 17657 (q 19430) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 8977789c/1676e, edits +17935p/13596c/2782e drop 0, spawned 0 / failed 0, cam0 yaw -180 pitch 5, wish 0 forced 940, native 38080c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 1938397p/4684b fail 0 ~0 kB/s cache 19952/24576 h1478455 m459942 e439990, srbeat 138975s/0q drop 85stale/10494undec, ereb 83604s/0q, sweep 0r/68d
[11:41:19 INFO]: [c-LoD] stream: tick 13100 65536/65536 cells, lap 1255, cone 66099 zone 25401 dirty 17657 (q 19430) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 8977789c/1676e, edits +17935p/13596c/2782e drop 0, spawned 0 / failed 0, cam0 yaw -180 pitch 5, wish 0 forced 940, native 38080c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 1938397p/4684b fail 0 ~0 kB/s cache 19952/24576 h1478455 m459942 e439990, srbeat 138975s/0q drop 85stale/10494undec, ereb 83604s/0q, sweep 0r/68d
- parity stream max cone: 66099
- parity stream max zone: 25401
- parity stream max wire bundle: bundle
### parity world capture lens (debug chain on the §7b world)
lens_red=0 lens_black=0 lens_water=0 lens_dark=0 lens_zero=0 rot=none scan=0 carriers=0
- parity mesh probe @png_y=2: x306=(151,184,255) x307=(151,184,255) x308=(151,184,255) x309h=(151,184,255) x310c=(151,184,255)
- parity mesh probe @png_y=717: x306=(65,81,67) x307=(76,102,82) x308=(83,109,89) x309h=(83,109,89) x310c=(85,112,90)
- parity mesh engagement: magenta-ish=0 of 921600 sampled px
- mask8: the un-fog engagement masks shot on the PARITY WORLD (vd 8) — parity_dbg8/01m|07m|10m
- mask8 01m: green=0 red=0 blue=94176 dim=4096 (px, sampled /16)
- mask8 07m: green=11616 red=0 blue=220624 dim=5824 (px, sampled /16)
- mask8 10m: green=0 red=0 blue=216688 dim=4096 (px, sampled /16)
- mesh probe @png_y=2: x306=(255,255,255) x307=(13,254,0) x308=(0,254,0) x309h=(0,0,0) x310c=(0,0,0)
- mesh probe @png_y=717: x306=(255,255,255) x307=(13,254,0) x308=(0,254,0) x309h=(0,0,0) x310c=(0,0,0)
- mesh engagement: magenta-ish=0 of 921600 sampled px (the vanilla-path paint coverage)
- mask8 fog state @png_y=2: raw (300)=(151,184,255) (301)=(151,184,255)
- mask8 fog state @png_y=717: raw (300)=(0,128,8) (301)=(179,206,255)
- mask8 fog state decoded: envStart=0 envEnd=1024 rdEnd=128 fogColor=(179,206,255)
- buffer dump 10m: marked h=0/2048 c=0/2048; height windows r0<142:0 r1:0 r2>=202:0 maxByte=0
- buffer dump 10m: color idx0(black)=0 water/height mismatches=0 of 0 pairs
- buffer dump 01m: marked h=0/2048 c=0/2048; height windows r0<142:0 r1:0 r2>=202:0 maxByte=0
- buffer dump 01m: color idx0(black)=0 water/height mismatches=0 of 0 pairs
- rim mask: scan populated (cells=65536; the state paint reads the same buffers the march reads)
- phase: 10 rim mask lane (11:45:23)
- rim mask: decode-state frames shot (rim_mask/00|01|12 — GREEN full decode / RED colorless / BLUE heightless; 00 = the leg-1 tower eye, 01 = the near-tower contrast, 12 = the tower re-read; the inherited 07/12 vantages sat 760 blocks outside the streamed disk and 01 sat INSIDE the marker column — their earlier verdicts were classifier noise, purged in v6.33.1)
- rim mask 00: green=35552 red=30624 blue=240544 yellow=64384 (px, sampled /16) -> DOMINANT BLUE: rim = HEIGHTS NEVER DECODED — a data-coverage class: the state paint cannot even see the cells
- rim mask 01: green=111344 red=28832 blue=238768 yellow=59040 (px, sampled /16) -> DOMINANT BLUE: rim = HEIGHTS NEVER DECODED — a data-coverage class: the state paint cannot even see the cells
- rim mask 12: green=74208 red=23920 blue=268304 yellow=66992 (px, sampled /16) -> DOMINANT BLUE: rim = HEIGHTS NEVER DECODED — a data-coverage class: the state paint cannot even see the cells

### boot / stream
```
[c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 149754 bytes, sha1 5dc283742995ead8546b0a2bbfadc892d3c58358 — zero textures, zero world data, one push per join
[c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[11:44:45 INFO]: [c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 149754 bytes, sha1 5dc283742995ead8546b0a2bbfadc892d3c58358 — zero textures, zero world data, one push per join
[11:44:45 INFO]: [c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[c-LoD] stream: tick 800 65536/65536 cells, lap 603, cone 10479 zone 894 dirty 23 (q 4218) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 498101c/1047e, edits +11p/11c/9e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 311 forced 505, native 21886c/950ch fail 0 (3w q 0) helper on, trees 950ch/fail 0, wire bundle 128233p/225b fail 0 ~30 kB/s cache 13987/24576 h114246 m13987 e0, srbeat 6567s/470q drop 0stale/1089undec, ereb 152s/30q, sweep 0r/9d
[11:46:14 INFO]: [c-LoD] stream: tick 800 65536/65536 cells, lap 603, cone 10479 zone 894 dirty 23 (q 4218) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 498101c/1047e, edits +11p/11c/9e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 311 forced 505, native 21886c/950ch fail 0 (3w q 0) helper on, trees 950ch/fail 0, wire bundle 128233p/225b fail 0 ~30 kB/s cache 13987/24576 h114246 m13987 e0, srbeat 6567s/470q drop 0stale/1089undec, ereb 152s/30q, sweep 0r/9d
```
### sessions
```
[11:45:02 INFO]: System chat: Stepan joined the game
[11:46:18 INFO]: System chat: Stepan left the game
```
### client log tail
```
[11:46:16] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[11:46:16] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[11:46:16] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[11:46:16] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[11:46:16] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[11:46:16] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[11:46:16] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[11:46:16] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[11:46:17] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[11:46:17] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[11:46:17] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[11:46:17] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[11:46:17] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[11:46:17] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[11:46:17] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:move_player_pos] net.minecraft.network.protocol.game.ServerboundMovePlayerPacket$Pos -> 26 bytes
[11:46:17] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[11:46:17] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[11:46:17] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[11:46:17] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[11:46:17] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[11:46:17] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[11:46:17] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[11:46:17] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[11:46:17] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[11:46:17] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[11:46:17] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:set_time] net.minecraft.network.protocol.game.ClientboundSetTimePacket -> 10 bytes
[11:46:17] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[11:46:17] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:keep_alive] net.minecraft.network.protocol.common.ClientboundKeepAlivePacket -> 9 bytes
[11:46:17] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:keep_alive] net.minecraft.network.protocol.common.ServerboundKeepAlivePacket -> 9 bytes
[11:46:17] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:bundle_delimiter] net.minecraft.network.protocol.game.ClientboundBundleDelimiterPacket -> 1 bytes
[11:46:17] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[11:46:17] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[11:46:17] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[11:46:17] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[11:46:17] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[11:46:17] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[11:46:17] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[11:46:17] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[11:46:17] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:bundle_delimiter] net.minecraft.network.protocol.game.ClientboundBundleDelimiterPacket -> 1 bytes
[11:46:17] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[11:46:17] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[11:46:17] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[11:46:17] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[11:46:17] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[11:46:17] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[11:46:17] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[11:46:17] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[11:46:17] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[11:46:17] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[11:46:17] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[11:46:17] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[11:46:17] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[11:46:17] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[11:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[11:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[11:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[11:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[11:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[11:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[11:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[11:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[11:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[11:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[11:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[11:46:18] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:move_player_pos] net.minecraft.network.protocol.game.ServerboundMovePlayerPacket$Pos -> 26 bytes
[11:46:18] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[11:46:18] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[11:46:18] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[11:46:18] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[11:46:18] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[11:46:18] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[11:46:18] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[11:46:18] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[11:46:18] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[11:46:18] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[11:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[11:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[11:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:set_time] net.minecraft.network.protocol.game.ClientboundSetTimePacket -> 10 bytes
[11:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[11:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
--- exceptions (Realms/RTKit noise filtered) ---
25:[11:44:58] [Render thread/ERROR]: Error parsing option value 4 for option Simulation Distance: Value 4 outside of range [5:33]
48:[11:44:59] [Download-2/ERROR]: Failed to fetch user properties
49:com.mojang.authlib.exceptions.InvalidCredentialsException: Status: 401
50:	at com.mojang.authlib.exceptions.MinecraftClientHttpException.toAuthenticationException(MinecraftClientHttpException.java:70)
57:Caused by: MinecraftClientHttpException[type=HTTP_ERROR, status=401, response=ErrorResponse[path=/player/attributes, error=null, errorMessage=null, details=null]]
58:	at com.mojang.authlib.minecraft.client.MinecraftClient.buildHttpException(MinecraftClient.java:170)
```
### server log tail
```
[11:46:19 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:overworld'
[11:46:19 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[11:46:19 INFO]: [ChunkHolderManager] Waiting 60s for chunk system to halt for world 'minecraft:the_nether'
[11:46:19 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:the_nether'
[11:46:19 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_nether'
[11:46:19 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_nether' in 0.00s
[11:46:19 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_nether'
[11:46:19 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_nether'
[11:46:19 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[11:46:19 INFO]: [ChunkHolderManager] Waiting 60s for chunk system to halt for world 'minecraft:the_end'
[11:46:19 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:the_end'
[11:46:19 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_end'
[11:46:19 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_end' in 0.00s
[11:46:19 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_end'
[11:46:19 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_end'
[11:46:19 INFO]: Waiting for all RegionFile I/O tasks to complete...
[11:46:19 INFO]: All RegionFile I/O tasks to complete
[11:46:19 INFO]: [MoonriseCommon] Awaiting termination of worker pool for up to 60s...
[11:46:19 INFO]: [MoonriseCommon] Awaiting termination of I/O pool for up to 60s...
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[c-LoD] engine tick alive: tick 850
[c-LoD] EB: enter, native_gen=true
[11:46:19 INFO]: [c-LoD] engine tick alive: tick 850
[launcher] server exited with code 143
```
### shots
```
-rw-r--r-- 1 runner runner  12395 Nov 17 11:05 lens_12_after_edit_probe3.png
-rw-r--r-- 1 runner runner  12123 Nov 17 11:09 lens_12_after_edit_probe30.png
-rw-r--r-- 1 runner runner  12848 Nov 17 11:09 lens_12_after_edit_probe31.png
-rw-r--r-- 1 runner runner  12337 Nov 17 11:09 lens_12_after_edit_probe32.png
-rw-r--r-- 1 runner runner  11826 Nov 17 11:09 lens_12_after_edit_probe33.png
-rw-r--r-- 1 runner runner  11948 Nov 17 11:10 lens_12_after_edit_probe34.png
-rw-r--r-- 1 runner runner  12546 Nov 17 11:10 lens_12_after_edit_probe35.png
-rw-r--r-- 1 runner runner  12482 Nov 17 11:05 lens_12_after_edit_probe4.png
-rw-r--r-- 1 runner runner  12771 Nov 17 11:05 lens_12_after_edit_probe5.png
-rw-r--r-- 1 runner runner  12678 Nov 17 11:06 lens_12_after_edit_probe6.png
-rw-r--r-- 1 runner runner  13456 Nov 17 11:06 lens_12_after_edit_probe7.png
-rw-r--r-- 1 runner runner  12365 Nov 17 11:06 lens_12_after_edit_probe8.png
-rw-r--r-- 1 runner runner  12472 Nov 17 11:06 lens_12_after_edit_probe9.png
-rw-r--r-- 1 runner runner   2658 Nov 17 11:18 lens_13_after_relog_probe0.png
-rw-r--r-- 1 runner runner  11016 Nov 17 11:18 lens_13_after_relog_probe1.png
-rw-r--r-- 1 runner runner  11130 Nov 17 11:18 lens_13_after_relog_probe2.png
-rw-r--r-- 1 runner runner  12132 Nov 17 11:19 lens_13_after_relog_probe3.png
drwxr-xr-x 2 runner runner   4096 Nov 17 11:41 lens_parity
drwxr-xr-x 2 runner runner   4096 Nov 17 11:41 parity
drwxr-xr-x 2 runner runner   4096 Nov 17 11:12 parity_dbg
drwxr-xr-x 2 runner runner   4096 Nov 17 11:44 parity_dbg8
-rw-r--r-- 1 runner runner   8980 Nov 17 11:19 probe_counts.txt
drwxr-xr-x 2 runner runner   4096 Nov 17 11:46 rim_mask
-rw-r--r-- 1 runner runner  55672 Nov 17 11:46 server_full.log
-rw-r--r-- 1 runner runner  66297 Nov 17 11:46 summary.md
```

**exit=1** failures=1 leg1=0 leg2=1
