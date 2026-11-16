- phase: 0 sanity (18:55:53)
### c-LoD visual run (-930,796, marker=red_concrete, vantage -1005,150 yaw 270)
- GL:     Vendor: Mesa (0xffffffff);    Device: llvmpipe (LLVM 21.1.8, 256 bits) (0xffffffff);OpenGL renderer string: llvmpipe (LLVM 21.1.8, 256 bits);OpenGL core profile version string: 4.5 (Core Profile) Mesa 26.0.8-1ubuntu0.3;
- boot + gates OK (18:56:10)
- phase: 1 server booted (18:56:10)
- sky-grid armed
- phase: 2 leg-1 client joined (18:56:27)
- joined session 1 (18:56:28)
- stream live: cells=65536
- phase: 3 scan populated (18:56:50)
- terrain range: 2 terraced mountains at (CX-4,CZ-20)/(CX-2,CZ+16), tops 185/190, grass terraces + stone caps, 66..82 m from the eye (inside the 83.5 m band, chunk edge 64 m)
- scan complete=1 (cells=cells=65536/65536)
- paint settled=1 (wire 519874p)
- baseline try 1: drift=0 px
- lod_silhouette_cols=50 (LOD paint ON SCREEN at 75 m if > 20 — the march-drawn marker column; vanilla cannot render there, chunk edge 64 m; #255 re-measures 49, the ZONE_FAR=48 regression #256 and the control read 0)
- 8437 terrain-green px in the sky region (LOD MOUNTAINS on screen if > 5000; the range paints ColLow-green on low terraces via the fallback palette; flat-world baselines and the pack-free control read ~0)
- burst_close white px: 477 (renderer alive if > 50)
- burst_far white px: 361 (renderer alive in the marker zone if > 20)
- cut walk 40 m: 0 white px (> 20 = rendered)
- cut walk 60 m: 0 white px (> 20 = rendered)
- cut walk 100 m: 0 white px (> 20 = rendered)
- measured far-render cut: none m (G4 flap instrument; the decode proof needs >= 83 m from the y=151.6 eye)
- fill issued: 5x111x5 red_concrete at -930,796 (zone forceloaded ±48)
- phase: 4 marker filled (19:01:57)

### /clod-cells AFTER FILL (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 34 0 0x8400 0x2000
123 122 -940 784 71 0 1 [97, 119, 97] 32 0 0x8700 0x2000
124 122 -938 784 68 0 1 [97, 119, 97] 33 0 0x8400 0x2000
125 122 -936 784 62 1 1 [63, 118, 228] 73 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 72 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 72 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 71 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 356 0 0x8000 0x4100
130 122 -926 784 66 0 1 [134, 183, 131] 355 0 0x8200 0x4100
131 122 -924 784 75 0 1 [97, 119, 97] 358 0 0x8b00 0x2000
132 122 -922 784 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 122 -920 784 75 0 1 [97, 119, 97] 18446744073709551615 1 0x8b00 0x2000
134 122 -918 784 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x2000
122 123 -942 786 70 0 1 [97, 119, 97] 34 0 0x8600 0x2000
123 123 -940 786 65 0 1 [97, 119, 97] 32 0 0x8100 0x2000
124 123 -938 786 65 0 1 [97, 119, 97] 33 0 0x8100 0x2000
125 123 -936 786 62 1 1 [63, 118, 228] 73 0 0x7e00 0xd800
126 123 -934 786 62 1 1 [63, 118, 228] 72 0 0x7e00 0xd800
127 123 -932 786 62 1 1 [63, 118, 228] 72 0 0x7e00 0xd800
128 123 -930 786 62 1 1 [63, 118, 228] 71 0 0x7e00 0xd800
129 123 -928 786 73 0 1 [97, 119, 97] 356 0 0x8900 0x2000
130 123 -926 786 67 0 1 [134, 183, 131] 355 0 0x8300 0x4100
131 123 -924 786 69 0 1 [134, 183, 131] 358 0 0x8500 0x4100
132 123 -922 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 123 -920 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 123 -918 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
122 124 -942 788 72 0 1 [97, 119, 97] 34 0 0x8800 0x2000
123 124 -940 788 70 0 1 [97, 119, 97] 32 0 0x8600 0x2000
124 124 -938 788 68 0 1 [97, 119, 97] 33 0 0x8400 0x2000
125 124 -936 788 62 1 1 [63, 118, 228] 73 0 0x7e00 0xd800
126 124 -934 788 62 1 1 [63, 118, 228] 72 0 0x7e00 0xd800
127 124 -932 788 62 1 1 [63, 118, 228] 72 0 0x7e00 0xd800
128 124 -930 788 73 0 1 [97, 119, 97] 31 0 0x8900 0x2000
129 124 -928 788 76 0 1 [97, 119, 97] 356 0 0x8c00 0x2000
130 124 -926 788 73 0 1 [97, 119, 97] 355 0 0x8900 0x2000
131 124 -924 788 69 0 1 [134, 183, 131] 358 0 0x8500 0x4100
132 124 -922 788 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 124 -920 788 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 124 -918 788 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x2000
122 125 -942 790 74 0 1 [97, 119, 97] 34 0 0x8a00 0x2000
123 125 -940 790 73 0 1 [97, 119, 97] 32 0 0x8900 0x2000
124 125 -938 790 71 0 1 [97, 119, 97] 33 0 0x8700 0x2000
125 125 -936 790 62 1 1 [63, 118, 228] 73 0 0x7e00 0xd800
126 125 -934 790 62 1 1 [63, 118, 228] 72 0 0x7e00 0xd800
127 125 -932 790 62 1 1 [63, 118, 228] 72 0 0x7e00 0xd800
128 125 -930 790 62 1 1 [63, 118, 228] 71 0 0x7e00 0xd800
129 125 -928 790 73 0 1 [97, 119, 97] 31 0 0x8900 0x2000
130 125 -926 790 68 0 1 [134, 183, 131] 355 0 0x8400 0x4100
131 125 -924 790 69 0 1 [134, 183, 131] 358 0 0x8500 0x4100
132 125 -922 790 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
133 125 -920 790 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 125 -918 790 70 0 1 [134, 183, 131] 18446744073709551615 1 0x8600 0x4100
122 126 -942 792 74 0 1 [97, 119, 97] 34 0 0x8a00 0x2000
123 126 -940 792 73 0 1 [97, 119, 97] 32 0 0x8900 0x2000
124 126 -938 792 68 0 1 [97, 119, 97] 33 0 0x8400 0x2000
125 126 -936 792 62 1 1 [63, 118, 228] 73 0 0x7e00 0xd800
126 126 -934 792 62 1 1 [63, 118, 228] 72 0 0x7e00 0xd800
127 126 -932 792 62 1 1 [63, 118, 228] 72 0 0x7e00 0xd800
128 126 -930 792 62 1 1 [63, 118, 228] 71 0 0x7e00 0xd800
129 126 -928 792 67 0 1 [134, 183, 131] 31 0 0x8300 0x4100
130 126 -926 792 74 0 1 [97, 119, 97] 355 0 0x8a00 0x2000
131 126 -924 792 77 0 1 [97, 119, 97] 358 0 0x8d00 0x2000
132 126 -922 792 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 126 -920 792 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3200
134 126 -918 792 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x5000
122 127 -942 794 67 0 1 [134, 183, 131] 34 0 0x8300 0x4100
123 127 -940 794 68 0 1 [134, 183, 131] 32 0 0x8400 0x4100
124 127 -938 794 65 0 1 [134, 183, 131] 32 0 0x8100 0x4100
125 127 -936 794 62 1 1 [63, 118, 228] 73 0 0x7e00 0xd800
126 127 -934 794 62 1 1 [63, 118, 228] 72 0 0x7e00 0xd800
127 127 -932 794 170 0 1 [142, 33, 33] 72 1 0xea00 0x0a00
128 127 -930 794 170 0 1 [142, 33, 33] 71 1 0xea00 0x0a00
129 127 -928 794 170 0 1 [142, 33, 33] 71 1 0xea00 0x0a00
130 127 -926 794 74 0 1 [97, 119, 97] 355 0 0x8a00 0x2000
131 127 -924 794 77 0 1 [97, 119, 97] 358 0 0x8d00 0x2000
132 127 -922 794 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 127 -920 794 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3200
134 127 -918 794 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x5000
122 128 -942 796 68 0 1 [134, 183, 131] 35 0 0x8400 0x4100
123 128 -940 796 72 0 1 [97, 119, 97] 32 0 0x8800 0x2000
124 128 -938 796 75 0 1 [97, 119, 97] 32 0 0x8b00 0x2000
125 128 -936 796 72 0 1 [97, 119, 97] 33 0 0x8800 0x2000
126 128 -934 796 62 1 1 [63, 118, 228] 72 0 0x7e00 0xd800
127 128 -932 796 170 0 1 [142, 33, 33] 72 1 0xea00 0x0a00
128 128 -930 796 170 0 1 [142, 33, 33] 71 1 0xea00 0x0a00
129 128 -928 796 170 0 1 [142, 33, 33] 71 1 0xea00 0x0a00
130 128 -926 796 68 0 1 [134, 183, 131] 355 0 0x8400 0x4100
131 128 -924 796 74 0 1 [97, 119, 97] 358 0 0x8a00 0x2000
132 128 -922 796 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 128 -920 796 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
134 128 -918 796 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 129 -942 798 69 0 1 [134, 183, 131] 34 0 0x8500 0x4100
123 129 -940 798 75 0 1 [97, 119, 97] 32 0 0x8b00 0x2000
124 129 -938 798 78 0 1 [97, 119, 97] 35 0 0x8e00 0x2000
125 129 -936 798 75 0 1 [97, 119, 97] 33 0 0x8b00 0x2000
126 129 -934 798 62 1 1 [63, 118, 228] 72 0 0x7e00 0xd800
127 129 -932 798 170 0 1 [142, 33, 33] 72 1 0xea00 0x0a00
128 129 -930 798 170 0 1 [142, 33, 33] 71 1 0xea00 0x0a00
129 129 -928 798 170 0 1 [142, 33, 33] 31 1 0xea00 0x0a00
130 129 -926 798 71 0 1 [97, 119, 97] 355 0 0x8700 0x2000
131 129 -924 798 69 0 1 [134, 183, 131] 358 0 0x8500 0x4100
132 129 -922 798 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 129 -920 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 129 -918 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 130 -942 800 70 0 1 [134, 183, 131] 34 0 0x8600 0x4100
123 130 -940 800 76 0 1 [97, 119, 97] 32 0 0x8c00 0x2000
124 130 -938 800 76 0 1 [97, 119, 97] 33 0 0x8c00 0x2000
125 130 -936 800 72 0 1 [97, 119, 97] 33 0 0x8800 0x2000
126 130 -934 800 62 1 1 [63, 118, 228] 72 0 0x7e00 0xd800
127 130 -932 800 62 1 1 [63, 118, 228] 72 0 0x7e00 0xd800
128 130 -930 800 62 1 1 [63, 118, 228] 71 0 0x7e00 0xd800
129 130 -928 800 71 0 1 [97, 119, 97] 31 0 0x8700 0x2000
130 130 -926 800 71 0 1 [97, 119, 97] 355 0 0x8700 0x2000
131 130 -924 800 69 0 1 [134, 183, 131] 358 0 0x8500 0x4100
132 130 -922 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
133 130 -920 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 130 -918 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 131 -942 802 78 0 1 [97, 119, 97] 34 0 0x8e00 0x2000
123 131 -940 802 77 0 1 [97, 119, 97] 32 0 0x8d00 0x2000
124 131 -938 802 77 0 1 [97, 119, 97] 33 0 0x8d00 0x2000
125 131 -936 802 66 0 1 [134, 183, 131] 33 0 0x8200 0x4100
126 131 -934 802 62 1 1 [63, 118, 228] 72 0 0x7e00 0xd800
127 131 -932 802 62 1 1 [63, 118, 228] 72 0 0x7e00 0xd800
128 131 -930 802 62 1 1 [63, 118, 228] 71 0 0x7e00 0xd800
129 131 -928 802 62 1 1 [63, 118, 228] 399 0 0x7e00 0xd800
130 131 -926 802 66 0 1 [134, 183, 131] 355 0 0x8200 0x4100
131 131 -924 802 68 0 1 [134, 183, 131] 358 0 0x8400 0x4100
132 131 -922 802 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 131 -920 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 131 -918 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 132 -942 804 79 0 1 [97, 119, 97] 34 0 0x8f00 0x2000
123 132 -940 804 76 0 1 [97, 119, 97] 32 0 0x8c00 0x2000
124 132 -938 804 76 0 1 [97, 119, 97] 33 0 0x8c00 0x2000
125 132 -936 804 76 0 1 [97, 119, 97] 33 0 0x8c00 0x2000
126 132 -934 804 62 1 1 [63, 118, 228] 72 0 0x7e00 0xd800
127 132 -932 804 62 1 1 [63, 118, 228] 72 0 0x7e00 0xd800
128 132 -930 804 62 1 1 [63, 118, 228] 71 0 0x7e00 0xd800
129 132 -928 804 62 1 1 [63, 118, 228] 399 0 0x7e00 0xd800
130 132 -926 804 65 0 1 [134, 183, 131] 355 0 0x8100 0x4100
131 132 -924 804 77 0 1 [97, 119, 97] 358 0 0x8d00 0x2000
132 132 -922 804 79 0 1 [97, 119, 97] 356 0 0x8f00 0x2000
133 132 -920 804 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 132 -918 804 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x2000
122 133 -942 806 78 0 1 [97, 119, 97] 34 0 0x8e00 0x2000
123 133 -940 806 69 0 1 [134, 183, 131] 32 0 0x8500 0x4100
124 133 -938 806 155 0 1 [134, 183, 131] 34 0 0xdb00 0x4100
125 133 -936 806 155 0 1 [134, 183, 131] 32 0 0xdb00 0x4100
126 133 -934 806 155 0 1 [134, 183, 131] 33 0 0xdb00 0x4100
127 133 -932 806 155 0 1 [134, 183, 131] 33 0 0xdb00 0x4100
128 133 -930 806 155 0 1 [134, 183, 131] 30 0 0xdb00 0x4100
129 133 -928 806 155 0 1 [134, 183, 131] 31 0 0xdb00 0x4100
130 133 -926 806 155 0 1 [134, 183, 131] 31 0 0xdb00 0x4100
131 133 -924 806 67 0 1 [134, 183, 131] 358 0 0x8300 0x4100
132 133 -922 806 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 133 -920 806 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 133 -918 806 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x2000
122 134 -942 808 79 0 1 [97, 119, 97] 34 0 0x8f00 0x2000
123 134 -940 808 68 0 1 [134, 183, 131] 32 0 0x8400 0x4100
124 134 -938 808 155 0 1 [134, 183, 131] 34 0 0xdb00 0x4100
125 134 -936 808 180 0 1 [134, 183, 131] 31 0 0xf400 0x4100
126 134 -934 808 180 0 1 [134, 183, 131] 31 0 0xf400 0x4100
127 134 -932 808 180 0 1 [134, 183, 131] 356 0 0xf400 0x4100
128 134 -930 808 180 0 1 [134, 183, 131] 357 0 0xf400 0x4100
129 134 -928 808 180 0 1 [134, 183, 131] 358 0 0xf400 0x4100
130 134 -926 808 155 0 1 [134, 183, 131] 31 0 0xdb00 0x4100
131 134 -924 808 67 0 1 [134, 183, 131] 358 0 0x8300 0x4100
132 134 -922 808 68 0 1 [134, 183, 131] 18446744073709551615 1 0x8400 0x4100
133 134 -920 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 134 -918 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
```
### stream diag after fill:
[c-LoD] stream: tick 3400 65536/65536 cells, lap 706, cone 16189 zone 1522 dirty 928 (q 10307) ewin 213 rbeat 0p/0 rearm 0c/0t rescan 857376c/3672e, edits +6170p/4671c/685e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 378 forced 134, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 597323p/2011b fail 0 ~73 kB/s cache 13603/24576 h583720 m13603 e0, srbeat 56197s/0q drop 70stale/92undec, ereb 48150s/99q, sweep 0r/78d
[19:01:59 INFO]: [c-LoD] stream: tick 3400 65536/65536 cells, lap 706, cone 16189 zone 1522 dirty 928 (q 10307) ewin 213 rbeat 0p/0 rearm 0c/0t rescan 857376c/3672e, edits +6170p/4671c/685e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 378 forced 134, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 597323p/2011b fail 0 ~73 kB/s cache 13603/24576 h583720 m13603 e0, srbeat 56197s/0q drop 70stale/92undec, ereb 48150s/99q, sweep 0r/78d
- carrier ladder y=155: 16716 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=165: 16716 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=168: 16716 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=172: 16716 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=176: 16716 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=180: 16716 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=185: 16716 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- probe 0: red=0 paint=17391 lod_silhouette_cols=41 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=3 hits_hi=11 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 1: red=0 paint=17391 lod_silhouette_cols=41 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=3 hits_hi=11 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 2: red=0 paint=18856 lod_silhouette_cols=41 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=3 hits_hi=11 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 3: red=0 paint=19351 lod_silhouette_cols=41 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=3 hits_hi=11 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 4: red=0 paint=19089 lod_silhouette_cols=41 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=3 hits_hi=11 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 5: red=0 paint=19211 lod_silhouette_cols=41 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=3 hits_hi=11 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 6: red=0 paint=19639 lod_silhouette_cols=41 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=3 hits_hi=11 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 7: red=0 paint=19534 lod_silhouette_cols=41 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=3 hits_hi=11 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 8: red=0 paint=20214 lod_silhouette_cols=41 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=3 hits_hi=11 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 9: red=0 paint=19570 lod_silhouette_cols=41 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=3 hits_hi=11 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 10: red=0 paint=19540 lod_silhouette_cols=41 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=3 hits_hi=11 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 11: red=0 paint=19540 lod_silhouette_cols=41 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=3 hits_hi=11 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 12: red=0 paint=19540 lod_silhouette_cols=41 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=3 hits_hi=11 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 13: red=0 paint=19540 lod_silhouette_cols=41 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=3 hits_hi=11 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 14: red=0 paint=20455 lod_silhouette_cols=41 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=3 hits_hi=11 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 15: red=0 paint=19791 lod_silhouette_cols=41 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=3 hits_hi=11 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 16: red=0 paint=20018 lod_silhouette_cols=41 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=3 hits_hi=11 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 17: red=0 paint=20311 lod_silhouette_cols=41 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=3 hits_hi=11 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 18: red=0 paint=20321 lod_silhouette_cols=41 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=3 hits_hi=11 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 19: red=0 paint=22050 lod_silhouette_cols=41 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=3 hits_hi=11 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 20: red=0 paint=20881 lod_silhouette_cols=41 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=3 hits_hi=11 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 21: red=0 paint=20990 lod_silhouette_cols=41 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=3 hits_hi=11 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 22: red=0 paint=21537 lod_silhouette_cols=41 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=3 hits_hi=11 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 23: red=0 paint=21184 lod_silhouette_cols=41 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=3 hits_hi=11 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 24: red=0 paint=22061 lod_silhouette_cols=41 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=3 hits_hi=11 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 25: red=0 paint=20871 lod_silhouette_cols=41 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=3 hits_hi=11 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 26: red=0 paint=20958 lod_silhouette_cols=41 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=3 hits_hi=11 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 27: red=0 paint=21480 lod_silhouette_cols=41 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=3 hits_hi=11 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 28: red=0 paint=21244 lod_silhouette_cols=41 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=3 hits_hi=11 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 29: red=0 paint=21154 lod_silhouette_cols=41 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=3 hits_hi=11 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 30: red=0 paint=20917 lod_silhouette_cols=41 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=3 hits_hi=11 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 31: red=0 paint=21290 lod_silhouette_cols=41 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=3 hits_hi=11 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 32: red=0 paint=24930 lod_silhouette_cols=41 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=3 hits_hi=11 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 33: red=0 paint=25262 lod_silhouette_cols=41 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=3 hits_hi=11 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 34: red=0 paint=24696 lod_silhouette_cols=41 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=3 hits_hi=11 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 35: red=0 paint=24485 lod_silhouette_cols=41 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=3 hits_hi=11 mask_hits=30 (need red>=60 paint>=300 lod>20)
- phase: 5 leg-1 probes done (19:07:40)
- **FAIL:** leg-1 never saw the marker in 24 probes (red < 60 everywhere) — the edit never reached the LOD paint; this failure was SILENT before v6.31.1
### stream diag after probes:
[19:07:20 INFO]: [c-LoD] stream: tick 6600 65536/65536 cells, lap 894, cone 40936 zone 3288 dirty 978 (q 10338) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1891383c/3737e, edits +7666p/5719c/1022e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 378 forced 134, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 931841p/2623b fail 0 ~73 kB/s cache 13705/24576 h918136 m13705 e0, srbeat 71711s/0q drop 76stale/92undec, ereb 49413s/0q, sweep 0r/14d
[c-LoD] stream: tick 6700 65536/65536 cells, lap 903, cone 42088 zone 3288 dirty 978 (q 10339) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1925195c/3737e, edits +7668p/5721c/1023e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 378 forced 134, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 945825p/2642b fail 0 ~73 kB/s cache 13705/24576 h932120 m13705 e0, srbeat 72319s/0q drop 76stale/92undec, ereb 49413s/0q, sweep 0r/23d
[19:07:30 INFO]: [c-LoD] stream: tick 6700 65536/65536 cells, lap 903, cone 42088 zone 3288 dirty 978 (q 10339) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1925195c/3737e, edits +7668p/5721c/1023e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 378 forced 134, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 945825p/2642b fail 0 ~73 kB/s cache 13705/24576 h932120 m13705 e0, srbeat 72319s/0q drop 76stale/92undec, ereb 49413s/0q, sweep 0r/23d
- parity debug tour: 13 vantages (session-1 client, DEBUG pack — evidence only, the gate pairs the §7b world session)
- parity debug tour done: 13 shots in parity_dbg/ (the §7b world session feeds the gate)
- joined session 2 (19:10:28)
- leg-2 baseline scene px (x<940): 0 (0 = the relog client paints no world at all)
- leg-2 renderer probe NEAR (20 m) white px: 578 raw / 578 changed-vs-baseline (> 20 = the client renders particles)
- leg-2 renderer probe FAR (80 m, marker zone) white px: 500 raw / 500 changed-vs-baseline (> 20 = no distance cut; ~0 with NEAR alive = particles beyond the client chunk radius do not render)
- leg-2 carrier probe green dust (80 m): changed-green px 0 / total changed 111 vs baseline (> 100 green = a stable green carrier exists; total > 0 with green ~0 = the dust renders but fog washes it — a ruler problem, not a carrier problem)
- phase: 8 leg-2 relog probes (19:15:51)
- LEG-2 SCENE DEAD: the relog client paints ZERO scene pixels (x<940) — pure sky + lens panels; the fresh-session capture chain never commits (renderer-dead class, #227/#228); these probes measure a worldless client, not the transport
- relog probe 0: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=0 mask_hits=0 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 1: red=0 scene=104300 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=3 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 2: red=0 scene=111718 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=5 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 3: red=0 scene=122150 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=5 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 4: red=0 scene=134273 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=5 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 5: red=0 scene=133796 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=5 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 6: red=0 scene=131605 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=6 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 7: red=0 scene=131933 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=6 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 8: red=0 scene=132188 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=6 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 9: red=0 scene=128587 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=6 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 10: red=0 scene=129727 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=6 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 11: red=0 scene=129599 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=6 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 12: red=0 scene=128129 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=6 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 13: red=0 scene=128325 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=6 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 14: red=0 scene=128182 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=2 hits_hi=6 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 15: red=0 scene=129713 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=2 hits_hi=6 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 16: red=0 scene=130751 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=2 hits_hi=6 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 17: red=0 scene=130242 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=2 hits_hi=6 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 18: red=0 scene=130523 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=2 hits_hi=6 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 19: red=0 scene=130079 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=2 hits_hi=6 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- **FAIL:** marker lost after relog — stream state is not persistent/deterministic
- phase: 9 leg-2 done (19:18:44)

### client log forensics (pipeline/particle/atlas — v6.8.2)
```
[19:10:19] [main/DEBUG]: Loading LWJGL system
WARNING: java.lang.System::load has been called by org.lwjgl.system.Library$$Lambda/0x00000000271e3d60 in an unnamed module (file:/home/runner/mc-test/client/libraries/org/lwjgl/lwjgl/3.4.3/lwjgl-3.4.3.jar)
WARNING: sun.misc.Unsafe::objectFieldOffset has been called by org.lwjgl.system.MemoryBackendUnsafeLegacy (file:/home/runner/mc-test/client/libraries/org/lwjgl/lwjgl/3.4.3/lwjgl-3.4.3.jar)
WARNING: Please consider reporting this to the maintainers of class org.lwjgl.system.MemoryBackendUnsafeLegacy
[19:10:19] [main/DEBUG]: Loading OpenGL
[19:10:20] [main/DEBUG]: Loading shaderc
[19:10:23] [Render thread/ERROR]: Error parsing option value 4 for option Simulation Distance: Value 4 outside of range [5:33]
[19:10:23] [Render thread/INFO]: Backend library: LWJGL version 3.4.3+4
[19:10:23] [Render thread/INFO]: Using graphics backend OpenGL, using drivers: 4.5 (Core Profile) Mesa 26.0.8-1ubuntu0.3
[19:10:23] [Render thread/INFO]: Using graphics device: llvmpipe (LLVM 21.1.8, 256 bits) (Mesa)
[19:10:23] [Render thread/INFO]: Using graphics device extensions: GL_ARB_multi_draw_indirect, GL_ARB_buffer_storage, GL_ARB_shader_draw_parameters, GL_ARB_base_instance, GL_KHR_debug, GL_ARB_draw_indirect, GL_ARB_clip_control, GL_ARB_vertex_attrib_binding, GL_ARB_direct_state_access, GL_EXT_texture_filter_anisotropic
[19:10:23] [Download-2/ERROR]: Failed to fetch user properties
com.mojang.authlib.exceptions.InvalidCredentialsException: Status: 401
	at com.mojang.authlib.exceptions.MinecraftClientHttpException.toAuthenticationException(MinecraftClientHttpException.java:70)
Caused by: MinecraftClientHttpException[type=HTTP_ERROR, status=401, response=ErrorResponse[path=/player/attributes, error=null, errorMessage=null, details=null]]
	at com.mojang.authlib.minecraft.client.MinecraftClient.buildHttpException(MinecraftClient.java:170)
[19:10:23] [Download-1/ERROR]: Got an error with a html body connecting to https://sessionserver.mojang.com/session/minecraft/profile/d1652ad977ce32009642f973c97adf36?unsigned=false: <!DOCTYPE html PUBLIC '-//W3C//DTD XHTML 1.0 Transitional//EN' 'http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd'>
        #errorref {
MinecraftClientHttpException[type=HTTP_ERROR, status=429, response=null]
	at com.mojang.authlib.minecraft.client.MinecraftClient.buildHttpException(MinecraftClient.java:167)
[19:10:24] [Render thread/INFO]: Reloading ResourceManager: vanilla, vanilla
[19:10:24] [Download-1/INFO]: Could not authorize you against Realms server: java.lang.RuntimeException: Failed to parse into SignedJWT: 0
[19:10:24] [Download-1/ERROR]: Failed to fetch Realms feature flags
com.mojang.realmsclient.exception.RealmsServiceException: Realms authentication error with message 'java.lang.RuntimeException: Failed to parse into SignedJWT: 0'
[19:10:26] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[19:10:26] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[19:10:26] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[19:10:26] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[19:10:26] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[19:10:26] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[19:10:26] [Render thread/INFO]: Created: 2048x2048x4 minecraft:textures/atlas/blocks.png-atlas
[19:10:26] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[19:10:26] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[19:10:26] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[19:10:26] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[19:10:26] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
[19:10:32] [Render thread/INFO]: Reloading ResourceManager: vanilla, vanilla, server/00000000/6b1f2f7e-9c0d-4a4b-8b0a-b10d1b4ab100
[19:10:34] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[19:10:34] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[19:10:34] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[19:10:34] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[19:10:34] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[19:10:34] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[19:10:34] [Render thread/INFO]: Created: 2048x2048x4 minecraft:textures/atlas/blocks.png-atlas
[19:10:34] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[19:10:34] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[19:10:34] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[19:10:34] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[19:10:34] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
```

### /clod-cells AFTER PROBES (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 20 0 0x8400 0x2000
123 122 -940 784 71 0 1 [97, 119, 97] 20 0 0x8700 0x2000
124 122 -938 784 68 0 1 [97, 119, 97] 19 0 0x8400 0x2000
125 122 -936 784 62 1 1 [63, 118, 228] 19 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 18 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 18 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 17 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 301 0 0x8000 0x4100
130 122 -926 784 66 0 1 [134, 183, 131] 301 0 0x8200 0x4100
131 122 -924 784 75 0 1 [97, 119, 97] 301 0 0x8b00 0x2000
132 122 -922 784 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 122 -920 784 75 0 1 [97, 119, 97] 18446744073709551615 1 0x8b00 0x2000
134 122 -918 784 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x2000
122 123 -942 786 70 0 1 [97, 119, 97] 20 0 0x8600 0x2000
123 123 -940 786 65 0 1 [97, 119, 97] 20 0 0x8100 0x2000
124 123 -938 786 65 0 1 [97, 119, 97] 19 0 0x8100 0x2000
125 123 -936 786 62 1 1 [63, 118, 228] 19 0 0x7e00 0xd800
126 123 -934 786 62 1 1 [63, 118, 228] 18 0 0x7e00 0xd800
127 123 -932 786 62 1 1 [63, 118, 228] 18 0 0x7e00 0xd800
128 123 -930 786 62 1 1 [63, 118, 228] 17 0 0x7e00 0xd800
129 123 -928 786 73 0 1 [97, 119, 97] 301 0 0x8900 0x2000
130 123 -926 786 67 0 1 [134, 183, 131] 301 0 0x8300 0x4100
131 123 -924 786 69 0 1 [134, 183, 131] 301 0 0x8500 0x4100
132 123 -922 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 123 -920 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 123 -918 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
122 124 -942 788 72 0 1 [97, 119, 97] 20 0 0x8800 0x2000
123 124 -940 788 70 0 1 [97, 119, 97] 20 0 0x8600 0x2000
124 124 -938 788 68 0 1 [97, 119, 97] 19 0 0x8400 0x2000
125 124 -936 788 62 1 1 [63, 118, 228] 19 0 0x7e00 0xd800
126 124 -934 788 62 1 1 [63, 118, 228] 18 0 0x7e00 0xd800
127 124 -932 788 62 1 1 [63, 118, 228] 18 0 0x7e00 0xd800
128 124 -930 788 73 0 1 [97, 119, 97] 17 0 0x8900 0x2000
129 124 -928 788 76 0 1 [97, 119, 97] 301 0 0x8c00 0x2000
130 124 -926 788 73 0 1 [97, 119, 97] 301 0 0x8900 0x2000
131 124 -924 788 69 0 1 [134, 183, 131] 311 0 0x8500 0x4100
132 124 -922 788 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 124 -920 788 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 124 -918 788 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x2000
122 125 -942 790 74 0 1 [97, 119, 97] 20 0 0x8a00 0x2000
123 125 -940 790 73 0 1 [97, 119, 97] 20 0 0x8900 0x2000
124 125 -938 790 71 0 1 [97, 119, 97] 19 0 0x8700 0x2000
125 125 -936 790 62 1 1 [63, 118, 228] 19 0 0x7e00 0xd800
126 125 -934 790 62 1 1 [63, 118, 228] 18 0 0x7e00 0xd800
127 125 -932 790 62 1 1 [63, 118, 228] 18 0 0x7e00 0xd800
128 125 -930 790 62 1 1 [63, 118, 228] 17 0 0x7e00 0xd800
129 125 -928 790 73 0 1 [97, 119, 97] 17 0 0x8900 0x2000
130 125 -926 790 68 0 1 [134, 183, 131] 311 0 0x8400 0x4100
131 125 -924 790 69 0 1 [134, 183, 131] 311 0 0x8500 0x4100
132 125 -922 790 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
133 125 -920 790 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 125 -918 790 70 0 1 [134, 183, 131] 18446744073709551615 1 0x8600 0x4100
122 126 -942 792 74 0 1 [97, 119, 97] 20 0 0x8a00 0x2000
123 126 -940 792 73 0 1 [97, 119, 97] 20 0 0x8900 0x2000
124 126 -938 792 68 0 1 [97, 119, 97] 19 0 0x8400 0x2000
125 126 -936 792 62 1 1 [63, 118, 228] 19 0 0x7e00 0xd800
126 126 -934 792 62 1 1 [63, 118, 228] 18 0 0x7e00 0xd800
127 126 -932 792 62 1 1 [63, 118, 228] 18 0 0x7e00 0xd800
128 126 -930 792 62 1 1 [63, 118, 228] 17 0 0x7e00 0xd800
129 126 -928 792 67 0 1 [134, 183, 131] 17 0 0x8300 0x4100
130 126 -926 792 74 0 1 [97, 119, 97] 311 0 0x8a00 0x2000
131 126 -924 792 77 0 1 [97, 119, 97] 311 0 0x8d00 0x2000
132 126 -922 792 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 126 -920 792 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3200
134 126 -918 792 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x5000
122 127 -942 794 67 0 1 [134, 183, 131] 21 0 0x8300 0x4100
123 127 -940 794 68 0 1 [134, 183, 131] 20 0 0x8400 0x4100
124 127 -938 794 65 0 1 [134, 183, 131] 19 0 0x8100 0x4100
125 127 -936 794 62 1 1 [63, 118, 228] 19 0 0x7e00 0xd800
126 127 -934 794 62 1 1 [63, 118, 228] 18 0 0x7e00 0xd800
127 127 -932 794 170 0 1 [142, 33, 33] 18 0 0xea00 0x0a00
128 127 -930 794 170 0 1 [142, 33, 33] 17 0 0xea00 0x0a00
129 127 -928 794 170 0 1 [142, 33, 33] 17 0 0xea00 0x0a00
130 127 -926 794 74 0 1 [97, 119, 97] 311 0 0x8a00 0x2000
131 127 -924 794 77 0 1 [97, 119, 97] 311 0 0x8d00 0x2000
132 127 -922 794 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 127 -920 794 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3200
134 127 -918 794 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x5000
122 128 -942 796 68 0 1 [134, 183, 131] 21 0 0x8400 0x4100
123 128 -940 796 72 0 1 [97, 119, 97] 20 0 0x8800 0x2000
124 128 -938 796 75 0 1 [97, 119, 97] 19 0 0x8b00 0x2000
125 128 -936 796 72 0 1 [97, 119, 97] 19 0 0x8800 0x2000
126 128 -934 796 62 1 1 [63, 118, 228] 18 0 0x7e00 0xd800
127 128 -932 796 170 0 1 [142, 33, 33] 18 0 0xea00 0x0a00
128 128 -930 796 170 0 1 [142, 33, 33] 17 0 0xea00 0x0a00
129 128 -928 796 170 0 1 [142, 33, 33] 17 0 0xea00 0x0a00
130 128 -926 796 68 0 1 [134, 183, 131] 311 0 0x8400 0x4100
131 128 -924 796 74 0 1 [97, 119, 97] 311 0 0x8a00 0x2000
132 128 -922 796 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 128 -920 796 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
134 128 -918 796 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 129 -942 798 69 0 1 [134, 183, 131] 21 0 0x8500 0x4100
123 129 -940 798 75 0 1 [97, 119, 97] 20 0 0x8b00 0x2000
124 129 -938 798 78 0 1 [97, 119, 97] 21 0 0x8e00 0x2000
125 129 -936 798 75 0 1 [97, 119, 97] 19 0 0x8b00 0x2000
126 129 -934 798 62 1 1 [63, 118, 228] 18 0 0x7e00 0xd800
127 129 -932 798 170 0 1 [142, 33, 33] 18 0 0xea00 0x0a00
128 129 -930 798 170 0 1 [142, 33, 33] 17 0 0xea00 0x0a00
129 129 -928 798 170 0 1 [142, 33, 33] 17 0 0xea00 0x0a00
130 129 -926 798 71 0 1 [97, 119, 97] 311 0 0x8700 0x2000
131 129 -924 798 69 0 1 [134, 183, 131] 311 0 0x8500 0x4100
132 129 -922 798 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 129 -920 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 129 -918 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 130 -942 800 70 0 1 [134, 183, 131] 20 0 0x8600 0x4100
123 130 -940 800 76 0 1 [97, 119, 97] 20 0 0x8c00 0x2000
124 130 -938 800 76 0 1 [97, 119, 97] 19 0 0x8c00 0x2000
125 130 -936 800 72 0 1 [97, 119, 97] 19 0 0x8800 0x2000
126 130 -934 800 62 1 1 [63, 118, 228] 18 0 0x7e00 0xd800
127 130 -932 800 62 1 1 [63, 118, 228] 18 0 0x7e00 0xd800
128 130 -930 800 62 1 1 [63, 118, 228] 17 0 0x7e00 0xd800
129 130 -928 800 71 0 1 [97, 119, 97] 17 0 0x8700 0x2000
130 130 -926 800 71 0 1 [97, 119, 97] 311 0 0x8700 0x2000
131 130 -924 800 69 0 1 [134, 183, 131] 311 0 0x8500 0x4100
132 130 -922 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
133 130 -920 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 130 -918 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 131 -942 802 78 0 1 [97, 119, 97] 22 0 0x8e00 0x2000
123 131 -940 802 77 0 1 [97, 119, 97] 20 0 0x8d00 0x2000
124 131 -938 802 77 0 1 [97, 119, 97] 19 0 0x8d00 0x2000
125 131 -936 802 66 0 1 [134, 183, 131] 19 0 0x8200 0x4100
126 131 -934 802 62 1 1 [63, 118, 228] 18 0 0x7e00 0xd800
127 131 -932 802 62 1 1 [63, 118, 228] 18 0 0x7e00 0xd800
128 131 -930 802 62 1 1 [63, 118, 228] 17 0 0x7e00 0xd800
129 131 -928 802 62 1 1 [63, 118, 228] 352 0 0x7e00 0xd800
130 131 -926 802 66 0 1 [134, 183, 131] 311 0 0x8200 0x4100
131 131 -924 802 68 0 1 [134, 183, 131] 311 0 0x8400 0x4100
132 131 -922 802 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 131 -920 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 131 -918 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 132 -942 804 79 0 1 [97, 119, 97] 22 0 0x8f00 0x2000
123 132 -940 804 76 0 1 [97, 119, 97] 20 0 0x8c00 0x2000
124 132 -938 804 76 0 1 [97, 119, 97] 19 0 0x8c00 0x2000
125 132 -936 804 76 0 1 [97, 119, 97] 19 0 0x8c00 0x2000
126 132 -934 804 62 1 1 [63, 118, 228] 18 0 0x7e00 0xd800
127 132 -932 804 62 1 1 [63, 118, 228] 18 0 0x7e00 0xd800
128 132 -930 804 62 1 1 [63, 118, 228] 17 0 0x7e00 0xd800
129 132 -928 804 62 1 1 [63, 118, 228] 352 0 0x7e00 0xd800
130 132 -926 804 65 0 1 [134, 183, 131] 311 0 0x8100 0x4100
131 132 -924 804 77 0 1 [97, 119, 97] 311 0 0x8d00 0x2000
132 132 -922 804 79 0 1 [97, 119, 97] 311 0 0x8f00 0x2000
133 132 -920 804 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 132 -918 804 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x2000
122 133 -942 806 78 0 1 [97, 119, 97] 22 0 0x8e00 0x2000
123 133 -940 806 69 0 1 [134, 183, 131] 20 0 0x8500 0x4100
124 133 -938 806 155 0 1 [134, 183, 131] 20 0 0xdb00 0x4100
125 133 -936 806 155 0 1 [134, 183, 131] 20 0 0xdb00 0x4100
126 133 -934 806 155 0 1 [134, 183, 131] 19 0 0xdb00 0x4100
127 133 -932 806 155 0 1 [134, 183, 131] 19 0 0xdb00 0x4100
128 133 -930 806 155 0 1 [134, 183, 131] 18 0 0xdb00 0x4100
129 133 -928 806 155 0 1 [134, 183, 131] 17 0 0xdb00 0x4100
130 133 -926 806 155 0 1 [134, 183, 131] 17 0 0xdb00 0x4100
131 133 -924 806 67 0 1 [134, 183, 131] 311 0 0x8300 0x4100
132 133 -922 806 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 133 -920 806 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 133 -918 806 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x2000
122 134 -942 808 79 0 1 [97, 119, 97] 22 0 0x8f00 0x2000
123 134 -940 808 68 0 1 [134, 183, 131] 20 0 0x8400 0x4100
124 134 -938 808 155 0 1 [134, 183, 131] 20 0 0xdb00 0x4100
125 134 -936 808 180 0 1 [134, 183, 131] 17 0 0xf400 0x4100
126 134 -934 808 180 0 1 [134, 183, 131] 17 0 0xf400 0x4100
127 134 -932 808 180 0 1 [134, 183, 131] 311 0 0xf400 0x4100
128 134 -930 808 180 0 1 [134, 183, 131] 311 0 0xf400 0x4100
129 134 -928 808 180 0 1 [134, 183, 131] 311 0 0xf400 0x4100
130 134 -926 808 155 0 1 [134, 183, 131] 17 0 0xdb00 0x4100
131 134 -924 808 67 0 1 [134, 183, 131] 311 0 0x8300 0x4100
132 134 -922 808 68 0 1 [134, 183, 131] 18446744073709551615 1 0x8400 0x4100
133 134 -920 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 134 -918 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
```
- edit channel: 0 linkage errors, 0 failed event dispatches (both must be 0)
- phase: 6 parity world boot (19:18:44)
- parity world: view-distance=8 (the voxy leg's server value)
- parity world: joined session 3 (19:19:18)
- parity world: scan populated (cells=65536; the march paints from these buffers)
- parity world tour: 13 vantages (production pack, vd 8, shared structures)
- coverage sweeps: 72 eye passes = 36 positions (12 az x r144/200 + 12 off-az x r172, y=160) x 2 looks (outward+inward), settle-by-laps >= 6 s (q stable 3x1s), deadline 12 s/pos, no shots — the v6.26.0 data-coverage lever + v6.26.6 lottery kill; probe rides parity/view_probe.md
- parity drain-settle TIMEOUT: queue 14790 -> 14791 > 11500 after 150s — continuing anyway; the queue did not drain without eye proximity (priority-lane evidence, the 01/05/10 dip attribution reads from the tour deltas)
- pre-tour pass done: 13 vantages x 10s (the tour now reads the second OR-merge lap; the rim/dip transport race reads from the tour deltas)
- parity world tour done: 13 shots, 0 night-guard renames in parity/ (the gate pairs voxy/<name>.png vs parity/<name>.png)
- phase: 7 parity tour done (19:41:17)
- sparse re-beat channel at tour end: streamed=140300 q=0 dropped_stale=94 dropped_undecodable=10504 (frac=8 boot; undecodable-dominant = the rim words die in their decodability window, stale-dominant = edit churn owns the cells)
### parity world stream counters (session 3, tail + maxima)
[c-LoD] stream: tick 13200 65536/65536 cells, lap 1270, cone 65285 zone 22862 dirty 14402 (q 14771) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 9056044c/1384e, edits +18062p/13694c/2834e drop 0, spawned 0 / failed 0, cam0 yaw -180 pitch 5, wish 0 forced 901, native 29679c/948ch fail 0 (3w q 0) helper on, trees 948ch/fail 0, wire bundle 1896926p/5212b fail 0 ~0 kB/s cache 21907/24576 h1484654 m412272 e390365, srbeat 140300s/0q drop 94stale/10504undec, ereb 91027s/0q, sweep 0r/88d
[19:41:12 INFO]: [c-LoD] stream: tick 13200 65536/65536 cells, lap 1270, cone 65285 zone 22862 dirty 14402 (q 14771) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 9056044c/1384e, edits +18062p/13694c/2834e drop 0, spawned 0 / failed 0, cam0 yaw -180 pitch 5, wish 0 forced 901, native 29679c/948ch fail 0 (3w q 0) helper on, trees 948ch/fail 0, wire bundle 1896926p/5212b fail 0 ~0 kB/s cache 21907/24576 h1484654 m412272 e390365, srbeat 140300s/0q drop 94stale/10504undec, ereb 91027s/0q, sweep 0r/88d
- parity stream max cone: 65285
- parity stream max zone: 22862
- parity stream max wire bundle: bundle
### parity world capture lens (debug chain on the §7b world)
lens_red=0 lens_black=0 lens_water=0 lens_dark=0 lens_zero=0 rot=none scan=0 carriers=0
- parity mesh probe @png_y=2: x306=(151,184,255) x307=(151,184,255) x308=(151,184,255) x309h=(151,184,255) x310c=(151,184,255)
- parity mesh probe @png_y=717: x306=(44,62,51) x307=(65,89,72) x308=(81,107,87) x309h=(82,108,87) x310c=(84,111,90)
- parity mesh engagement: magenta-ish=0 of 921600 sampled px
- mask8: the un-fog engagement masks shot on the PARITY WORLD (vd 8) — parity_dbg8/01m|07m|10m
- mask8 01m: green=416 red=0 blue=98512 dim=14768 (px, sampled /16)
- mask8 07m: green=12064 red=0 blue=221376 dim=5808 (px, sampled /16)
- mask8 10m: green=3984 red=240 blue=231872 dim=138800 (px, sampled /16)
- mesh probe @png_y=2: x306=(255,255,255) x307=(13,254,0) x308=(0,254,0) x309h=(2,0,0) x310c=(0,0,0)
- mesh probe @png_y=717: x306=(255,255,255) x307=(13,254,0) x308=(0,254,0) x309h=(2,0,0) x310c=(0,0,0)
- mesh engagement: magenta-ish=0 of 921600 sampled px (the vanilla-path paint coverage)
- mask8 fog state @png_y=2: raw (300)=(151,184,255) (301)=(151,184,255)
- mask8 fog state @png_y=717: raw (300)=(0,128,8) (301)=(179,206,255)
- mask8 fog state decoded: envStart=0 envEnd=1024 rdEnd=128 fogColor=(179,206,255)
- buffer dump 10m: marked h=93/2048 c=8/2048; height windows r0<142:31 r1:8 r2>=202:54 maxByte=255
- buffer dump 10m: RANK-2-WINDOW HEIGHTS (the fake-wall class): (+36,-32)h=143, (+74,-32)h=191, (+76,-32)h=159, (+18,-30)h=188, (+74,-30)h=189, (+76,-30)h=191, (-4,-26)h=188, (+2,-26)h=144, (+66,-26)h=191, (+76,-26)h=191
- rim mask: scan populated (cells=65536; the state paint reads the same buffers the march reads)
- phase: 10 rim mask lane (19:45:23)
- rim mask: decode-state frames shot (rim_mask/00|01|12 — GREEN full decode / RED colorless / BLUE heightless; 00 = the leg-1 tower eye, 01 = the near-tower contrast, 12 = the tower re-read; the inherited 07/12 vantages sat 760 blocks outside the streamed disk and 01 sat INSIDE the marker column — their earlier verdicts were classifier noise, purged in v6.33.1)
- rim mask 00: green=21280 red=58960 blue=246464 yellow=28608 (px, sampled /16) -> DOMINANT BLUE: rim = HEIGHTS NEVER DECODED — a data-coverage class: the state paint cannot even see the cells
- rim mask 01: green=66720 red=23488 blue=290832 yellow=48320 (px, sampled /16) -> DOMINANT BLUE: rim = HEIGHTS NEVER DECODED — a data-coverage class: the state paint cannot even see the cells
- rim mask 12: green=42192 red=44192 blue=289712 yellow=45728 (px, sampled /16) -> DOMINANT BLUE: rim = HEIGHTS NEVER DECODED — a data-coverage class: the state paint cannot even see the cells

### boot / stream
```
[c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 146494 bytes, sha1 00ace27c12089a3bf32b909809eb5dd5ff8e656f — zero textures, zero world data, one push per join
[c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[19:44:42 INFO]: [c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 146494 bytes, sha1 00ace27c12089a3bf32b909809eb5dd5ff8e656f — zero textures, zero world data, one push per join
[19:44:42 INFO]: [c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[c-LoD] stream: tick 800 65536/65536 cells, lap 603, cone 9958 zone 881 dirty 22 (q 1748) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 493475c/5e, edits +12p/12c/6e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 238 forced 620, native 15006c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 129491p/382b fail 0 ~12 kB/s cache 14108/24576 h115383 m14108 e0, srbeat 6681s/216q drop 0stale/926undec, ereb 344s/27q, sweep 0r/10d
[19:46:12 INFO]: [c-LoD] stream: tick 800 65536/65536 cells, lap 603, cone 9958 zone 881 dirty 22 (q 1748) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 493475c/5e, edits +12p/12c/6e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 238 forced 620, native 15006c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 129491p/382b fail 0 ~12 kB/s cache 14108/24576 h115383 m14108 e0, srbeat 6681s/216q drop 0stale/926undec, ereb 344s/27q, sweep 0r/10d
```
### sessions
```
[19:45:01 INFO]: System chat: Stepan joined the game
[19:46:18 INFO]: System chat: Stepan left the game
```
### client log tail
```
[19:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[19:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[19:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[19:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[19:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[19:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[19:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[19:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[19:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[19:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[19:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[19:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[19:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[19:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[19:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[19:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[19:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[19:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[19:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[19:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[19:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[19:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[19:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[19:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[19:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[19:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[19:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[19:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[19:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[19:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[19:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[19:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[19:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[19:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[19:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[19:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[19:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[19:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[19:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[19:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[19:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[19:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[19:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[19:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[19:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[19:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[19:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[19:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[19:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[19:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[19:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[19:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[19:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[19:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[19:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[19:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[19:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[19:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[19:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[19:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[19:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[19:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[19:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[19:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[19:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[19:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[19:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[19:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[19:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[19:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[19:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[19:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[19:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[19:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[19:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[19:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[19:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[19:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[19:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:bundle_delimiter] net.minecraft.network.protocol.game.ClientboundBundleDelimiterPacket -> 1 bytes
[19:46:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
--- exceptions (Realms/RTKit noise filtered) ---
25:[19:44:56] [Render thread/ERROR]: Error parsing option value 4 for option Simulation Distance: Value 4 outside of range [5:33]
48:[19:44:57] [Download-2/ERROR]: Failed to fetch user properties
49:com.mojang.authlib.exceptions.InvalidCredentialsException: Status: 401
50:	at com.mojang.authlib.exceptions.MinecraftClientHttpException.toAuthenticationException(MinecraftClientHttpException.java:70)
57:Caused by: MinecraftClientHttpException[type=HTTP_ERROR, status=401, response=ErrorResponse[path=/player/attributes, error=null, errorMessage=null, details=null]]
58:	at com.mojang.authlib.minecraft.client.MinecraftClient.buildHttpException(MinecraftClient.java:170)
```
### server log tail
```
[19:46:19 INFO]: [ChunkHolderManager] Saved 999 block chunks, 980 entity chunks, 0 poi chunks in world 'minecraft:overworld' in 0.74s
[19:46:19 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:overworld'
[19:46:19 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:overworld'
[19:46:19 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[19:46:19 INFO]: [ChunkHolderManager] Waiting 60s for chunk system to halt for world 'minecraft:the_nether'
[19:46:19 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:the_nether'
[19:46:19 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_nether'
[19:46:19 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_nether' in 0.00s
[19:46:19 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_nether'
[19:46:19 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_nether'
[19:46:19 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[19:46:19 INFO]: [ChunkHolderManager] Waiting 60s for chunk system to halt for world 'minecraft:the_end'
[19:46:19 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:the_end'
[19:46:19 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_end'
[19:46:19 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_end' in 0.00s
[19:46:19 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_end'
[19:46:19 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_end'
[19:46:19 INFO]: Waiting for all RegionFile I/O tasks to complete...
[19:46:19 INFO]: All RegionFile I/O tasks to complete
[19:46:19 INFO]: [MoonriseCommon] Awaiting termination of worker pool for up to 60s...
[19:46:19 INFO]: [MoonriseCommon] Awaiting termination of I/O pool for up to 60s...
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[launcher] server exited with code 143
```
### shots
```
-rw-r--r-- 1 runner runner  12053 Nov 16 19:17 lens_13_after_relog_probe11.png
-rw-r--r-- 1 runner runner  12364 Nov 16 19:17 lens_13_after_relog_probe12.png
-rw-r--r-- 1 runner runner  12465 Nov 16 19:17 lens_13_after_relog_probe13.png
-rw-r--r-- 1 runner runner  12336 Nov 16 19:17 lens_13_after_relog_probe14.png
-rw-r--r-- 1 runner runner  11923 Nov 16 19:18 lens_13_after_relog_probe15.png
-rw-r--r-- 1 runner runner  12562 Nov 16 19:18 lens_13_after_relog_probe16.png
-rw-r--r-- 1 runner runner  12735 Nov 16 19:18 lens_13_after_relog_probe17.png
-rw-r--r-- 1 runner runner  12591 Nov 16 19:18 lens_13_after_relog_probe18.png
-rw-r--r-- 1 runner runner  12658 Nov 16 19:18 lens_13_after_relog_probe19.png
-rw-r--r-- 1 runner runner  11758 Nov 16 19:16 lens_13_after_relog_probe2.png
-rw-r--r-- 1 runner runner  11773 Nov 16 19:16 lens_13_after_relog_probe3.png
-rw-r--r-- 1 runner runner  12292 Nov 16 19:16 lens_13_after_relog_probe4.png
-rw-r--r-- 1 runner runner  11844 Nov 16 19:16 lens_13_after_relog_probe5.png
-rw-r--r-- 1 runner runner  12148 Nov 16 19:16 lens_13_after_relog_probe6.png
-rw-r--r-- 1 runner runner  12322 Nov 16 19:16 lens_13_after_relog_probe7.png
-rw-r--r-- 1 runner runner  12259 Nov 16 19:17 lens_13_after_relog_probe8.png
-rw-r--r-- 1 runner runner  12096 Nov 16 19:17 lens_13_after_relog_probe9.png
drwxr-xr-x 2 runner runner   4096 Nov 16 19:41 lens_parity
drwxr-xr-x 2 runner runner   4096 Nov 16 19:41 parity
drwxr-xr-x 2 runner runner   4096 Nov 16 19:10 parity_dbg
drwxr-xr-x 2 runner runner   4096 Nov 16 19:44 parity_dbg8
-rw-r--r-- 1 runner runner  12262 Nov 16 19:18 probe_counts.txt
drwxr-xr-x 2 runner runner   4096 Nov 16 19:46 rim_mask
-rw-r--r-- 1 runner runner  56230 Nov 16 19:46 server_full.log
-rw-r--r-- 1 runner runner  71407 Nov 16 19:46 summary.md
```

**exit=1** failures=2 leg1=0 leg2=0
