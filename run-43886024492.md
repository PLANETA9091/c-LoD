- phase: 0 sanity (14:30:50)
### c-LoD visual run (-930,796, marker=red_concrete, vantage -1005,150 yaw 270)
- GL:     Vendor: Mesa (0xffffffff);    Device: llvmpipe (LLVM 21.1.8, 256 bits) (0xffffffff);OpenGL renderer string: llvmpipe (LLVM 21.1.8, 256 bits);OpenGL core profile version string: 4.5 (Core Profile) Mesa 26.0.8-1ubuntu0.3;
- boot + gates OK (14:31:14)
- phase: 1 server booted (14:31:14)
- sky-grid armed
- phase: 2 leg-1 client joined (14:31:37)
- joined session 1 (14:31:37)
- stream live: cells=65536
- phase: 3 scan populated (14:32:00)
- terrain range: 2 terraced mountains at (CX-4,CZ-20)/(CX-2,CZ+16), tops 185/190, grass terraces + stone caps, 66..82 m from the eye (inside the 83.5 m band, chunk edge 64 m)
- scan complete=1 (cells=cells=65536/65536)
- paint settled=1 (wire 584155p)
- baseline try 1: drift=468 px
- lod_silhouette_cols=43 (LOD paint ON SCREEN at 75 m if > 20 — the march-drawn marker column; vanilla cannot render there, chunk edge 64 m; #255 re-measures 49, the ZONE_FAR=48 regression #256 and the control read 0)
- 0 terrain-green px in the sky region (LOD MOUNTAINS on screen if > 5000; the range paints ColLow-green on low terraces via the fallback palette; flat-world baselines and the pack-free control read ~0)
- burst_close white px: 503 (renderer alive if > 50)
- burst_far white px: 449 (renderer alive in the marker zone if > 20)
- cut walk 40 m: 0 white px (> 20 = rendered)
- cut walk 60 m: 0 white px (> 20 = rendered)
- cut walk 100 m: 0 white px (> 20 = rendered)
- measured far-render cut: none m (G4 flap instrument; the decode proof needs >= 83 m from the y=151.6 eye)
- fill issued: 5x111x5 red_concrete at -930,796 (zone forceloaded ±48)
- phase: 4 marker filled (14:38:28)

### /clod-cells AFTER FILL (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 9 0 0x8400 0x3100
123 122 -940 784 71 0 1 [97, 119, 97] 9 0 0x8700 0x3100
124 122 -938 784 68 0 1 [97, 119, 97] 8 0 0x8400 0x3100
125 122 -936 784 62 1 1 [63, 118, 228] 8 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 7 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 7 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 6 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 557 0 0x8000 0x5c00
130 122 -926 784 66 0 1 [134, 183, 131] 557 0 0x8200 0x5c00
131 122 -924 784 75 0 1 [97, 119, 97] 513 0 0x8b00 0x3100
132 122 -922 784 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 122 -920 784 75 0 1 [97, 119, 97] 18446744073709551615 1 0x8b00 0x3100
134 122 -918 784 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x3100
122 123 -942 786 70 0 1 [97, 119, 97] 9 0 0x8600 0x3100
123 123 -940 786 65 0 1 [97, 119, 97] 9 0 0x8100 0x3100
124 123 -938 786 65 0 1 [97, 119, 97] 8 0 0x8100 0x3100
125 123 -936 786 62 1 1 [63, 118, 228] 8 0 0x7e00 0xd800
126 123 -934 786 62 1 1 [63, 118, 228] 7 0 0x7e00 0xd800
127 123 -932 786 62 1 1 [63, 118, 228] 7 0 0x7e00 0xd800
128 123 -930 786 62 1 1 [63, 118, 228] 6 0 0x7e00 0xd800
129 123 -928 786 73 0 1 [97, 119, 97] 6 0 0x8900 0x3100
130 123 -926 786 67 0 1 [134, 183, 131] 557 0 0x8300 0x5c00
131 123 -924 786 69 0 1 [134, 183, 131] 553 0 0x8500 0x5c00
132 123 -922 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 123 -920 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
134 123 -918 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
122 124 -942 788 72 0 1 [97, 119, 97] 9 0 0x8800 0x3100
123 124 -940 788 70 0 1 [97, 119, 97] 9 0 0x8600 0x3100
124 124 -938 788 68 0 1 [97, 119, 97] 8 0 0x8400 0x3100
125 124 -936 788 62 1 1 [63, 118, 228] 8 0 0x7e00 0xd800
126 124 -934 788 62 1 1 [63, 118, 228] 7 0 0x7e00 0xd800
127 124 -932 788 62 1 1 [63, 118, 228] 7 0 0x7e00 0xd800
128 124 -930 788 73 0 1 [97, 119, 97] 6 0 0x8900 0x3100
129 124 -928 788 76 0 1 [97, 119, 97] 6 0 0x8c00 0x3100
130 124 -926 788 73 0 1 [97, 119, 97] 517 0 0x8900 0x3100
131 124 -924 788 69 0 1 [134, 183, 131] 553 0 0x8500 0x5c00
132 124 -922 788 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 124 -920 788 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
134 124 -918 788 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x3100
122 125 -942 790 74 0 1 [97, 119, 97] 9 0 0x8a00 0x3100
123 125 -940 790 73 0 1 [97, 119, 97] 9 0 0x8900 0x3100
124 125 -938 790 71 0 1 [97, 119, 97] 8 0 0x8700 0x3100
125 125 -936 790 62 1 1 [63, 118, 228] 8 0 0x7e00 0xd800
126 125 -934 790 62 1 1 [63, 118, 228] 7 0 0x7e00 0xd800
127 125 -932 790 62 1 1 [63, 118, 228] 7 0 0x7e00 0xd800
128 125 -930 790 62 1 1 [63, 118, 228] 6 0 0x7e00 0xd800
129 125 -928 790 73 0 1 [97, 119, 97] 6 0 0x8900 0x3100
130 125 -926 790 68 0 1 [134, 183, 131] 559 0 0x8400 0x5c00
131 125 -924 790 69 0 1 [134, 183, 131] 553 0 0x8500 0x5c00
132 125 -922 790 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
133 125 -920 790 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
134 125 -918 790 70 0 1 [134, 183, 131] 18446744073709551615 1 0x8600 0x5c00
122 126 -942 792 74 0 1 [97, 119, 97] 9 0 0x8a00 0x3100
123 126 -940 792 73 0 1 [97, 119, 97] 9 0 0x8900 0x3100
124 126 -938 792 68 0 1 [97, 119, 97] 8 0 0x8400 0x3100
125 126 -936 792 62 1 1 [63, 118, 228] 8 0 0x7e00 0xd800
126 126 -934 792 62 1 1 [63, 118, 228] 7 0 0x7e00 0xd800
127 126 -932 792 62 1 1 [63, 118, 228] 7 0 0x7e00 0xd800
128 126 -930 792 62 1 1 [63, 118, 228] 6 0 0x7e00 0xd800
129 126 -928 792 67 0 1 [134, 183, 131] 6 0 0x8300 0x5c00
130 126 -926 792 74 0 1 [97, 119, 97] 517 0 0x8a00 0x3100
131 126 -924 792 77 0 1 [97, 119, 97] 513 0 0x8d00 0x3100
132 126 -922 792 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x3100
133 126 -920 792 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3800
134 126 -918 792 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x8800
122 127 -942 794 67 0 1 [134, 183, 131] 9 0 0x8300 0x5c00
123 127 -940 794 68 0 1 [134, 183, 131] 9 0 0x8400 0x5c00
124 127 -938 794 65 0 1 [134, 183, 131] 8 0 0x8100 0x5c00
125 127 -936 794 62 1 1 [63, 118, 228] 8 0 0x7e00 0xd800
126 127 -934 794 62 1 1 [63, 118, 228] 7 0 0x7e00 0xd800
127 127 -932 794 62 1 1 [63, 118, 228] 7 0 0x7e00 0xd800
128 127 -930 794 62 1 1 [63, 118, 228] 6 0 0x7e00 0xd800
129 127 -928 794 66 0 1 [134, 183, 131] 6 0 0x8200 0x5c00
130 127 -926 794 74 0 1 [97, 119, 97] 517 0 0x8a00 0x3100
131 127 -924 794 77 0 1 [97, 119, 97] 513 0 0x8d00 0x3100
132 127 -922 794 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x3100
133 127 -920 794 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3800
134 127 -918 794 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x8800
122 128 -942 796 68 0 1 [134, 183, 131] 9 0 0x8400 0x5c00
123 128 -940 796 72 0 1 [97, 119, 97] 9 0 0x8800 0x3100
124 128 -938 796 75 0 1 [97, 119, 97] 8 0 0x8b00 0x3100
125 128 -936 796 72 0 1 [97, 119, 97] 8 0 0x8800 0x3100
126 128 -934 796 62 1 1 [63, 118, 228] 7 0 0x7e00 0xd800
127 128 -932 796 62 1 1 [63, 118, 228] 7 0 0x7e00 0xd800
128 128 -930 796 62 1 1 [63, 118, 228] 6 0 0x7e00 0xd800
129 128 -928 796 66 0 1 [134, 183, 131] 6 0 0x8200 0x5c00
130 128 -926 796 68 0 1 [134, 183, 131] 559 0 0x8400 0x5c00
131 128 -924 796 74 0 1 [97, 119, 97] 513 0 0x8a00 0x3100
132 128 -922 796 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 128 -920 796 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x3100
134 128 -918 796 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
122 129 -942 798 69 0 1 [134, 183, 131] 9 0 0x8500 0x5c00
123 129 -940 798 75 0 1 [97, 119, 97] 9 0 0x8b00 0x3100
124 129 -938 798 78 0 1 [97, 119, 97] 10 0 0x8e00 0x3100
125 129 -936 798 75 0 1 [97, 119, 97] 8 0 0x8b00 0x3100
126 129 -934 798 62 1 1 [63, 118, 228] 7 0 0x7e00 0xd800
127 129 -932 798 62 1 1 [63, 118, 228] 7 0 0x7e00 0xd800
128 129 -930 798 62 1 1 [63, 118, 228] 6 0 0x7e00 0xd800
129 129 -928 798 71 0 1 [97, 119, 97] 6 0 0x8700 0x3100
130 129 -926 798 71 0 1 [97, 119, 97] 517 0 0x8700 0x3100
131 129 -924 798 69 0 1 [134, 183, 131] 553 0 0x8500 0x5c00
132 129 -922 798 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x3100
133 129 -920 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
134 129 -918 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
122 130 -942 800 70 0 1 [134, 183, 131] 9 0 0x8600 0x5c00
123 130 -940 800 76 0 1 [97, 119, 97] 9 0 0x8c00 0x3100
124 130 -938 800 76 0 1 [97, 119, 97] 8 0 0x8c00 0x3100
125 130 -936 800 72 0 1 [97, 119, 97] 8 0 0x8800 0x3100
126 130 -934 800 62 1 1 [63, 118, 228] 7 0 0x7e00 0xd800
127 130 -932 800 62 1 1 [63, 118, 228] 7 0 0x7e00 0xd800
128 130 -930 800 62 1 1 [63, 118, 228] 6 0 0x7e00 0xd800
129 130 -928 800 71 0 1 [97, 119, 97] 6 0 0x8700 0x3100
130 130 -926 800 71 0 1 [97, 119, 97] 517 0 0x8700 0x3100
131 130 -924 800 69 0 1 [134, 183, 131] 553 0 0x8500 0x5c00
132 130 -922 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
133 130 -920 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
134 130 -918 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
122 131 -942 802 78 0 1 [97, 119, 97] 11 0 0x8e00 0x3100
123 131 -940 802 77 0 1 [97, 119, 97] 9 0 0x8d00 0x3100
124 131 -938 802 77 0 1 [97, 119, 97] 8 0 0x8d00 0x3100
125 131 -936 802 66 0 1 [134, 183, 131] 8 0 0x8200 0x5c00
126 131 -934 802 62 1 1 [63, 118, 228] 7 0 0x7e00 0xd800
127 131 -932 802 62 1 1 [63, 118, 228] 7 0 0x7e00 0xd800
128 131 -930 802 62 1 1 [63, 118, 228] 6 0 0x7e00 0xd800
129 131 -928 802 62 1 1 [63, 118, 228] 6 0 0x7e00 0xd800
130 131 -926 802 66 0 1 [134, 183, 131] 559 0 0x8200 0x5c00
131 131 -924 802 68 0 1 [134, 183, 131] 553 0 0x8400 0x5c00
132 131 -922 802 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 131 -920 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
134 131 -918 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
122 132 -942 804 79 0 1 [97, 119, 97] 11 0 0x8f00 0x3100
123 132 -940 804 76 0 1 [97, 119, 97] 9 0 0x8c00 0x3100
124 132 -938 804 76 0 1 [97, 119, 97] 8 0 0x8c00 0x3100
125 132 -936 804 76 0 1 [97, 119, 97] 8 0 0x8c00 0x3100
126 132 -934 804 62 1 1 [63, 118, 228] 7 0 0x7e00 0xd800
127 132 -932 804 62 1 1 [63, 118, 228] 7 0 0x7e00 0xd800
128 132 -930 804 62 1 1 [63, 118, 228] 6 0 0x7e00 0xd800
129 132 -928 804 62 1 1 [63, 118, 228] 6 0 0x7e00 0xd800
130 132 -926 804 65 0 1 [134, 183, 131] 559 0 0x8100 0x5c00
131 132 -924 804 77 0 1 [97, 119, 97] 513 0 0x8d00 0x3100
132 132 -922 804 79 0 1 [97, 119, 97] 518 0 0x8f00 0x3100
133 132 -920 804 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
134 132 -918 804 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x3100
122 133 -942 806 78 0 1 [97, 119, 97] 11 0 0x8e00 0x3100
123 133 -940 806 69 0 1 [134, 183, 131] 9 0 0x8500 0x5c00
124 133 -938 806 155 0 1 [134, 183, 131] 9 0 0xdb00 0x5c00
125 133 -936 806 155 0 1 [134, 183, 131] 9 0 0xdb00 0x5c00
126 133 -934 806 155 0 1 [134, 183, 131] 8 0 0xdb00 0x5c00
127 133 -932 806 155 0 1 [134, 183, 131] 8 0 0xdb00 0x5c00
128 133 -930 806 155 0 1 [134, 183, 131] 7 0 0xdb00 0x5c00
129 133 -928 806 155 0 1 [134, 183, 131] 7 0 0xdb00 0x5c00
130 133 -926 806 155 0 1 [134, 183, 131] 6 0 0xdb00 0x5c00
131 133 -924 806 67 0 1 [134, 183, 131] 553 0 0x8300 0x5c00
132 133 -922 806 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 133 -920 806 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
134 133 -918 806 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x3100
122 134 -942 808 79 0 1 [97, 119, 97] 11 0 0x8f00 0x3100
123 134 -940 808 68 0 1 [134, 183, 131] 9 0 0x8400 0x5c00
124 134 -938 808 155 0 1 [134, 183, 131] 9 0 0xdb00 0x5c00
125 134 -936 808 180 0 1 [134, 183, 131] 18446744073709551615 1 0xf400 0x5c00
126 134 -934 808 180 0 1 [134, 183, 131] 18446744073709551615 1 0xf400 0x5c00
127 134 -932 808 180 0 1 [134, 183, 131] 18446744073709551615 1 0xf400 0x5c00
128 134 -930 808 180 0 1 [134, 183, 131] 18446744073709551615 1 0xf400 0x5c00
129 134 -928 808 180 0 1 [134, 183, 131] 3714 1 0xf400 0x5c00
130 134 -926 808 155 0 1 [134, 183, 131] 6 0 0xdb00 0x5c00
131 134 -924 808 67 0 1 [134, 183, 131] 553 0 0x8300 0x5c00
132 134 -922 808 68 0 1 [134, 183, 131] 18446744073709551615 1 0x8400 0x5c00
133 134 -920 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
134 134 -918 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
```
### stream diag after fill:
[c-LoD] stream: tick 4200 65536/65536 cells, lap 776, cone 25448 zone 1810 dirty 869 (q 11709) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1065861c/1273e, edits +5282p/3997c/506e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 295 forced 217, native 65536c/1090ch fail 0 (3w q 0) helper on, trees 1090ch/fail 0, wire bundle 676260p/2247b fail 0 ~71 kB/s cache 13113/24576 h663147 m13113 e0, srbeat 56852s/543q drop 32stale/0undec, ereb 46238s/76q, sweep 0r/95d
[14:38:25 INFO]: [c-LoD] stream: tick 4200 65536/65536 cells, lap 776, cone 25448 zone 1810 dirty 869 (q 11709) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1065861c/1273e, edits +5282p/3997c/506e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 295 forced 217, native 65536c/1090ch fail 0 (3w q 0) helper on, trees 1090ch/fail 0, wire bundle 676260p/2247b fail 0 ~71 kB/s cache 13113/24576 h663147 m13113 e0, srbeat 56852s/543q drop 32stale/0undec, ereb 46238s/76q, sweep 0r/95d
- carrier ladder y=155: 9380 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=165: 9364 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=168: 9364 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=172: 9364 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=176: 9364 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=180: 9364 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=185: 9364 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- probe 0: red=0 paint=8297 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 1: red=0 paint=8297 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 2: red=0 paint=8637 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 3: red=0 paint=8589 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 4: red=0 paint=8686 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 5: red=0 paint=8942 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 6: red=0 paint=9198 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 7: red=0 paint=9714 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 8: red=0 paint=9928 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 9: red=0 paint=9538 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 10: red=0 paint=9537 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 11: red=0 paint=9537 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 12: red=0 paint=9537 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 13: red=0 paint=9537 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 14: red=0 paint=9843 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 15: red=0 paint=9857 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 16: red=0 paint=9831 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 17: red=0 paint=10164 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 18: red=0 paint=9593 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 19: red=0 paint=9682 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 20: red=0 paint=9921 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 21: red=0 paint=9831 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 22: red=0 paint=10437 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 23: red=0 paint=9798 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 24: red=0 paint=10039 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 25: red=0 paint=10242 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 26: red=0 paint=10288 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 27: red=0 paint=10340 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 28: red=0 paint=10677 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 29: red=0 paint=10124 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 30: red=0 paint=10274 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 31: red=0 paint=10400 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 32: red=293 paint=11325 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- **LEG 1 PASS** (zero relogs)
- phase: 5 leg-1 probes done (14:43:50)
### stream diag after probes:
[14:43:36 INFO]: [c-LoD] stream: tick 7300 65536/65536 cells, lap 957, cone 49034 zone 3378 dirty 919 (q 11722) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 2069254c/1340e, edits +6966p/5192c/865e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 295 forced 217, native 65536c/1090ch fail 0 (3w q 0) helper on, trees 1090ch/fail 0, wire bundle 977160p/2970b fail 0 ~68 kB/s cache 13205/24576 h963955 m13205 e0, srbeat 69540s/42q drop 39stale/0undec, ereb 47421s/0q, sweep 0r/56d
[c-LoD] stream: tick 7400 65536/65536 cells, lap 966, cone 50186 zone 3378 dirty 919 (q 11722) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 2099857c/1340e, edits +6966p/5192c/865e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 295 forced 217, native 65536c/1090ch fail 0 (3w q 0) helper on, trees 1090ch/fail 0, wire bundle 987916p/2986b fail 0 ~56 kB/s cache 13205/24576 h974711 m13205 e0, srbeat 69742s/335q drop 39stale/0undec, ereb 47421s/0q, sweep 0r/65d
[14:43:46 INFO]: [c-LoD] stream: tick 7400 65536/65536 cells, lap 966, cone 50186 zone 3378 dirty 919 (q 11722) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 2099857c/1340e, edits +6966p/5192c/865e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 295 forced 217, native 65536c/1090ch fail 0 (3w q 0) helper on, trees 1090ch/fail 0, wire bundle 987916p/2986b fail 0 ~56 kB/s cache 13205/24576 h974711 m13205 e0, srbeat 69742s/335q drop 39stale/0undec, ereb 47421s/0q, sweep 0r/65d
- parity debug tour: 13 vantages (session-1 client, DEBUG pack — evidence only, the gate pairs the §7b world session)
- parity debug tour done: 13 shots in parity_dbg/ (the §7b world session feeds the gate)
- joined session 2 (14:46:43)
- leg-2 baseline scene px (x<940): 0 (0 = the relog client paints no world at all)
- leg-2 renderer probe NEAR (20 m) white px: 519 raw / 519 changed-vs-baseline (> 20 = the client renders particles)
- leg-2 renderer probe FAR (80 m, marker zone) white px: 512 raw / 512 changed-vs-baseline (> 20 = no distance cut; ~0 with NEAR alive = particles beyond the client chunk radius do not render)
- leg-2 carrier probe green dust (80 m): changed-green px 0 / total changed 326 vs baseline (> 100 green = a stable green carrier exists; total > 0 with green ~0 = the dust renders but fog washes it — a ruler problem, not a carrier problem)
- phase: 8 leg-2 relog probes (14:49:34)
- LEG-2 SCENE DEAD: the relog client paints ZERO scene pixels (x<940) — pure sky + lens panels; the fresh-session capture chain never commits (renderer-dead class, #227/#228); these probes measure a worldless client, not the transport
- relog probe 0: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 mask_hits=0 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 1: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 mask_hits=0 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 2: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 mask_hits=0 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 3: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 mask_hits=0 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 4: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 mask_hits=0 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 5: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 mask_hits=0 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 6: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 mask_hits=0 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 7: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 mask_hits=0 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 8: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 mask_hits=0 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 9: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 mask_hits=0 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 10: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 mask_hits=0 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 11: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 mask_hits=0 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 12: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 mask_hits=0 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 13: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 mask_hits=0 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 14: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 mask_hits=0 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 15: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 mask_hits=0 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 16: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 mask_hits=0 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 17: red=0 scene=1065 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 18: red=0 scene=27968 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 19: red=0 scene=47818 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- **FAIL:** marker lost after relog — stream state is not persistent/deterministic
- phase: 9 leg-2 done (14:52:34)

### client log forensics (pipeline/particle/atlas — v6.8.2)
```
[14:46:30] [main/DEBUG]: Loading LWJGL system
WARNING: java.lang.System::load has been called by org.lwjgl.system.Library$$Lambda/0x00000000141e3d60 in an unnamed module (file:/home/runner/mc-test/client/libraries/org/lwjgl/lwjgl/3.4.3/lwjgl-3.4.3.jar)
WARNING: sun.misc.Unsafe::objectFieldOffset has been called by org.lwjgl.system.MemoryBackendUnsafeLegacy (file:/home/runner/mc-test/client/libraries/org/lwjgl/lwjgl/3.4.3/lwjgl-3.4.3.jar)
WARNING: Please consider reporting this to the maintainers of class org.lwjgl.system.MemoryBackendUnsafeLegacy
[14:46:30] [main/DEBUG]: Loading shaderc
[14:46:30] [main/DEBUG]: Loading OpenGL
[14:46:36] [Render thread/ERROR]: Error parsing option value 4 for option Simulation Distance: Value 4 outside of range [5:33]
[14:46:36] [Render thread/INFO]: Backend library: LWJGL version 3.4.3+4
[14:46:36] [Download-2/ERROR]: Failed to fetch user properties
com.mojang.authlib.exceptions.InvalidCredentialsException: Status: 401
	at com.mojang.authlib.exceptions.MinecraftClientHttpException.toAuthenticationException(MinecraftClientHttpException.java:70)
Caused by: MinecraftClientHttpException[type=HTTP_ERROR, status=401, response=ErrorResponse[path=/player/attributes, error=null, errorMessage=null, details=null]]
	at com.mojang.authlib.minecraft.client.MinecraftClient.buildHttpException(MinecraftClient.java:170)
[14:46:36] [Render thread/INFO]: Using graphics backend OpenGL, using drivers: 4.5 (Core Profile) Mesa 26.0.8-1ubuntu0.3
[14:46:36] [Render thread/INFO]: Using graphics device: llvmpipe (LLVM 21.1.8, 256 bits) (Mesa)
[14:46:36] [Render thread/INFO]: Using graphics device extensions: GL_ARB_multi_draw_indirect, GL_ARB_buffer_storage, GL_ARB_shader_draw_parameters, GL_ARB_base_instance, GL_KHR_debug, GL_ARB_draw_indirect, GL_ARB_clip_control, GL_ARB_vertex_attrib_binding, GL_ARB_direct_state_access, GL_EXT_texture_filter_anisotropic
[14:46:36] [Download-2/INFO]: Could not authorize you against Realms server: java.lang.RuntimeException: Failed to parse into SignedJWT: 0
[14:46:36] [Download-2/ERROR]: Failed to fetch Realms feature flags
com.mojang.realmsclient.exception.RealmsServiceException: Realms authentication error with message 'java.lang.RuntimeException: Failed to parse into SignedJWT: 0'
[14:46:36] [Render thread/INFO]: Reloading ResourceManager: vanilla, vanilla
[14:46:39] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[14:46:39] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[14:46:39] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[14:46:39] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[14:46:39] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[14:46:39] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[14:46:39] [Render thread/INFO]: Created: 2048x2048x4 minecraft:textures/atlas/blocks.png-atlas
[14:46:40] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[14:46:40] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[14:46:40] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[14:46:40] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[14:46:40] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
[14:46:47] [Render thread/INFO]: Reloading ResourceManager: vanilla, vanilla, server/00000000/6b1f2f7e-9c0d-4a4b-8b0a-b10d1b4ab100
[14:46:49] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[14:46:49] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[14:46:49] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[14:46:49] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[14:46:49] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[14:46:49] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[14:46:49] [Render thread/INFO]: Created: 2048x2048x4 minecraft:textures/atlas/blocks.png-atlas
[14:46:49] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[14:46:49] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[14:46:49] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[14:46:49] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[14:46:49] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
```

### /clod-cells AFTER PROBES (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 60 0 0x8400 0x3100
123 122 -940 784 71 0 1 [97, 119, 97] 60 0 0x8700 0x3100
124 122 -938 784 68 0 1 [97, 119, 97] 60 0 0x8400 0x3100
125 122 -936 784 62 1 1 [63, 118, 228] 107 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 106 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 106 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 105 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 104 0 0x8000 0x5c00
130 122 -926 784 66 0 1 [134, 183, 131] 104 0 0x8200 0x5c00
131 122 -924 784 75 0 1 [97, 119, 97] 165 0 0x8b00 0x3100
132 122 -922 784 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 122 -920 784 75 0 1 [97, 119, 97] 18446744073709551615 1 0x8b00 0x3100
134 122 -918 784 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x3100
122 123 -942 786 70 0 1 [97, 119, 97] 60 0 0x8600 0x3100
123 123 -940 786 65 0 1 [97, 119, 97] 60 0 0x8100 0x3100
124 123 -938 786 65 0 1 [97, 119, 97] 60 0 0x8100 0x3100
125 123 -936 786 62 1 1 [63, 118, 228] 107 0 0x7e00 0xd800
126 123 -934 786 62 1 1 [63, 118, 228] 106 0 0x7e00 0xd800
127 123 -932 786 62 1 1 [63, 118, 228] 106 0 0x7e00 0xd800
128 123 -930 786 62 1 1 [63, 118, 228] 105 0 0x7e00 0xd800
129 123 -928 786 73 0 1 [97, 119, 97] 64 0 0x8900 0x3100
130 123 -926 786 67 0 1 [134, 183, 131] 104 0 0x8300 0x5c00
131 123 -924 786 69 0 1 [134, 183, 131] 218 0 0x8500 0x5c00
132 123 -922 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 123 -920 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
134 123 -918 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
122 124 -942 788 72 0 1 [97, 119, 97] 60 0 0x8800 0x3100
123 124 -940 788 70 0 1 [97, 119, 97] 60 0 0x8600 0x3100
124 124 -938 788 68 0 1 [97, 119, 97] 60 0 0x8400 0x3100
125 124 -936 788 62 1 1 [63, 118, 228] 107 0 0x7e00 0xd800
126 124 -934 788 62 1 1 [63, 118, 228] 106 0 0x7e00 0xd800
127 124 -932 788 62 1 1 [63, 118, 228] 106 0 0x7e00 0xd800
128 124 -930 788 73 0 1 [97, 119, 97] 59 0 0x8900 0x3100
129 124 -928 788 76 0 1 [97, 119, 97] 59 0 0x8c00 0x3100
130 124 -926 788 73 0 1 [97, 119, 97] 64 0 0x8900 0x3100
131 124 -924 788 69 0 1 [134, 183, 131] 218 0 0x8500 0x5c00
132 124 -922 788 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 124 -920 788 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
134 124 -918 788 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x3100
122 125 -942 790 74 0 1 [97, 119, 97] 60 0 0x8a00 0x3100
123 125 -940 790 73 0 1 [97, 119, 97] 64 0 0x8900 0x3100
124 125 -938 790 71 0 1 [97, 119, 97] 64 0 0x8700 0x3100
125 125 -936 790 62 1 1 [63, 118, 228] 107 0 0x7e00 0xd800
126 125 -934 790 62 1 1 [63, 118, 228] 106 0 0x7e00 0xd800
127 125 -932 790 62 1 1 [63, 118, 228] 106 0 0x7e00 0xd800
128 125 -930 790 62 1 1 [63, 118, 228] 105 0 0x7e00 0xd800
129 125 -928 790 73 0 1 [97, 119, 97] 64 0 0x8900 0x3100
130 125 -926 790 68 0 1 [134, 183, 131] 104 0 0x8400 0x5c00
131 125 -924 790 69 0 1 [134, 183, 131] 218 0 0x8500 0x5c00
132 125 -922 790 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
133 125 -920 790 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
134 125 -918 790 70 0 1 [134, 183, 131] 18446744073709551615 1 0x8600 0x5c00
122 126 -942 792 74 0 1 [97, 119, 97] 64 0 0x8a00 0x3100
123 126 -940 792 73 0 1 [97, 119, 97] 64 0 0x8900 0x3100
124 126 -938 792 68 0 1 [97, 119, 97] 64 0 0x8400 0x3100
125 126 -936 792 62 1 1 [63, 118, 228] 107 0 0x7e00 0xd800
126 126 -934 792 62 1 1 [63, 118, 228] 106 0 0x7e00 0xd800
127 126 -932 792 62 1 1 [63, 118, 228] 106 0 0x7e00 0xd800
128 126 -930 792 62 1 1 [63, 118, 228] 105 0 0x7e00 0xd800
129 126 -928 792 67 0 1 [134, 183, 131] 105 0 0x8300 0x5c00
130 126 -926 792 74 0 1 [97, 119, 97] 64 0 0x8a00 0x3100
131 126 -924 792 77 0 1 [97, 119, 97] 164 0 0x8d00 0x3100
132 126 -922 792 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x3100
133 126 -920 792 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3800
134 126 -918 792 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x8800
122 127 -942 794 67 0 1 [134, 183, 131] 109 0 0x8300 0x5c00
123 127 -940 794 68 0 1 [134, 183, 131] 108 0 0x8400 0x5c00
124 127 -938 794 65 0 1 [134, 183, 131] 108 0 0x8100 0x5c00
125 127 -936 794 62 1 1 [63, 118, 228] 107 0 0x7e00 0xd800
126 127 -934 794 62 1 1 [63, 118, 228] 107 0 0x7e00 0xd800
127 127 -932 794 170 0 1 [142, 33, 33] 66 0 0xea00 0x0200
128 127 -930 794 170 0 1 [142, 33, 33] 59 0 0xea00 0x0200
129 127 -928 794 170 0 1 [142, 33, 33] 59 0 0xea00 0x0200
130 127 -926 794 74 0 1 [97, 119, 97] 64 0 0x8a00 0x3100
131 127 -924 794 77 0 1 [97, 119, 97] 176 0 0x8d00 0x3100
132 127 -922 794 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x3100
133 127 -920 794 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3800
134 127 -918 794 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x8800
122 128 -942 796 68 0 1 [134, 183, 131] 109 0 0x8400 0x5c00
123 128 -940 796 72 0 1 [97, 119, 97] 68 0 0x8800 0x3100
124 128 -938 796 75 0 1 [97, 119, 97] 68 0 0x8b00 0x3100
125 128 -936 796 72 0 1 [97, 119, 97] 61 0 0x8800 0x3100
126 128 -934 796 62 1 1 [63, 118, 228] 107 0 0x7e00 0xd800
127 128 -932 796 170 0 1 [142, 33, 33] 66 0 0xea00 0x0200
128 128 -930 796 170 0 1 [142, 33, 33] 59 0 0xea00 0x0200
129 128 -928 796 170 0 1 [142, 33, 33] 59 0 0xea00 0x0200
130 128 -926 796 68 0 1 [134, 183, 131] 104 0 0x8400 0x5c00
131 128 -924 796 74 0 1 [97, 119, 97] 64 0 0x8a00 0x3100
132 128 -922 796 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 128 -920 796 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x3100
134 128 -918 796 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
122 129 -942 798 69 0 1 [134, 183, 131] 109 0 0x8500 0x5c00
123 129 -940 798 75 0 1 [97, 119, 97] 68 0 0x8b00 0x3100
124 129 -938 798 78 0 1 [97, 119, 97] 62 0 0x8e00 0x3100
125 129 -936 798 75 0 1 [97, 119, 97] 61 0 0x8b00 0x3100
126 129 -934 798 62 1 1 [63, 118, 228] 106 0 0x7e00 0xd800
127 129 -932 798 170 0 1 [142, 33, 33] 66 0 0xea00 0x0200
128 129 -930 798 170 0 1 [142, 33, 33] 59 0 0xea00 0x0200
129 129 -928 798 170 0 1 [142, 33, 33] 59 0 0xea00 0x0200
130 129 -926 798 71 0 1 [97, 119, 97] 64 0 0x8700 0x3100
131 129 -924 798 69 0 1 [134, 183, 131] 217 0 0x8500 0x5c00
132 129 -922 798 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x3100
133 129 -920 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
134 129 -918 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
122 130 -942 800 70 0 1 [134, 183, 131] 109 0 0x8600 0x5c00
123 130 -940 800 76 0 1 [97, 119, 97] 68 0 0x8c00 0x3100
124 130 -938 800 76 0 1 [97, 119, 97] 68 0 0x8c00 0x3100
125 130 -936 800 72 0 1 [97, 119, 97] 61 0 0x8800 0x3100
126 130 -934 800 62 1 1 [63, 118, 228] 106 0 0x7e00 0xd800
127 130 -932 800 62 1 1 [63, 118, 228] 106 0 0x7e00 0xd800
128 130 -930 800 62 1 1 [63, 118, 228] 105 0 0x7e00 0xd800
129 130 -928 800 71 0 1 [97, 119, 97] 59 0 0x8700 0x3100
130 130 -926 800 71 0 1 [97, 119, 97] 64 0 0x8700 0x3100
131 130 -924 800 69 0 1 [134, 183, 131] 217 0 0x8500 0x5c00
132 130 -922 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
133 130 -920 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
134 130 -918 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
122 131 -942 802 78 0 1 [97, 119, 97] 0 0 0x8e00 0x3100
123 131 -940 802 77 0 1 [97, 119, 97] 68 0 0x8d00 0x3100
124 131 -938 802 77 0 1 [97, 119, 97] 68 0 0x8d00 0x3100
125 131 -936 802 66 0 1 [134, 183, 131] 107 0 0x8200 0x5c00
126 131 -934 802 62 1 1 [63, 118, 228] 106 0 0x7e00 0xd800
127 131 -932 802 62 1 1 [63, 118, 228] 106 0 0x7e00 0xd800
128 131 -930 802 62 1 1 [63, 118, 228] 105 0 0x7e00 0xd800
129 131 -928 802 62 1 1 [63, 118, 228] 105 0 0x7e00 0xd800
130 131 -926 802 66 0 1 [134, 183, 131] 104 0 0x8200 0x5c00
131 131 -924 802 68 0 1 [134, 183, 131] 217 0 0x8400 0x5c00
132 131 -922 802 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 131 -920 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
134 131 -918 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
122 132 -942 804 79 0 1 [97, 119, 97] 0 0 0x8f00 0x3100
123 132 -940 804 76 0 1 [97, 119, 97] 68 0 0x8c00 0x3100
124 132 -938 804 76 0 1 [97, 119, 97] 68 0 0x8c00 0x3100
125 132 -936 804 76 0 1 [97, 119, 97] 61 0 0x8c00 0x3100
126 132 -934 804 62 1 1 [63, 118, 228] 106 0 0x7e00 0xd800
127 132 -932 804 62 1 1 [63, 118, 228] 106 0 0x7e00 0xd800
128 132 -930 804 62 1 1 [63, 118, 228] 105 0 0x7e00 0xd800
129 132 -928 804 62 1 1 [63, 118, 228] 104 0 0x7e00 0xd800
130 132 -926 804 65 0 1 [134, 183, 131] 104 0 0x8100 0x5c00
131 132 -924 804 77 0 1 [97, 119, 97] 177 0 0x8d00 0x3100
132 132 -922 804 79 0 1 [97, 119, 97] 64 0 0x8f00 0x3100
133 132 -920 804 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
134 132 -918 804 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x3100
122 133 -942 806 78 0 1 [97, 119, 97] 0 0 0x8e00 0x3100
123 133 -940 806 69 0 1 [134, 183, 131] 108 0 0x8500 0x5c00
124 133 -938 806 155 0 1 [134, 183, 131] 108 0 0xdb00 0x5c00
125 133 -936 806 155 0 1 [134, 183, 131] 108 0 0xdb00 0x5c00
126 133 -934 806 155 0 1 [134, 183, 131] 107 0 0xdb00 0x5c00
127 133 -932 806 155 0 1 [134, 183, 131] 107 0 0xdb00 0x5c00
128 133 -930 806 155 0 1 [134, 183, 131] 106 0 0xdb00 0x5c00
129 133 -928 806 155 0 1 [134, 183, 131] 105 0 0xdb00 0x5c00
130 133 -926 806 155 0 1 [134, 183, 131] 105 0 0xdb00 0x5c00
131 133 -924 806 67 0 1 [134, 183, 131] 217 0 0x8300 0x5c00
132 133 -922 806 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 133 -920 806 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
134 133 -918 806 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x3100
122 134 -942 808 79 0 1 [97, 119, 97] 0 0 0x8f00 0x3100
123 134 -940 808 68 0 1 [134, 183, 131] 108 0 0x8400 0x5c00
124 134 -938 808 155 0 1 [134, 183, 131] 108 0 0xdb00 0x5c00
125 134 -936 808 180 0 1 [134, 183, 131] 4846 0 0xf400 0x5c00
126 134 -934 808 180 0 1 [134, 183, 131] 4846 0 0xf400 0x5c00
127 134 -932 808 180 0 1 [134, 183, 131] 4846 0 0xf400 0x5c00
128 134 -930 808 180 0 1 [134, 183, 131] 4846 0 0xf400 0x5c00
129 134 -928 808 180 0 1 [134, 183, 131] 4845 0 0xf400 0x5c00
130 134 -926 808 155 0 1 [134, 183, 131] 105 0 0xdb00 0x5c00
131 134 -924 808 67 0 1 [134, 183, 131] 217 0 0x8300 0x5c00
132 134 -922 808 68 0 1 [134, 183, 131] 18446744073709551615 1 0x8400 0x5c00
133 134 -920 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
134 134 -918 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
```
- edit channel: 0 linkage errors, 0 failed event dispatches (both must be 0)
- phase: 6 parity world boot (14:52:34)
- parity world: view-distance=8 (the voxy leg's server value)
- parity world: joined session 3 (14:53:16)
- parity world: scan populated (cells=65536; the march paints from these buffers)
- parity world tour: 13 vantages (production pack, vd 8, shared structures)
- coverage sweeps: 72 eye passes = 36 positions (12 az x r144/200 + 12 off-az x r172, y=160) x 2 looks (outward+inward), settle-by-laps >= 6 s (q stable 3x1s), deadline 12 s/pos, no shots — the v6.26.0 data-coverage lever + v6.26.6 lottery kill; probe rides parity/view_probe.md
- parity drain-settle TIMEOUT: queue 14332 -> 14327 > 11500 after 150s — continuing anyway; the queue did not drain without eye proximity (priority-lane evidence, the 01/05/10 dip attribution reads from the tour deltas)
- pre-tour pass done: 13 vantages x 10s (the tour now reads the second OR-merge lap; the rim/dip transport race reads from the tour deltas)
- parity world tour done: 13 shots, 0 night-guard renames in parity/ (the gate pairs voxy/<name>.png vs parity/<name>.png)
- phase: 7 parity tour done (15:15:16)
- sparse re-beat channel at tour end: streamed=58250 q=0 dropped_stale=118 dropped_undecodable=11831 (frac=8 boot; undecodable-dominant = the rim words die in their decodability window, stale-dominant = edit churn owns the cells)
### parity world stream counters (session 3, tail + maxima)
[c-LoD] stream: tick 13300 65536/65536 cells, lap 1221, cone 69386 zone 23296 dirty 6804 (q 14326) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 9051906c/740e, edits +18088p/13703c/2821e drop 0, spawned 0 / failed 0, cam0 yaw -180 pitch 5, wish 0 forced 933, native 20800c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 1235508p/3555b fail 0 ~0 kB/s cache 20735/24576 h857424 m378084 e357349, srbeat 58250s/0q drop 118stale/11831undec, ereb 29128s/0q, sweep 0r/22d
[15:15:15 INFO]: [c-LoD] stream: tick 13300 65536/65536 cells, lap 1221, cone 69386 zone 23296 dirty 6804 (q 14326) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 9051906c/740e, edits +18088p/13703c/2821e drop 0, spawned 0 / failed 0, cam0 yaw -180 pitch 5, wish 0 forced 933, native 20800c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 1235508p/3555b fail 0 ~0 kB/s cache 20735/24576 h857424 m378084 e357349, srbeat 58250s/0q drop 118stale/11831undec, ereb 29128s/0q, sweep 0r/22d
- parity stream max cone: 69386
- parity stream max zone: 23296
- parity stream max wire bundle: bundle
### parity world capture lens (debug chain on the §7b world)
lens_red=0 lens_black=0 lens_water=0 lens_dark=0 lens_zero=0 rot=none scan=0 carriers=0
- parity mesh probe @png_y=2: x306=(151,184,255) x307=(151,184,255) x308=(151,184,255) x309h=(151,184,255) x310c=(151,184,255)
- parity mesh probe @png_y=717: x306=(44,62,51) x307=(65,89,72) x308=(81,107,87) x309h=(82,108,87) x310c=(84,111,90)
- parity mesh engagement: magenta-ish=32 of 921600 sampled px
- mask8: the un-fog engagement masks shot on the PARITY WORLD (vd 8) — parity_dbg8/01m|07m|10m
- mask8 01m: green=7568 red=0 blue=99984 dim=4096 (px, sampled /16)
- mask8 07m: green=12064 red=0 blue=221328 dim=5808 (px, sampled /16)
- mask8 10m: green=3472 red=80 blue=220336 dim=189488 (px, sampled /16)
- mesh probe @png_y=2: x306=(255,255,255) x307=(13,254,0) x308=(0,254,0) x309h=(0,0,0) x310c=(0,0,0)
- mesh probe @png_y=717: x306=(255,255,255) x307=(13,254,0) x308=(0,254,0) x309h=(0,0,0) x310c=(0,0,0)
- mesh engagement: magenta-ish=0 of 921600 sampled px (the vanilla-path paint coverage)
- mask8 fog state @png_y=2: raw (300)=(151,184,255) (301)=(151,184,255)
- mask8 fog state @png_y=717: raw (300)=(0,128,8) (301)=(179,206,255)
- mask8 fog state decoded: envStart=0 envEnd=1024 rdEnd=128 fogColor=(179,206,255)
- buffer dump 10m: marked h=17/2048 c=0/2048; height windows r0<142:12 r1:3 r2>=202:2 maxByte=223
- buffer dump 10m: RANK-2-WINDOW HEIGHTS (the fake-wall class): (+10,-20)h=143, (+8,-18)h=159
- buffer dump 10m: color idx0(black)=0 water/height mismatches=0 of 0 pairs
- buffer dump 01m: marked h=0/2048 c=0/2048; height windows r0<142:0 r1:0 r2>=202:0 maxByte=0
- buffer dump 01m: color idx0(black)=0 water/height mismatches=0 of 0 pairs
- rim mask: scan populated (cells=65536; the state paint reads the same buffers the march reads)
- phase: 10 rim mask lane (15:21:16)
- rim mask: decode-state frames shot (rim_mask/00|01|12 — GREEN full decode / RED colorless / BLUE heightless; 00 = the leg-1 tower eye, 01 = the near-tower contrast, 12 = the tower re-read; the inherited 07/12 vantages sat 760 blocks outside the streamed disk and 01 sat INSIDE the marker column — their earlier verdicts were classifier noise, purged in v6.33.1)
- rim mask 00: green=5728 red=11232 blue=275968 yellow=2512 (px, sampled /16) -> DOMINANT BLUE: rim = HEIGHTS NEVER DECODED — a data-coverage class: the state paint cannot even see the cells
- rim mask 01: green=8208 red=22736 blue=323376 yellow=3600 (px, sampled /16) -> DOMINANT BLUE: rim = HEIGHTS NEVER DECODED — a data-coverage class: the state paint cannot even see the cells
- rim mask 12: green=7488 red=10960 blue=322384 yellow=2848 (px, sampled /16) -> DOMINANT BLUE: rim = HEIGHTS NEVER DECODED — a data-coverage class: the state paint cannot even see the cells

### boot / stream
```
[15:18:52 INFO]: [c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 135240 bytes, sha1 4deb542aa160399db8dd601c64852cb635cf8531 — zero textures, zero world data, one push per join
[c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 135240 bytes, sha1 4deb542aa160399db8dd601c64852cb635cf8531 — zero textures, zero world data, one push per join
[c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[15:18:52 INFO]: [c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
stream: tick 1900 65536/65536 cells, lap 692, cone 22857 zone 1325 dirty 0 (q 3562) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1250842c/415e, edits +19p/19c/9e drop 0, spawned 0 / failed 0, cam0 none, wish 186 forced 726, native 9798c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 255728p/546b fail 0 ~36 kB/s cache 13970/24576 h241758 m13970 e0, srbeat 8906s/324q drop 0stale/440undec, ereb 0s/5q, sweep 0r/32d
[15:22:11 INFO]: [c-LoD] stream: tick 1900 65536/65536 cells, lap 692, cone 22857 zone 1325 dirty 0 (q 3562) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1250842c/415e, edits +19p/19c/9e drop 0, spawned 0 / failed 0, cam0 none, wish 186 forced 726, native 9798c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 255728p/546b fail 0 ~36 kB/s cache 13970/24576 h241758 m13970 e0, srbeat 8906s/324q drop 0stale/440undec, ereb 0s/5q, sweep 0r/32d
```
### sessions
```
[15:19:12 INFO]: System chat: Stepan joined the game
[15:22:11 INFO]: System chat: Stepan left the game
```
### client log tail
```
[15:22:08] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[15:22:08] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[15:22:08] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[15:22:08] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[15:22:08] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[15:22:08] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[15:22:08] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[15:22:08] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[15:22:08] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[15:22:08] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[15:22:08] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[15:22:08] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[15:22:08] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[15:22:08] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:set_time] net.minecraft.network.protocol.game.ClientboundSetTimePacket -> 10 bytes
[15:22:08] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:keep_alive] net.minecraft.network.protocol.common.ClientboundKeepAlivePacket -> 9 bytes
[15:22:08] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:keep_alive] net.minecraft.network.protocol.common.ServerboundKeepAlivePacket -> 9 bytes
[15:22:09] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[15:22:09] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[15:22:09] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[15:22:09] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:move_player_pos] net.minecraft.network.protocol.game.ServerboundMovePlayerPacket$Pos -> 26 bytes
[15:22:09] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[15:22:09] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[15:22:09] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[15:22:09] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[15:22:09] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[15:22:09] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[15:22:09] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[15:22:09] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[15:22:09] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[15:22:09] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[15:22:09] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[15:22:09] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[15:22:09] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[15:22:09] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[15:22:09] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[15:22:09] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[15:22:09] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[15:22:09] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[15:22:09] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[15:22:09] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[15:22:09] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[15:22:09] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:set_time] net.minecraft.network.protocol.game.ClientboundSetTimePacket -> 10 bytes
[15:22:09] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[15:22:09] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[15:22:09] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[15:22:09] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[15:22:09] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[15:22:09] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[15:22:09] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[15:22:09] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[15:22:09] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[15:22:09] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:entity_event] net.minecraft.network.protocol.game.ClientboundEntityEventPacket -> 6 bytes
[15:22:09] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[15:22:10] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[15:22:10] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[15:22:10] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:keep_alive] net.minecraft.network.protocol.common.ClientboundKeepAlivePacket -> 9 bytes
[15:22:10] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:keep_alive] net.minecraft.network.protocol.common.ServerboundKeepAlivePacket -> 9 bytes
[15:22:10] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[15:22:10] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[15:22:10] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:block_update] net.minecraft.network.protocol.game.ClientboundBlockUpdatePacket -> 12 bytes
[15:22:10] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[15:22:10] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[15:22:10] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[15:22:10] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[15:22:10] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[15:22:10] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[15:22:10] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[15:22:10] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[15:22:10] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[15:22:10] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[15:22:10] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[15:22:10] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[15:22:10] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[15:22:10] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[15:22:10] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[15:22:10] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[15:22:10] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[15:22:10] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[15:22:10] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:set_time] net.minecraft.network.protocol.game.ClientboundSetTimePacket -> 10 bytes
[15:22:10] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
--- exceptions (Realms/RTKit noise filtered) ---
25:[15:19:07] [Render thread/ERROR]: Error parsing option value 4 for option Simulation Distance: Value 4 outside of range [5:33]
39:[15:19:07] [Download-2/ERROR]: Failed to fetch user properties
40:com.mojang.authlib.exceptions.InvalidCredentialsException: Status: 401
41:	at com.mojang.authlib.exceptions.MinecraftClientHttpException.toAuthenticationException(MinecraftClientHttpException.java:70)
48:Caused by: MinecraftClientHttpException[type=HTTP_ERROR, status=401, response=ErrorResponse[path=/player/attributes, error=null, errorMessage=null, details=null]]
49:	at com.mojang.authlib.minecraft.client.MinecraftClient.buildHttpException(MinecraftClient.java:170)
57:[15:19:07] [Download-1/ERROR]: Got an error with a html body connecting to https://sessionserver.mojang.com/session/minecraft/profile/d1652ad977ce32009642f973c97adf36?unsigned=false: <!DOCTYPE html PUBLIC '-//W3C//DTD XHTML 1.0 Transitional//EN' 'http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd'>
120:MinecraftClientHttpException[type=HTTP_ERROR, status=429, response=null]
121:	at com.mojang.authlib.minecraft.client.MinecraftClient.buildHttpException(MinecraftClient.java:167)
```
### server log tail
```
[c-LoD] EB: enter, native_gen=true
[15:22:12 INFO]: [ChunkHolderManager] Saved 1002 block chunks, 975 entity chunks, 0 poi chunks in world 'minecraft:overworld' in 0.95s
[15:22:12 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:overworld'
[15:22:12 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:overworld'
[15:22:12 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[15:22:12 INFO]: [ChunkHolderManager] Waiting 60s for chunk system to halt for world 'minecraft:the_nether'
[15:22:12 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:the_nether'
[15:22:12 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_nether'
[15:22:12 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_nether' in 0.00s
[15:22:12 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_nether'
[15:22:12 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_nether'
[15:22:12 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[15:22:12 INFO]: [ChunkHolderManager] Waiting 60s for chunk system to halt for world 'minecraft:the_end'
[15:22:12 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:the_end'
[15:22:12 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_end'
[15:22:12 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_end' in 0.00s
[15:22:12 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_end'
[15:22:12 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_end'
[15:22:12 INFO]: Waiting for all RegionFile I/O tasks to complete...
[15:22:12 INFO]: All RegionFile I/O tasks to complete
[15:22:12 INFO]: [MoonriseCommon] Awaiting termination of worker pool for up to 60s...
[15:22:12 INFO]: [MoonriseCommon] Awaiting termination of I/O pool for up to 60s...
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[launcher] server exited with code 143
```
### shots
```
-rw-r--r-- 1 runner runner   2669 Nov 16 14:51 lens_13_after_relog_probe11.png
-rw-r--r-- 1 runner runner   2669 Nov 16 14:51 lens_13_after_relog_probe12.png
-rw-r--r-- 1 runner runner   2669 Nov 16 14:51 lens_13_after_relog_probe13.png
-rw-r--r-- 1 runner runner   2669 Nov 16 14:51 lens_13_after_relog_probe14.png
-rw-r--r-- 1 runner runner   2669 Nov 16 14:51 lens_13_after_relog_probe15.png
-rw-r--r-- 1 runner runner   2669 Nov 16 14:51 lens_13_after_relog_probe16.png
-rw-r--r-- 1 runner runner   5668 Nov 16 14:52 lens_13_after_relog_probe17.png
-rw-r--r-- 1 runner runner   5812 Nov 16 14:52 lens_13_after_relog_probe18.png
-rw-r--r-- 1 runner runner   6153 Nov 16 14:52 lens_13_after_relog_probe19.png
-rw-r--r-- 1 runner runner   2669 Nov 16 14:49 lens_13_after_relog_probe2.png
-rw-r--r-- 1 runner runner   2669 Nov 16 14:50 lens_13_after_relog_probe3.png
-rw-r--r-- 1 runner runner   2669 Nov 16 14:50 lens_13_after_relog_probe4.png
-rw-r--r-- 1 runner runner   2669 Nov 16 14:50 lens_13_after_relog_probe5.png
-rw-r--r-- 1 runner runner   2669 Nov 16 14:50 lens_13_after_relog_probe6.png
-rw-r--r-- 1 runner runner   2669 Nov 16 14:50 lens_13_after_relog_probe7.png
-rw-r--r-- 1 runner runner   2669 Nov 16 14:50 lens_13_after_relog_probe8.png
-rw-r--r-- 1 runner runner   2669 Nov 16 14:50 lens_13_after_relog_probe9.png
drwxr-xr-x 2 runner runner   4096 Nov 16 15:15 lens_parity
drwxr-xr-x 2 runner runner   4096 Nov 16 15:15 parity
drwxr-xr-x 2 runner runner   4096 Nov 16 14:46 parity_dbg
drwxr-xr-x 2 runner runner   4096 Nov 16 15:18 parity_dbg8
-rw-r--r-- 1 runner runner  10913 Nov 16 14:52 probe_counts.txt
drwxr-xr-x 2 runner runner   4096 Nov 16 15:22 rim_mask
-rw-r--r-- 1 runner runner 104404 Nov 16 15:22 server_full.log
-rw-r--r-- 1 runner runner  69543 Nov 16 15:22 summary.md
```

**exit=1** failures=1 leg1=1 leg2=0
