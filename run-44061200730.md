- phase: 0 sanity (10:04:45)
### c-LoD visual run (-930,796, marker=red_concrete, vantage -1005,150 yaw 270)
- GL:     Vendor: Mesa (0xffffffff);    Device: llvmpipe (LLVM 21.1.8, 256 bits) (0xffffffff);OpenGL renderer string: llvmpipe (LLVM 21.1.8, 256 bits);OpenGL core profile version string: 4.5 (Core Profile) Mesa 26.0.8-1ubuntu0.3;
- boot + gates OK (10:05:12)
- phase: 1 server booted (10:05:12)
- sky-grid armed
- phase: 2 leg-1 client joined (10:05:33)
- joined session 1 (10:05:33)
- stream live: cells=65536
- phase: 3 scan populated (10:05:56)
- terrain range: 2 terraced mountains at (CX-4,CZ-20)/(CX-2,CZ+16), tops 185/190, grass terraces + stone caps, 66..82 m from the eye (inside the 83.5 m band, chunk edge 64 m)
- scan complete=1 (cells=cells=65536/65536)
- paint settled=1 (wire 748853p)
- baseline try 1: drift=1797 px
- baseline try 2: drift=1726 px
- baseline try 3: drift=1890 px
- baseline try 4: drift=2169 px
- baseline try 5: drift=1301 px
- lod_silhouette_cols=49 (LOD paint ON SCREEN at 75 m if > 20 — the march-drawn marker column; vanilla cannot render there, chunk edge 64 m; #255 re-measures 49, the ZONE_FAR=48 regression #256 and the control read 0)
- 3674 terrain-green px in the sky region (LOD MOUNTAINS on screen if > 5000; the range paints ColLow-green on low terraces via the fallback palette; flat-world baselines and the pack-free control read ~0)
- burst_close white px: 587 (renderer alive if > 50)
- burst_far white px: 629 (renderer alive in the marker zone if > 20)
- cut walk 40 m: 0 white px (> 20 = rendered)
- cut walk 60 m: 0 white px (> 20 = rendered)
- cut walk 100 m: 0 white px (> 20 = rendered)
- measured far-render cut: none m (G4 flap instrument; the decode proof needs >= 83 m from the y=151.6 eye)
- fill issued: 5x111x5 red_concrete at -930,796 (zone forceloaded ±48)
- phase: 4 marker filled (10:15:47)

### /clod-cells AFTER FILL (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 2 0 0x8400 0x2000
123 122 -940 784 71 0 1 [97, 119, 97] 2 0 0x8700 0x2000
124 122 -938 784 68 0 1 [97, 119, 97] 1 0 0x8400 0x2000
125 122 -936 784 62 1 1 [63, 118, 228] 1 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 0 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 0 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 108 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 294 0 0x8000 0x4100
130 122 -926 784 66 0 1 [134, 183, 131] 294 0 0x8200 0x4100
131 122 -924 784 75 0 1 [97, 119, 97] 294 0 0x8b00 0x2000
132 122 -922 784 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 122 -920 784 75 0 1 [97, 119, 97] 18446744073709551615 1 0x8b00 0x2000
134 122 -918 784 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x2000
122 123 -942 786 70 0 1 [97, 119, 97] 2 0 0x8600 0x2000
123 123 -940 786 65 0 1 [97, 119, 97] 2 0 0x8100 0x2000
124 123 -938 786 65 0 1 [97, 119, 97] 1 0 0x8100 0x2000
125 123 -936 786 62 1 1 [63, 118, 228] 1 0 0x7e00 0xd800
126 123 -934 786 62 1 1 [63, 118, 228] 0 0 0x7e00 0xd800
127 123 -932 786 62 1 1 [63, 118, 228] 0 0 0x7e00 0xd800
128 123 -930 786 62 1 1 [63, 118, 228] 108 0 0x7e00 0xd800
129 123 -928 786 73 0 1 [97, 119, 97] 294 0 0x8900 0x2000
130 123 -926 786 67 0 1 [134, 183, 131] 294 0 0x8300 0x4100
131 123 -924 786 69 0 1 [134, 183, 131] 294 0 0x8500 0x4100
132 123 -922 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 123 -920 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 123 -918 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
122 124 -942 788 72 0 1 [97, 119, 97] 2 0 0x8800 0x2000
123 124 -940 788 70 0 1 [97, 119, 97] 2 0 0x8600 0x2000
124 124 -938 788 68 0 1 [97, 119, 97] 1 0 0x8400 0x2000
125 124 -936 788 62 1 1 [63, 118, 228] 1 0 0x7e00 0xd800
126 124 -934 788 62 1 1 [63, 118, 228] 0 0 0x7e00 0xd800
127 124 -932 788 62 1 1 [63, 118, 228] 0 0 0x7e00 0xd800
128 124 -930 788 73 0 1 [97, 119, 97] 68 0 0x8900 0x2000
129 124 -928 788 76 0 1 [97, 119, 97] 294 0 0x8c00 0x2000
130 124 -926 788 73 0 1 [97, 119, 97] 294 0 0x8900 0x2000
131 124 -924 788 69 0 1 [134, 183, 131] 294 0 0x8500 0x4100
132 124 -922 788 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 124 -920 788 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 124 -918 788 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x2000
122 125 -942 790 74 0 1 [97, 119, 97] 2 0 0x8a00 0x2000
123 125 -940 790 73 0 1 [97, 119, 97] 2 0 0x8900 0x2000
124 125 -938 790 71 0 1 [97, 119, 97] 1 0 0x8700 0x2000
125 125 -936 790 62 1 1 [63, 118, 228] 1 0 0x7e00 0xd800
126 125 -934 790 62 1 1 [63, 118, 228] 0 0 0x7e00 0xd800
127 125 -932 790 62 1 1 [63, 118, 228] 0 0 0x7e00 0xd800
128 125 -930 790 62 1 1 [63, 118, 228] 108 0 0x7e00 0xd800
129 125 -928 790 73 0 1 [97, 119, 97] 68 0 0x8900 0x2000
130 125 -926 790 68 0 1 [134, 183, 131] 294 0 0x8400 0x4100
131 125 -924 790 69 0 1 [134, 183, 131] 294 0 0x8500 0x4100
132 125 -922 790 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
133 125 -920 790 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 125 -918 790 70 0 1 [134, 183, 131] 18446744073709551615 1 0x8600 0x4100
122 126 -942 792 74 0 1 [97, 119, 97] 2 0 0x8a00 0x2000
123 126 -940 792 73 0 1 [97, 119, 97] 2 0 0x8900 0x2000
124 126 -938 792 68 0 1 [97, 119, 97] 1 0 0x8400 0x2000
125 126 -936 792 62 1 1 [63, 118, 228] 1 0 0x7e00 0xd800
126 126 -934 792 62 1 1 [63, 118, 228] 0 0 0x7e00 0xd800
127 126 -932 792 62 1 1 [63, 118, 228] 0 0 0x7e00 0xd800
128 126 -930 792 62 1 1 [63, 118, 228] 108 0 0x7e00 0xd800
129 126 -928 792 67 0 1 [134, 183, 131] 68 0 0x8300 0x4100
130 126 -926 792 74 0 1 [97, 119, 97] 294 0 0x8a00 0x2000
131 126 -924 792 77 0 1 [97, 119, 97] 294 0 0x8d00 0x2000
132 126 -922 792 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 126 -920 792 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3200
134 126 -918 792 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x5000
122 127 -942 794 67 0 1 [134, 183, 131] 3 0 0x8300 0x4100
123 127 -940 794 68 0 1 [134, 183, 131] 2 0 0x8400 0x4100
124 127 -938 794 65 0 1 [134, 183, 131] 1 0 0x8100 0x4100
125 127 -936 794 62 1 1 [63, 118, 228] 1 0 0x7e00 0xd800
126 127 -934 794 62 1 1 [63, 118, 228] 0 0 0x7e00 0xd800
127 127 -932 794 62 1 1 [63, 118, 228] 0 0 0x7e00 0xd800
128 127 -930 794 62 1 1 [63, 118, 228] 108 0 0x7e00 0xd800
129 127 -928 794 66 0 1 [134, 183, 131] 68 0 0x8200 0x4100
130 127 -926 794 74 0 1 [97, 119, 97] 294 0 0x8a00 0x2000
131 127 -924 794 77 0 1 [97, 119, 97] 294 0 0x8d00 0x2000
132 127 -922 794 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 127 -920 794 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3200
134 127 -918 794 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x5000
122 128 -942 796 68 0 1 [134, 183, 131] 3 0 0x8400 0x4100
123 128 -940 796 72 0 1 [97, 119, 97] 2 0 0x8800 0x2000
124 128 -938 796 75 0 1 [97, 119, 97] 1 0 0x8b00 0x2000
125 128 -936 796 72 0 1 [97, 119, 97] 1 0 0x8800 0x2000
126 128 -934 796 62 1 1 [63, 118, 228] 0 0 0x7e00 0xd800
127 128 -932 796 62 1 1 [63, 118, 228] 0 0 0x7e00 0xd800
128 128 -930 796 62 1 1 [63, 118, 228] 108 0 0x7e00 0xd800
129 128 -928 796 66 0 1 [134, 183, 131] 68 0 0x8200 0x4100
130 128 -926 796 68 0 1 [134, 183, 131] 294 0 0x8400 0x4100
131 128 -924 796 74 0 1 [97, 119, 97] 294 0 0x8a00 0x2000
132 128 -922 796 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 128 -920 796 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
134 128 -918 796 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 129 -942 798 69 0 1 [134, 183, 131] 2 0 0x8500 0x4100
123 129 -940 798 75 0 1 [97, 119, 97] 2 0 0x8b00 0x2000
124 129 -938 798 78 0 1 [97, 119, 97] 3 0 0x8e00 0x2000
125 129 -936 798 75 0 1 [97, 119, 97] 1 0 0x8b00 0x2000
126 129 -934 798 62 1 1 [63, 118, 228] 0 0 0x7e00 0xd800
127 129 -932 798 62 1 1 [63, 118, 228] 0 0 0x7e00 0xd800
128 129 -930 798 62 1 1 [63, 118, 228] 108 0 0x7e00 0xd800
129 129 -928 798 71 0 1 [97, 119, 97] 68 0 0x8700 0x2000
130 129 -926 798 71 0 1 [97, 119, 97] 294 0 0x8700 0x2000
131 129 -924 798 69 0 1 [134, 183, 131] 294 0 0x8500 0x4100
132 129 -922 798 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 129 -920 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 129 -918 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 130 -942 800 70 0 1 [134, 183, 131] 2 0 0x8600 0x4100
123 130 -940 800 76 0 1 [97, 119, 97] 2 0 0x8c00 0x2000
124 130 -938 800 76 0 1 [97, 119, 97] 1 0 0x8c00 0x2000
125 130 -936 800 72 0 1 [97, 119, 97] 1 0 0x8800 0x2000
126 130 -934 800 62 1 1 [63, 118, 228] 0 0 0x7e00 0xd800
127 130 -932 800 62 1 1 [63, 118, 228] 0 0 0x7e00 0xd800
128 130 -930 800 62 1 1 [63, 118, 228] 108 0 0x7e00 0xd800
129 130 -928 800 71 0 1 [97, 119, 97] 68 0 0x8700 0x2000
130 130 -926 800 71 0 1 [97, 119, 97] 294 0 0x8700 0x2000
131 130 -924 800 69 0 1 [134, 183, 131] 294 0 0x8500 0x4100
132 130 -922 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
133 130 -920 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 130 -918 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 131 -942 802 78 0 1 [97, 119, 97] 4 0 0x8e00 0x2000
123 131 -940 802 77 0 1 [97, 119, 97] 2 0 0x8d00 0x2000
124 131 -938 802 77 0 1 [97, 119, 97] 1 0 0x8d00 0x2000
125 131 -936 802 66 0 1 [134, 183, 131] 1 0 0x8200 0x4100
126 131 -934 802 62 1 1 [63, 118, 228] 0 0 0x7e00 0xd800
127 131 -932 802 62 1 1 [63, 118, 228] 0 0 0x7e00 0xd800
128 131 -930 802 62 1 1 [63, 118, 228] 108 0 0x7e00 0xd800
129 131 -928 802 62 1 1 [63, 118, 228] 334 0 0x7e00 0xd800
130 131 -926 802 66 0 1 [134, 183, 131] 294 0 0x8200 0x4100
131 131 -924 802 68 0 1 [134, 183, 131] 294 0 0x8400 0x4100
132 131 -922 802 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 131 -920 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 131 -918 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 132 -942 804 79 0 1 [97, 119, 97] 4 0 0x8f00 0x2000
123 132 -940 804 76 0 1 [97, 119, 97] 2 0 0x8c00 0x2000
124 132 -938 804 76 0 1 [97, 119, 97] 1 0 0x8c00 0x2000
125 132 -936 804 76 0 1 [97, 119, 97] 1 0 0x8c00 0x2000
126 132 -934 804 62 1 1 [63, 118, 228] 0 0 0x7e00 0xd800
127 132 -932 804 62 1 1 [63, 118, 228] 0 0 0x7e00 0xd800
128 132 -930 804 62 1 1 [63, 118, 228] 108 0 0x7e00 0xd800
129 132 -928 804 62 1 1 [63, 118, 228] 334 0 0x7e00 0xd800
130 132 -926 804 65 0 1 [134, 183, 131] 294 0 0x8100 0x4100
131 132 -924 804 77 0 1 [97, 119, 97] 294 0 0x8d00 0x2000
132 132 -922 804 79 0 1 [97, 119, 97] 294 0 0x8f00 0x2000
133 132 -920 804 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 132 -918 804 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x2000
122 133 -942 806 78 0 1 [97, 119, 97] 4 0 0x8e00 0x2000
123 133 -940 806 69 0 1 [134, 183, 131] 2 0 0x8500 0x4100
124 133 -938 806 155 0 1 [134, 183, 131] 2 0 0xdb00 0x4100
125 133 -936 806 155 0 1 [134, 183, 131] 2 0 0xdb00 0x4100
126 133 -934 806 155 0 1 [134, 183, 131] 1 0 0xdb00 0x4100
127 133 -932 806 155 0 1 [134, 183, 131] 1 0 0xdb00 0x4100
128 133 -930 806 155 0 1 [134, 183, 131] 0 0 0xdb00 0x4100
129 133 -928 806 155 0 1 [134, 183, 131] 68 0 0xdb00 0x4100
130 133 -926 806 155 0 1 [134, 183, 131] 68 0 0xdb00 0x4100
131 133 -924 806 67 0 1 [134, 183, 131] 294 0 0x8300 0x4100
132 133 -922 806 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 133 -920 806 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 133 -918 806 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x2000
122 134 -942 808 79 0 1 [97, 119, 97] 4 0 0x8f00 0x2000
123 134 -940 808 68 0 1 [134, 183, 131] 2 0 0x8400 0x4100
124 134 -938 808 155 0 1 [134, 183, 131] 2 0 0xdb00 0x4100
125 134 -936 808 180 0 1 [134, 183, 131] 68 0 0xf400 0x4100
126 134 -934 808 180 0 1 [134, 183, 131] 68 0 0xf400 0x4100
127 134 -932 808 180 0 1 [134, 183, 131] 294 0 0xf400 0x4100
128 134 -930 808 180 0 1 [134, 183, 131] 294 0 0xf400 0x4100
129 134 -928 808 180 0 1 [134, 183, 131] 294 0 0xf400 0x4100
130 134 -926 808 155 0 1 [134, 183, 131] 68 0 0xdb00 0x4100
131 134 -924 808 67 0 1 [134, 183, 131] 294 0 0x8300 0x4100
132 134 -922 808 68 0 1 [134, 183, 131] 18446744073709551615 1 0x8400 0x4100
133 134 -920 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 134 -918 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
```
### stream diag after fill:
[c-LoD] stream: tick 6300 65536/65536 cells, lap 945, cone 47151 zone 2859 dirty 939 (q 11516) ewin 183 rbeat 9p/1 rearm 0c/0t rescan 1675711c/2558e, edits +5817p/4419c/697e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 332 forced 180, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 982389p/2493b fail 0 ~35 kB/s cache 13582/24576 h968807 m13582 e0, srbeat 72391s/9q drop 65stale/101undec, ereb 48342s/9q, sweep 0r/80d
[10:15:53 INFO]: [c-LoD] stream: tick 6300 65536/65536 cells, lap 945, cone 47151 zone 2859 dirty 939 (q 11516) ewin 183 rbeat 9p/1 rearm 0c/0t rescan 1675711c/2558e, edits +5817p/4419c/697e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 332 forced 180, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 982389p/2493b fail 0 ~35 kB/s cache 13582/24576 h968807 m13582 e0, srbeat 72391s/9q drop 65stale/101undec, ereb 48342s/9q, sweep 0r/80d
- carrier ladder y=155: 17324 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=165: 17264 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=168: 17264 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=172: 17264 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=176: 17264 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=180: 17264 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=185: 17264 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- probe 0: red=0 paint=2685 lod_silhouette_cols=43 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=16 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 1: red=0 paint=2685 lod_silhouette_cols=43 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=16 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 2: red=0 paint=3347 lod_silhouette_cols=43 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=16 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 3: red=0 paint=4256 lod_silhouette_cols=43 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=16 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 4: red=0 paint=2762 lod_silhouette_cols=43 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=16 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 5: red=0 paint=2829 lod_silhouette_cols=43 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=16 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 6: red=0 paint=3339 lod_silhouette_cols=43 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=16 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 7: red=0 paint=3755 lod_silhouette_cols=43 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=16 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 8: red=0 paint=3626 lod_silhouette_cols=43 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=16 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 9: red=0 paint=2848 lod_silhouette_cols=43 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=16 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 10: red=0 paint=2805 lod_silhouette_cols=43 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=16 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 11: red=0 paint=2805 lod_silhouette_cols=43 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=16 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 12: red=0 paint=2805 lod_silhouette_cols=43 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=16 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 13: red=0 paint=2805 lod_silhouette_cols=43 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=16 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 14: red=0 paint=3821 lod_silhouette_cols=43 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=16 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 15: red=0 paint=3342 lod_silhouette_cols=43 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=4 hits_hi=16 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 16: red=0 paint=4331 lod_silhouette_cols=43 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=5 hits_hi=16 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 17: red=0 paint=2977 lod_silhouette_cols=43 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=5 hits_hi=16 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 18: red=0 paint=3098 lod_silhouette_cols=43 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=5 hits_hi=16 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 19: red=0 paint=5044 lod_silhouette_cols=43 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=5 hits_hi=17 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 20: red=0 paint=5554 lod_silhouette_cols=43 lens_red=0 lens_black=568 lens_water=0 lens_dark=192626 lens_zero=0 rot=decoded cam_x_b=? cam_z_b=? az_b=? el_b=? fog=live fog_end_b=? strobe_hits=8 hits_hi=8 mask_hits=8 (need red>=60 paint>=300 lod>20)
- probe 21: red=0 paint=5414 lod_silhouette_cols=43 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=5 hits_hi=17 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 22: red=0 paint=5865 lod_silhouette_cols=43 lens_red=0 lens_black=568 lens_water=0 lens_dark=192626 lens_zero=0 rot=decoded cam_x_b=? cam_z_b=? az_b=? el_b=? fog=live fog_end_b=? strobe_hits=8 hits_hi=8 mask_hits=8 (need red>=60 paint>=300 lod>20)
- probe 23: red=0 paint=5167 lod_silhouette_cols=43 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=5 hits_hi=17 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 24: red=0 paint=5453 lod_silhouette_cols=43 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=5 hits_hi=17 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 25: red=0 paint=6027 lod_silhouette_cols=43 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=5 hits_hi=17 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 26: red=0 paint=5574 lod_silhouette_cols=43 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=5 hits_hi=17 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 27: red=0 paint=6594 lod_silhouette_cols=43 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=5 hits_hi=17 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 28: red=0 paint=5141 lod_silhouette_cols=43 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=5 hits_hi=17 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 29: red=0 paint=5296 lod_silhouette_cols=43 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=5 hits_hi=17 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 30: red=0 paint=5858 lod_silhouette_cols=43 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=5 hits_hi=17 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 31: red=0 paint=6090 lod_silhouette_cols=43 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=5 hits_hi=17 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 32: red=0 paint=6099 lod_silhouette_cols=43 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=5 hits_hi=17 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 33: red=0 paint=6422 lod_silhouette_cols=43 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=5 hits_hi=17 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 34: red=0 paint=5228 lod_silhouette_cols=43 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=5 hits_hi=17 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 35: red=0 paint=5534 lod_silhouette_cols=43 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=5 hits_hi=17 mask_hits=30 (need red>=60 paint>=300 lod>20)
- phase: 5 leg-1 probes done (10:21:39)
- **FAIL:** leg-1 never saw the marker in 24 probes (red < 60 everywhere) — the edit never reached the LOD paint; this failure was SILENT before v6.31.1
### stream diag after probes:
[10:21:24 INFO]: [c-LoD] stream: tick 9600 65536/65536 cells, lap 1152, cone 74202 zone 4564 dirty 980 (q 11545) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 2740692c/2618e, edits +7140p/5328c/967e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 332 forced 180, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 1334763p/3073b fail 0 ~73 kB/s cache 13603/24576 h1321160 m13603 e0, srbeat 87922s/592q drop 71stale/101undec, ereb 49575s/0q, sweep 0r/86d
[c-LoD] stream: tick 9700 65536/65536 cells, lap 1160, cone 75354 zone 4807 dirty 980 (q 11545) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 2774358c/2618e, edits +7141p/5329c/967e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 332 forced 180, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 1350560p/3093b fail 0 ~83 kB/s cache 13603/24576 h1336957 m13603 e0, srbeat 88514s/777q drop 71stale/101undec, ereb 49575s/0q, sweep 0r/95d
[10:21:34 INFO]: [c-LoD] stream: tick 9700 65536/65536 cells, lap 1160, cone 75354 zone 4807 dirty 980 (q 11545) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 2774358c/2618e, edits +7141p/5329c/967e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 332 forced 180, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 1350560p/3093b fail 0 ~83 kB/s cache 13603/24576 h1336957 m13603 e0, srbeat 88514s/777q drop 71stale/101undec, ereb 49575s/0q, sweep 0r/95d
- parity debug tour: 13 vantages (session-1 client, DEBUG pack — evidence only, the gate pairs the §7b world session)
- parity debug tour done: 13 shots in parity_dbg/ (the §7b world session feeds the gate)
- joined session 2 (10:24:30)
- leg-2 baseline scene px (x<940): 0 (0 = the relog client paints no world at all)
- leg-2 renderer probe NEAR (20 m) white px: 0 raw / 0 changed-vs-baseline (> 20 = the client renders particles)
- leg-2 renderer probe FAR (80 m, marker zone) white px: 581 raw / 581 changed-vs-baseline (> 20 = no distance cut; ~0 with NEAR alive = particles beyond the client chunk radius do not render)
- leg-2 carrier probe green dust (80 m): changed-green px 0 / total changed 828 vs baseline (> 100 green = a stable green carrier exists; total > 0 with green ~0 = the dust renders but fog washes it — a ruler problem, not a carrier problem)
- phase: 8 leg-2 relog probes (10:29:54)
- relog probe 0: red=0 scene=10 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=0 mask_hits=1 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 1: red=0 scene=125238 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=0 hits_hi=6 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 2: red=7079 scene=121160 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 fog=live fog_end_b=23 strobe_hits=1 hits_hi=8 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- **LEG 2 PASS** (marker persisted over relog)
- phase: 9 leg-2 done (10:30:15)

### client log forensics (pipeline/particle/atlas — v6.8.2)
```
[10:24:19] [main/DEBUG]: Loading LWJGL system
WARNING: java.lang.System::load has been called by org.lwjgl.system.Library$$Lambda/0x00000000101e3d60 in an unnamed module (file:/home/runner/mc-test/client/libraries/org/lwjgl/lwjgl/3.4.3/lwjgl-3.4.3.jar)
WARNING: sun.misc.Unsafe::objectFieldOffset has been called by org.lwjgl.system.MemoryBackendUnsafeLegacy (file:/home/runner/mc-test/client/libraries/org/lwjgl/lwjgl/3.4.3/lwjgl-3.4.3.jar)
WARNING: Please consider reporting this to the maintainers of class org.lwjgl.system.MemoryBackendUnsafeLegacy
[10:24:19] [main/DEBUG]: Loading shaderc
[10:24:19] [main/DEBUG]: Loading OpenGL
[10:24:24] [Render thread/ERROR]: Error parsing option value 4 for option Simulation Distance: Value 4 outside of range [5:33]
[10:24:24] [Render thread/INFO]: Backend library: LWJGL version 3.4.3+4
[10:24:24] [Download-2/ERROR]: Failed to fetch user properties
com.mojang.authlib.exceptions.InvalidCredentialsException: Status: 401
	at com.mojang.authlib.exceptions.MinecraftClientHttpException.toAuthenticationException(MinecraftClientHttpException.java:70)
Caused by: MinecraftClientHttpException[type=HTTP_ERROR, status=401, response=ErrorResponse[path=/player/attributes, error=null, errorMessage=null, details=null]]
	at com.mojang.authlib.minecraft.client.MinecraftClient.buildHttpException(MinecraftClient.java:170)
[10:24:24] [Render thread/INFO]: Using graphics backend OpenGL, using drivers: 4.5 (Core Profile) Mesa 26.0.8-1ubuntu0.3
[10:24:24] [Render thread/INFO]: Using graphics device: llvmpipe (LLVM 21.1.8, 256 bits) (Mesa)
[10:24:24] [Render thread/INFO]: Using graphics device extensions: GL_ARB_multi_draw_indirect, GL_ARB_buffer_storage, GL_ARB_shader_draw_parameters, GL_ARB_base_instance, GL_KHR_debug, GL_ARB_draw_indirect, GL_ARB_clip_control, GL_ARB_vertex_attrib_binding, GL_ARB_direct_state_access, GL_EXT_texture_filter_anisotropic
[10:24:24] [Download-1/INFO]: Could not authorize you against Realms server: java.lang.RuntimeException: Failed to parse into SignedJWT: 0
[10:24:24] [Download-1/ERROR]: Failed to fetch Realms feature flags
com.mojang.realmsclient.exception.RealmsServiceException: Realms authentication error with message 'java.lang.RuntimeException: Failed to parse into SignedJWT: 0'
[10:24:24] [Render thread/INFO]: Reloading ResourceManager: vanilla, vanilla
[10:24:27] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[10:24:27] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[10:24:27] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[10:24:27] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[10:24:27] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[10:24:27] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[10:24:27] [Render thread/INFO]: Created: 2048x2048x4 minecraft:textures/atlas/blocks.png-atlas
[10:24:27] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[10:24:27] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[10:24:27] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[10:24:27] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[10:24:27] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
[10:24:34] [Render thread/INFO]: Reloading ResourceManager: vanilla, vanilla, server/00000000/6b1f2f7e-9c0d-4a4b-8b0a-b10d1b4ab100
[10:24:36] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[10:24:36] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[10:24:36] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[10:24:36] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[10:24:36] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[10:24:36] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[10:24:36] [Render thread/INFO]: Created: 2048x2048x4 minecraft:textures/atlas/blocks.png-atlas
[10:24:36] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[10:24:36] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[10:24:36] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[10:24:36] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[10:24:36] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
```

### /clod-cells AFTER PROBES (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 36 0 0x8400 0x2000
123 122 -940 784 71 0 1 [97, 119, 97] 36 0 0x8700 0x2000
124 122 -938 784 68 0 1 [97, 119, 97] 35 0 0x8400 0x2000
125 122 -936 784 62 1 1 [63, 118, 228] 75 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 74 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 73 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 73 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 142 0 0x8000 0x4100
130 122 -926 784 66 0 1 [134, 183, 131] 142 0 0x8200 0x4100
131 122 -924 784 75 0 1 [97, 119, 97] 142 0 0x8b00 0x2000
132 122 -922 784 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 122 -920 784 75 0 1 [97, 119, 97] 18446744073709551615 1 0x8b00 0x2000
134 122 -918 784 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x2000
122 123 -942 786 70 0 1 [97, 119, 97] 36 0 0x8600 0x2000
123 123 -940 786 65 0 1 [97, 119, 97] 36 0 0x8100 0x2000
124 123 -938 786 65 0 1 [97, 119, 97] 35 0 0x8100 0x2000
125 123 -936 786 62 1 1 [63, 118, 228] 75 0 0x7e00 0xd800
126 123 -934 786 62 1 1 [63, 118, 228] 74 0 0x7e00 0xd800
127 123 -932 786 62 1 1 [63, 118, 228] 74 0 0x7e00 0xd800
128 123 -930 786 62 1 1 [63, 118, 228] 73 0 0x7e00 0xd800
129 123 -928 786 73 0 1 [97, 119, 97] 142 0 0x8900 0x2000
130 123 -926 786 67 0 1 [134, 183, 131] 142 0 0x8300 0x4100
131 123 -924 786 69 0 1 [134, 183, 131] 142 0 0x8500 0x4100
132 123 -922 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 123 -920 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 123 -918 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
122 124 -942 788 72 0 1 [97, 119, 97] 36 0 0x8800 0x2000
123 124 -940 788 70 0 1 [97, 119, 97] 36 0 0x8600 0x2000
124 124 -938 788 68 0 1 [97, 119, 97] 35 0 0x8400 0x2000
125 124 -936 788 62 1 1 [63, 118, 228] 75 0 0x7e00 0xd800
126 124 -934 788 62 1 1 [63, 118, 228] 74 0 0x7e00 0xd800
127 124 -932 788 62 1 1 [63, 118, 228] 74 0 0x7e00 0xd800
128 124 -930 788 73 0 1 [97, 119, 97] 33 0 0x8900 0x2000
129 124 -928 788 76 0 1 [97, 119, 97] 142 0 0x8c00 0x2000
130 124 -926 788 73 0 1 [97, 119, 97] 142 0 0x8900 0x2000
131 124 -924 788 69 0 1 [134, 183, 131] 142 0 0x8500 0x4100
132 124 -922 788 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 124 -920 788 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 124 -918 788 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x2000
122 125 -942 790 74 0 1 [97, 119, 97] 36 0 0x8a00 0x2000
123 125 -940 790 73 0 1 [97, 119, 97] 36 0 0x8900 0x2000
124 125 -938 790 71 0 1 [97, 119, 97] 35 0 0x8700 0x2000
125 125 -936 790 62 1 1 [63, 118, 228] 75 0 0x7e00 0xd800
126 125 -934 790 62 1 1 [63, 118, 228] 74 0 0x7e00 0xd800
127 125 -932 790 62 1 1 [63, 118, 228] 74 0 0x7e00 0xd800
128 125 -930 790 62 1 1 [63, 118, 228] 73 0 0x7e00 0xd800
129 125 -928 790 73 0 1 [97, 119, 97] 142 0 0x8900 0x2000
130 125 -926 790 68 0 1 [134, 183, 131] 142 0 0x8400 0x4100
131 125 -924 790 69 0 1 [134, 183, 131] 142 0 0x8500 0x4100
132 125 -922 790 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
133 125 -920 790 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 125 -918 790 70 0 1 [134, 183, 131] 18446744073709551615 1 0x8600 0x4100
122 126 -942 792 74 0 1 [97, 119, 97] 36 0 0x8a00 0x2000
123 126 -940 792 73 0 1 [97, 119, 97] 36 0 0x8900 0x2000
124 126 -938 792 68 0 1 [97, 119, 97] 35 0 0x8400 0x2000
125 126 -936 792 62 1 1 [63, 118, 228] 75 0 0x7e00 0xd800
126 126 -934 792 62 1 1 [63, 118, 228] 74 0 0x7e00 0xd800
127 126 -932 792 62 1 1 [63, 118, 228] 74 0 0x7e00 0xd800
128 126 -930 792 62 1 1 [63, 118, 228] 73 0 0x7e00 0xd800
129 126 -928 792 67 0 1 [134, 183, 131] 142 0 0x8300 0x4100
130 126 -926 792 74 0 1 [97, 119, 97] 142 0 0x8a00 0x2000
131 126 -924 792 77 0 1 [97, 119, 97] 142 0 0x8d00 0x2000
132 126 -922 792 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 126 -920 792 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3200
134 126 -918 792 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x5000
122 127 -942 794 67 0 1 [134, 183, 131] 37 0 0x8300 0x4100
123 127 -940 794 68 0 1 [134, 183, 131] 36 0 0x8400 0x4100
124 127 -938 794 65 0 1 [134, 183, 131] 35 0 0x8100 0x4100
125 127 -936 794 62 1 1 [63, 118, 228] 75 0 0x7e00 0xd800
126 127 -934 794 62 1 1 [63, 118, 228] 74 0 0x7e00 0xd800
127 127 -932 794 170 0 1 [142, 33, 33] 34 0 0xea00 0x0a00
128 127 -930 794 170 0 1 [142, 33, 33] 33 0 0xea00 0x0a00
129 127 -928 794 170 0 1 [142, 33, 33] 33 0 0xea00 0x0a00
130 127 -926 794 74 0 1 [97, 119, 97] 142 0 0x8a00 0x2000
131 127 -924 794 77 0 1 [97, 119, 97] 142 0 0x8d00 0x2000
132 127 -922 794 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 127 -920 794 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3200
134 127 -918 794 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x5000
122 128 -942 796 68 0 1 [134, 183, 131] 37 0 0x8400 0x4100
123 128 -940 796 72 0 1 [97, 119, 97] 36 0 0x8800 0x2000
124 128 -938 796 75 0 1 [97, 119, 97] 35 0 0x8b00 0x2000
125 128 -936 796 72 0 1 [97, 119, 97] 35 0 0x8800 0x2000
126 128 -934 796 62 1 1 [63, 118, 228] 74 0 0x7e00 0xd800
127 128 -932 796 170 0 1 [142, 33, 33] 34 0 0xea00 0x0a00
128 128 -930 796 170 0 1 [142, 33, 33] 33 0 0xea00 0x0a00
129 128 -928 796 170 0 1 [142, 33, 33] 33 0 0xea00 0x0a00
130 128 -926 796 68 0 1 [134, 183, 131] 142 0 0x8400 0x4100
131 128 -924 796 74 0 1 [97, 119, 97] 142 0 0x8a00 0x2000
132 128 -922 796 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 128 -920 796 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
134 128 -918 796 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 129 -942 798 69 0 1 [134, 183, 131] 36 0 0x8500 0x4100
123 129 -940 798 75 0 1 [97, 119, 97] 36 0 0x8b00 0x2000
124 129 -938 798 78 0 1 [97, 119, 97] 37 0 0x8e00 0x2000
125 129 -936 798 75 0 1 [97, 119, 97] 35 0 0x8b00 0x2000
126 129 -934 798 62 1 1 [63, 118, 228] 74 0 0x7e00 0xd800
127 129 -932 798 170 0 1 [142, 33, 33] 34 0 0xea00 0x0a00
128 129 -930 798 170 0 1 [142, 33, 33] 33 0 0xea00 0x0a00
129 129 -928 798 170 0 1 [142, 33, 33] 33 0 0xea00 0x0a00
130 129 -926 798 71 0 1 [97, 119, 97] 142 0 0x8700 0x2000
131 129 -924 798 69 0 1 [134, 183, 131] 142 0 0x8500 0x4100
132 129 -922 798 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 129 -920 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 129 -918 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 130 -942 800 70 0 1 [134, 183, 131] 36 0 0x8600 0x4100
123 130 -940 800 76 0 1 [97, 119, 97] 36 0 0x8c00 0x2000
124 130 -938 800 76 0 1 [97, 119, 97] 35 0 0x8c00 0x2000
125 130 -936 800 72 0 1 [97, 119, 97] 35 0 0x8800 0x2000
126 130 -934 800 62 1 1 [63, 118, 228] 74 0 0x7e00 0xd800
127 130 -932 800 62 1 1 [63, 118, 228] 74 0 0x7e00 0xd800
128 130 -930 800 62 1 1 [63, 118, 228] 73 0 0x7e00 0xd800
129 130 -928 800 71 0 1 [97, 119, 97] 142 0 0x8700 0x2000
130 130 -926 800 71 0 1 [97, 119, 97] 142 0 0x8700 0x2000
131 130 -924 800 69 0 1 [134, 183, 131] 142 0 0x8500 0x4100
132 130 -922 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
133 130 -920 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 130 -918 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 131 -942 802 78 0 1 [97, 119, 97] 38 0 0x8e00 0x2000
123 131 -940 802 77 0 1 [97, 119, 97] 36 0 0x8d00 0x2000
124 131 -938 802 77 0 1 [97, 119, 97] 35 0 0x8d00 0x2000
125 131 -936 802 66 0 1 [134, 183, 131] 35 0 0x8200 0x4100
126 131 -934 802 62 1 1 [63, 118, 228] 74 0 0x7e00 0xd800
127 131 -932 802 62 1 1 [63, 118, 228] 74 0 0x7e00 0xd800
128 131 -930 802 62 1 1 [63, 118, 228] 73 0 0x7e00 0xd800
129 131 -928 802 62 1 1 [63, 118, 228] 183 0 0x7e00 0xd800
130 131 -926 802 66 0 1 [134, 183, 131] 142 0 0x8200 0x4100
131 131 -924 802 68 0 1 [134, 183, 131] 142 0 0x8400 0x4100
132 131 -922 802 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 131 -920 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 131 -918 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 132 -942 804 79 0 1 [97, 119, 97] 38 0 0x8f00 0x2000
123 132 -940 804 76 0 1 [97, 119, 97] 36 0 0x8c00 0x2000
124 132 -938 804 76 0 1 [97, 119, 97] 35 0 0x8c00 0x2000
125 132 -936 804 76 0 1 [97, 119, 97] 35 0 0x8c00 0x2000
126 132 -934 804 62 1 1 [63, 118, 228] 74 0 0x7e00 0xd800
127 132 -932 804 62 1 1 [63, 118, 228] 74 0 0x7e00 0xd800
128 132 -930 804 62 1 1 [63, 118, 228] 73 0 0x7e00 0xd800
129 132 -928 804 62 1 1 [63, 118, 228] 182 0 0x7e00 0xd800
130 132 -926 804 65 0 1 [134, 183, 131] 142 0 0x8100 0x4100
131 132 -924 804 77 0 1 [97, 119, 97] 142 0 0x8d00 0x2000
132 132 -922 804 79 0 1 [97, 119, 97] 142 0 0x8f00 0x2000
133 132 -920 804 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 132 -918 804 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x2000
122 133 -942 806 78 0 1 [97, 119, 97] 38 0 0x8e00 0x2000
123 133 -940 806 69 0 1 [134, 183, 131] 36 0 0x8500 0x4100
124 133 -938 806 155 0 1 [134, 183, 131] 36 0 0xdb00 0x4100
125 133 -936 806 155 0 1 [134, 183, 131] 36 0 0xdb00 0x4100
126 133 -934 806 155 0 1 [134, 183, 131] 35 0 0xdb00 0x4100
127 133 -932 806 155 0 1 [134, 183, 131] 35 0 0xdb00 0x4100
128 133 -930 806 155 0 1 [134, 183, 131] 34 0 0xdb00 0x4100
129 133 -928 806 155 0 1 [134, 183, 131] 33 0 0xdb00 0x4100
130 133 -926 806 155 0 1 [134, 183, 131] 33 0 0xdb00 0x4100
131 133 -924 806 67 0 1 [134, 183, 131] 148 0 0x8300 0x4100
132 133 -922 806 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 133 -920 806 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 133 -918 806 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x2000
122 134 -942 808 79 0 1 [97, 119, 97] 38 0 0x8f00 0x2000
123 134 -940 808 68 0 1 [134, 183, 131] 36 0 0x8400 0x4100
124 134 -938 808 155 0 1 [134, 183, 131] 36 0 0xdb00 0x4100
125 134 -936 808 180 0 1 [134, 183, 131] 33 0 0xf400 0x4100
126 134 -934 808 180 0 1 [134, 183, 131] 33 0 0xf400 0x4100
127 134 -932 808 180 0 1 [134, 183, 131] 148 0 0xf400 0x4100
128 134 -930 808 180 0 1 [134, 183, 131] 148 0 0xf400 0x4100
129 134 -928 808 180 0 1 [134, 183, 131] 148 0 0xf400 0x4100
130 134 -926 808 155 0 1 [134, 183, 131] 33 0 0xdb00 0x4100
131 134 -924 808 67 0 1 [134, 183, 131] 148 0 0x8300 0x4100
132 134 -922 808 68 0 1 [134, 183, 131] 18446744073709551615 1 0x8400 0x4100
133 134 -920 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 134 -918 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
```
- edit channel: 0 linkage errors, 0 failed event dispatches (both must be 0)
- phase: 6 parity world boot (10:30:15)
- parity world: view-distance=8 (the voxy leg's server value)
- parity world: joined session 3 (10:30:53)
- parity world: scan populated (cells=65536; the march paints from these buffers)
- parity world tour: 13 vantages (production pack, vd 8, shared structures)
- coverage sweeps: 72 eye passes = 36 positions (12 az x r144/200 + 12 off-az x r172, y=160) x 2 looks (outward+inward), settle-by-laps >= 6 s (q stable 3x1s), deadline 12 s/pos, no shots — the v6.26.0 data-coverage lever + v6.26.6 lottery kill; probe rides parity/view_probe.md
- parity drain-settle TIMEOUT: queue 12563 -> 12549 > 11500 after 150s — continuing anyway; the queue did not drain without eye proximity (priority-lane evidence, the 01/05/10 dip attribution reads from the tour deltas)
- pre-tour pass done: 13 vantages x 10s (the tour now reads the second OR-merge lap; the rim/dip transport race reads from the tour deltas)
- parity world tour done: 13 shots, 0 night-guard renames in parity/ (the gate pairs voxy/<name>.png vs parity/<name>.png)
- phase: 7 parity tour done (10:52:45)
- sparse re-beat channel at tour end: streamed=93506 q=0 dropped_stale=73 dropped_undecodable=9753 (frac=8 boot; undecodable-dominant = the rim words die in their decodability window, stale-dominant = edit churn owns the cells)
### parity world stream counters (session 3, tail + maxima)
[c-LoD] stream: tick 13200 65536/65536 cells, lap 1290, cone 82636 zone 28407 dirty 7852 (q 12523) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 9010814c/898e, edits +18110p/13720c/2837e drop 0, spawned 0 / failed 0, cam0 yaw -180 pitch 5, wish 0 forced 905, native 20160c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 1644825p/3122b fail 0 ~0 kB/s cache 24241/24576 h1197535 m447290 e423049, srbeat 93506s/0q drop 73stale/9753undec, ereb 29198s/3q, sweep 0r/78d
[10:52:44 INFO]: [c-LoD] stream: tick 13200 65536/65536 cells, lap 1290, cone 82636 zone 28407 dirty 7852 (q 12523) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 9010814c/898e, edits +18110p/13720c/2837e drop 0, spawned 0 / failed 0, cam0 yaw -180 pitch 5, wish 0 forced 905, native 20160c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 1644825p/3122b fail 0 ~0 kB/s cache 24241/24576 h1197535 m447290 e423049, srbeat 93506s/0q drop 73stale/9753undec, ereb 29198s/3q, sweep 0r/78d
- parity stream max cone: 82636
- parity stream max zone: 28407
- parity stream max wire bundle: bundle
### parity world capture lens (debug chain on the §7b world)
lens_red=0 lens_black=0 lens_water=0 lens_dark=0 lens_zero=0 rot=none scan=0 carriers=0
- parity mesh probe @png_y=2: x306=(151,184,255) x307=(151,184,255) x308=(151,184,255) x309h=(151,184,255) x310c=(151,184,255)
- parity mesh probe @png_y=717: x306=(50,63,52) x307=(45,65,52) x308=(47,67,54) x309h=(45,64,51) x310c=(43,61,50)
- parity mesh engagement: magenta-ish=0 of 921600 sampled px
- mask8: the un-fog engagement masks shot on the PARITY WORLD (vd 8) — parity_dbg8/01m|07m|10m
- mask8 01m: green=1024 red=0 blue=95776 dim=26800 (px, sampled /16)
- mask8 07m: green=11408 red=0 blue=220656 dim=5808 (px, sampled /16)
- mask8 10m: green=5920 red=480 blue=233392 dim=80576 (px, sampled /16)
- mesh probe @png_y=2: x306=(255,255,255) x307=(13,254,0) x308=(0,254,0) x309h=(46,0,0) x310c=(0,0,0)
- mesh probe @png_y=717: x306=(255,255,255) x307=(13,254,0) x308=(0,254,0) x309h=(46,0,0) x310c=(0,0,0)
- mesh engagement: magenta-ish=0 of 921600 sampled px (the vanilla-path paint coverage)
- mask8 fog state @png_y=2: raw (300)=(151,184,255) (301)=(151,184,255)
- mask8 fog state @png_y=717: raw (300)=(0,128,8) (301)=(179,206,255)
- mask8 fog state decoded: envStart=0 envEnd=1024 rdEnd=128 fogColor=(179,206,255)
- buffer dump 10m: marked h=196/2048 c=35/2048; height windows r0<142:63 r1:18 r2>=202:115 maxByte=255
- buffer dump 10m: RANK-2-WINDOW HEIGHTS (the fake-wall class): (+22,-32)h=181, (+36,-32)h=183, (+38,-32)h=189, (+40,-32)h=191, (+78,-32)h=188, (+76,-30)h=191, (+78,-30)h=191, (+8,-28)h=143, (+72,-28)h=149, (-6,-26)h=187
- rim mask: scan populated (cells=65536; the state paint reads the same buffers the march reads)
- phase: 10 rim mask lane (10:56:59)
- rim mask: decode-state frames shot (rim_mask/00|01|12 — GREEN full decode / RED colorless / BLUE heightless; 00 = the leg-1 tower eye, 01 = the near-tower contrast, 12 = the tower re-read; the inherited 07/12 vantages sat 760 blocks outside the streamed disk and 01 sat INSIDE the marker column — their earlier verdicts were classifier noise, purged in v6.33.1)
- rim mask 00: green=29360 red=35440 blue=239968 yellow=63328 (px, sampled /16) -> DOMINANT BLUE: rim = HEIGHTS NEVER DECODED — a data-coverage class: the state paint cannot even see the cells
- rim mask 01: green=115584 red=36736 blue=238832 yellow=47664 (px, sampled /16) -> DOMINANT BLUE: rim = HEIGHTS NEVER DECODED — a data-coverage class: the state paint cannot even see the cells
- rim mask 12: green=76736 red=27888 blue=277760 yellow=53600 (px, sampled /16) -> DOMINANT BLUE: rim = HEIGHTS NEVER DECODED — a data-coverage class: the state paint cannot even see the cells

### boot / stream
```
[c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 149754 bytes, sha1 5dc283742995ead8546b0a2bbfadc892d3c58358 — zero textures, zero world data, one push per join
[c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[10:56:18 INFO]: [c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 149754 bytes, sha1 5dc283742995ead8546b0a2bbfadc892d3c58358 — zero textures, zero world data, one push per join
[10:56:18 INFO]: [c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[c-LoD] stream: tick 800 65536/65536 cells, lap 603, cone 9983 zone 855 dirty 19 (q 11218) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 466720c/1172e, edits +10p/10c/2e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 316 forced 605, native 19692c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 126848p/236b fail 0 ~16 kB/s cache 13936/24576 h112912 m13936 e0, srbeat 6697s/374q drop 0stale/787undec, ereb 276s/19q, sweep 0r/31d
[10:57:47 INFO]: [c-LoD] stream: tick 800 65536/65536 cells, lap 603, cone 9983 zone 855 dirty 19 (q 11218) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 466720c/1172e, edits +10p/10c/2e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 316 forced 605, native 19692c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 126848p/236b fail 0 ~16 kB/s cache 13936/24576 h112912 m13936 e0, srbeat 6697s/374q drop 0stale/787undec, ereb 276s/19q, sweep 0r/31d
```
### sessions
```
[10:56:38 INFO]: System chat: Stepan joined the game
[10:57:54 INFO]: System chat: Stepan left the game
```
### client log tail
```
[10:57:52] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[10:57:52] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[10:57:52] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[10:57:52] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[10:57:52] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:move_player_pos] net.minecraft.network.protocol.game.ServerboundMovePlayerPacket$Pos -> 26 bytes
[10:57:52] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[10:57:52] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[10:57:52] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[10:57:52] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[10:57:52] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[10:57:52] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[10:57:52] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[10:57:52] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[10:57:52] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[10:57:52] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[10:57:52] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[10:57:52] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[10:57:52] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[10:57:52] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[10:57:52] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[10:57:52] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[10:57:52] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:keep_alive] net.minecraft.network.protocol.common.ClientboundKeepAlivePacket -> 9 bytes
[10:57:52] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:keep_alive] net.minecraft.network.protocol.common.ServerboundKeepAlivePacket -> 9 bytes
[10:57:52] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:set_time] net.minecraft.network.protocol.game.ClientboundSetTimePacket -> 10 bytes
[10:57:52] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:bundle_delimiter] net.minecraft.network.protocol.game.ClientboundBundleDelimiterPacket -> 1 bytes
[10:57:52] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[10:57:52] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[10:57:52] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[10:57:52] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[10:57:52] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[10:57:52] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[10:57:52] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[10:57:52] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[10:57:52] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:bundle_delimiter] net.minecraft.network.protocol.game.ClientboundBundleDelimiterPacket -> 1 bytes
[10:57:53] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[10:57:53] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[10:57:53] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[10:57:53] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[10:57:53] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[10:57:53] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[10:57:53] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[10:57:53] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[10:57:53] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[10:57:53] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[10:57:53] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[10:57:53] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[10:57:53] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[10:57:53] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[10:57:53] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[10:57:53] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[10:57:53] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[10:57:53] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[10:57:53] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[10:57:53] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[10:57:53] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:keep_alive] net.minecraft.network.protocol.common.ClientboundKeepAlivePacket -> 9 bytes
[10:57:53] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:keep_alive] net.minecraft.network.protocol.common.ServerboundKeepAlivePacket -> 9 bytes
[10:57:53] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:move_player_pos] net.minecraft.network.protocol.game.ServerboundMovePlayerPacket$Pos -> 26 bytes
[10:57:53] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[10:57:53] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[10:57:53] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[10:57:53] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[10:57:53] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[10:57:53] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[10:57:53] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[10:57:53] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[10:57:53] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[10:57:53] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[10:57:53] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:set_time] net.minecraft.network.protocol.game.ClientboundSetTimePacket -> 10 bytes
[10:57:53] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:block_update] net.minecraft.network.protocol.game.ClientboundBlockUpdatePacket -> 10 bytes
[10:57:53] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[10:57:54] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:bundle_delimiter] net.minecraft.network.protocol.game.ClientboundBundleDelimiterPacket -> 1 bytes
[10:57:54] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[10:57:54] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[10:57:54] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[10:57:54] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[10:57:54] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[10:57:54] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[10:57:54] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[10:57:54] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[10:57:54] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:bundle_delimiter] net.minecraft.network.protocol.game.ClientboundBundleDelimiterPacket -> 1 bytes
--- exceptions (Realms/RTKit noise filtered) ---
25:[10:56:33] [Render thread/ERROR]: Error parsing option value 4 for option Simulation Distance: Value 4 outside of range [5:33]
48:[10:56:33] [Download-2/ERROR]: Failed to fetch user properties
49:com.mojang.authlib.exceptions.InvalidCredentialsException: Status: 401
50:	at com.mojang.authlib.exceptions.MinecraftClientHttpException.toAuthenticationException(MinecraftClientHttpException.java:70)
57:Caused by: MinecraftClientHttpException[type=HTTP_ERROR, status=401, response=ErrorResponse[path=/player/attributes, error=null, errorMessage=null, details=null]]
58:	at com.mojang.authlib.minecraft.client.MinecraftClient.buildHttpException(MinecraftClient.java:170)
```
### server log tail
```
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[10:57:55 INFO]: [ChunkHolderManager] Saved 1000 block chunks, 976 entity chunks, 0 poi chunks in world 'minecraft:overworld' in 0.79s
[10:57:55 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:overworld'
[10:57:55 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:overworld'
[10:57:55 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[10:57:55 INFO]: [ChunkHolderManager] Waiting 60s for chunk system to halt for world 'minecraft:the_nether'
[10:57:55 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:the_nether'
[10:57:55 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_nether'
[10:57:55 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_nether' in 0.00s
[10:57:55 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_nether'
[10:57:55 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_nether'
[10:57:55 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[10:57:55 INFO]: [ChunkHolderManager] Waiting 60s for chunk system to halt for world 'minecraft:the_end'
[10:57:55 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:the_end'
[10:57:55 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_end'
[10:57:55 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_end' in 0.00s
[10:57:55 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_end'
[10:57:55 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_end'
[10:57:55 INFO]: Waiting for all RegionFile I/O tasks to complete...
[10:57:55 INFO]: All RegionFile I/O tasks to complete
[10:57:55 INFO]: [MoonriseCommon] Awaiting termination of worker pool for up to 60s...
[10:57:55 INFO]: [MoonriseCommon] Awaiting termination of I/O pool for up to 60s...
[c-LoD] EB: enter, native_gen=true
[launcher] server exited with code 143
```
### shots
```
-rw-r--r-- 1 runner runner  11333 Nov 17 10:20 lens_12_after_edit_probe29.png
-rw-r--r-- 1 runner runner  11877 Nov 17 10:16 lens_12_after_edit_probe3.png
-rw-r--r-- 1 runner runner  11382 Nov 17 10:20 lens_12_after_edit_probe30.png
-rw-r--r-- 1 runner runner  11248 Nov 17 10:20 lens_12_after_edit_probe31.png
-rw-r--r-- 1 runner runner  11088 Nov 17 10:21 lens_12_after_edit_probe32.png
-rw-r--r-- 1 runner runner  11309 Nov 17 10:21 lens_12_after_edit_probe33.png
-rw-r--r-- 1 runner runner  10590 Nov 17 10:21 lens_12_after_edit_probe34.png
-rw-r--r-- 1 runner runner  10962 Nov 17 10:21 lens_12_after_edit_probe35.png
-rw-r--r-- 1 runner runner  11148 Nov 17 10:16 lens_12_after_edit_probe4.png
-rw-r--r-- 1 runner runner  11240 Nov 17 10:17 lens_12_after_edit_probe5.png
-rw-r--r-- 1 runner runner  11620 Nov 17 10:17 lens_12_after_edit_probe6.png
-rw-r--r-- 1 runner runner  11706 Nov 17 10:17 lens_12_after_edit_probe7.png
-rw-r--r-- 1 runner runner  11548 Nov 17 10:17 lens_12_after_edit_probe8.png
-rw-r--r-- 1 runner runner  11054 Nov 17 10:17 lens_12_after_edit_probe9.png
-rw-r--r-- 1 runner runner   2669 Nov 17 10:29 lens_13_after_relog_probe0.png
-rw-r--r-- 1 runner runner  10487 Nov 17 10:30 lens_13_after_relog_probe1.png
-rw-r--r-- 1 runner runner  10692 Nov 17 10:30 lens_13_after_relog_probe2.png
drwxr-xr-x 2 runner runner   4096 Nov 17 10:52 lens_parity
drwxr-xr-x 2 runner runner   4096 Nov 17 10:52 parity
drwxr-xr-x 2 runner runner   4096 Nov 17 10:24 parity_dbg
drwxr-xr-x 2 runner runner   4096 Nov 17 10:55 parity_dbg8
-rw-r--r-- 1 runner runner   8744 Nov 17 10:30 probe_counts.txt
drwxr-xr-x 2 runner runner   4096 Nov 17 10:57 rim_mask
-rw-r--r-- 1 runner runner  56728 Nov 17 10:57 server_full.log
-rw-r--r-- 1 runner runner  65830 Nov 17 10:57 summary.md
```

**exit=1** failures=1 leg1=0 leg2=1
