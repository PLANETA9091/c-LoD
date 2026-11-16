- phase: 0 sanity (15:39:12)
### c-LoD visual run (-930,796, marker=red_concrete, vantage -1005,150 yaw 270)
- GL:     Vendor: Mesa (0xffffffff);    Device: llvmpipe (LLVM 21.1.8, 256 bits) (0xffffffff);OpenGL renderer string: llvmpipe (LLVM 21.1.8, 256 bits);OpenGL core profile version string: 4.5 (Core Profile) Mesa 26.0.8-1ubuntu0.3;
- boot + gates OK (15:39:50)
- phase: 1 server booted (15:39:50)
- sky-grid armed
- phase: 2 leg-1 client joined (15:40:13)
- joined session 1 (15:40:13)
- stream live: cells=65536
- phase: 3 scan populated (15:40:36)
- terrain range: 2 terraced mountains at (CX-4,CZ-20)/(CX-2,CZ+16), tops 185/190, grass terraces + stone caps, 66..82 m from the eye (inside the 83.5 m band, chunk edge 64 m)
- scan complete=1 (cells=cells=65536/65536)
- paint settled=1 (wire 526849p)
- baseline try 1: drift=1445 px
- lod_silhouette_cols=30 (LOD paint ON SCREEN at 75 m if > 20 — the march-drawn marker column; vanilla cannot render there, chunk edge 64 m; #255 re-measures 49, the ZONE_FAR=48 regression #256 and the control read 0)
- 1670 terrain-green px in the sky region (LOD MOUNTAINS on screen if > 5000; the range paints ColLow-green on low terraces via the fallback palette; flat-world baselines and the pack-free control read ~0)
- burst_close white px: 0 (renderer alive if > 50)
- burst_far white px: 475 (renderer alive in the marker zone if > 20)
- cut walk 40 m: 0 white px (> 20 = rendered)
- cut walk 60 m: 0 white px (> 20 = rendered)
- cut walk 100 m: 0 white px (> 20 = rendered)
- measured far-render cut: none m (G4 flap instrument; the decode proof needs >= 83 m from the y=151.6 eye)
- burst_close retry white px: 0 (preflight passes if > 50)
- preflight lens_zero=0 (1 = capture NEVER committed = no particle EVER rendered)
- preflight stage eyes: ?
- fill issued: 5x111x5 red_concrete at -930,796 (zone forceloaded ±48)
- phase: 4 marker filled (15:46:28)

### /clod-cells AFTER FILL (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 16 0 0x8400 0x3100
123 122 -940 784 71 0 1 [97, 119, 97] 15 0 0x8700 0x3100
124 122 -938 784 68 0 1 [97, 119, 97] 20 0 0x8400 0x3100
125 122 -936 784 62 1 1 [63, 118, 228] 60 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 59 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 59 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 58 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 284 0 0x8000 0x5c00
130 122 -926 784 66 0 1 [134, 183, 131] 392 0 0x8200 0x5c00
131 122 -924 784 75 0 1 [97, 119, 97] 340 0 0x8b00 0x3100
132 122 -922 784 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 122 -920 784 75 0 1 [97, 119, 97] 18446744073709551615 1 0x8b00 0x3100
134 122 -918 784 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x3100
122 123 -942 786 66 0 1 [97, 119, 97] 16 0 0x8200 0x3100
123 123 -940 786 65 0 1 [97, 119, 97] 15 0 0x8100 0x3100
124 123 -938 786 65 0 1 [97, 119, 97] 19 0 0x8100 0x3100
125 123 -936 786 62 1 1 [63, 118, 228] 60 0 0x7e00 0xd800
126 123 -934 786 62 1 1 [63, 118, 228] 59 0 0x7e00 0xd800
127 123 -932 786 62 1 1 [63, 118, 228] 59 0 0x7e00 0xd800
128 123 -930 786 62 1 1 [63, 118, 228] 58 0 0x7e00 0xd800
129 123 -928 786 73 0 1 [97, 119, 97] 244 0 0x8900 0x3100
130 123 -926 786 67 0 1 [134, 183, 131] 392 0 0x8300 0x5c00
131 123 -924 786 69 0 1 [134, 183, 131] 392 0 0x8500 0x5c00
132 123 -922 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 123 -920 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
134 123 -918 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
122 124 -942 788 66 0 1 [97, 119, 97] 16 0 0x8200 0x3100
123 124 -940 788 64 0 1 [134, 183, 131] 61 0 0x8000 0x5c00
124 124 -938 788 62 1 1 [63, 118, 228] 60 0 0x7e00 0xd800
125 124 -936 788 62 1 1 [63, 118, 228] 60 0 0x7e00 0xd800
126 124 -934 788 62 1 1 [63, 118, 228] 59 0 0x7e00 0xd800
127 124 -932 788 62 1 1 [63, 118, 228] 59 0 0x7e00 0xd800
128 124 -930 788 73 0 1 [97, 119, 97] 18 0 0x8900 0x3100
129 124 -928 788 76 0 1 [97, 119, 97] 244 0 0x8c00 0x3100
130 124 -926 788 73 0 1 [97, 119, 97] 340 0 0x8900 0x3100
131 124 -924 788 69 0 1 [134, 183, 131] 392 0 0x8500 0x5c00
132 124 -922 788 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 124 -920 788 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
134 124 -918 788 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x3100
122 125 -942 790 65 0 1 [134, 183, 131] 61 0 0x8100 0x5c00
123 125 -940 790 65 0 1 [134, 183, 131] 61 0 0x8100 0x5c00
124 125 -938 790 62 1 1 [63, 118, 228] 60 0 0x7e00 0xd800
125 125 -936 790 62 1 1 [63, 118, 228] 60 0 0x7e00 0xd800
126 125 -934 790 62 1 1 [63, 118, 228] 59 0 0x7e00 0xd800
127 125 -932 790 62 1 1 [63, 118, 228] 59 0 0x7e00 0xd800
128 125 -930 790 62 1 1 [63, 118, 228] 58 0 0x7e00 0xd800
129 125 -928 790 73 0 1 [97, 119, 97] 18 0 0x8900 0x3100
130 125 -926 790 68 0 1 [134, 183, 131] 276 0 0x8400 0x5c00
131 125 -924 790 69 0 1 [134, 183, 131] 392 0 0x8500 0x5c00
132 125 -922 790 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
133 125 -920 790 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
134 125 -918 790 70 0 1 [134, 183, 131] 18446744073709551615 1 0x8600 0x5c00
122 126 -942 792 70 0 1 [97, 119, 97] 16 0 0x8600 0x3100
123 126 -940 792 66 0 1 [134, 183, 131] 61 0 0x8200 0x5c00
124 126 -938 792 63 0 1 [134, 183, 131] 60 0 0x7f00 0x5c00
125 126 -936 792 62 1 1 [63, 118, 228] 60 0 0x7e00 0xd800
126 126 -934 792 62 1 1 [63, 118, 228] 59 0 0x7e00 0xd800
127 126 -932 792 62 1 1 [63, 118, 228] 59 0 0x7e00 0xd800
128 126 -930 792 62 1 1 [63, 118, 228] 58 0 0x7e00 0xd800
129 126 -928 792 67 0 1 [134, 183, 131] 58 0 0x8300 0x5c00
130 126 -926 792 74 0 1 [97, 119, 97] 236 0 0x8a00 0x3100
131 126 -924 792 77 0 1 [97, 119, 97] 340 0 0x8d00 0x3100
132 126 -922 792 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x3100
133 126 -920 792 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3800
134 126 -918 792 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x8800
122 127 -942 794 70 0 1 [97, 119, 97] 16 0 0x8600 0x3100
123 127 -940 794 68 0 1 [134, 183, 131] 61 0 0x8400 0x5c00
124 127 -938 794 65 0 1 [134, 183, 131] 60 0 0x8100 0x5c00
125 127 -936 794 62 1 1 [63, 118, 228] 60 0 0x7e00 0xd800
126 127 -934 794 62 1 1 [63, 118, 228] 59 0 0x7e00 0xd800
127 127 -932 794 170 0 1 [142, 33, 33] 59 1 0xea00 0x0200
128 127 -930 794 170 0 1 [142, 33, 33] 58 1 0xea00 0x0200
129 127 -928 794 170 0 1 [142, 33, 33] 58 1 0xea00 0x0200
130 127 -926 794 74 0 1 [97, 119, 97] 236 0 0x8a00 0x3100
131 127 -924 794 77 0 1 [97, 119, 97] 340 0 0x8d00 0x3100
132 127 -922 794 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x3100
133 127 -920 794 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3800
134 127 -918 794 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x8800
122 128 -942 796 78 0 1 [97, 119, 97] 17 0 0x8e00 0x3100
123 128 -940 796 75 0 1 [97, 119, 97] 16 0 0x8b00 0x3100
124 128 -938 796 75 0 1 [97, 119, 97] 19 0 0x8b00 0x3100
125 128 -936 796 72 0 1 [97, 119, 97] 20 0 0x8800 0x3100
126 128 -934 796 62 1 1 [63, 118, 228] 59 0 0x7e00 0xd800
127 128 -932 796 170 0 1 [142, 33, 33] 59 1 0xea00 0x0200
128 128 -930 796 170 0 1 [142, 33, 33] 58 1 0xea00 0x0200
129 128 -928 796 170 0 1 [142, 33, 33] 58 1 0xea00 0x0200
130 128 -926 796 68 0 1 [134, 183, 131] 276 0 0x8400 0x5c00
131 128 -924 796 74 0 1 [97, 119, 97] 340 0 0x8a00 0x3100
132 128 -922 796 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 128 -920 796 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x3100
134 128 -918 796 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
122 129 -942 798 78 0 1 [97, 119, 97] 17 0 0x8e00 0x3100
123 129 -940 798 75 0 1 [97, 119, 97] 16 0 0x8b00 0x3100
124 129 -938 798 78 0 1 [97, 119, 97] 22 0 0x8e00 0x3100
125 129 -936 798 75 0 1 [97, 119, 97] 20 0 0x8b00 0x3100
126 129 -934 798 62 1 1 [63, 118, 228] 59 0 0x7e00 0xd800
127 129 -932 798 170 0 1 [142, 33, 33] 59 1 0xea00 0x0200
128 129 -930 798 170 0 1 [142, 33, 33] 58 1 0xea00 0x0200
129 129 -928 798 170 0 1 [142, 33, 33] 58 1 0xea00 0x0200
130 129 -926 798 71 0 1 [97, 119, 97] 236 0 0x8700 0x3100
131 129 -924 798 69 0 1 [134, 183, 131] 392 0 0x8500 0x5c00
132 129 -922 798 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x3100
133 129 -920 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
134 129 -918 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
122 130 -942 800 70 0 1 [134, 183, 131] 61 0 0x8600 0x5c00
123 130 -940 800 76 0 1 [97, 119, 97] 15 0 0x8c00 0x3100
124 130 -938 800 76 0 1 [97, 119, 97] 19 0 0x8c00 0x3100
125 130 -936 800 72 0 1 [97, 119, 97] 20 0 0x8800 0x3100
126 130 -934 800 62 1 1 [63, 118, 228] 59 0 0x7e00 0xd800
127 130 -932 800 62 1 1 [63, 118, 228] 59 0 0x7e00 0xd800
128 130 -930 800 62 1 1 [63, 118, 228] 58 0 0x7e00 0xd800
129 130 -928 800 71 0 1 [97, 119, 97] 18 0 0x8700 0x3100
130 130 -926 800 71 0 1 [97, 119, 97] 236 0 0x8700 0x3100
131 130 -924 800 69 0 1 [134, 183, 131] 392 0 0x8500 0x5c00
132 130 -922 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
133 130 -920 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
134 130 -918 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
122 131 -942 802 78 0 1 [97, 119, 97] 17 0 0x8e00 0x3100
123 131 -940 802 77 0 1 [97, 119, 97] 15 0 0x8d00 0x3100
124 131 -938 802 77 0 1 [97, 119, 97] 19 0 0x8d00 0x3100
125 131 -936 802 66 0 1 [134, 183, 131] 60 0 0x8200 0x5c00
126 131 -934 802 62 1 1 [63, 118, 228] 59 0 0x7e00 0xd800
127 131 -932 802 62 1 1 [63, 118, 228] 59 0 0x7e00 0xd800
128 131 -930 802 62 1 1 [63, 118, 228] 58 0 0x7e00 0xd800
129 131 -928 802 62 1 1 [63, 118, 228] 284 0 0x7e00 0xd800
130 131 -926 802 66 0 1 [134, 183, 131] 392 0 0x8200 0x5c00
131 131 -924 802 68 0 1 [134, 183, 131] 392 0 0x8400 0x5c00
132 131 -922 802 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 131 -920 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
134 131 -918 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
122 132 -942 804 79 0 1 [97, 119, 97] 17 0 0x8f00 0x3100
123 132 -940 804 76 0 1 [97, 119, 97] 15 0 0x8c00 0x3100
124 132 -938 804 76 0 1 [97, 119, 97] 19 0 0x8c00 0x3100
125 132 -936 804 76 0 1 [97, 119, 97] 20 0 0x8c00 0x3100
126 132 -934 804 62 1 1 [63, 118, 228] 59 0 0x7e00 0xd800
127 132 -932 804 62 1 1 [63, 118, 228] 59 0 0x7e00 0xd800
128 132 -930 804 62 1 1 [63, 118, 228] 58 0 0x7e00 0xd800
129 132 -928 804 62 1 1 [63, 118, 228] 284 0 0x7e00 0xd800
130 132 -926 804 65 0 1 [134, 183, 131] 393 0 0x8100 0x5c00
131 132 -924 804 77 0 1 [97, 119, 97] 344 0 0x8d00 0x3100
132 132 -922 804 79 0 1 [97, 119, 97] 244 0 0x8f00 0x3100
133 132 -920 804 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
134 132 -918 804 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x3100
122 133 -942 806 78 0 1 [97, 119, 97] 17 0 0x8e00 0x3100
123 133 -940 806 69 0 1 [134, 183, 131] 61 0 0x8500 0x5c00
124 133 -938 806 155 0 1 [134, 183, 131] 61 0 0xdb00 0x5c00
125 133 -936 806 155 0 1 [134, 183, 131] 61 0 0xdb00 0x5c00
126 133 -934 806 155 0 1 [134, 183, 131] 60 0 0xdb00 0x5c00
127 133 -932 806 155 0 1 [134, 183, 131] 60 0 0xdb00 0x5c00
128 133 -930 806 155 0 1 [134, 183, 131] 59 0 0xdb00 0x5c00
129 133 -928 806 155 0 1 [134, 183, 131] 58 0 0xdb00 0x5c00
130 133 -926 806 155 0 1 [134, 183, 131] 58 0 0xdb00 0x5c00
131 133 -924 806 67 0 1 [134, 183, 131] 393 0 0x8300 0x5c00
132 133 -922 806 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 133 -920 806 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
134 133 -918 806 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x3100
122 134 -942 808 79 0 1 [97, 119, 97] 17 0 0x8f00 0x3100
123 134 -940 808 68 0 1 [134, 183, 131] 61 0 0x8400 0x5c00
124 134 -938 808 155 0 1 [134, 183, 131] 61 0 0xdb00 0x5c00
125 134 -936 808 180 0 1 [134, 183, 131] 58 0 0xf400 0x5c00
126 134 -934 808 180 0 1 [134, 183, 131] 58 0 0xf400 0x5c00
127 134 -932 808 180 0 1 [134, 183, 131] 284 0 0xf400 0x5c00
128 134 -930 808 180 0 1 [134, 183, 131] 393 0 0xf400 0x5c00
129 134 -928 808 180 0 1 [134, 183, 131] 393 0 0xf400 0x5c00
130 134 -926 808 155 0 1 [134, 183, 131] 58 0 0xdb00 0x5c00
131 134 -924 808 67 0 1 [134, 183, 131] 393 0 0x8300 0x5c00
132 134 -922 808 68 0 1 [134, 183, 131] 18446744073709551615 1 0x8400 0x5c00
133 134 -920 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
134 134 -918 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
```
### stream diag after fill:
[c-LoD] stream: tick 3900 65536/65536 cells, lap 772, cone 23875 zone 1569 dirty 919 (q 12354) ewin 218 rbeat 0p/0 rearm 0c/0t rescan 980381c/1484e, edits +5878p/4443c/587e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 303 forced 209, native 65536c/1090ch fail 0 (3w q 0) helper on, trees 1090ch/fail 0, wire bundle 659081p/2147b fail 0 ~65 kB/s cache 13574/24576 h645507 m13574 e0, srbeat 56284s/0q drop 41stale/0undec, ereb 48054s/69q, sweep 0r/86d
[15:46:30 INFO]: [c-LoD] stream: tick 3900 65536/65536 cells, lap 772, cone 23875 zone 1569 dirty 919 (q 12354) ewin 218 rbeat 0p/0 rearm 0c/0t rescan 980381c/1484e, edits +5878p/4443c/587e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 303 forced 209, native 65536c/1090ch fail 0 (3w q 0) helper on, trees 1090ch/fail 0, wire bundle 659081p/2147b fail 0 ~65 kB/s cache 13574/24576 h645507 m13574 e0, srbeat 56284s/0q drop 41stale/0undec, ereb 48054s/69q, sweep 0r/86d
- carrier ladder y=155: 6032 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=165: 6032 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=168: 6032 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=172: 6032 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=176: 6032 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=180: 6032 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=185: 6032 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- probe 0: red=0 paint=4496 lod_silhouette_cols=25 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=4 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 1: red=0 paint=4496 lod_silhouette_cols=25 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=4 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 2: red=0 paint=4565 lod_silhouette_cols=25 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=4 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 3: red=0 paint=4602 lod_silhouette_cols=25 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=4 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 4: red=0 paint=4666 lod_silhouette_cols=25 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=4 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 5: red=0 paint=4508 lod_silhouette_cols=25 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=4 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 6: red=0 paint=4528 lod_silhouette_cols=25 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=4 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 7: red=0 paint=4543 lod_silhouette_cols=25 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=4 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 8: red=0 paint=4579 lod_silhouette_cols=25 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=4 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 9: red=0 paint=4501 lod_silhouette_cols=25 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=4 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 10: red=0 paint=4499 lod_silhouette_cols=25 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=4 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 11: red=0 paint=4499 lod_silhouette_cols=25 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=4 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 12: red=0 paint=4499 lod_silhouette_cols=25 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=4 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 13: red=0 paint=4499 lod_silhouette_cols=25 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=4 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 14: red=0 paint=4561 lod_silhouette_cols=25 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=4 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 15: red=0 paint=4599 lod_silhouette_cols=25 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=4 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 16: red=0 paint=4691 lod_silhouette_cols=25 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=4 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 17: red=0 paint=9864 lod_silhouette_cols=24 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=4 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 18: red=0 paint=9860 lod_silhouette_cols=24 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=4 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 19: red=0 paint=9879 lod_silhouette_cols=23 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=4 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 20: red=0 paint=9880 lod_silhouette_cols=23 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=4 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 21: red=0 paint=9920 lod_silhouette_cols=23 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=4 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 22: red=0 paint=9976 lod_silhouette_cols=23 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=4 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 23: red=0 paint=9855 lod_silhouette_cols=24 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=4 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 24: red=0 paint=9871 lod_silhouette_cols=24 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=4 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 25: red=0 paint=9862 lod_silhouette_cols=23 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=4 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 26: red=0 paint=9917 lod_silhouette_cols=23 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=4 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 27: red=0 paint=9935 lod_silhouette_cols=23 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=4 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 28: red=0 paint=9868 lod_silhouette_cols=24 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=4 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 29: red=0 paint=9872 lod_silhouette_cols=23 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=4 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 30: red=0 paint=9889 lod_silhouette_cols=23 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=4 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 31: red=0 paint=9899 lod_silhouette_cols=23 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=4 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 32: red=0 paint=9911 lod_silhouette_cols=23 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=4 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 33: red=0 paint=9955 lod_silhouette_cols=23 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=4 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 34: red=0 paint=9850 lod_silhouette_cols=24 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=4 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 35: red=0 paint=9884 lod_silhouette_cols=23 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=4 mask_hits=30 (need red>=60 paint>=300 lod>20)
- phase: 5 leg-1 probes done (15:52:21)
- **FAIL:** leg-1 never saw the marker in 24 probes (red < 60 everywhere) — the edit never reached the LOD paint; this failure was SILENT before v6.31.1
### stream diag after probes:
[15:52:10 INFO]: [c-LoD] stream: tick 7300 65536/65536 cells, lap 979, cone 50961 zone 3349 dirty 970 (q 12373) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 2078953c/1550e, edits +7434p/5541c/933e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 303 forced 209, native 65536c/1090ch fail 0 (3w q 0) helper on, trees 1090ch/fail 0, wire bundle 980821p/2837b fail 0 ~66 kB/s cache 13666/24576 h967155 m13666 e0, srbeat 67801s/0q drop 47stale/0undec, ereb 48949s/0q, sweep 0r/40d
[c-LoD] stream: tick 7400 65536/65536 cells, lap 987, cone 52113 zone 3514 dirty 970 (q 12373) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 2109405c/1550e, edits +7434p/5541c/933e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 303 forced 209, native 65536c/1090ch fail 0 (3w q 0) helper on, trees 1090ch/fail 0, wire bundle 995825p/2866b fail 0 ~79 kB/s cache 13666/24576 h982159 m13666 e0, srbeat 68369s/0q drop 47stale/0undec, ereb 48949s/0q, sweep 0r/49d
[15:52:21 INFO]: [c-LoD] stream: tick 7400 65536/65536 cells, lap 987, cone 52113 zone 3514 dirty 970 (q 12373) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 2109405c/1550e, edits +7434p/5541c/933e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 303 forced 209, native 65536c/1090ch fail 0 (3w q 0) helper on, trees 1090ch/fail 0, wire bundle 995825p/2866b fail 0 ~79 kB/s cache 13666/24576 h982159 m13666 e0, srbeat 68369s/0q drop 47stale/0undec, ereb 48949s/0q, sweep 0r/49d
- parity debug tour: 13 vantages (session-1 client, DEBUG pack — evidence only, the gate pairs the §7b world session)
- parity debug tour done: 13 shots in parity_dbg/ (the §7b world session feeds the gate)
- joined session 2 (15:55:12)
- leg-2 baseline scene px (x<940): 0 (0 = the relog client paints no world at all)
- leg-2 renderer probe NEAR (20 m) white px: 0 raw / 0 changed-vs-baseline (> 20 = the client renders particles)
- leg-2 renderer probe FAR (80 m, marker zone) white px: 506 raw / 506 changed-vs-baseline (> 20 = no distance cut; ~0 with NEAR alive = particles beyond the client chunk radius do not render)
- leg-2 carrier probe green dust (80 m): changed-green px 0 / total changed 335 vs baseline (> 100 green = a stable green carrier exists; total > 0 with green ~0 = the dust renders but fog washes it — a ruler problem, not a carrier problem)
- phase: 8 leg-2 relog probes (15:58:02)
- LEG-2 SCENE DEAD: the relog client paints ZERO scene pixels (x<940) — pure sky + lens panels; the fresh-session capture chain never commits (renderer-dead class, #227/#228); these probes measure a worldless client, not the transport
- relog probe 0: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=0 mask_hits=0 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 1: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=0 mask_hits=0 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 2: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=0 mask_hits=0 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 3: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=0 mask_hits=0 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 4: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=0 mask_hits=0 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 5: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=0 mask_hits=0 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 6: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=0 mask_hits=0 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 7: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=0 mask_hits=0 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 8: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=0 mask_hits=0 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 9: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=0 mask_hits=0 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 10: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=0 mask_hits=0 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 11: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=0 mask_hits=0 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 12: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=0 mask_hits=0 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 13: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=0 mask_hits=0 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 14: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=0 mask_hits=0 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 15: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=0 mask_hits=0 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 16: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=0 mask_hits=0 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 17: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=0 mask_hits=0 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 18: red=0 scene=1079 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=1 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 19: red=0 scene=1057 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=1 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- **FAIL:** marker lost after relog — stream state is not persistent/deterministic
- phase: 9 leg-2 done (16:00:59)

### client log forensics (pipeline/particle/atlas — v6.8.2)
```
[15:55:01] [main/DEBUG]: Loading LWJGL system
WARNING: java.lang.System::load has been called by org.lwjgl.system.Library$$Lambda/0x000000004c1e3d60 in an unnamed module (file:/home/runner/mc-test/client/libraries/org/lwjgl/lwjgl/3.4.3/lwjgl-3.4.3.jar)
WARNING: sun.misc.Unsafe::objectFieldOffset has been called by org.lwjgl.system.MemoryBackendUnsafeLegacy (file:/home/runner/mc-test/client/libraries/org/lwjgl/lwjgl/3.4.3/lwjgl-3.4.3.jar)
WARNING: Please consider reporting this to the maintainers of class org.lwjgl.system.MemoryBackendUnsafeLegacy
[15:55:01] [main/DEBUG]: Loading shaderc
[15:55:01] [main/DEBUG]: Loading OpenGL
[15:55:06] [Render thread/ERROR]: Error parsing option value 4 for option Simulation Distance: Value 4 outside of range [5:33]
[15:55:06] [Render thread/INFO]: Backend library: LWJGL version 3.4.3+4
[15:55:06] [Download-2/ERROR]: Failed to fetch user properties
com.mojang.authlib.exceptions.InvalidCredentialsException: Status: 401
	at com.mojang.authlib.exceptions.MinecraftClientHttpException.toAuthenticationException(MinecraftClientHttpException.java:70)
Caused by: MinecraftClientHttpException[type=HTTP_ERROR, status=401, response=ErrorResponse[path=/player/attributes, error=null, errorMessage=null, details=null]]
	at com.mojang.authlib.minecraft.client.MinecraftClient.buildHttpException(MinecraftClient.java:170)
[15:55:06] [Render thread/INFO]: Using graphics backend OpenGL, using drivers: 4.5 (Core Profile) Mesa 26.0.8-1ubuntu0.3
[15:55:06] [Render thread/INFO]: Using graphics device: llvmpipe (LLVM 21.1.8, 256 bits) (Mesa)
[15:55:06] [Render thread/INFO]: Using graphics device extensions: GL_ARB_multi_draw_indirect, GL_ARB_buffer_storage, GL_ARB_shader_draw_parameters, GL_ARB_base_instance, GL_KHR_debug, GL_ARB_draw_indirect, GL_ARB_clip_control, GL_ARB_vertex_attrib_binding, GL_ARB_direct_state_access, GL_EXT_texture_filter_anisotropic
[15:55:06] [Download-1/INFO]: Could not authorize you against Realms server: java.lang.RuntimeException: Failed to parse into SignedJWT: 0
[15:55:06] [Download-1/ERROR]: Failed to fetch Realms feature flags
com.mojang.realmsclient.exception.RealmsServiceException: Realms authentication error with message 'java.lang.RuntimeException: Failed to parse into SignedJWT: 0'
[15:55:06] [Render thread/INFO]: Reloading ResourceManager: vanilla, vanilla
[15:55:09] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[15:55:09] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[15:55:09] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[15:55:09] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[15:55:09] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[15:55:09] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[15:55:09] [Render thread/INFO]: Created: 2048x2048x4 minecraft:textures/atlas/blocks.png-atlas
[15:55:09] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[15:55:09] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[15:55:09] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[15:55:09] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[15:55:09] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
[15:55:16] [Render thread/INFO]: Reloading ResourceManager: vanilla, vanilla, server/00000000/6b1f2f7e-9c0d-4a4b-8b0a-b10d1b4ab100
[15:55:17] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[15:55:17] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[15:55:17] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[15:55:17] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[15:55:17] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[15:55:17] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[15:55:17] [Render thread/INFO]: Created: 2048x2048x4 minecraft:textures/atlas/blocks.png-atlas
[15:55:18] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[15:55:18] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[15:55:18] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[15:55:18] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[15:55:18] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
```

### /clod-cells AFTER PROBES (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 8 0 0x8400 0x3100
123 122 -940 784 71 0 1 [97, 119, 97] 12 0 0x8700 0x3100
124 122 -938 784 68 0 1 [97, 119, 97] 12 0 0x8400 0x3100
125 122 -936 784 62 1 1 [63, 118, 228] 51 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 51 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 50 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 50 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 49 0 0x8000 0x5c00
130 122 -926 784 66 0 1 [134, 183, 131] 49 0 0x8200 0x5c00
131 122 -924 784 75 0 1 [97, 119, 97] 120 0 0x8b00 0x3100
132 122 -922 784 77 0 1 [97, 119, 97] 4220 0 0x8d00 0x3100
133 122 -920 784 75 0 1 [97, 119, 97] 4220 0 0x8b00 0x3100
134 122 -918 784 73 0 1 [97, 119, 97] 4220 0 0x8900 0x3100
122 123 -942 786 66 0 1 [97, 119, 97] 8 0 0x8200 0x3100
123 123 -940 786 65 0 1 [97, 119, 97] 12 0 0x8100 0x3100
124 123 -938 786 65 0 1 [97, 119, 97] 12 0 0x8100 0x3100
125 123 -936 786 62 1 1 [63, 118, 228] 51 0 0x7e00 0xd800
126 123 -934 786 62 1 1 [63, 118, 228] 51 0 0x7e00 0xd800
127 123 -932 786 62 1 1 [63, 118, 228] 50 0 0x7e00 0xd800
128 123 -930 786 62 1 1 [63, 118, 228] 50 0 0x7e00 0xd800
129 123 -928 786 73 0 1 [97, 119, 97] 7 0 0x8900 0x3100
130 123 -926 786 67 0 1 [134, 183, 131] 49 0 0x8300 0x5c00
131 123 -924 786 69 0 1 [134, 183, 131] 160 0 0x8500 0x5c00
132 123 -922 786 77 0 1 [97, 119, 97] 4220 0 0x8d00 0x3100
133 123 -920 786 77 0 1 [97, 119, 97] 4220 0 0x8d00 0x3100
134 123 -918 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
122 124 -942 788 66 0 1 [97, 119, 97] 8 0 0x8200 0x3100
123 124 -940 788 64 0 1 [134, 183, 131] 52 0 0x8000 0x5c00
124 124 -938 788 62 1 1 [63, 118, 228] 52 0 0x7e00 0xd800
125 124 -936 788 62 1 1 [63, 118, 228] 51 0 0x7e00 0xd800
126 124 -934 788 62 1 1 [63, 118, 228] 51 0 0x7e00 0xd800
127 124 -932 788 62 1 1 [63, 118, 228] 50 0 0x7e00 0xd800
128 124 -930 788 73 0 1 [97, 119, 97] 10 0 0x8900 0x3100
129 124 -928 788 76 0 1 [97, 119, 97] 7 0 0x8c00 0x3100
130 124 -926 788 73 0 1 [97, 119, 97] 6 0 0x8900 0x3100
131 124 -924 788 69 0 1 [134, 183, 131] 160 0 0x8500 0x5c00
132 124 -922 788 77 0 1 [97, 119, 97] 4220 0 0x8d00 0x3100
133 124 -920 788 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
134 124 -918 788 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x3100
122 125 -942 790 65 0 1 [134, 183, 131] 53 0 0x8100 0x5c00
123 125 -940 790 65 0 1 [134, 183, 131] 52 0 0x8100 0x5c00
124 125 -938 790 62 1 1 [63, 118, 228] 52 0 0x7e00 0xd800
125 125 -936 790 62 1 1 [63, 118, 228] 51 0 0x7e00 0xd800
126 125 -934 790 62 1 1 [63, 118, 228] 51 0 0x7e00 0xd800
127 125 -932 790 62 1 1 [63, 118, 228] 50 0 0x7e00 0xd800
128 125 -930 790 62 1 1 [63, 118, 228] 50 0 0x7e00 0xd800
129 125 -928 790 73 0 1 [97, 119, 97] 7 0 0x8900 0x3100
130 125 -926 790 68 0 1 [134, 183, 131] 49 0 0x8400 0x5c00
131 125 -924 790 69 0 1 [134, 183, 131] 160 0 0x8500 0x5c00
132 125 -922 790 69 0 1 [134, 183, 131] 4219 0 0x8500 0x5c00
133 125 -920 790 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
134 125 -918 790 70 0 1 [134, 183, 131] 18446744073709551615 1 0x8600 0x5c00
122 126 -942 792 70 0 1 [97, 119, 97] 9 0 0x8600 0x3100
123 126 -940 792 66 0 1 [134, 183, 131] 52 0 0x8200 0x5c00
124 126 -938 792 63 0 1 [134, 183, 131] 52 0 0x7f00 0x5c00
125 126 -936 792 62 1 1 [63, 118, 228] 51 0 0x7e00 0xd800
126 126 -934 792 62 1 1 [63, 118, 228] 51 0 0x7e00 0xd800
127 126 -932 792 62 1 1 [63, 118, 228] 50 0 0x7e00 0xd800
128 126 -930 792 62 1 1 [63, 118, 228] 50 0 0x7e00 0xd800
129 126 -928 792 67 0 1 [134, 183, 131] 49 0 0x8300 0x5c00
130 126 -926 792 74 0 1 [97, 119, 97] 106 0 0x8a00 0x3100
131 126 -924 792 77 0 1 [97, 119, 97] 106 0 0x8d00 0x3100
132 126 -922 792 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x3100
133 126 -920 792 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3800
134 126 -918 792 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x8800
122 127 -942 794 70 0 1 [97, 119, 97] 9 0 0x8600 0x3100
123 127 -940 794 68 0 1 [134, 183, 131] 52 0 0x8400 0x5c00
124 127 -938 794 65 0 1 [134, 183, 131] 52 0 0x8100 0x5c00
125 127 -936 794 62 1 1 [63, 118, 228] 51 0 0x7e00 0xd800
126 127 -934 794 62 1 1 [63, 118, 228] 51 0 0x7e00 0xd800
127 127 -932 794 170 0 1 [142, 33, 33] 106 0 0xea00 0x0200
128 127 -930 794 170 0 1 [142, 33, 33] 106 0 0xea00 0x0200
129 127 -928 794 170 0 1 [142, 33, 33] 106 0 0xea00 0x0200
130 127 -926 794 74 0 1 [97, 119, 97] 106 0 0x8a00 0x3100
131 127 -924 794 77 0 1 [97, 119, 97] 105 0 0x8d00 0x3100
132 127 -922 794 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x3100
133 127 -920 794 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3800
134 127 -918 794 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x8800
122 128 -942 796 78 0 1 [97, 119, 97] 108 0 0x8e00 0x3100
123 128 -940 796 75 0 1 [97, 119, 97] 107 0 0x8b00 0x3100
124 128 -938 796 75 0 1 [97, 119, 97] 106 0 0x8b00 0x3100
125 128 -936 796 72 0 1 [97, 119, 97] 106 0 0x8800 0x3100
126 128 -934 796 62 1 1 [63, 118, 228] 51 0 0x7e00 0xd800
127 128 -932 796 170 0 1 [142, 33, 33] 106 0 0xea00 0x0200
128 128 -930 796 170 0 1 [142, 33, 33] 105 0 0xea00 0x0200
129 128 -928 796 170 0 1 [142, 33, 33] 105 0 0xea00 0x0200
130 128 -926 796 68 0 1 [134, 183, 131] 49 0 0x8400 0x5c00
131 128 -924 796 74 0 1 [97, 119, 97] 105 0 0x8a00 0x3100
132 128 -922 796 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 128 -920 796 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x3100
134 128 -918 796 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
122 129 -942 798 78 0 1 [97, 119, 97] 108 0 0x8e00 0x3100
123 129 -940 798 75 0 1 [97, 119, 97] 107 0 0x8b00 0x3100
124 129 -938 798 78 0 1 [97, 119, 97] 14 0 0x8e00 0x3100
125 129 -936 798 75 0 1 [97, 119, 97] 106 0 0x8b00 0x3100
126 129 -934 798 62 1 1 [63, 118, 228] 51 0 0x7e00 0xd800
127 129 -932 798 170 0 1 [142, 33, 33] 106 0 0xea00 0x0200
128 129 -930 798 170 0 1 [142, 33, 33] 105 0 0xea00 0x0200
129 129 -928 798 170 0 1 [142, 33, 33] 105 0 0xea00 0x0200
130 129 -926 798 71 0 1 [97, 119, 97] 105 0 0x8700 0x3100
131 129 -924 798 69 0 1 [134, 183, 131] 159 0 0x8500 0x5c00
132 129 -922 798 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x3100
133 129 -920 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
134 129 -918 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
122 130 -942 800 70 0 1 [134, 183, 131] 53 0 0x8600 0x5c00
123 130 -940 800 76 0 1 [97, 119, 97] 107 0 0x8c00 0x3100
124 130 -938 800 76 0 1 [97, 119, 97] 106 0 0x8c00 0x3100
125 130 -936 800 72 0 1 [97, 119, 97] 106 0 0x8800 0x3100
126 130 -934 800 62 1 1 [63, 118, 228] 51 0 0x7e00 0xd800
127 130 -932 800 62 1 1 [63, 118, 228] 50 0 0x7e00 0xd800
128 130 -930 800 62 1 1 [63, 118, 228] 50 0 0x7e00 0xd800
129 130 -928 800 71 0 1 [97, 119, 97] 105 0 0x8700 0x3100
130 130 -926 800 71 0 1 [97, 119, 97] 105 0 0x8700 0x3100
131 130 -924 800 69 0 1 [134, 183, 131] 159 0 0x8500 0x5c00
132 130 -922 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
133 130 -920 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
134 130 -918 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
122 131 -942 802 78 0 1 [97, 119, 97] 107 0 0x8e00 0x3100
123 131 -940 802 77 0 1 [97, 119, 97] 107 0 0x8d00 0x3100
124 131 -938 802 77 0 1 [97, 119, 97] 106 0 0x8d00 0x3100
125 131 -936 802 66 0 1 [134, 183, 131] 51 0 0x8200 0x5c00
126 131 -934 802 62 1 1 [63, 118, 228] 51 0 0x7e00 0xd800
127 131 -932 802 62 1 1 [63, 118, 228] 50 0 0x7e00 0xd800
128 131 -930 802 62 1 1 [63, 118, 228] 50 0 0x7e00 0xd800
129 131 -928 802 62 1 1 [63, 118, 228] 49 0 0x7e00 0xd800
130 131 -926 802 66 0 1 [134, 183, 131] 49 0 0x8200 0x5c00
131 131 -924 802 68 0 1 [134, 183, 131] 159 0 0x8400 0x5c00
132 131 -922 802 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 131 -920 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
134 131 -918 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
122 132 -942 804 79 0 1 [97, 119, 97] 107 0 0x8f00 0x3100
123 132 -940 804 76 0 1 [97, 119, 97] 107 0 0x8c00 0x3100
124 132 -938 804 76 0 1 [97, 119, 97] 106 0 0x8c00 0x3100
125 132 -936 804 76 0 1 [97, 119, 97] 106 0 0x8c00 0x3100
126 132 -934 804 62 1 1 [63, 118, 228] 51 0 0x7e00 0xd800
127 132 -932 804 62 1 1 [63, 118, 228] 50 0 0x7e00 0xd800
128 132 -930 804 62 1 1 [63, 118, 228] 50 0 0x7e00 0xd800
129 132 -928 804 62 1 1 [63, 118, 228] 49 0 0x7e00 0xd800
130 132 -926 804 65 0 1 [134, 183, 131] 49 0 0x8100 0x5c00
131 132 -924 804 77 0 1 [97, 119, 97] 105 0 0x8d00 0x3100
132 132 -922 804 79 0 1 [97, 119, 97] 105 0 0x8f00 0x3100
133 132 -920 804 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
134 132 -918 804 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x3100
122 133 -942 806 78 0 1 [97, 119, 97] 107 0 0x8e00 0x3100
123 133 -940 806 69 0 1 [134, 183, 131] 52 0 0x8500 0x5c00
124 133 -938 806 155 0 1 [134, 183, 131] 53 0 0xdb00 0x5c00
125 133 -936 806 155 0 1 [134, 183, 131] 52 0 0xdb00 0x5c00
126 133 -934 806 155 0 1 [134, 183, 131] 52 0 0xdb00 0x5c00
127 133 -932 806 155 0 1 [134, 183, 131] 51 0 0xdb00 0x5c00
128 133 -930 806 155 0 1 [134, 183, 131] 51 0 0xdb00 0x5c00
129 133 -928 806 155 0 1 [134, 183, 131] 50 0 0xdb00 0x5c00
130 133 -926 806 155 0 1 [134, 183, 131] 50 0 0xdb00 0x5c00
131 133 -924 806 67 0 1 [134, 183, 131] 159 0 0x8300 0x5c00
132 133 -922 806 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 133 -920 806 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
134 133 -918 806 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x3100
122 134 -942 808 79 0 1 [97, 119, 97] 107 0 0x8f00 0x3100
123 134 -940 808 68 0 1 [134, 183, 131] 52 0 0x8400 0x5c00
124 134 -938 808 155 0 1 [134, 183, 131] 53 0 0xdb00 0x5c00
125 134 -936 808 180 0 1 [134, 183, 131] 50 0 0xf400 0x5c00
126 134 -934 808 180 0 1 [134, 183, 131] 49 0 0xf400 0x5c00
127 134 -932 808 180 0 1 [134, 183, 131] 49 0 0xf400 0x5c00
128 134 -930 808 180 0 1 [134, 183, 131] 159 0 0xf400 0x5c00
129 134 -928 808 180 0 1 [134, 183, 131] 159 0 0xf400 0x5c00
130 134 -926 808 155 0 1 [134, 183, 131] 50 0 0xdb00 0x5c00
131 134 -924 808 67 0 1 [134, 183, 131] 159 0 0x8300 0x5c00
132 134 -922 808 68 0 1 [134, 183, 131] 18446744073709551615 1 0x8400 0x5c00
133 134 -920 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
134 134 -918 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
```
- edit channel: 0 linkage errors, 0 failed event dispatches (both must be 0)
- phase: 6 parity world boot (16:00:59)
- parity world: view-distance=8 (the voxy leg's server value)
- parity world: joined session 3 (16:01:36)
- parity world: scan populated (cells=65536; the march paints from these buffers)
- parity world tour: 13 vantages (production pack, vd 8, shared structures)
- coverage sweeps: 72 eye passes = 36 positions (12 az x r144/200 + 12 off-az x r172, y=160) x 2 looks (outward+inward), settle-by-laps >= 6 s (q stable 3x1s), deadline 12 s/pos, no shots — the v6.26.0 data-coverage lever + v6.26.6 lottery kill; probe rides parity/view_probe.md
- parity drain-settle TIMEOUT: queue 22435 -> 22436 > 11500 after 150s — continuing anyway; the queue did not drain without eye proximity (priority-lane evidence, the 01/05/10 dip attribution reads from the tour deltas)
- pre-tour pass done: 13 vantages x 10s (the tour now reads the second OR-merge lap; the rim/dip transport race reads from the tour deltas)
- parity world tour done: 13 shots, 0 night-guard renames in parity/ (the gate pairs voxy/<name>.png vs parity/<name>.png)
- phase: 7 parity tour done (16:23:33)
- sparse re-beat channel at tour end: streamed=96952 q=0 dropped_stale=91 dropped_undecodable=15700 (frac=8 boot; undecodable-dominant = the rim words die in their decodability window, stale-dominant = edit churn owns the cells)
### parity world stream counters (session 3, tail + maxima)
[c-LoD] stream: tick 13200 65536/65536 cells, lap 1262, cone 73237 zone 25322 dirty 15077 (q 22412) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 9029384c/1945e, edits +18031p/13675c/2819e drop 0, spawned 0 / failed 0, cam0 yaw -180 pitch 5, wish 0 forced 863, native 39360c/952ch fail 0 (3w q 0) helper on, trees 952ch/fail 0, wire bundle 1651646p/4724b fail 0 ~0 kB/s cache 22582/24576 h1199498 m452148 e429566, srbeat 96952s/0q drop 91stale/15700undec, ereb 67623s/0q, sweep 0r/18d
[16:23:29 INFO]: [c-LoD] stream: tick 13200 65536/65536 cells, lap 1262, cone 73237 zone 25322 dirty 15077 (q 22412) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 9029384c/1945e, edits +18031p/13675c/2819e drop 0, spawned 0 / failed 0, cam0 yaw -180 pitch 5, wish 0 forced 863, native 39360c/952ch fail 0 (3w q 0) helper on, trees 952ch/fail 0, wire bundle 1651646p/4724b fail 0 ~0 kB/s cache 22582/24576 h1199498 m452148 e429566, srbeat 96952s/0q drop 91stale/15700undec, ereb 67623s/0q, sweep 0r/18d
- parity stream max cone: 73237
- parity stream max zone: 25322
- parity stream max wire bundle: bundle
### parity world capture lens (debug chain on the §7b world)
lens_red=0 lens_black=0 lens_water=0 lens_dark=0 lens_zero=0 rot=none scan=0 carriers=0
- parity mesh probe @png_y=2: x306=(151,184,255) x307=(151,184,255) x308=(151,184,255) x309h=(151,184,255) x310c=(151,184,255)
- parity mesh probe @png_y=717: x306=(44,62,51) x307=(47,67,54) x308=(43,61,50) x309h=(41,58,48) x310c=(49,70,56)
- parity mesh engagement: magenta-ish=0 of 921600 sampled px
- mask8: the un-fog engagement masks shot on the PARITY WORLD (vd 8) — parity_dbg8/01m|07m|10m
- mask8 01m: green=0 red=0 blue=94176 dim=4096 (px, sampled /16)
- mask8 07m: green=11936 red=0 blue=221264 dim=5744 (px, sampled /16)
- mask8 10m: green=6736 red=80 blue=228448 dim=173504 (px, sampled /16)
- mesh probe @png_y=2: x306=(255,255,255) x307=(13,254,0) x308=(0,254,0) x309h=(12,0,0) x310c=(0,0,0)
- mesh probe @png_y=717: x306=(255,255,255) x307=(13,254,0) x308=(0,254,0) x309h=(12,0,0) x310c=(0,0,0)
- mesh engagement: magenta-ish=0 of 921600 sampled px (the vanilla-path paint coverage)
- mask8 fog state @png_y=2: raw (300)=(151,184,255) (301)=(151,184,255)
- mask8 fog state @png_y=717: raw (300)=(0,128,8) (301)=(179,206,255)
- mask8 fog state decoded: envStart=0 envEnd=1024 rdEnd=128 fogColor=(179,206,255)
- buffer dump 10m: marked h=29/2048 c=3/2048; height windows r0<142:8 r1:0 r2>=202:21 maxByte=255
- buffer dump 10m: RANK-2-WINDOW HEIGHTS (the fake-wall class): (+70,-26)h=174, (+82,-24)h=191, (+84,-24)h=189, (+82,-22)h=191, (+88,-22)h=191, (-2,-18)h=175, (+4,-18)h=187, (+0,-16)h=182, (+78,-2)h=191, (+86,+0)h=191
- rim mask: scan populated (cells=65536; the state paint reads the same buffers the march reads)
- phase: 10 rim mask lane (16:29:31)
- rim mask: decode-state frames shot (rim_mask/00|01|12 — GREEN full decode / RED colorless / BLUE heightless; 00 = the leg-1 tower eye, 01 = the near-tower contrast, 12 = the tower re-read; the inherited 07/12 vantages sat 760 blocks outside the streamed disk and 01 sat INSIDE the marker column — their earlier verdicts were classifier noise, purged in v6.33.1)
- rim mask 00: green=7680 red=6928 blue=211360 yellow=6960 (px, sampled /16) -> DOMINANT BLUE: rim = HEIGHTS NEVER DECODED — a data-coverage class: the state paint cannot even see the cells
- rim mask 01: green=33328 red=16128 blue=214832 yellow=10576 (px, sampled /16) -> DOMINANT BLUE: rim = HEIGHTS NEVER DECODED — a data-coverage class: the state paint cannot even see the cells
- rim mask 12: green=12064 red=11792 blue=244800 yellow=7008 (px, sampled /16) -> DOMINANT BLUE: rim = HEIGHTS NEVER DECODED — a data-coverage class: the state paint cannot even see the cells

### boot / stream
```
[c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 135240 bytes, sha1 43c1952bf2b927859caa54eee66364ea0a5e48e8 — zero textures, zero world data, one push per join
[c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[16:27:07 INFO]: [c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 135240 bytes, sha1 43c1952bf2b927859caa54eee66364ea0a5e48e8 — zero textures, zero world data, one push per join
[16:27:07 INFO]: [c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[c-LoD] stream: tick 1900 65536/65536 cells, lap 692, cone 23395 zone 1328 dirty 19 (q 1268) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1257016c/6e, edits +17p/17c/8e drop 0, spawned 0 / failed 0, cam0 none, wish 128 forced 750, native 6522c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 266514p/636b fail 0 ~36 kB/s cache 14005/24576 h252509 m14005 e0, srbeat 9711s/309q drop 0stale/453undec, ereb 1103s/4q, sweep 0r/28d
[16:30:27 INFO]: [c-LoD] stream: tick 1900 65536/65536 cells, lap 692, cone 23395 zone 1328 dirty 19 (q 1268) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1257016c/6e, edits +17p/17c/8e drop 0, spawned 0 / failed 0, cam0 none, wish 128 forced 750, native 6522c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 266514p/636b fail 0 ~36 kB/s cache 14005/24576 h252509 m14005 e0, srbeat 9711s/309q drop 0stale/453undec, ereb 1103s/4q, sweep 0r/28d
```
### sessions
```
[16:27:28 INFO]: System chat: Stepan joined the game
[16:30:26 INFO]: System chat: Stepan left the game
```
### client log tail
```
[16:30:24] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[16:30:24] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[16:30:24] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[16:30:24] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[16:30:24] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[16:30:24] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[16:30:24] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[16:30:24] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[16:30:24] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[16:30:24] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[16:30:24] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:keep_alive] net.minecraft.network.protocol.common.ClientboundKeepAlivePacket -> 9 bytes
[16:30:24] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:keep_alive] net.minecraft.network.protocol.common.ServerboundKeepAlivePacket -> 9 bytes
[16:30:24] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[16:30:24] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[16:30:24] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[16:30:24] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[16:30:24] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[16:30:24] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:bundle_delimiter] net.minecraft.network.protocol.game.ClientboundBundleDelimiterPacket -> 1 bytes
[16:30:24] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[16:30:24] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[16:30:24] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[16:30:24] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[16:30:24] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[16:30:24] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[16:30:24] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[16:30:24] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[16:30:24] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:bundle_delimiter] net.minecraft.network.protocol.game.ClientboundBundleDelimiterPacket -> 1 bytes
[16:30:24] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[16:30:24] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[16:30:24] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:set_time] net.minecraft.network.protocol.game.ClientboundSetTimePacket -> 10 bytes
[16:30:25] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[16:30:25] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[16:30:25] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[16:30:25] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[16:30:25] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[16:30:25] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[16:30:25] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[16:30:25] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[16:30:25] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[16:30:25] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[16:30:25] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[16:30:25] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:keep_alive] net.minecraft.network.protocol.common.ClientboundKeepAlivePacket -> 9 bytes
[16:30:25] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:keep_alive] net.minecraft.network.protocol.common.ServerboundKeepAlivePacket -> 9 bytes
[16:30:25] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[16:30:25] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[16:30:25] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[16:30:25] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[16:30:25] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[16:30:25] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[16:30:25] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[16:30:25] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[16:30:25] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:set_time] net.minecraft.network.protocol.game.ClientboundSetTimePacket -> 10 bytes
[16:30:25] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:move_player_pos] net.minecraft.network.protocol.game.ServerboundMovePlayerPacket$Pos -> 26 bytes
[16:30:25] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[16:30:25] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[16:30:25] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[16:30:25] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[16:30:25] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[16:30:25] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[16:30:25] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[16:30:25] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[16:30:25] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[16:30:25] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[16:30:26] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:bundle_delimiter] net.minecraft.network.protocol.game.ClientboundBundleDelimiterPacket -> 1 bytes
[16:30:26] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[16:30:26] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[16:30:26] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[16:30:26] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[16:30:26] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[16:30:26] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[16:30:26] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[16:30:26] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[16:30:26] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:bundle_delimiter] net.minecraft.network.protocol.game.ClientboundBundleDelimiterPacket -> 1 bytes
[16:30:26] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[16:30:26] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[16:30:26] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[16:30:26] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[16:30:26] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[16:30:26] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[16:30:26] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
--- exceptions (Realms/RTKit noise filtered) ---
25:[16:27:22] [Render thread/ERROR]: Error parsing option value 4 for option Simulation Distance: Value 4 outside of range [5:33]
42:[16:27:23] [Download-2/ERROR]: Failed to fetch user properties
43:com.mojang.authlib.exceptions.InvalidCredentialsException: Status: 401
44:	at com.mojang.authlib.exceptions.MinecraftClientHttpException.toAuthenticationException(MinecraftClientHttpException.java:70)
51:Caused by: MinecraftClientHttpException[type=HTTP_ERROR, status=401, response=ErrorResponse[path=/player/attributes, error=null, errorMessage=null, details=null]]
52:	at com.mojang.authlib.minecraft.client.MinecraftClient.buildHttpException(MinecraftClient.java:170)
```
### server log tail
```
[c-LoD] EB: enter, native_gen=true
[16:30:27 INFO]: [ChunkHolderManager] Saved 1004 block chunks, 975 entity chunks, 0 poi chunks in world 'minecraft:overworld' in 0.95s
[16:30:27 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:overworld'
[16:30:27 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:overworld'
[16:30:27 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[16:30:27 INFO]: [ChunkHolderManager] Waiting 60s for chunk system to halt for world 'minecraft:the_nether'
[16:30:27 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:the_nether'
[16:30:27 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_nether'
[16:30:27 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_nether' in 0.00s
[16:30:27 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_nether'
[16:30:27 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_nether'
[16:30:27 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[16:30:27 INFO]: [ChunkHolderManager] Waiting 60s for chunk system to halt for world 'minecraft:the_end'
[16:30:27 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:the_end'
[16:30:27 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_end'
[16:30:27 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_end' in 0.00s
[16:30:27 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_end'
[16:30:27 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_end'
[16:30:27 INFO]: Waiting for all RegionFile I/O tasks to complete...
[16:30:27 INFO]: All RegionFile I/O tasks to complete
[16:30:27 INFO]: [MoonriseCommon] Awaiting termination of worker pool for up to 60s...
[16:30:27 INFO]: [MoonriseCommon] Awaiting termination of I/O pool for up to 60s...
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[launcher] server exited with code 143
```
### shots
```
-rw-r--r-- 1 runner runner   2658 Nov 16 15:59 lens_13_after_relog_probe11.png
-rw-r--r-- 1 runner runner   2658 Nov 16 15:59 lens_13_after_relog_probe12.png
-rw-r--r-- 1 runner runner   2658 Nov 16 15:59 lens_13_after_relog_probe13.png
-rw-r--r-- 1 runner runner   2658 Nov 16 16:00 lens_13_after_relog_probe14.png
-rw-r--r-- 1 runner runner   2658 Nov 16 16:00 lens_13_after_relog_probe15.png
-rw-r--r-- 1 runner runner   2658 Nov 16 16:00 lens_13_after_relog_probe16.png
-rw-r--r-- 1 runner runner   2658 Nov 16 16:00 lens_13_after_relog_probe17.png
-rw-r--r-- 1 runner runner   3786 Nov 16 16:00 lens_13_after_relog_probe18.png
-rw-r--r-- 1 runner runner   3935 Nov 16 16:00 lens_13_after_relog_probe19.png
-rw-r--r-- 1 runner runner   2658 Nov 16 15:58 lens_13_after_relog_probe2.png
-rw-r--r-- 1 runner runner   2658 Nov 16 15:58 lens_13_after_relog_probe3.png
-rw-r--r-- 1 runner runner   2658 Nov 16 15:58 lens_13_after_relog_probe4.png
-rw-r--r-- 1 runner runner   2658 Nov 16 15:58 lens_13_after_relog_probe5.png
-rw-r--r-- 1 runner runner   2658 Nov 16 15:58 lens_13_after_relog_probe6.png
-rw-r--r-- 1 runner runner   2658 Nov 16 15:59 lens_13_after_relog_probe7.png
-rw-r--r-- 1 runner runner   2658 Nov 16 15:59 lens_13_after_relog_probe8.png
-rw-r--r-- 1 runner runner   2658 Nov 16 15:59 lens_13_after_relog_probe9.png
drwxr-xr-x 2 runner runner   4096 Nov 16 16:23 lens_parity
drwxr-xr-x 2 runner runner   4096 Nov 16 16:23 parity
drwxr-xr-x 2 runner runner   4096 Nov 16 15:54 parity_dbg
drwxr-xr-x 2 runner runner   4096 Nov 16 16:26 parity_dbg8
-rw-r--r-- 1 runner runner  12080 Nov 16 16:00 probe_counts.txt
drwxr-xr-x 2 runner runner   4096 Nov 16 16:30 rim_mask
-rw-r--r-- 1 runner runner 104713 Nov 16 16:30 server_full.log
-rw-r--r-- 1 runner runner  70508 Nov 16 16:30 summary.md
```

**exit=1** failures=2 leg1=0 leg2=0
