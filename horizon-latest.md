- phase: 0 sanity (18:36:02)
### c-LoD visual run (-930,796, marker=red_concrete, vantage -1005,150 yaw 270)
- GL:     Vendor: Mesa (0xffffffff);    Device: llvmpipe (LLVM 21.1.8, 256 bits) (0xffffffff);OpenGL renderer string: llvmpipe (LLVM 21.1.8, 256 bits);OpenGL core profile version string: 4.5 (Core Profile) Mesa 26.0.8-1ubuntu0.3;
- boot + gates OK (18:36:29)
- phase: 1 server booted (18:36:29)
- sky-grid armed
- phase: 2 leg-1 client joined (18:36:44)
- joined session 1 (18:36:44)
- stream live: cells=65536
- phase: 3 scan populated (18:37:08)
- terrain range: 2 terraced mountains at (CX-4,CZ-20)/(CX-2,CZ+16), tops 185/190, grass terraces + stone caps, 66..82 m from the eye (inside the 83.5 m band, chunk edge 64 m)
- scan complete=1 (cells=cells=65536/65536)
- paint settled=1 (wire 522483p)
- baseline try 1: drift=11592 px
- baseline try 2: drift=19532 px
- baseline try 3: drift=22665 px
- baseline try 4: drift=22384 px
- baseline try 5: drift=20703 px
- baseline try 6: drift=18436 px
- **FAIL:** no stable baseline after 6 tries (drift=18436) — the A/B would lie
- lod_silhouette_cols=61 (LOD paint ON SCREEN at 75 m if > 20 — the march-drawn marker column; vanilla cannot render there, chunk edge 64 m; #255 re-measures 49, the ZONE_FAR=48 regression #256 and the control read 0)
- 184 terrain-green px in the sky region (LOD MOUNTAINS on screen if > 5000; the range paints ColLow-green on low terraces via the fallback palette; flat-world baselines and the pack-free control read ~0)
- burst_close white px: 2167 (renderer alive if > 50)
- burst_far white px: 1633 (renderer alive in the marker zone if > 20)
- cut walk 40 m: 72 white px (> 20 = rendered)
- cut walk 60 m: 33 white px (> 20 = rendered)
- cut walk 100 m: 0 white px (> 20 = rendered)
- measured far-render cut: 60 m (G4 flap instrument; the decode proof needs >= 83 m from the y=151.6 eye)
- fill issued: 5x111x5 red_concrete at -930,796 (zone forceloaded ±48)
- phase: 4 marker filled (18:44:32)

### /clod-cells AFTER FILL (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 8 0 0x8400 0x2000
123 122 -940 784 71 0 1 [97, 119, 97] 8 0 0x8700 0x2000
124 122 -938 784 68 0 1 [97, 119, 97] 7 0 0x8400 0x2000
125 122 -936 784 62 1 1 [63, 118, 228] 47 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 46 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 45 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 45 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 340 0 0x8000 0x4100
130 122 -926 784 66 0 1 [134, 183, 131] 340 0 0x8200 0x4100
131 122 -924 784 75 0 1 [97, 119, 97] 332 0 0x8b00 0x2000
132 122 -922 784 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 122 -920 784 75 0 1 [97, 119, 97] 18446744073709551615 1 0x8b00 0x2000
134 122 -918 784 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x2000
122 123 -942 786 70 0 1 [97, 119, 97] 8 0 0x8600 0x2000
123 123 -940 786 65 0 1 [97, 119, 97] 8 0 0x8100 0x2000
124 123 -938 786 65 0 1 [97, 119, 97] 7 0 0x8100 0x2000
125 123 -936 786 62 1 1 [63, 118, 228] 47 0 0x7e00 0xd800
126 123 -934 786 62 1 1 [63, 118, 228] 46 0 0x7e00 0xd800
127 123 -932 786 62 1 1 [63, 118, 228] 46 0 0x7e00 0xd800
128 123 -930 786 62 1 1 [63, 118, 228] 45 0 0x7e00 0xd800
129 123 -928 786 73 0 1 [97, 119, 97] 340 0 0x8900 0x2000
130 123 -926 786 67 0 1 [134, 183, 131] 340 0 0x8300 0x4100
131 123 -924 786 69 0 1 [134, 183, 131] 340 0 0x8500 0x4100
132 123 -922 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 123 -920 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 123 -918 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
122 124 -942 788 72 0 1 [97, 119, 97] 8 0 0x8800 0x2000
123 124 -940 788 70 0 1 [97, 119, 97] 8 0 0x8600 0x2000
124 124 -938 788 68 0 1 [97, 119, 97] 7 0 0x8400 0x2000
125 124 -936 788 62 1 1 [63, 118, 228] 47 0 0x7e00 0xd800
126 124 -934 788 62 1 1 [63, 118, 228] 46 0 0x7e00 0xd800
127 124 -932 788 62 1 1 [63, 118, 228] 46 0 0x7e00 0xd800
128 124 -930 788 73 0 1 [97, 119, 97] 5 0 0x8900 0x2000
129 124 -928 788 76 0 1 [97, 119, 97] 340 0 0x8c00 0x2000
130 124 -926 788 73 0 1 [97, 119, 97] 340 0 0x8900 0x2000
131 124 -924 788 69 0 1 [134, 183, 131] 340 0 0x8500 0x4100
132 124 -922 788 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 124 -920 788 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 124 -918 788 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x2000
122 125 -942 790 74 0 1 [97, 119, 97] 8 0 0x8a00 0x2000
123 125 -940 790 73 0 1 [97, 119, 97] 8 0 0x8900 0x2000
124 125 -938 790 71 0 1 [97, 119, 97] 7 0 0x8700 0x2000
125 125 -936 790 62 1 1 [63, 118, 228] 47 0 0x7e00 0xd800
126 125 -934 790 62 1 1 [63, 118, 228] 46 0 0x7e00 0xd800
127 125 -932 790 62 1 1 [63, 118, 228] 46 0 0x7e00 0xd800
128 125 -930 790 62 1 1 [63, 118, 228] 45 0 0x7e00 0xd800
129 125 -928 790 73 0 1 [97, 119, 97] 5 0 0x8900 0x2000
130 125 -926 790 68 0 1 [134, 183, 131] 340 0 0x8400 0x4100
131 125 -924 790 69 0 1 [134, 183, 131] 340 0 0x8500 0x4100
132 125 -922 790 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
133 125 -920 790 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 125 -918 790 70 0 1 [134, 183, 131] 18446744073709551615 1 0x8600 0x4100
122 126 -942 792 74 0 1 [97, 119, 97] 8 0 0x8a00 0x2000
123 126 -940 792 73 0 1 [97, 119, 97] 8 0 0x8900 0x2000
124 126 -938 792 68 0 1 [97, 119, 97] 7 0 0x8400 0x2000
125 126 -936 792 62 1 1 [63, 118, 228] 47 0 0x7e00 0xd800
126 126 -934 792 62 1 1 [63, 118, 228] 46 0 0x7e00 0xd800
127 126 -932 792 62 1 1 [63, 118, 228] 46 0 0x7e00 0xd800
128 126 -930 792 62 1 1 [63, 118, 228] 45 0 0x7e00 0xd800
129 126 -928 792 67 0 1 [134, 183, 131] 5 0 0x8300 0x4100
130 126 -926 792 74 0 1 [97, 119, 97] 340 0 0x8a00 0x2000
131 126 -924 792 77 0 1 [97, 119, 97] 340 0 0x8d00 0x2000
132 126 -922 792 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 126 -920 792 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3200
134 126 -918 792 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x5000
122 127 -942 794 67 0 1 [134, 183, 131] 8 0 0x8300 0x4100
123 127 -940 794 68 0 1 [134, 183, 131] 8 0 0x8400 0x4100
124 127 -938 794 65 0 1 [134, 183, 131] 7 0 0x8100 0x4100
125 127 -936 794 62 1 1 [63, 118, 228] 47 0 0x7e00 0xd800
126 127 -934 794 62 1 1 [63, 118, 228] 46 0 0x7e00 0xd800
127 127 -932 794 62 1 1 [63, 118, 228] 46 0 0x7e00 0xd800
128 127 -930 794 62 1 1 [63, 118, 228] 45 0 0x7e00 0xd800
129 127 -928 794 66 0 1 [134, 183, 131] 5 0 0x8200 0x4100
130 127 -926 794 74 0 1 [97, 119, 97] 340 0 0x8a00 0x2000
131 127 -924 794 77 0 1 [97, 119, 97] 340 0 0x8d00 0x2000
132 127 -922 794 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 127 -920 794 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3200
134 127 -918 794 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x5000
122 128 -942 796 68 0 1 [134, 183, 131] 9 0 0x8400 0x4100
123 128 -940 796 72 0 1 [97, 119, 97] 8 0 0x8800 0x2000
124 128 -938 796 75 0 1 [97, 119, 97] 7 0 0x8b00 0x2000
125 128 -936 796 72 0 1 [97, 119, 97] 7 0 0x8800 0x2000
126 128 -934 796 62 1 1 [63, 118, 228] 46 0 0x7e00 0xd800
127 128 -932 796 62 1 1 [63, 118, 228] 46 0 0x7e00 0xd800
128 128 -930 796 62 1 1 [63, 118, 228] 45 0 0x7e00 0xd800
129 128 -928 796 66 0 1 [134, 183, 131] 5 0 0x8200 0x4100
130 128 -926 796 68 0 1 [134, 183, 131] 340 0 0x8400 0x4100
131 128 -924 796 74 0 1 [97, 119, 97] 340 0 0x8a00 0x2000
132 128 -922 796 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 128 -920 796 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
134 128 -918 796 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 129 -942 798 69 0 1 [134, 183, 131] 8 0 0x8500 0x4100
123 129 -940 798 75 0 1 [97, 119, 97] 8 0 0x8b00 0x2000
124 129 -938 798 78 0 1 [97, 119, 97] 9 0 0x8e00 0x2000
125 129 -936 798 75 0 1 [97, 119, 97] 7 0 0x8b00 0x2000
126 129 -934 798 62 1 1 [63, 118, 228] 46 0 0x7e00 0xd800
127 129 -932 798 62 1 1 [63, 118, 228] 46 0 0x7e00 0xd800
128 129 -930 798 62 1 1 [63, 118, 228] 45 0 0x7e00 0xd800
129 129 -928 798 71 0 1 [97, 119, 97] 5 0 0x8700 0x2000
130 129 -926 798 71 0 1 [97, 119, 97] 340 0 0x8700 0x2000
131 129 -924 798 69 0 1 [134, 183, 131] 340 0 0x8500 0x4100
132 129 -922 798 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 129 -920 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 129 -918 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 130 -942 800 70 0 1 [134, 183, 131] 8 0 0x8600 0x4100
123 130 -940 800 76 0 1 [97, 119, 97] 8 0 0x8c00 0x2000
124 130 -938 800 76 0 1 [97, 119, 97] 7 0 0x8c00 0x2000
125 130 -936 800 72 0 1 [97, 119, 97] 7 0 0x8800 0x2000
126 130 -934 800 62 1 1 [63, 118, 228] 46 0 0x7e00 0xd800
127 130 -932 800 62 1 1 [63, 118, 228] 46 0 0x7e00 0xd800
128 130 -930 800 62 1 1 [63, 118, 228] 45 0 0x7e00 0xd800
129 130 -928 800 71 0 1 [97, 119, 97] 5 0 0x8700 0x2000
130 130 -926 800 71 0 1 [97, 119, 97] 340 0 0x8700 0x2000
131 130 -924 800 69 0 1 [134, 183, 131] 340 0 0x8500 0x4100
132 130 -922 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
133 130 -920 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 130 -918 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 131 -942 802 78 0 1 [97, 119, 97] 10 0 0x8e00 0x2000
123 131 -940 802 77 0 1 [97, 119, 97] 8 0 0x8d00 0x2000
124 131 -938 802 77 0 1 [97, 119, 97] 7 0 0x8d00 0x2000
125 131 -936 802 66 0 1 [134, 183, 131] 7 0 0x8200 0x4100
126 131 -934 802 62 1 1 [63, 118, 228] 46 0 0x7e00 0xd800
127 131 -932 802 62 1 1 [63, 118, 228] 46 0 0x7e00 0xd800
128 131 -930 802 62 1 1 [63, 118, 228] 45 0 0x7e00 0xd800
129 131 -928 802 62 1 1 [63, 118, 228] 372 0 0x7e00 0xd800
130 131 -926 802 66 0 1 [134, 183, 131] 340 0 0x8200 0x4100
131 131 -924 802 68 0 1 [134, 183, 131] 340 0 0x8400 0x4100
132 131 -922 802 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 131 -920 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 131 -918 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 132 -942 804 79 0 1 [97, 119, 97] 10 0 0x8f00 0x2000
123 132 -940 804 76 0 1 [97, 119, 97] 8 0 0x8c00 0x2000
124 132 -938 804 76 0 1 [97, 119, 97] 7 0 0x8c00 0x2000
125 132 -936 804 76 0 1 [97, 119, 97] 7 0 0x8c00 0x2000
126 132 -934 804 62 1 1 [63, 118, 228] 46 0 0x7e00 0xd800
127 132 -932 804 62 1 1 [63, 118, 228] 46 0 0x7e00 0xd800
128 132 -930 804 62 1 1 [63, 118, 228] 45 0 0x7e00 0xd800
129 132 -928 804 62 1 1 [63, 118, 228] 372 0 0x7e00 0xd800
130 132 -926 804 65 0 1 [134, 183, 131] 339 0 0x8100 0x4100
131 132 -924 804 77 0 1 [97, 119, 97] 339 0 0x8d00 0x2000
132 132 -922 804 79 0 1 [97, 119, 97] 339 0 0x8f00 0x2000
133 132 -920 804 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 132 -918 804 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x2000
122 133 -942 806 78 0 1 [97, 119, 97] 10 0 0x8e00 0x2000
123 133 -940 806 69 0 1 [134, 183, 131] 8 0 0x8500 0x4100
124 133 -938 806 155 0 1 [134, 183, 131] 8 0 0xdb00 0x4100
125 133 -936 806 155 0 1 [134, 183, 131] 8 0 0xdb00 0x4100
126 133 -934 806 155 0 1 [134, 183, 131] 7 0 0xdb00 0x4100
127 133 -932 806 155 0 1 [134, 183, 131] 7 0 0xdb00 0x4100
128 133 -930 806 155 0 1 [134, 183, 131] 6 0 0xdb00 0x4100
129 133 -928 806 155 0 1 [134, 183, 131] 5 0 0xdb00 0x4100
130 133 -926 806 155 0 1 [134, 183, 131] 5 0 0xdb00 0x4100
131 133 -924 806 67 0 1 [134, 183, 131] 332 0 0x8300 0x4100
132 133 -922 806 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 133 -920 806 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 133 -918 806 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x2000
122 134 -942 808 79 0 1 [97, 119, 97] 10 0 0x8f00 0x2000
123 134 -940 808 68 0 1 [134, 183, 131] 8 0 0x8400 0x4100
124 134 -938 808 155 0 1 [134, 183, 131] 8 0 0xdb00 0x4100
125 134 -936 808 180 0 1 [134, 183, 131] 5 0 0xf400 0x4100
126 134 -934 808 180 0 1 [134, 183, 131] 5 0 0xf400 0x4100
127 134 -932 808 180 0 1 [134, 183, 131] 339 0 0xf400 0x4100
128 134 -930 808 180 0 1 [134, 183, 131] 339 0 0xf400 0x4100
129 134 -928 808 180 0 1 [134, 183, 131] 332 0 0xf400 0x4100
130 134 -926 808 155 0 1 [134, 183, 131] 5 0 0xdb00 0x4100
131 134 -924 808 67 0 1 [134, 183, 131] 332 0 0x8300 0x4100
132 134 -922 808 68 0 1 [134, 183, 131] 18446744073709551615 1 0x8400 0x4100
133 134 -920 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 134 -918 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
```
### stream diag after fill:
[c-LoD] stream: tick 4900 65536/65536 cells, lap 787, cone 26216 zone 2283 dirty 950 (q 11753) ewin 167 rbeat 9p/1 rearm 0c/0t rescan 1257983c/1972e, edits +4900p/3674c/639e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 319 forced 193, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 734950p/2197b fail 0 ~1 kB/s cache 13605/24576 h721345 m13605 e0, srbeat 62714s/9q drop 62stale/63undec, ereb 47922s/9q, sweep 0r/50d
[18:44:39 INFO]: [c-LoD] stream: tick 4900 65536/65536 cells, lap 787, cone 26216 zone 2283 dirty 950 (q 11753) ewin 167 rbeat 9p/1 rearm 0c/0t rescan 1257983c/1972e, edits +4900p/3674c/639e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 319 forced 193, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 734950p/2197b fail 0 ~1 kB/s cache 13605/24576 h721345 m13605 e0, srbeat 62714s/9q drop 62stale/63undec, ereb 47922s/9q, sweep 0r/50d
- carrier ladder y=155: 13468 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=165: 13472 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=168: 13480 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=172: 13192 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=176: 13564 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=180: 13272 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=185: 13648 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- probe 0: red=16054 paint=125223 lod_silhouette_cols=91 lens_red=0 lens_black=0 lens_water=0 lens_dark=0 lens_zero=0 rot=none scan=0 carriers=0 (need red>=60 paint>=300 lod>20)
- **LEG 1 PASS** (zero relogs)
- phase: 5 leg-1 probes done (18:45:07)
### stream diag after probes:
[c-LoD] stream: tick 5000 65536/65536 cells, lap 787, cone 26216 zone 2283 dirty 959 (q 11767) ewin 167 rbeat 9p/0 rearm 0c/0t rescan 1287647c/1990e, edits +5114p/3836c/712e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 319 forced 193, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 735679p/2207b fail 0 ~3 kB/s cache 13605/24576 h722074 m13605 e0, srbeat 62795s/9q drop 62stale/63undec, ereb 47994s/9q, sweep 0r/50d
[c-LoD] stream: tick 5100 65536/65536 cells, lap 787, cone 26216 zone 2283 dirty 968 (q 11770) ewin 167 rbeat 0p/0 rearm 0c/0t rescan 1322551c/1999e, edits +5249p/3927c/733e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 319 forced 193, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 736408p/2217b fail 0 ~3 kB/s cache 13605/24576 h722803 m13605 e0, srbeat 62876s/9q drop 62stale/63undec, ereb 48066s/9q, sweep 0r/50d
[18:44:59 INFO]: [c-LoD] stream: tick 5100 65536/65536 cells, lap 787, cone 26216 zone 2283 dirty 968 (q 11770) ewin 167 rbeat 0p/0 rearm 0c/0t rescan 1322551c/1999e, edits +5249p/3927c/733e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 319 forced 193, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 736408p/2217b fail 0 ~3 kB/s cache 13605/24576 h722803 m13605 e0, srbeat 62876s/9q drop 62stale/63undec, ereb 48066s/9q, sweep 0r/50d
- parity debug tour: 13 vantages (session-1 client, DEBUG pack — evidence only, the gate pairs the §7b world session)
- parity debug tour done: 13 shots in parity_dbg/ (the §7b world session feeds the gate)
- joined session 2 (18:47:49)
- leg-2 baseline scene px (x<940): 24860 (0 = the relog client paints no world at all)
- leg-2 renderer probe NEAR (20 m) white px: 2228 raw / 2721 changed-vs-baseline (> 20 = the client renders particles)
- leg-2 renderer probe FAR (80 m, marker zone) white px: 1708 raw / 4783 changed-vs-baseline (> 20 = no distance cut; ~0 with NEAR alive = particles beyond the client chunk radius do not render)
- leg-2 carrier probe green dust (80 m): changed-green px 0 / total changed 24774 vs baseline (> 100 green = a stable green carrier exists; total > 0 with green ~0 = the dust renders but fog washes it — a ruler problem, not a carrier problem)
- phase: 8 leg-2 relog probes (18:53:16)
- relog probe 0: red=15867 scene=29501 lens_red=0 lens_black=6 lens_water=0 lens_dark=0 lens_zero=0 rot=decoded cam_x_b=? cam_z_b=? az_b=? el_b=? fog=live fog_end_b=? strobe_hits=22 hits_hi=22 mask_hits=22 (need red >= 60; scene > 0 = the client paints a world at all)
- **LEG 2 PASS** (marker persisted over relog)
- phase: 9 leg-2 done (18:53:19)

### client log forensics (pipeline/particle/atlas — v6.8.2)
```
Horizon renderer: Vulkan has no conformant adapter (Vulkan: only a software adapter (llvmpipe (LLVM 21.1.8, 256 bits))); asking again for the ones wgpu hides as non-conformant
Horizon renderer: trying OpenGL (Vulkan: only a software adapter (llvmpipe (LLVM 21.1.8, 256 bits)))
libEGL warning: DRI3 error: Could not get DRI3 device
Horizon: ignoring minecraft:ambient_entity_effect: no such vanilla particle in this version
Horizon renderer: OpenGL has no conformant adapter (OpenGL: only a software adapter (llvmpipe (LLVM 21.1.8, 256 bits))); asking again for the ones wgpu hides as non-conformant
libEGL warning: DRI3 error: Could not get DRI3 device
Horizon renderer: llvmpipe (LLVM 21.1.8, 256 bits) (Vulkan, Cpu)
Horizon renderer: requesting the device (driver llvmpipe Mesa 26.0.8-1ubuntu0.3 (LLVM 21.1.8)) with features [TEXTURE_FORMAT_16BIT_NORM | TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES | PIPELINE_STATISTICS_QUERY | TIMESTAMP_QUERY_INSIDE_ENCODERS | TIMESTAMP_QUERY_INSIDE_PASSES | MAPPABLE_PRIMARY_BUFFERS | MULTI_DRAW_INDIRECT_COUNT | VERTEX_WRITABLE_STORAGE | CLEAR_TEXTURE | TEXTURE_ATOMIC | SHADER_EARLY_DEPTH_TEST | SUBGROUP | TIMESTAMP_QUERY | INDIRECT_FIRST_INSTANCE | RG11B10UFLOAT_RENDERABLE | FLOAT32_FILTERABLE | FLOAT32_BLENDABLE] and the adapter's limits (buffers up to 2147483647 bytes, storage bindings up to 134217728 bytes, 2048 texture layers)
Horizon renderer: the chosen graphics API gave no GPU (No Vulkan or OpenGL graphics driver found: Horizon needs the Vulkan loader (libvulkan.so.1) and a Vulkan driver for your GPU (Mesa's, or NVIDIA's), or OpenGL through EGL (Vulkan: only a software adapter (llvmpipe (LLVM 21.1.8, 256 bits)); OpenGL: only a software adapter (llvmpipe (LLVM 21.1.8, 256 bits)))); rendering on the CPU, which is slow
Horizon renderer: creating the pipelines
Horizon renderer: pipelines ready (multi-draw on, occlusion culling off, GPU culling unavailable, GPU timer on)
Horizon renderer: llvmpipe (LLVM 21.1.8, 256 bits) (Vulkan, Cpu)
Horizon renderer: rebuilt on the same device (51 shader modules, layouts and pipelines kept)
Horizon renderer: llvmpipe (LLVM 21.1.8, 256 bits) (Vulkan, Cpu)
Horizon renderer: rebuilt on the same device (51 shader modules, layouts and pipelines kept)
Horizon renderer: llvmpipe (LLVM 21.1.8, 256 bits) (Vulkan, Cpu)
Horizon renderer: rebuilt on the same device (51 shader modules, layouts and pipelines kept)
```

### /clod-cells AFTER PROBES (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 3478 0 0x8400 0x2000
123 122 -940 784 71 0 1 [97, 119, 97] 3478 0 0x8700 0x2000
124 122 -938 784 68 0 1 [97, 119, 97] 3478 0 0x8400 0x2000
125 122 -936 784 62 1 1 [63, 118, 228] 3478 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 3478 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 3478 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 3478 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 3478 0 0x8000 0x4100
130 122 -926 784 66 0 1 [134, 183, 131] 3478 0 0x8200 0x4100
131 122 -924 784 75 0 1 [97, 119, 97] 3478 0 0x8b00 0x2000
132 122 -922 784 77 0 1 [97, 119, 97] 4045 1 0x8d00 0x2000
133 122 -920 784 75 0 1 [97, 119, 97] 4045 1 0x8b00 0x2000
134 122 -918 784 73 0 1 [97, 119, 97] 4046 1 0x8900 0x2000
122 123 -942 786 70 0 1 [97, 119, 97] 3478 0 0x8600 0x2000
123 123 -940 786 65 0 1 [97, 119, 97] 3478 0 0x8100 0x2000
124 123 -938 786 65 0 1 [97, 119, 97] 3478 0 0x8100 0x2000
125 123 -936 786 62 1 1 [63, 118, 228] 3478 0 0x7e00 0xd800
126 123 -934 786 62 1 1 [63, 118, 228] 3478 0 0x7e00 0xd800
127 123 -932 786 62 1 1 [63, 118, 228] 3478 0 0x7e00 0xd800
128 123 -930 786 62 1 1 [63, 118, 228] 3478 0 0x7e00 0xd800
129 123 -928 786 73 0 1 [97, 119, 97] 3478 0 0x8900 0x2000
130 123 -926 786 67 0 1 [134, 183, 131] 3478 0 0x8300 0x4100
131 123 -924 786 69 0 1 [134, 183, 131] 3478 0 0x8500 0x4100
132 123 -922 786 77 0 1 [97, 119, 97] 4046 1 0x8d00 0x2000
133 123 -920 786 77 0 1 [97, 119, 97] 4046 1 0x8d00 0x2000
134 123 -918 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
122 124 -942 788 72 0 1 [97, 119, 97] 3478 0 0x8800 0x2000
123 124 -940 788 70 0 1 [97, 119, 97] 3478 0 0x8600 0x2000
124 124 -938 788 68 0 1 [97, 119, 97] 3478 0 0x8400 0x2000
125 124 -936 788 62 1 1 [63, 118, 228] 3478 0 0x7e00 0xd800
126 124 -934 788 62 1 1 [63, 118, 228] 3478 0 0x7e00 0xd800
127 124 -932 788 62 1 1 [63, 118, 228] 3478 0 0x7e00 0xd800
128 124 -930 788 73 0 1 [97, 119, 97] 3478 0 0x8900 0x2000
129 124 -928 788 76 0 1 [97, 119, 97] 3478 0 0x8c00 0x2000
130 124 -926 788 73 0 1 [97, 119, 97] 3478 0 0x8900 0x2000
131 124 -924 788 69 0 1 [134, 183, 131] 3478 0 0x8500 0x4100
132 124 -922 788 77 0 1 [97, 119, 97] 4046 1 0x8d00 0x2000
133 124 -920 788 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 124 -918 788 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x2000
122 125 -942 790 74 0 1 [97, 119, 97] 3478 0 0x8a00 0x2000
123 125 -940 790 73 0 1 [97, 119, 97] 3478 0 0x8900 0x2000
124 125 -938 790 71 0 1 [97, 119, 97] 3478 0 0x8700 0x2000
125 125 -936 790 62 1 1 [63, 118, 228] 3478 0 0x7e00 0xd800
126 125 -934 790 62 1 1 [63, 118, 228] 3478 0 0x7e00 0xd800
127 125 -932 790 62 1 1 [63, 118, 228] 3478 0 0x7e00 0xd800
128 125 -930 790 62 1 1 [63, 118, 228] 3478 0 0x7e00 0xd800
129 125 -928 790 73 0 1 [97, 119, 97] 3478 0 0x8900 0x2000
130 125 -926 790 68 0 1 [134, 183, 131] 3478 0 0x8400 0x4100
131 125 -924 790 69 0 1 [134, 183, 131] 3478 0 0x8500 0x4100
132 125 -922 790 69 0 1 [134, 183, 131] 4045 1 0x8500 0x4100
133 125 -920 790 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 125 -918 790 70 0 1 [134, 183, 131] 18446744073709551615 1 0x8600 0x4100
122 126 -942 792 74 0 1 [97, 119, 97] 3478 0 0x8a00 0x2000
123 126 -940 792 73 0 1 [97, 119, 97] 3478 0 0x8900 0x2000
124 126 -938 792 68 0 1 [97, 119, 97] 3478 0 0x8400 0x2000
125 126 -936 792 62 1 1 [63, 118, 228] 3478 0 0x7e00 0xd800
126 126 -934 792 62 1 1 [63, 118, 228] 3478 0 0x7e00 0xd800
127 126 -932 792 62 1 1 [63, 118, 228] 3478 0 0x7e00 0xd800
128 126 -930 792 62 1 1 [63, 118, 228] 3478 0 0x7e00 0xd800
129 126 -928 792 67 0 1 [134, 183, 131] 3478 0 0x8300 0x4100
130 126 -926 792 74 0 1 [97, 119, 97] 3478 0 0x8a00 0x2000
131 126 -924 792 77 0 1 [97, 119, 97] 3478 0 0x8d00 0x2000
132 126 -922 792 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 126 -920 792 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3200
134 126 -918 792 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x5000
122 127 -942 794 67 0 1 [134, 183, 131] 3477 0 0x8300 0x4100
123 127 -940 794 68 0 1 [134, 183, 131] 3477 0 0x8400 0x4100
124 127 -938 794 65 0 1 [134, 183, 131] 3477 0 0x8100 0x4100
125 127 -936 794 62 1 1 [63, 118, 228] 3477 0 0x7e00 0xd800
126 127 -934 794 62 1 1 [63, 118, 228] 3477 0 0x7e00 0xd800
127 127 -932 794 170 0 1 [142, 33, 33] 3461 0 0xea00 0x0a00
128 127 -930 794 170 0 1 [142, 33, 33] 3461 0 0xea00 0x0a00
129 127 -928 794 170 0 1 [142, 33, 33] 3461 0 0xea00 0x0a00
130 127 -926 794 74 0 1 [97, 119, 97] 3477 0 0x8a00 0x2000
131 127 -924 794 77 0 1 [97, 119, 97] 3477 0 0x8d00 0x2000
132 127 -922 794 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 127 -920 794 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3200
134 127 -918 794 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x5000
122 128 -942 796 68 0 1 [134, 183, 131] 3477 0 0x8400 0x4100
123 128 -940 796 72 0 1 [97, 119, 97] 3477 0 0x8800 0x2000
124 128 -938 796 75 0 1 [97, 119, 97] 3477 0 0x8b00 0x2000
125 128 -936 796 72 0 1 [97, 119, 97] 3477 0 0x8800 0x2000
126 128 -934 796 62 1 1 [63, 118, 228] 3477 0 0x7e00 0xd800
127 128 -932 796 170 0 1 [142, 33, 33] 3461 0 0xea00 0x0a00
128 128 -930 796 170 0 1 [142, 33, 33] 3461 0 0xea00 0x0a00
129 128 -928 796 170 0 1 [142, 33, 33] 3461 0 0xea00 0x0a00
130 128 -926 796 68 0 1 [134, 183, 131] 3477 0 0x8400 0x4100
131 128 -924 796 74 0 1 [97, 119, 97] 3477 0 0x8a00 0x2000
132 128 -922 796 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 128 -920 796 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
134 128 -918 796 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 129 -942 798 69 0 1 [134, 183, 131] 3477 0 0x8500 0x4100
123 129 -940 798 75 0 1 [97, 119, 97] 3477 0 0x8b00 0x2000
124 129 -938 798 78 0 1 [97, 119, 97] 3477 0 0x8e00 0x2000
125 129 -936 798 75 0 1 [97, 119, 97] 3477 0 0x8b00 0x2000
126 129 -934 798 62 1 1 [63, 118, 228] 3477 0 0x7e00 0xd800
127 129 -932 798 170 0 1 [142, 33, 33] 3461 0 0xea00 0x0a00
128 129 -930 798 170 0 1 [142, 33, 33] 3461 0 0xea00 0x0a00
129 129 -928 798 170 0 1 [142, 33, 33] 3461 0 0xea00 0x0a00
130 129 -926 798 71 0 1 [97, 119, 97] 3477 0 0x8700 0x2000
131 129 -924 798 69 0 1 [134, 183, 131] 3477 0 0x8500 0x4100
132 129 -922 798 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 129 -920 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 129 -918 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 130 -942 800 70 0 1 [134, 183, 131] 3477 0 0x8600 0x4100
123 130 -940 800 76 0 1 [97, 119, 97] 3477 0 0x8c00 0x2000
124 130 -938 800 76 0 1 [97, 119, 97] 3477 0 0x8c00 0x2000
125 130 -936 800 72 0 1 [97, 119, 97] 3477 0 0x8800 0x2000
126 130 -934 800 62 1 1 [63, 118, 228] 3477 0 0x7e00 0xd800
127 130 -932 800 62 1 1 [63, 118, 228] 3477 0 0x7e00 0xd800
128 130 -930 800 62 1 1 [63, 118, 228] 3477 0 0x7e00 0xd800
129 130 -928 800 71 0 1 [97, 119, 97] 3477 0 0x8700 0x2000
130 130 -926 800 71 0 1 [97, 119, 97] 3477 0 0x8700 0x2000
131 130 -924 800 69 0 1 [134, 183, 131] 3477 0 0x8500 0x4100
132 130 -922 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
133 130 -920 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 130 -918 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 131 -942 802 78 0 1 [97, 119, 97] 3477 0 0x8e00 0x2000
123 131 -940 802 77 0 1 [97, 119, 97] 3477 0 0x8d00 0x2000
124 131 -938 802 77 0 1 [97, 119, 97] 3477 0 0x8d00 0x2000
125 131 -936 802 66 0 1 [134, 183, 131] 3477 0 0x8200 0x4100
126 131 -934 802 62 1 1 [63, 118, 228] 3477 0 0x7e00 0xd800
127 131 -932 802 62 1 1 [63, 118, 228] 3477 0 0x7e00 0xd800
128 131 -930 802 62 1 1 [63, 118, 228] 3477 0 0x7e00 0xd800
129 131 -928 802 62 1 1 [63, 118, 228] 3477 0 0x7e00 0xd800
130 131 -926 802 66 0 1 [134, 183, 131] 3477 0 0x8200 0x4100
131 131 -924 802 68 0 1 [134, 183, 131] 3477 0 0x8400 0x4100
132 131 -922 802 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 131 -920 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 131 -918 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 132 -942 804 79 0 1 [97, 119, 97] 3477 0 0x8f00 0x2000
123 132 -940 804 76 0 1 [97, 119, 97] 3477 0 0x8c00 0x2000
124 132 -938 804 76 0 1 [97, 119, 97] 3477 0 0x8c00 0x2000
125 132 -936 804 76 0 1 [97, 119, 97] 3477 0 0x8c00 0x2000
126 132 -934 804 62 1 1 [63, 118, 228] 3477 0 0x7e00 0xd800
127 132 -932 804 62 1 1 [63, 118, 228] 3477 0 0x7e00 0xd800
128 132 -930 804 62 1 1 [63, 118, 228] 3477 0 0x7e00 0xd800
129 132 -928 804 62 1 1 [63, 118, 228] 3477 0 0x7e00 0xd800
130 132 -926 804 65 0 1 [134, 183, 131] 3477 0 0x8100 0x4100
131 132 -924 804 77 0 1 [97, 119, 97] 3477 0 0x8d00 0x2000
132 132 -922 804 79 0 1 [97, 119, 97] 3477 0 0x8f00 0x2000
133 132 -920 804 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 132 -918 804 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x2000
122 133 -942 806 78 0 1 [97, 119, 97] 3477 0 0x8e00 0x2000
123 133 -940 806 69 0 1 [134, 183, 131] 3477 0 0x8500 0x4100
124 133 -938 806 155 0 1 [134, 183, 131] 3477 0 0xdb00 0x4100
125 133 -936 806 155 0 1 [134, 183, 131] 3477 0 0xdb00 0x4100
126 133 -934 806 155 0 1 [134, 183, 131] 3477 0 0xdb00 0x4100
127 133 -932 806 155 0 1 [134, 183, 131] 3477 0 0xdb00 0x4100
128 133 -930 806 155 0 1 [134, 183, 131] 3477 0 0xdb00 0x4100
129 133 -928 806 155 0 1 [134, 183, 131] 3477 0 0xdb00 0x4100
130 133 -926 806 155 0 1 [134, 183, 131] 3477 0 0xdb00 0x4100
131 133 -924 806 67 0 1 [134, 183, 131] 3477 0 0x8300 0x4100
132 133 -922 806 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 133 -920 806 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 133 -918 806 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x2000
122 134 -942 808 79 0 1 [97, 119, 97] 3477 0 0x8f00 0x2000
123 134 -940 808 68 0 1 [134, 183, 131] 3477 0 0x8400 0x4100
124 134 -938 808 155 0 1 [134, 183, 131] 3477 0 0xdb00 0x4100
125 134 -936 808 180 0 1 [134, 183, 131] 3477 0 0xf400 0x4100
126 134 -934 808 180 0 1 [134, 183, 131] 3477 0 0xf400 0x4100
127 134 -932 808 180 0 1 [134, 183, 131] 3477 0 0xf400 0x4100
128 134 -930 808 180 0 1 [134, 183, 131] 3477 0 0xf400 0x4100
129 134 -928 808 180 0 1 [134, 183, 131] 3477 0 0xf400 0x4100
130 134 -926 808 155 0 1 [134, 183, 131] 3477 0 0xdb00 0x4100
131 134 -924 808 67 0 1 [134, 183, 131] 3477 0 0x8300 0x4100
132 134 -922 808 68 0 1 [134, 183, 131] 18446744073709551615 1 0x8400 0x4100
133 134 -920 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 134 -918 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
```
- edit channel: 0 linkage errors, 0 failed event dispatches (both must be 0)
- phase: 6 parity world boot (18:53:19)
- parity world: view-distance=8 (the voxy leg's server value)
- parity world: joined session 3 (18:53:50)
- parity world: scan populated (cells=65536; the march paints from these buffers)
- parity world tour: 13 vantages (production pack, vd 8, shared structures)
- coverage sweeps: 72 eye passes = 36 positions (12 az x r144/200 + 12 off-az x r172, y=160) x 2 looks (outward+inward), settle-by-laps >= 6 s (q stable 3x1s), deadline 12 s/pos, no shots — the v6.26.0 data-coverage lever + v6.26.6 lottery kill; probe rides parity/view_probe.md
- parity drain-settle TIMEOUT: queue 22298 -> 22203 > 11500 after 150s — continuing anyway; the queue did not drain without eye proximity (priority-lane evidence, the 01/05/10 dip attribution reads from the tour deltas)
- pre-tour pass done: 13 vantages x 10s (the tour now reads the second OR-merge lap; the rim/dip transport race reads from the tour deltas)
- **DAY GUARD:** 01_aerial_west still night after re-pin+reshoot (zenith < 130) — renamed night_01_aerial_west.png; the row pairs as missing, NOT as a renderer diff
- **DAY GUARD:** 02_aerial_south still night after re-pin+reshoot (zenith < 130) — renamed night_02_aerial_south.png; the row pairs as missing, NOT as a renderer diff
- **DAY GUARD:** 03_aerial_north still night after re-pin+reshoot (zenith < 130) — renamed night_03_aerial_north.png; the row pairs as missing, NOT as a renderer diff
- **DAY GUARD:** 04_aerial_east still night after re-pin+reshoot (zenith < 130) — renamed night_04_aerial_east.png; the row pairs as missing, NOT as a renderer diff
- **DAY GUARD:** 06_ground_west still night after re-pin+reshoot (zenith < 130) — renamed night_06_ground_west.png; the row pairs as missing, NOT as a renderer diff
- **DAY GUARD:** 08_back_inside still night after re-pin+reshoot (zenith < 130) — renamed night_08_back_inside.png; the row pairs as missing, NOT as a renderer diff
- **DAY GUARD:** 09_after_mesh still night after re-pin+reshoot (zenith < 130) — renamed night_09_after_mesh.png; the row pairs as missing, NOT as a renderer diff
- parity world tour done: 13 shots, 7 night-guard renames in parity/ (the gate pairs voxy/<name>.png vs parity/<name>.png)
- phase: 7 parity tour done (19:17:02)
- sparse re-beat channel at tour end: streamed=118475 q=0 dropped_stale=78 dropped_undecodable=11982 (frac=8 boot; undecodable-dominant = the rim words die in their decodability window, stale-dominant = edit churn owns the cells)
- **FAIL:** parity world time-of-day uncontrollable: 7 vantage(s) night after re-pin+reshoot — the §7b noon-pin class is broken on this runner; the matrix is infrastructure-blocked, not a renderer verdict (DAY GUARD lines + the feed's time-audit.md carry the evidence)
### parity world stream counters (session 3, tail + maxima)
[c-LoD] stream: tick 14000 65536/65536 cells, lap 1262, cone 68155 zone 26255 dirty 15823 (q 22179) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 9565736c/2320e, edits +18098p/13715c/2845e drop 0, spawned 0 / failed 0, cam0 yaw -180 pitch 5, wish 0 forced 923, native 39351c/956ch fail 0 (3w q 0) helper on, trees 956ch/fail 0, wire bundle 1792925p/4108b fail 0 ~0 kB/s cache 20997/24576 h1347192 m445733 e424736, srbeat 118475s/0q drop 78stale/11982undec, ereb 62334s/1q, sweep 0r/22d
[19:17:01 INFO]: [c-LoD] stream: tick 14000 65536/65536 cells, lap 1262, cone 68155 zone 26255 dirty 15823 (q 22179) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 9565736c/2320e, edits +18098p/13715c/2845e drop 0, spawned 0 / failed 0, cam0 yaw -180 pitch 5, wish 0 forced 923, native 39351c/956ch fail 0 (3w q 0) helper on, trees 956ch/fail 0, wire bundle 1792925p/4108b fail 0 ~0 kB/s cache 20997/24576 h1347192 m445733 e424736, srbeat 118475s/0q drop 78stale/11982undec, ereb 62334s/1q, sweep 0r/22d
- parity stream max cone: 68155
- parity stream max zone: 26255
- parity stream max wire bundle: bundle
### parity world capture lens (debug chain on the §7b world)
lens_red=? lens_black=? lens_water=? lens_dark=? lens_zero=? rot=? scan=? carriers=?
- mask8: the un-fog engagement masks shot on the PARITY WORLD (vd 8) — parity_dbg8/01m|07m|10m
- mask8 01m: green=25552 red=0 blue=203120 dim=375536 (px, sampled /16)
- mask8 07m: green=9360 red=0 blue=148080 dim=10800 (px, sampled /16)
- mask8 10m: green=3936 red=0 blue=128752 dim=144608 (px, sampled /16)
- mesh probe @png_y=2: x306=(83,102,141) x307=(83,102,141) x308=(83,102,141) x309h=(83,102,141) x310c=(83,102,141)
- mesh probe @png_y=717: x306=(25,36,29) x307=(26,38,30) x308=(25,35,29) x309h=(15,20,19) x310c=(27,39,31)
- mesh engagement: magenta-ish=1504 of 921600 sampled px (the vanilla-path paint coverage)
- mask8 fog state @png_y=2: raw (300)=(82,100,139) (301)=(83,101,140)
- mask8 fog state decoded: envStart=656 envEnd=800 rdEnd=2224 fogColor=(83,101,140)
- mask8 fog state @png_y=717: raw (300)=(26,37,30) (301)=(23,33,27)
- mask8 fog state decoded: envStart=208 envEnd=296 rdEnd=480 fogColor=(23,33,27)
- buffer dump 10m: marked h=2048/2048 c=2048/2048; height windows r0<142:0 r1:2048 r2>=202:0 maxByte=178
- rim mask: scan populated (cells=65536; the state paint reads the same buffers the march reads)
- phase: 10 rim mask lane (19:22:43)
- rim mask: decode-state frames shot (rim_mask/00|01|12 — GREEN full decode / RED colorless / BLUE heightless; 00 = the leg-1 tower eye, 01 = the near-tower contrast, 12 = the tower re-read; the inherited 07/12 vantages sat 760 blocks outside the streamed disk and 01 sat INSIDE the marker column — their earlier verdicts were classifier noise, purged in v6.33.1)
- rim mask 00: green=288 red=224 blue=0 yellow=0 (px, sampled /16) -> no state paint (the march owns too few pixels — check the vantage/scan)
- rim mask 01: green=1360 red=464 blue=1088 yellow=0 (px, sampled /16) -> DOMINANT GREEN: rim fully decoded — the fallback is the PAINT path (shade/fog/composite), not data
- rim mask 12: green=672 red=224 blue=0 yellow=0 (px, sampled /16) -> DOMINANT GREEN: rim fully decoded — the fallback is the PAINT path (shade/fog/composite), not data

### boot / stream
```
[c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 143229 bytes, sha1 d25a1161c1d1c6b6f86482ec981b01c009043b33 — zero textures, zero world data, one push per join
[19:20:28 INFO]: [c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 143229 bytes, sha1 d25a1161c1d1c6b6f86482ec981b01c009043b33 — zero textures, zero world data, one push per join
[c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[19:20:28 INFO]: [c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[c-LoD] stream: tick 1800 65536/65536 cells, lap 684, cone 20488 zone 1310 dirty 30 (q 14680) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1151310c/1348e, edits +23p/23c/9e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 375 forced 565, native 22400c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 298136p/423b fail 0 ~39 kB/s cache 14003/24576 h284133 m14003 e0, srbeat 16181s/452q drop 0stale/380undec, ereb 1364s/10q, sweep 0r/25d
[19:23:29 INFO]: [c-LoD] stream: tick 1800 65536/65536 cells, lap 684, cone 20488 zone 1310 dirty 30 (q 14680) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1151310c/1348e, edits +23p/23c/9e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 375 forced 565, native 22400c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 298136p/423b fail 0 ~39 kB/s cache 14003/24576 h284133 m14003 e0, srbeat 16181s/452q drop 0stale/380undec, ereb 1364s/10q, sweep 0r/25d
```
### sessions
```
[19:20:41 INFO]: System chat: Stepan joined the game
[19:23:38 INFO]: System chat: Stepan left the game
```
### client log tail
```
Horizon 0.1.0
Horizon renderer: Vulkan has no conformant adapter (Vulkan: only a software adapter (llvmpipe (LLVM 21.1.8, 256 bits))); asking again for the ones wgpu hides as non-conformant
Horizon renderer: trying OpenGL (Vulkan: only a software adapter (llvmpipe (LLVM 21.1.8, 256 bits)))
libEGL warning: DRI3 error: Could not get DRI3 device
libEGL warning: Ensure your X server supports DRI3 to get accelerated rendering
Horizon: ignoring minecraft:ambient_entity_effect: no such vanilla particle in this version
Horizon renderer: OpenGL has no conformant adapter (OpenGL: only a software adapter (llvmpipe (LLVM 21.1.8, 256 bits))); asking again for the ones wgpu hides as non-conformant
libEGL warning: DRI3 error: Could not get DRI3 device
libEGL warning: Ensure your X server supports DRI3 to get accelerated rendering
Horizon renderer: llvmpipe (LLVM 21.1.8, 256 bits) (Vulkan, Cpu)
Horizon renderer: requesting the device (driver llvmpipe Mesa 26.0.8-1ubuntu0.3 (LLVM 21.1.8)) with features [TEXTURE_FORMAT_16BIT_NORM | TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES | PIPELINE_STATISTICS_QUERY | TIMESTAMP_QUERY_INSIDE_ENCODERS | TIMESTAMP_QUERY_INSIDE_PASSES | MAPPABLE_PRIMARY_BUFFERS | MULTI_DRAW_INDIRECT_COUNT | VERTEX_WRITABLE_STORAGE | CLEAR_TEXTURE | TEXTURE_ATOMIC | SHADER_EARLY_DEPTH_TEST | SUBGROUP | TIMESTAMP_QUERY | INDIRECT_FIRST_INSTANCE | RG11B10UFLOAT_RENDERABLE | FLOAT32_FILTERABLE | FLOAT32_BLENDABLE] and the adapter's limits (buffers up to 2147483647 bytes, storage bindings up to 134217728 bytes, 2048 texture layers)
Horizon renderer: device ready; configuring the surface (1280x720 Bgra8Unorm, Mailbox)
Horizon renderer: the chosen graphics API gave no GPU (No Vulkan or OpenGL graphics driver found: Horizon needs the Vulkan loader (libvulkan.so.1) and a Vulkan driver for your GPU (Mesa's, or NVIDIA's), or OpenGL through EGL (Vulkan: only a software adapter (llvmpipe (LLVM 21.1.8, 256 bits)); OpenGL: only a software adapter (llvmpipe (LLVM 21.1.8, 256 bits)))); rendering on the CPU, which is slow
Horizon renderer: HDR output not offered by this display
Horizon renderer: creating the pipelines
Horizon: UI bundle ready after 563 ms
Horizon: block sprites: 1301 layers at 64 px (largest 64 px, budget 256 MiB)
load core tables: 1.00s
Horizon renderer: pipelines ready (multi-draw on, occlusion culling off, GPU culling unavailable, GPU timer on)
Horizon renderer: acquiring the first frame
Horizon renderer: submitting the first frame
Horizon renderer: presenting the first frame
load pack tables: 1.19s
Horizon: loading screen shown after 1202 ms
Horizon: loading the title artwork after 1202 ms
Horizon: core-ready after 1207 ms
Horizon renderer: llvmpipe (LLVM 21.1.8, 256 bits) (Vulkan, Cpu)
Horizon: handoff CPU resources prepared on worker in 131 ms
load items: 1.45s
Horizon: resources loaded in 1.5s; 35723 block states, 1658 items, 67 biomes; 3 worker threads
Horizon: rest-ready after 1501 ms
Horizon renderer: rebuilt on the same device (51 shader modules, layouts and pipelines kept)
Horizon renderer: llvmpipe (LLVM 21.1.8, 256 bits) (Vulkan, Cpu)
Horizon renderer: rebuilt on the same device (51 shader modules, layouts and pipelines kept)
Horizon: world renderer ready after 1994 ms
Horizon: no sounds in the active resources; prepare the game again in the launcher (or run horizon-prepare sounds)
Horizon: no sound: audio off: HORIZON_NO_AUDIO is set
Horizon: waiting for world resources
Horizon: world resources joined after 0 ms
Horizon: startup handoff 362 ms (renderer 338 ms, start_game 17 ms)
Horizon: first frame after 704 ms
Horizon: worldgen data loaded in 740.217294ms
Horizon: block sprites: 1301 layers at 64 px (largest 64 px, budget 256 MiB)
Horizon renderer: llvmpipe (LLVM 21.1.8, 256 bits) (Vulkan, Cpu)
Horizon renderer: rebuilt on the same device (51 shader modules, layouts and pipelines kept)
Failed to load post effect clod:lod_terrain: pass 4: reads the target it writes
XIO:  fatal IO error 2 (No such file or directory) on X server ":99"
      after 6484 requests (419 known processed) with 0 events remaining.
--- exceptions (Realms/RTKit noise filtered) ---
(no client log)
```
### server log tail
```
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[19:23:39 INFO]: [ChunkHolderManager] Saved 1004 block chunks, 973 entity chunks, 0 poi chunks in world 'minecraft:overworld' in 0.92s
[19:23:39 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:overworld'
[19:23:39 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:overworld'
[19:23:39 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[19:23:39 INFO]: [ChunkHolderManager] Waiting 60s for chunk system to halt for world 'minecraft:the_nether'
[19:23:39 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:the_nether'
[19:23:39 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_nether'
[19:23:39 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_nether' in 0.00s
[19:23:39 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_nether'
[19:23:39 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_nether'
[19:23:39 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[19:23:39 INFO]: [ChunkHolderManager] Waiting 60s for chunk system to halt for world 'minecraft:the_end'
[19:23:39 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:the_end'
[19:23:39 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_end'
[19:23:39 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_end' in 0.00s
[19:23:39 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_end'
[19:23:39 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_end'
[19:23:39 INFO]: Waiting for all RegionFile I/O tasks to complete...
[19:23:39 INFO]: All RegionFile I/O tasks to complete
[19:23:39 INFO]: [MoonriseCommon] Awaiting termination of worker pool for up to 60s...
[19:23:39 INFO]: [MoonriseCommon] Awaiting termination of I/O pool for up to 60s...
[c-LoD] EB: enter, native_gen=true
[launcher] server exited with code 143
```
### shots
```
-rw-r--r-- 1 runner runner 162264 Nov 18 18:44 11_h165_spark.png
-rw-r--r-- 1 runner runner 162549 Nov 18 18:44 11_h168_spark.png
-rw-r--r-- 1 runner runner 162799 Nov 18 18:44 11_h172_spark.png
-rw-r--r-- 1 runner runner 162821 Nov 18 18:44 11_h176_spark.png
-rw-r--r-- 1 runner runner 162841 Nov 18 18:45 11_h180_spark.png
-rw-r--r-- 1 runner runner 163260 Nov 18 18:45 11_h185_spark.png
-rw-r--r-- 1 runner runner 163350 Nov 18 18:45 12_after_edit_probe0.png
-rw-r--r-- 1 runner runner 211782 Nov 18 18:53 13_after_relog_probe0.png
-rw-r--r-- 1 runner runner 169285 Nov 18 18:53 14_leg2_baseline.png
-rw-r--r-- 1 runner runner 169164 Nov 18 18:53 14_leg2_burst_far.png
-rw-r--r-- 1 runner runner 168528 Nov 18 18:53 14_leg2_burst_greendust.png
-rw-r--r-- 1 runner runner 169235 Nov 18 18:53 14_leg2_burst_near.png
-rw-r--r-- 1 runner runner 157880 Nov 18 18:53 diff_after_relog.png
-rw-r--r-- 1 runner runner 152210 Nov 18 18:45 diff_live_edit.png
-rw-r--r-- 1 runner runner      0 Nov 18 18:41 ffmpeg.err
-rw-r--r-- 1 runner runner   7160 Nov 18 18:45 lens_12_after_edit_probe0.png
-rw-r--r-- 1 runner runner  14419 Nov 18 18:53 lens_13_after_relog_probe0.png
drwxr-xr-x 2 runner runner   4096 Nov 18 18:53 lens_parity
drwxr-xr-x 2 runner runner   4096 Nov 18 19:17 parity
drwxr-xr-x 2 runner runner   4096 Nov 18 18:47 parity_dbg
drwxr-xr-x 2 runner runner   4096 Nov 18 19:20 parity_dbg8
-rw-r--r-- 1 runner runner    549 Nov 18 18:53 probe_counts.txt
drwxr-xr-x 2 runner runner   4096 Nov 18 19:23 rim_mask
-rw-r--r-- 1 runner runner 103369 Nov 18 19:23 server_full.log
-rw-r--r-- 1 runner runner  45043 Nov 18 19:23 summary.md
```

**exit=1** failures=2 leg1=1 leg2=1
