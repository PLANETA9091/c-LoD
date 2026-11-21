- phase: 0 sanity (09:52:44)
### c-LoD visual run (-930,796, marker=red_concrete, vantage -1005,150 yaw 270)
- GL:     Vendor: Mesa (0xffffffff);    Device: llvmpipe (LLVM 21.1.8, 256 bits) (0xffffffff);OpenGL renderer string: llvmpipe (LLVM 21.1.8, 256 bits);OpenGL core profile version string: 4.5 (Core Profile) Mesa 26.0.8-1ubuntu0.3;
- boot + gates OK (09:53:12)
- phase: 1 server booted (09:53:12)
- sky-grid armed
- phase: 2 leg-1 client joined (09:53:27)
- joined session 1 (09:53:27)
- stream live: cells=65536
- phase: 3 scan populated (09:53:50)
- terrain range: 2 terraced mountains at (CX-4,CZ-20)/(CX-2,CZ+16), tops 185/190, grass terraces + stone caps, 66..82 m from the eye (inside the 83.5 m band, chunk edge 64 m)
- scan complete=1 (cells=cells=65536/65536)
- paint settled=0 (wire 826450p)
- baseline try 1: drift=15832 px
- baseline try 2: drift=12982 px
- baseline try 3: drift=17066 px
- baseline try 4: drift=20560 px
- baseline try 5: drift=10114 px
- baseline try 6: drift=12214 px
- baseline try 7: drift=18261 px
- baseline try 8: drift=9679 px
- baseline try 9: drift=19558 px
- baseline try 10: drift=36585 px
- baseline try 11: drift=31024 px
- baseline try 12: drift=22408 px
- **FAIL:** no stable baseline after 12 tries (drift=22408) — the A/B would lie
- lod_silhouette_cols=61 (LOD paint ON SCREEN at 75 m if > 20 — the march-drawn marker column; vanilla cannot render there, chunk edge 64 m; #255 re-measures 49, the ZONE_FAR=48 regression #256 and the control read 0)
- 1498 terrain-green px in the sky region (LOD MOUNTAINS on screen if > 5000; the range paints ColLow-green on low terraces via the fallback palette; flat-world baselines and the pack-free control read ~0)
- burst_close white px: 1762 (renderer alive if > 50)
- burst_far white px: 1260 (renderer alive in the marker zone if > 20)
- cut walk 40 m: 64 white px (> 20 = rendered)
- cut walk 60 m: 32 white px (> 20 = rendered)
- cut walk 100 m: 0 white px (> 20 = rendered)
- measured far-render cut: 60 m (G4 flap instrument; the decode proof needs >= 83 m from the y=151.6 eye)
- fill issued: 5x111x5 red_concrete at -930,796 (zone forceloaded ±48)
- phase: 4 marker filled (10:09:07)

### /clod-cells AFTER FILL (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 12 0 0x8400 0x2000
123 122 -940 784 71 0 1 [97, 119, 97] 12 0 0x8700 0x2000
124 122 -938 784 68 0 1 [97, 119, 97] 11 0 0x8400 0x2000
125 122 -936 784 62 1 1 [63, 118, 228] 51 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 50 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 50 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 49 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 17 0 0x8000 0x4100
130 122 -926 784 66 0 1 [134, 183, 131] 17 0 0x8200 0x4100
131 122 -924 784 75 0 1 [97, 119, 97] 17 0 0x8b00 0x2000
132 122 -922 784 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 122 -920 784 75 0 1 [97, 119, 97] 18446744073709551615 1 0x8b00 0x2000
134 122 -918 784 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x2000
122 123 -942 786 70 0 1 [97, 119, 97] 12 0 0x8600 0x2000
123 123 -940 786 65 0 1 [97, 119, 97] 12 0 0x8100 0x2000
124 123 -938 786 65 0 1 [97, 119, 97] 11 0 0x8100 0x2000
125 123 -936 786 62 1 1 [63, 118, 228] 51 0 0x7e00 0xd800
126 123 -934 786 62 1 1 [63, 118, 228] 50 0 0x7e00 0xd800
127 123 -932 786 62 1 1 [63, 118, 228] 50 0 0x7e00 0xd800
128 123 -930 786 62 1 1 [63, 118, 228] 49 0 0x7e00 0xd800
129 123 -928 786 73 0 1 [97, 119, 97] 17 0 0x8900 0x2000
130 123 -926 786 67 0 1 [134, 183, 131] 17 0 0x8300 0x4100
131 123 -924 786 69 0 1 [134, 183, 131] 562 0 0x8500 0x4100
132 123 -922 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 123 -920 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 123 -918 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
122 124 -942 788 72 0 1 [97, 119, 97] 12 0 0x8800 0x2000
123 124 -940 788 70 0 1 [97, 119, 97] 12 0 0x8600 0x2000
124 124 -938 788 68 0 1 [97, 119, 97] 11 0 0x8400 0x2000
125 124 -936 788 62 1 1 [63, 118, 228] 51 0 0x7e00 0xd800
126 124 -934 788 62 1 1 [63, 118, 228] 50 0 0x7e00 0xd800
127 124 -932 788 62 1 1 [63, 118, 228] 50 0 0x7e00 0xd800
128 124 -930 788 73 0 1 [97, 119, 97] 9 0 0x8900 0x2000
129 124 -928 788 76 0 1 [97, 119, 97] 17 0 0x8c00 0x2000
130 124 -926 788 73 0 1 [97, 119, 97] 17 0 0x8900 0x2000
131 124 -924 788 69 0 1 [134, 183, 131] 562 0 0x8500 0x4100
132 124 -922 788 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 124 -920 788 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 124 -918 788 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x2000
122 125 -942 790 74 0 1 [97, 119, 97] 12 0 0x8a00 0x2000
123 125 -940 790 73 0 1 [97, 119, 97] 12 0 0x8900 0x2000
124 125 -938 790 71 0 1 [97, 119, 97] 11 0 0x8700 0x2000
125 125 -936 790 62 1 1 [63, 118, 228] 51 0 0x7e00 0xd800
126 125 -934 790 62 1 1 [63, 118, 228] 50 0 0x7e00 0xd800
127 125 -932 790 62 1 1 [63, 118, 228] 50 0 0x7e00 0xd800
128 125 -930 790 62 1 1 [63, 118, 228] 49 0 0x7e00 0xd800
129 125 -928 790 73 0 1 [97, 119, 97] 9 0 0x8900 0x2000
130 125 -926 790 68 0 1 [134, 183, 131] 17 0 0x8400 0x4100
131 125 -924 790 69 0 1 [134, 183, 131] 562 0 0x8500 0x4100
132 125 -922 790 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
133 125 -920 790 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 125 -918 790 70 0 1 [134, 183, 131] 18446744073709551615 1 0x8600 0x4100
122 126 -942 792 74 0 1 [97, 119, 97] 12 0 0x8a00 0x2000
123 126 -940 792 73 0 1 [97, 119, 97] 12 0 0x8900 0x2000
124 126 -938 792 68 0 1 [97, 119, 97] 11 0 0x8400 0x2000
125 126 -936 792 62 1 1 [63, 118, 228] 51 0 0x7e00 0xd800
126 126 -934 792 62 1 1 [63, 118, 228] 50 0 0x7e00 0xd800
127 126 -932 792 62 1 1 [63, 118, 228] 50 0 0x7e00 0xd800
128 126 -930 792 62 1 1 [63, 118, 228] 49 0 0x7e00 0xd800
129 126 -928 792 67 0 1 [134, 183, 131] 9 0 0x8300 0x4100
130 126 -926 792 74 0 1 [97, 119, 97] 17 0 0x8a00 0x2000
131 126 -924 792 77 0 1 [97, 119, 97] 562 0 0x8d00 0x2000
132 126 -922 792 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 126 -920 792 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3200
134 126 -918 792 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x5000
122 127 -942 794 67 0 1 [134, 183, 131] 12 0 0x8300 0x4100
123 127 -940 794 68 0 1 [134, 183, 131] 12 0 0x8400 0x4100
124 127 -938 794 65 0 1 [134, 183, 131] 11 0 0x8100 0x4100
125 127 -936 794 62 1 1 [63, 118, 228] 51 0 0x7e00 0xd800
126 127 -934 794 62 1 1 [63, 118, 228] 50 0 0x7e00 0xd800
127 127 -932 794 62 1 1 [63, 118, 228] 50 0 0x7e00 0xd800
128 127 -930 794 62 1 1 [63, 118, 228] 49 0 0x7e00 0xd800
129 127 -928 794 66 0 1 [134, 183, 131] 9 0 0x8200 0x4100
130 127 -926 794 74 0 1 [97, 119, 97] 17 0 0x8a00 0x2000
131 127 -924 794 77 0 1 [97, 119, 97] 17 0 0x8d00 0x2000
132 127 -922 794 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 127 -920 794 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3200
134 127 -918 794 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x5000
122 128 -942 796 68 0 1 [134, 183, 131] 12 0 0x8400 0x4100
123 128 -940 796 72 0 1 [97, 119, 97] 12 0 0x8800 0x2000
124 128 -938 796 75 0 1 [97, 119, 97] 11 0 0x8b00 0x2000
125 128 -936 796 72 0 1 [97, 119, 97] 11 0 0x8800 0x2000
126 128 -934 796 62 1 1 [63, 118, 228] 50 0 0x7e00 0xd800
127 128 -932 796 62 1 1 [63, 118, 228] 50 0 0x7e00 0xd800
128 128 -930 796 62 1 1 [63, 118, 228] 49 0 0x7e00 0xd800
129 128 -928 796 66 0 1 [134, 183, 131] 9 0 0x8200 0x4100
130 128 -926 796 68 0 1 [134, 183, 131] 17 0 0x8400 0x4100
131 128 -924 796 74 0 1 [97, 119, 97] 17 0 0x8a00 0x2000
132 128 -922 796 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 128 -920 796 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
134 128 -918 796 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 129 -942 798 69 0 1 [134, 183, 131] 12 0 0x8500 0x4100
123 129 -940 798 75 0 1 [97, 119, 97] 12 0 0x8b00 0x2000
124 129 -938 798 78 0 1 [97, 119, 97] 13 0 0x8e00 0x2000
125 129 -936 798 75 0 1 [97, 119, 97] 11 0 0x8b00 0x2000
126 129 -934 798 62 1 1 [63, 118, 228] 50 0 0x7e00 0xd800
127 129 -932 798 62 1 1 [63, 118, 228] 50 0 0x7e00 0xd800
128 129 -930 798 62 1 1 [63, 118, 228] 49 0 0x7e00 0xd800
129 129 -928 798 71 0 1 [97, 119, 97] 9 0 0x8700 0x2000
130 129 -926 798 71 0 1 [97, 119, 97] 17 0 0x8700 0x2000
131 129 -924 798 69 0 1 [134, 183, 131] 17 0 0x8500 0x4100
132 129 -922 798 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 129 -920 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 129 -918 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 130 -942 800 70 0 1 [134, 183, 131] 12 0 0x8600 0x4100
123 130 -940 800 76 0 1 [97, 119, 97] 12 0 0x8c00 0x2000
124 130 -938 800 76 0 1 [97, 119, 97] 11 0 0x8c00 0x2000
125 130 -936 800 72 0 1 [97, 119, 97] 11 0 0x8800 0x2000
126 130 -934 800 62 1 1 [63, 118, 228] 50 0 0x7e00 0xd800
127 130 -932 800 62 1 1 [63, 118, 228] 50 0 0x7e00 0xd800
128 130 -930 800 62 1 1 [63, 118, 228] 49 0 0x7e00 0xd800
129 130 -928 800 71 0 1 [97, 119, 97] 9 0 0x8700 0x2000
130 130 -926 800 71 0 1 [97, 119, 97] 562 0 0x8700 0x2000
131 130 -924 800 69 0 1 [134, 183, 131] 562 0 0x8500 0x4100
132 130 -922 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
133 130 -920 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 130 -918 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 131 -942 802 78 0 1 [97, 119, 97] 14 0 0x8e00 0x2000
123 131 -940 802 77 0 1 [97, 119, 97] 12 0 0x8d00 0x2000
124 131 -938 802 77 0 1 [97, 119, 97] 11 0 0x8d00 0x2000
125 131 -936 802 66 0 1 [134, 183, 131] 11 0 0x8200 0x4100
126 131 -934 802 62 1 1 [63, 118, 228] 50 0 0x7e00 0xd800
127 131 -932 802 62 1 1 [63, 118, 228] 50 0 0x7e00 0xd800
128 131 -930 802 62 1 1 [63, 118, 228] 49 0 0x7e00 0xd800
129 131 -928 802 62 1 1 [63, 118, 228] 49 0 0x7e00 0xd800
130 131 -926 802 66 0 1 [134, 183, 131] 562 0 0x8200 0x4100
131 131 -924 802 68 0 1 [134, 183, 131] 562 0 0x8400 0x4100
132 131 -922 802 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 131 -920 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 131 -918 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 132 -942 804 79 0 1 [97, 119, 97] 14 0 0x8f00 0x2000
123 132 -940 804 76 0 1 [97, 119, 97] 12 0 0x8c00 0x2000
124 132 -938 804 76 0 1 [97, 119, 97] 11 0 0x8c00 0x2000
125 132 -936 804 76 0 1 [97, 119, 97] 11 0 0x8c00 0x2000
126 132 -934 804 62 1 1 [63, 118, 228] 50 0 0x7e00 0xd800
127 132 -932 804 62 1 1 [63, 118, 228] 50 0 0x7e00 0xd800
128 132 -930 804 62 1 1 [63, 118, 228] 49 0 0x7e00 0xd800
129 132 -928 804 62 1 1 [63, 118, 228] 602 0 0x7e00 0xd800
130 132 -926 804 65 0 1 [134, 183, 131] 562 0 0x8100 0x4100
131 132 -924 804 77 0 1 [97, 119, 97] 562 0 0x8d00 0x2000
132 132 -922 804 79 0 1 [97, 119, 97] 562 0 0x8f00 0x2000
133 132 -920 804 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 132 -918 804 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x2000
122 133 -942 806 78 0 1 [97, 119, 97] 14 0 0x8e00 0x2000
123 133 -940 806 69 0 1 [134, 183, 131] 12 0 0x8500 0x4100
124 133 -938 806 155 0 1 [134, 183, 131] 12 0 0xdb00 0x4100
125 133 -936 806 155 0 1 [134, 183, 131] 12 0 0xdb00 0x4100
126 133 -934 806 155 0 1 [134, 183, 131] 11 0 0xdb00 0x4100
127 133 -932 806 155 0 1 [134, 183, 131] 11 0 0xdb00 0x4100
128 133 -930 806 155 0 1 [134, 183, 131] 10 0 0xdb00 0x4100
129 133 -928 806 155 0 1 [134, 183, 131] 10 0 0xdb00 0x4100
130 133 -926 806 155 0 1 [134, 183, 131] 9 0 0xdb00 0x4100
131 133 -924 806 67 0 1 [134, 183, 131] 562 0 0x8300 0x4100
132 133 -922 806 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 133 -920 806 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 133 -918 806 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x2000
122 134 -942 808 79 0 1 [97, 119, 97] 14 0 0x8f00 0x2000
123 134 -940 808 68 0 1 [134, 183, 131] 12 0 0x8400 0x4100
124 134 -938 808 155 0 1 [134, 183, 131] 12 0 0xdb00 0x4100
125 134 -936 808 180 0 1 [134, 183, 131] 9 0 0xf400 0x4100
126 134 -934 808 180 0 1 [134, 183, 131] 9 0 0xf400 0x4100
127 134 -932 808 180 0 1 [134, 183, 131] 562 0 0xf400 0x4100
128 134 -930 808 180 0 1 [134, 183, 131] 562 0 0xf400 0x4100
129 134 -928 808 180 0 1 [134, 183, 131] 562 0 0xf400 0x4100
130 134 -926 808 155 0 1 [134, 183, 131] 9 0 0xdb00 0x4100
131 134 -924 808 67 0 1 [134, 183, 131] 562 0 0x8300 0x4100
132 134 -922 808 68 0 1 [134, 183, 131] 18446744073709551615 1 0x8400 0x4100
133 134 -920 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 134 -918 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
```
### stream diag after fill:
[c-LoD] stream: tick 9500 65536/65536 cells, lap 1239, cone 84696 zone 4810 dirty 939 (q 12334) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 2566048c/1031e, edits +5585p/4234c/524e drop 0, spawned 0 / failed 0, cas 0d, cam0 yaw -90 pitch 5, wish 296 forced 216, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 1575931p/3070b fail 0 ~82 kB/s cache 13519/24576 h1562412 m13519 e0, srbeat 107735s/0q drop 64stale/117undec, ereb 49793s/0q, sweep 0r/52d, cdiag off
[10:09:04 INFO]: [c-LoD] stream: tick 9500 65536/65536 cells, lap 1239, cone 84696 zone 4810 dirty 939 (q 12334) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 2566048c/1031e, edits +5585p/4234c/524e drop 0, spawned 0 / failed 0, cas 0d, cam0 yaw -90 pitch 5, wish 296 forced 216, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 1575931p/3070b fail 0 ~82 kB/s cache 13519/24576 h1562412 m13519 e0, srbeat 107735s/0q drop 64stale/117undec, ereb 49793s/0q, sweep 0r/52d, cdiag off
- carrier ladder y=155: 14956 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=165: 14864 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=168: 14852 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=172: 14736 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=176: 14764 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=180: 15112 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=185: 15268 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- probe 0: red=16054 paint=160318 lod_silhouette_cols=91 lens_red=0 lens_black=0 lens_water=0 lens_dark=0 lens_zero=0 rot=none scan=0 carriers=0 (need red>=60 paint>=300 lod>20)
- **LEG 1 PASS** (zero relogs)
- phase: 5 leg-1 probes done (10:09:41)
### stream diag after probes:
[10:09:24 INFO]: [c-LoD] stream: tick 9700 65536/65536 cells, lap 1239, cone 84696 zone 4810 dirty 957 (q 12337) ewin 183 rbeat 9p/0 rearm 0c/0t rescan 2630622c/1059e, edits +6234p/4740c/717e drop 0, spawned 0 / failed 0, cas 0d, cam0 yaw -90 pitch 5, wish 296 forced 216, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 1576813p/3082b fail 0 ~3 kB/s cache 13600/24576 h1563213 m13600 e0, srbeat 107825s/9q drop 64stale/117undec, ereb 49874s/9q, sweep 0r/30d, cdiag off
[c-LoD] stream: tick 9800 65536/65536 cells, lap 1239, cone 84696 zone 4810 dirty 966 (q 12337) ewin 183 rbeat 0p/0 rearm 0c/0t rescan 2667286c/1068e, edits +6360p/4822c/737e drop 0, spawned 0 / failed 0, cas 0d, cam0 yaw -90 pitch 5, wish 296 forced 216, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 1577542p/3092b fail 0 ~3 kB/s cache 13600/24576 h1563942 m13600 e0, srbeat 107906s/9q drop 64stale/117undec, ereb 49946s/9q, sweep 0r/30d, cdiag off
[10:09:34 INFO]: [c-LoD] stream: tick 9800 65536/65536 cells, lap 1239, cone 84696 zone 4810 dirty 966 (q 12337) ewin 183 rbeat 0p/0 rearm 0c/0t rescan 2667286c/1068e, edits +6360p/4822c/737e drop 0, spawned 0 / failed 0, cas 0d, cam0 yaw -90 pitch 5, wish 296 forced 216, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 1577542p/3092b fail 0 ~3 kB/s cache 13600/24576 h1563942 m13600 e0, srbeat 107906s/9q drop 64stale/117undec, ereb 49946s/9q, sweep 0r/30d, cdiag off
- parity debug tour: 13 vantages (session-1 client, DEBUG pack — evidence only, the gate pairs the §7b world session)
- parity debug tour done: 13 shots in parity_dbg/ (the §7b world session feeds the gate)
- joined session 2 (10:12:23)
- leg-2 baseline scene px (x<940): 24860 (0 = the relog client paints no world at all)
- leg-2 renderer probe NEAR (20 m) white px: 2570 raw / 1707 changed-vs-baseline (> 20 = the client renders particles)
- leg-2 renderer probe FAR (80 m, marker zone) white px: 2019 raw / 2199 changed-vs-baseline (> 20 = no distance cut; ~0 with NEAR alive = particles beyond the client chunk radius do not render)
- leg-2 carrier probe green dust (80 m): changed-green px 0 / total changed 14197 vs baseline (> 100 green = a stable green carrier exists; total > 0 with green ~0 = the dust renders but fog washes it — a ruler problem, not a carrier problem)
- phase: 8 leg-2 relog probes (10:17:50)
- relog probe 0: red=15876 scene=29648 lens_red=0 lens_black=0 lens_water=0 lens_dark=0 lens_zero=0 rot=decoded cam_x_b=? cam_z_b=? az_b=? el_b=? fog=live fog_end_b=? strobe_hits=22 hits_hi=22 mask_hits=22 (need red >= 60; scene > 0 = the client paints a world at all)
- **LEG 2 PASS** (marker persisted over relog)
- phase: 9 leg-2 done (10:17:53)

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
122 122 -942 784 68 0 1 [97, 119, 97] 3486 0 0x8400 0x2000
123 122 -940 784 71 0 1 [97, 119, 97] 3486 0 0x8700 0x2000
124 122 -938 784 68 0 1 [97, 119, 97] 3486 0 0x8400 0x2000
125 122 -936 784 62 1 1 [63, 118, 228] 3486 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 3486 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 3486 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 3486 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 3486 0 0x8000 0x4100
130 122 -926 784 66 0 1 [134, 183, 131] 3486 0 0x8200 0x4100
131 122 -924 784 75 0 1 [97, 119, 97] 3486 0 0x8b00 0x2000
132 122 -922 784 77 0 1 [97, 119, 97] 4048 1 0x8d00 0x2000
133 122 -920 784 75 0 1 [97, 119, 97] 4048 1 0x8b00 0x2000
134 122 -918 784 73 0 1 [97, 119, 97] 4048 1 0x8900 0x2000
122 123 -942 786 70 0 1 [97, 119, 97] 3486 0 0x8600 0x2000
123 123 -940 786 65 0 1 [97, 119, 97] 3486 0 0x8100 0x2000
124 123 -938 786 65 0 1 [97, 119, 97] 3486 0 0x8100 0x2000
125 123 -936 786 62 1 1 [63, 118, 228] 3486 0 0x7e00 0xd800
126 123 -934 786 62 1 1 [63, 118, 228] 3486 0 0x7e00 0xd800
127 123 -932 786 62 1 1 [63, 118, 228] 3486 0 0x7e00 0xd800
128 123 -930 786 62 1 1 [63, 118, 228] 3486 0 0x7e00 0xd800
129 123 -928 786 73 0 1 [97, 119, 97] 3486 0 0x8900 0x2000
130 123 -926 786 67 0 1 [134, 183, 131] 3486 0 0x8300 0x4100
131 123 -924 786 69 0 1 [134, 183, 131] 3486 0 0x8500 0x4100
132 123 -922 786 77 0 1 [97, 119, 97] 4048 1 0x8d00 0x2000
133 123 -920 786 77 0 1 [97, 119, 97] 4048 1 0x8d00 0x2000
134 123 -918 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
122 124 -942 788 72 0 1 [97, 119, 97] 3486 0 0x8800 0x2000
123 124 -940 788 70 0 1 [97, 119, 97] 3486 0 0x8600 0x2000
124 124 -938 788 68 0 1 [97, 119, 97] 3486 0 0x8400 0x2000
125 124 -936 788 62 1 1 [63, 118, 228] 3486 0 0x7e00 0xd800
126 124 -934 788 62 1 1 [63, 118, 228] 3486 0 0x7e00 0xd800
127 124 -932 788 62 1 1 [63, 118, 228] 3486 0 0x7e00 0xd800
128 124 -930 788 73 0 1 [97, 119, 97] 3486 0 0x8900 0x2000
129 124 -928 788 76 0 1 [97, 119, 97] 3486 0 0x8c00 0x2000
130 124 -926 788 73 0 1 [97, 119, 97] 3486 0 0x8900 0x2000
131 124 -924 788 69 0 1 [134, 183, 131] 3486 0 0x8500 0x4100
132 124 -922 788 77 0 1 [97, 119, 97] 4048 1 0x8d00 0x2000
133 124 -920 788 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 124 -918 788 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x2000
122 125 -942 790 74 0 1 [97, 119, 97] 3486 0 0x8a00 0x2000
123 125 -940 790 73 0 1 [97, 119, 97] 3486 0 0x8900 0x2000
124 125 -938 790 71 0 1 [97, 119, 97] 3486 0 0x8700 0x2000
125 125 -936 790 62 1 1 [63, 118, 228] 3486 0 0x7e00 0xd800
126 125 -934 790 62 1 1 [63, 118, 228] 3486 0 0x7e00 0xd800
127 125 -932 790 62 1 1 [63, 118, 228] 3486 0 0x7e00 0xd800
128 125 -930 790 62 1 1 [63, 118, 228] 3486 0 0x7e00 0xd800
129 125 -928 790 73 0 1 [97, 119, 97] 3486 0 0x8900 0x2000
130 125 -926 790 68 0 1 [134, 183, 131] 3486 0 0x8400 0x4100
131 125 -924 790 69 0 1 [134, 183, 131] 3486 0 0x8500 0x4100
132 125 -922 790 69 0 1 [134, 183, 131] 4047 1 0x8500 0x4100
133 125 -920 790 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 125 -918 790 70 0 1 [134, 183, 131] 18446744073709551615 1 0x8600 0x4100
122 126 -942 792 74 0 1 [97, 119, 97] 3486 0 0x8a00 0x2000
123 126 -940 792 73 0 1 [97, 119, 97] 3486 0 0x8900 0x2000
124 126 -938 792 68 0 1 [97, 119, 97] 3486 0 0x8400 0x2000
125 126 -936 792 62 1 1 [63, 118, 228] 3486 0 0x7e00 0xd800
126 126 -934 792 62 1 1 [63, 118, 228] 3486 0 0x7e00 0xd800
127 126 -932 792 62 1 1 [63, 118, 228] 3486 0 0x7e00 0xd800
128 126 -930 792 62 1 1 [63, 118, 228] 3486 0 0x7e00 0xd800
129 126 -928 792 67 0 1 [134, 183, 131] 3486 0 0x8300 0x4100
130 126 -926 792 74 0 1 [97, 119, 97] 3486 0 0x8a00 0x2000
131 126 -924 792 77 0 1 [97, 119, 97] 3486 0 0x8d00 0x2000
132 126 -922 792 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 126 -920 792 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3200
134 126 -918 792 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x5000
122 127 -942 794 67 0 1 [134, 183, 131] 3486 0 0x8300 0x4100
123 127 -940 794 68 0 1 [134, 183, 131] 3486 0 0x8400 0x4100
124 127 -938 794 65 0 1 [134, 183, 131] 3486 0 0x8100 0x4100
125 127 -936 794 62 1 1 [63, 118, 228] 3486 0 0x7e00 0xd800
126 127 -934 794 62 1 1 [63, 118, 228] 3486 0 0x7e00 0xd800
127 127 -932 794 170 0 1 [142, 33, 33] 3472 0 0xea00 0x0a00
128 127 -930 794 170 0 1 [142, 33, 33] 3472 0 0xea00 0x0a00
129 127 -928 794 170 0 1 [142, 33, 33] 3472 0 0xea00 0x0a00
130 127 -926 794 74 0 1 [97, 119, 97] 3486 0 0x8a00 0x2000
131 127 -924 794 77 0 1 [97, 119, 97] 3486 0 0x8d00 0x2000
132 127 -922 794 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 127 -920 794 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3200
134 127 -918 794 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x5000
122 128 -942 796 68 0 1 [134, 183, 131] 3486 0 0x8400 0x4100
123 128 -940 796 72 0 1 [97, 119, 97] 3486 0 0x8800 0x2000
124 128 -938 796 75 0 1 [97, 119, 97] 3486 0 0x8b00 0x2000
125 128 -936 796 72 0 1 [97, 119, 97] 3486 0 0x8800 0x2000
126 128 -934 796 62 1 1 [63, 118, 228] 3486 0 0x7e00 0xd800
127 128 -932 796 170 0 1 [142, 33, 33] 3472 0 0xea00 0x0a00
128 128 -930 796 170 0 1 [142, 33, 33] 3472 0 0xea00 0x0a00
129 128 -928 796 170 0 1 [142, 33, 33] 3472 0 0xea00 0x0a00
130 128 -926 796 68 0 1 [134, 183, 131] 3486 0 0x8400 0x4100
131 128 -924 796 74 0 1 [97, 119, 97] 3486 0 0x8a00 0x2000
132 128 -922 796 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 128 -920 796 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
134 128 -918 796 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 129 -942 798 69 0 1 [134, 183, 131] 3486 0 0x8500 0x4100
123 129 -940 798 75 0 1 [97, 119, 97] 3486 0 0x8b00 0x2000
124 129 -938 798 78 0 1 [97, 119, 97] 3486 0 0x8e00 0x2000
125 129 -936 798 75 0 1 [97, 119, 97] 3486 0 0x8b00 0x2000
126 129 -934 798 62 1 1 [63, 118, 228] 3486 0 0x7e00 0xd800
127 129 -932 798 170 0 1 [142, 33, 33] 3472 0 0xea00 0x0a00
128 129 -930 798 170 0 1 [142, 33, 33] 3472 0 0xea00 0x0a00
129 129 -928 798 170 0 1 [142, 33, 33] 3472 0 0xea00 0x0a00
130 129 -926 798 71 0 1 [97, 119, 97] 3486 0 0x8700 0x2000
131 129 -924 798 69 0 1 [134, 183, 131] 3486 0 0x8500 0x4100
132 129 -922 798 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 129 -920 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 129 -918 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 130 -942 800 70 0 1 [134, 183, 131] 3486 0 0x8600 0x4100
123 130 -940 800 76 0 1 [97, 119, 97] 3485 0 0x8c00 0x2000
124 130 -938 800 76 0 1 [97, 119, 97] 3485 0 0x8c00 0x2000
125 130 -936 800 72 0 1 [97, 119, 97] 3485 0 0x8800 0x2000
126 130 -934 800 62 1 1 [63, 118, 228] 3485 0 0x7e00 0xd800
127 130 -932 800 62 1 1 [63, 118, 228] 3485 0 0x7e00 0xd800
128 130 -930 800 62 1 1 [63, 118, 228] 3485 0 0x7e00 0xd800
129 130 -928 800 71 0 1 [97, 119, 97] 3485 0 0x8700 0x2000
130 130 -926 800 71 0 1 [97, 119, 97] 3485 0 0x8700 0x2000
131 130 -924 800 69 0 1 [134, 183, 131] 3485 0 0x8500 0x4100
132 130 -922 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
133 130 -920 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 130 -918 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 131 -942 802 78 0 1 [97, 119, 97] 3485 0 0x8e00 0x2000
123 131 -940 802 77 0 1 [97, 119, 97] 3485 0 0x8d00 0x2000
124 131 -938 802 77 0 1 [97, 119, 97] 3485 0 0x8d00 0x2000
125 131 -936 802 66 0 1 [134, 183, 131] 3485 0 0x8200 0x4100
126 131 -934 802 62 1 1 [63, 118, 228] 3485 0 0x7e00 0xd800
127 131 -932 802 62 1 1 [63, 118, 228] 3485 0 0x7e00 0xd800
128 131 -930 802 62 1 1 [63, 118, 228] 3485 0 0x7e00 0xd800
129 131 -928 802 62 1 1 [63, 118, 228] 3485 0 0x7e00 0xd800
130 131 -926 802 66 0 1 [134, 183, 131] 3485 0 0x8200 0x4100
131 131 -924 802 68 0 1 [134, 183, 131] 3485 0 0x8400 0x4100
132 131 -922 802 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 131 -920 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 131 -918 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 132 -942 804 79 0 1 [97, 119, 97] 3485 0 0x8f00 0x2000
123 132 -940 804 76 0 1 [97, 119, 97] 3485 0 0x8c00 0x2000
124 132 -938 804 76 0 1 [97, 119, 97] 3485 0 0x8c00 0x2000
125 132 -936 804 76 0 1 [97, 119, 97] 3485 0 0x8c00 0x2000
126 132 -934 804 62 1 1 [63, 118, 228] 3485 0 0x7e00 0xd800
127 132 -932 804 62 1 1 [63, 118, 228] 3485 0 0x7e00 0xd800
128 132 -930 804 62 1 1 [63, 118, 228] 3485 0 0x7e00 0xd800
129 132 -928 804 62 1 1 [63, 118, 228] 3485 0 0x7e00 0xd800
130 132 -926 804 65 0 1 [134, 183, 131] 3485 0 0x8100 0x4100
131 132 -924 804 77 0 1 [97, 119, 97] 3485 0 0x8d00 0x2000
132 132 -922 804 79 0 1 [97, 119, 97] 3485 0 0x8f00 0x2000
133 132 -920 804 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 132 -918 804 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x2000
122 133 -942 806 78 0 1 [97, 119, 97] 3485 0 0x8e00 0x2000
123 133 -940 806 69 0 1 [134, 183, 131] 3485 0 0x8500 0x4100
124 133 -938 806 155 0 1 [134, 183, 131] 3485 0 0xdb00 0x4100
125 133 -936 806 155 0 1 [134, 183, 131] 3485 0 0xdb00 0x4100
126 133 -934 806 155 0 1 [134, 183, 131] 3485 0 0xdb00 0x4100
127 133 -932 806 155 0 1 [134, 183, 131] 3485 0 0xdb00 0x4100
128 133 -930 806 155 0 1 [134, 183, 131] 3485 0 0xdb00 0x4100
129 133 -928 806 155 0 1 [134, 183, 131] 3485 0 0xdb00 0x4100
130 133 -926 806 155 0 1 [134, 183, 131] 3485 0 0xdb00 0x4100
131 133 -924 806 67 0 1 [134, 183, 131] 3485 0 0x8300 0x4100
132 133 -922 806 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 133 -920 806 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 133 -918 806 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x2000
122 134 -942 808 79 0 1 [97, 119, 97] 3485 0 0x8f00 0x2000
123 134 -940 808 68 0 1 [134, 183, 131] 3485 0 0x8400 0x4100
124 134 -938 808 155 0 1 [134, 183, 131] 3485 0 0xdb00 0x4100
125 134 -936 808 180 0 1 [134, 183, 131] 3485 0 0xf400 0x4100
126 134 -934 808 180 0 1 [134, 183, 131] 3485 0 0xf400 0x4100
127 134 -932 808 180 0 1 [134, 183, 131] 3485 0 0xf400 0x4100
128 134 -930 808 180 0 1 [134, 183, 131] 3485 0 0xf400 0x4100
129 134 -928 808 180 0 1 [134, 183, 131] 3485 0 0xf400 0x4100
130 134 -926 808 155 0 1 [134, 183, 131] 3485 0 0xdb00 0x4100
131 134 -924 808 67 0 1 [134, 183, 131] 3485 0 0x8300 0x4100
132 134 -922 808 68 0 1 [134, 183, 131] 18446744073709551615 1 0x8400 0x4100
133 134 -920 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 134 -918 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
```
- edit channel: 0 linkage errors, 0 failed event dispatches (both must be 0)
- phase: 6 parity world boot (10:17:53)
- parity world: view-distance=8 (the voxy leg's server value)
- parity world: joined session 3 (10:18:24)
- parity world: scan populated (cells=65536; the march paints from these buffers)
- parity world tour: 13 vantages (production pack, vd 8, shared structures)
- coverage sweeps: 72 eye passes = 36 positions (12 az x r144/200 + 12 off-az x r172, y=160) x 2 looks (outward+inward), settle-by-laps >= 6 s (q stable 3x1s), deadline 12 s/pos, no shots — the v6.26.0 data-coverage lever + v6.26.6 lottery kill; probe rides parity/view_probe.md
- parity drain-settle TIMEOUT: queue 18608 -> 18608 > 11500 after 150s — continuing anyway; the queue did not drain without eye proximity (priority-lane evidence, the 01/05/10 dip attribution reads from the tour deltas)
- pre-tour pass done: 13 vantages x 10s (the tour now reads the second OR-merge lap; the rim/dip transport race reads from the tour deltas)
- **DAY GUARD:** 01_aerial_west still night after re-pin+reshoot (zenith < 130) — renamed night_01_aerial_west.png; the row pairs as missing, NOT as a renderer diff
- **DAY GUARD:** 02_aerial_south still night after re-pin+reshoot (zenith < 130) — renamed night_02_aerial_south.png; the row pairs as missing, NOT as a renderer diff
- **DAY GUARD:** 03_aerial_north still night after re-pin+reshoot (zenith < 130) — renamed night_03_aerial_north.png; the row pairs as missing, NOT as a renderer diff
- **DAY GUARD:** 06_ground_west still night after re-pin+reshoot (zenith < 130) — renamed night_06_ground_west.png; the row pairs as missing, NOT as a renderer diff
- **DAY GUARD:** 08_back_inside still night after re-pin+reshoot (zenith < 130) — renamed night_08_back_inside.png; the row pairs as missing, NOT as a renderer diff
- **DAY GUARD:** 09_after_mesh still night after re-pin+reshoot (zenith < 130) — renamed night_09_after_mesh.png; the row pairs as missing, NOT as a renderer diff
- parity world tour done: 13 shots, 6 night-guard renames in parity/ (the gate pairs voxy/<name>.png vs parity/<name>.png)
- phase: 7 parity tour done (10:41:31)
- sparse re-beat channel at tour end: streamed=106243 q=0 dropped_stale=75 dropped_undecodable=7310 (frac=8 boot; undecodable-dominant = the rim words die in their decodability window, stale-dominant = edit churn owns the cells)
- **FAIL:** parity world time-of-day uncontrollable: 6 vantage(s) night after re-pin+reshoot — the §7b noon-pin class is broken on this runner; the matrix is infrastructure-blocked, not a renderer verdict (DAY GUARD lines + the feed's time-audit.md carry the evidence)
### parity world stream counters (session 3, tail + maxima)
[c-LoD] stream: tick 13900 65536/65536 cells, lap 1288, cone 78274 zone 25488 dirty 13397 (q 18582) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 9529457c/923e, edits +18053p/13677c/2750e drop 0, spawned 0 / failed 0, cas 0d, cam0 yaw -180 pitch 5, wish 0 forced 920, native 32256c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 1737183p/3519b fail 0 ~0 kB/s cache 23988/24576 h1271361 m465822 e441834, srbeat 106243s/0q drop 75stale/7310undec, ereb 43850s/1q, sweep 0r/8d, cdiag off
[10:41:25 INFO]: [c-LoD] stream: tick 13900 65536/65536 cells, lap 1288, cone 78274 zone 25488 dirty 13397 (q 18582) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 9529457c/923e, edits +18053p/13677c/2750e drop 0, spawned 0 / failed 0, cas 0d, cam0 yaw -180 pitch 5, wish 0 forced 920, native 32256c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 1737183p/3519b fail 0 ~0 kB/s cache 23988/24576 h1271361 m465822 e441834, srbeat 106243s/0q drop 75stale/7310undec, ereb 43850s/1q, sweep 0r/8d, cdiag off
- parity stream max cone: 78274
- parity stream max zone: 25488
- parity stream max wire bundle: bundle
### parity world capture lens (debug chain on the §7b world)
lens_red=? lens_black=? lens_water=? lens_dark=? lens_zero=? rot=? scan=? carriers=?
- mask8: the un-fog engagement masks shot on the PARITY WORLD (vd 8) — parity_dbg8/01m|07m|10m
- mask8 01m: green=23488 red=0 blue=203456 dim=382016 (px, sampled /16)
- mask8 07m: green=9568 red=0 blue=153472 dim=10784 (px, sampled /16)
- mask8 10m: green=7248 red=0 blue=207520 dim=144880 (px, sampled /16)
- mesh probe @png_y=2: x306=(83,102,141) x307=(83,102,141) x308=(83,102,141) x309h=(83,102,141) x310c=(83,102,141)
- mesh probe @png_y=717: x306=(25,35,29) x307=(26,36,30) x308=(45,60,49) x309h=(45,60,49) x310c=(47,62,50)
- mesh engagement: magenta-ish=1504 of 921600 sampled px (the vanilla-path paint coverage)
- mask8 fog state @png_y=2: raw (300)=(83,101,140) (301)=(83,102,141)
- mask8 fog state decoded: envStart=664 envEnd=808 rdEnd=2240 fogColor=(83,102,141)
- mask8 fog state @png_y=717: raw (300)=(26,38,30) (301)=(28,40,32)
- mask8 fog state decoded: envStart=208 envEnd=304 rdEnd=480 fogColor=(28,40,32)
- buffer dump 10m: marked h=2048/2048 c=2048/2048; height windows r0<142:0 r1:2048 r2>=202:0 maxByte=178
