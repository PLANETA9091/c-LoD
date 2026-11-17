- phase: 0 sanity (13:08:52)
### c-LoD visual run (-930,796, marker=red_concrete, vantage -1005,150 yaw 270)
- GL:     Vendor: Mesa (0xffffffff);    Device: llvmpipe (LLVM 21.1.8, 256 bits) (0xffffffff);OpenGL renderer string: llvmpipe (LLVM 21.1.8, 256 bits);OpenGL core profile version string: 4.5 (Core Profile) Mesa 26.0.8-1ubuntu0.3;
- boot + gates OK (13:09:21)
- phase: 1 server booted (13:09:21)
- sky-grid armed
- phase: 2 leg-1 client joined (13:09:44)
- joined session 1 (13:09:44)
- stream live: cells=65536
- phase: 3 scan populated (13:10:07)
- terrain range: 2 terraced mountains at (CX-4,CZ-20)/(CX-2,CZ+16), tops 185/190, grass terraces + stone caps, 66..82 m from the eye (inside the 83.5 m band, chunk edge 64 m)
- scan complete=1 (cells=cells=65536/65536)
- paint settled=0 (wire 795892p)
- baseline try 1: drift=915 px
- lod_silhouette_cols=48 (LOD paint ON SCREEN at 75 m if > 20 — the march-drawn marker column; vanilla cannot render there, chunk edge 64 m; #255 re-measures 49, the ZONE_FAR=48 regression #256 and the control read 0)
- 3334 terrain-green px in the sky region (LOD MOUNTAINS on screen if > 5000; the range paints ColLow-green on low terraces via the fallback palette; flat-world baselines and the pack-free control read ~0)
- burst_close white px: 0 (renderer alive if > 50)
- burst_far white px: 524 (renderer alive in the marker zone if > 20)
- cut walk 40 m: 0 white px (> 20 = rendered)
- cut walk 60 m: 0 white px (> 20 = rendered)
- cut walk 100 m: 0 white px (> 20 = rendered)
- measured far-render cut: none m (G4 flap instrument; the decode proof needs >= 83 m from the y=151.6 eye)
- burst_close retry white px: 0 (preflight passes if > 50)
- preflight lens_zero=0 (1 = capture NEVER committed = no particle EVER rendered)
- preflight stage eyes: ?
- fill issued: 5x111x5 red_concrete at -930,796 (zone forceloaded ±48)
- phase: 4 marker filled (13:18:59)

### /clod-cells AFTER FILL (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 67 0 0x8400 0x2000
123 122 -940 784 71 0 1 [97, 119, 97] 67 0 0x8700 0x2000
124 122 -938 784 68 0 1 [97, 119, 97] 66 0 0x8400 0x2000
125 122 -936 784 62 1 1 [63, 118, 228] 106 0 0x7e00 0xd800
126 122 -934 784 62 0 1 [122, 122, 122] 105 0 0x7e00 0x2a00
127 122 -932 784 62 0 1 [122, 122, 122] 104 0 0x7e00 0x2a00
128 122 -930 784 62 0 1 [122, 122, 122] 104 0 0x7e00 0x2a00
129 122 -928 784 64 0 1 [134, 183, 131] 70 0 0x8000 0x4100
130 122 -926 784 66 0 1 [134, 183, 131] 70 0 0x8200 0x4100
131 122 -924 784 75 0 1 [97, 119, 97] 70 0 0x8b00 0x2000
132 122 -922 784 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 122 -920 784 75 0 1 [97, 119, 97] 18446744073709551615 1 0x8b00 0x2000
134 122 -918 784 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x2000
122 123 -942 786 70 0 1 [97, 119, 97] 67 0 0x8600 0x2000
123 123 -940 786 65 0 1 [97, 119, 97] 67 0 0x8100 0x2000
124 123 -938 786 65 0 1 [97, 119, 97] 66 0 0x8100 0x2000
125 123 -936 786 62 1 1 [63, 118, 228] 106 0 0x7e00 0xd800
126 123 -934 786 62 0 1 [122, 122, 122] 105 0 0x7e00 0x2a00
127 123 -932 786 62 0 1 [122, 122, 122] 105 0 0x7e00 0x2a00
128 123 -930 786 62 0 1 [122, 122, 122] 104 0 0x7e00 0x2a00
129 123 -928 786 73 0 1 [97, 119, 97] 70 0 0x8900 0x2000
130 123 -926 786 67 0 1 [134, 183, 131] 70 0 0x8300 0x4100
131 123 -924 786 69 0 1 [134, 183, 131] 70 0 0x8500 0x4100
132 123 -922 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 123 -920 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 123 -918 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
122 124 -942 788 72 0 1 [97, 119, 97] 67 0 0x8800 0x2000
123 124 -940 788 70 0 1 [97, 119, 97] 67 0 0x8600 0x2000
124 124 -938 788 68 0 1 [97, 119, 97] 66 0 0x8400 0x2000
125 124 -936 788 62 0 1 [122, 122, 122] 106 0 0x7e00 0x2a00
126 124 -934 788 62 0 1 [122, 122, 122] 105 0 0x7e00 0x2a00
127 124 -932 788 62 1 1 [63, 118, 228] 105 0 0x7e00 0xd800
128 124 -930 788 73 0 1 [97, 119, 97] 64 0 0x8900 0x2000
129 124 -928 788 76 0 1 [97, 119, 97] 70 0 0x8c00 0x2000
130 124 -926 788 73 0 1 [97, 119, 97] 70 0 0x8900 0x2000
131 124 -924 788 69 0 1 [134, 183, 131] 70 0 0x8500 0x4100
132 124 -922 788 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 124 -920 788 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 124 -918 788 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x2000
122 125 -942 790 74 0 1 [97, 119, 97] 67 0 0x8a00 0x2000
123 125 -940 790 73 0 1 [97, 119, 97] 67 0 0x8900 0x2000
124 125 -938 790 71 0 1 [97, 119, 97] 66 0 0x8700 0x2000
125 125 -936 790 62 0 1 [122, 122, 122] 106 0 0x7e00 0x2a00
126 125 -934 790 62 0 1 [122, 122, 122] 105 0 0x7e00 0x2a00
127 125 -932 790 62 1 1 [63, 118, 228] 105 0 0x7e00 0xd800
128 125 -930 790 62 1 1 [63, 118, 228] 104 0 0x7e00 0xd800
129 125 -928 790 73 0 1 [97, 119, 97] 64 0 0x8900 0x2000
130 125 -926 790 68 0 1 [134, 183, 131] 70 0 0x8400 0x4100
131 125 -924 790 69 0 1 [134, 183, 131] 70 0 0x8500 0x4100
132 125 -922 790 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
133 125 -920 790 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 125 -918 790 70 0 1 [134, 183, 131] 18446744073709551615 1 0x8600 0x4100
122 126 -942 792 74 0 1 [97, 119, 97] 67 0 0x8a00 0x2000
123 126 -940 792 73 0 1 [97, 119, 97] 67 0 0x8900 0x2000
124 126 -938 792 68 0 1 [97, 119, 97] 66 0 0x8400 0x2000
125 126 -936 792 62 0 1 [122, 122, 122] 106 0 0x7e00 0x2a00
126 126 -934 792 62 0 1 [122, 122, 122] 105 0 0x7e00 0x2a00
127 126 -932 792 62 0 1 [122, 122, 122] 105 0 0x7e00 0x2a00
128 126 -930 792 62 0 1 [122, 122, 122] 104 0 0x7e00 0x2a00
129 126 -928 792 67 0 1 [134, 183, 131] 64 0 0x8300 0x4100
130 126 -926 792 74 0 1 [97, 119, 97] 70 0 0x8a00 0x2000
131 126 -924 792 77 0 1 [97, 119, 97] 70 0 0x8d00 0x2000
132 126 -922 792 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 126 -920 792 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3200
134 126 -918 792 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x5000
122 127 -942 794 67 0 1 [122, 122, 122] 107 0 0x8300 0x2a00
123 127 -940 794 68 0 1 [122, 122, 122] 107 0 0x8400 0x2a00
124 127 -938 794 65 0 1 [134, 183, 131] 66 0 0x8100 0x4100
125 127 -936 794 62 0 1 [122, 122, 122] 106 0 0x7e00 0x2a00
126 127 -934 794 62 0 1 [122, 122, 122] 105 0 0x7e00 0x2a00
127 127 -932 794 62 1 1 [63, 118, 228] 105 0 0x7e00 0xd800
128 127 -930 794 62 1 1 [63, 118, 228] 104 0 0x7e00 0xd800
129 127 -928 794 66 0 1 [134, 183, 131] 64 0 0x8200 0x4100
130 127 -926 794 74 0 1 [97, 119, 97] 70 0 0x8a00 0x2000
131 127 -924 794 77 0 1 [97, 119, 97] 70 0 0x8d00 0x2000
132 127 -922 794 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 127 -920 794 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3200
134 127 -918 794 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x5000
122 128 -942 796 68 0 1 [122, 122, 122] 108 0 0x8400 0x2a00
123 128 -940 796 72 0 1 [97, 119, 97] 67 0 0x8800 0x2000
124 128 -938 796 75 0 1 [122, 122, 122] 106 0 0x8b00 0x2a00
125 128 -936 796 72 0 1 [97, 119, 97] 66 0 0x8800 0x2000
126 128 -934 796 62 0 1 [122, 122, 122] 105 0 0x7e00 0x2a00
127 128 -932 796 62 1 1 [63, 118, 228] 105 0 0x7e00 0xd800
128 128 -930 796 62 1 1 [63, 118, 228] 104 0 0x7e00 0xd800
129 128 -928 796 66 0 1 [134, 183, 131] 64 0 0x8200 0x4100
130 128 -926 796 68 0 1 [134, 183, 131] 70 0 0x8400 0x4100
131 128 -924 796 74 0 1 [97, 119, 97] 70 0 0x8a00 0x2000
132 128 -922 796 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 128 -920 796 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
134 128 -918 796 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 129 -942 798 69 0 1 [122, 122, 122] 107 0 0x8500 0x2a00
123 129 -940 798 75 0 1 [97, 119, 97] 67 0 0x8b00 0x2000
124 129 -938 798 78 0 1 [97, 119, 97] 68 0 0x8e00 0x2000
125 129 -936 798 75 0 1 [97, 119, 97] 66 0 0x8b00 0x2000
126 129 -934 798 62 0 1 [122, 122, 122] 105 0 0x7e00 0x2a00
127 129 -932 798 62 1 1 [63, 118, 228] 105 0 0x7e00 0xd800
128 129 -930 798 62 1 1 [63, 118, 228] 104 0 0x7e00 0xd800
129 129 -928 798 71 0 1 [97, 119, 97] 64 0 0x8700 0x2000
130 129 -926 798 71 0 1 [97, 119, 97] 70 0 0x8700 0x2000
131 129 -924 798 69 0 1 [134, 183, 131] 70 0 0x8500 0x4100
132 129 -922 798 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 129 -920 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 129 -918 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 130 -942 800 70 0 1 [134, 183, 131] 67 0 0x8600 0x4100
123 130 -940 800 76 0 1 [97, 119, 97] 67 0 0x8c00 0x2000
124 130 -938 800 76 0 1 [97, 119, 97] 66 0 0x8c00 0x2000
125 130 -936 800 72 0 1 [97, 119, 97] 66 0 0x8800 0x2000
126 130 -934 800 62 1 1 [63, 118, 228] 105 0 0x7e00 0xd800
127 130 -932 800 62 1 1 [63, 118, 228] 105 0 0x7e00 0xd800
128 130 -930 800 62 1 1 [63, 118, 228] 104 0 0x7e00 0xd800
129 130 -928 800 71 0 1 [97, 119, 97] 64 0 0x8700 0x2000
130 130 -926 800 71 0 1 [97, 119, 97] 70 0 0x8700 0x2000
131 130 -924 800 69 0 1 [134, 183, 131] 70 0 0x8500 0x4100
132 130 -922 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
133 130 -920 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 130 -918 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 131 -942 802 78 0 1 [97, 119, 97] 0 0 0x8e00 0x2000
123 131 -940 802 77 0 1 [97, 119, 97] 67 0 0x8d00 0x2000
124 131 -938 802 77 0 1 [97, 119, 97] 66 0 0x8d00 0x2000
125 131 -936 802 66 0 1 [134, 183, 131] 66 0 0x8200 0x4100
126 131 -934 802 62 1 1 [63, 118, 228] 105 0 0x7e00 0xd800
127 131 -932 802 62 1 1 [63, 118, 228] 105 0 0x7e00 0xd800
128 131 -930 802 62 1 1 [63, 118, 228] 104 0 0x7e00 0xd800
129 131 -928 802 62 1 1 [63, 118, 228] 3 0 0x7e00 0xd800
130 131 -926 802 66 0 1 [134, 183, 131] 70 0 0x8200 0x4100
131 131 -924 802 68 0 1 [134, 183, 131] 70 0 0x8400 0x4100
132 131 -922 802 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 131 -920 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 131 -918 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 132 -942 804 79 0 1 [97, 119, 97] 0 0 0x8f00 0x2000
123 132 -940 804 76 0 1 [97, 119, 97] 67 0 0x8c00 0x2000
124 132 -938 804 76 0 1 [97, 119, 97] 66 0 0x8c00 0x2000
125 132 -936 804 76 0 1 [97, 119, 97] 66 0 0x8c00 0x2000
126 132 -934 804 62 1 1 [63, 118, 228] 105 0 0x7e00 0xd800
127 132 -932 804 62 1 1 [63, 118, 228] 105 0 0x7e00 0xd800
128 132 -930 804 62 1 1 [63, 118, 228] 104 0 0x7e00 0xd800
129 132 -928 804 62 1 1 [63, 118, 228] 110 0 0x7e00 0xd800
130 132 -926 804 65 0 1 [134, 183, 131] 70 0 0x8100 0x4100
131 132 -924 804 77 0 1 [97, 119, 97] 70 0 0x8d00 0x2000
132 132 -922 804 79 0 1 [97, 119, 97] 70 0 0x8f00 0x2000
133 132 -920 804 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 132 -918 804 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x2000
122 133 -942 806 78 0 1 [97, 119, 97] 0 0 0x8e00 0x2000
123 133 -940 806 69 0 1 [134, 183, 131] 67 0 0x8500 0x4100
124 133 -938 806 155 0 1 [134, 183, 131] 67 0 0xdb00 0x4100
125 133 -936 806 155 0 1 [134, 183, 131] 67 0 0xdb00 0x4100
126 133 -934 806 155 0 1 [134, 183, 131] 66 0 0xdb00 0x4100
127 133 -932 806 155 0 1 [134, 183, 131] 66 0 0xdb00 0x4100
128 133 -930 806 155 0 1 [134, 183, 131] 65 0 0xdb00 0x4100
129 133 -928 806 155 0 1 [134, 183, 131] 64 0 0xdb00 0x4100
130 133 -926 806 155 0 1 [134, 183, 131] 64 0 0xdb00 0x4100
131 133 -924 806 67 0 1 [134, 183, 131] 72 0 0x8300 0x4100
132 133 -922 806 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 133 -920 806 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 133 -918 806 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x2000
122 134 -942 808 79 0 1 [97, 119, 97] 0 0 0x8f00 0x2000
123 134 -940 808 68 0 1 [134, 183, 131] 67 0 0x8400 0x4100
124 134 -938 808 155 0 1 [134, 183, 131] 67 0 0xdb00 0x4100
125 134 -936 808 180 0 1 [134, 183, 131] 64 0 0xf400 0x4100
126 134 -934 808 180 0 1 [134, 183, 131] 64 0 0xf400 0x4100
127 134 -932 808 180 0 1 [134, 183, 131] 72 0 0xf400 0x4100
128 134 -930 808 180 0 1 [134, 183, 131] 72 0 0xf400 0x4100
129 134 -928 808 180 0 1 [134, 183, 131] 72 0 0xf400 0x4100
130 134 -926 808 155 0 1 [134, 183, 131] 64 0 0xdb00 0x4100
131 134 -924 808 67 0 1 [134, 183, 131] 72 0 0x8300 0x4100
132 134 -922 808 68 0 1 [134, 183, 131] 18446744073709551615 1 0x8400 0x4100
133 134 -920 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 134 -918 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
```
### stream diag after fill:
[c-LoD] stream: tick 5700 65536/65536 cells, lap 928, cone 45164 zone 2783 dirty 942 (q 11839) ewin 203 rbeat 0p/0 rearm 0c/0t rescan 1498850c/1855e, edits +6279p/4760c/661e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 314 forced 198, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 973760p/2341b fail 0 ~58 kB/s cache 13558/24576 h960202 m13558 e0, srbeat 73371s/0q drop 68stale/106undec, ereb 48518s/0q, sweep 0r/64d
[13:19:03 INFO]: [c-LoD] stream: tick 5700 65536/65536 cells, lap 928, cone 45164 zone 2783 dirty 942 (q 11839) ewin 203 rbeat 0p/0 rearm 0c/0t rescan 1498850c/1855e, edits +6279p/4760c/661e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 314 forced 198, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 973760p/2341b fail 0 ~58 kB/s cache 13558/24576 h960202 m13558 e0, srbeat 73371s/0q drop 68stale/106undec, ereb 48518s/0q, sweep 0r/64d
- carrier ladder y=155: 22464 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=165: 22464 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=168: 22464 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=172: 22464 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=176: 22464 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=180: 22464 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=185: 22464 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- probe 0: red=4715 paint=6387 lod_silhouette_cols=42 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=5 hits_hi=18 mask_hits=30 (need red>=60 paint>=300 lod>20)
- **LEG 1 PASS** (zero relogs)
- phase: 5 leg-1 probes done (13:19:34)
### stream diag after probes:
[13:19:23 INFO]: [c-LoD] stream: tick 5900 65536/65536 cells, lap 928, cone 45164 zone 2783 dirty 960 (q 11859) ewin 203 rbeat 0p/0 rearm 0c/0t rescan 1563399c/1881e, edits +6676p/5059c/774e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 314 forced 198, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 975218p/2361b fail 0 ~3 kB/s cache 13639/24576 h961579 m13639 e0, srbeat 73533s/0q drop 68stale/106undec, ereb 48662s/9q, sweep 0r/64d
[c-LoD] stream: tick 6000 65536/65536 cells, lap 928, cone 45164 zone 2783 dirty 969 (q 11852) ewin 103 rbeat 0p/0 rearm 0c/0t rescan 1594102c/1885e, edits +6800p/5130c/794e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 314 forced 198, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 975947p/2371b fail 0 ~3 kB/s cache 13639/24576 h962308 m13639 e0, srbeat 73614s/0q drop 68stale/106undec, ereb 48734s/9q, sweep 0r/64d
[13:19:33 INFO]: [c-LoD] stream: tick 6000 65536/65536 cells, lap 928, cone 45164 zone 2783 dirty 969 (q 11852) ewin 103 rbeat 0p/0 rearm 0c/0t rescan 1594102c/1885e, edits +6800p/5130c/794e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 314 forced 198, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 975947p/2371b fail 0 ~3 kB/s cache 13639/24576 h962308 m13639 e0, srbeat 73614s/0q drop 68stale/106undec, ereb 48734s/9q, sweep 0r/64d
- parity debug tour: 13 vantages (session-1 client, DEBUG pack — evidence only, the gate pairs the §7b world session)
- parity debug tour done: 13 shots in parity_dbg/ (the §7b world session feeds the gate)
- joined session 2 (13:22:25)
- leg-2 baseline scene px (x<940): 0 (0 = the relog client paints no world at all)
- leg-2 renderer probe NEAR (20 m) white px: 0 raw / 0 changed-vs-baseline (> 20 = the client renders particles)
- leg-2 renderer probe FAR (80 m, marker zone) white px: 553 raw / 553 changed-vs-baseline (> 20 = no distance cut; ~0 with NEAR alive = particles beyond the client chunk radius do not render)
- leg-2 carrier probe green dust (80 m): changed-green px 0 / total changed 789 vs baseline (> 100 green = a stable green carrier exists; total > 0 with green ~0 = the dust renders but fog washes it — a ruler problem, not a carrier problem)
- phase: 8 leg-2 relog probes (13:27:50)
- relog probe 0: red=0 scene=16 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=0 mask_hits=0 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 1: red=7864 scene=131108 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=6 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- **LEG 2 PASS** (marker persisted over relog)
- phase: 9 leg-2 done (13:28:02)

### client log forensics (pipeline/particle/atlas — v6.8.2)
```
[13:22:14] [main/DEBUG]: Loading LWJGL system
WARNING: java.lang.System::load has been called by org.lwjgl.system.Library$$Lambda/0x00000000081e3d60 in an unnamed module (file:/home/runner/mc-test/client/libraries/org/lwjgl/lwjgl/3.4.3/lwjgl-3.4.3.jar)
WARNING: sun.misc.Unsafe::objectFieldOffset has been called by org.lwjgl.system.MemoryBackendUnsafeLegacy (file:/home/runner/mc-test/client/libraries/org/lwjgl/lwjgl/3.4.3/lwjgl-3.4.3.jar)
WARNING: Please consider reporting this to the maintainers of class org.lwjgl.system.MemoryBackendUnsafeLegacy
[13:22:14] [main/DEBUG]: Loading OpenGL
[13:22:14] [main/DEBUG]: Loading shaderc
[13:22:19] [Render thread/ERROR]: Error parsing option value 4 for option Simulation Distance: Value 4 outside of range [5:33]
[13:22:19] [Render thread/INFO]: Backend library: LWJGL version 3.4.3+4
[13:22:19] [Render thread/INFO]: Using graphics backend OpenGL, using drivers: 4.5 (Core Profile) Mesa 26.0.8-1ubuntu0.3
[13:22:19] [Render thread/INFO]: Using graphics device: llvmpipe (LLVM 21.1.8, 256 bits) (Mesa)
[13:22:19] [Render thread/INFO]: Using graphics device extensions: GL_ARB_multi_draw_indirect, GL_ARB_buffer_storage, GL_ARB_shader_draw_parameters, GL_ARB_base_instance, GL_KHR_debug, GL_ARB_draw_indirect, GL_ARB_clip_control, GL_ARB_vertex_attrib_binding, GL_ARB_direct_state_access, GL_EXT_texture_filter_anisotropic
[13:22:20] [Download-2/ERROR]: Failed to fetch user properties
com.mojang.authlib.exceptions.InvalidCredentialsException: Status: 401
	at com.mojang.authlib.exceptions.MinecraftClientHttpException.toAuthenticationException(MinecraftClientHttpException.java:70)
Caused by: MinecraftClientHttpException[type=HTTP_ERROR, status=401, response=ErrorResponse[path=/player/attributes, error=null, errorMessage=null, details=null]]
	at com.mojang.authlib.minecraft.client.MinecraftClient.buildHttpException(MinecraftClient.java:170)
[13:22:20] [Download-1/INFO]: Could not authorize you against Realms server: java.lang.RuntimeException: Failed to parse into SignedJWT: 0
[13:22:20] [Download-1/ERROR]: Failed to fetch Realms feature flags
com.mojang.realmsclient.exception.RealmsServiceException: Realms authentication error with message 'java.lang.RuntimeException: Failed to parse into SignedJWT: 0'
[13:22:20] [Render thread/INFO]: Reloading ResourceManager: vanilla, vanilla
[13:22:23] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[13:22:23] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[13:22:23] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[13:22:23] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[13:22:23] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[13:22:23] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[13:22:23] [Render thread/INFO]: Created: 2048x2048x4 minecraft:textures/atlas/blocks.png-atlas
[13:22:23] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[13:22:23] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[13:22:23] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[13:22:23] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[13:22:23] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
[13:22:30] [Render thread/INFO]: Reloading ResourceManager: vanilla, vanilla, server/00000000/6b1f2f7e-9c0d-4a4b-8b0a-b10d1b4ab100
[13:22:32] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[13:22:32] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[13:22:32] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[13:22:32] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[13:22:32] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[13:22:32] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[13:22:32] [Render thread/INFO]: Created: 2048x2048x4 minecraft:textures/atlas/blocks.png-atlas
[13:22:32] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[13:22:32] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[13:22:32] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[13:22:32] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[13:22:32] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
```

### /clod-cells AFTER PROBES (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 27 0 0x8400 0x2000
123 122 -940 784 71 0 1 [97, 119, 97] 27 0 0x8700 0x2000
124 122 -938 784 68 0 1 [97, 119, 97] 27 0 0x8400 0x2000
125 122 -936 784 62 1 1 [63, 118, 228] 27 0 0x7e00 0xd800
126 122 -934 784 62 0 1 [122, 122, 122] 27 0 0x7e00 0x2a00
127 122 -932 784 62 0 1 [122, 122, 122] 27 0 0x7e00 0x2a00
128 122 -930 784 62 0 1 [122, 122, 122] 27 0 0x7e00 0x2a00
129 122 -928 784 64 0 1 [134, 183, 131] 27 0 0x8000 0x4100
130 122 -926 784 66 0 1 [134, 183, 131] 27 0 0x8200 0x4100
131 122 -924 784 75 0 1 [97, 119, 97] 27 0 0x8b00 0x2000
132 122 -922 784 77 0 1 [97, 119, 97] 4102 1 0x8d00 0x2000
133 122 -920 784 75 0 1 [97, 119, 97] 4102 1 0x8b00 0x2000
134 122 -918 784 73 0 1 [97, 119, 97] 4102 1 0x8900 0x2000
122 123 -942 786 70 0 1 [97, 119, 97] 27 0 0x8600 0x2000
123 123 -940 786 65 0 1 [97, 119, 97] 27 0 0x8100 0x2000
124 123 -938 786 65 0 1 [97, 119, 97] 27 0 0x8100 0x2000
125 123 -936 786 62 1 1 [63, 118, 228] 27 0 0x7e00 0xd800
126 123 -934 786 62 0 1 [122, 122, 122] 27 0 0x7e00 0x2a00
127 123 -932 786 62 0 1 [122, 122, 122] 27 0 0x7e00 0x2a00
128 123 -930 786 62 0 1 [122, 122, 122] 27 0 0x7e00 0x2a00
129 123 -928 786 73 0 1 [97, 119, 97] 27 0 0x8900 0x2000
130 123 -926 786 67 0 1 [134, 183, 131] 27 0 0x8300 0x4100
131 123 -924 786 69 0 1 [134, 183, 131] 27 0 0x8500 0x4100
132 123 -922 786 77 0 1 [97, 119, 97] 4102 1 0x8d00 0x2000
133 123 -920 786 77 0 1 [97, 119, 97] 4102 1 0x8d00 0x2000
134 123 -918 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
122 124 -942 788 72 0 1 [97, 119, 97] 27 0 0x8800 0x2000
123 124 -940 788 70 0 1 [97, 119, 97] 27 0 0x8600 0x2000
124 124 -938 788 68 0 1 [97, 119, 97] 27 0 0x8400 0x2000
125 124 -936 788 62 0 1 [122, 122, 122] 27 0 0x7e00 0x2a00
126 124 -934 788 62 0 1 [122, 122, 122] 26 0 0x7e00 0x2a00
127 124 -932 788 62 1 1 [63, 118, 228] 26 0 0x7e00 0xd800
128 124 -930 788 73 0 1 [97, 119, 97] 26 0 0x8900 0x2000
129 124 -928 788 76 0 1 [97, 119, 97] 26 0 0x8c00 0x2000
130 124 -926 788 73 0 1 [97, 119, 97] 26 0 0x8900 0x2000
131 124 -924 788 69 0 1 [134, 183, 131] 26 0 0x8500 0x4100
132 124 -922 788 77 0 1 [97, 119, 97] 4102 1 0x8d00 0x2000
133 124 -920 788 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 124 -918 788 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x2000
122 125 -942 790 74 0 1 [97, 119, 97] 27 0 0x8a00 0x2000
123 125 -940 790 73 0 1 [97, 119, 97] 26 0 0x8900 0x2000
124 125 -938 790 71 0 1 [97, 119, 97] 26 0 0x8700 0x2000
125 125 -936 790 62 0 1 [122, 122, 122] 26 0 0x7e00 0x2a00
126 125 -934 790 62 0 1 [122, 122, 122] 26 0 0x7e00 0x2a00
127 125 -932 790 62 1 1 [63, 118, 228] 26 0 0x7e00 0xd800
128 125 -930 790 62 1 1 [63, 118, 228] 26 0 0x7e00 0xd800
129 125 -928 790 73 0 1 [97, 119, 97] 26 0 0x8900 0x2000
130 125 -926 790 68 0 1 [134, 183, 131] 26 0 0x8400 0x4100
131 125 -924 790 69 0 1 [134, 183, 131] 26 0 0x8500 0x4100
132 125 -922 790 69 0 1 [134, 183, 131] 4102 1 0x8500 0x4100
133 125 -920 790 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 125 -918 790 70 0 1 [134, 183, 131] 18446744073709551615 1 0x8600 0x4100
122 126 -942 792 74 0 1 [97, 119, 97] 27 0 0x8a00 0x2000
123 126 -940 792 73 0 1 [97, 119, 97] 26 0 0x8900 0x2000
124 126 -938 792 68 0 1 [97, 119, 97] 26 0 0x8400 0x2000
125 126 -936 792 62 0 1 [122, 122, 122] 26 0 0x7e00 0x2a00
126 126 -934 792 62 0 1 [122, 122, 122] 26 0 0x7e00 0x2a00
127 126 -932 792 62 0 1 [122, 122, 122] 26 0 0x7e00 0x2a00
128 126 -930 792 62 0 1 [122, 122, 122] 26 0 0x7e00 0x2a00
129 126 -928 792 67 0 1 [134, 183, 131] 26 0 0x8300 0x4100
130 126 -926 792 74 0 1 [97, 119, 97] 26 0 0x8a00 0x2000
131 126 -924 792 77 0 1 [97, 119, 97] 26 0 0x8d00 0x2000
132 126 -922 792 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 126 -920 792 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3200
134 126 -918 792 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x5000
122 127 -942 794 67 0 1 [122, 122, 122] 27 0 0x8300 0x2a00
123 127 -940 794 68 0 1 [122, 122, 122] 26 0 0x8400 0x2a00
124 127 -938 794 65 0 1 [134, 183, 131] 26 0 0x8100 0x4100
125 127 -936 794 62 0 1 [122, 122, 122] 26 0 0x7e00 0x2a00
126 127 -934 794 62 0 1 [122, 122, 122] 26 0 0x7e00 0x2a00
127 127 -932 794 170 0 1 [142, 33, 33] 26 0 0xea00 0x0a00
128 127 -930 794 170 0 1 [142, 33, 33] 26 0 0xea00 0x0a00
129 127 -928 794 170 0 1 [142, 33, 33] 26 0 0xea00 0x0a00
130 127 -926 794 74 0 1 [97, 119, 97] 26 0 0x8a00 0x2000
131 127 -924 794 77 0 1 [97, 119, 97] 26 0 0x8d00 0x2000
132 127 -922 794 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 127 -920 794 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3200
134 127 -918 794 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x5000
122 128 -942 796 68 0 1 [122, 122, 122] 27 0 0x8400 0x2a00
123 128 -940 796 72 0 1 [97, 119, 97] 26 0 0x8800 0x2000
124 128 -938 796 75 0 1 [122, 122, 122] 26 0 0x8b00 0x2a00
125 128 -936 796 72 0 1 [97, 119, 97] 26 0 0x8800 0x2000
126 128 -934 796 62 0 1 [122, 122, 122] 26 0 0x7e00 0x2a00
127 128 -932 796 170 0 1 [142, 33, 33] 26 0 0xea00 0x0a00
128 128 -930 796 170 0 1 [142, 33, 33] 26 0 0xea00 0x0a00
129 128 -928 796 170 0 1 [142, 33, 33] 26 0 0xea00 0x0a00
130 128 -926 796 68 0 1 [134, 183, 131] 26 0 0x8400 0x4100
131 128 -924 796 74 0 1 [97, 119, 97] 26 0 0x8a00 0x2000
132 128 -922 796 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 128 -920 796 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
134 128 -918 796 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 129 -942 798 69 0 1 [122, 122, 122] 27 0 0x8500 0x2a00
123 129 -940 798 75 0 1 [97, 119, 97] 26 0 0x8b00 0x2000
124 129 -938 798 78 0 1 [97, 119, 97] 27 0 0x8e00 0x2000
125 129 -936 798 75 0 1 [97, 119, 97] 26 0 0x8b00 0x2000
126 129 -934 798 62 0 1 [122, 122, 122] 26 0 0x7e00 0x2a00
127 129 -932 798 170 0 1 [142, 33, 33] 26 0 0xea00 0x0a00
128 129 -930 798 170 0 1 [142, 33, 33] 26 0 0xea00 0x0a00
129 129 -928 798 170 0 1 [142, 33, 33] 26 0 0xea00 0x0a00
130 129 -926 798 71 0 1 [97, 119, 97] 26 0 0x8700 0x2000
131 129 -924 798 69 0 1 [134, 183, 131] 26 0 0x8500 0x4100
132 129 -922 798 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 129 -920 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 129 -918 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 130 -942 800 70 0 1 [134, 183, 131] 27 0 0x8600 0x4100
123 130 -940 800 76 0 1 [97, 119, 97] 26 0 0x8c00 0x2000
124 130 -938 800 76 0 1 [97, 119, 97] 26 0 0x8c00 0x2000
125 130 -936 800 72 0 1 [97, 119, 97] 26 0 0x8800 0x2000
126 130 -934 800 62 1 1 [63, 118, 228] 26 0 0x7e00 0xd800
127 130 -932 800 62 1 1 [63, 118, 228] 26 0 0x7e00 0xd800
128 130 -930 800 62 1 1 [63, 118, 228] 26 0 0x7e00 0xd800
129 130 -928 800 71 0 1 [97, 119, 97] 26 0 0x8700 0x2000
130 130 -926 800 71 0 1 [97, 119, 97] 26 0 0x8700 0x2000
131 130 -924 800 69 0 1 [134, 183, 131] 26 0 0x8500 0x4100
132 130 -922 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
133 130 -920 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 130 -918 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 131 -942 802 78 0 1 [97, 119, 97] 28 0 0x8e00 0x2000
123 131 -940 802 77 0 1 [97, 119, 97] 26 0 0x8d00 0x2000
124 131 -938 802 77 0 1 [97, 119, 97] 26 0 0x8d00 0x2000
125 131 -936 802 66 0 1 [134, 183, 131] 26 0 0x8200 0x4100
126 131 -934 802 62 1 1 [63, 118, 228] 26 0 0x7e00 0xd800
127 131 -932 802 62 1 1 [63, 118, 228] 26 0 0x7e00 0xd800
128 131 -930 802 62 1 1 [63, 118, 228] 26 0 0x7e00 0xd800
129 131 -928 802 62 1 1 [63, 118, 228] 26 0 0x7e00 0xd800
130 131 -926 802 66 0 1 [134, 183, 131] 26 0 0x8200 0x4100
131 131 -924 802 68 0 1 [134, 183, 131] 26 0 0x8400 0x4100
132 131 -922 802 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 131 -920 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 131 -918 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 132 -942 804 79 0 1 [97, 119, 97] 28 0 0x8f00 0x2000
123 132 -940 804 76 0 1 [97, 119, 97] 26 0 0x8c00 0x2000
124 132 -938 804 76 0 1 [97, 119, 97] 26 0 0x8c00 0x2000
125 132 -936 804 76 0 1 [97, 119, 97] 26 0 0x8c00 0x2000
126 132 -934 804 62 1 1 [63, 118, 228] 26 0 0x7e00 0xd800
127 132 -932 804 62 1 1 [63, 118, 228] 26 0 0x7e00 0xd800
128 132 -930 804 62 1 1 [63, 118, 228] 26 0 0x7e00 0xd800
129 132 -928 804 62 1 1 [63, 118, 228] 26 0 0x7e00 0xd800
130 132 -926 804 65 0 1 [134, 183, 131] 26 0 0x8100 0x4100
131 132 -924 804 77 0 1 [97, 119, 97] 26 0 0x8d00 0x2000
132 132 -922 804 79 0 1 [97, 119, 97] 26 0 0x8f00 0x2000
133 132 -920 804 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 132 -918 804 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x2000
122 133 -942 806 78 0 1 [97, 119, 97] 28 0 0x8e00 0x2000
123 133 -940 806 69 0 1 [134, 183, 131] 26 0 0x8500 0x4100
124 133 -938 806 155 0 1 [134, 183, 131] 27 0 0xdb00 0x4100
125 133 -936 806 155 0 1 [134, 183, 131] 26 0 0xdb00 0x4100
126 133 -934 806 155 0 1 [134, 183, 131] 26 0 0xdb00 0x4100
127 133 -932 806 155 0 1 [134, 183, 131] 26 0 0xdb00 0x4100
128 133 -930 806 155 0 1 [134, 183, 131] 26 0 0xdb00 0x4100
129 133 -928 806 155 0 1 [134, 183, 131] 26 0 0xdb00 0x4100
130 133 -926 806 155 0 1 [134, 183, 131] 26 0 0xdb00 0x4100
131 133 -924 806 67 0 1 [134, 183, 131] 26 0 0x8300 0x4100
132 133 -922 806 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 133 -920 806 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 133 -918 806 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x2000
122 134 -942 808 79 0 1 [97, 119, 97] 28 0 0x8f00 0x2000
123 134 -940 808 68 0 1 [134, 183, 131] 26 0 0x8400 0x4100
124 134 -938 808 155 0 1 [134, 183, 131] 27 0 0xdb00 0x4100
125 134 -936 808 180 0 1 [134, 183, 131] 26 0 0xf400 0x4100
126 134 -934 808 180 0 1 [134, 183, 131] 26 0 0xf400 0x4100
127 134 -932 808 180 0 1 [134, 183, 131] 26 0 0xf400 0x4100
128 134 -930 808 180 0 1 [134, 183, 131] 26 0 0xf400 0x4100
129 134 -928 808 180 0 1 [134, 183, 131] 26 0 0xf400 0x4100
130 134 -926 808 155 0 1 [134, 183, 131] 26 0 0xdb00 0x4100
131 134 -924 808 67 0 1 [134, 183, 131] 26 0 0x8300 0x4100
132 134 -922 808 68 0 1 [134, 183, 131] 18446744073709551615 1 0x8400 0x4100
133 134 -920 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 134 -918 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
```
- edit channel: 0 linkage errors, 0 failed event dispatches (both must be 0)
- phase: 6 parity world boot (13:28:02)
- parity world: view-distance=8 (the voxy leg's server value)
- parity world: joined session 3 (13:28:41)
- parity world: scan populated (cells=65536; the march paints from these buffers)
- parity world tour: 13 vantages (production pack, vd 8, shared structures)
- coverage sweeps: 72 eye passes = 36 positions (12 az x r144/200 + 12 off-az x r172, y=160) x 2 looks (outward+inward), settle-by-laps >= 6 s (q stable 3x1s), deadline 12 s/pos, no shots — the v6.26.0 data-coverage lever + v6.26.6 lottery kill; probe rides parity/view_probe.md
- parity drain-settle TIMEOUT: queue 12765 -> 12765 > 11500 after 150s — continuing anyway; the queue did not drain without eye proximity (priority-lane evidence, the 01/05/10 dip attribution reads from the tour deltas)
- pre-tour pass done: 13 vantages x 10s (the tour now reads the second OR-merge lap; the rim/dip transport race reads from the tour deltas)
- parity world tour done: 13 shots, 0 night-guard renames in parity/ (the gate pairs voxy/<name>.png vs parity/<name>.png)
- phase: 7 parity tour done (13:50:39)
- sparse re-beat channel at tour end: streamed=78505 q=0 dropped_stale=71 dropped_undecodable=15085 (frac=8 boot; undecodable-dominant = the rim words die in their decodability window, stale-dominant = edit churn owns the cells)
### parity world stream counters (session 3, tail + maxima)
[c-LoD] stream: tick 13200 65536/65536 cells, lap 1205, cone 70614 zone 26951 dirty 7961 (q 12703) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 9023903c/1115e, edits +18041p/13669c/2763e drop 0, spawned 0 / failed 0, cam0 yaw -180 pitch 5, wish 0 forced 899, native 20935c/952ch fail 0 (3w q 0) helper on, trees 952ch/fail 0, wire bundle 1422546p/3100b fail 0 ~0 kB/s cache 21483/24576 h966848 m455698 e434215, srbeat 78505s/0q drop 71stale/15085undec, ereb 31992s/0q, sweep 0r/26d
[13:50:33 INFO]: [c-LoD] stream: tick 13200 65536/65536 cells, lap 1205, cone 70614 zone 26951 dirty 7961 (q 12703) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 9023903c/1115e, edits +18041p/13669c/2763e drop 0, spawned 0 / failed 0, cam0 yaw -180 pitch 5, wish 0 forced 899, native 20935c/952ch fail 0 (3w q 0) helper on, trees 952ch/fail 0, wire bundle 1422546p/3100b fail 0 ~0 kB/s cache 21483/24576 h966848 m455698 e434215, srbeat 78505s/0q drop 71stale/15085undec, ereb 31992s/0q, sweep 0r/26d
- parity stream max cone: 70614
- parity stream max zone: 26951
- parity stream max wire bundle: bundle
### parity world capture lens (debug chain on the §7b world)
lens_red=0 lens_black=0 lens_water=0 lens_dark=0 lens_zero=0 rot=none scan=0 carriers=0
- parity mesh probe @png_y=2: x306=(151,184,255) x307=(151,184,255) x308=(151,184,255) x309h=(151,184,255) x310c=(151,184,255)
- parity mesh probe @png_y=717: x306=(15,28,59) x307=(15,27,58) x308=(15,27,58) x309h=(15,28,59) x310c=(15,27,58)
- parity mesh engagement: magenta-ish=0 of 921600 sampled px
- mask8: the un-fog engagement masks shot on the PARITY WORLD (vd 8) — parity_dbg8/01m|07m|10m
- mask8 01m: green=1040 red=0 blue=97248 dim=21824 (px, sampled /16)
- mask8 07m: green=12032 red=0 blue=221360 dim=5792 (px, sampled /16)
- mask8 10m: green=7712 red=448 blue=231968 dim=136528 (px, sampled /16)
- mesh probe @png_y=2: x306=(255,255,255) x307=(13,254,0) x308=(0,254,0) x309h=(192,0,0) x310c=(0,0,0)
- mesh probe @png_y=717: x306=(255,255,255) x307=(13,254,0) x308=(0,254,0) x309h=(192,0,0) x310c=(0,0,0)
- mesh engagement: magenta-ish=16 of 921600 sampled px (the vanilla-path paint coverage)
- mask8 fog state @png_y=2: raw (300)=(151,184,255) (301)=(151,184,255)
- mask8 fog state @png_y=717: raw (300)=(0,128,8) (301)=(179,206,255)
- mask8 fog state decoded: envStart=0 envEnd=1024 rdEnd=128 fogColor=(179,206,255)
- buffer dump 10m: marked h=172/2048 c=35/2048; height windows r0<142:70 r1:19 r2>=202:83 maxByte=255
- buffer dump 10m: RANK-2-WINDOW HEIGHTS (the fake-wall class): (+26,-32)h=189, (+80,-32)h=159, (+86,-32)h=179, (+20,-30)h=183, (+76,-30)h=191, (+88,-30)h=157, (+90,-30)h=176, (+66,-28)h=191, (+68,-28)h=191, (+2,-26)h=144
- rim mask: scan populated (cells=65536; the state paint reads the same buffers the march reads)
- phase: 10 rim mask lane (13:54:58)
- rim mask: decode-state frames shot (rim_mask/00|01|12 — GREEN full decode / RED colorless / BLUE heightless; 00 = the leg-1 tower eye, 01 = the near-tower contrast, 12 = the tower re-read; the inherited 07/12 vantages sat 760 blocks outside the streamed disk and 01 sat INSIDE the marker column — their earlier verdicts were classifier noise, purged in v6.33.1)
- rim mask 00: green=52672 red=30928 blue=234752 yellow=54768 (px, sampled /16) -> DOMINANT BLUE: rim = HEIGHTS NEVER DECODED — a data-coverage class: the state paint cannot even see the cells
- rim mask 01: green=142400 red=27808 blue=232896 yellow=40752 (px, sampled /16) -> DOMINANT BLUE: rim = HEIGHTS NEVER DECODED — a data-coverage class: the state paint cannot even see the cells
- rim mask 12: green=91488 red=27040 blue=265968 yellow=50416 (px, sampled /16) -> DOMINANT BLUE: rim = HEIGHTS NEVER DECODED — a data-coverage class: the state paint cannot even see the cells

### boot / stream
```
[13:54:13 INFO]: [c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 149754 bytes, sha1 5dc283742995ead8546b0a2bbfadc892d3c58358 — zero textures, zero world data, one push per join
[c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 149754 bytes, sha1 5dc283742995ead8546b0a2bbfadc892d3c58358 — zero textures, zero world data, one push per join
[c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[13:54:13 INFO]: [c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[c-LoD] stream: tick 900 65536/65536 cells, lap 612, cone 11238 zone 892 dirty 19 (q 819) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 543805c/32e, edits +10p/10c/7e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 128 forced 789, native 6312c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 136937p/238b fail 0 ~39 kB/s cache 14042/24576 h122895 m14042 e0, srbeat 6799s/636q drop 0stale/1059undec, ereb 155s/23q, sweep 0r/43d
[13:55:53 INFO]: [c-LoD] stream: tick 900 65536/65536 cells, lap 612, cone 11238 zone 892 dirty 19 (q 819) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 543805c/32e, edits +10p/10c/7e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 128 forced 789, native 6312c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 136937p/238b fail 0 ~39 kB/s cache 14042/24576 h122895 m14042 e0, srbeat 6799s/636q drop 0stale/1059undec, ereb 155s/23q, sweep 0r/43d
```
### sessions
```
[13:54:34 INFO]: System chat: Stepan joined the game
[13:55:53 INFO]: System chat: Stepan left the game
```
### client log tail
```
[13:55:51] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[13:55:51] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[13:55:51] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:keep_alive] net.minecraft.network.protocol.common.ClientboundKeepAlivePacket -> 9 bytes
[13:55:51] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:keep_alive] net.minecraft.network.protocol.common.ServerboundKeepAlivePacket -> 9 bytes
[13:55:51] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[13:55:51] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[13:55:51] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[13:55:52] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:set_time] net.minecraft.network.protocol.game.ClientboundSetTimePacket -> 10 bytes
[13:55:52] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:move_player_pos] net.minecraft.network.protocol.game.ServerboundMovePlayerPacket$Pos -> 26 bytes
[13:55:52] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[13:55:52] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[13:55:52] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[13:55:52] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[13:55:52] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[13:55:52] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[13:55:52] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[13:55:52] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[13:55:52] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[13:55:52] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[13:55:52] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:bundle_delimiter] net.minecraft.network.protocol.game.ClientboundBundleDelimiterPacket -> 1 bytes
[13:55:52] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[13:55:52] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[13:55:52] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[13:55:52] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[13:55:52] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[13:55:52] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[13:55:52] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[13:55:52] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[13:55:52] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:bundle_delimiter] net.minecraft.network.protocol.game.ClientboundBundleDelimiterPacket -> 1 bytes
[13:55:52] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[13:55:52] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:keep_alive] net.minecraft.network.protocol.common.ClientboundKeepAlivePacket -> 9 bytes
[13:55:52] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:keep_alive] net.minecraft.network.protocol.common.ServerboundKeepAlivePacket -> 9 bytes
[13:55:52] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[13:55:52] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[13:55:52] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[13:55:52] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[13:55:52] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[13:55:52] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[13:55:52] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[13:55:52] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[13:55:52] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[13:55:52] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[13:55:52] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[13:55:52] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[13:55:52] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[13:55:52] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[13:55:52] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[13:55:52] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[13:55:52] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[13:55:52] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[13:55:52] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[13:55:52] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[13:55:52] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[13:55:52] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[13:55:53] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[13:55:53] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:set_time] net.minecraft.network.protocol.game.ClientboundSetTimePacket -> 10 bytes
[13:55:53] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[13:55:53] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:player_info_update] net.minecraft.network.protocol.game.ClientboundPlayerInfoUpdatePacket -> 20 bytes
[13:55:53] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[13:55:53] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:move_player_pos] net.minecraft.network.protocol.game.ServerboundMovePlayerPacket$Pos -> 26 bytes
[13:55:53] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[13:55:53] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[13:55:53] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[13:55:53] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[13:55:53] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[13:55:53] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[13:55:53] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[13:55:53] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[13:55:53] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[13:55:53] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[13:55:53] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:bundle_delimiter] net.minecraft.network.protocol.game.ClientboundBundleDelimiterPacket -> 1 bytes
[13:55:53] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[13:55:53] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[13:55:53] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[13:55:53] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[13:55:53] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[13:55:53] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[13:55:53] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[13:55:53] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[13:55:53] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:bundle_delimiter] net.minecraft.network.protocol.game.ClientboundBundleDelimiterPacket -> 1 bytes
--- exceptions (Realms/RTKit noise filtered) ---
25:[13:54:29] [Render thread/ERROR]: Error parsing option value 4 for option Simulation Distance: Value 4 outside of range [5:33]
48:[13:54:29] [Download-2/ERROR]: Got an error with a html body connecting to https://api.minecraftservices.com/player/attributes: <!DOCTYPE html PUBLIC '-//W3C//DTD XHTML 1.0 Transitional//EN' 'http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd'>
110:[13:54:29] [Download-2/ERROR]: Failed to fetch user properties
111:com.mojang.authlib.exceptions.AuthenticationException: Status: 429
112:	at com.mojang.authlib.exceptions.MinecraftClientHttpException.toAuthenticationException(MinecraftClientHttpException.java:77)
119:Caused by: MinecraftClientHttpException[type=HTTP_ERROR, status=429, response=null]
120:	at com.mojang.authlib.minecraft.client.MinecraftClient.buildHttpException(MinecraftClient.java:167)
157:[13:54:33] [Download-1/ERROR]: Got an error with a html body connecting to https://api.minecraftservices.com/privacy/blocklist: <!DOCTYPE html PUBLIC '-//W3C//DTD XHTML 1.0 Transitional//EN' 'http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd'>
219:[13:54:33] [Download-2/ERROR]: Got an error with a html body connecting to https://api.minecraftservices.com/player/certificates: <!DOCTYPE html PUBLIC '-//W3C//DTD XHTML 1.0 Transitional//EN' 'http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd'>
```
### server log tail
```
[c-LoD] EB: enter, native_gen=true
[13:55:54 INFO]: [ChunkHolderManager] Saved 998 block chunks, 974 entity chunks, 0 poi chunks in world 'minecraft:overworld' in 0.93s
[13:55:54 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:overworld'
[13:55:54 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:overworld'
[13:55:54 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[13:55:54 INFO]: [ChunkHolderManager] Waiting 60s for chunk system to halt for world 'minecraft:the_nether'
[13:55:54 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:the_nether'
[13:55:54 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_nether'
[13:55:54 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_nether' in 0.00s
[13:55:54 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_nether'
[13:55:54 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_nether'
[13:55:54 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[13:55:54 INFO]: [ChunkHolderManager] Waiting 60s for chunk system to halt for world 'minecraft:the_end'
[13:55:54 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:the_end'
[13:55:54 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_end'
[13:55:54 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_end' in 0.00s
[13:55:54 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_end'
[13:55:54 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_end'
[13:55:54 INFO]: Waiting for all RegionFile I/O tasks to complete...
[13:55:54 INFO]: All RegionFile I/O tasks to complete
[13:55:54 INFO]: [MoonriseCommon] Awaiting termination of worker pool for up to 60s...
[13:55:54 INFO]: [MoonriseCommon] Awaiting termination of I/O pool for up to 60s...
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[launcher] server exited with code 143
```
### shots
```
-rw-r--r-- 1 runner runner 219933 Nov 17 13:19 11_h176_spark.png
-rw-r--r-- 1 runner runner 219961 Nov 17 13:19 11_h180_spark.png
-rw-r--r-- 1 runner runner 219967 Nov 17 13:19 11_h185_spark.png
-rw-r--r-- 1 runner runner 219986 Nov 17 13:19 12_after_edit_probe0.png
-rw-r--r-- 1 runner runner  14637 Nov 17 13:27 13_after_relog_probe0.png
-rw-r--r-- 1 runner runner 159843 Nov 17 13:28 13_after_relog_probe1.png
-rw-r--r-- 1 runner runner  13773 Nov 17 13:27 14_leg2_baseline.png
-rw-r--r-- 1 runner runner  15124 Nov 17 13:27 14_leg2_burst_far.png
-rw-r--r-- 1 runner runner  16477 Nov 17 13:27 14_leg2_burst_greendust.png
-rw-r--r-- 1 runner runner  13773 Nov 17 13:27 14_leg2_burst_near.png
-rw-r--r-- 1 runner runner 189874 Nov 17 13:28 diff_after_relog.png
-rw-r--r-- 1 runner runner 227912 Nov 17 13:19 diff_live_edit.png
-rw-r--r-- 1 runner runner      0 Nov 17 13:17 ffmpeg.err
-rw-r--r-- 1 runner runner  14091 Nov 17 13:18 lens_09c_burst_close_retry.png
-rw-r--r-- 1 runner runner  13791 Nov 17 13:19 lens_12_after_edit_probe0.png
-rw-r--r-- 1 runner runner   2658 Nov 17 13:27 lens_13_after_relog_probe0.png
-rw-r--r-- 1 runner runner   8393 Nov 17 13:28 lens_13_after_relog_probe1.png
drwxr-xr-x 2 runner runner   4096 Nov 17 13:50 lens_parity
drwxr-xr-x 2 runner runner   4096 Nov 17 13:50 parity
drwxr-xr-x 2 runner runner   4096 Nov 17 13:22 parity_dbg
drwxr-xr-x 2 runner runner   4096 Nov 17 13:53 parity_dbg8
-rw-r--r-- 1 runner runner    821 Nov 17 13:28 probe_counts.txt
drwxr-xr-x 2 runner runner   4096 Nov 17 13:55 rim_mask
-rw-r--r-- 1 runner runner  59031 Nov 17 13:55 server_full.log
-rw-r--r-- 1 runner runner  57164 Nov 17 13:55 summary.md
```

**exit=0** failures=0 leg1=1 leg2=1
