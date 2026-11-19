- phase: 0 sanity (20:10:42)
### c-LoD visual run (-930,796, marker=red_concrete, vantage -1005,150 yaw 270)
- GL:     Vendor: Mesa (0xffffffff);    Device: llvmpipe (LLVM 21.1.8, 256 bits) (0xffffffff);OpenGL renderer string: llvmpipe (LLVM 21.1.8, 256 bits);OpenGL core profile version string: 4.5 (Core Profile) Mesa 26.0.8-1ubuntu0.3;
- boot + gates OK (20:11:00)
- phase: 1 server booted (20:11:00)
- sky-grid armed
- phase: 2 leg-1 client joined (20:11:12)
- joined session 1 (20:11:12)
- stream live: cells=65536
- phase: 3 scan populated (20:11:34)
- terrain range: 2 terraced mountains at (CX-4,CZ-20)/(CX-2,CZ+16), tops 185/190, grass terraces + stone caps, 66..82 m from the eye (inside the 83.5 m band, chunk edge 64 m)
- scan complete=1 (cells=cells=65536/65536)
- paint settled=1 (wire 19836p)
- baseline try 1: drift=373775 px
- baseline try 2: drift=274424 px
- baseline try 3: drift=344919 px
- baseline try 4: drift=249866 px
- baseline try 5: drift=362848 px
- baseline try 6: drift=335936 px
- **FAIL:** no stable baseline after 6 tries (drift=335936) — the A/B would lie
- lod_silhouette_cols=200 (LOD paint ON SCREEN at 75 m if > 20 — the march-drawn marker column; vanilla cannot render there, chunk edge 64 m; #255 re-measures 49, the ZONE_FAR=48 regression #256 and the control read 0)
- 0 terrain-green px in the sky region (LOD MOUNTAINS on screen if > 5000; the range paints ColLow-green on low terraces via the fallback palette; flat-world baselines and the pack-free control read ~0)
- burst_close white px: 3061 (renderer alive if > 50)
- burst_far white px: 3061 (renderer alive in the marker zone if > 20)
- cut walk 40 m: 2881 white px (> 20 = rendered)
- cut walk 60 m: 2881 white px (> 20 = rendered)
- cut walk 100 m: 2881 white px (> 20 = rendered)
- measured far-render cut: 100 m (G4 flap instrument; the decode proof needs >= 83 m from the y=151.6 eye)
- fill issued: 5x111x5 red_concrete at -930,796 (zone forceloaded ±48)
- phase: 4 marker filled (20:15:58)

### /clod-cells AFTER FILL (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 19 0 0x8400 0x2000
123 122 -940 784 71 0 1 [97, 119, 97] 19 0 0x8700 0x2000
124 122 -938 784 68 0 1 [97, 119, 97] 18 0 0x8400 0x2000
125 122 -936 784 62 1 1 [63, 118, 228] 18 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 17 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 17 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 16 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 22 0 0x8000 0x4100
130 122 -926 784 66 0 1 [134, 183, 131] 22 0 0x8200 0x4100
131 122 -924 784 75 0 1 [97, 119, 97] 22 0 0x8b00 0x2000
132 122 -922 784 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 122 -920 784 75 0 1 [97, 119, 97] 18446744073709551615 1 0x8b00 0x2000
134 122 -918 784 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x2000
122 123 -942 786 70 0 1 [97, 119, 97] 19 0 0x8600 0x2000
123 123 -940 786 65 0 1 [97, 119, 97] 19 0 0x8100 0x2000
124 123 -938 786 65 0 1 [97, 119, 97] 18 0 0x8100 0x2000
125 123 -936 786 62 1 1 [63, 118, 228] 18 0 0x7e00 0xd800
126 123 -934 786 62 1 1 [63, 118, 228] 17 0 0x7e00 0xd800
127 123 -932 786 62 1 1 [63, 118, 228] 17 0 0x7e00 0xd800
128 123 -930 786 62 1 1 [63, 118, 228] 16 0 0x7e00 0xd800
129 123 -928 786 73 0 1 [97, 119, 97] 22 0 0x8900 0x2000
130 123 -926 786 67 0 1 [134, 183, 131] 22 0 0x8300 0x4100
131 123 -924 786 69 0 1 [134, 183, 131] 22 0 0x8500 0x4100
132 123 -922 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 123 -920 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 123 -918 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
122 124 -942 788 72 0 1 [97, 119, 97] 19 0 0x8800 0x2000
123 124 -940 788 70 0 1 [97, 119, 97] 19 0 0x8600 0x2000
124 124 -938 788 68 0 1 [97, 119, 97] 18 0 0x8400 0x2000
125 124 -936 788 62 1 1 [63, 118, 228] 18 0 0x7e00 0xd800
126 124 -934 788 62 1 1 [63, 118, 228] 17 0 0x7e00 0xd800
127 124 -932 788 62 1 1 [63, 118, 228] 17 0 0x7e00 0xd800
128 124 -930 788 73 0 1 [97, 119, 97] 16 0 0x8900 0x2000
129 124 -928 788 76 0 1 [97, 119, 97] 22 0 0x8c00 0x2000
130 124 -926 788 73 0 1 [97, 119, 97] 22 0 0x8900 0x2000
131 124 -924 788 69 0 1 [134, 183, 131] 22 0 0x8500 0x4100
132 124 -922 788 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 124 -920 788 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 124 -918 788 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x2000
122 125 -942 790 74 0 1 [97, 119, 97] 19 0 0x8a00 0x2000
123 125 -940 790 73 0 1 [97, 119, 97] 19 0 0x8900 0x2000
124 125 -938 790 71 0 1 [97, 119, 97] 18 0 0x8700 0x2000
125 125 -936 790 62 1 1 [63, 118, 228] 18 0 0x7e00 0xd800
126 125 -934 790 62 1 1 [63, 118, 228] 17 0 0x7e00 0xd800
127 125 -932 790 62 1 1 [63, 118, 228] 17 0 0x7e00 0xd800
128 125 -930 790 62 1 1 [63, 118, 228] 16 0 0x7e00 0xd800
129 125 -928 790 73 0 1 [97, 119, 97] 16 0 0x8900 0x2000
130 125 -926 790 68 0 1 [134, 183, 131] 22 0 0x8400 0x4100
131 125 -924 790 69 0 1 [134, 183, 131] 22 0 0x8500 0x4100
132 125 -922 790 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
133 125 -920 790 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 125 -918 790 70 0 1 [134, 183, 131] 18446744073709551615 1 0x8600 0x4100
122 126 -942 792 74 0 1 [97, 119, 97] 19 0 0x8a00 0x2000
123 126 -940 792 73 0 1 [97, 119, 97] 19 0 0x8900 0x2000
124 126 -938 792 68 0 1 [97, 119, 97] 18 0 0x8400 0x2000
125 126 -936 792 62 1 1 [63, 118, 228] 18 0 0x7e00 0xd800
126 126 -934 792 62 1 1 [63, 118, 228] 17 0 0x7e00 0xd800
127 126 -932 792 62 1 1 [63, 118, 228] 17 0 0x7e00 0xd800
128 126 -930 792 62 1 1 [63, 118, 228] 16 0 0x7e00 0xd800
129 126 -928 792 67 0 1 [134, 183, 131] 16 0 0x8300 0x4100
130 126 -926 792 74 0 1 [97, 119, 97] 22 0 0x8a00 0x2000
131 126 -924 792 77 0 1 [97, 119, 97] 22 0 0x8d00 0x2000
132 126 -922 792 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 126 -920 792 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3200
134 126 -918 792 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x5000
122 127 -942 794 67 0 1 [134, 183, 131] 19 0 0x8300 0x4100
123 127 -940 794 68 0 1 [134, 183, 131] 19 0 0x8400 0x4100
124 127 -938 794 65 0 1 [134, 183, 131] 18 0 0x8100 0x4100
125 127 -936 794 62 1 1 [63, 118, 228] 18 0 0x7e00 0xd800
126 127 -934 794 62 1 1 [63, 118, 228] 17 0 0x7e00 0xd800
127 127 -932 794 62 1 1 [63, 118, 228] 17 0 0x7e00 0xd800
128 127 -930 794 62 1 1 [63, 118, 228] 16 0 0x7e00 0xd800
129 127 -928 794 66 0 1 [134, 183, 131] 16 0 0x8200 0x4100
130 127 -926 794 74 0 1 [97, 119, 97] 22 0 0x8a00 0x2000
131 127 -924 794 77 0 1 [97, 119, 97] 22 0 0x8d00 0x2000
132 127 -922 794 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 127 -920 794 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3200
134 127 -918 794 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x5000
122 128 -942 796 68 0 1 [134, 183, 131] 19 0 0x8400 0x4100
123 128 -940 796 72 0 1 [97, 119, 97] 19 0 0x8800 0x2000
124 128 -938 796 75 0 1 [97, 119, 97] 18 0 0x8b00 0x2000
125 128 -936 796 72 0 1 [97, 119, 97] 18 0 0x8800 0x2000
126 128 -934 796 62 1 1 [63, 118, 228] 17 0 0x7e00 0xd800
127 128 -932 796 62 1 1 [63, 118, 228] 17 0 0x7e00 0xd800
128 128 -930 796 62 1 1 [63, 118, 228] 16 0 0x7e00 0xd800
129 128 -928 796 66 0 1 [134, 183, 131] 16 0 0x8200 0x4100
130 128 -926 796 68 0 1 [134, 183, 131] 22 0 0x8400 0x4100
131 128 -924 796 74 0 1 [97, 119, 97] 22 0 0x8a00 0x2000
132 128 -922 796 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 128 -920 796 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
134 128 -918 796 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 129 -942 798 69 0 1 [134, 183, 131] 19 0 0x8500 0x4100
123 129 -940 798 75 0 1 [97, 119, 97] 19 0 0x8b00 0x2000
124 129 -938 798 78 0 1 [97, 119, 97] 20 0 0x8e00 0x2000
125 129 -936 798 75 0 1 [97, 119, 97] 18 0 0x8b00 0x2000
126 129 -934 798 62 1 1 [63, 118, 228] 17 0 0x7e00 0xd800
127 129 -932 798 62 1 1 [63, 118, 228] 17 0 0x7e00 0xd800
128 129 -930 798 62 1 1 [63, 118, 228] 16 0 0x7e00 0xd800
129 129 -928 798 71 0 1 [97, 119, 97] 16 0 0x8700 0x2000
130 129 -926 798 71 0 1 [97, 119, 97] 22 0 0x8700 0x2000
131 129 -924 798 69 0 1 [134, 183, 131] 22 0 0x8500 0x4100
132 129 -922 798 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 129 -920 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 129 -918 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 130 -942 800 70 0 1 [134, 183, 131] 19 0 0x8600 0x4100
123 130 -940 800 76 0 1 [97, 119, 97] 19 0 0x8c00 0x2000
124 130 -938 800 76 0 1 [97, 119, 97] 18 0 0x8c00 0x2000
125 130 -936 800 72 0 1 [97, 119, 97] 18 0 0x8800 0x2000
126 130 -934 800 62 1 1 [63, 118, 228] 17 0 0x7e00 0xd800
127 130 -932 800 62 1 1 [63, 118, 228] 17 0 0x7e00 0xd800
128 130 -930 800 62 1 1 [63, 118, 228] 16 0 0x7e00 0xd800
129 130 -928 800 71 0 1 [97, 119, 97] 16 0 0x8700 0x2000
130 130 -926 800 71 0 1 [97, 119, 97] 22 0 0x8700 0x2000
131 130 -924 800 69 0 1 [134, 183, 131] 22 0 0x8500 0x4100
132 130 -922 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
133 130 -920 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 130 -918 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 131 -942 802 78 0 1 [97, 119, 97] 21 0 0x8e00 0x2000
123 131 -940 802 77 0 1 [97, 119, 97] 19 0 0x8d00 0x2000
124 131 -938 802 77 0 1 [97, 119, 97] 18 0 0x8d00 0x2000
125 131 -936 802 66 0 1 [134, 183, 131] 18 0 0x8200 0x4100
126 131 -934 802 62 1 1 [63, 118, 228] 17 0 0x7e00 0xd800
127 131 -932 802 62 1 1 [63, 118, 228] 17 0 0x7e00 0xd800
128 131 -930 802 62 1 1 [63, 118, 228] 16 0 0x7e00 0xd800
129 131 -928 802 62 1 1 [63, 118, 228] 16 0 0x7e00 0xd800
130 131 -926 802 66 0 1 [134, 183, 131] 22 0 0x8200 0x4100
131 131 -924 802 68 0 1 [134, 183, 131] 22 0 0x8400 0x4100
132 131 -922 802 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 131 -920 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 131 -918 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 132 -942 804 79 0 1 [97, 119, 97] 21 0 0x8f00 0x2000
123 132 -940 804 76 0 1 [97, 119, 97] 19 0 0x8c00 0x2000
124 132 -938 804 76 0 1 [97, 119, 97] 18 0 0x8c00 0x2000
125 132 -936 804 76 0 1 [97, 119, 97] 18 0 0x8c00 0x2000
126 132 -934 804 62 1 1 [63, 118, 228] 17 0 0x7e00 0xd800
127 132 -932 804 62 1 1 [63, 118, 228] 17 0 0x7e00 0xd800
128 132 -930 804 62 1 1 [63, 118, 228] 16 0 0x7e00 0xd800
129 132 -928 804 62 1 1 [63, 118, 228] 22 0 0x7e00 0xd800
130 132 -926 804 65 0 1 [134, 183, 131] 22 0 0x8100 0x4100
131 132 -924 804 77 0 1 [97, 119, 97] 22 0 0x8d00 0x2000
132 132 -922 804 79 0 1 [97, 119, 97] 22 0 0x8f00 0x2000
133 132 -920 804 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 132 -918 804 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x2000
122 133 -942 806 78 0 1 [97, 119, 97] 21 0 0x8e00 0x2000
123 133 -940 806 69 0 1 [134, 183, 131] 19 0 0x8500 0x4100
124 133 -938 806 155 0 1 [134, 183, 131] 18 0 0xdb00 0x4100
125 133 -936 806 155 0 1 [134, 183, 131] 18 0 0xdb00 0x4100
126 133 -934 806 155 0 1 [134, 183, 131] 17 0 0xdb00 0x4100
127 133 -932 806 155 0 1 [134, 183, 131] 17 0 0xdb00 0x4100
128 133 -930 806 155 0 1 [134, 183, 131] 16 0 0xdb00 0x4100
129 133 -928 806 155 0 1 [134, 183, 131] 22 0 0xdb00 0x4100
130 133 -926 806 155 0 1 [134, 183, 131] 22 0 0xdb00 0x4100
131 133 -924 806 67 0 1 [134, 183, 131] 22 0 0x8300 0x4100
132 133 -922 806 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 133 -920 806 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 133 -918 806 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x2000
122 134 -942 808 79 0 1 [97, 119, 97] 21 0 0x8f00 0x2000
123 134 -940 808 68 0 1 [134, 183, 131] 19 0 0x8400 0x4100
124 134 -938 808 155 0 1 [134, 183, 131] 18 0 0xdb00 0x4100
125 134 -936 808 180 0 1 [134, 183, 131] 18 0 0xf400 0x4100
126 134 -934 808 180 0 1 [134, 183, 131] 17 0 0xf400 0x4100
127 134 -932 808 180 0 1 [134, 183, 131] 16 0 0xf400 0x4100
128 134 -930 808 180 0 1 [134, 183, 131] 16 0 0xf400 0x4100
129 134 -928 808 180 0 1 [134, 183, 131] 22 0 0xf400 0x4100
130 134 -926 808 155 0 1 [134, 183, 131] 22 0 0xdb00 0x4100
131 134 -924 808 67 0 1 [134, 183, 131] 22 0 0x8300 0x4100
132 134 -922 808 68 0 1 [134, 183, 131] 18446744073709551615 1 0x8400 0x4100
133 134 -920 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 134 -918 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
```
### stream diag after fill:
[c-LoD] stream: tick 3000 65536/65536 cells, lap 790, cone 24096 zone 1131 dirty 844 (q 8906) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 379928c/2175e, edits +384p/286c/25e drop 0, spawned 0 / failed 0, cas 0d, cam0 yaw -90 pitch 5, wish 359 forced 153, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 661902p/1774b fail 0 ~82 kB/s cache 14118/24576 h647784 m14118 e0, srbeat 57104s/573q drop 18stale/26undec, ereb 48600s/1q, sweep 0r/94d, cdiag off
[20:16:02 INFO]: [c-LoD] stream: tick 3000 65536/65536 cells, lap 790, cone 24096 zone 1131 dirty 844 (q 8906) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 379928c/2175e, edits +384p/286c/25e drop 0, spawned 0 / failed 0, cas 0d, cam0 yaw -90 pitch 5, wish 359 forced 153, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 661902p/1774b fail 0 ~82 kB/s cache 14118/24576 h647784 m14118 e0, srbeat 57104s/573q drop 18stale/26undec, ereb 48600s/1q, sweep 0r/94d, cdiag off
- carrier ladder y=155: 2332 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=165: 2156 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=168: 2000 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=172: 1636 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=176: 1128 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=180: 948 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=185: 1104 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- probe 0: red=220877 paint=576619 lod_silhouette_cols=200 lens_red=0 lens_black=0 lens_water=0 lens_dark=0 lens_zero=0 rot=none scan=0 carriers=0 (need red>=60 paint>=300 lod>20)
- **LEG 1 PASS** (zero relogs)
- phase: 5 leg-1 probes done (20:16:30)
### stream diag after probes:
stream: tick 3100 65536/65536 cells, lap 797, cone 24992 zone 1131 dirty 844 (q 8906) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 394438c/2175e, edits +384p/286c/25e drop 0, spawned 0 / failed 0, cas 0d, cam0 yaw -90 pitch 5, wish 359 forced 153, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 672531p/1789b fail 0 ~56 kB/s cache 14118/24576 h658413 m14118 e0, srbeat 57676s/461q drop 18stale/26undec, ereb 48600s/1q, sweep 3r/0d, cdiag off
[c-LoD] stream: tick 3200 65536/65536 cells, lap 799, cone 25248 zone 1131 dirty 844 (q 8906) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 408088c/2175e, edits +384p/286c/25e drop 0, spawned 0 / failed 0, cas 0d, cam0 yaw -90 pitch 5, wish 359 forced 153, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 680132p/1801b fail 0 ~40 kB/s cache 14118/24576 h666014 m14118 e0, srbeat 58248s/1q drop 18stale/27undec, ereb 48600s/1q, sweep 0r/3d, cdiag off
[20:16:22 INFO]: [c-LoD] stream: tick 3200 65536/65536 cells, lap 799, cone 25248 zone 1131 dirty 844 (q 8906) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 408088c/2175e, edits +384p/286c/25e drop 0, spawned 0 / failed 0, cas 0d, cam0 yaw -90 pitch 5, wish 359 forced 153, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 680132p/1801b fail 0 ~40 kB/s cache 14118/24576 h666014 m14118 e0, srbeat 58248s/1q drop 18stale/27undec, ereb 48600s/1q, sweep 0r/3d, cdiag off
- parity debug tour: 13 vantages (session-1 client, DEBUG pack — evidence only, the gate pairs the §7b world session)
- parity debug tour done: 13 shots in parity_dbg/ (the §7b world session feeds the gate)

### boot / stream
```
[c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 153603 bytes, sha1 2867a7a3d6fcecee18045acb46d0b51c4e117fa4 — zero textures, zero world data, one push per join
[20:10:57 INFO]: [c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 153603 bytes, sha1 2867a7a3d6fcecee18045acb46d0b51c4e117fa4 — zero textures, zero world data, one push per join
[c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[20:10:57 INFO]: [c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[c-LoD] stream: tick 4300 65536/65536 cells, lap 894, cone 37792 zone 1857 dirty 844 (q 8906) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 563398c/2175e, edits +384p/286c/25e drop 0, spawned 0 / failed 0, cas 0d, cam0 yaw -90 pitch 5, wish 359 forced 153, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 833706p/1994b fail 0 ~71 kB/s cache 14118/24576 h819588 m14118 e0, srbeat 64440s/512q drop 18stale/37undec, ereb 48600s/1q, sweep 2r/0d, cdiag off
[20:18:13 INFO]: [c-LoD] stream: tick 4300 65536/65536 cells, lap 894, cone 37792 zone 1857 dirty 844 (q 8906) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 563398c/2175e, edits +384p/286c/25e drop 0, spawned 0 / failed 0, cas 0d, cam0 yaw -90 pitch 5, wish 359 forced 153, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 833706p/1994b fail 0 ~71 kB/s cache 14118/24576 h819588 m14118 e0, srbeat 64440s/512q drop 18stale/37undec, ereb 48600s/1q, sweep 2r/0d, cdiag off
```
### sessions
```
[20:11:10 INFO]: System chat: Stepan joined the game
```
### client log tail
```
Horizon 0.1.0
Horizon: ignoring minecraft:ambient_entity_effect: no such vanilla particle in this version
Horizon renderer: Vulkan has no conformant adapter (Vulkan: only a software adapter (llvmpipe (LLVM 21.1.8, 256 bits))); asking again for the ones wgpu hides as non-conformant
Horizon renderer: trying OpenGL (Vulkan: only a software adapter (llvmpipe (LLVM 21.1.8, 256 bits)))
libEGL warning: DRI3 error: Could not get DRI3 device
libEGL warning: Ensure your X server supports DRI3 to get accelerated rendering
libEGL warning: DRI3 error: Could not get DRI3 device
libEGL warning: Ensure your X server supports DRI3 to get accelerated rendering
Horizon renderer: OpenGL has no conformant adapter (OpenGL: only a software adapter (llvmpipe (LLVM 21.1.8, 256 bits))); asking again for the ones wgpu hides as non-conformant
Horizon renderer: llvmpipe (LLVM 21.1.8, 256 bits) (Vulkan, Cpu)
Horizon renderer: requesting the device (driver llvmpipe Mesa 26.0.8-1ubuntu0.3 (LLVM 21.1.8)) with features [TEXTURE_FORMAT_16BIT_NORM | TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES | PIPELINE_STATISTICS_QUERY | TIMESTAMP_QUERY_INSIDE_ENCODERS | TIMESTAMP_QUERY_INSIDE_PASSES | MAPPABLE_PRIMARY_BUFFERS | MULTI_DRAW_INDIRECT_COUNT | VERTEX_WRITABLE_STORAGE | CLEAR_TEXTURE | TEXTURE_ATOMIC | SHADER_EARLY_DEPTH_TEST | SUBGROUP | TIMESTAMP_QUERY | INDIRECT_FIRST_INSTANCE | RG11B10UFLOAT_RENDERABLE | FLOAT32_FILTERABLE | FLOAT32_BLENDABLE] and the adapter's limits (buffers up to 2147483647 bytes, storage bindings up to 134217728 bytes, 2048 texture layers)
Horizon renderer: device ready; configuring the surface (1280x720 Bgra8Unorm, Mailbox)
Horizon renderer: the chosen graphics API gave no GPU (No Vulkan or OpenGL graphics driver found: Horizon needs the Vulkan loader (libvulkan.so.1) and a Vulkan driver for your GPU (Mesa's, or NVIDIA's), or OpenGL through EGL (Vulkan: only a software adapter (llvmpipe (LLVM 21.1.8, 256 bits)); OpenGL: only a software adapter (llvmpipe (LLVM 21.1.8, 256 bits)))); rendering on the CPU, which is slow
Horizon renderer: HDR output not offered by this display
Horizon renderer: creating the pipelines
Horizon: UI bundle ready after 634 ms
Horizon: block sprites: 1301 layers at 64 px (largest 64 px, budget 256 MiB)
Horizon renderer: pipelines ready (multi-draw on, occlusion culling off, GPU culling unavailable, GPU timer on)
Horizon renderer: acquiring the first frame
Horizon renderer: submitting the first frame
Horizon renderer: presenting the first frame
load core tables: 1.12s
Horizon: loading screen shown after 1162 ms
Horizon: loading the title artwork after 1162 ms
Horizon renderer: llvmpipe (LLVM 21.1.8, 256 bits) (Vulkan, Cpu)
load pack tables: 1.28s
Horizon: core-ready after 1293 ms
Horizon: handoff CPU resources prepared on worker in 144 ms
load items: 1.58s
Horizon: resources loaded in 1.6s; 35723 block states, 1658 items, 67 biomes; 3 worker threads
Horizon: rest-ready after 1633 ms
Horizon renderer: rebuilt on the same device (51 shader modules, layouts and pipelines kept)
Horizon renderer: llvmpipe (LLVM 21.1.8, 256 bits) (Vulkan, Cpu)
Horizon renderer: rebuilt on the same device (51 shader modules, layouts and pipelines kept)
Horizon: world renderer ready after 2060 ms
Horizon: no sounds in the active resources; prepare the game again in the launcher (or run horizon-prepare sounds)
Horizon: no sound: audio off: HORIZON_NO_AUDIO is set
Horizon: waiting for world resources
Horizon: world resources joined after 0 ms
Horizon: startup handoff 410 ms (renderer 378 ms, start_game 20 ms)
Horizon: disconnected: Connection refused: Connection refused (os error 111)
Horizon: first frame after 585 ms
Horizon: worldgen data loaded in 945.506114ms
XIO:  fatal IO error 2 (No such file or directory) on X server ":99"
      after 24055 requests (321 known processed) with 0 events remaining.
--- exceptions (Realms/RTKit noise filtered) ---
41:Horizon: disconnected: Connection refused: Connection refused (os error 111)
```
### server log tail
```
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true

Exception: java.lang.OutOfMemoryError thrown from the UncaughtExceptionHandler in thread "JNA Cleaner"

Exception: java.lang.OutOfMemoryError thrown from the UncaughtExceptionHandler in thread "spark-monitoring-1-thread-4"
[cplug-sdk] main-thread flush timed out (1198 queued)
[cplug-sdk] main-thread flush timed out (2406 queued)
[cplug-sdk] main-thread flush timed out (3604 queued)
[cplug-sdk] main-thread flush timed out (4813 queued)
OpenJDK 64-Bit Server VM warning: Exception java.lang.OutOfMemoryError occurred dispatching signal SIGTERM to handler- the VM may need to be forcibly terminated
[cplug-sdk] main-thread flush timed out (6011 queued)
```
### shots
```
total 7504
drwxr-xr-x 3 runner runner   4096 Nov 19 20:28 .
drwxr-xr-x 3 runner runner   4096 Nov 19 19:54 ..
-rw-r--r-- 1 runner runner 450334 Nov 19 20:15 09_burst_close.png
-rw-r--r-- 1 runner runner 472567 Nov 19 20:15 09b_burst_far.png
-rw-r--r-- 1 runner runner 420591 Nov 19 20:15 09w100.png
-rw-r--r-- 1 runner runner 503877 Nov 19 20:15 09w40.png
-rw-r--r-- 1 runner runner 464076 Nov 19 20:15 09w60.png
-rw-r--r-- 1 runner runner 511872 Nov 19 20:14 10_before_edit.png
-rw-r--r-- 1 runner runner 491397 Nov 19 20:14 11_baseline_settle.png
-rw-r--r-- 1 runner runner 461887 Nov 19 20:16 11_h155_spark.png
-rw-r--r-- 1 runner runner 458133 Nov 19 20:16 11_h165_spark.png
-rw-r--r-- 1 runner runner 459025 Nov 19 20:16 11_h168_spark.png
-rw-r--r-- 1 runner runner 468364 Nov 19 20:16 11_h172_spark.png
-rw-r--r-- 1 runner runner 472533 Nov 19 20:16 11_h176_spark.png
-rw-r--r-- 1 runner runner 462486 Nov 19 20:16 11_h180_spark.png
-rw-r--r-- 1 runner runner 450473 Nov 19 20:16 11_h185_spark.png
-rw-r--r-- 1 runner runner 446064 Nov 19 20:16 12_after_edit_probe0.png
-rw-r--r-- 1 runner runner 212030 Nov 19 20:16 diff_live_edit.png
-rw-r--r-- 1 runner runner      0 Nov 19 20:12 ffmpeg.err
-rw-r--r-- 1 runner runner 107472 Nov 19 20:16 lens_12_after_edit_probe0.png
drwxr-xr-x 2 runner runner   4096 Nov 19 20:19 parity_dbg
-rw-r--r-- 1 runner runner    143 Nov 19 20:16 probe_counts.txt
-rw-r--r-- 1 runner runner 303044 Nov 19 20:28 server_full.log
-rw-r--r-- 1 runner runner  23520 Nov 19 20:28 summary.md
```

**exit=1** failures=1 leg1=1 leg2=0
