- phase: 0 sanity (11:48:18)
### c-LoD visual run (-930,796, marker=red_concrete, vantage -1005,150 yaw 270)
- GL:     Vendor: Mesa (0xffffffff);    Device: llvmpipe (LLVM 21.1.8, 256 bits) (0xffffffff);OpenGL renderer string: llvmpipe (LLVM 21.1.8, 256 bits);OpenGL core profile version string: 4.5 (Core Profile) Mesa 26.0.8-1ubuntu0.3;
- boot + gates OK (11:48:45)
- phase: 1 server booted (11:48:45)
- sky-grid armed
- phase: 2 leg-1 client joined (11:49:08)
- joined session 1 (11:49:08)
- stream live: cells=65536
- phase: 3 scan populated (11:49:31)
- terrain range: 2 terraced mountains at (CX-4,CZ-20)/(CX-2,CZ+16), tops 185/190, grass terraces + stone caps, 66..82 m from the eye (inside the 83.5 m band, chunk edge 64 m)
- scan complete=1 (cells=cells=65536/65536)
- paint settled=1 (wire 522580p)
- baseline try 1: drift=7012 px
- baseline try 2: drift=6597 px
- baseline try 3: drift=1 px
- lod_silhouette_cols=49 (LOD paint ON SCREEN at 75 m if > 20 — the march-drawn marker column; vanilla cannot render there, chunk edge 64 m; #255 re-measures 49, the ZONE_FAR=48 regression #256 and the control read 0)
- 130 terrain-green px in the sky region (LOD MOUNTAINS on screen if > 5000; the range paints ColLow-green on low terraces via the fallback palette; flat-world baselines and the pack-free control read ~0)
- burst_close white px: 0 (renderer alive if > 50)
- burst_far white px: 534 (renderer alive in the marker zone if > 20)
- cut walk 40 m: 73 white px (> 20 = rendered)
- cut walk 60 m: 0 white px (> 20 = rendered)
- cut walk 100 m: 0 white px (> 20 = rendered)
- measured far-render cut: 40 m (G4 flap instrument; the decode proof needs >= 83 m from the y=151.6 eye)
- burst_close retry white px: 0 (preflight passes if > 50)
- preflight lens_zero=0 (1 = capture NEVER committed = no particle EVER rendered)
- preflight stage eyes: ?
- fill issued: 5x111x5 red_concrete at -930,796 (zone forceloaded ±48)
- phase: 4 marker filled (11:56:14)

### /clod-cells AFTER FILL (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 14 0 0x8400 0x2000
123 122 -940 784 71 0 1 [97, 119, 97] 14 0 0x8700 0x2000
124 122 -938 784 68 0 1 [97, 119, 97] 13 0 0x8400 0x2000
125 122 -936 784 62 1 1 [63, 118, 228] 53 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 52 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 52 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 51 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 560 0 0x8000 0x4100
130 122 -926 784 66 0 1 [134, 183, 131] 560 0 0x8200 0x4100
131 122 -924 784 75 0 1 [97, 119, 97] 560 0 0x8b00 0x2000
132 122 -922 784 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 122 -920 784 75 0 1 [97, 119, 97] 18446744073709551615 1 0x8b00 0x2000
134 122 -918 784 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x2000
122 123 -942 786 70 0 1 [97, 119, 97] 14 0 0x8600 0x2000
123 123 -940 786 65 0 1 [97, 119, 97] 14 0 0x8100 0x2000
124 123 -938 786 65 0 1 [97, 119, 97] 13 0 0x8100 0x2000
125 123 -936 786 62 1 1 [63, 118, 228] 53 0 0x7e00 0xd800
126 123 -934 786 62 1 1 [63, 118, 228] 52 0 0x7e00 0xd800
127 123 -932 786 62 1 1 [63, 118, 228] 52 0 0x7e00 0xd800
128 123 -930 786 62 1 1 [63, 118, 228] 51 0 0x7e00 0xd800
129 123 -928 786 73 0 1 [97, 119, 97] 560 0 0x8900 0x2000
130 123 -926 786 67 0 1 [134, 183, 131] 560 0 0x8300 0x4100
131 123 -924 786 69 0 1 [134, 183, 131] 560 0 0x8500 0x4100
132 123 -922 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 123 -920 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 123 -918 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
122 124 -942 788 72 0 1 [97, 119, 97] 14 0 0x8800 0x2000
123 124 -940 788 70 0 1 [97, 119, 97] 14 0 0x8600 0x2000
124 124 -938 788 68 0 1 [97, 119, 97] 13 0 0x8400 0x2000
125 124 -936 788 62 1 1 [63, 118, 228] 53 0 0x7e00 0xd800
126 124 -934 788 62 1 1 [63, 118, 228] 52 0 0x7e00 0xd800
127 124 -932 788 62 1 1 [63, 118, 228] 52 0 0x7e00 0xd800
128 124 -930 788 73 0 1 [97, 119, 97] 11 0 0x8900 0x2000
129 124 -928 788 76 0 1 [97, 119, 97] 560 0 0x8c00 0x2000
130 124 -926 788 73 0 1 [97, 119, 97] 560 0 0x8900 0x2000
131 124 -924 788 69 0 1 [134, 183, 131] 560 0 0x8500 0x4100
132 124 -922 788 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 124 -920 788 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 124 -918 788 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x2000
122 125 -942 790 74 0 1 [97, 119, 97] 14 0 0x8a00 0x2000
123 125 -940 790 73 0 1 [97, 119, 97] 14 0 0x8900 0x2000
124 125 -938 790 71 0 1 [97, 119, 97] 13 0 0x8700 0x2000
125 125 -936 790 62 1 1 [63, 118, 228] 53 0 0x7e00 0xd800
126 125 -934 790 62 1 1 [63, 118, 228] 52 0 0x7e00 0xd800
127 125 -932 790 62 1 1 [63, 118, 228] 52 0 0x7e00 0xd800
128 125 -930 790 62 1 1 [63, 118, 228] 51 0 0x7e00 0xd800
129 125 -928 790 73 0 1 [97, 119, 97] 11 0 0x8900 0x2000
130 125 -926 790 68 0 1 [134, 183, 131] 560 0 0x8400 0x4100
131 125 -924 790 69 0 1 [134, 183, 131] 560 0 0x8500 0x4100
132 125 -922 790 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
133 125 -920 790 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 125 -918 790 70 0 1 [134, 183, 131] 18446744073709551615 1 0x8600 0x4100
122 126 -942 792 74 0 1 [97, 119, 97] 14 0 0x8a00 0x2000
123 126 -940 792 73 0 1 [97, 119, 97] 14 0 0x8900 0x2000
124 126 -938 792 68 0 1 [97, 119, 97] 13 0 0x8400 0x2000
125 126 -936 792 62 1 1 [63, 118, 228] 53 0 0x7e00 0xd800
126 126 -934 792 62 1 1 [63, 118, 228] 52 0 0x7e00 0xd800
127 126 -932 792 62 1 1 [63, 118, 228] 52 0 0x7e00 0xd800
128 126 -930 792 62 1 1 [63, 118, 228] 51 0 0x7e00 0xd800
129 126 -928 792 67 0 1 [134, 183, 131] 11 0 0x8300 0x4100
130 126 -926 792 74 0 1 [97, 119, 97] 560 0 0x8a00 0x2000
131 126 -924 792 77 0 1 [97, 119, 97] 560 0 0x8d00 0x2000
132 126 -922 792 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 126 -920 792 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3200
134 126 -918 792 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x5000
122 127 -942 794 67 0 1 [134, 183, 131] 15 0 0x8300 0x4100
123 127 -940 794 68 0 1 [134, 183, 131] 14 0 0x8400 0x4100
124 127 -938 794 65 0 1 [134, 183, 131] 13 0 0x8100 0x4100
125 127 -936 794 62 1 1 [63, 118, 228] 53 0 0x7e00 0xd800
126 127 -934 794 62 1 1 [63, 118, 228] 52 0 0x7e00 0xd800
127 127 -932 794 170 0 1 [142, 33, 33] 52 1 0xea00 0x0a00
128 127 -930 794 170 0 1 [142, 33, 33] 51 1 0xea00 0x0a00
129 127 -928 794 170 0 1 [142, 33, 33] 51 1 0xea00 0x0a00
130 127 -926 794 74 0 1 [97, 119, 97] 560 0 0x8a00 0x2000
131 127 -924 794 77 0 1 [97, 119, 97] 560 0 0x8d00 0x2000
132 127 -922 794 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 127 -920 794 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3200
134 127 -918 794 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x5000
122 128 -942 796 68 0 1 [134, 183, 131] 15 0 0x8400 0x4100
123 128 -940 796 72 0 1 [97, 119, 97] 14 0 0x8800 0x2000
124 128 -938 796 75 0 1 [97, 119, 97] 13 0 0x8b00 0x2000
125 128 -936 796 72 0 1 [97, 119, 97] 13 0 0x8800 0x2000
126 128 -934 796 62 1 1 [63, 118, 228] 52 0 0x7e00 0xd800
127 128 -932 796 170 0 1 [142, 33, 33] 52 1 0xea00 0x0a00
128 128 -930 796 170 0 1 [142, 33, 33] 51 1 0xea00 0x0a00
129 128 -928 796 170 0 1 [142, 33, 33] 51 1 0xea00 0x0a00
130 128 -926 796 68 0 1 [134, 183, 131] 560 0 0x8400 0x4100
131 128 -924 796 74 0 1 [97, 119, 97] 560 0 0x8a00 0x2000
132 128 -922 796 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 128 -920 796 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
134 128 -918 796 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 129 -942 798 69 0 1 [134, 183, 131] 14 0 0x8500 0x4100
123 129 -940 798 75 0 1 [97, 119, 97] 14 0 0x8b00 0x2000
124 129 -938 798 78 0 1 [97, 119, 97] 15 0 0x8e00 0x2000
125 129 -936 798 75 0 1 [97, 119, 97] 13 0 0x8b00 0x2000
126 129 -934 798 62 1 1 [63, 118, 228] 52 0 0x7e00 0xd800
127 129 -932 798 170 0 1 [142, 33, 33] 52 1 0xea00 0x0a00
128 129 -930 798 170 0 1 [142, 33, 33] 51 1 0xea00 0x0a00
129 129 -928 798 170 0 1 [142, 33, 33] 51 1 0xea00 0x0a00
130 129 -926 798 71 0 1 [97, 119, 97] 560 0 0x8700 0x2000
131 129 -924 798 69 0 1 [134, 183, 131] 560 0 0x8500 0x4100
132 129 -922 798 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 129 -920 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 129 -918 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 130 -942 800 70 0 1 [134, 183, 131] 14 0 0x8600 0x4100
123 130 -940 800 76 0 1 [97, 119, 97] 14 0 0x8c00 0x2000
124 130 -938 800 76 0 1 [97, 119, 97] 13 0 0x8c00 0x2000
125 130 -936 800 72 0 1 [97, 119, 97] 13 0 0x8800 0x2000
126 130 -934 800 62 1 1 [63, 118, 228] 52 0 0x7e00 0xd800
127 130 -932 800 62 1 1 [63, 118, 228] 52 0 0x7e00 0xd800
128 130 -930 800 62 1 1 [63, 118, 228] 51 0 0x7e00 0xd800
129 130 -928 800 71 0 1 [97, 119, 97] 11 0 0x8700 0x2000
130 130 -926 800 71 0 1 [97, 119, 97] 560 0 0x8700 0x2000
131 130 -924 800 69 0 1 [134, 183, 131] 560 0 0x8500 0x4100
132 130 -922 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
133 130 -920 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 130 -918 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 131 -942 802 78 0 1 [97, 119, 97] 16 0 0x8e00 0x2000
123 131 -940 802 77 0 1 [97, 119, 97] 14 0 0x8d00 0x2000
124 131 -938 802 77 0 1 [97, 119, 97] 13 0 0x8d00 0x2000
125 131 -936 802 66 0 1 [134, 183, 131] 13 0 0x8200 0x4100
126 131 -934 802 62 1 1 [63, 118, 228] 52 0 0x7e00 0xd800
127 131 -932 802 62 1 1 [63, 118, 228] 52 0 0x7e00 0xd800
128 131 -930 802 62 1 1 [63, 118, 228] 51 0 0x7e00 0xd800
129 131 -928 802 62 1 1 [63, 118, 228] 600 0 0x7e00 0xd800
130 131 -926 802 66 0 1 [134, 183, 131] 560 0 0x8200 0x4100
131 131 -924 802 68 0 1 [134, 183, 131] 560 0 0x8400 0x4100
132 131 -922 802 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 131 -920 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 131 -918 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 132 -942 804 79 0 1 [97, 119, 97] 16 0 0x8f00 0x2000
123 132 -940 804 76 0 1 [97, 119, 97] 14 0 0x8c00 0x2000
124 132 -938 804 76 0 1 [97, 119, 97] 13 0 0x8c00 0x2000
125 132 -936 804 76 0 1 [97, 119, 97] 13 0 0x8c00 0x2000
126 132 -934 804 62 1 1 [63, 118, 228] 52 0 0x7e00 0xd800
127 132 -932 804 62 1 1 [63, 118, 228] 52 0 0x7e00 0xd800
128 132 -930 804 62 1 1 [63, 118, 228] 51 0 0x7e00 0xd800
129 132 -928 804 62 1 1 [63, 118, 228] 600 0 0x7e00 0xd800
130 132 -926 804 65 0 1 [134, 183, 131] 560 0 0x8100 0x4100
131 132 -924 804 77 0 1 [97, 119, 97] 560 0 0x8d00 0x2000
132 132 -922 804 79 0 1 [97, 119, 97] 560 0 0x8f00 0x2000
133 132 -920 804 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 132 -918 804 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x2000
122 133 -942 806 78 0 1 [97, 119, 97] 16 0 0x8e00 0x2000
123 133 -940 806 69 0 1 [134, 183, 131] 14 0 0x8500 0x4100
124 133 -938 806 155 0 1 [134, 183, 131] 14 0 0xdb00 0x4100
125 133 -936 806 155 0 1 [134, 183, 131] 14 0 0xdb00 0x4100
126 133 -934 806 155 0 1 [134, 183, 131] 13 0 0xdb00 0x4100
127 133 -932 806 155 0 1 [134, 183, 131] 13 0 0xdb00 0x4100
128 133 -930 806 155 0 1 [134, 183, 131] 12 0 0xdb00 0x4100
129 133 -928 806 155 0 1 [134, 183, 131] 11 0 0xdb00 0x4100
130 133 -926 806 155 0 1 [134, 183, 131] 11 0 0xdb00 0x4100
131 133 -924 806 67 0 1 [134, 183, 131] 560 0 0x8300 0x4100
132 133 -922 806 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 133 -920 806 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 133 -918 806 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x2000
122 134 -942 808 79 0 1 [97, 119, 97] 16 0 0x8f00 0x2000
123 134 -940 808 68 0 1 [134, 183, 131] 14 0 0x8400 0x4100
124 134 -938 808 155 0 1 [134, 183, 131] 14 0 0xdb00 0x4100
125 134 -936 808 180 0 1 [134, 183, 131] 11 0 0xf400 0x4100
126 134 -934 808 180 0 1 [134, 183, 131] 11 0 0xf400 0x4100
127 134 -932 808 180 0 1 [134, 183, 131] 560 0 0xf400 0x4100
128 134 -930 808 180 0 1 [134, 183, 131] 560 0 0xf400 0x4100
129 134 -928 808 180 0 1 [134, 183, 131] 560 0 0xf400 0x4100
130 134 -926 808 155 0 1 [134, 183, 131] 11 0 0xdb00 0x4100
131 134 -924 808 67 0 1 [134, 183, 131] 560 0 0x8300 0x4100
132 134 -922 808 68 0 1 [134, 183, 131] 18446744073709551615 1 0x8400 0x4100
133 134 -920 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 134 -918 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
```
### stream diag after fill:
[c-LoD] stream: tick 4400 65536/65536 cells, lap 753, cone 22713 zone 1758 dirty 908 (q 12076) ewin 218 rbeat 0p/0 rearm 0c/0t rescan 1121948c/2496e, edits +6027p/4545c/621e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 326 forced 186, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 670076p/2109b fail 0 ~73 kB/s cache 13313/24576 h656763 m13313 e0, srbeat 58638s/0q drop 54stale/0undec, ereb 47282s/0q, sweep 0r/93d
[11:56:15 INFO]: [c-LoD] stream: tick 4400 65536/65536 cells, lap 753, cone 22713 zone 1758 dirty 908 (q 12076) ewin 218 rbeat 0p/0 rearm 0c/0t rescan 1121948c/2496e, edits +6027p/4545c/621e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 326 forced 186, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 670076p/2109b fail 0 ~73 kB/s cache 13313/24576 h656763 m13313 e0, srbeat 58638s/0q drop 54stale/0undec, ereb 47282s/0q, sweep 0r/93d
- carrier ladder y=155: 16696 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=165: 16676 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=168: 16676 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=172: 16676 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=176: 16676 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=180: 16676 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=185: 16676 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- probe 0: red=1965 paint=13887 lod_silhouette_cols=43 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=6 hits_hi=16 mask_hits=30 (need red>=60 paint>=300 lod>20)
- **LEG 1 PASS** (zero relogs)
- phase: 5 leg-1 probes done (11:56:48)
### stream diag after probes:
[11:56:35 INFO]: [c-LoD] stream: tick 4600 65536/65536 cells, lap 753, cone 22713 zone 1758 dirty 926 (q 12109) ewin 218 rbeat 0p/0 rearm 0c/0t rescan 1186483c/2521e, edits +6565p/4963c/803e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 326 forced 186, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 671390p/2127b fail 0 ~3 kB/s cache 13394/24576 h657996 m13394 e0, srbeat 58782s/0q drop 54stale/0undec, ereb 47408s/9q, sweep 0r/93d
[c-LoD] stream: tick 4700 65536/65536 cells, lap 753, cone 22713 zone 1758 dirty 935 (q 12103) ewin 118 rbeat 0p/0 rearm 0c/0t rescan 1220308c/2528e, edits +6673p/5027c/820e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 326 forced 186, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 672119p/2137b fail 0 ~3 kB/s cache 13394/24576 h658725 m13394 e0, srbeat 58863s/0q drop 54stale/0undec, ereb 47480s/9q, sweep 0r/93d
[11:56:45 INFO]: [c-LoD] stream: tick 4700 65536/65536 cells, lap 753, cone 22713 zone 1758 dirty 935 (q 12103) ewin 118 rbeat 0p/0 rearm 0c/0t rescan 1220308c/2528e, edits +6673p/5027c/820e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 326 forced 186, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 672119p/2137b fail 0 ~3 kB/s cache 13394/24576 h658725 m13394 e0, srbeat 58863s/0q drop 54stale/0undec, ereb 47480s/9q, sweep 0r/93d
- parity debug tour: 13 vantages (session-1 client, DEBUG pack — evidence only, the gate pairs the §7b world session)
- parity debug tour done: 13 shots in parity_dbg/ (the §7b world session feeds the gate)
- joined session 2 (11:59:39)
- leg-2 baseline scene px (x<940): 0 (0 = the relog client paints no world at all)
- leg-2 renderer probe NEAR (20 m) white px: 0 raw / 0 changed-vs-baseline (> 20 = the client renders particles)
- leg-2 renderer probe FAR (80 m, marker zone) white px: 510 raw / 510 changed-vs-baseline (> 20 = no distance cut; ~0 with NEAR alive = particles beyond the client chunk radius do not render)
- leg-2 carrier probe green dust (80 m): changed-green px 0 / total changed 123140 vs baseline (> 100 green = a stable green carrier exists; total > 0 with green ~0 = the dust renders but fog washes it — a ruler problem, not a carrier problem)
- phase: 8 leg-2 relog probes (12:05:04)
- relog probe 0: red=0 scene=129931 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=5 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 1: red=0 scene=138734 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=7 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 2: red=0 scene=145936 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=3 hits_hi=8 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 3: red=0 scene=157795 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=3 hits_hi=9 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 4: red=0 scene=155432 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=3 hits_hi=9 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 5: red=5507 scene=153559 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=3 hits_hi=9 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- **LEG 2 PASS** (marker persisted over relog)
- phase: 9 leg-2 done (12:05:52)

### client log forensics (pipeline/particle/atlas — v6.8.2)
```
[11:59:28] [main/DEBUG]: Loading LWJGL system
WARNING: java.lang.System::load has been called by org.lwjgl.system.Library$$Lambda/0x00000000691e3d60 in an unnamed module (file:/home/runner/mc-test/client/libraries/org/lwjgl/lwjgl/3.4.3/lwjgl-3.4.3.jar)
WARNING: sun.misc.Unsafe::objectFieldOffset has been called by org.lwjgl.system.MemoryBackendUnsafeLegacy (file:/home/runner/mc-test/client/libraries/org/lwjgl/lwjgl/3.4.3/lwjgl-3.4.3.jar)
WARNING: Please consider reporting this to the maintainers of class org.lwjgl.system.MemoryBackendUnsafeLegacy
[11:59:28] [main/DEBUG]: Loading OpenGL
[11:59:29] [main/DEBUG]: Loading shaderc
[11:59:33] [Render thread/ERROR]: Error parsing option value 4 for option Simulation Distance: Value 4 outside of range [5:33]
[11:59:33] [Render thread/INFO]: Backend library: LWJGL version 3.4.3+4
[11:59:33] [Render thread/INFO]: Using graphics backend OpenGL, using drivers: 4.5 (Core Profile) Mesa 26.0.8-1ubuntu0.3
[11:59:33] [Render thread/INFO]: Using graphics device: llvmpipe (LLVM 21.1.8, 256 bits) (Mesa)
[11:59:33] [Render thread/INFO]: Using graphics device extensions: GL_ARB_multi_draw_indirect, GL_ARB_buffer_storage, GL_ARB_shader_draw_parameters, GL_ARB_base_instance, GL_KHR_debug, GL_ARB_draw_indirect, GL_ARB_clip_control, GL_ARB_vertex_attrib_binding, GL_ARB_direct_state_access, GL_EXT_texture_filter_anisotropic
[11:59:34] [Download-2/ERROR]: Failed to fetch user properties
com.mojang.authlib.exceptions.InvalidCredentialsException: Status: 401
	at com.mojang.authlib.exceptions.MinecraftClientHttpException.toAuthenticationException(MinecraftClientHttpException.java:70)
Caused by: MinecraftClientHttpException[type=HTTP_ERROR, status=401, response=ErrorResponse[path=/player/attributes, error=null, errorMessage=null, details=null]]
	at com.mojang.authlib.minecraft.client.MinecraftClient.buildHttpException(MinecraftClient.java:170)
[11:59:34] [Download-2/INFO]: Could not authorize you against Realms server: java.lang.RuntimeException: Failed to parse into SignedJWT: 0
[11:59:34] [Download-2/ERROR]: Failed to fetch Realms feature flags
com.mojang.realmsclient.exception.RealmsServiceException: Realms authentication error with message 'java.lang.RuntimeException: Failed to parse into SignedJWT: 0'
[11:59:34] [Render thread/INFO]: Reloading ResourceManager: vanilla, vanilla
[11:59:36] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[11:59:37] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[11:59:37] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[11:59:37] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[11:59:37] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[11:59:37] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[11:59:37] [Render thread/INFO]: Created: 2048x2048x4 minecraft:textures/atlas/blocks.png-atlas
[11:59:37] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[11:59:37] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[11:59:37] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[11:59:37] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[11:59:37] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
[11:59:43] [Render thread/INFO]: Reloading ResourceManager: vanilla, vanilla, server/00000000/6b1f2f7e-9c0d-4a4b-8b0a-b10d1b4ab100
[11:59:45] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[11:59:45] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[11:59:45] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[11:59:45] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[11:59:45] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[11:59:45] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[11:59:45] [Render thread/INFO]: Created: 2048x2048x4 minecraft:textures/atlas/blocks.png-atlas
[11:59:45] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[11:59:45] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[11:59:45] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[11:59:45] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[11:59:45] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
```

### /clod-cells AFTER PROBES (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 62 0 0x8400 0x2000
123 122 -940 784 71 0 1 [97, 119, 97] 62 0 0x8700 0x2000
124 122 -938 784 68 0 1 [97, 119, 97] 61 0 0x8400 0x2000
125 122 -936 784 62 1 1 [63, 118, 228] 101 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 100 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 99 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 99 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 388 0 0x8000 0x4100
130 122 -926 784 66 0 1 [134, 183, 131] 388 0 0x8200 0x4100
131 122 -924 784 75 0 1 [97, 119, 97] 388 0 0x8b00 0x2000
132 122 -922 784 77 0 1 [97, 119, 97] 4476 0 0x8d00 0x2000
133 122 -920 784 75 0 1 [97, 119, 97] 4476 0 0x8b00 0x2000
134 122 -918 784 73 0 1 [97, 119, 97] 4476 0 0x8900 0x2000
122 123 -942 786 70 0 1 [97, 119, 97] 62 0 0x8600 0x2000
123 123 -940 786 65 0 1 [97, 119, 97] 62 0 0x8100 0x2000
124 123 -938 786 65 0 1 [97, 119, 97] 61 0 0x8100 0x2000
125 123 -936 786 62 1 1 [63, 118, 228] 101 0 0x7e00 0xd800
126 123 -934 786 62 1 1 [63, 118, 228] 100 0 0x7e00 0xd800
127 123 -932 786 62 1 1 [63, 118, 228] 100 0 0x7e00 0xd800
128 123 -930 786 62 1 1 [63, 118, 228] 99 0 0x7e00 0xd800
129 123 -928 786 73 0 1 [97, 119, 97] 388 0 0x8900 0x2000
130 123 -926 786 67 0 1 [134, 183, 131] 388 0 0x8300 0x4100
131 123 -924 786 69 0 1 [134, 183, 131] 388 0 0x8500 0x4100
132 123 -922 786 77 0 1 [97, 119, 97] 4476 0 0x8d00 0x2000
133 123 -920 786 77 0 1 [97, 119, 97] 4476 0 0x8d00 0x2000
134 123 -918 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
122 124 -942 788 72 0 1 [97, 119, 97] 62 0 0x8800 0x2000
123 124 -940 788 70 0 1 [97, 119, 97] 62 0 0x8600 0x2000
124 124 -938 788 68 0 1 [97, 119, 97] 61 0 0x8400 0x2000
125 124 -936 788 62 1 1 [63, 118, 228] 101 0 0x7e00 0xd800
126 124 -934 788 62 1 1 [63, 118, 228] 100 0 0x7e00 0xd800
127 124 -932 788 62 1 1 [63, 118, 228] 100 0 0x7e00 0xd800
128 124 -930 788 73 0 1 [97, 119, 97] 59 0 0x8900 0x2000
129 124 -928 788 76 0 1 [97, 119, 97] 388 0 0x8c00 0x2000
130 124 -926 788 73 0 1 [97, 119, 97] 388 0 0x8900 0x2000
131 124 -924 788 69 0 1 [134, 183, 131] 388 0 0x8500 0x4100
132 124 -922 788 77 0 1 [97, 119, 97] 4476 0 0x8d00 0x2000
133 124 -920 788 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 124 -918 788 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x2000
122 125 -942 790 74 0 1 [97, 119, 97] 62 0 0x8a00 0x2000
123 125 -940 790 73 0 1 [97, 119, 97] 62 0 0x8900 0x2000
124 125 -938 790 71 0 1 [97, 119, 97] 61 0 0x8700 0x2000
125 125 -936 790 62 1 1 [63, 118, 228] 101 0 0x7e00 0xd800
126 125 -934 790 62 1 1 [63, 118, 228] 100 0 0x7e00 0xd800
127 125 -932 790 62 1 1 [63, 118, 228] 100 0 0x7e00 0xd800
128 125 -930 790 62 1 1 [63, 118, 228] 99 0 0x7e00 0xd800
129 125 -928 790 73 0 1 [97, 119, 97] 388 0 0x8900 0x2000
130 125 -926 790 68 0 1 [134, 183, 131] 388 0 0x8400 0x4100
131 125 -924 790 69 0 1 [134, 183, 131] 388 0 0x8500 0x4100
132 125 -922 790 69 0 1 [134, 183, 131] 4476 0 0x8500 0x4100
133 125 -920 790 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 125 -918 790 70 0 1 [134, 183, 131] 18446744073709551615 1 0x8600 0x4100
122 126 -942 792 74 0 1 [97, 119, 97] 62 0 0x8a00 0x2000
123 126 -940 792 73 0 1 [97, 119, 97] 62 0 0x8900 0x2000
124 126 -938 792 68 0 1 [97, 119, 97] 61 0 0x8400 0x2000
125 126 -936 792 62 1 1 [63, 118, 228] 101 0 0x7e00 0xd800
126 126 -934 792 62 1 1 [63, 118, 228] 100 0 0x7e00 0xd800
127 126 -932 792 62 1 1 [63, 118, 228] 100 0 0x7e00 0xd800
128 126 -930 792 62 1 1 [63, 118, 228] 99 0 0x7e00 0xd800
129 126 -928 792 67 0 1 [134, 183, 131] 388 0 0x8300 0x4100
130 126 -926 792 74 0 1 [97, 119, 97] 388 0 0x8a00 0x2000
131 126 -924 792 77 0 1 [97, 119, 97] 388 0 0x8d00 0x2000
132 126 -922 792 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 126 -920 792 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3200
134 126 -918 792 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x5000
122 127 -942 794 67 0 1 [134, 183, 131] 62 0 0x8300 0x4100
123 127 -940 794 68 0 1 [134, 183, 131] 62 0 0x8400 0x4100
124 127 -938 794 65 0 1 [134, 183, 131] 61 0 0x8100 0x4100
125 127 -936 794 62 1 1 [63, 118, 228] 101 0 0x7e00 0xd800
126 127 -934 794 62 1 1 [63, 118, 228] 100 0 0x7e00 0xd800
127 127 -932 794 170 0 1 [142, 33, 33] 60 0 0xea00 0x0a00
128 127 -930 794 170 0 1 [142, 33, 33] 59 0 0xea00 0x0a00
129 127 -928 794 170 0 1 [142, 33, 33] 59 0 0xea00 0x0a00
130 127 -926 794 74 0 1 [97, 119, 97] 386 0 0x8a00 0x2000
131 127 -924 794 77 0 1 [97, 119, 97] 386 0 0x8d00 0x2000
132 127 -922 794 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 127 -920 794 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3200
134 127 -918 794 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x5000
122 128 -942 796 68 0 1 [134, 183, 131] 62 0 0x8400 0x4100
123 128 -940 796 72 0 1 [97, 119, 97] 62 0 0x8800 0x2000
124 128 -938 796 75 0 1 [97, 119, 97] 61 0 0x8b00 0x2000
125 128 -936 796 72 0 1 [97, 119, 97] 61 0 0x8800 0x2000
126 128 -934 796 62 1 1 [63, 118, 228] 100 0 0x7e00 0xd800
127 128 -932 796 170 0 1 [142, 33, 33] 60 0 0xea00 0x0a00
128 128 -930 796 170 0 1 [142, 33, 33] 59 0 0xea00 0x0a00
129 128 -928 796 170 0 1 [142, 33, 33] 59 0 0xea00 0x0a00
130 128 -926 796 68 0 1 [134, 183, 131] 386 0 0x8400 0x4100
131 128 -924 796 74 0 1 [97, 119, 97] 386 0 0x8a00 0x2000
132 128 -922 796 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 128 -920 796 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
134 128 -918 796 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 129 -942 798 69 0 1 [134, 183, 131] 62 0 0x8500 0x4100
123 129 -940 798 75 0 1 [97, 119, 97] 62 0 0x8b00 0x2000
124 129 -938 798 78 0 1 [97, 119, 97] 63 0 0x8e00 0x2000
125 129 -936 798 75 0 1 [97, 119, 97] 61 0 0x8b00 0x2000
126 129 -934 798 62 1 1 [63, 118, 228] 100 0 0x7e00 0xd800
127 129 -932 798 170 0 1 [142, 33, 33] 60 0 0xea00 0x0a00
128 129 -930 798 170 0 1 [142, 33, 33] 59 0 0xea00 0x0a00
129 129 -928 798 170 0 1 [142, 33, 33] 59 0 0xea00 0x0a00
130 129 -926 798 71 0 1 [97, 119, 97] 386 0 0x8700 0x2000
131 129 -924 798 69 0 1 [134, 183, 131] 386 0 0x8500 0x4100
132 129 -922 798 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 129 -920 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 129 -918 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 130 -942 800 70 0 1 [134, 183, 131] 62 0 0x8600 0x4100
123 130 -940 800 76 0 1 [97, 119, 97] 62 0 0x8c00 0x2000
124 130 -938 800 76 0 1 [97, 119, 97] 61 0 0x8c00 0x2000
125 130 -936 800 72 0 1 [97, 119, 97] 61 0 0x8800 0x2000
126 130 -934 800 62 1 1 [63, 118, 228] 100 0 0x7e00 0xd800
127 130 -932 800 62 1 1 [63, 118, 228] 100 0 0x7e00 0xd800
128 130 -930 800 62 1 1 [63, 118, 228] 99 0 0x7e00 0xd800
129 130 -928 800 71 0 1 [97, 119, 97] 386 0 0x8700 0x2000
130 130 -926 800 71 0 1 [97, 119, 97] 386 0 0x8700 0x2000
131 130 -924 800 69 0 1 [134, 183, 131] 386 0 0x8500 0x4100
132 130 -922 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
133 130 -920 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 130 -918 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 131 -942 802 78 0 1 [97, 119, 97] 64 0 0x8e00 0x2000
123 131 -940 802 77 0 1 [97, 119, 97] 62 0 0x8d00 0x2000
124 131 -938 802 77 0 1 [97, 119, 97] 61 0 0x8d00 0x2000
125 131 -936 802 66 0 1 [134, 183, 131] 61 0 0x8200 0x4100
126 131 -934 802 62 1 1 [63, 118, 228] 100 0 0x7e00 0xd800
127 131 -932 802 62 1 1 [63, 118, 228] 100 0 0x7e00 0xd800
128 131 -930 802 62 1 1 [63, 118, 228] 99 0 0x7e00 0xd800
129 131 -928 802 62 1 1 [63, 118, 228] 428 0 0x7e00 0xd800
130 131 -926 802 66 0 1 [134, 183, 131] 386 0 0x8200 0x4100
131 131 -924 802 68 0 1 [134, 183, 131] 386 0 0x8400 0x4100
132 131 -922 802 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 131 -920 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 131 -918 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 132 -942 804 79 0 1 [97, 119, 97] 64 0 0x8f00 0x2000
123 132 -940 804 76 0 1 [97, 119, 97] 62 0 0x8c00 0x2000
124 132 -938 804 76 0 1 [97, 119, 97] 61 0 0x8c00 0x2000
125 132 -936 804 76 0 1 [97, 119, 97] 61 0 0x8c00 0x2000
126 132 -934 804 62 1 1 [63, 118, 228] 100 0 0x7e00 0xd800
127 132 -932 804 62 1 1 [63, 118, 228] 100 0 0x7e00 0xd800
128 132 -930 804 62 1 1 [63, 118, 228] 99 0 0x7e00 0xd800
129 132 -928 804 62 1 1 [63, 118, 228] 428 0 0x7e00 0xd800
130 132 -926 804 65 0 1 [134, 183, 131] 386 0 0x8100 0x4100
131 132 -924 804 77 0 1 [97, 119, 97] 386 0 0x8d00 0x2000
132 132 -922 804 79 0 1 [97, 119, 97] 386 0 0x8f00 0x2000
133 132 -920 804 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 132 -918 804 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x2000
122 133 -942 806 78 0 1 [97, 119, 97] 64 0 0x8e00 0x2000
123 133 -940 806 69 0 1 [134, 183, 131] 62 0 0x8500 0x4100
124 133 -938 806 155 0 1 [134, 183, 131] 62 0 0xdb00 0x4100
125 133 -936 806 155 0 1 [134, 183, 131] 62 0 0xdb00 0x4100
126 133 -934 806 155 0 1 [134, 183, 131] 61 0 0xdb00 0x4100
127 133 -932 806 155 0 1 [134, 183, 131] 61 0 0xdb00 0x4100
128 133 -930 806 155 0 1 [134, 183, 131] 60 0 0xdb00 0x4100
129 133 -928 806 155 0 1 [134, 183, 131] 59 0 0xdb00 0x4100
130 133 -926 806 155 0 1 [134, 183, 131] 59 0 0xdb00 0x4100
131 133 -924 806 67 0 1 [134, 183, 131] 386 0 0x8300 0x4100
132 133 -922 806 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 133 -920 806 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 133 -918 806 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x2000
122 134 -942 808 79 0 1 [97, 119, 97] 64 0 0x8f00 0x2000
123 134 -940 808 68 0 1 [134, 183, 131] 62 0 0x8400 0x4100
124 134 -938 808 155 0 1 [134, 183, 131] 62 0 0xdb00 0x4100
125 134 -936 808 180 0 1 [134, 183, 131] 59 0 0xf400 0x4100
126 134 -934 808 180 0 1 [134, 183, 131] 59 0 0xf400 0x4100
127 134 -932 808 180 0 1 [134, 183, 131] 386 0 0xf400 0x4100
128 134 -930 808 180 0 1 [134, 183, 131] 386 0 0xf400 0x4100
129 134 -928 808 180 0 1 [134, 183, 131] 386 0 0xf400 0x4100
130 134 -926 808 155 0 1 [134, 183, 131] 59 0 0xdb00 0x4100
131 134 -924 808 67 0 1 [134, 183, 131] 386 0 0x8300 0x4100
132 134 -922 808 68 0 1 [134, 183, 131] 18446744073709551615 1 0x8400 0x4100
133 134 -920 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 134 -918 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
```
- edit channel: 0 linkage errors, 0 failed event dispatches (both must be 0)
- phase: 6 parity world boot (12:05:52)
- parity world: view-distance=8 (the voxy leg's server value)
- parity world: joined session 3 (12:06:29)
- parity world: scan populated (cells=65536; the march paints from these buffers)
- parity world tour: 13 vantages (production pack, vd 8, shared structures)
- coverage sweeps: 72 eye passes = 36 positions (12 az x r144/200 + 12 off-az x r172, y=160) x 2 looks (outward+inward), settle-by-laps >= 6 s (q stable 3x1s), deadline 12 s/pos, no shots — the v6.26.0 data-coverage lever + v6.26.6 lottery kill; probe rides parity/view_probe.md
- parity drain-settle TIMEOUT: queue 17210 -> 17210 > 11500 after 150s — continuing anyway; the queue did not drain without eye proximity (priority-lane evidence, the 01/05/10 dip attribution reads from the tour deltas)
- pre-tour pass done: 13 vantages x 10s (the tour now reads the second OR-merge lap; the rim/dip transport race reads from the tour deltas)
- parity world tour done: 13 shots, 0 night-guard renames in parity/ (the gate pairs voxy/<name>.png vs parity/<name>.png)
- phase: 7 parity tour done (12:28:09)
- sparse re-beat channel at tour end: streamed=94650 q=0 dropped_stale=96 dropped_undecodable=5851 (frac=8 boot; undecodable-dominant = the rim words die in their decodability window, stale-dominant = edit churn owns the cells)
### parity world stream counters (session 3, tail + maxima)
[c-LoD] stream: tick 13000 65536/65536 cells, lap 1216, cone 64993 zone 23388 dirty 11890 (q 17210) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 8925686c/1801e, edits +18297p/13892c/2780e drop 0, spawned 0 / failed 0, cam0 yaw -180 pitch 5, wish 0 forced 901, native 29568c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 1512553p/3352b fail 0 ~0 kB/s cache 21290/24576 h1113504 m399049 e377759, srbeat 94650s/0q drop 96stale/5851undec, ereb 40381s/0q, sweep 0r/28d
[12:28:02 INFO]: [c-LoD] stream: tick 13000 65536/65536 cells, lap 1216, cone 64993 zone 23388 dirty 11890 (q 17210) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 8925686c/1801e, edits +18297p/13892c/2780e drop 0, spawned 0 / failed 0, cam0 yaw -180 pitch 5, wish 0 forced 901, native 29568c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 1512553p/3352b fail 0 ~0 kB/s cache 21290/24576 h1113504 m399049 e377759, srbeat 94650s/0q drop 96stale/5851undec, ereb 40381s/0q, sweep 0r/28d
- parity stream max cone: 64993
- parity stream max zone: 23388
- parity stream max wire bundle: bundle
### parity world capture lens (debug chain on the §7b world)
lens_red=0 lens_black=0 lens_water=0 lens_dark=0 lens_zero=0 rot=none scan=0 carriers=0
- parity mesh probe @png_y=2: x306=(151,184,255) x307=(151,184,255) x308=(151,184,255) x309h=(151,184,255) x310c=(151,184,255)
- parity mesh probe @png_y=717: x306=(53,66,54) x307=(46,66,53) x308=(49,70,55) x309h=(46,65,52) x310c=(47,67,53)
- parity mesh engagement: magenta-ish=0 of 921600 sampled px
- mask8: the un-fog engagement masks shot on the PARITY WORLD (vd 8) — parity_dbg8/01m|07m|10m
- mask8 01m: green=704 red=0 blue=99296 dim=12896 (px, sampled /16)
- mask8 07m: green=11872 red=0 blue=221440 dim=5824 (px, sampled /16)
- mask8 10m: green=5680 red=368 blue=230512 dim=134352 (px, sampled /16)
- mesh probe @png_y=2: x306=(255,255,255) x307=(13,254,0) x308=(0,254,0) x309h=(3,0,0) x310c=(0,0,0)
- mesh probe @png_y=717: x306=(255,255,255) x307=(13,254,0) x308=(0,254,0) x309h=(3,0,0) x310c=(0,0,0)
- mesh engagement: magenta-ish=0 of 921600 sampled px (the vanilla-path paint coverage)
- mask8 fog state @png_y=2: raw (300)=(151,184,255) (301)=(151,184,255)
- mask8 fog state @png_y=717: raw (300)=(0,128,8) (301)=(179,206,255)
- mask8 fog state decoded: envStart=0 envEnd=1024 rdEnd=128 fogColor=(179,206,255)
- buffer dump 10m: marked h=196/2048 c=48/2048; height windows r0<142:55 r1:27 r2>=202:114 maxByte=255
- buffer dump 10m: RANK-2-WINDOW HEIGHTS (the fake-wall class): (+26,-32)h=157, (+32,-32)h=157, (+62,-32)h=191, (+80,-32)h=159, (-14,-30)h=140, (+18,-30)h=185, (+36,-30)h=179, (+76,-30)h=191, (-14,-28)h=151, (+12,-28)h=181
- rim mask: scan populated (cells=65536; the state paint reads the same buffers the march reads)
- phase: 10 rim mask lane (12:32:22)
- rim mask: decode-state frames shot (rim_mask/00|01|12 — GREEN full decode / RED colorless / BLUE heightless; 00 = the leg-1 tower eye, 01 = the near-tower contrast, 12 = the tower re-read; the inherited 07/12 vantages sat 760 blocks outside the streamed disk and 01 sat INSIDE the marker column — their earlier verdicts were classifier noise, purged in v6.33.1)
- rim mask 00: green=39744 red=25216 blue=242800 yellow=57136 (px, sampled /16) -> DOMINANT BLUE: rim = HEIGHTS NEVER DECODED — a data-coverage class: the state paint cannot even see the cells
- rim mask 01: green=116624 red=19936 blue=245776 yellow=51856 (px, sampled /16) -> DOMINANT BLUE: rim = HEIGHTS NEVER DECODED — a data-coverage class: the state paint cannot even see the cells
- rim mask 12: green=79424 red=16864 blue=277328 yellow=60320 (px, sampled /16) -> DOMINANT BLUE: rim = HEIGHTS NEVER DECODED — a data-coverage class: the state paint cannot even see the cells

### boot / stream
```
[c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 149754 bytes, sha1 5dc283742995ead8546b0a2bbfadc892d3c58358 — zero textures, zero world data, one push per join
[c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[12:31:40 INFO]: [c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 149754 bytes, sha1 5dc283742995ead8546b0a2bbfadc892d3c58358 — zero textures, zero world data, one push per join
[12:31:40 INFO]: [c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[c-LoD] stream: tick 800 65536/65536 cells, lap 603, cone 10078 zone 843 dirty 30 (q 14680) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 465846c/2260e, edits +9p/9c/3e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 362 forced 556, native 24960c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 127865p/247b fail 0 ~21 kB/s cache 13950/24576 h113915 m13950 e0, srbeat 6737s/461q drop 0stale/787undec, ereb 321s/32q, sweep 0r/35d
[12:33:09 INFO]: [c-LoD] stream: tick 800 65536/65536 cells, lap 603, cone 10078 zone 843 dirty 30 (q 14680) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 465846c/2260e, edits +9p/9c/3e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 362 forced 556, native 24960c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 127865p/247b fail 0 ~21 kB/s cache 13950/24576 h113915 m13950 e0, srbeat 6737s/461q drop 0stale/787undec, ereb 321s/32q, sweep 0r/35d
```
### sessions
```
[12:32:01 INFO]: System chat: Stepan joined the game
[12:33:16 INFO]: System chat: Stepan left the game
```
### client log tail
```
[12:33:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:keep_alive] net.minecraft.network.protocol.common.ClientboundKeepAlivePacket -> 9 bytes
[12:33:14] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:keep_alive] net.minecraft.network.protocol.common.ServerboundKeepAlivePacket -> 9 bytes
[12:33:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[12:33:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[12:33:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:entity_position_sync] net.minecraft.network.protocol.game.ClientboundEntityPositionSyncPacket -> 38 bytes
[12:33:15] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:move_player_pos] net.minecraft.network.protocol.game.ServerboundMovePlayerPacket$Pos -> 26 bytes
[12:33:15] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[12:33:15] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[12:33:15] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[12:33:15] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[12:33:15] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[12:33:15] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[12:33:15] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[12:33:15] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[12:33:15] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[12:33:15] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[12:33:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[12:33:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[12:33:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[12:33:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[12:33:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[12:33:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[12:33:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:set_time] net.minecraft.network.protocol.game.ClientboundSetTimePacket -> 10 bytes
[12:33:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:bundle_delimiter] net.minecraft.network.protocol.game.ClientboundBundleDelimiterPacket -> 1 bytes
[12:33:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[12:33:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[12:33:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[12:33:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[12:33:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[12:33:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[12:33:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[12:33:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[12:33:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:bundle_delimiter] net.minecraft.network.protocol.game.ClientboundBundleDelimiterPacket -> 1 bytes
[12:33:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:block_update] net.minecraft.network.protocol.game.ClientboundBlockUpdatePacket -> 12 bytes
[12:33:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:keep_alive] net.minecraft.network.protocol.common.ClientboundKeepAlivePacket -> 9 bytes
[12:33:15] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:keep_alive] net.minecraft.network.protocol.common.ServerboundKeepAlivePacket -> 9 bytes
[12:33:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[12:33:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[12:33:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:entity_position_sync] net.minecraft.network.protocol.game.ClientboundEntityPositionSyncPacket -> 38 bytes
[12:33:15] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[12:33:15] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[12:33:15] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[12:33:15] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[12:33:15] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[12:33:15] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[12:33:15] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[12:33:15] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[12:33:15] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[12:33:15] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[12:33:16] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:block_update] net.minecraft.network.protocol.game.ClientboundBlockUpdatePacket -> 10 bytes
[12:33:16] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[12:33:16] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[12:33:16] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[12:33:16] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[12:33:16] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[12:33:16] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[12:33:16] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:move_player_pos] net.minecraft.network.protocol.game.ServerboundMovePlayerPacket$Pos -> 26 bytes
[12:33:16] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[12:33:16] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[12:33:16] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[12:33:16] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[12:33:16] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[12:33:16] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[12:33:16] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[12:33:16] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[12:33:16] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[12:33:16] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[12:33:16] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:set_time] net.minecraft.network.protocol.game.ClientboundSetTimePacket -> 10 bytes
[12:33:16] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:keep_alive] net.minecraft.network.protocol.common.ClientboundKeepAlivePacket -> 9 bytes
[12:33:16] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:keep_alive] net.minecraft.network.protocol.common.ServerboundKeepAlivePacket -> 9 bytes
[12:33:16] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:bundle_delimiter] net.minecraft.network.protocol.game.ClientboundBundleDelimiterPacket -> 1 bytes
[12:33:16] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[12:33:16] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[12:33:16] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[12:33:16] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[12:33:16] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[12:33:16] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[12:33:16] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[12:33:16] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[12:33:16] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:bundle_delimiter] net.minecraft.network.protocol.game.ClientboundBundleDelimiterPacket -> 1 bytes
--- exceptions (Realms/RTKit noise filtered) ---
25:[12:31:55] [Render thread/ERROR]: Error parsing option value 4 for option Simulation Distance: Value 4 outside of range [5:33]
49:[12:31:56] [Download-2/ERROR]: Failed to fetch user properties
50:com.mojang.authlib.exceptions.InvalidCredentialsException: Status: 401
51:	at com.mojang.authlib.exceptions.MinecraftClientHttpException.toAuthenticationException(MinecraftClientHttpException.java:70)
58:Caused by: MinecraftClientHttpException[type=HTTP_ERROR, status=401, response=ErrorResponse[path=/player/attributes, error=null, errorMessage=null, details=null]]
59:	at com.mojang.authlib.minecraft.client.MinecraftClient.buildHttpException(MinecraftClient.java:170)
```
### server log tail
```
[c-LoD] EB: enter, native_gen=true
[12:33:17 INFO]: [ChunkHolderManager] Saved 998 block chunks, 976 entity chunks, 0 poi chunks in world 'minecraft:overworld' in 0.78s
[12:33:17 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:overworld'
[12:33:17 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:overworld'
[12:33:17 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[12:33:17 INFO]: [ChunkHolderManager] Waiting 60s for chunk system to halt for world 'minecraft:the_nether'
[12:33:17 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:the_nether'
[12:33:17 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_nether'
[12:33:17 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_nether' in 0.00s
[12:33:17 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_nether'
[12:33:17 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_nether'
[12:33:17 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[12:33:17 INFO]: [ChunkHolderManager] Waiting 60s for chunk system to halt for world 'minecraft:the_end'
[12:33:17 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:the_end'
[12:33:17 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_end'
[12:33:17 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_end' in 0.00s
[12:33:17 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_end'
[12:33:17 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_end'
[12:33:17 INFO]: Waiting for all RegionFile I/O tasks to complete...
[12:33:17 INFO]: All RegionFile I/O tasks to complete
[12:33:17 INFO]: [MoonriseCommon] Awaiting termination of worker pool for up to 60s...
[12:33:17 INFO]: [MoonriseCommon] Awaiting termination of I/O pool for up to 60s...
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[launcher] server exited with code 143
```
### shots
```
-rw-r--r-- 1 runner runner 198124 Nov 17 12:05 13_after_relog_probe4.png
-rw-r--r-- 1 runner runner 194864 Nov 17 12:05 13_after_relog_probe5.png
-rw-r--r-- 1 runner runner  13773 Nov 17 12:04 14_leg2_baseline.png
-rw-r--r-- 1 runner runner  14802 Nov 17 12:05 14_leg2_burst_far.png
-rw-r--r-- 1 runner runner 145532 Nov 17 12:05 14_leg2_burst_greendust.png
-rw-r--r-- 1 runner runner  13773 Nov 17 12:04 14_leg2_burst_near.png
-rw-r--r-- 1 runner runner 192698 Nov 17 12:05 diff_after_relog.png
-rw-r--r-- 1 runner runner 222752 Nov 17 11:56 diff_live_edit.png
-rw-r--r-- 1 runner runner      0 Nov 17 11:53 ffmpeg.err
-rw-r--r-- 1 runner runner  10375 Nov 17 11:56 lens_09c_burst_close_retry.png
-rw-r--r-- 1 runner runner   9696 Nov 17 11:56 lens_12_after_edit_probe0.png
-rw-r--r-- 1 runner runner  11837 Nov 17 12:05 lens_13_after_relog_probe0.png
-rw-r--r-- 1 runner runner  12262 Nov 17 12:05 lens_13_after_relog_probe1.png
-rw-r--r-- 1 runner runner  11482 Nov 17 12:05 lens_13_after_relog_probe2.png
-rw-r--r-- 1 runner runner  11243 Nov 17 12:05 lens_13_after_relog_probe3.png
-rw-r--r-- 1 runner runner  11701 Nov 17 12:05 lens_13_after_relog_probe4.png
-rw-r--r-- 1 runner runner  11736 Nov 17 12:05 lens_13_after_relog_probe5.png
drwxr-xr-x 2 runner runner   4096 Nov 17 12:28 lens_parity
drwxr-xr-x 2 runner runner   4096 Nov 17 12:28 parity
drwxr-xr-x 2 runner runner   4096 Nov 17 11:59 parity_dbg
drwxr-xr-x 2 runner runner   4096 Nov 17 12:31 parity_dbg8
-rw-r--r-- 1 runner runner   1646 Nov 17 12:05 probe_counts.txt
drwxr-xr-x 2 runner runner   4096 Nov 17 12:33 rim_mask
-rw-r--r-- 1 runner runner  56880 Nov 17 12:33 server_full.log
-rw-r--r-- 1 runner runner  57569 Nov 17 12:33 summary.md
```

**exit=0** failures=0 leg1=1 leg2=1
