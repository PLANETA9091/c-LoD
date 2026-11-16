- phase: 0 sanity (16:04:10)
### c-LoD visual run (-930,796, marker=red_concrete, vantage -1005,150 yaw 270)
- GL:     Vendor: Mesa (0xffffffff);    Device: llvmpipe (LLVM 21.1.8, 256 bits) (0xffffffff);OpenGL renderer string: llvmpipe (LLVM 21.1.8, 256 bits);OpenGL core profile version string: 4.5 (Core Profile) Mesa 26.0.8-1ubuntu0.3;
- boot + gates OK (16:04:32)
- phase: 1 server booted (16:04:32)
- sky-grid armed
- phase: 2 leg-1 client joined (16:04:55)
- joined session 1 (16:04:55)
- stream live: cells=65536
- phase: 3 scan populated (16:05:18)
- terrain range: 2 terraced mountains at (CX-4,CZ-20)/(CX-2,CZ+16), tops 185/190, grass terraces + stone caps, 66..82 m from the eye (inside the 83.5 m band, chunk edge 64 m)
- scan complete=1 (cells=cells=65536/65536)
- paint settled=1 (wire 539038p)
- baseline try 1: drift=8 px
- lod_silhouette_cols=25 (LOD paint ON SCREEN at 75 m if > 20 — the march-drawn marker column; vanilla cannot render there, chunk edge 64 m; #255 re-measures 49, the ZONE_FAR=48 regression #256 and the control read 0)
- 1014 terrain-green px in the sky region (LOD MOUNTAINS on screen if > 5000; the range paints ColLow-green on low terraces via the fallback palette; flat-world baselines and the pack-free control read ~0)
- burst_close white px: 558 (renderer alive if > 50)
- burst_far white px: 553 (renderer alive in the marker zone if > 20)
- cut walk 40 m: 60 white px (> 20 = rendered)
- cut walk 60 m: 0 white px (> 20 = rendered)
- cut walk 100 m: 0 white px (> 20 = rendered)
- measured far-render cut: 40 m (G4 flap instrument; the decode proof needs >= 83 m from the y=151.6 eye)
- fill issued: 5x111x5 red_concrete at -930,796 (zone forceloaded ±48)
- phase: 4 marker filled (16:10:46)

### /clod-cells AFTER FILL (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 5 0 0x8400 0x3100
123 122 -940 784 71 0 1 [97, 119, 97] 5 0 0x8700 0x3100
124 122 -938 784 68 0 1 [97, 119, 97] 4 0 0x8400 0x3100
125 122 -936 784 62 1 1 [63, 118, 228] 4 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 3 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 2 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 2 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 440 0 0x8000 0x5c00
130 122 -926 784 66 0 1 [134, 183, 131] 439 0 0x8200 0x5c00
131 122 -924 784 75 0 1 [97, 119, 97] 393 0 0x8b00 0x3100
132 122 -922 784 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 122 -920 784 75 0 1 [97, 119, 97] 18446744073709551615 1 0x8b00 0x3100
134 122 -918 784 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x3100
122 123 -942 786 70 0 1 [97, 119, 97] 5 0 0x8600 0x3100
123 123 -940 786 65 0 1 [97, 119, 97] 5 0 0x8100 0x3100
124 123 -938 786 65 0 1 [97, 119, 97] 4 0 0x8100 0x3100
125 123 -936 786 62 1 1 [63, 118, 228] 4 0 0x7e00 0xd800
126 123 -934 786 62 1 1 [63, 118, 228] 3 0 0x7e00 0xd800
127 123 -932 786 62 1 1 [63, 118, 228] 3 0 0x7e00 0xd800
128 123 -930 786 62 1 1 [63, 118, 228] 2 0 0x7e00 0xd800
129 123 -928 786 73 0 1 [97, 119, 97] 394 0 0x8900 0x3100
130 123 -926 786 67 0 1 [134, 183, 131] 439 0 0x8300 0x5c00
131 123 -924 786 69 0 1 [134, 183, 131] 438 0 0x8500 0x5c00
132 123 -922 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 123 -920 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
134 123 -918 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
122 124 -942 788 72 0 1 [97, 119, 97] 5 0 0x8800 0x3100
123 124 -940 788 70 0 1 [97, 119, 97] 5 0 0x8600 0x3100
124 124 -938 788 68 0 1 [97, 119, 97] 4 0 0x8400 0x3100
125 124 -936 788 62 1 1 [63, 118, 228] 4 0 0x7e00 0xd800
126 124 -934 788 62 1 1 [63, 118, 228] 3 0 0x7e00 0xd800
127 124 -932 788 62 1 1 [63, 118, 228] 3 0 0x7e00 0xd800
128 124 -930 788 73 0 1 [97, 119, 97] 2 0 0x8900 0x3100
129 124 -928 788 76 0 1 [97, 119, 97] 394 0 0x8c00 0x3100
130 124 -926 788 73 0 1 [97, 119, 97] 399 0 0x8900 0x3100
131 124 -924 788 69 0 1 [134, 183, 131] 438 0 0x8500 0x5c00
132 124 -922 788 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 124 -920 788 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
134 124 -918 788 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x3100
122 125 -942 790 74 0 1 [97, 119, 97] 5 0 0x8a00 0x3100
123 125 -940 790 73 0 1 [97, 119, 97] 5 0 0x8900 0x3100
124 125 -938 790 71 0 1 [97, 119, 97] 4 0 0x8700 0x3100
125 125 -936 790 62 1 1 [63, 118, 228] 4 0 0x7e00 0xd800
126 125 -934 790 62 1 1 [63, 118, 228] 3 0 0x7e00 0xd800
127 125 -932 790 62 1 1 [63, 118, 228] 3 0 0x7e00 0xd800
128 125 -930 790 62 1 1 [63, 118, 228] 2 0 0x7e00 0xd800
129 125 -928 790 73 0 1 [97, 119, 97] 395 0 0x8900 0x3100
130 125 -926 790 68 0 1 [134, 183, 131] 439 0 0x8400 0x5c00
131 125 -924 790 69 0 1 [134, 183, 131] 438 0 0x8500 0x5c00
132 125 -922 790 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
133 125 -920 790 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
134 125 -918 790 70 0 1 [134, 183, 131] 18446744073709551615 1 0x8600 0x5c00
122 126 -942 792 74 0 1 [97, 119, 97] 5 0 0x8a00 0x3100
123 126 -940 792 73 0 1 [97, 119, 97] 5 0 0x8900 0x3100
124 126 -938 792 68 0 1 [97, 119, 97] 4 0 0x8400 0x3100
125 126 -936 792 62 1 1 [63, 118, 228] 4 0 0x7e00 0xd800
126 126 -934 792 62 1 1 [63, 118, 228] 3 0 0x7e00 0xd800
127 126 -932 792 62 1 1 [63, 118, 228] 3 0 0x7e00 0xd800
128 126 -930 792 62 1 1 [63, 118, 228] 2 0 0x7e00 0xd800
129 126 -928 792 67 0 1 [134, 183, 131] 2 0 0x8300 0x5c00
130 126 -926 792 74 0 1 [97, 119, 97] 399 0 0x8a00 0x3100
131 126 -924 792 77 0 1 [97, 119, 97] 393 0 0x8d00 0x3100
132 126 -922 792 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x3100
133 126 -920 792 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3800
134 126 -918 792 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x8800
122 127 -942 794 67 0 1 [134, 183, 131] 5 0 0x8300 0x5c00
123 127 -940 794 68 0 1 [134, 183, 131] 5 0 0x8400 0x5c00
124 127 -938 794 65 0 1 [134, 183, 131] 4 0 0x8100 0x5c00
125 127 -936 794 62 1 1 [63, 118, 228] 4 0 0x7e00 0xd800
126 127 -934 794 62 1 1 [63, 118, 228] 3 0 0x7e00 0xd800
127 127 -932 794 62 1 1 [63, 118, 228] 3 0 0x7e00 0xd800
128 127 -930 794 62 1 1 [63, 118, 228] 2 0 0x7e00 0xd800
129 127 -928 794 66 0 1 [134, 183, 131] 2 0 0x8200 0x5c00
130 127 -926 794 74 0 1 [97, 119, 97] 399 0 0x8a00 0x3100
131 127 -924 794 77 0 1 [97, 119, 97] 393 0 0x8d00 0x3100
132 127 -922 794 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x3100
133 127 -920 794 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3800
134 127 -918 794 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x8800
122 128 -942 796 68 0 1 [134, 183, 131] 6 0 0x8400 0x5c00
123 128 -940 796 72 0 1 [97, 119, 97] 5 0 0x8800 0x3100
124 128 -938 796 75 0 1 [97, 119, 97] 4 0 0x8b00 0x3100
125 128 -936 796 72 0 1 [97, 119, 97] 4 0 0x8800 0x3100
126 128 -934 796 62 1 1 [63, 118, 228] 3 0 0x7e00 0xd800
127 128 -932 796 62 1 1 [63, 118, 228] 3 0 0x7e00 0xd800
128 128 -930 796 62 1 1 [63, 118, 228] 2 0 0x7e00 0xd800
129 128 -928 796 66 0 1 [134, 183, 131] 2 0 0x8200 0x5c00
130 128 -926 796 68 0 1 [134, 183, 131] 439 0 0x8400 0x5c00
131 128 -924 796 74 0 1 [97, 119, 97] 393 0 0x8a00 0x3100
132 128 -922 796 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 128 -920 796 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x3100
134 128 -918 796 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
122 129 -942 798 69 0 1 [134, 183, 131] 5 0 0x8500 0x5c00
123 129 -940 798 75 0 1 [97, 119, 97] 5 0 0x8b00 0x3100
124 129 -938 798 78 0 1 [97, 119, 97] 6 0 0x8e00 0x3100
125 129 -936 798 75 0 1 [97, 119, 97] 4 0 0x8b00 0x3100
126 129 -934 798 62 1 1 [63, 118, 228] 3 0 0x7e00 0xd800
127 129 -932 798 62 1 1 [63, 118, 228] 3 0 0x7e00 0xd800
128 129 -930 798 62 1 1 [63, 118, 228] 2 0 0x7e00 0xd800
129 129 -928 798 71 0 1 [97, 119, 97] 2 0 0x8700 0x3100
130 129 -926 798 71 0 1 [97, 119, 97] 399 0 0x8700 0x3100
131 129 -924 798 69 0 1 [134, 183, 131] 438 0 0x8500 0x5c00
132 129 -922 798 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x3100
133 129 -920 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
134 129 -918 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
122 130 -942 800 70 0 1 [134, 183, 131] 5 0 0x8600 0x5c00
123 130 -940 800 76 0 1 [97, 119, 97] 5 0 0x8c00 0x3100
124 130 -938 800 76 0 1 [97, 119, 97] 4 0 0x8c00 0x3100
125 130 -936 800 72 0 1 [97, 119, 97] 4 0 0x8800 0x3100
126 130 -934 800 62 1 1 [63, 118, 228] 3 0 0x7e00 0xd800
127 130 -932 800 62 1 1 [63, 118, 228] 3 0 0x7e00 0xd800
128 130 -930 800 62 1 1 [63, 118, 228] 2 0 0x7e00 0xd800
129 130 -928 800 71 0 1 [97, 119, 97] 399 0 0x8700 0x3100
130 130 -926 800 71 0 1 [97, 119, 97] 399 0 0x8700 0x3100
131 130 -924 800 69 0 1 [134, 183, 131] 438 0 0x8500 0x5c00
132 130 -922 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
133 130 -920 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
134 130 -918 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
122 131 -942 802 78 0 1 [97, 119, 97] 7 0 0x8e00 0x3100
123 131 -940 802 77 0 1 [97, 119, 97] 5 0 0x8d00 0x3100
124 131 -938 802 77 0 1 [97, 119, 97] 4 0 0x8d00 0x3100
125 131 -936 802 66 0 1 [134, 183, 131] 4 0 0x8200 0x5c00
126 131 -934 802 62 1 1 [63, 118, 228] 3 0 0x7e00 0xd800
127 131 -932 802 62 1 1 [63, 118, 228] 3 0 0x7e00 0xd800
128 131 -930 802 62 1 1 [63, 118, 228] 2 0 0x7e00 0xd800
129 131 -928 802 62 1 1 [63, 118, 228] 440 0 0x7e00 0xd800
130 131 -926 802 66 0 1 [134, 183, 131] 439 0 0x8200 0x5c00
131 131 -924 802 68 0 1 [134, 183, 131] 438 0 0x8400 0x5c00
132 131 -922 802 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 131 -920 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
134 131 -918 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
122 132 -942 804 79 0 1 [97, 119, 97] 7 0 0x8f00 0x3100
123 132 -940 804 76 0 1 [97, 119, 97] 5 0 0x8c00 0x3100
124 132 -938 804 76 0 1 [97, 119, 97] 4 0 0x8c00 0x3100
125 132 -936 804 76 0 1 [97, 119, 97] 4 0 0x8c00 0x3100
126 132 -934 804 62 1 1 [63, 118, 228] 3 0 0x7e00 0xd800
127 132 -932 804 62 1 1 [63, 118, 228] 3 0 0x7e00 0xd800
128 132 -930 804 62 1 1 [63, 118, 228] 2 0 0x7e00 0xd800
129 132 -928 804 62 1 1 [63, 118, 228] 440 0 0x7e00 0xd800
130 132 -926 804 65 0 1 [134, 183, 131] 439 0 0x8100 0x5c00
131 132 -924 804 77 0 1 [97, 119, 97] 393 0 0x8d00 0x3100
132 132 -922 804 79 0 1 [97, 119, 97] 399 0 0x8f00 0x3100
133 132 -920 804 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
134 132 -918 804 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x3100
122 133 -942 806 78 0 1 [97, 119, 97] 7 0 0x8e00 0x3100
123 133 -940 806 69 0 1 [134, 183, 131] 5 0 0x8500 0x5c00
124 133 -938 806 155 0 1 [134, 183, 131] 5 0 0xdb00 0x5c00
125 133 -936 806 155 0 1 [134, 183, 131] 5 0 0xdb00 0x5c00
126 133 -934 806 155 0 1 [134, 183, 131] 4 0 0xdb00 0x5c00
127 133 -932 806 155 0 1 [134, 183, 131] 4 0 0xdb00 0x5c00
128 133 -930 806 155 0 1 [134, 183, 131] 3 0 0xdb00 0x5c00
129 133 -928 806 155 0 1 [134, 183, 131] 2 0 0xdb00 0x5c00
130 133 -926 806 155 0 1 [134, 183, 131] 2 0 0xdb00 0x5c00
131 133 -924 806 67 0 1 [134, 183, 131] 438 0 0x8300 0x5c00
132 133 -922 806 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 133 -920 806 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
134 133 -918 806 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x3100
122 134 -942 808 79 0 1 [97, 119, 97] 7 0 0x8f00 0x3100
123 134 -940 808 68 0 1 [134, 183, 131] 5 0 0x8400 0x5c00
124 134 -938 808 155 0 1 [134, 183, 131] 5 0 0xdb00 0x5c00
125 134 -936 808 180 0 1 [134, 183, 131] 2 0 0xf400 0x5c00
126 134 -934 808 180 0 1 [134, 183, 131] 2 0 0xf400 0x5c00
127 134 -932 808 180 0 1 [134, 183, 131] 439 0 0xf400 0x5c00
128 134 -930 808 180 0 1 [134, 183, 131] 439 0 0xf400 0x5c00
129 134 -928 808 180 0 1 [134, 183, 131] 438 0 0xf400 0x5c00
130 134 -926 808 155 0 1 [134, 183, 131] 2 0 0xdb00 0x5c00
131 134 -924 808 67 0 1 [134, 183, 131] 438 0 0x8300 0x5c00
132 134 -922 808 68 0 1 [134, 183, 131] 18446744073709551615 1 0x8400 0x5c00
133 134 -920 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
134 134 -918 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
```
### stream diag after fill:
[c-LoD] stream: tick 3600 65536/65536 cells, lap 730, cone 19541 zone 1355 dirty 944 (q 11402) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 900937c/2577e, edits +5624p/4238c/562e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 331 forced 181, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 615215p/2097b fail 0 ~64 kB/s cache 13619/24576 h601596 m13619 e0, srbeat 55236s/391q drop 42stale/0undec, ereb 49478s/81q, sweep 0r/99d
[16:10:43 INFO]: [c-LoD] stream: tick 3600 65536/65536 cells, lap 730, cone 19541 zone 1355 dirty 944 (q 11402) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 900937c/2577e, edits +5624p/4238c/562e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 331 forced 181, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 615215p/2097b fail 0 ~64 kB/s cache 13619/24576 h601596 m13619 e0, srbeat 55236s/391q drop 42stale/0undec, ereb 49478s/81q, sweep 0r/99d
- carrier ladder y=155: 12852 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=165: 12860 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=168: 12860 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=172: 12860 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=176: 12860 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=180: 12860 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=185: 12860 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- probe 0: red=0 paint=3063 lod_silhouette_cols=21 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=5 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 1: red=0 paint=3063 lod_silhouette_cols=21 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=5 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 2: red=0 paint=3332 lod_silhouette_cols=21 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=5 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 3: red=0 paint=3285 lod_silhouette_cols=20 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=5 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 4: red=0 paint=3540 lod_silhouette_cols=20 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=5 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 5: red=0 paint=3137 lod_silhouette_cols=21 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=5 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 6: red=0 paint=4664 lod_silhouette_cols=20 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=5 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 7: red=0 paint=4829 lod_silhouette_cols=20 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=5 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 8: red=0 paint=4874 lod_silhouette_cols=21 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=5 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 9: red=0 paint=4620 lod_silhouette_cols=21 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=5 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 10: red=0 paint=4603 lod_silhouette_cols=21 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=5 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 11: red=0 paint=4603 lod_silhouette_cols=21 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=5 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 12: red=0 paint=4603 lod_silhouette_cols=21 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=5 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 13: red=0 paint=4603 lod_silhouette_cols=21 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=5 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 14: red=0 paint=4809 lod_silhouette_cols=21 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=5 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 15: red=0 paint=4909 lod_silhouette_cols=21 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=5 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 16: red=0 paint=4758 lod_silhouette_cols=20 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=5 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 17: red=0 paint=5080 lod_silhouette_cols=20 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=5 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 18: red=0 paint=4699 lod_silhouette_cols=21 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=5 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 19: red=0 paint=4715 lod_silhouette_cols=21 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=5 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 20: red=0 paint=4910 lod_silhouette_cols=20 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=5 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 21: red=0 paint=4829 lod_silhouette_cols=21 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=5 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 22: red=0 paint=5061 lod_silhouette_cols=20 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=5 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 23: red=0 paint=5263 lod_silhouette_cols=20 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=6 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 24: red=0 paint=5030 lod_silhouette_cols=21 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=6 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 25: red=0 paint=5066 lod_silhouette_cols=21 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=6 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 26: red=0 paint=5210 lod_silhouette_cols=21 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=6 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 27: red=0 paint=5171 lod_silhouette_cols=20 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=6 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 28: red=0 paint=5320 lod_silhouette_cols=20 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=6 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 29: red=0 paint=6196 lod_silhouette_cols=21 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=6 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 30: red=0 paint=6261 lod_silhouette_cols=21 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=6 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 31: red=0 paint=6281 lod_silhouette_cols=21 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=6 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 32: red=0 paint=6441 lod_silhouette_cols=21 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=6 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 33: red=0 paint=6432 lod_silhouette_cols=20 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=6 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 34: red=0 paint=6556 lod_silhouette_cols=20 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=6 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 35: red=0 paint=6204 lod_silhouette_cols=21 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=6 mask_hits=30 (need red>=60 paint>=300 lod>20)
- phase: 5 leg-1 probes done (16:16:42)
- **FAIL:** leg-1 never saw the marker in 24 probes (red < 60 everywhere) — the edit never reached the LOD paint; this failure was SILENT before v6.31.1
### stream diag after probes:
[16:16:24 INFO]: [c-LoD] stream: tick 7000 65536/65536 cells, lap 936, cone 46610 zone 3143 dirty 994 (q 11448) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1999553c/2648e, edits +7310p/5435c/961e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 331 forced 181, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 935701p/2792b fail 0 ~65 kB/s cache 13711/24576 h921990 m13711 e0, srbeat 66620s/407q drop 49stale/0undec, ereb 50540s/0q, sweep 0r/83d
[c-LoD] stream: tick 7100 65536/65536 cells, lap 944, cone 47762 zone 3312 dirty 994 (q 11448) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 2031484c/2648e, edits +7310p/5435c/961e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 331 forced 181, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 949445p/2816b fail 0 ~72 kB/s cache 13711/24576 h935734 m13711 e0, srbeat 67027s/559q drop 49stale/0undec, ereb 50540s/0q, sweep 0r/92d
[16:16:34 INFO]: [c-LoD] stream: tick 7100 65536/65536 cells, lap 944, cone 47762 zone 3312 dirty 994 (q 11448) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 2031484c/2648e, edits +7310p/5435c/961e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 331 forced 181, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 949445p/2816b fail 0 ~72 kB/s cache 13711/24576 h935734 m13711 e0, srbeat 67027s/559q drop 49stale/0undec, ereb 50540s/0q, sweep 0r/92d
- parity debug tour: 13 vantages (session-1 client, DEBUG pack — evidence only, the gate pairs the §7b world session)
- parity debug tour done: 13 shots in parity_dbg/ (the §7b world session feeds the gate)
- joined session 2 (16:19:33)
- leg-2 baseline scene px (x<940): 0 (0 = the relog client paints no world at all)
- leg-2 renderer probe NEAR (20 m) white px: 0 raw / 0 changed-vs-baseline (> 20 = the client renders particles)
- leg-2 renderer probe FAR (80 m, marker zone) white px: 498 raw / 498 changed-vs-baseline (> 20 = no distance cut; ~0 with NEAR alive = particles beyond the client chunk radius do not render)
- leg-2 carrier probe green dust (80 m): changed-green px 0 / total changed 329 vs baseline (> 100 green = a stable green carrier exists; total > 0 with green ~0 = the dust renders but fog washes it — a ruler problem, not a carrier problem)
- phase: 8 leg-2 relog probes (16:22:24)
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
- relog probe 18: red=0 scene=1067 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 19: red=0 scene=1425 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=2 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- **FAIL:** marker lost after relog — stream state is not persistent/deterministic
- phase: 9 leg-2 done (16:25:23)

### client log forensics (pipeline/particle/atlas — v6.8.2)
```
[16:19:22] [main/DEBUG]: Loading LWJGL system
WARNING: java.lang.System::load has been called by org.lwjgl.system.Library$$Lambda/0x000000004b1e3d60 in an unnamed module (file:/home/runner/mc-test/client/libraries/org/lwjgl/lwjgl/3.4.3/lwjgl-3.4.3.jar)
WARNING: sun.misc.Unsafe::objectFieldOffset has been called by org.lwjgl.system.MemoryBackendUnsafeLegacy (file:/home/runner/mc-test/client/libraries/org/lwjgl/lwjgl/3.4.3/lwjgl-3.4.3.jar)
WARNING: Please consider reporting this to the maintainers of class org.lwjgl.system.MemoryBackendUnsafeLegacy
[16:19:22] [main/DEBUG]: Loading shaderc
[16:19:23] [main/DEBUG]: Loading OpenGL
[16:19:27] [Render thread/ERROR]: Error parsing option value 4 for option Simulation Distance: Value 4 outside of range [5:33]
[16:19:27] [Render thread/INFO]: Backend library: LWJGL version 3.4.3+4
[16:19:28] [Render thread/INFO]: Using graphics backend OpenGL, using drivers: 4.5 (Core Profile) Mesa 26.0.8-1ubuntu0.3
[16:19:28] [Render thread/INFO]: Using graphics device: llvmpipe (LLVM 21.1.8, 256 bits) (Mesa)
[16:19:28] [Render thread/INFO]: Using graphics device extensions: GL_ARB_multi_draw_indirect, GL_ARB_buffer_storage, GL_ARB_shader_draw_parameters, GL_ARB_base_instance, GL_KHR_debug, GL_ARB_draw_indirect, GL_ARB_clip_control, GL_ARB_vertex_attrib_binding, GL_ARB_direct_state_access, GL_EXT_texture_filter_anisotropic
[16:19:28] [Download-2/ERROR]: Failed to fetch user properties
com.mojang.authlib.exceptions.InvalidCredentialsException: Status: 401
	at com.mojang.authlib.exceptions.MinecraftClientHttpException.toAuthenticationException(MinecraftClientHttpException.java:70)
Caused by: MinecraftClientHttpException[type=HTTP_ERROR, status=401, response=ErrorResponse[path=/player/attributes, error=null, errorMessage=null, details=null]]
	at com.mojang.authlib.minecraft.client.MinecraftClient.buildHttpException(MinecraftClient.java:170)
[16:19:28] [Download-1/INFO]: Could not authorize you against Realms server: java.lang.RuntimeException: Failed to parse into SignedJWT: 0
[16:19:28] [Download-1/ERROR]: Failed to fetch Realms feature flags
com.mojang.realmsclient.exception.RealmsServiceException: Realms authentication error with message 'java.lang.RuntimeException: Failed to parse into SignedJWT: 0'
[16:19:28] [Render thread/INFO]: Reloading ResourceManager: vanilla, vanilla
[16:19:31] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[16:19:31] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[16:19:31] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[16:19:31] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[16:19:31] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[16:19:31] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[16:19:31] [Render thread/INFO]: Created: 2048x2048x4 minecraft:textures/atlas/blocks.png-atlas
[16:19:31] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[16:19:31] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[16:19:31] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[16:19:31] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[16:19:31] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
[16:19:38] [Render thread/INFO]: Reloading ResourceManager: vanilla, vanilla, server/00000000/6b1f2f7e-9c0d-4a4b-8b0a-b10d1b4ab100
[16:19:40] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[16:19:40] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[16:19:40] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[16:19:40] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[16:19:40] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[16:19:40] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[16:19:40] [Render thread/INFO]: Created: 2048x2048x4 minecraft:textures/atlas/blocks.png-atlas
[16:19:40] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[16:19:40] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[16:19:40] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[16:19:40] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[16:19:40] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
```

### /clod-cells AFTER PROBES (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 16 0 0x8400 0x3100
123 122 -940 784 71 0 1 [97, 119, 97] 16 0 0x8700 0x3100
124 122 -938 784 68 0 1 [97, 119, 97] 15 0 0x8400 0x3100
125 122 -936 784 62 1 1 [63, 118, 228] 15 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 14 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 13 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 13 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 126 0 0x8000 0x5c00
130 122 -926 784 66 0 1 [134, 183, 131] 126 0 0x8200 0x5c00
131 122 -924 784 75 0 1 [97, 119, 97] 86 0 0x8b00 0x3100
132 122 -922 784 77 0 1 [97, 119, 97] 4233 0 0x8d00 0x3100
133 122 -920 784 75 0 1 [97, 119, 97] 4233 0 0x8b00 0x3100
134 122 -918 784 73 0 1 [97, 119, 97] 4233 0 0x8900 0x3100
122 123 -942 786 70 0 1 [97, 119, 97] 17 0 0x8600 0x3100
123 123 -940 786 65 0 1 [97, 119, 97] 16 0 0x8100 0x3100
124 123 -938 786 65 0 1 [97, 119, 97] 15 0 0x8100 0x3100
125 123 -936 786 62 1 1 [63, 118, 228] 15 0 0x7e00 0xd800
126 123 -934 786 62 1 1 [63, 118, 228] 14 0 0x7e00 0xd800
127 123 -932 786 62 1 1 [63, 118, 228] 14 0 0x7e00 0xd800
128 123 -930 786 62 1 1 [63, 118, 228] 13 0 0x7e00 0xd800
129 123 -928 786 73 0 1 [97, 119, 97] 86 0 0x8900 0x3100
130 123 -926 786 67 0 1 [134, 183, 131] 126 0 0x8300 0x5c00
131 123 -924 786 69 0 1 [134, 183, 131] 126 0 0x8500 0x5c00
132 123 -922 786 77 0 1 [97, 119, 97] 4233 0 0x8d00 0x3100
133 123 -920 786 77 0 1 [97, 119, 97] 4233 0 0x8d00 0x3100
134 123 -918 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
122 124 -942 788 72 0 1 [97, 119, 97] 17 0 0x8800 0x3100
123 124 -940 788 70 0 1 [97, 119, 97] 16 0 0x8600 0x3100
124 124 -938 788 68 0 1 [97, 119, 97] 15 0 0x8400 0x3100
125 124 -936 788 62 1 1 [63, 118, 228] 15 0 0x7e00 0xd800
126 124 -934 788 62 1 1 [63, 118, 228] 14 0 0x7e00 0xd800
127 124 -932 788 62 1 1 [63, 118, 228] 14 0 0x7e00 0xd800
128 124 -930 788 73 0 1 [97, 119, 97] 13 0 0x8900 0x3100
129 124 -928 788 76 0 1 [97, 119, 97] 75 0 0x8c00 0x3100
130 124 -926 788 73 0 1 [97, 119, 97] 75 0 0x8900 0x3100
131 124 -924 788 69 0 1 [134, 183, 131] 125 0 0x8500 0x5c00
132 124 -922 788 77 0 1 [97, 119, 97] 4233 0 0x8d00 0x3100
133 124 -920 788 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
134 124 -918 788 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x3100
122 125 -942 790 74 0 1 [97, 119, 97] 15 0 0x8a00 0x3100
123 125 -940 790 73 0 1 [97, 119, 97] 15 0 0x8900 0x3100
124 125 -938 790 71 0 1 [97, 119, 97] 15 0 0x8700 0x3100
125 125 -936 790 62 1 1 [63, 118, 228] 15 0 0x7e00 0xd800
126 125 -934 790 62 1 1 [63, 118, 228] 14 0 0x7e00 0xd800
127 125 -932 790 62 1 1 [63, 118, 228] 14 0 0x7e00 0xd800
128 125 -930 790 62 1 1 [63, 118, 228] 13 0 0x7e00 0xd800
129 125 -928 790 73 0 1 [97, 119, 97] 75 0 0x8900 0x3100
130 125 -926 790 68 0 1 [134, 183, 131] 125 0 0x8400 0x5c00
131 125 -924 790 69 0 1 [134, 183, 131] 125 0 0x8500 0x5c00
132 125 -922 790 69 0 1 [134, 183, 131] 4233 0 0x8500 0x5c00
133 125 -920 790 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
134 125 -918 790 70 0 1 [134, 183, 131] 18446744073709551615 1 0x8600 0x5c00
122 126 -942 792 74 0 1 [97, 119, 97] 16 0 0x8a00 0x3100
123 126 -940 792 73 0 1 [97, 119, 97] 15 0 0x8900 0x3100
124 126 -938 792 68 0 1 [97, 119, 97] 15 0 0x8400 0x3100
125 126 -936 792 62 1 1 [63, 118, 228] 15 0 0x7e00 0xd800
126 126 -934 792 62 1 1 [63, 118, 228] 14 0 0x7e00 0xd800
127 126 -932 792 62 1 1 [63, 118, 228] 14 0 0x7e00 0xd800
128 126 -930 792 62 1 1 [63, 118, 228] 13 0 0x7e00 0xd800
129 126 -928 792 67 0 1 [134, 183, 131] 125 0 0x8300 0x5c00
130 126 -926 792 74 0 1 [97, 119, 97] 75 0 0x8a00 0x3100
131 126 -924 792 77 0 1 [97, 119, 97] 75 0 0x8d00 0x3100
132 126 -922 792 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x3100
133 126 -920 792 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3800
134 126 -918 792 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x8800
122 127 -942 794 67 0 1 [134, 183, 131] 17 0 0x8300 0x5c00
123 127 -940 794 68 0 1 [134, 183, 131] 16 0 0x8400 0x5c00
124 127 -938 794 65 0 1 [134, 183, 131] 15 0 0x8100 0x5c00
125 127 -936 794 62 1 1 [63, 118, 228] 15 0 0x7e00 0xd800
126 127 -934 794 62 1 1 [63, 118, 228] 14 0 0x7e00 0xd800
127 127 -932 794 170 0 1 [142, 33, 33] 14 0 0xea00 0x0200
128 127 -930 794 170 0 1 [142, 33, 33] 13 0 0xea00 0x0200
129 127 -928 794 170 0 1 [142, 33, 33] 75 0 0xea00 0x0200
130 127 -926 794 74 0 1 [97, 119, 97] 75 0 0x8a00 0x3100
131 127 -924 794 77 0 1 [97, 119, 97] 75 0 0x8d00 0x3100
132 127 -922 794 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x3100
133 127 -920 794 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3800
134 127 -918 794 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x8800
122 128 -942 796 68 0 1 [134, 183, 131] 17 0 0x8400 0x5c00
123 128 -940 796 72 0 1 [97, 119, 97] 15 0 0x8800 0x3100
124 128 -938 796 75 0 1 [97, 119, 97] 15 0 0x8b00 0x3100
125 128 -936 796 72 0 1 [97, 119, 97] 15 0 0x8800 0x3100
126 128 -934 796 62 1 1 [63, 118, 228] 14 0 0x7e00 0xd800
127 128 -932 796 170 0 1 [142, 33, 33] 14 0 0xea00 0x0200
128 128 -930 796 170 0 1 [142, 33, 33] 13 0 0xea00 0x0200
129 128 -928 796 170 0 1 [142, 33, 33] 75 0 0xea00 0x0200
130 128 -926 796 68 0 1 [134, 183, 131] 125 0 0x8400 0x5c00
131 128 -924 796 74 0 1 [97, 119, 97] 75 0 0x8a00 0x3100
132 128 -922 796 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 128 -920 796 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x3100
134 128 -918 796 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
122 129 -942 798 69 0 1 [134, 183, 131] 17 0 0x8500 0x5c00
123 129 -940 798 75 0 1 [97, 119, 97] 15 0 0x8b00 0x3100
124 129 -938 798 78 0 1 [97, 119, 97] 16 0 0x8e00 0x3100
125 129 -936 798 75 0 1 [97, 119, 97] 15 0 0x8b00 0x3100
126 129 -934 798 62 1 1 [63, 118, 228] 14 0 0x7e00 0xd800
127 129 -932 798 170 0 1 [142, 33, 33] 14 0 0xea00 0x0200
128 129 -930 798 170 0 1 [142, 33, 33] 13 0 0xea00 0x0200
129 129 -928 798 170 0 1 [142, 33, 33] 75 0 0xea00 0x0200
130 129 -926 798 71 0 1 [97, 119, 97] 75 0 0x8700 0x3100
131 129 -924 798 69 0 1 [134, 183, 131] 125 0 0x8500 0x5c00
132 129 -922 798 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x3100
133 129 -920 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
134 129 -918 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
122 130 -942 800 70 0 1 [134, 183, 131] 17 0 0x8600 0x5c00
123 130 -940 800 76 0 1 [97, 119, 97] 15 0 0x8c00 0x3100
124 130 -938 800 76 0 1 [97, 119, 97] 14 0 0x8c00 0x3100
125 130 -936 800 72 0 1 [97, 119, 97] 14 0 0x8800 0x3100
126 130 -934 800 62 1 1 [63, 118, 228] 14 0 0x7e00 0xd800
127 130 -932 800 62 1 1 [63, 118, 228] 14 0 0x7e00 0xd800
128 130 -930 800 62 1 1 [63, 118, 228] 13 0 0x7e00 0xd800
129 130 -928 800 71 0 1 [97, 119, 97] 74 0 0x8700 0x3100
130 130 -926 800 71 0 1 [97, 119, 97] 74 0 0x8700 0x3100
131 130 -924 800 69 0 1 [134, 183, 131] 125 0 0x8500 0x5c00
132 130 -922 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
133 130 -920 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
134 130 -918 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
122 131 -942 802 78 0 1 [97, 119, 97] 17 0 0x8e00 0x3100
123 131 -940 802 77 0 1 [97, 119, 97] 14 0 0x8d00 0x3100
124 131 -938 802 77 0 1 [97, 119, 97] 14 0 0x8d00 0x3100
125 131 -936 802 66 0 1 [134, 183, 131] 15 0 0x8200 0x5c00
126 131 -934 802 62 1 1 [63, 118, 228] 14 0 0x7e00 0xd800
127 131 -932 802 62 1 1 [63, 118, 228] 14 0 0x7e00 0xd800
128 131 -930 802 62 1 1 [63, 118, 228] 13 0 0x7e00 0xd800
129 131 -928 802 62 1 1 [63, 118, 228] 125 0 0x7e00 0xd800
130 131 -926 802 66 0 1 [134, 183, 131] 125 0 0x8200 0x5c00
131 131 -924 802 68 0 1 [134, 183, 131] 125 0 0x8400 0x5c00
132 131 -922 802 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 131 -920 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
134 131 -918 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
122 132 -942 804 79 0 1 [97, 119, 97] 17 0 0x8f00 0x3100
123 132 -940 804 76 0 1 [97, 119, 97] 14 0 0x8c00 0x3100
124 132 -938 804 76 0 1 [97, 119, 97] 14 0 0x8c00 0x3100
125 132 -936 804 76 0 1 [97, 119, 97] 14 0 0x8c00 0x3100
126 132 -934 804 62 1 1 [63, 118, 228] 14 0 0x7e00 0xd800
127 132 -932 804 62 1 1 [63, 118, 228] 14 0 0x7e00 0xd800
128 132 -930 804 62 1 1 [63, 118, 228] 13 0 0x7e00 0xd800
129 132 -928 804 62 1 1 [63, 118, 228] 125 0 0x7e00 0xd800
130 132 -926 804 65 0 1 [134, 183, 131] 125 0 0x8100 0x5c00
131 132 -924 804 77 0 1 [97, 119, 97] 74 0 0x8d00 0x3100
132 132 -922 804 79 0 1 [97, 119, 97] 74 0 0x8f00 0x3100
133 132 -920 804 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
134 132 -918 804 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x3100
122 133 -942 806 78 0 1 [97, 119, 97] 17 0 0x8e00 0x3100
123 133 -940 806 69 0 1 [134, 183, 131] 16 0 0x8500 0x5c00
124 133 -938 806 155 0 1 [134, 183, 131] 17 0 0xdb00 0x5c00
125 133 -936 806 155 0 1 [134, 183, 131] 16 0 0xdb00 0x5c00
126 133 -934 806 155 0 1 [134, 183, 131] 15 0 0xdb00 0x5c00
127 133 -932 806 155 0 1 [134, 183, 131] 15 0 0xdb00 0x5c00
128 133 -930 806 155 0 1 [134, 183, 131] 14 0 0xdb00 0x5c00
129 133 -928 806 155 0 1 [134, 183, 131] 13 0 0xdb00 0x5c00
130 133 -926 806 155 0 1 [134, 183, 131] 13 0 0xdb00 0x5c00
131 133 -924 806 67 0 1 [134, 183, 131] 125 0 0x8300 0x5c00
132 133 -922 806 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 133 -920 806 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
134 133 -918 806 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x3100
122 134 -942 808 79 0 1 [97, 119, 97] 17 0 0x8f00 0x3100
123 134 -940 808 68 0 1 [134, 183, 131] 16 0 0x8400 0x5c00
124 134 -938 808 155 0 1 [134, 183, 131] 16 0 0xdb00 0x5c00
125 134 -936 808 180 0 1 [134, 183, 131] 13 0 0xf400 0x5c00
126 134 -934 808 180 0 1 [134, 183, 131] 13 0 0xf400 0x5c00
127 134 -932 808 180 0 1 [134, 183, 131] 125 0 0xf400 0x5c00
128 134 -930 808 180 0 1 [134, 183, 131] 125 0 0xf400 0x5c00
129 134 -928 808 180 0 1 [134, 183, 131] 125 0 0xf400 0x5c00
130 134 -926 808 155 0 1 [134, 183, 131] 13 0 0xdb00 0x5c00
131 134 -924 808 67 0 1 [134, 183, 131] 125 0 0x8300 0x5c00
132 134 -922 808 68 0 1 [134, 183, 131] 18446744073709551615 1 0x8400 0x5c00
133 134 -920 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
134 134 -918 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
```
- edit channel: 0 linkage errors, 0 failed event dispatches (both must be 0)
- phase: 6 parity world boot (16:25:24)
- parity world: view-distance=8 (the voxy leg's server value)
- parity world: joined session 3 (16:26:05)
- parity world: scan populated (cells=65536; the march paints from these buffers)
- parity world tour: 13 vantages (production pack, vd 8, shared structures)
- coverage sweeps: 72 eye passes = 36 positions (12 az x r144/200 + 12 off-az x r172, y=160) x 2 looks (outward+inward), settle-by-laps >= 6 s (q stable 3x1s), deadline 12 s/pos, no shots — the v6.26.0 data-coverage lever + v6.26.6 lottery kill; probe rides parity/view_probe.md
- parity drain-settle TIMEOUT: queue 18737 -> 18730 > 11500 after 150s — continuing anyway; the queue did not drain without eye proximity (priority-lane evidence, the 01/05/10 dip attribution reads from the tour deltas)
- pre-tour pass done: 13 vantages x 10s (the tour now reads the second OR-merge lap; the rim/dip transport race reads from the tour deltas)
- parity world tour done: 13 shots, 0 night-guard renames in parity/ (the gate pairs voxy/<name>.png vs parity/<name>.png)
- phase: 7 parity tour done (16:47:56)
- sparse re-beat channel at tour end: streamed=75278 q=0 dropped_stale=189 dropped_undecodable=10319 (frac=8 boot; undecodable-dominant = the rim words die in their decodability window, stale-dominant = edit churn owns the cells)
### parity world stream counters (session 3, tail + maxima)
[c-LoD] stream: tick 13200 65536/65536 cells, lap 1211, cone 62425 zone 20022 dirty 14205 (q 18711) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 9006253c/1945e, edits +18010p/13655c/2889e drop 0, spawned 0 / failed 0, cam0 yaw -180 pitch 5, wish 0 forced 912, native 32832c/955ch fail 0 (3w q 0) helper on, trees 955ch/fail 0, wire bundle 1341382p/3937b fail 0 ~0 kB/s cache 22590/24576 h950892 m390490 e367900, srbeat 75278s/0q drop 189stale/10319undec, ereb 47589s/0q, sweep 0r/41d
[16:47:55 INFO]: [c-LoD] stream: tick 13200 65536/65536 cells, lap 1211, cone 62425 zone 20022 dirty 14205 (q 18711) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 9006253c/1945e, edits +18010p/13655c/2889e drop 0, spawned 0 / failed 0, cam0 yaw -180 pitch 5, wish 0 forced 912, native 32832c/955ch fail 0 (3w q 0) helper on, trees 955ch/fail 0, wire bundle 1341382p/3937b fail 0 ~0 kB/s cache 22590/24576 h950892 m390490 e367900, srbeat 75278s/0q drop 189stale/10319undec, ereb 47589s/0q, sweep 0r/41d
- parity stream max cone: 62425
- parity stream max zone: 20022
- parity stream max wire bundle: bundle
### parity world capture lens (debug chain on the §7b world)
lens_red=0 lens_black=0 lens_water=0 lens_dark=0 lens_zero=0 rot=none scan=0 carriers=0
- parity mesh probe @png_y=2: x306=(151,184,255) x307=(151,184,255) x308=(151,184,255) x309h=(151,184,255) x310c=(151,184,255)
- parity mesh probe @png_y=717: x306=(44,62,51) x307=(66,90,72) x308=(83,109,89) x309h=(83,109,89) x310c=(85,112,90)
- parity mesh engagement: magenta-ish=16 of 921600 sampled px
- mask8: the un-fog engagement masks shot on the PARITY WORLD (vd 8) — parity_dbg8/01m|07m|10m
- mask8 01m: green=0 red=0 blue=94176 dim=4096 (px, sampled /16)
- mask8 07m: green=12032 red=0 blue=221376 dim=5792 (px, sampled /16)
- mask8 10m: green=4960 red=128 blue=234544 dim=132784 (px, sampled /16)
- mesh probe @png_y=2: x306=(255,255,255) x307=(13,254,0) x308=(0,254,0) x309h=(12,0,0) x310c=(0,0,0)
- mesh probe @png_y=717: x306=(255,255,255) x307=(13,254,0) x308=(0,254,0) x309h=(12,0,0) x310c=(0,0,0)
- mesh engagement: magenta-ish=0 of 921600 sampled px (the vanilla-path paint coverage)
- mask8 fog state @png_y=2: raw (300)=(151,184,255) (301)=(151,184,255)
- mask8 fog state @png_y=717: raw (300)=(0,128,8) (301)=(179,206,255)
- mask8 fog state decoded: envStart=0 envEnd=1024 rdEnd=128 fogColor=(179,206,255)
- buffer dump 10m: marked h=32/2048 c=7/2048; height windows r0<142:15 r1:1 r2>=202:16 maxByte=255
- buffer dump 10m: RANK-2-WINDOW HEIGHTS (the fake-wall class): (+74,-32)h=175, (+76,-32)h=181, (+76,-30)h=179, (+78,-30)h=191, (+76,-26)h=191, (+76,-24)h=189, (+80,-22)h=191, (+82,-22)h=191, (-8,-18)h=189, (+82,-4)h=191
- rim mask: scan populated (cells=65536; the state paint reads the same buffers the march reads)
- phase: 10 rim mask lane (16:52:19)
- rim mask: decode-state frames shot (rim_mask/00|01|12 — GREEN full decode / RED colorless / BLUE heightless; 00 = the leg-1 tower eye, 01 = the near-tower contrast, 12 = the tower re-read; the inherited 07/12 vantages sat 760 blocks outside the streamed disk and 01 sat INSIDE the marker column — their earlier verdicts were classifier noise, purged in v6.33.1)
- rim mask 00: green=2704 red=6080 blue=194848 yellow=2992 (px, sampled /16) -> DOMINANT BLUE: rim = HEIGHTS NEVER DECODED — a data-coverage class: the state paint cannot even see the cells
- rim mask 01: green=4704 red=16016 blue=220752 yellow=10816 (px, sampled /16) -> DOMINANT BLUE: rim = HEIGHTS NEVER DECODED — a data-coverage class: the state paint cannot even see the cells
- rim mask 12: green=3472 red=10864 blue=252896 yellow=5120 (px, sampled /16) -> DOMINANT BLUE: rim = HEIGHTS NEVER DECODED — a data-coverage class: the state paint cannot even see the cells

### boot / stream
```
[c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 135240 bytes, sha1 b8a57b8dc12325d7967a161c21c4b0fd8636196c — zero textures, zero world data, one push per join
[c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[16:51:33 INFO]: [c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 135240 bytes, sha1 b8a57b8dc12325d7967a161c21c4b0fd8636196c — zero textures, zero world data, one push per join
[16:51:33 INFO]: [c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[c-LoD] stream: tick 900 65536/65536 cells, lap 612, cone 12647 zone 883 dirty 10 (q 3228) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 532467c/384e, edits +5p/5c/1e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 36 forced 877, native 4694c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 125937p/415b fail 0 ~36 kB/s cache 14064/24576 h111873 m14064 e0, srbeat 4184s/313q drop 0stale/572undec, ereb 78s/11q, sweep 0r/57d
[16:53:12 INFO]: [c-LoD] stream: tick 900 65536/65536 cells, lap 612, cone 12647 zone 883 dirty 10 (q 3228) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 532467c/384e, edits +5p/5c/1e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 36 forced 877, native 4694c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 125937p/415b fail 0 ~36 kB/s cache 14064/24576 h111873 m14064 e0, srbeat 4184s/313q drop 0stale/572undec, ereb 78s/11q, sweep 0r/57d
```
### sessions
```
[16:51:54 INFO]: System chat: Stepan joined the game
[16:53:14 INFO]: System chat: Stepan left the game
```
### client log tail
```
[16:53:11] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[16:53:11] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[16:53:11] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[16:53:11] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[16:53:11] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[16:53:11] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[16:53:11] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[16:53:11] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[16:53:11] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[16:53:11] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[16:53:11] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[16:53:11] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[16:53:11] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[16:53:11] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[16:53:11] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[16:53:11] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[16:53:11] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[16:53:11] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[16:53:11] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[16:53:11] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[16:53:11] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:bundle_delimiter] net.minecraft.network.protocol.game.ClientboundBundleDelimiterPacket -> 1 bytes
[16:53:11] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:set_entity_motion] net.minecraft.network.protocol.game.ClientboundSetEntityMotionPacket -> 8 bytes
[16:53:11] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[16:53:12] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:entity_position_sync] net.minecraft.network.protocol.game.ClientboundEntityPositionSyncPacket -> 38 bytes
[16:53:12] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[16:53:12] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[16:53:12] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[16:53:12] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[16:53:12] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[16:53:12] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[16:53:12] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[16:53:12] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[16:53:12] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[16:53:12] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[16:53:12] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[16:53:12] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[16:53:12] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[16:53:12] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[16:53:12] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[16:53:12] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[16:53:12] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:move_player_pos] net.minecraft.network.protocol.game.ServerboundMovePlayerPacket$Pos -> 26 bytes
[16:53:12] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[16:53:12] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[16:53:12] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[16:53:12] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[16:53:12] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[16:53:12] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[16:53:12] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[16:53:12] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[16:53:12] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[16:53:12] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[16:53:12] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[16:53:12] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[16:53:12] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:set_time] net.minecraft.network.protocol.game.ClientboundSetTimePacket -> 10 bytes
[16:53:12] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[16:53:12] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:keep_alive] net.minecraft.network.protocol.common.ClientboundKeepAlivePacket -> 9 bytes
[16:53:12] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:keep_alive] net.minecraft.network.protocol.common.ServerboundKeepAlivePacket -> 9 bytes
[16:53:13] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:set_entity_data] net.minecraft.network.protocol.game.ClientboundSetEntityDataPacket -> 6 bytes
[16:53:13] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[16:53:13] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[16:53:13] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[16:53:13] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[16:53:13] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[16:53:13] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[16:53:13] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[16:53:13] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[16:53:13] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[16:53:13] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[16:53:13] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[16:53:13] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[16:53:13] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[16:53:13] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[16:53:13] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[16:53:13] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[16:53:13] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[16:53:13] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[16:53:13] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:set_time] net.minecraft.network.protocol.game.ClientboundSetTimePacket -> 10 bytes
[16:53:13] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[16:53:13] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:keep_alive] net.minecraft.network.protocol.common.ClientboundKeepAlivePacket -> 9 bytes
[16:53:13] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:keep_alive] net.minecraft.network.protocol.common.ServerboundKeepAlivePacket -> 9 bytes
--- exceptions (Realms/RTKit noise filtered) ---
25:[16:51:49] [Render thread/ERROR]: Error parsing option value 4 for option Simulation Distance: Value 4 outside of range [5:33]
42:[16:51:49] [Download-1/ERROR]: Got an error with a html body connecting to https://sessionserver.mojang.com/session/minecraft/profile/d1652ad977ce32009642f973c97adf36?unsigned=false: <!DOCTYPE html PUBLIC '-//W3C//DTD XHTML 1.0 Transitional//EN' 'http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd'>
105:MinecraftClientHttpException[type=HTTP_ERROR, status=429, response=null]
106:	at com.mojang.authlib.minecraft.client.MinecraftClient.buildHttpException(MinecraftClient.java:167)
123:[16:51:49] [Download-2/ERROR]: Failed to fetch user properties
124:com.mojang.authlib.exceptions.InvalidCredentialsException: Status: 401
125:	at com.mojang.authlib.exceptions.MinecraftClientHttpException.toAuthenticationException(MinecraftClientHttpException.java:70)
132:Caused by: MinecraftClientHttpException[type=HTTP_ERROR, status=401, response=ErrorResponse[path=/player/attributes, error=null, errorMessage=null, details=null]]
133:	at com.mojang.authlib.minecraft.client.MinecraftClient.buildHttpException(MinecraftClient.java:170)
```
### server log tail
```
[16:53:15 INFO]: [ChunkHolderManager] Saved 1000 block chunks, 975 entity chunks, 0 poi chunks in world 'minecraft:overworld' in 0.80s
[16:53:15 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:overworld'
[16:53:15 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:overworld'
[16:53:15 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[16:53:15 INFO]: [ChunkHolderManager] Waiting 60s for chunk system to halt for world 'minecraft:the_nether'
[16:53:15 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:the_nether'
[16:53:15 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_nether'
[16:53:15 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_nether' in 0.00s
[16:53:15 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_nether'
[16:53:15 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_nether'
[16:53:15 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[16:53:15 INFO]: [ChunkHolderManager] Waiting 60s for chunk system to halt for world 'minecraft:the_end'
[16:53:15 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:the_end'
[16:53:15 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_end'
[16:53:15 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_end' in 0.00s
[16:53:15 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_end'
[16:53:15 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_end'
[16:53:15 INFO]: Waiting for all RegionFile I/O tasks to complete...
[16:53:15 INFO]: All RegionFile I/O tasks to complete
[16:53:15 INFO]: [MoonriseCommon] Awaiting termination of worker pool for up to 60s...
[16:53:15 INFO]: [MoonriseCommon] Awaiting termination of I/O pool for up to 60s...
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[launcher] server exited with code 143
```
### shots
```
-rw-r--r-- 1 runner runner  2658 Nov 16 16:24 lens_13_after_relog_probe11.png
-rw-r--r-- 1 runner runner  2658 Nov 16 16:24 lens_13_after_relog_probe12.png
-rw-r--r-- 1 runner runner  2658 Nov 16 16:24 lens_13_after_relog_probe13.png
-rw-r--r-- 1 runner runner  2658 Nov 16 16:24 lens_13_after_relog_probe14.png
-rw-r--r-- 1 runner runner  2658 Nov 16 16:24 lens_13_after_relog_probe15.png
-rw-r--r-- 1 runner runner  2658 Nov 16 16:24 lens_13_after_relog_probe16.png
-rw-r--r-- 1 runner runner  2658 Nov 16 16:24 lens_13_after_relog_probe17.png
-rw-r--r-- 1 runner runner  4158 Nov 16 16:25 lens_13_after_relog_probe18.png
-rw-r--r-- 1 runner runner  4329 Nov 16 16:25 lens_13_after_relog_probe19.png
-rw-r--r-- 1 runner runner  2658 Nov 16 16:22 lens_13_after_relog_probe2.png
-rw-r--r-- 1 runner runner  2658 Nov 16 16:22 lens_13_after_relog_probe3.png
-rw-r--r-- 1 runner runner  2658 Nov 16 16:23 lens_13_after_relog_probe4.png
-rw-r--r-- 1 runner runner  2658 Nov 16 16:23 lens_13_after_relog_probe5.png
-rw-r--r-- 1 runner runner  2658 Nov 16 16:23 lens_13_after_relog_probe6.png
-rw-r--r-- 1 runner runner  2658 Nov 16 16:23 lens_13_after_relog_probe7.png
-rw-r--r-- 1 runner runner  2658 Nov 16 16:23 lens_13_after_relog_probe8.png
-rw-r--r-- 1 runner runner  2658 Nov 16 16:23 lens_13_after_relog_probe9.png
drwxr-xr-x 2 runner runner  4096 Nov 16 16:47 lens_parity
drwxr-xr-x 2 runner runner  4096 Nov 16 16:47 parity
drwxr-xr-x 2 runner runner  4096 Nov 16 16:19 parity_dbg
drwxr-xr-x 2 runner runner  4096 Nov 16 16:51 parity_dbg8
-rw-r--r-- 1 runner runner 12080 Nov 16 16:25 probe_counts.txt
drwxr-xr-x 2 runner runner  4096 Nov 16 16:53 rim_mask
-rw-r--r-- 1 runner runner 59964 Nov 16 16:53 server_full.log
-rw-r--r-- 1 runner runner 70769 Nov 16 16:53 summary.md
```

**exit=1** failures=2 leg1=0 leg2=0
