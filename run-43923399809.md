- phase: 0 sanity (17:56:46)
### c-LoD visual run (-930,796, marker=red_concrete, vantage -1005,150 yaw 270)
- GL:     Vendor: Mesa (0xffffffff);    Device: llvmpipe (LLVM 21.1.8, 256 bits) (0xffffffff);OpenGL renderer string: llvmpipe (LLVM 21.1.8, 256 bits);OpenGL core profile version string: 4.5 (Core Profile) Mesa 26.0.8-1ubuntu0.3;
- boot + gates OK (17:57:14)
- phase: 1 server booted (17:57:14)
- sky-grid armed
- phase: 2 leg-1 client joined (17:57:35)
- joined session 1 (17:57:35)
- stream live: cells=65536
- phase: 3 scan populated (17:57:58)
- terrain range: 2 terraced mountains at (CX-4,CZ-20)/(CX-2,CZ+16), tops 185/190, grass terraces + stone caps, 66..82 m from the eye (inside the 83.5 m band, chunk edge 64 m)
- scan complete=1 (cells=cells=65536/65536)
- paint settled=1 (wire 589715p)
- baseline try 1: drift=1563 px
- baseline try 2: drift=920 px
- lod_silhouette_cols=63 (LOD paint ON SCREEN at 75 m if > 20 — the march-drawn marker column; vanilla cannot render there, chunk edge 64 m; #255 re-measures 49, the ZONE_FAR=48 regression #256 and the control read 0)
- 7694 terrain-green px in the sky region (LOD MOUNTAINS on screen if > 5000; the range paints ColLow-green on low terraces via the fallback palette; flat-world baselines and the pack-free control read ~0)
- burst_close white px: 541 (renderer alive if > 50)
- burst_far white px: 424 (renderer alive in the marker zone if > 20)
- cut walk 40 m: 67 white px (> 20 = rendered)
- cut walk 60 m: 0 white px (> 20 = rendered)
- cut walk 100 m: 0 white px (> 20 = rendered)
- measured far-render cut: 40 m (G4 flap instrument; the decode proof needs >= 83 m from the y=151.6 eye)
- fill issued: 5x111x5 red_concrete at -930,796 (zone forceloaded ±48)
- phase: 4 marker filled (18:04:31)

### /clod-cells AFTER FILL (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 115 0 0x8400 0x2000
123 122 -940 784 71 0 1 [97, 119, 97] 114 0 0x8700 0x2000
124 122 -938 784 68 0 1 [97, 119, 97] 116 0 0x8400 0x2000
125 122 -936 784 62 1 1 [63, 118, 228] 62 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 62 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 61 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 61 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 9 0 0x8000 0x4100
130 122 -926 784 66 0 1 [134, 183, 131] 19 0 0x8200 0x4100
131 122 -924 784 75 0 1 [97, 119, 97] 112 0 0x8b00 0x2000
132 122 -922 784 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 122 -920 784 75 0 1 [97, 119, 97] 18446744073709551615 1 0x8b00 0x2000
134 122 -918 784 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x2000
122 123 -942 786 70 0 1 [97, 119, 97] 14 0 0x8600 0x2000
123 123 -940 786 65 0 1 [97, 119, 97] 117 0 0x8100 0x2000
124 123 -938 786 65 0 1 [97, 119, 97] 116 0 0x8100 0x2000
125 123 -936 786 62 1 1 [63, 118, 228] 62 0 0x7e00 0xd800
126 123 -934 786 62 1 1 [63, 118, 228] 62 0 0x7e00 0xd800
127 123 -932 786 62 1 1 [63, 118, 228] 61 0 0x7e00 0xd800
128 123 -930 786 62 1 1 [63, 118, 228] 61 0 0x7e00 0xd800
129 123 -928 786 73 0 1 [97, 119, 97] 10 0 0x8900 0x2000
130 123 -926 786 67 0 1 [134, 183, 131] 19 0 0x8300 0x4100
131 123 -924 786 69 0 1 [134, 183, 131] 112 0 0x8500 0x4100
132 123 -922 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 123 -920 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 123 -918 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
122 124 -942 788 72 0 1 [97, 119, 97] 14 0 0x8800 0x2000
123 124 -940 788 70 0 1 [97, 119, 97] 23 0 0x8600 0x2000
124 124 -938 788 68 0 1 [97, 119, 97] 23 0 0x8400 0x2000
125 124 -936 788 62 1 1 [63, 118, 228] 62 0 0x7e00 0xd800
126 124 -934 788 62 1 1 [63, 118, 228] 62 0 0x7e00 0xd800
127 124 -932 788 62 1 1 [63, 118, 228] 61 0 0x7e00 0xd800
128 124 -930 788 73 0 1 [97, 119, 97] 113 0 0x8900 0x2000
129 124 -928 788 76 0 1 [97, 119, 97] 10 0 0x8c00 0x2000
130 124 -926 788 73 0 1 [97, 119, 97] 19 0 0x8900 0x2000
131 124 -924 788 69 0 1 [134, 183, 131] 112 0 0x8500 0x4100
132 124 -922 788 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 124 -920 788 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 124 -918 788 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x2000
122 125 -942 790 74 0 1 [97, 119, 97] 14 0 0x8a00 0x2000
123 125 -940 790 73 0 1 [97, 119, 97] 23 0 0x8900 0x2000
124 125 -938 790 71 0 1 [97, 119, 97] 23 0 0x8700 0x2000
125 125 -936 790 62 1 1 [63, 118, 228] 62 0 0x7e00 0xd800
126 125 -934 790 62 1 1 [63, 118, 228] 62 0 0x7e00 0xd800
127 125 -932 790 62 1 1 [63, 118, 228] 61 0 0x7e00 0xd800
128 125 -930 790 62 1 1 [63, 118, 228] 61 0 0x7e00 0xd800
129 125 -928 790 73 0 1 [97, 119, 97] 10 0 0x8900 0x2000
130 125 -926 790 68 0 1 [134, 183, 131] 19 0 0x8400 0x4100
131 125 -924 790 69 0 1 [134, 183, 131] 19 0 0x8500 0x4100
132 125 -922 790 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
133 125 -920 790 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 125 -918 790 70 0 1 [134, 183, 131] 18446744073709551615 1 0x8600 0x4100
122 126 -942 792 74 0 1 [97, 119, 97] 14 0 0x8a00 0x2000
123 126 -940 792 73 0 1 [97, 119, 97] 23 0 0x8900 0x2000
124 126 -938 792 68 0 1 [97, 119, 97] 114 0 0x8400 0x2000
125 126 -936 792 62 1 1 [63, 118, 228] 63 0 0x7e00 0xd800
126 126 -934 792 62 1 1 [63, 118, 228] 62 0 0x7e00 0xd800
127 126 -932 792 62 1 1 [63, 118, 228] 61 0 0x7e00 0xd800
128 126 -930 792 62 1 1 [63, 118, 228] 61 0 0x7e00 0xd800
129 126 -928 792 67 0 1 [134, 183, 131] 10 0 0x8300 0x4100
130 126 -926 792 74 0 1 [97, 119, 97] 18 0 0x8a00 0x2000
131 126 -924 792 77 0 1 [97, 119, 97] 19 0 0x8d00 0x2000
132 126 -922 792 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 126 -920 792 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3200
134 126 -918 792 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x5000
122 127 -942 794 67 0 1 [134, 183, 131] 14 0 0x8300 0x4100
123 127 -940 794 68 0 1 [134, 183, 131] 23 0 0x8400 0x4100
124 127 -938 794 65 0 1 [134, 183, 131] 117 0 0x8100 0x4100
125 127 -936 794 62 1 1 [63, 118, 228] 63 0 0x7e00 0xd800
126 127 -934 794 62 1 1 [63, 118, 228] 62 0 0x7e00 0xd800
127 127 -932 794 170 0 1 [142, 33, 33] 61 1 0xea00 0x0a00
128 127 -930 794 170 0 1 [142, 33, 33] 61 1 0xea00 0x0a00
129 127 -928 794 170 0 1 [142, 33, 33] 60 1 0xea00 0x0a00
130 127 -926 794 74 0 1 [97, 119, 97] 18 0 0x8a00 0x2000
131 127 -924 794 77 0 1 [97, 119, 97] 19 0 0x8d00 0x2000
132 127 -922 794 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 127 -920 794 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3200
134 127 -918 794 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x5000
122 128 -942 796 68 0 1 [134, 183, 131] 14 0 0x8400 0x4100
123 128 -940 796 72 0 1 [97, 119, 97] 23 0 0x8800 0x2000
124 128 -938 796 75 0 1 [97, 119, 97] 117 0 0x8b00 0x2000
125 128 -936 796 72 0 1 [97, 119, 97] 23 0 0x8800 0x2000
126 128 -934 796 62 1 1 [63, 118, 228] 62 0 0x7e00 0xd800
127 128 -932 796 170 0 1 [142, 33, 33] 61 1 0xea00 0x0a00
128 128 -930 796 170 0 1 [142, 33, 33] 61 1 0xea00 0x0a00
129 128 -928 796 170 0 1 [142, 33, 33] 60 1 0xea00 0x0a00
130 128 -926 796 68 0 1 [134, 183, 131] 18 0 0x8400 0x4100
131 128 -924 796 74 0 1 [97, 119, 97] 19 0 0x8a00 0x2000
132 128 -922 796 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 128 -920 796 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
134 128 -918 796 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 129 -942 798 69 0 1 [134, 183, 131] 14 0 0x8500 0x4100
123 129 -940 798 75 0 1 [97, 119, 97] 23 0 0x8b00 0x2000
124 129 -938 798 78 0 1 [97, 119, 97] 15 0 0x8e00 0x2000
125 129 -936 798 75 0 1 [97, 119, 97] 23 0 0x8b00 0x2000
126 129 -934 798 62 1 1 [63, 118, 228] 62 0 0x7e00 0xd800
127 129 -932 798 170 0 1 [142, 33, 33] 61 1 0xea00 0x0a00
128 129 -930 798 170 0 1 [142, 33, 33] 61 1 0xea00 0x0a00
129 129 -928 798 170 0 1 [142, 33, 33] 60 1 0xea00 0x0a00
130 129 -926 798 71 0 1 [97, 119, 97] 18 0 0x8700 0x2000
131 129 -924 798 69 0 1 [134, 183, 131] 19 0 0x8500 0x4100
132 129 -922 798 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 129 -920 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 129 -918 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 130 -942 800 70 0 1 [134, 183, 131] 14 0 0x8600 0x4100
123 130 -940 800 76 0 1 [97, 119, 97] 23 0 0x8c00 0x2000
124 130 -938 800 76 0 1 [97, 119, 97] 114 0 0x8c00 0x2000
125 130 -936 800 72 0 1 [97, 119, 97] 12 0 0x8800 0x2000
126 130 -934 800 62 1 1 [63, 118, 228] 62 0 0x7e00 0xd800
127 130 -932 800 62 1 1 [63, 118, 228] 61 0 0x7e00 0xd800
128 130 -930 800 62 1 1 [63, 118, 228] 61 0 0x7e00 0xd800
129 130 -928 800 71 0 1 [97, 119, 97] 10 0 0x8700 0x2000
130 130 -926 800 71 0 1 [97, 119, 97] 19 0 0x8700 0x2000
131 130 -924 800 69 0 1 [134, 183, 131] 19 0 0x8500 0x4100
132 130 -922 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
133 130 -920 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 130 -918 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 131 -942 802 78 0 1 [97, 119, 97] 25 0 0x8e00 0x2000
123 131 -940 802 77 0 1 [97, 119, 97] 23 0 0x8d00 0x2000
124 131 -938 802 77 0 1 [97, 119, 97] 23 0 0x8d00 0x2000
125 131 -936 802 66 0 1 [134, 183, 131] 12 0 0x8200 0x4100
126 131 -934 802 62 1 1 [63, 118, 228] 62 0 0x7e00 0xd800
127 131 -932 802 62 1 1 [63, 118, 228] 61 0 0x7e00 0xd800
128 131 -930 802 62 1 1 [63, 118, 228] 61 0 0x7e00 0xd800
129 131 -928 802 62 1 1 [63, 118, 228] 60 0 0x7e00 0xd800
130 131 -926 802 66 0 1 [134, 183, 131] 19 0 0x8200 0x4100
131 131 -924 802 68 0 1 [134, 183, 131] 19 0 0x8400 0x4100
132 131 -922 802 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 131 -920 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 131 -918 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 132 -942 804 79 0 1 [97, 119, 97] 118 0 0x8f00 0x2000
123 132 -940 804 76 0 1 [97, 119, 97] 23 0 0x8c00 0x2000
124 132 -938 804 76 0 1 [97, 119, 97] 117 0 0x8c00 0x2000
125 132 -936 804 76 0 1 [97, 119, 97] 12 0 0x8c00 0x2000
126 132 -934 804 62 1 1 [63, 118, 228] 62 0 0x7e00 0xd800
127 132 -932 804 62 1 1 [63, 118, 228] 61 0 0x7e00 0xd800
128 132 -930 804 62 1 1 [63, 118, 228] 61 0 0x7e00 0xd800
129 132 -928 804 62 1 1 [63, 118, 228] 60 0 0x7e00 0xd800
130 132 -926 804 65 0 1 [134, 183, 131] 19 0 0x8100 0x4100
131 132 -924 804 77 0 1 [97, 119, 97] 116 0 0x8d00 0x2000
132 132 -922 804 79 0 1 [97, 119, 97] 9 0 0x8f00 0x2000
133 132 -920 804 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 132 -918 804 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x2000
122 133 -942 806 78 0 1 [97, 119, 97] 118 0 0x8e00 0x2000
123 133 -940 806 69 0 1 [134, 183, 131] 117 0 0x8500 0x4100
124 133 -938 806 155 0 1 [134, 183, 131] 14 0 0xdb00 0x4100
125 133 -936 806 155 0 1 [134, 183, 131] 117 0 0xdb00 0x4100
126 133 -934 806 155 0 1 [134, 183, 131] 116 0 0xdb00 0x4100
127 133 -932 806 155 0 1 [134, 183, 131] 12 0 0xdb00 0x4100
128 133 -930 806 155 0 1 [134, 183, 131] 20 0 0xdb00 0x4100
129 133 -928 806 155 0 1 [134, 183, 131] 114 0 0xdb00 0x4100
130 133 -926 806 155 0 1 [134, 183, 131] 21 0 0xdb00 0x4100
131 133 -924 806 67 0 1 [134, 183, 131] 116 0 0x8300 0x4100
132 133 -922 806 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 133 -920 806 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 133 -918 806 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x2000
122 134 -942 808 79 0 1 [97, 119, 97] 25 0 0x8f00 0x2000
123 134 -940 808 68 0 1 [134, 183, 131] 114 0 0x8400 0x4100
124 134 -938 808 155 0 1 [134, 183, 131] 115 0 0xdb00 0x4100
125 134 -936 808 180 0 1 [134, 183, 131] 113 0 0xf400 0x4100
126 134 -934 808 180 0 1 [134, 183, 131] 10 0 0xf400 0x4100
127 134 -932 808 180 0 1 [134, 183, 131] 9 0 0xf400 0x4100
128 134 -930 808 180 0 1 [134, 183, 131] 116 0 0xf400 0x4100
129 134 -928 808 180 0 1 [134, 183, 131] 130 0 0xf400 0x4100
130 134 -926 808 155 0 1 [134, 183, 131] 10 0 0xdb00 0x4100
131 134 -924 808 67 0 1 [134, 183, 131] 115 0 0x8300 0x4100
132 134 -922 808 68 0 1 [134, 183, 131] 18446744073709551615 1 0x8400 0x4100
133 134 -920 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 134 -918 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
```
### stream diag after fill:
[c-LoD] stream: tick 4300 65536/65536 cells, lap 784, cone 25506 zone 1921 dirty 949 (q 11655) ewin 217 rbeat 0p/0 rearm 0c/0t rescan 1103086c/2546e, edits +5875p/4455c/626e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 316 forced 196, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 724234p/2446b fail 0 ~75 kB/s cache 13596/24576 h710638 m13596 e0, srbeat 62468s/0q drop 68stale/102undec, ereb 48951s/78q, sweep 0r/86d
[18:04:34 INFO]: [c-LoD] stream: tick 4300 65536/65536 cells, lap 784, cone 25506 zone 1921 dirty 949 (q 11655) ewin 217 rbeat 0p/0 rearm 0c/0t rescan 1103086c/2546e, edits +5875p/4455c/626e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 316 forced 196, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 724234p/2446b fail 0 ~75 kB/s cache 13596/24576 h710638 m13596 e0, srbeat 62468s/0q drop 68stale/102undec, ereb 48951s/78q, sweep 0r/86d
- carrier ladder y=155: 14388 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=165: 14336 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=168: 14336 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=172: 14336 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=176: 14336 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=180: 14336 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=185: 14336 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- probe 0: red=0 paint=9090 lod_silhouette_cols=57 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=8 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 1: red=0 paint=9090 lod_silhouette_cols=57 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=8 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 2: red=0 paint=9769 lod_silhouette_cols=57 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=8 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 3: red=0 paint=9617 lod_silhouette_cols=57 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=8 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 4: red=0 paint=10511 lod_silhouette_cols=57 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=8 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 5: red=0 paint=9199 lod_silhouette_cols=57 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=8 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 6: red=0 paint=9440 lod_silhouette_cols=57 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=8 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 7: red=0 paint=10149 lod_silhouette_cols=57 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=8 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 8: red=0 paint=9769 lod_silhouette_cols=57 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=8 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 9: red=0 paint=12261 lod_silhouette_cols=57 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=9 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 10: red=0 paint=12259 lod_silhouette_cols=57 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=9 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 11: red=0 paint=12259 lod_silhouette_cols=57 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=9 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 12: red=0 paint=12259 lod_silhouette_cols=57 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=9 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 13: red=0 paint=12259 lod_silhouette_cols=57 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=9 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 14: red=0 paint=13510 lod_silhouette_cols=57 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=9 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 15: red=0 paint=13452 lod_silhouette_cols=57 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=9 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 16: red=0 paint=13380 lod_silhouette_cols=57 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=9 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 17: red=0 paint=14380 lod_silhouette_cols=57 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=9 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 18: red=0 paint=12965 lod_silhouette_cols=57 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=9 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 19: red=0 paint=13962 lod_silhouette_cols=57 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=9 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 20: red=0 paint=16082 lod_silhouette_cols=57 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=9 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 21: red=0 paint=15633 lod_silhouette_cols=57 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=9 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 22: red=0 paint=16283 lod_silhouette_cols=57 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=9 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 23: red=0 paint=16148 lod_silhouette_cols=57 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=9 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 24: red=0 paint=15325 lod_silhouette_cols=57 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=9 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 25: red=0 paint=15818 lod_silhouette_cols=57 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=9 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 26: red=0 paint=15994 lod_silhouette_cols=57 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=9 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 27: red=0 paint=15876 lod_silhouette_cols=57 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=9 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 28: red=0 paint=16784 lod_silhouette_cols=57 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=9 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 29: red=0 paint=15559 lod_silhouette_cols=57 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=9 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 30: red=0 paint=15784 lod_silhouette_cols=57 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=9 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 31: red=0 paint=16023 lod_silhouette_cols=57 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=9 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 32: red=0 paint=16068 lod_silhouette_cols=57 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=9 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 33: red=0 paint=16892 lod_silhouette_cols=57 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=9 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 34: red=0 paint=15506 lod_silhouette_cols=57 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=9 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 35: red=0 paint=15616 lod_silhouette_cols=57 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=9 mask_hits=30 (need red>=60 paint>=300 lod>20)
- phase: 5 leg-1 probes done (18:10:25)
- **FAIL:** leg-1 never saw the marker in 24 probes (red < 60 everywhere) — the edit never reached the LOD paint; this failure was SILENT before v6.31.1
### stream diag after probes:
[18:10:05 INFO]: [c-LoD] stream: tick 7600 65536/65536 cells, lap 984, cone 51273 zone 3537 dirty 999 (q 11679) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 2170509c/2604e, edits +7335p/5471c/945e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 316 forced 196, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 1069051p/3281b fail 0 ~73 kB/s cache 13698/24576 h1055353 m13698 e0, srbeat 78426s/0q drop 74stale/132undec, ereb 50139s/0q, sweep 0r/16d
[c-LoD] stream: tick 7700 65536/65536 cells, lap 993, cone 52425 zone 3537 dirty 999 (q 11679) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 2201702c/2604e, edits +7335p/5471c/945e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 316 forced 196, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 1083114p/3310b fail 0 ~74 kB/s cache 13698/24576 h1069416 m13698 e0, srbeat 79044s/0q drop 74stale/132undec, ereb 50139s/0q, sweep 0r/25d
[18:10:15 INFO]: [c-LoD] stream: tick 7700 65536/65536 cells, lap 993, cone 52425 zone 3537 dirty 999 (q 11679) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 2201702c/2604e, edits +7335p/5471c/945e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 316 forced 196, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 1083114p/3310b fail 0 ~74 kB/s cache 13698/24576 h1069416 m13698 e0, srbeat 79044s/0q drop 74stale/132undec, ereb 50139s/0q, sweep 0r/25d
- parity debug tour: 13 vantages (session-1 client, DEBUG pack — evidence only, the gate pairs the §7b world session)
- parity debug tour done: 13 shots in parity_dbg/ (the §7b world session feeds the gate)
- joined session 2 (18:13:16)
- leg-2 baseline scene px (x<940): 0 (0 = the relog client paints no world at all)
- leg-2 renderer probe NEAR (20 m) white px: 463 raw / 463 changed-vs-baseline (> 20 = the client renders particles)
- leg-2 renderer probe FAR (80 m, marker zone) white px: 418 raw / 418 changed-vs-baseline (> 20 = no distance cut; ~0 with NEAR alive = particles beyond the client chunk radius do not render)
- leg-2 carrier probe green dust (80 m): changed-green px 0 / total changed 330 vs baseline (> 100 green = a stable green carrier exists; total > 0 with green ~0 = the dust renders but fog washes it — a ruler problem, not a carrier problem)
- phase: 8 leg-2 relog probes (18:18:40)
- LEG-2 SCENE DEAD: the relog client paints ZERO scene pixels (x<940) — pure sky + lens panels; the fresh-session capture chain never commits (renderer-dead class, #227/#228); these probes measure a worldless client, not the transport
- relog probe 0: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=0 mask_hits=0 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 1: red=0 scene=138157 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=3 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 2: red=0 scene=160562 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=5 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 3: red=0 scene=153950 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=6 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 4: red=0 scene=152766 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=6 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 5: red=0 scene=153260 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=6 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 6: red=0 scene=154626 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=6 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 7: red=0 scene=158359 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=2 hits_hi=6 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 8: red=0 scene=159657 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=2 hits_hi=6 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 9: red=0 scene=160103 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=2 hits_hi=6 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 10: red=0 scene=159201 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=2 hits_hi=6 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 11: red=0 scene=161338 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=2 hits_hi=6 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 12: red=0 scene=160793 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=2 hits_hi=6 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 13: red=0 scene=159555 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=2 hits_hi=6 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 14: red=0 scene=159944 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=2 hits_hi=6 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 15: red=0 scene=160155 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=2 hits_hi=6 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 16: red=0 scene=162317 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=2 hits_hi=6 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 17: red=0 scene=162323 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=2 hits_hi=6 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 18: red=0 scene=162323 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=2 hits_hi=6 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 19: red=0 scene=162323 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=2 hits_hi=6 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- **FAIL:** marker lost after relog — stream state is not persistent/deterministic
- phase: 9 leg-2 done (18:21:38)

### client log forensics (pipeline/particle/atlas — v6.8.2)
```
[18:13:05] [main/DEBUG]: Loading LWJGL system
WARNING: java.lang.System::load has been called by org.lwjgl.system.Library$$Lambda/0x00000000551e3d60 in an unnamed module (file:/home/runner/mc-test/client/libraries/org/lwjgl/lwjgl/3.4.3/lwjgl-3.4.3.jar)
WARNING: sun.misc.Unsafe::objectFieldOffset has been called by org.lwjgl.system.MemoryBackendUnsafeLegacy (file:/home/runner/mc-test/client/libraries/org/lwjgl/lwjgl/3.4.3/lwjgl-3.4.3.jar)
WARNING: Please consider reporting this to the maintainers of class org.lwjgl.system.MemoryBackendUnsafeLegacy
[18:13:05] [main/DEBUG]: Loading OpenGL
[18:13:05] [main/DEBUG]: Loading shaderc
[18:13:10] [Render thread/ERROR]: Error parsing option value 4 for option Simulation Distance: Value 4 outside of range [5:33]
[18:13:10] [Render thread/INFO]: Backend library: LWJGL version 3.4.3+4
[18:13:10] [Render thread/INFO]: Using graphics backend OpenGL, using drivers: 4.5 (Core Profile) Mesa 26.0.8-1ubuntu0.3
[18:13:10] [Render thread/INFO]: Using graphics device: llvmpipe (LLVM 21.1.8, 256 bits) (Mesa)
[18:13:10] [Render thread/INFO]: Using graphics device extensions: GL_ARB_multi_draw_indirect, GL_ARB_buffer_storage, GL_ARB_shader_draw_parameters, GL_ARB_base_instance, GL_KHR_debug, GL_ARB_draw_indirect, GL_ARB_clip_control, GL_ARB_vertex_attrib_binding, GL_ARB_direct_state_access, GL_EXT_texture_filter_anisotropic
[18:13:10] [Download-2/ERROR]: Failed to fetch user properties
com.mojang.authlib.exceptions.InvalidCredentialsException: Status: 401
	at com.mojang.authlib.exceptions.MinecraftClientHttpException.toAuthenticationException(MinecraftClientHttpException.java:70)
Caused by: MinecraftClientHttpException[type=HTTP_ERROR, status=401, response=ErrorResponse[path=/player/attributes, error=null, errorMessage=null, details=null]]
	at com.mojang.authlib.minecraft.client.MinecraftClient.buildHttpException(MinecraftClient.java:170)
[18:13:10] [Download-1/INFO]: Could not authorize you against Realms server: java.lang.RuntimeException: Failed to parse into SignedJWT: 0
[18:13:10] [Download-1/ERROR]: Failed to fetch Realms feature flags
com.mojang.realmsclient.exception.RealmsServiceException: Realms authentication error with message 'java.lang.RuntimeException: Failed to parse into SignedJWT: 0'
[18:13:10] [Render thread/INFO]: Reloading ResourceManager: vanilla, vanilla
[18:13:13] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[18:13:13] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[18:13:13] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[18:13:13] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[18:13:13] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[18:13:13] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[18:13:13] [Render thread/INFO]: Created: 2048x2048x4 minecraft:textures/atlas/blocks.png-atlas
[18:13:13] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[18:13:13] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[18:13:13] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[18:13:13] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[18:13:13] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
[18:13:20] [Render thread/INFO]: Reloading ResourceManager: vanilla, vanilla, server/00000000/6b1f2f7e-9c0d-4a4b-8b0a-b10d1b4ab100
[18:13:21] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[18:13:21] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[18:13:21] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[18:13:21] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[18:13:21] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[18:13:21] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[18:13:21] [Render thread/INFO]: Created: 2048x2048x4 minecraft:textures/atlas/blocks.png-atlas
[18:13:21] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[18:13:21] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[18:13:21] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[18:13:21] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[18:13:21] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
```

### /clod-cells AFTER PROBES (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 484 0 0x8400 0x2000
123 122 -940 784 71 0 1 [97, 119, 97] 485 0 0x8700 0x2000
124 122 -938 784 68 0 1 [97, 119, 97] 494 0 0x8400 0x2000
125 122 -936 784 62 1 1 [63, 118, 228] 535 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 534 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 534 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 533 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 493 0 0x8000 0x4100
130 122 -926 784 66 0 1 [134, 183, 131] 821 0 0x8200 0x4100
131 122 -924 784 75 0 1 [97, 119, 97] 812 0 0x8b00 0x2000
132 122 -922 784 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 122 -920 784 75 0 1 [97, 119, 97] 18446744073709551615 1 0x8b00 0x2000
134 122 -918 784 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x2000
122 123 -942 786 70 0 1 [97, 119, 97] 484 0 0x8600 0x2000
123 123 -940 786 65 0 1 [97, 119, 97] 485 0 0x8100 0x2000
124 123 -938 786 65 0 1 [97, 119, 97] 494 0 0x8100 0x2000
125 123 -936 786 62 1 1 [63, 118, 228] 535 0 0x7e00 0xd800
126 123 -934 786 62 1 1 [63, 118, 228] 535 0 0x7e00 0xd800
127 123 -932 786 62 1 1 [63, 118, 228] 534 0 0x7e00 0xd800
128 123 -930 786 62 1 1 [63, 118, 228] 533 0 0x7e00 0xd800
129 123 -928 786 73 0 1 [97, 119, 97] 493 0 0x8900 0x2000
130 123 -926 786 67 0 1 [134, 183, 131] 821 0 0x8300 0x4100
131 123 -924 786 69 0 1 [134, 183, 131] 812 0 0x8500 0x4100
132 123 -922 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 123 -920 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 123 -918 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
122 124 -942 788 72 0 1 [97, 119, 97] 484 0 0x8800 0x2000
123 124 -940 788 70 0 1 [97, 119, 97] 485 0 0x8600 0x2000
124 124 -938 788 68 0 1 [97, 119, 97] 494 0 0x8400 0x2000
125 124 -936 788 62 1 1 [63, 118, 228] 535 0 0x7e00 0xd800
126 124 -934 788 62 1 1 [63, 118, 228] 535 0 0x7e00 0xd800
127 124 -932 788 62 1 1 [63, 118, 228] 534 0 0x7e00 0xd800
128 124 -930 788 73 0 1 [97, 119, 97] 592 0 0x8900 0x2000
129 124 -928 788 76 0 1 [97, 119, 97] 493 0 0x8c00 0x2000
130 124 -926 788 73 0 1 [97, 119, 97] 821 0 0x8900 0x2000
131 124 -924 788 69 0 1 [134, 183, 131] 813 0 0x8500 0x4100
132 124 -922 788 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 124 -920 788 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 124 -918 788 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x2000
122 125 -942 790 74 0 1 [97, 119, 97] 484 0 0x8a00 0x2000
123 125 -940 790 73 0 1 [97, 119, 97] 486 0 0x8900 0x2000
124 125 -938 790 71 0 1 [97, 119, 97] 494 0 0x8700 0x2000
125 125 -936 790 62 1 1 [63, 118, 228] 535 0 0x7e00 0xd800
126 125 -934 790 62 1 1 [63, 118, 228] 535 0 0x7e00 0xd800
127 125 -932 790 62 1 1 [63, 118, 228] 534 0 0x7e00 0xd800
128 125 -930 790 62 1 1 [63, 118, 228] 533 0 0x7e00 0xd800
129 125 -928 790 73 0 1 [97, 119, 97] 493 0 0x8900 0x2000
130 125 -926 790 68 0 1 [134, 183, 131] 821 0 0x8400 0x4100
131 125 -924 790 69 0 1 [134, 183, 131] 813 0 0x8500 0x4100
132 125 -922 790 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
133 125 -920 790 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 125 -918 790 70 0 1 [134, 183, 131] 18446744073709551615 1 0x8600 0x4100
122 126 -942 792 74 0 1 [97, 119, 97] 497 0 0x8a00 0x2000
123 126 -940 792 73 0 1 [97, 119, 97] 486 0 0x8900 0x2000
124 126 -938 792 68 0 1 [97, 119, 97] 494 0 0x8400 0x2000
125 126 -936 792 62 1 1 [63, 118, 228] 535 0 0x7e00 0xd800
126 126 -934 792 62 1 1 [63, 118, 228] 535 0 0x7e00 0xd800
127 126 -932 792 62 1 1 [63, 118, 228] 534 0 0x7e00 0xd800
128 126 -930 792 62 1 1 [63, 118, 228] 534 0 0x7e00 0xd800
129 126 -928 792 67 0 1 [134, 183, 131] 493 0 0x8300 0x4100
130 126 -926 792 74 0 1 [97, 119, 97] 821 0 0x8a00 0x2000
131 126 -924 792 77 0 1 [97, 119, 97] 813 0 0x8d00 0x2000
132 126 -922 792 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 126 -920 792 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3200
134 126 -918 792 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x5000
122 127 -942 794 67 0 1 [134, 183, 131] 497 0 0x8300 0x4100
123 127 -940 794 68 0 1 [134, 183, 131] 486 0 0x8400 0x4100
124 127 -938 794 65 0 1 [134, 183, 131] 485 0 0x8100 0x4100
125 127 -936 794 62 1 1 [63, 118, 228] 535 0 0x7e00 0xd800
126 127 -934 794 62 1 1 [63, 118, 228] 535 0 0x7e00 0xd800
127 127 -932 794 170 0 1 [142, 33, 33] 483 0 0xea00 0x0a00
128 127 -930 794 170 0 1 [142, 33, 33] 492 0 0xea00 0x0a00
129 127 -928 794 170 0 1 [142, 33, 33] 493 0 0xea00 0x0a00
130 127 -926 794 74 0 1 [97, 119, 97] 821 0 0x8a00 0x2000
131 127 -924 794 77 0 1 [97, 119, 97] 813 0 0x8d00 0x2000
132 127 -922 794 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 127 -920 794 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3200
134 127 -918 794 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x5000
122 128 -942 796 68 0 1 [134, 183, 131] 497 0 0x8400 0x4100
123 128 -940 796 72 0 1 [97, 119, 97] 486 0 0x8800 0x2000
124 128 -938 796 75 0 1 [97, 119, 97] 485 0 0x8b00 0x2000
125 128 -936 796 72 0 1 [97, 119, 97] 484 0 0x8800 0x2000
126 128 -934 796 62 1 1 [63, 118, 228] 535 0 0x7e00 0xd800
127 128 -932 796 170 0 1 [142, 33, 33] 483 0 0xea00 0x0a00
128 128 -930 796 170 0 1 [142, 33, 33] 492 0 0xea00 0x0a00
129 128 -928 796 170 0 1 [142, 33, 33] 493 0 0xea00 0x0a00
130 128 -926 796 68 0 1 [134, 183, 131] 821 0 0x8400 0x4100
131 128 -924 796 74 0 1 [97, 119, 97] 813 0 0x8a00 0x2000
132 128 -922 796 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 128 -920 796 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
134 128 -918 796 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 129 -942 798 69 0 1 [134, 183, 131] 497 0 0x8500 0x4100
123 129 -940 798 75 0 1 [97, 119, 97] 486 0 0x8b00 0x2000
124 129 -938 798 78 0 1 [97, 119, 97] 496 0 0x8e00 0x2000
125 129 -936 798 75 0 1 [97, 119, 97] 484 0 0x8b00 0x2000
126 129 -934 798 62 1 1 [63, 118, 228] 535 0 0x7e00 0xd800
127 129 -932 798 170 0 1 [142, 33, 33] 483 0 0xea00 0x0a00
128 129 -930 798 170 0 1 [142, 33, 33] 592 0 0xea00 0x0a00
129 129 -928 798 170 0 1 [142, 33, 33] 493 0 0xea00 0x0a00
130 129 -926 798 71 0 1 [97, 119, 97] 821 0 0x8700 0x2000
131 129 -924 798 69 0 1 [134, 183, 131] 813 0 0x8500 0x4100
132 129 -922 798 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 129 -920 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 129 -918 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 130 -942 800 70 0 1 [134, 183, 131] 484 0 0x8600 0x4100
123 130 -940 800 76 0 1 [97, 119, 97] 486 0 0x8c00 0x2000
124 130 -938 800 76 0 1 [97, 119, 97] 494 0 0x8c00 0x2000
125 130 -936 800 72 0 1 [97, 119, 97] 484 0 0x8800 0x2000
126 130 -934 800 62 1 1 [63, 118, 228] 535 0 0x7e00 0xd800
127 130 -932 800 62 1 1 [63, 118, 228] 534 0 0x7e00 0xd800
128 130 -930 800 62 1 1 [63, 118, 228] 533 0 0x7e00 0xd800
129 130 -928 800 71 0 1 [97, 119, 97] 493 0 0x8700 0x2000
130 130 -926 800 71 0 1 [97, 119, 97] 821 0 0x8700 0x2000
131 130 -924 800 69 0 1 [134, 183, 131] 813 0 0x8500 0x4100
132 130 -922 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
133 130 -920 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 130 -918 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 131 -942 802 78 0 1 [97, 119, 97] 486 0 0x8e00 0x2000
123 131 -940 802 77 0 1 [97, 119, 97] 486 0 0x8d00 0x2000
124 131 -938 802 77 0 1 [97, 119, 97] 494 0 0x8d00 0x2000
125 131 -936 802 66 0 1 [134, 183, 131] 484 0 0x8200 0x4100
126 131 -934 802 62 1 1 [63, 118, 228] 535 0 0x7e00 0xd800
127 131 -932 802 62 1 1 [63, 118, 228] 534 0 0x7e00 0xd800
128 131 -930 802 62 1 1 [63, 118, 228] 533 0 0x7e00 0xd800
129 131 -928 802 62 1 1 [63, 118, 228] 533 0 0x7e00 0xd800
130 131 -926 802 66 0 1 [134, 183, 131] 821 0 0x8200 0x4100
131 131 -924 802 68 0 1 [134, 183, 131] 813 0 0x8400 0x4100
132 131 -922 802 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 131 -920 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 131 -918 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 132 -942 804 79 0 1 [97, 119, 97] 486 0 0x8f00 0x2000
123 132 -940 804 76 0 1 [97, 119, 97] 485 0 0x8c00 0x2000
124 132 -938 804 76 0 1 [97, 119, 97] 494 0 0x8c00 0x2000
125 132 -936 804 76 0 1 [97, 119, 97] 495 0 0x8c00 0x2000
126 132 -934 804 62 1 1 [63, 118, 228] 535 0 0x7e00 0xd800
127 132 -932 804 62 1 1 [63, 118, 228] 534 0 0x7e00 0xd800
128 132 -930 804 62 1 1 [63, 118, 228] 533 0 0x7e00 0xd800
129 132 -928 804 62 1 1 [63, 118, 228] 533 0 0x7e00 0xd800
130 132 -926 804 65 0 1 [134, 183, 131] 821 0 0x8100 0x4100
131 132 -924 804 77 0 1 [97, 119, 97] 813 0 0x8d00 0x2000
132 132 -922 804 79 0 1 [97, 119, 97] 602 0 0x8f00 0x2000
133 132 -920 804 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 132 -918 804 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x2000
122 133 -942 806 78 0 1 [97, 119, 97] 486 0 0x8e00 0x2000
123 133 -940 806 69 0 1 [134, 183, 131] 485 0 0x8500 0x4100
124 133 -938 806 155 0 1 [134, 183, 131] 484 0 0xdb00 0x4100
125 133 -936 806 155 0 1 [134, 183, 131] 485 0 0xdb00 0x4100
126 133 -934 806 155 0 1 [134, 183, 131] 494 0 0xdb00 0x4100
127 133 -932 806 155 0 1 [134, 183, 131] 495 0 0xdb00 0x4100
128 133 -930 806 155 0 1 [134, 183, 131] 483 0 0xdb00 0x4100
129 133 -928 806 155 0 1 [134, 183, 131] 482 0 0xdb00 0x4100
130 133 -926 806 155 0 1 [134, 183, 131] 592 0 0xdb00 0x4100
131 133 -924 806 67 0 1 [134, 183, 131] 812 0 0x8300 0x4100
132 133 -922 806 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 133 -920 806 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 133 -918 806 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x2000
122 134 -942 808 79 0 1 [97, 119, 97] 486 0 0x8f00 0x2000
123 134 -940 808 68 0 1 [134, 183, 131] 485 0 0x8400 0x4100
124 134 -938 808 155 0 1 [134, 183, 131] 484 0 0xdb00 0x4100
125 134 -936 808 180 0 1 [134, 183, 131] 492 0 0xf400 0x4100
126 134 -934 808 180 0 1 [134, 183, 131] 492 0 0xf400 0x4100
127 134 -932 808 180 0 1 [134, 183, 131] 602 0 0xf400 0x4100
128 134 -930 808 180 0 1 [134, 183, 131] 813 0 0xf400 0x4100
129 134 -928 808 180 0 1 [134, 183, 131] 812 0 0xf400 0x4100
130 134 -926 808 155 0 1 [134, 183, 131] 492 0 0xdb00 0x4100
131 134 -924 808 67 0 1 [134, 183, 131] 812 0 0x8300 0x4100
132 134 -922 808 68 0 1 [134, 183, 131] 18446744073709551615 1 0x8400 0x4100
133 134 -920 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 134 -918 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
```
- edit channel: 0 linkage errors, 0 failed event dispatches (both must be 0)
- phase: 6 parity world boot (18:21:38)
- parity world: view-distance=8 (the voxy leg's server value)
- parity world: joined session 3 (18:22:18)
- parity world: scan populated (cells=65536; the march paints from these buffers)
- parity world tour: 13 vantages (production pack, vd 8, shared structures)
- coverage sweeps: 72 eye passes = 36 positions (12 az x r144/200 + 12 off-az x r172, y=160) x 2 looks (outward+inward), settle-by-laps >= 6 s (q stable 3x1s), deadline 12 s/pos, no shots — the v6.26.0 data-coverage lever + v6.26.6 lottery kill; probe rides parity/view_probe.md
- parity drain-settle TIMEOUT: queue 24791 -> 24533 > 11500 after 150s — continuing anyway; the queue did not drain without eye proximity (priority-lane evidence, the 01/05/10 dip attribution reads from the tour deltas)
- pre-tour pass done: 13 vantages x 10s (the tour now reads the second OR-merge lap; the rim/dip transport race reads from the tour deltas)
- parity world tour done: 13 shots, 0 night-guard renames in parity/ (the gate pairs voxy/<name>.png vs parity/<name>.png)
- phase: 7 parity tour done (18:44:09)
- sparse re-beat channel at tour end: streamed=152145 q=0 dropped_stale=62 dropped_undecodable=16779 (frac=8 boot; undecodable-dominant = the rim words die in their decodability window, stale-dominant = edit churn owns the cells)
### parity world stream counters (session 3, tail + maxima)
[c-LoD] stream: tick 13200 65536/65536 cells, lap 1334, cone 64557 zone 20265 dirty 23835 (q 24520) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 9004594c/4323e, edits +18007p/13655c/2876e drop 0, spawned 0 / failed 0, cam0 yaw -180 pitch 5, wish 0 forced 905, native 50816c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 2046610p/6187b fail 0 ~0 kB/s cache 23011/24576 h1568990 m477620 e454609, srbeat 152145s/0q drop 62stale/16779undec, ereb 108545s/0q, sweep 0r/54d
[18:44:07 INFO]: [c-LoD] stream: tick 13200 65536/65536 cells, lap 1334, cone 64557 zone 20265 dirty 23835 (q 24520) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 9004594c/4323e, edits +18007p/13655c/2876e drop 0, spawned 0 / failed 0, cam0 yaw -180 pitch 5, wish 0 forced 905, native 50816c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 2046610p/6187b fail 0 ~0 kB/s cache 23011/24576 h1568990 m477620 e454609, srbeat 152145s/0q drop 62stale/16779undec, ereb 108545s/0q, sweep 0r/54d
- parity stream max cone: 64557
- parity stream max zone: 20265
- parity stream max wire bundle: bundle
### parity world capture lens (debug chain on the §7b world)
lens_red=0 lens_black=0 lens_water=0 lens_dark=0 lens_zero=0 rot=none scan=0 carriers=0
- parity mesh probe @png_y=2: x306=(151,184,255) x307=(151,184,255) x308=(151,184,255) x309h=(151,184,255) x310c=(151,184,255)
- parity mesh probe @png_y=717: x306=(44,62,51) x307=(65,89,72) x308=(81,107,87) x309h=(82,108,87) x310c=(84,111,90)
- parity mesh engagement: magenta-ish=0 of 921600 sampled px
- mask8: the un-fog engagement masks shot on the PARITY WORLD (vd 8) — parity_dbg8/01m|07m|10m
- mask8 01m: green=96 red=0 blue=94736 dim=10288 (px, sampled /16)
- mask8 07m: green=11888 red=0 blue=221472 dim=5824 (px, sampled /16)
- mask8 10m: green=5120 red=320 blue=232272 dim=218640 (px, sampled /16)
- mesh probe @png_y=2: x306=(255,255,255) x307=(13,254,0) x308=(0,254,0) x309h=(0,0,0) x310c=(0,0,0)
- mesh probe @png_y=717: x306=(255,255,255) x307=(13,254,0) x308=(0,254,0) x309h=(0,0,0) x310c=(0,0,0)
- mesh engagement: magenta-ish=0 of 921600 sampled px (the vanilla-path paint coverage)
- mask8 fog state @png_y=2: raw (300)=(151,184,255) (301)=(151,184,255)
- mask8 fog state @png_y=717: raw (300)=(0,128,8) (301)=(179,206,255)
- mask8 fog state decoded: envStart=0 envEnd=1024 rdEnd=128 fogColor=(179,206,255)
- buffer dump 10m: marked h=94/2048 c=8/2048; height windows r0<142:35 r1:13 r2>=202:46 maxByte=255
- buffer dump 10m: RANK-2-WINDOW HEIGHTS (the fake-wall class): (-12,-32)h=149, (+22,-32)h=181, (+24,-32)h=191, (+76,-30)h=191, (+26,-28)h=187, (+30,-28)h=187, (+72,-28)h=151, (-6,-26)h=184, (+52,-26)h=175, (+76,-26)h=191
- rim mask: scan populated (cells=65536; the state paint reads the same buffers the march reads)
- phase: 10 rim mask lane (18:48:21)
- rim mask: decode-state frames shot (rim_mask/00|01|12 — GREEN full decode / RED colorless / BLUE heightless; 00 = the leg-1 tower eye, 01 = the near-tower contrast, 12 = the tower re-read; the inherited 07/12 vantages sat 760 blocks outside the streamed disk and 01 sat INSIDE the marker column — their earlier verdicts were classifier noise, purged in v6.33.1)
- rim mask 00: green=22128 red=39360 blue=265488 yellow=30896 (px, sampled /16) -> DOMINANT BLUE: rim = HEIGHTS NEVER DECODED — a data-coverage class: the state paint cannot even see the cells
- rim mask 01: green=65408 red=36112 blue=270800 yellow=62288 (px, sampled /16) -> DOMINANT BLUE: rim = HEIGHTS NEVER DECODED — a data-coverage class: the state paint cannot even see the cells
- rim mask 12: green=53168 red=28032 blue=288000 yellow=45776 (px, sampled /16) -> DOMINANT BLUE: rim = HEIGHTS NEVER DECODED — a data-coverage class: the state paint cannot even see the cells

### boot / stream
```
[c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 145820 bytes, sha1 190b18e007c6594d27f487361abf2ceddfe55ede — zero textures, zero world data, one push per join
[c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[18:47:39 INFO]: [c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 145820 bytes, sha1 190b18e007c6594d27f487361abf2ceddfe55ede — zero textures, zero world data, one push per join
[18:47:39 INFO]: [c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[c-LoD] stream: tick 800 65536/65536 cells, lap 603, cone 10059 zone 864 dirty 22 (q 1536) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 470622c/96e, edits +8p/8c/3e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 80 forced 847, native 4160c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 128768p/496b fail 0 ~20 kB/s cache 14081/24576 h114687 m14081 e0, srbeat 6511s/213q drop 0stale/1095undec, ereb 189s/25q, sweep 0r/29d
[18:49:09 INFO]: [c-LoD] stream: tick 800 65536/65536 cells, lap 603, cone 10059 zone 864 dirty 22 (q 1536) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 470622c/96e, edits +8p/8c/3e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 80 forced 847, native 4160c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 128768p/496b fail 0 ~20 kB/s cache 14081/24576 h114687 m14081 e0, srbeat 6511s/213q drop 0stale/1095undec, ereb 189s/25q, sweep 0r/29d
```
### sessions
```
[18:48:00 INFO]: System chat: Stepan joined the game
[18:49:16 INFO]: System chat: Stepan left the game
```
### client log tail
```
[18:49:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[18:49:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[18:49:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[18:49:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:bundle_delimiter] net.minecraft.network.protocol.game.ClientboundBundleDelimiterPacket -> 1 bytes
[18:49:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[18:49:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[18:49:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[18:49:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[18:49:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[18:49:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[18:49:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[18:49:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[18:49:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:bundle_delimiter] net.minecraft.network.protocol.game.ClientboundBundleDelimiterPacket -> 1 bytes
[18:49:14] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:set_time] net.minecraft.network.protocol.game.ClientboundSetTimePacket -> 10 bytes
[18:49:14] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[18:49:14] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[18:49:14] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[18:49:14] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[18:49:14] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[18:49:14] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[18:49:14] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[18:49:14] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[18:49:14] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[18:49:14] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[18:49:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:keep_alive] net.minecraft.network.protocol.common.ClientboundKeepAlivePacket -> 9 bytes
[18:49:15] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:keep_alive] net.minecraft.network.protocol.common.ServerboundKeepAlivePacket -> 9 bytes
[18:49:15] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:move_player_pos] net.minecraft.network.protocol.game.ServerboundMovePlayerPacket$Pos -> 26 bytes
[18:49:15] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[18:49:15] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[18:49:15] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[18:49:15] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[18:49:15] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[18:49:15] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[18:49:15] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[18:49:15] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[18:49:15] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[18:49:15] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[18:49:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[18:49:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[18:49:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[18:49:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[18:49:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[18:49:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[18:49:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:set_time] net.minecraft.network.protocol.game.ClientboundSetTimePacket -> 10 bytes
[18:49:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[18:49:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[18:49:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[18:49:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[18:49:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[18:49:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[18:49:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[18:49:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[18:49:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:bundle_delimiter] net.minecraft.network.protocol.game.ClientboundBundleDelimiterPacket -> 1 bytes
[18:49:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[18:49:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[18:49:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[18:49:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[18:49:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[18:49:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[18:49:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[18:49:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[18:49:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:bundle_delimiter] net.minecraft.network.protocol.game.ClientboundBundleDelimiterPacket -> 1 bytes
[18:49:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[18:49:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[18:49:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[18:49:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[18:49:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[18:49:16] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:keep_alive] net.minecraft.network.protocol.common.ClientboundKeepAlivePacket -> 9 bytes
[18:49:16] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:keep_alive] net.minecraft.network.protocol.common.ServerboundKeepAlivePacket -> 9 bytes
[18:49:16] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[18:49:16] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[18:49:16] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[18:49:16] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[18:49:16] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[18:49:16] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[18:49:16] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[18:49:16] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[18:49:16] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[18:49:16] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[18:49:16] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:entity_position_sync] net.minecraft.network.protocol.game.ClientboundEntityPositionSyncPacket -> 38 bytes
--- exceptions (Realms/RTKit noise filtered) ---
25:[18:47:55] [Render thread/ERROR]: Error parsing option value 4 for option Simulation Distance: Value 4 outside of range [5:33]
48:[18:47:55] [Download-2/ERROR]: Failed to fetch user properties
49:com.mojang.authlib.exceptions.InvalidCredentialsException: Status: 401
50:	at com.mojang.authlib.exceptions.MinecraftClientHttpException.toAuthenticationException(MinecraftClientHttpException.java:70)
57:Caused by: MinecraftClientHttpException[type=HTTP_ERROR, status=401, response=ErrorResponse[path=/player/attributes, error=null, errorMessage=null, details=null]]
58:	at com.mojang.authlib.minecraft.client.MinecraftClient.buildHttpException(MinecraftClient.java:170)
65:[18:47:55] [Download-1/ERROR]: Got an error with a html body connecting to https://sessionserver.mojang.com/session/minecraft/profile/d1652ad977ce32009642f973c97adf36?unsigned=false: <!DOCTYPE html PUBLIC '-//W3C//DTD XHTML 1.0 Transitional//EN' 'http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd'>
128:MinecraftClientHttpException[type=HTTP_ERROR, status=429, response=null]
129:	at com.mojang.authlib.minecraft.client.MinecraftClient.buildHttpException(MinecraftClient.java:167)
```
### server log tail
```
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[18:49:17 INFO]: [ChunkHolderManager] Saved 1001 block chunks, 975 entity chunks, 0 poi chunks in world 'minecraft:overworld' in 0.99s
[18:49:17 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:overworld'
[18:49:17 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:overworld'
[18:49:17 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[18:49:17 INFO]: [ChunkHolderManager] Waiting 60s for chunk system to halt for world 'minecraft:the_nether'
[18:49:17 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:the_nether'
[18:49:17 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_nether'
[18:49:17 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_nether' in 0.00s
[18:49:17 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_nether'
[18:49:17 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_nether'
[18:49:17 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[18:49:17 INFO]: [ChunkHolderManager] Waiting 60s for chunk system to halt for world 'minecraft:the_end'
[18:49:17 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:the_end'
[18:49:17 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_end'
[18:49:17 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_end' in 0.00s
[18:49:17 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_end'
[18:49:17 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_end'
[18:49:17 INFO]: Waiting for all RegionFile I/O tasks to complete...
[18:49:17 INFO]: All RegionFile I/O tasks to complete
[18:49:17 INFO]: [MoonriseCommon] Awaiting termination of worker pool for up to 60s...
[18:49:17 INFO]: [MoonriseCommon] Awaiting termination of I/O pool for up to 60s...
[launcher] server exited with code 143
```
### shots
```
-rw-r--r-- 1 runner runner  11724 Nov 16 18:20 lens_13_after_relog_probe11.png
-rw-r--r-- 1 runner runner  11789 Nov 16 18:20 lens_13_after_relog_probe12.png
-rw-r--r-- 1 runner runner  12379 Nov 16 18:20 lens_13_after_relog_probe13.png
-rw-r--r-- 1 runner runner  11950 Nov 16 18:20 lens_13_after_relog_probe14.png
-rw-r--r-- 1 runner runner  11724 Nov 16 18:20 lens_13_after_relog_probe15.png
-rw-r--r-- 1 runner runner  11605 Nov 16 18:21 lens_13_after_relog_probe16.png
-rw-r--r-- 1 runner runner  11605 Nov 16 18:21 lens_13_after_relog_probe17.png
-rw-r--r-- 1 runner runner  11605 Nov 16 18:21 lens_13_after_relog_probe18.png
-rw-r--r-- 1 runner runner  11605 Nov 16 18:21 lens_13_after_relog_probe19.png
-rw-r--r-- 1 runner runner  12415 Nov 16 18:19 lens_13_after_relog_probe2.png
-rw-r--r-- 1 runner runner  12089 Nov 16 18:19 lens_13_after_relog_probe3.png
-rw-r--r-- 1 runner runner  12091 Nov 16 18:19 lens_13_after_relog_probe4.png
-rw-r--r-- 1 runner runner  12632 Nov 16 18:19 lens_13_after_relog_probe5.png
-rw-r--r-- 1 runner runner  11917 Nov 16 18:19 lens_13_after_relog_probe6.png
-rw-r--r-- 1 runner runner  12104 Nov 16 18:19 lens_13_after_relog_probe7.png
-rw-r--r-- 1 runner runner  12199 Nov 16 18:19 lens_13_after_relog_probe8.png
-rw-r--r-- 1 runner runner  12217 Nov 16 18:20 lens_13_after_relog_probe9.png
drwxr-xr-x 2 runner runner   4096 Nov 16 18:44 lens_parity
drwxr-xr-x 2 runner runner   4096 Nov 16 18:44 parity
drwxr-xr-x 2 runner runner   4096 Nov 16 18:12 parity_dbg
drwxr-xr-x 2 runner runner   4096 Nov 16 18:47 parity_dbg8
-rw-r--r-- 1 runner runner  12219 Nov 16 18:21 probe_counts.txt
drwxr-xr-x 2 runner runner   4096 Nov 16 18:49 rim_mask
-rw-r--r-- 1 runner runner  56883 Nov 16 18:49 server_full.log
-rw-r--r-- 1 runner runner  71306 Nov 16 18:49 summary.md
```

**exit=1** failures=2 leg1=0 leg2=0
