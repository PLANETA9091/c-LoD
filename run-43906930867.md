- phase: 0 sanity (16:25:16)
### c-LoD visual run (-930,796, marker=red_concrete, vantage -1005,150 yaw 270)
- GL:     Vendor: Mesa (0xffffffff);    Device: llvmpipe (LLVM 21.1.8, 256 bits) (0xffffffff);OpenGL renderer string: llvmpipe (LLVM 21.1.8, 256 bits);OpenGL core profile version string: 4.5 (Core Profile) Mesa 26.0.8-1ubuntu0.3;
- boot + gates OK (16:25:44)
- phase: 1 server booted (16:25:44)
- sky-grid armed
- phase: 2 leg-1 client joined (16:26:13)
- joined session 1 (16:26:13)
- stream live: cells=65536
- phase: 3 scan populated (16:26:36)
- terrain range: 2 terraced mountains at (CX-4,CZ-20)/(CX-2,CZ+16), tops 185/190, grass terraces + stone caps, 66..82 m from the eye (inside the 83.5 m band, chunk edge 64 m)
- scan complete=1 (cells=cells=65536/65536)
- paint settled=0 (wire 816742p)
- baseline try 1: drift=855 px
- lod_silhouette_cols=25 (LOD paint ON SCREEN at 75 m if > 20 — the march-drawn marker column; vanilla cannot render there, chunk edge 64 m; #255 re-measures 49, the ZONE_FAR=48 regression #256 and the control read 0)
- 0 terrain-green px in the sky region (LOD MOUNTAINS on screen if > 5000; the range paints ColLow-green on low terraces via the fallback palette; flat-world baselines and the pack-free control read ~0)
- burst_close white px: 0 (renderer alive if > 50)
- burst_far white px: 532 (renderer alive in the marker zone if > 20)
- cut walk 40 m: 0 white px (> 20 = rendered)
- cut walk 60 m: 40 white px (> 20 = rendered)
- cut walk 100 m: 8 white px (> 20 = rendered)
- measured far-render cut: 60 m (G4 flap instrument; the decode proof needs >= 83 m from the y=151.6 eye)
- burst_close retry white px: 0 (preflight passes if > 50)
- preflight lens_zero=0 (1 = capture NEVER committed = no particle EVER rendered)
- preflight stage eyes: ?
- fill issued: 5x111x5 red_concrete at -930,796 (zone forceloaded ±48)
- phase: 4 marker filled (16:35:30)

### /clod-cells AFTER FILL (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 18 0 0x8400 0x2000
123 122 -940 784 71 0 1 [97, 119, 97] 17 0 0x8700 0x2000
124 122 -938 784 68 0 1 [97, 119, 97] 31 0 0x8400 0x2000
125 122 -936 784 62 1 1 [63, 118, 228] 70 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 70 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 69 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 69 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 13 0 0x8000 0x4100
130 122 -926 784 66 0 1 [134, 183, 131] 26 0 0x8200 0x4100
131 122 -924 784 75 0 1 [97, 119, 97] 27 0 0x8b00 0x2000
132 122 -922 784 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 122 -920 784 75 0 1 [97, 119, 97] 18446744073709551615 1 0x8b00 0x2000
134 122 -918 784 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x2000
122 123 -942 786 70 0 1 [97, 119, 97] 19 0 0x8600 0x2000
123 123 -940 786 65 0 1 [97, 119, 97] 17 0 0x8100 0x2000
124 123 -938 786 65 0 1 [97, 119, 97] 30 0 0x8100 0x2000
125 123 -936 786 62 1 1 [63, 118, 228] 70 0 0x7e00 0xd800
126 123 -934 786 62 1 1 [63, 118, 228] 70 0 0x7e00 0xd800
127 123 -932 786 62 1 1 [63, 118, 228] 69 0 0x7e00 0xd800
128 123 -930 786 62 1 1 [63, 118, 228] 69 0 0x7e00 0xd800
129 123 -928 786 73 0 1 [97, 119, 97] 121 0 0x8900 0x2000
130 123 -926 786 67 0 1 [134, 183, 131] 26 0 0x8300 0x4100
131 123 -924 786 69 0 1 [134, 183, 131] 27 0 0x8500 0x4100
132 123 -922 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 123 -920 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 123 -918 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
122 124 -942 788 72 0 1 [97, 119, 97] 19 0 0x8800 0x2000
123 124 -940 788 70 0 1 [97, 119, 97] 17 0 0x8600 0x2000
124 124 -938 788 68 0 1 [97, 119, 97] 30 0 0x8400 0x2000
125 124 -936 788 62 1 1 [63, 118, 228] 71 0 0x7e00 0xd800
126 124 -934 788 62 1 1 [63, 118, 228] 70 0 0x7e00 0xd800
127 124 -932 788 62 1 1 [63, 118, 228] 69 0 0x7e00 0xd800
128 124 -930 788 73 0 1 [97, 119, 97] 29 0 0x8900 0x2000
129 124 -928 788 76 0 1 [97, 119, 97] 121 0 0x8c00 0x2000
130 124 -926 788 73 0 1 [97, 119, 97] 12 0 0x8900 0x2000
131 124 -924 788 69 0 1 [134, 183, 131] 27 0 0x8500 0x4100
132 124 -922 788 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 124 -920 788 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 124 -918 788 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x2000
122 125 -942 790 74 0 1 [97, 119, 97] 19 0 0x8a00 0x2000
123 125 -940 790 73 0 1 [97, 119, 97] 17 0 0x8900 0x2000
124 125 -938 790 71 0 1 [97, 119, 97] 30 0 0x8700 0x2000
125 125 -936 790 62 1 1 [63, 118, 228] 71 0 0x7e00 0xd800
126 125 -934 790 62 1 1 [63, 118, 228] 70 0 0x7e00 0xd800
127 125 -932 790 62 1 1 [63, 118, 228] 70 0 0x7e00 0xd800
128 125 -930 790 62 1 1 [63, 118, 228] 69 0 0x7e00 0xd800
129 125 -928 790 73 0 1 [97, 119, 97] 121 0 0x8900 0x2000
130 125 -926 790 68 0 1 [134, 183, 131] 12 0 0x8400 0x4100
131 125 -924 790 69 0 1 [134, 183, 131] 27 0 0x8500 0x4100
132 125 -922 790 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
133 125 -920 790 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 125 -918 790 70 0 1 [134, 183, 131] 18446744073709551615 1 0x8600 0x4100
122 126 -942 792 74 0 1 [97, 119, 97] 126 0 0x8a00 0x2000
123 126 -940 792 73 0 1 [97, 119, 97] 18 0 0x8900 0x2000
124 126 -938 792 68 0 1 [97, 119, 97] 30 0 0x8400 0x2000
125 126 -936 792 62 1 1 [63, 118, 228] 71 0 0x7e00 0xd800
126 126 -934 792 62 1 1 [63, 118, 228] 70 0 0x7e00 0xd800
127 126 -932 792 62 1 1 [63, 118, 228] 70 0 0x7e00 0xd800
128 126 -930 792 62 1 1 [63, 118, 228] 69 0 0x7e00 0xd800
129 126 -928 792 67 0 1 [134, 183, 131] 122 0 0x8300 0x4100
130 126 -926 792 74 0 1 [97, 119, 97] 12 0 0x8a00 0x2000
131 126 -924 792 77 0 1 [97, 119, 97] 27 0 0x8d00 0x2000
132 126 -922 792 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 126 -920 792 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3200
134 126 -918 792 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x5000
122 127 -942 794 67 0 1 [134, 183, 131] 126 0 0x8300 0x4100
123 127 -940 794 68 0 1 [134, 183, 131] 18 0 0x8400 0x4100
124 127 -938 794 65 0 1 [134, 183, 131] 30 0 0x8100 0x4100
125 127 -936 794 62 1 1 [63, 118, 228] 71 0 0x7e00 0xd800
126 127 -934 794 62 1 1 [63, 118, 228] 70 0 0x7e00 0xd800
127 127 -932 794 62 1 1 [63, 118, 228] 70 0 0x7e00 0xd800
128 127 -930 794 62 1 1 [63, 118, 228] 69 0 0x7e00 0xd800
129 127 -928 794 66 0 1 [134, 183, 131] 122 0 0x8200 0x4100
130 127 -926 794 74 0 1 [97, 119, 97] 12 0 0x8a00 0x2000
131 127 -924 794 77 0 1 [97, 119, 97] 27 0 0x8d00 0x2000
132 127 -922 794 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 127 -920 794 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3200
134 127 -918 794 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x5000
122 128 -942 796 68 0 1 [134, 183, 131] 126 0 0x8400 0x4100
123 128 -940 796 72 0 1 [97, 119, 97] 18 0 0x8800 0x2000
124 128 -938 796 75 0 1 [97, 119, 97] 30 0 0x8b00 0x2000
125 128 -936 796 72 0 1 [97, 119, 97] 31 0 0x8800 0x2000
126 128 -934 796 62 1 1 [63, 118, 228] 70 0 0x7e00 0xd800
127 128 -932 796 62 1 1 [63, 118, 228] 70 0 0x7e00 0xd800
128 128 -930 796 62 1 1 [63, 118, 228] 69 0 0x7e00 0xd800
129 128 -928 796 66 0 1 [134, 183, 131] 122 0 0x8200 0x4100
130 128 -926 796 68 0 1 [134, 183, 131] 12 0 0x8400 0x4100
131 128 -924 796 74 0 1 [97, 119, 97] 27 0 0x8a00 0x2000
132 128 -922 796 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 128 -920 796 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
134 128 -918 796 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 129 -942 798 69 0 1 [134, 183, 131] 126 0 0x8500 0x4100
123 129 -940 798 75 0 1 [97, 119, 97] 18 0 0x8b00 0x2000
124 129 -938 798 78 0 1 [97, 119, 97] 25 0 0x8e00 0x2000
125 129 -936 798 75 0 1 [97, 119, 97] 31 0 0x8b00 0x2000
126 129 -934 798 62 1 1 [63, 118, 228] 70 0 0x7e00 0xd800
127 129 -932 798 62 1 1 [63, 118, 228] 70 0 0x7e00 0xd800
128 129 -930 798 62 1 1 [63, 118, 228] 69 0 0x7e00 0xd800
129 129 -928 798 71 0 1 [97, 119, 97] 122 0 0x8700 0x2000
130 129 -926 798 71 0 1 [97, 119, 97] 12 0 0x8700 0x2000
131 129 -924 798 69 0 1 [134, 183, 131] 27 0 0x8500 0x4100
132 129 -922 798 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 129 -920 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 129 -918 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 130 -942 800 70 0 1 [134, 183, 131] 126 0 0x8600 0x4100
123 130 -940 800 76 0 1 [97, 119, 97] 17 0 0x8c00 0x2000
124 130 -938 800 76 0 1 [97, 119, 97] 30 0 0x8c00 0x2000
125 130 -936 800 72 0 1 [97, 119, 97] 31 0 0x8800 0x2000
126 130 -934 800 62 1 1 [63, 118, 228] 70 0 0x7e00 0xd800
127 130 -932 800 62 1 1 [63, 118, 228] 70 0 0x7e00 0xd800
128 130 -930 800 62 1 1 [63, 118, 228] 69 0 0x7e00 0xd800
129 130 -928 800 71 0 1 [97, 119, 97] 122 0 0x8700 0x2000
130 130 -926 800 71 0 1 [97, 119, 97] 12 0 0x8700 0x2000
131 130 -924 800 69 0 1 [134, 183, 131] 27 0 0x8500 0x4100
132 130 -922 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
133 130 -920 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 130 -918 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 131 -942 802 78 0 1 [97, 119, 97] 32 0 0x8e00 0x2000
123 131 -940 802 77 0 1 [97, 119, 97] 17 0 0x8d00 0x2000
124 131 -938 802 77 0 1 [97, 119, 97] 30 0 0x8d00 0x2000
125 131 -936 802 66 0 1 [134, 183, 131] 31 0 0x8200 0x4100
126 131 -934 802 62 1 1 [63, 118, 228] 70 0 0x7e00 0xd800
127 131 -932 802 62 1 1 [63, 118, 228] 70 0 0x7e00 0xd800
128 131 -930 802 62 1 1 [63, 118, 228] 69 0 0x7e00 0xd800
129 131 -928 802 62 1 1 [63, 118, 228] 68 0 0x7e00 0xd800
130 131 -926 802 66 0 1 [134, 183, 131] 12 0 0x8200 0x4100
131 131 -924 802 68 0 1 [134, 183, 131] 27 0 0x8400 0x4100
132 131 -922 802 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 131 -920 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 131 -918 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 132 -942 804 79 0 1 [97, 119, 97] 125 0 0x8f00 0x2000
123 132 -940 804 76 0 1 [97, 119, 97] 17 0 0x8c00 0x2000
124 132 -938 804 76 0 1 [97, 119, 97] 30 0 0x8c00 0x2000
125 132 -936 804 76 0 1 [97, 119, 97] 16 0 0x8c00 0x2000
126 132 -934 804 62 1 1 [63, 118, 228] 70 0 0x7e00 0xd800
127 132 -932 804 62 1 1 [63, 118, 228] 69 0 0x7e00 0xd800
128 132 -930 804 62 1 1 [63, 118, 228] 69 0 0x7e00 0xd800
129 132 -928 804 62 1 1 [63, 118, 228] 68 0 0x7e00 0xd800
130 132 -926 804 65 0 1 [134, 183, 131] 26 0 0x8100 0x4100
131 132 -924 804 77 0 1 [97, 119, 97] 27 0 0x8d00 0x2000
132 132 -922 804 79 0 1 [97, 119, 97] 13 0 0x8f00 0x2000
133 132 -920 804 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 132 -918 804 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x2000
122 133 -942 806 78 0 1 [97, 119, 97] 32 0 0x8e00 0x2000
123 133 -940 806 69 0 1 [134, 183, 131] 17 0 0x8500 0x4100
124 133 -938 806 155 0 1 [134, 183, 131] 19 0 0xdb00 0x4100
125 133 -936 806 155 0 1 [134, 183, 131] 17 0 0xdb00 0x4100
126 133 -934 806 155 0 1 [134, 183, 131] 31 0 0xdb00 0x4100
127 133 -932 806 155 0 1 [134, 183, 131] 124 0 0xdb00 0x4100
128 133 -930 806 155 0 1 [134, 183, 131] 15 0 0xdb00 0x4100
129 133 -928 806 155 0 1 [134, 183, 131] 28 0 0xdb00 0x4100
130 133 -926 806 155 0 1 [134, 183, 131] 122 0 0xdb00 0x4100
131 133 -924 806 67 0 1 [134, 183, 131] 27 0 0x8300 0x4100
132 133 -922 806 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 133 -920 806 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 133 -918 806 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x2000
122 134 -942 808 79 0 1 [97, 119, 97] 32 0 0x8f00 0x2000
123 134 -940 808 68 0 1 [134, 183, 131] 24 0 0x8400 0x4100
124 134 -938 808 155 0 1 [134, 183, 131] 18 0 0xdb00 0x4100
125 134 -936 808 180 0 1 [134, 183, 131] 29 0 0xf400 0x4100
126 134 -934 808 180 0 1 [134, 183, 131] 122 0 0xf400 0x4100
127 134 -932 808 180 0 1 [134, 183, 131] 13 0 0xf400 0x4100
128 134 -930 808 180 0 1 [134, 183, 131] 26 0 0xf400 0x4100
129 134 -928 808 180 0 1 [134, 183, 131] 561 0 0xf400 0x4100
130 134 -926 808 155 0 1 [134, 183, 131] 122 0 0xdb00 0x4100
131 134 -924 808 67 0 1 [134, 183, 131] 580 0 0x8300 0x4100
132 134 -922 808 68 0 1 [134, 183, 131] 18446744073709551615 1 0x8400 0x4100
133 134 -920 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 134 -918 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
```
### stream diag after fill:
[c-LoD] stream: tick 5700 65536/65536 cells, lap 932, cone 44553 zone 1896 dirty 936 (q 14073) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1484044c/2388e, edits +5269p/3960c/515e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 267 forced 245, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 983251p/2871b fail 0 ~83 kB/s cache 13613/24576 h969638 m13613 e0, srbeat 76138s/0q drop 63stale/116undec, ereb 48435s/0q, sweep 0r/34d
[16:35:27 INFO]: [c-LoD] stream: tick 5700 65536/65536 cells, lap 932, cone 44553 zone 1896 dirty 936 (q 14073) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1484044c/2388e, edits +5269p/3960c/515e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 267 forced 245, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 983251p/2871b fail 0 ~83 kB/s cache 13613/24576 h969638 m13613 e0, srbeat 76138s/0q drop 63stale/116undec, ereb 48435s/0q, sweep 0r/34d
- carrier ladder y=155: 11572 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=165: 11576 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=168: 11576 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=172: 11576 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=176: 11576 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=180: 11576 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=185: 11576 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- probe 0: red=0 paint=3338 lod_silhouette_cols=21 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=6 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 1: red=0 paint=3338 lod_silhouette_cols=21 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=6 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 2: red=0 paint=3531 lod_silhouette_cols=21 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=6 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 3: red=0 paint=3547 lod_silhouette_cols=21 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=6 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 4: red=0 paint=3596 lod_silhouette_cols=21 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=6 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 5: red=0 paint=4571 lod_silhouette_cols=21 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=6 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 6: red=0 paint=4458 lod_silhouette_cols=21 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=6 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 7: red=0 paint=4541 lod_silhouette_cols=21 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=6 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 8: red=0 paint=4582 lod_silhouette_cols=21 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=6 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 9: red=0 paint=4363 lod_silhouette_cols=21 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=6 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 10: red=0 paint=4350 lod_silhouette_cols=21 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=6 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 11: red=0 paint=4350 lod_silhouette_cols=21 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=6 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 12: red=0 paint=4350 lod_silhouette_cols=21 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=6 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 13: red=0 paint=4463 lod_silhouette_cols=21 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=6 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 14: red=0 paint=4507 lod_silhouette_cols=21 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=6 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 15: red=0 paint=4591 lod_silhouette_cols=21 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=6 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 16: red=0 paint=4501 lod_silhouette_cols=21 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=6 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 17: red=0 paint=4499 lod_silhouette_cols=21 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=6 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 18: red=0 paint=4536 lod_silhouette_cols=21 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=6 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 19: red=0 paint=4607 lod_silhouette_cols=21 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=6 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 20: red=0 paint=4542 lod_silhouette_cols=21 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=6 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 21: red=0 paint=4657 lod_silhouette_cols=21 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=6 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 22: red=0 paint=4551 lod_silhouette_cols=21 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=6 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 23: red=0 paint=4563 lod_silhouette_cols=21 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=6 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 24: red=0 paint=4746 lod_silhouette_cols=21 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=6 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 25: red=0 paint=4751 lod_silhouette_cols=21 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=7 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 26: red=0 paint=4797 lod_silhouette_cols=21 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=7 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 27: red=0 paint=4731 lod_silhouette_cols=21 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=7 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 28: red=0 paint=4810 lod_silhouette_cols=21 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=7 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 29: red=0 paint=4786 lod_silhouette_cols=21 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=7 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 30: red=0 paint=9737 lod_silhouette_cols=20 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=7 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 31: red=0 paint=9820 lod_silhouette_cols=20 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=7 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 32: red=0 paint=9798 lod_silhouette_cols=20 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=7 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 33: red=0 paint=9793 lod_silhouette_cols=20 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=7 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 34: red=0 paint=9817 lod_silhouette_cols=20 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=7 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 35: red=0 paint=9793 lod_silhouette_cols=20 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=7 mask_hits=30 (need red>=60 paint>=300 lod>20)
- phase: 5 leg-1 probes done (16:41:42)
- **FAIL:** leg-1 never saw the marker in 24 probes (red < 60 everywhere) — the edit never reached the LOD paint; this failure was SILENT before v6.31.1
### stream diag after probes:
[16:41:28 INFO]: [c-LoD] stream: tick 9300 65536/65536 cells, lap 1159, cone 73430 zone 3651 dirty 986 (q 14092) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 2647256c/2463e, edits +7012p/5202c/909e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 267 forced 245, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 1379503p/3846b fail 0 ~85 kB/s cache 13705/24576 h1365798 m13705 e0, srbeat 95369s/798q drop 69stale/160undec, ereb 49677s/0q, sweep 0r/100d
[c-LoD] stream: tick 9400 65536/65536 cells, lap 1160, cone 73558 zone 3671 dirty 986 (q 14092) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 2675928c/2463e, edits +7012p/5202c/909e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 267 forced 245, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 1385827p/3871b fail 0 ~33 kB/s cache 13705/24576 h1372122 m13705 e0, srbeat 96167s/23q drop 69stale/160undec, ereb 49677s/0q, sweep 0r/0d
[16:41:38 INFO]: [c-LoD] stream: tick 9400 65536/65536 cells, lap 1160, cone 73558 zone 3671 dirty 986 (q 14092) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 2675928c/2463e, edits +7012p/5202c/909e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 267 forced 245, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 1385827p/3871b fail 0 ~33 kB/s cache 13705/24576 h1372122 m13705 e0, srbeat 96167s/23q drop 69stale/160undec, ereb 49677s/0q, sweep 0r/0d
- parity debug tour: 13 vantages (session-1 client, DEBUG pack — evidence only, the gate pairs the §7b world session)
- parity debug tour done: 13 shots in parity_dbg/ (the §7b world session feeds the gate)
- joined session 2 (16:44:37)
- leg-2 baseline scene px (x<940): 0 (0 = the relog client paints no world at all)
- leg-2 renderer probe NEAR (20 m) white px: 0 raw / 0 changed-vs-baseline (> 20 = the client renders particles)
- leg-2 renderer probe FAR (80 m, marker zone) white px: 483 raw / 483 changed-vs-baseline (> 20 = no distance cut; ~0 with NEAR alive = particles beyond the client chunk radius do not render)
- leg-2 carrier probe green dust (80 m): changed-green px 0 / total changed 437 vs baseline (> 100 green = a stable green carrier exists; total > 0 with green ~0 = the dust renders but fog washes it — a ruler problem, not a carrier problem)
- phase: 8 leg-2 relog probes (16:50:05)
- relog probe 0: red=0 scene=392 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=0 mask_hits=0 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 1: red=0 scene=3850 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 2: red=0 scene=3914 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 3: red=0 scene=3872 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 4: red=0 scene=3871 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 5: red=0 scene=3281 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 6: red=0 scene=3316 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 7: red=0 scene=3622 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 8: red=0 scene=3377 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 9: red=0 scene=3314 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=1 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 10: red=0 scene=3302 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=1 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 11: red=0 scene=4980 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=1 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 12: red=0 scene=5959 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=2 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 13: red=0 scene=5963 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=2 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 14: red=0 scene=6063 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=2 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 15: red=0 scene=5925 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=2 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 16: red=0 scene=5906 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=2 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 17: red=0 scene=5912 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=2 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 18: red=0 scene=5871 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=2 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 19: red=0 scene=6194 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=2 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- **FAIL:** marker lost after relog — stream state is not persistent/deterministic
- phase: 9 leg-2 done (16:53:12)

### client log forensics (pipeline/particle/atlas — v6.8.2)
```
[16:44:23] [main/DEBUG]: Loading LWJGL system
WARNING: java.lang.System::load has been called by org.lwjgl.system.Library$$Lambda/0x00000000151e3d60 in an unnamed module (file:/home/runner/mc-test/client/libraries/org/lwjgl/lwjgl/3.4.3/lwjgl-3.4.3.jar)
WARNING: sun.misc.Unsafe::objectFieldOffset has been called by org.lwjgl.system.MemoryBackendUnsafeLegacy (file:/home/runner/mc-test/client/libraries/org/lwjgl/lwjgl/3.4.3/lwjgl-3.4.3.jar)
WARNING: Please consider reporting this to the maintainers of class org.lwjgl.system.MemoryBackendUnsafeLegacy
[16:44:23] [main/DEBUG]: Loading OpenGL
[16:44:23] [main/DEBUG]: Loading shaderc
[16:44:29] [Render thread/ERROR]: Error parsing option value 4 for option Simulation Distance: Value 4 outside of range [5:33]
[16:44:29] [Render thread/INFO]: Backend library: LWJGL version 3.4.3+4
[16:44:30] [Render thread/INFO]: Using graphics backend OpenGL, using drivers: 4.5 (Core Profile) Mesa 26.0.8-1ubuntu0.3
[16:44:30] [Render thread/INFO]: Using graphics device: llvmpipe (LLVM 21.1.8, 256 bits) (Mesa)
[16:44:30] [Render thread/INFO]: Using graphics device extensions: GL_ARB_multi_draw_indirect, GL_ARB_buffer_storage, GL_ARB_shader_draw_parameters, GL_ARB_base_instance, GL_KHR_debug, GL_ARB_draw_indirect, GL_ARB_clip_control, GL_ARB_vertex_attrib_binding, GL_ARB_direct_state_access, GL_EXT_texture_filter_anisotropic
[16:44:30] [Download-2/ERROR]: Failed to fetch user properties
com.mojang.authlib.exceptions.InvalidCredentialsException: Status: 401
	at com.mojang.authlib.exceptions.MinecraftClientHttpException.toAuthenticationException(MinecraftClientHttpException.java:70)
Caused by: MinecraftClientHttpException[type=HTTP_ERROR, status=401, response=ErrorResponse[path=/player/attributes, error=null, errorMessage=null, details=null]]
	at com.mojang.authlib.minecraft.client.MinecraftClient.buildHttpException(MinecraftClient.java:170)
[16:44:30] [Download-1/INFO]: Could not authorize you against Realms server: java.lang.RuntimeException: Failed to parse into SignedJWT: 0
[16:44:30] [Download-1/ERROR]: Failed to fetch Realms feature flags
com.mojang.realmsclient.exception.RealmsServiceException: Realms authentication error with message 'java.lang.RuntimeException: Failed to parse into SignedJWT: 0'
[16:44:30] [Render thread/INFO]: Reloading ResourceManager: vanilla, vanilla
[16:44:34] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[16:44:34] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[16:44:34] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[16:44:34] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[16:44:34] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[16:44:34] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[16:44:34] [Render thread/INFO]: Created: 2048x2048x4 minecraft:textures/atlas/blocks.png-atlas
[16:44:34] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[16:44:34] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[16:44:34] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[16:44:34] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[16:44:34] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
[16:44:41] [Render thread/INFO]: Reloading ResourceManager: vanilla, vanilla, server/00000000/6b1f2f7e-9c0d-4a4b-8b0a-b10d1b4ab100
[16:44:44] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[16:44:44] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[16:44:44] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[16:44:44] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[16:44:44] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[16:44:44] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[16:44:44] [Render thread/INFO]: Created: 2048x2048x4 minecraft:textures/atlas/blocks.png-atlas
[16:44:44] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[16:44:44] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[16:44:44] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[16:44:44] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[16:44:44] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
```

### /clod-cells AFTER PROBES (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 6 0 0x8400 0x2000
123 122 -940 784 71 0 1 [97, 119, 97] 5 0 0x8700 0x2000
124 122 -938 784 68 0 1 [97, 119, 97] 2 0 0x8400 0x2000
125 122 -936 784 62 1 1 [63, 118, 228] 5 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 4 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 3 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 3 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 2 0 0x8000 0x4100
130 122 -926 784 66 0 1 [134, 183, 131] 2 0 0x8200 0x4100
131 122 -924 784 75 0 1 [97, 119, 97] 280 0 0x8b00 0x2000
132 122 -922 784 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 122 -920 784 75 0 1 [97, 119, 97] 18446744073709551615 1 0x8b00 0x2000
134 122 -918 784 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x2000
122 123 -942 786 70 0 1 [97, 119, 97] 6 0 0x8600 0x2000
123 123 -940 786 65 0 1 [97, 119, 97] 60 0 0x8100 0x2000
124 123 -938 786 65 0 1 [97, 119, 97] 2 0 0x8100 0x2000
125 123 -936 786 62 1 1 [63, 118, 228] 5 0 0x7e00 0xd800
126 123 -934 786 62 1 1 [63, 118, 228] 4 0 0x7e00 0xd800
127 123 -932 786 62 1 1 [63, 118, 228] 4 0 0x7e00 0xd800
128 123 -930 786 62 1 1 [63, 118, 228] 3 0 0x7e00 0xd800
129 123 -928 786 73 0 1 [97, 119, 97] 3 0 0x8900 0x2000
130 123 -926 786 67 0 1 [134, 183, 131] 2 0 0x8300 0x4100
131 123 -924 786 69 0 1 [134, 183, 131] 274 0 0x8500 0x4100
132 123 -922 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 123 -920 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 123 -918 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
122 124 -942 788 72 0 1 [97, 119, 97] 6 0 0x8800 0x2000
123 124 -940 788 70 0 1 [97, 119, 97] 60 0 0x8600 0x2000
124 124 -938 788 68 0 1 [97, 119, 97] 3 0 0x8400 0x2000
125 124 -936 788 62 1 1 [63, 118, 228] 5 0 0x7e00 0xd800
126 124 -934 788 62 1 1 [63, 118, 228] 4 0 0x7e00 0xd800
127 124 -932 788 62 1 1 [63, 118, 228] 4 0 0x7e00 0xd800
128 124 -930 788 73 0 1 [97, 119, 97] 59 0 0x8900 0x2000
129 124 -928 788 76 0 1 [97, 119, 97] 3 0 0x8c00 0x2000
130 124 -926 788 73 0 1 [97, 119, 97] 2 0 0x8900 0x2000
131 124 -924 788 69 0 1 [134, 183, 131] 274 0 0x8500 0x4100
132 124 -922 788 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 124 -920 788 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 124 -918 788 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x2000
122 125 -942 790 74 0 1 [97, 119, 97] 6 0 0x8a00 0x2000
123 125 -940 790 73 0 1 [97, 119, 97] 60 0 0x8900 0x2000
124 125 -938 790 71 0 1 [97, 119, 97] 3 0 0x8700 0x2000
125 125 -936 790 62 1 1 [63, 118, 228] 5 0 0x7e00 0xd800
126 125 -934 790 62 1 1 [63, 118, 228] 4 0 0x7e00 0xd800
127 125 -932 790 62 1 1 [63, 118, 228] 4 0 0x7e00 0xd800
128 125 -930 790 62 1 1 [63, 118, 228] 3 0 0x7e00 0xd800
129 125 -928 790 73 0 1 [97, 119, 97] 58 0 0x8900 0x2000
130 125 -926 790 68 0 1 [134, 183, 131] 2 0 0x8400 0x4100
131 125 -924 790 69 0 1 [134, 183, 131] 275 0 0x8500 0x4100
132 125 -922 790 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
133 125 -920 790 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 125 -918 790 70 0 1 [134, 183, 131] 18446744073709551615 1 0x8600 0x4100
122 126 -942 792 74 0 1 [97, 119, 97] 6 0 0x8a00 0x2000
123 126 -940 792 73 0 1 [97, 119, 97] 61 0 0x8900 0x2000
124 126 -938 792 68 0 1 [97, 119, 97] 3 0 0x8400 0x2000
125 126 -936 792 62 1 1 [63, 118, 228] 5 0 0x7e00 0xd800
126 126 -934 792 62 1 1 [63, 118, 228] 5 0 0x7e00 0xd800
127 126 -932 792 62 1 1 [63, 118, 228] 4 0 0x7e00 0xd800
128 126 -930 792 62 1 1 [63, 118, 228] 3 0 0x7e00 0xd800
129 126 -928 792 67 0 1 [134, 183, 131] 58 0 0x8300 0x4100
130 126 -926 792 74 0 1 [97, 119, 97] 2 0 0x8a00 0x2000
131 126 -924 792 77 0 1 [97, 119, 97] 275 0 0x8d00 0x2000
132 126 -922 792 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 126 -920 792 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3200
134 126 -918 792 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x5000
122 127 -942 794 67 0 1 [134, 183, 131] 6 0 0x8300 0x4100
123 127 -940 794 68 0 1 [134, 183, 131] 61 0 0x8400 0x4100
124 127 -938 794 65 0 1 [134, 183, 131] 5 0 0x8100 0x4100
125 127 -936 794 62 1 1 [63, 118, 228] 5 0 0x7e00 0xd800
126 127 -934 794 62 1 1 [63, 118, 228] 5 0 0x7e00 0xd800
127 127 -932 794 170 0 1 [142, 33, 33] 4 0 0xea00 0x0a00
128 127 -930 794 170 0 1 [142, 33, 33] 60 0 0xea00 0x0a00
129 127 -928 794 170 0 1 [142, 33, 33] 58 0 0xea00 0x0a00
130 127 -926 794 74 0 1 [97, 119, 97] 2 0 0x8a00 0x2000
131 127 -924 794 77 0 1 [97, 119, 97] 275 0 0x8d00 0x2000
132 127 -922 794 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 127 -920 794 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3200
134 127 -918 794 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x5000
122 128 -942 796 68 0 1 [134, 183, 131] 6 0 0x8400 0x4100
123 128 -940 796 72 0 1 [97, 119, 97] 61 0 0x8800 0x2000
124 128 -938 796 75 0 1 [97, 119, 97] 5 0 0x8b00 0x2000
125 128 -936 796 72 0 1 [97, 119, 97] 2 0 0x8800 0x2000
126 128 -934 796 62 1 1 [63, 118, 228] 5 0 0x7e00 0xd800
127 128 -932 796 170 0 1 [142, 33, 33] 4 0 0xea00 0x0a00
128 128 -930 796 170 0 1 [142, 33, 33] 60 0 0xea00 0x0a00
129 128 -928 796 170 0 1 [142, 33, 33] 58 0 0xea00 0x0a00
130 128 -926 796 68 0 1 [134, 183, 131] 2 0 0x8400 0x4100
131 128 -924 796 74 0 1 [97, 119, 97] 275 0 0x8a00 0x2000
132 128 -922 796 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 128 -920 796 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
134 128 -918 796 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 129 -942 798 69 0 1 [134, 183, 131] 6 0 0x8500 0x4100
123 129 -940 798 75 0 1 [97, 119, 97] 61 0 0x8b00 0x2000
124 129 -938 798 78 0 1 [97, 119, 97] 5 0 0x8e00 0x2000
125 129 -936 798 75 0 1 [97, 119, 97] 2 0 0x8b00 0x2000
126 129 -934 798 62 1 1 [63, 118, 228] 5 0 0x7e00 0xd800
127 129 -932 798 170 0 1 [142, 33, 33] 4 0 0xea00 0x0a00
128 129 -930 798 170 0 1 [142, 33, 33] 60 0 0xea00 0x0a00
129 129 -928 798 170 0 1 [142, 33, 33] 58 0 0xea00 0x0a00
130 129 -926 798 71 0 1 [97, 119, 97] 2 0 0x8700 0x2000
131 129 -924 798 69 0 1 [134, 183, 131] 275 0 0x8500 0x4100
132 129 -922 798 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 129 -920 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 129 -918 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 130 -942 800 70 0 1 [134, 183, 131] 6 0 0x8600 0x4100
123 130 -940 800 76 0 1 [97, 119, 97] 61 0 0x8c00 0x2000
124 130 -938 800 76 0 1 [97, 119, 97] 3 0 0x8c00 0x2000
125 130 -936 800 72 0 1 [97, 119, 97] 5 0 0x8800 0x2000
126 130 -934 800 62 1 1 [63, 118, 228] 5 0 0x7e00 0xd800
127 130 -932 800 62 1 1 [63, 118, 228] 4 0 0x7e00 0xd800
128 130 -930 800 62 1 1 [63, 118, 228] 3 0 0x7e00 0xd800
129 130 -928 800 71 0 1 [97, 119, 97] 58 0 0x8700 0x2000
130 130 -926 800 71 0 1 [97, 119, 97] 2 0 0x8700 0x2000
131 130 -924 800 69 0 1 [134, 183, 131] 275 0 0x8500 0x4100
132 130 -922 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
133 130 -920 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 130 -918 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 131 -942 802 78 0 1 [97, 119, 97] 4 0 0x8e00 0x2000
123 131 -940 802 77 0 1 [97, 119, 97] 60 0 0x8d00 0x2000
124 131 -938 802 77 0 1 [97, 119, 97] 3 0 0x8d00 0x2000
125 131 -936 802 66 0 1 [134, 183, 131] 2 0 0x8200 0x4100
126 131 -934 802 62 1 1 [63, 118, 228] 4 0 0x7e00 0xd800
127 131 -932 802 62 1 1 [63, 118, 228] 4 0 0x7e00 0xd800
128 131 -930 802 62 1 1 [63, 118, 228] 3 0 0x7e00 0xd800
129 131 -928 802 62 1 1 [63, 118, 228] 3 0 0x7e00 0xd800
130 131 -926 802 66 0 1 [134, 183, 131] 2 0 0x8200 0x4100
131 131 -924 802 68 0 1 [134, 183, 131] 274 0 0x8400 0x4100
132 131 -922 802 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 131 -920 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 131 -918 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 132 -942 804 79 0 1 [97, 119, 97] 4 0 0x8f00 0x2000
123 132 -940 804 76 0 1 [97, 119, 97] 60 0 0x8c00 0x2000
124 132 -938 804 76 0 1 [97, 119, 97] 3 0 0x8c00 0x2000
125 132 -936 804 76 0 1 [97, 119, 97] 61 0 0x8c00 0x2000
126 132 -934 804 62 1 1 [63, 118, 228] 4 0 0x7e00 0xd800
127 132 -932 804 62 1 1 [63, 118, 228] 4 0 0x7e00 0xd800
128 132 -930 804 62 1 1 [63, 118, 228] 3 0 0x7e00 0xd800
129 132 -928 804 62 1 1 [63, 118, 228] 3 0 0x7e00 0xd800
130 132 -926 804 65 0 1 [134, 183, 131] 2 0 0x8100 0x4100
131 132 -924 804 77 0 1 [97, 119, 97] 274 0 0x8d00 0x2000
132 132 -922 804 79 0 1 [97, 119, 97] 2 0 0x8f00 0x2000
133 132 -920 804 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 132 -918 804 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x2000
122 133 -942 806 78 0 1 [97, 119, 97] 4 0 0x8e00 0x2000
123 133 -940 806 69 0 1 [134, 183, 131] 6 0 0x8500 0x4100
124 133 -938 806 155 0 1 [134, 183, 131] 6 0 0xdb00 0x4100
125 133 -936 806 155 0 1 [134, 183, 131] 60 0 0xdb00 0x4100
126 133 -934 806 155 0 1 [134, 183, 131] 2 0 0xdb00 0x4100
127 133 -932 806 155 0 1 [134, 183, 131] 61 0 0xdb00 0x4100
128 133 -930 806 155 0 1 [134, 183, 131] 4 0 0xdb00 0x4100
129 133 -928 806 155 0 1 [134, 183, 131] 3 0 0xdb00 0x4100
130 133 -926 806 155 0 1 [134, 183, 131] 59 0 0xdb00 0x4100
131 133 -924 806 67 0 1 [134, 183, 131] 274 0 0x8300 0x4100
132 133 -922 806 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 133 -920 806 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 133 -918 806 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x2000
122 134 -942 808 79 0 1 [97, 119, 97] 3 0 0x8f00 0x2000
123 134 -940 808 68 0 1 [134, 183, 131] 5 0 0x8400 0x4100
124 134 -938 808 155 0 1 [134, 183, 131] 6 0 0xdb00 0x4100
125 134 -936 808 180 0 1 [134, 183, 131] 3 0 0xf400 0x4100
126 134 -934 808 180 0 1 [134, 183, 131] 59 0 0xf400 0x4100
127 134 -932 808 180 0 1 [134, 183, 131] 2 0 0xf400 0x4100
128 134 -930 808 180 0 1 [134, 183, 131] 71 0 0xf400 0x4100
129 134 -928 808 180 0 1 [134, 183, 131] 274 0 0xf400 0x4100
130 134 -926 808 155 0 1 [134, 183, 131] 59 0 0xdb00 0x4100
131 134 -924 808 67 0 1 [134, 183, 131] 274 0 0x8300 0x4100
132 134 -922 808 68 0 1 [134, 183, 131] 18446744073709551615 1 0x8400 0x4100
133 134 -920 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 134 -918 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
```
- edit channel: 0 linkage errors, 0 failed event dispatches (both must be 0)
- phase: 6 parity world boot (16:53:13)
- parity world: view-distance=8 (the voxy leg's server value)
- parity world: joined session 3 (16:54:01)
- parity world: scan populated (cells=65536; the march paints from these buffers)
- parity world tour: 13 vantages (production pack, vd 8, shared structures)
- coverage sweeps: 72 eye passes = 36 positions (12 az x r144/200 + 12 off-az x r172, y=160) x 2 looks (outward+inward), settle-by-laps >= 6 s (q stable 3x1s), deadline 12 s/pos, no shots — the v6.26.0 data-coverage lever + v6.26.6 lottery kill; probe rides parity/view_probe.md
- parity drain-settle TIMEOUT: queue 21000 -> 20873 > 11500 after 150s — continuing anyway; the queue did not drain without eye proximity (priority-lane evidence, the 01/05/10 dip attribution reads from the tour deltas)
- pre-tour pass done: 13 vantages x 10s (the tour now reads the second OR-merge lap; the rim/dip transport race reads from the tour deltas)
- parity world tour done: 13 shots, 0 night-guard renames in parity/ (the gate pairs voxy/<name>.png vs parity/<name>.png)
- phase: 7 parity tour done (17:16:20)
- sparse re-beat channel at tour end: streamed=139299 q=0 dropped_stale=79 dropped_undecodable=16041 (frac=8 boot; undecodable-dominant = the rim words die in their decodability window, stale-dominant = edit churn owns the cells)
### parity world stream counters (session 3, tail + maxima)
[c-LoD] stream: tick 13500 65536/65536 cells, lap 1272, cone 63484 zone 23205 dirty 20722 (q 20847) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 9170007c/2028e, edits +18063p/13682c/2853e drop 0, spawned 0 / failed 0, cam0 yaw -180 pitch 5, wish 0 forced 901, native 43584c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 1936280p/5894b fail 0 ~0 kB/s cache 23563/24576 h1441647 m494633 e471070, srbeat 139299s/0q drop 79stale/16041undec, ereb 95055s/1q, sweep 0r/16d
[17:16:18 INFO]: [c-LoD] stream: tick 13500 65536/65536 cells, lap 1272, cone 63484 zone 23205 dirty 20722 (q 20847) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 9170007c/2028e, edits +18063p/13682c/2853e drop 0, spawned 0 / failed 0, cam0 yaw -180 pitch 5, wish 0 forced 901, native 43584c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 1936280p/5894b fail 0 ~0 kB/s cache 23563/24576 h1441647 m494633 e471070, srbeat 139299s/0q drop 79stale/16041undec, ereb 95055s/1q, sweep 0r/16d
- parity stream max cone: 63484
- parity stream max zone: 23205
- parity stream max wire bundle: bundle
### parity world capture lens (debug chain on the §7b world)
lens_red=0 lens_black=0 lens_water=0 lens_dark=0 lens_zero=0 rot=none scan=0 carriers=0
- parity mesh probe @png_y=2: x306=(151,184,255) x307=(151,184,255) x308=(151,184,255) x309h=(151,184,255) x310c=(151,184,255)
- parity mesh probe @png_y=717: x306=(44,62,51) x307=(65,89,72) x308=(81,107,87) x309h=(82,108,87) x310c=(84,111,90)
- parity mesh engagement: magenta-ish=0 of 921600 sampled px
- mask8: the un-fog engagement masks shot on the PARITY WORLD (vd 8) — parity_dbg8/01m|07m|10m
- mask8 01m: green=6752 red=0 blue=100592 dim=27792 (px, sampled /16)
- mask8 07m: green=11072 red=0 blue=221792 dim=5872 (px, sampled /16)
- mask8 10m: green=6816 red=112 blue=229088 dim=100880 (px, sampled /16)
- mesh probe @png_y=2: x306=(255,255,255) x307=(13,254,0) x308=(0,254,0) x309h=(49,0,0) x310c=(0,0,0)
- mesh probe @png_y=717: x306=(255,255,255) x307=(13,254,0) x308=(0,254,0) x309h=(49,0,0) x310c=(0,0,0)
- mesh engagement: magenta-ish=0 of 921600 sampled px (the vanilla-path paint coverage)
- mask8 fog state @png_y=2: raw (300)=(151,184,255) (301)=(151,184,255)
- mask8 fog state @png_y=717: raw (300)=(0,128,8) (301)=(179,206,255)
- mask8 fog state decoded: envStart=0 envEnd=1024 rdEnd=128 fogColor=(179,206,255)
- buffer dump 10m: marked h=34/2048 c=3/2048; height windows r0<142:12 r1:2 r2>=202:20 maxByte=255
- buffer dump 10m: RANK-2-WINDOW HEIGHTS (the fake-wall class): (+62,-30)h=191, (+76,-30)h=191, (+52,-26)h=175, (+76,-26)h=191, (+76,-24)h=189, (+8,-20)h=159, (+8,-18)h=159, (+2,-12)h=156, (+80,-10)h=191, (+82,-4)h=191
- rim mask: scan populated (cells=65536; the state paint reads the same buffers the march reads)
- phase: 10 rim mask lane (17:21:20)
- rim mask: decode-state frames shot (rim_mask/00|01|12 — GREEN full decode / RED colorless / BLUE heightless; 00 = the leg-1 tower eye, 01 = the near-tower contrast, 12 = the tower re-read; the inherited 07/12 vantages sat 760 blocks outside the streamed disk and 01 sat INSIDE the marker column — their earlier verdicts were classifier noise, purged in v6.33.1)
- rim mask 00: green=2448 red=6112 blue=160352 yellow=5200 (px, sampled /16) -> DOMINANT BLUE: rim = HEIGHTS NEVER DECODED — a data-coverage class: the state paint cannot even see the cells
- rim mask 01: green=20176 red=11216 blue=208960 yellow=19168 (px, sampled /16) -> DOMINANT BLUE: rim = HEIGHTS NEVER DECODED — a data-coverage class: the state paint cannot even see the cells
- rim mask 12: green=11232 red=7488 blue=205184 yellow=10272 (px, sampled /16) -> DOMINANT BLUE: rim = HEIGHTS NEVER DECODED — a data-coverage class: the state paint cannot even see the cells

### boot / stream
```
[c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 143908 bytes, sha1 d6dacc36520f5b491ba8ffa26d217a24a430e30f — zero textures, zero world data, one push per join
[17:20:24 INFO]: [c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 143908 bytes, sha1 d6dacc36520f5b491ba8ffa26d217a24a430e30f — zero textures, zero world data, one push per join
[c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[17:20:24 INFO]: [c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[c-LoD] stream: tick 1000 65536/65536 cells, lap 612, cone 11716 zone 850 dirty 19 (q 6405) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 575086c/785e, edits +11p/11c/6e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 0 forced 937, native 6848c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 146797p/501b fail 0 ~39 kB/s cache 13970/24576 h132827 m13970 e0, srbeat 7587s/630q drop 0stale/782undec, ereb 435s/21q, sweep 0r/13d
[17:22:12 INFO]: [c-LoD] stream: tick 1000 65536/65536 cells, lap 612, cone 11716 zone 850 dirty 19 (q 6405) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 575086c/785e, edits +11p/11c/6e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 0 forced 937, native 6848c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 146797p/501b fail 0 ~39 kB/s cache 13970/24576 h132827 m13970 e0, srbeat 7587s/630q drop 0stale/782undec, ereb 435s/21q, sweep 0r/13d
```
### sessions
```
[17:20:50 INFO]: System chat: Stepan joined the game
[17:22:15 INFO]: System chat: Stepan left the game
```
### client log tail
```
[17:22:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[17:22:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[17:22:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[17:22:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[17:22:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[17:22:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[17:22:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[17:22:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[17:22:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[17:22:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[17:22:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[17:22:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[17:22:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[17:22:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[17:22:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[17:22:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[17:22:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[17:22:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[17:22:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[17:22:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[17:22:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[17:22:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[17:22:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[17:22:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:bundle_delimiter] net.minecraft.network.protocol.game.ClientboundBundleDelimiterPacket -> 1 bytes
[17:22:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[17:22:14] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:move_player_pos] net.minecraft.network.protocol.game.ServerboundMovePlayerPacket$Pos -> 26 bytes
[17:22:14] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[17:22:14] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[17:22:14] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[17:22:14] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[17:22:14] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[17:22:14] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[17:22:14] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[17:22:14] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[17:22:14] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[17:22:14] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[17:22:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[17:22:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:bundle_delimiter] net.minecraft.network.protocol.game.ClientboundBundleDelimiterPacket -> 1 bytes
[17:22:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[17:22:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[17:22:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[17:22:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[17:22:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[17:22:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[17:22:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[17:22:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[17:22:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:bundle_delimiter] net.minecraft.network.protocol.game.ClientboundBundleDelimiterPacket -> 1 bytes
[17:22:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[17:22:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[17:22:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[17:22:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[17:22:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[17:22:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[17:22:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[17:22:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:keep_alive] net.minecraft.network.protocol.common.ClientboundKeepAlivePacket -> 9 bytes
[17:22:14] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:keep_alive] net.minecraft.network.protocol.common.ServerboundKeepAlivePacket -> 9 bytes
[17:22:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[17:22:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[17:22:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[17:22:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[17:22:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[17:22:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[17:22:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[17:22:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[17:22:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[17:22:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[17:22:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[17:22:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[17:22:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:block_update] net.minecraft.network.protocol.game.ClientboundBlockUpdatePacket -> 10 bytes
[17:22:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:entity_position_sync] net.minecraft.network.protocol.game.ClientboundEntityPositionSyncPacket -> 38 bytes
[17:22:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:set_time] net.minecraft.network.protocol.game.ClientboundSetTimePacket -> 10 bytes
[17:22:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[17:22:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[17:22:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[17:22:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[17:22:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[17:22:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[17:22:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[17:22:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[17:22:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
--- exceptions (Realms/RTKit noise filtered) ---
25:[17:20:42] [Render thread/ERROR]: Error parsing option value 4 for option Simulation Distance: Value 4 outside of range [5:33]
48:[17:20:43] [Download-2/ERROR]: Failed to fetch user properties
49:com.mojang.authlib.exceptions.InvalidCredentialsException: Status: 401
50:	at com.mojang.authlib.exceptions.MinecraftClientHttpException.toAuthenticationException(MinecraftClientHttpException.java:70)
57:Caused by: MinecraftClientHttpException[type=HTTP_ERROR, status=401, response=ErrorResponse[path=/player/attributes, error=null, errorMessage=null, details=null]]
58:	at com.mojang.authlib.minecraft.client.MinecraftClient.buildHttpException(MinecraftClient.java:170)
65:[17:20:43] [Download-1/ERROR]: Got an error with a html body connecting to https://sessionserver.mojang.com/session/minecraft/profile/d1652ad977ce32009642f973c97adf36?unsigned=false: <!DOCTYPE html PUBLIC '-//W3C//DTD XHTML 1.0 Transitional//EN' 'http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd'>
128:MinecraftClientHttpException[type=HTTP_ERROR, status=429, response=null]
129:	at com.mojang.authlib.minecraft.client.MinecraftClient.buildHttpException(MinecraftClient.java:167)
```
### server log tail
```
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[17:22:16 INFO]: [ChunkHolderManager] Saved 999 block chunks, 980 entity chunks, 0 poi chunks in world 'minecraft:overworld' in 1.30s
[17:22:16 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:overworld'
[17:22:16 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:overworld'
[17:22:16 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[17:22:16 INFO]: [ChunkHolderManager] Waiting 60s for chunk system to halt for world 'minecraft:the_nether'
[17:22:16 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:the_nether'
[17:22:16 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_nether'
[17:22:16 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_nether' in 0.00s
[17:22:16 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_nether'
[17:22:16 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_nether'
[17:22:16 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[17:22:16 INFO]: [ChunkHolderManager] Waiting 60s for chunk system to halt for world 'minecraft:the_end'
[17:22:16 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:the_end'
[17:22:16 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_end'
[17:22:16 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_end' in 0.00s
[17:22:16 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_end'
[17:22:16 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_end'
[17:22:16 INFO]: Waiting for all RegionFile I/O tasks to complete...
[17:22:16 INFO]: All RegionFile I/O tasks to complete
[17:22:16 INFO]: [MoonriseCommon] Awaiting termination of worker pool for up to 60s...
[17:22:16 INFO]: [MoonriseCommon] Awaiting termination of I/O pool for up to 60s...
[launcher] server exited with code 143
```
### shots
```
-rw-r--r-- 1 runner runner   6212 Nov 16 16:51 lens_13_after_relog_probe11.png
-rw-r--r-- 1 runner runner   6178 Nov 16 16:51 lens_13_after_relog_probe12.png
-rw-r--r-- 1 runner runner   6418 Nov 16 16:52 lens_13_after_relog_probe13.png
-rw-r--r-- 1 runner runner   5675 Nov 16 16:52 lens_13_after_relog_probe14.png
-rw-r--r-- 1 runner runner   5887 Nov 16 16:52 lens_13_after_relog_probe15.png
-rw-r--r-- 1 runner runner   6182 Nov 16 16:52 lens_13_after_relog_probe16.png
-rw-r--r-- 1 runner runner   6134 Nov 16 16:52 lens_13_after_relog_probe17.png
-rw-r--r-- 1 runner runner   6065 Nov 16 16:52 lens_13_after_relog_probe18.png
-rw-r--r-- 1 runner runner   6329 Nov 16 16:53 lens_13_after_relog_probe19.png
-rw-r--r-- 1 runner runner   5426 Nov 16 16:50 lens_13_after_relog_probe2.png
-rw-r--r-- 1 runner runner   6083 Nov 16 16:50 lens_13_after_relog_probe3.png
-rw-r--r-- 1 runner runner   6094 Nov 16 16:50 lens_13_after_relog_probe4.png
-rw-r--r-- 1 runner runner   6418 Nov 16 16:50 lens_13_after_relog_probe5.png
-rw-r--r-- 1 runner runner   5805 Nov 16 16:51 lens_13_after_relog_probe6.png
-rw-r--r-- 1 runner runner   5587 Nov 16 16:51 lens_13_after_relog_probe7.png
-rw-r--r-- 1 runner runner   5936 Nov 16 16:51 lens_13_after_relog_probe8.png
-rw-r--r-- 1 runner runner   6056 Nov 16 16:51 lens_13_after_relog_probe9.png
drwxr-xr-x 2 runner runner   4096 Nov 16 17:16 lens_parity
drwxr-xr-x 2 runner runner   4096 Nov 16 17:16 parity
drwxr-xr-x 2 runner runner   4096 Nov 16 16:44 parity_dbg
drwxr-xr-x 2 runner runner   4096 Nov 16 17:19 parity_dbg8
-rw-r--r-- 1 runner runner  12150 Nov 16 16:53 probe_counts.txt
drwxr-xr-x 2 runner runner   4096 Nov 16 17:22 rim_mask
-rw-r--r-- 1 runner runner  64939 Nov 16 17:22 server_full.log
-rw-r--r-- 1 runner runner  70928 Nov 16 17:22 summary.md
```

**exit=1** failures=2 leg1=0 leg2=0
