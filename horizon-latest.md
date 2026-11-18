- phase: 0 sanity (16:51:41)
### c-LoD visual run (-930,796, marker=red_concrete, vantage -1005,150 yaw 270)
- GL:     Vendor: Mesa (0xffffffff);    Device: llvmpipe (LLVM 21.1.8, 256 bits) (0xffffffff);OpenGL renderer string: llvmpipe (LLVM 21.1.8, 256 bits);OpenGL core profile version string: 4.5 (Core Profile) Mesa 26.0.8-1ubuntu0.3;
- boot + gates OK (16:52:04)
- phase: 1 server booted (16:52:04)
- sky-grid armed
- phase: 2 leg-1 client joined (16:52:15)
- joined session 1 (16:52:15)
- stream live: cells=65536
- phase: 3 scan populated (16:52:38)
- terrain range: 2 terraced mountains at (CX-4,CZ-20)/(CX-2,CZ+16), tops 185/190, grass terraces + stone caps, 66..82 m from the eye (inside the 83.5 m band, chunk edge 64 m)
- scan complete=1 (cells=cells=65536/65536)
- paint settled=1 (wire 573808p)
- baseline try 1: drift=16683 px
- baseline try 2: drift=15034 px
- baseline try 3: drift=20703 px
- baseline try 4: drift=16471 px
- baseline try 5: drift=10163 px
- baseline try 6: drift=15801 px
- **FAIL:** no stable baseline after 6 tries (drift=15801) — the A/B would lie
- lod_silhouette_cols=61 (LOD paint ON SCREEN at 75 m if > 20 — the march-drawn marker column; vanilla cannot render there, chunk edge 64 m; #255 re-measures 49, the ZONE_FAR=48 regression #256 and the control read 0)
- 1204 terrain-green px in the sky region (LOD MOUNTAINS on screen if > 5000; the range paints ColLow-green on low terraces via the fallback palette; flat-world baselines and the pack-free control read ~0)
- burst_close white px: 1432 (renderer alive if > 50)
- burst_far white px: 887 (renderer alive in the marker zone if > 20)
- cut walk 40 m: 61 white px (> 20 = rendered)
- cut walk 60 m: 27 white px (> 20 = rendered)
- cut walk 100 m: 0 white px (> 20 = rendered)
- measured far-render cut: 60 m (G4 flap instrument; the decode proof needs >= 83 m from the y=151.6 eye)
- fill issued: 5x111x5 red_concrete at -930,796 (zone forceloaded ±48)
- phase: 4 marker filled (17:01:02)

### /clod-cells AFTER FILL (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 39 0 0x8400 0x2000
123 122 -940 784 71 0 1 [97, 119, 97] 39 0 0x8700 0x2000
124 122 -938 784 68 0 1 [97, 119, 97] 38 0 0x8400 0x2000
125 122 -936 784 62 1 1 [63, 118, 228] 38 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 44 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 44 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 44 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 36 0 0x8000 0x4100
130 122 -926 784 66 0 1 [134, 183, 131] 36 0 0x8200 0x4100
131 122 -924 784 75 0 1 [97, 119, 97] 111 0 0x8b00 0x2000
132 122 -922 784 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 122 -920 784 75 0 1 [97, 119, 97] 18446744073709551615 1 0x8b00 0x2000
134 122 -918 784 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x2000
122 123 -942 786 70 0 1 [97, 119, 97] 39 0 0x8600 0x2000
123 123 -940 786 65 0 1 [97, 119, 97] 39 0 0x8100 0x2000
124 123 -938 786 65 0 1 [97, 119, 97] 38 0 0x8100 0x2000
125 123 -936 786 62 1 1 [63, 118, 228] 38 0 0x7e00 0xd800
126 123 -934 786 62 1 1 [63, 118, 228] 44 0 0x7e00 0xd800
127 123 -932 786 62 1 1 [63, 118, 228] 44 0 0x7e00 0xd800
128 123 -930 786 62 1 1 [63, 118, 228] 44 0 0x7e00 0xd800
129 123 -928 786 73 0 1 [97, 119, 97] 37 0 0x8900 0x2000
130 123 -926 786 67 0 1 [134, 183, 131] 36 0 0x8300 0x4100
131 123 -924 786 69 0 1 [134, 183, 131] 111 0 0x8500 0x4100
132 123 -922 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 123 -920 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 123 -918 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
122 124 -942 788 72 0 1 [97, 119, 97] 39 0 0x8800 0x2000
123 124 -940 788 70 0 1 [97, 119, 97] 39 0 0x8600 0x2000
124 124 -938 788 68 0 1 [97, 119, 97] 38 0 0x8400 0x2000
125 124 -936 788 62 1 1 [63, 118, 228] 38 0 0x7e00 0xd800
126 124 -934 788 62 1 1 [63, 118, 228] 44 0 0x7e00 0xd800
127 124 -932 788 62 1 1 [63, 118, 228] 44 0 0x7e00 0xd800
128 124 -930 788 73 0 1 [97, 119, 97] 37 0 0x8900 0x2000
129 124 -928 788 76 0 1 [97, 119, 97] 37 0 0x8c00 0x2000
130 124 -926 788 73 0 1 [97, 119, 97] 36 0 0x8900 0x2000
131 124 -924 788 69 0 1 [134, 183, 131] 111 0 0x8500 0x4100
132 124 -922 788 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 124 -920 788 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 124 -918 788 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x2000
122 125 -942 790 74 0 1 [97, 119, 97] 39 0 0x8a00 0x2000
123 125 -940 790 73 0 1 [97, 119, 97] 39 0 0x8900 0x2000
124 125 -938 790 71 0 1 [97, 119, 97] 38 0 0x8700 0x2000
125 125 -936 790 62 1 1 [63, 118, 228] 38 0 0x7e00 0xd800
126 125 -934 790 62 1 1 [63, 118, 228] 44 0 0x7e00 0xd800
127 125 -932 790 62 1 1 [63, 118, 228] 44 0 0x7e00 0xd800
128 125 -930 790 62 1 1 [63, 118, 228] 44 0 0x7e00 0xd800
129 125 -928 790 73 0 1 [97, 119, 97] 37 0 0x8900 0x2000
130 125 -926 790 68 0 1 [134, 183, 131] 36 0 0x8400 0x4100
131 125 -924 790 69 0 1 [134, 183, 131] 111 0 0x8500 0x4100
132 125 -922 790 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
133 125 -920 790 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 125 -918 790 70 0 1 [134, 183, 131] 18446744073709551615 1 0x8600 0x4100
122 126 -942 792 74 0 1 [97, 119, 97] 39 0 0x8a00 0x2000
123 126 -940 792 73 0 1 [97, 119, 97] 39 0 0x8900 0x2000
124 126 -938 792 68 0 1 [97, 119, 97] 38 0 0x8400 0x2000
125 126 -936 792 62 1 1 [63, 118, 228] 38 0 0x7e00 0xd800
126 126 -934 792 62 1 1 [63, 118, 228] 44 0 0x7e00 0xd800
127 126 -932 792 62 1 1 [63, 118, 228] 44 0 0x7e00 0xd800
128 126 -930 792 62 1 1 [63, 118, 228] 44 0 0x7e00 0xd800
129 126 -928 792 67 0 1 [134, 183, 131] 37 0 0x8300 0x4100
130 126 -926 792 74 0 1 [97, 119, 97] 36 0 0x8a00 0x2000
131 126 -924 792 77 0 1 [97, 119, 97] 111 0 0x8d00 0x2000
132 126 -922 792 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 126 -920 792 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3200
134 126 -918 792 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x5000
122 127 -942 794 67 0 1 [134, 183, 131] 39 0 0x8300 0x4100
123 127 -940 794 68 0 1 [134, 183, 131] 39 0 0x8400 0x4100
124 127 -938 794 65 0 1 [134, 183, 131] 38 0 0x8100 0x4100
125 127 -936 794 62 1 1 [63, 118, 228] 38 0 0x7e00 0xd800
126 127 -934 794 62 1 1 [63, 118, 228] 44 0 0x7e00 0xd800
127 127 -932 794 62 1 1 [63, 118, 228] 44 0 0x7e00 0xd800
128 127 -930 794 62 1 1 [63, 118, 228] 44 0 0x7e00 0xd800
129 127 -928 794 66 0 1 [134, 183, 131] 37 0 0x8200 0x4100
130 127 -926 794 74 0 1 [97, 119, 97] 36 0 0x8a00 0x2000
131 127 -924 794 77 0 1 [97, 119, 97] 111 0 0x8d00 0x2000
132 127 -922 794 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 127 -920 794 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3200
134 127 -918 794 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x5000
122 128 -942 796 68 0 1 [134, 183, 131] 0 0 0x8400 0x4100
123 128 -940 796 72 0 1 [97, 119, 97] 39 0 0x8800 0x2000
124 128 -938 796 75 0 1 [97, 119, 97] 38 0 0x8b00 0x2000
125 128 -936 796 72 0 1 [97, 119, 97] 38 0 0x8800 0x2000
126 128 -934 796 62 1 1 [63, 118, 228] 44 0 0x7e00 0xd800
127 128 -932 796 62 1 1 [63, 118, 228] 44 0 0x7e00 0xd800
128 128 -930 796 62 1 1 [63, 118, 228] 44 0 0x7e00 0xd800
129 128 -928 796 66 0 1 [134, 183, 131] 37 0 0x8200 0x4100
130 128 -926 796 68 0 1 [134, 183, 131] 36 0 0x8400 0x4100
131 128 -924 796 74 0 1 [97, 119, 97] 111 0 0x8a00 0x2000
132 128 -922 796 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 128 -920 796 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
134 128 -918 796 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 129 -942 798 69 0 1 [134, 183, 131] 39 0 0x8500 0x4100
123 129 -940 798 75 0 1 [97, 119, 97] 39 0 0x8b00 0x2000
124 129 -938 798 78 0 1 [97, 119, 97] 0 0 0x8e00 0x2000
125 129 -936 798 75 0 1 [97, 119, 97] 38 0 0x8b00 0x2000
126 129 -934 798 62 1 1 [63, 118, 228] 44 0 0x7e00 0xd800
127 129 -932 798 62 1 1 [63, 118, 228] 44 0 0x7e00 0xd800
128 129 -930 798 62 1 1 [63, 118, 228] 44 0 0x7e00 0xd800
129 129 -928 798 71 0 1 [97, 119, 97] 37 0 0x8700 0x2000
130 129 -926 798 71 0 1 [97, 119, 97] 36 0 0x8700 0x2000
131 129 -924 798 69 0 1 [134, 183, 131] 111 0 0x8500 0x4100
132 129 -922 798 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 129 -920 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 129 -918 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 130 -942 800 70 0 1 [134, 183, 131] 39 0 0x8600 0x4100
123 130 -940 800 76 0 1 [97, 119, 97] 39 0 0x8c00 0x2000
124 130 -938 800 76 0 1 [97, 119, 97] 38 0 0x8c00 0x2000
125 130 -936 800 72 0 1 [97, 119, 97] 38 0 0x8800 0x2000
126 130 -934 800 62 1 1 [63, 118, 228] 44 0 0x7e00 0xd800
127 130 -932 800 62 1 1 [63, 118, 228] 44 0 0x7e00 0xd800
128 130 -930 800 62 1 1 [63, 118, 228] 44 0 0x7e00 0xd800
129 130 -928 800 71 0 1 [97, 119, 97] 37 0 0x8700 0x2000
130 130 -926 800 71 0 1 [97, 119, 97] 36 0 0x8700 0x2000
131 130 -924 800 69 0 1 [134, 183, 131] 110 0 0x8500 0x4100
132 130 -922 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
133 130 -920 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 130 -918 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 131 -942 802 78 0 1 [97, 119, 97] 1 0 0x8e00 0x2000
123 131 -940 802 77 0 1 [97, 119, 97] 39 0 0x8d00 0x2000
124 131 -938 802 77 0 1 [97, 119, 97] 38 0 0x8d00 0x2000
125 131 -936 802 66 0 1 [134, 183, 131] 38 0 0x8200 0x4100
126 131 -934 802 62 1 1 [63, 118, 228] 44 0 0x7e00 0xd800
127 131 -932 802 62 1 1 [63, 118, 228] 44 0 0x7e00 0xd800
128 131 -930 802 62 1 1 [63, 118, 228] 44 0 0x7e00 0xd800
129 131 -928 802 62 1 1 [63, 118, 228] 37 0 0x7e00 0xd800
130 131 -926 802 66 0 1 [134, 183, 131] 36 0 0x8200 0x4100
131 131 -924 802 68 0 1 [134, 183, 131] 110 0 0x8400 0x4100
132 131 -922 802 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 131 -920 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 131 -918 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 132 -942 804 79 0 1 [97, 119, 97] 1 0 0x8f00 0x2000
123 132 -940 804 76 0 1 [97, 119, 97] 39 0 0x8c00 0x2000
124 132 -938 804 76 0 1 [97, 119, 97] 38 0 0x8c00 0x2000
125 132 -936 804 76 0 1 [97, 119, 97] 38 0 0x8c00 0x2000
126 132 -934 804 62 1 1 [63, 118, 228] 44 0 0x7e00 0xd800
127 132 -932 804 62 1 1 [63, 118, 228] 44 0 0x7e00 0xd800
128 132 -930 804 62 1 1 [63, 118, 228] 44 0 0x7e00 0xd800
129 132 -928 804 62 1 1 [63, 118, 228] 37 0 0x7e00 0xd800
130 132 -926 804 65 0 1 [134, 183, 131] 36 0 0x8100 0x4100
131 132 -924 804 77 0 1 [97, 119, 97] 110 0 0x8d00 0x2000
132 132 -922 804 79 0 1 [97, 119, 97] 36 0 0x8f00 0x2000
133 132 -920 804 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 132 -918 804 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x2000
122 133 -942 806 78 0 1 [97, 119, 97] 1 0 0x8e00 0x2000
123 133 -940 806 69 0 1 [134, 183, 131] 39 0 0x8500 0x4100
124 133 -938 806 155 0 1 [134, 183, 131] 39 0 0xdb00 0x4100
125 133 -936 806 155 0 1 [134, 183, 131] 39 0 0xdb00 0x4100
126 133 -934 806 155 0 1 [134, 183, 131] 38 0 0xdb00 0x4100
127 133 -932 806 155 0 1 [134, 183, 131] 38 0 0xdb00 0x4100
128 133 -930 806 155 0 1 [134, 183, 131] 37 0 0xdb00 0x4100
129 133 -928 806 155 0 1 [134, 183, 131] 37 0 0xdb00 0x4100
130 133 -926 806 155 0 1 [134, 183, 131] 37 0 0xdb00 0x4100
131 133 -924 806 67 0 1 [134, 183, 131] 110 0 0x8300 0x4100
132 133 -922 806 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 133 -920 806 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 133 -918 806 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x2000
122 134 -942 808 79 0 1 [97, 119, 97] 1 0 0x8f00 0x2000
123 134 -940 808 68 0 1 [134, 183, 131] 39 0 0x8400 0x4100
124 134 -938 808 155 0 1 [134, 183, 131] 39 0 0xdb00 0x4100
125 134 -936 808 180 0 1 [134, 183, 131] 37 0 0xf400 0x4100
126 134 -934 808 180 0 1 [134, 183, 131] 37 0 0xf400 0x4100
127 134 -932 808 180 0 1 [134, 183, 131] 36 0 0xf400 0x4100
128 134 -930 808 180 0 1 [134, 183, 131] 36 0 0xf400 0x4100
129 134 -928 808 180 0 1 [134, 183, 131] 110 0 0xf400 0x4100
130 134 -926 808 155 0 1 [134, 183, 131] 37 0 0xdb00 0x4100
131 134 -924 808 67 0 1 [134, 183, 131] 110 0 0x8300 0x4100
132 134 -922 808 68 0 1 [134, 183, 131] 18446744073709551615 1 0x8400 0x4100
133 134 -920 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 134 -918 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
```
### stream diag after fill:
[c-LoD] stream: tick 5400 65536/65536 cells, lap 834, cone 33020 zone 2543 dirty 932 (q 11387) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1419978c/2748e, edits +6003p/4555c/713e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 345 forced 167, native 65536c/1091ch fail 0 (3w q 0) helper on, trees 1091ch/fail 0, wire bundle 807931p/2351b fail 0 ~67 kB/s cache 13595/24576 h794336 m13595 e0, srbeat 64898s/576q drop 57stale/101undec, ereb 48718s/10q, sweep 0r/74d
[17:01:06 INFO]: [c-LoD] stream: tick 5400 65536/65536 cells, lap 834, cone 33020 zone 2543 dirty 932 (q 11387) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1419978c/2748e, edits +6003p/4555c/713e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 345 forced 167, native 65536c/1091ch fail 0 (3w q 0) helper on, trees 1091ch/fail 0, wire bundle 807931p/2351b fail 0 ~67 kB/s cache 13595/24576 h794336 m13595 e0, srbeat 64898s/576q drop 57stale/101undec, ereb 48718s/10q, sweep 0r/74d
- carrier ladder y=155: 13912 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=165: 18012 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=168: 15720 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=172: 14344 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=176: 16100 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=180: 14764 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=185: 14308 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- probe 0: red=16053 paint=114404 lod_silhouette_cols=91 lens_red=0 lens_black=0 lens_water=0 lens_dark=0 lens_zero=0 rot=none scan=0 carriers=0 (need red>=60 paint>=300 lod>20)
- **LEG 1 PASS** (zero relogs)
- phase: 5 leg-1 probes done (17:01:35)
### stream diag after probes:
[17:01:16 INFO]: [c-LoD] stream: tick 5500 65536/65536 cells, lap 843, cone 34170 zone 2543 dirty 932 (q 11405) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1449822c/2766e, edits +6251p/4743c/800e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 345 forced 167, native 65536c/1091ch fail 0 (3w q 0) helper on, trees 1091ch/fail 0, wire bundle 822306p/2398b fail 0 ~75 kB/s cache 13595/24576 h808711 m13595 e0, srbeat 65557s/578q drop 57stale/101undec, ereb 48801s/10q, sweep 0r/83d
[c-LoD] stream: tick 5600 65536/65536 cells, lap 852, cone 35314 zone 2543 dirty 932 (q 11411) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1484525c/2777e, edits +6388p/4843c/827e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 345 forced 167, native 65536c/1091ch fail 0 (3w q 0) helper on, trees 1091ch/fail 0, wire bundle 836633p/2444b fail 0 ~75 kB/s cache 13595/24576 h823038 m13595 e0, srbeat 66216s/574q drop 57stale/101undec, ereb 48882s/10q, sweep 0r/92d
[17:01:26 INFO]: [c-LoD] stream: tick 5600 65536/65536 cells, lap 852, cone 35314 zone 2543 dirty 932 (q 11411) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1484525c/2777e, edits +6388p/4843c/827e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 345 forced 167, native 65536c/1091ch fail 0 (3w q 0) helper on, trees 1091ch/fail 0, wire bundle 836633p/2444b fail 0 ~75 kB/s cache 13595/24576 h823038 m13595 e0, srbeat 66216s/574q drop 57stale/101undec, ereb 48882s/10q, sweep 0r/92d
- parity debug tour: 13 vantages (session-1 client, DEBUG pack — evidence only, the gate pairs the §7b world session)
- parity debug tour done: 13 shots in parity_dbg/ (the §7b world session feeds the gate)
- joined session 2 (17:04:15)
- leg-2 baseline scene px (x<940): 24860 (0 = the relog client paints no world at all)
- leg-2 renderer probe NEAR (20 m) white px: 643 raw / 2687 changed-vs-baseline (> 20 = the client renders particles)
- leg-2 renderer probe FAR (80 m, marker zone) white px: 54 raw / 4466 changed-vs-baseline (> 20 = no distance cut; ~0 with NEAR alive = particles beyond the client chunk radius do not render)
- leg-2 carrier probe green dust (80 m): changed-green px 0 / total changed 29158 vs baseline (> 100 green = a stable green carrier exists; total > 0 with green ~0 = the dust renders but fog washes it — a ruler problem, not a carrier problem)
- phase: 8 leg-2 relog probes (17:09:39)
- relog probe 0: red=15944 scene=27268 lens_red=0 lens_black=3 lens_water=0 lens_dark=0 lens_zero=0 rot=decoded cam_x_b=? cam_z_b=? az_b=? el_b=? fog=live fog_end_b=? strobe_hits=22 hits_hi=22 mask_hits=22 (need red >= 60; scene > 0 = the client paints a world at all)
- **LEG 2 PASS** (marker persisted over relog)
- phase: 9 leg-2 done (17:09:42)

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
122 122 -942 784 68 0 1 [97, 119, 97] 3467 0 0x8400 0x2000
123 122 -940 784 71 0 1 [97, 119, 97] 3467 0 0x8700 0x2000
124 122 -938 784 68 0 1 [97, 119, 97] 3467 0 0x8400 0x2000
125 122 -936 784 62 1 1 [63, 118, 228] 3467 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 3466 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 3466 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 3466 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 3466 0 0x8000 0x4100
130 122 -926 784 66 0 1 [134, 183, 131] 3466 0 0x8200 0x4100
131 122 -924 784 75 0 1 [97, 119, 97] 3466 0 0x8b00 0x2000
132 122 -922 784 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 122 -920 784 75 0 1 [97, 119, 97] 18446744073709551615 1 0x8b00 0x2000
134 122 -918 784 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x2000
122 123 -942 786 70 0 1 [97, 119, 97] 3466 0 0x8600 0x2000
123 123 -940 786 65 0 1 [97, 119, 97] 3466 0 0x8100 0x2000
124 123 -938 786 65 0 1 [97, 119, 97] 3466 0 0x8100 0x2000
125 123 -936 786 62 1 1 [63, 118, 228] 3466 0 0x7e00 0xd800
126 123 -934 786 62 1 1 [63, 118, 228] 3466 0 0x7e00 0xd800
127 123 -932 786 62 1 1 [63, 118, 228] 3466 0 0x7e00 0xd800
128 123 -930 786 62 1 1 [63, 118, 228] 3466 0 0x7e00 0xd800
129 123 -928 786 73 0 1 [97, 119, 97] 3466 0 0x8900 0x2000
130 123 -926 786 67 0 1 [134, 183, 131] 3466 0 0x8300 0x4100
131 123 -924 786 69 0 1 [134, 183, 131] 3466 0 0x8500 0x4100
132 123 -922 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 123 -920 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 123 -918 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
122 124 -942 788 72 0 1 [97, 119, 97] 3466 0 0x8800 0x2000
123 124 -940 788 70 0 1 [97, 119, 97] 3466 0 0x8600 0x2000
124 124 -938 788 68 0 1 [97, 119, 97] 3466 0 0x8400 0x2000
125 124 -936 788 62 1 1 [63, 118, 228] 3466 0 0x7e00 0xd800
126 124 -934 788 62 1 1 [63, 118, 228] 3466 0 0x7e00 0xd800
127 124 -932 788 62 1 1 [63, 118, 228] 3466 0 0x7e00 0xd800
128 124 -930 788 73 0 1 [97, 119, 97] 3466 0 0x8900 0x2000
129 124 -928 788 76 0 1 [97, 119, 97] 3466 0 0x8c00 0x2000
130 124 -926 788 73 0 1 [97, 119, 97] 3466 0 0x8900 0x2000
131 124 -924 788 69 0 1 [134, 183, 131] 3466 0 0x8500 0x4100
132 124 -922 788 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 124 -920 788 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 124 -918 788 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x2000
122 125 -942 790 74 0 1 [97, 119, 97] 3466 0 0x8a00 0x2000
123 125 -940 790 73 0 1 [97, 119, 97] 3466 0 0x8900 0x2000
124 125 -938 790 71 0 1 [97, 119, 97] 3466 0 0x8700 0x2000
125 125 -936 790 62 1 1 [63, 118, 228] 3466 0 0x7e00 0xd800
126 125 -934 790 62 1 1 [63, 118, 228] 3466 0 0x7e00 0xd800
127 125 -932 790 62 1 1 [63, 118, 228] 3466 0 0x7e00 0xd800
128 125 -930 790 62 1 1 [63, 118, 228] 3466 0 0x7e00 0xd800
129 125 -928 790 73 0 1 [97, 119, 97] 3466 0 0x8900 0x2000
130 125 -926 790 68 0 1 [134, 183, 131] 3466 0 0x8400 0x4100
131 125 -924 790 69 0 1 [134, 183, 131] 3466 0 0x8500 0x4100
132 125 -922 790 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
133 125 -920 790 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 125 -918 790 70 0 1 [134, 183, 131] 18446744073709551615 1 0x8600 0x4100
122 126 -942 792 74 0 1 [97, 119, 97] 3466 0 0x8a00 0x2000
123 126 -940 792 73 0 1 [97, 119, 97] 3466 0 0x8900 0x2000
124 126 -938 792 68 0 1 [97, 119, 97] 3466 0 0x8400 0x2000
125 126 -936 792 62 1 1 [63, 118, 228] 3466 0 0x7e00 0xd800
126 126 -934 792 62 1 1 [63, 118, 228] 3466 0 0x7e00 0xd800
127 126 -932 792 62 1 1 [63, 118, 228] 3466 0 0x7e00 0xd800
128 126 -930 792 62 1 1 [63, 118, 228] 3466 0 0x7e00 0xd800
129 126 -928 792 67 0 1 [134, 183, 131] 3466 0 0x8300 0x4100
130 126 -926 792 74 0 1 [97, 119, 97] 3466 0 0x8a00 0x2000
131 126 -924 792 77 0 1 [97, 119, 97] 3466 0 0x8d00 0x2000
132 126 -922 792 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 126 -920 792 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3200
134 126 -918 792 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x5000
122 127 -942 794 67 0 1 [134, 183, 131] 3466 0 0x8300 0x4100
123 127 -940 794 68 0 1 [134, 183, 131] 3466 0 0x8400 0x4100
124 127 -938 794 65 0 1 [134, 183, 131] 3466 0 0x8100 0x4100
125 127 -936 794 62 1 1 [63, 118, 228] 3466 0 0x7e00 0xd800
126 127 -934 794 62 1 1 [63, 118, 228] 3466 0 0x7e00 0xd800
127 127 -932 794 170 0 1 [142, 33, 33] 3450 0 0xea00 0x0a00
128 127 -930 794 170 0 1 [142, 33, 33] 3450 0 0xea00 0x0a00
129 127 -928 794 170 0 1 [142, 33, 33] 3443 0 0xea00 0x0a00
130 127 -926 794 74 0 1 [97, 119, 97] 3466 0 0x8a00 0x2000
131 127 -924 794 77 0 1 [97, 119, 97] 3466 0 0x8d00 0x2000
132 127 -922 794 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 127 -920 794 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3200
134 127 -918 794 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x5000
122 128 -942 796 68 0 1 [134, 183, 131] 3466 0 0x8400 0x4100
123 128 -940 796 72 0 1 [97, 119, 97] 3466 0 0x8800 0x2000
124 128 -938 796 75 0 1 [97, 119, 97] 3466 0 0x8b00 0x2000
125 128 -936 796 72 0 1 [97, 119, 97] 3466 0 0x8800 0x2000
126 128 -934 796 62 1 1 [63, 118, 228] 3466 0 0x7e00 0xd800
127 128 -932 796 170 0 1 [142, 33, 33] 3450 0 0xea00 0x0a00
128 128 -930 796 170 0 1 [142, 33, 33] 3450 0 0xea00 0x0a00
129 128 -928 796 170 0 1 [142, 33, 33] 3443 0 0xea00 0x0a00
130 128 -926 796 68 0 1 [134, 183, 131] 3466 0 0x8400 0x4100
131 128 -924 796 74 0 1 [97, 119, 97] 3466 0 0x8a00 0x2000
132 128 -922 796 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 128 -920 796 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
134 128 -918 796 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 129 -942 798 69 0 1 [134, 183, 131] 3466 0 0x8500 0x4100
123 129 -940 798 75 0 1 [97, 119, 97] 3466 0 0x8b00 0x2000
124 129 -938 798 78 0 1 [97, 119, 97] 3466 0 0x8e00 0x2000
125 129 -936 798 75 0 1 [97, 119, 97] 3466 0 0x8b00 0x2000
126 129 -934 798 62 1 1 [63, 118, 228] 3466 0 0x7e00 0xd800
127 129 -932 798 170 0 1 [142, 33, 33] 3450 0 0xea00 0x0a00
128 129 -930 798 170 0 1 [142, 33, 33] 3450 0 0xea00 0x0a00
129 129 -928 798 170 0 1 [142, 33, 33] 3450 0 0xea00 0x0a00
130 129 -926 798 71 0 1 [97, 119, 97] 3466 0 0x8700 0x2000
131 129 -924 798 69 0 1 [134, 183, 131] 3466 0 0x8500 0x4100
132 129 -922 798 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 129 -920 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 129 -918 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 130 -942 800 70 0 1 [134, 183, 131] 3466 0 0x8600 0x4100
123 130 -940 800 76 0 1 [97, 119, 97] 3466 0 0x8c00 0x2000
124 130 -938 800 76 0 1 [97, 119, 97] 3466 0 0x8c00 0x2000
125 130 -936 800 72 0 1 [97, 119, 97] 3466 0 0x8800 0x2000
126 130 -934 800 62 1 1 [63, 118, 228] 3466 0 0x7e00 0xd800
127 130 -932 800 62 1 1 [63, 118, 228] 3466 0 0x7e00 0xd800
128 130 -930 800 62 1 1 [63, 118, 228] 3466 0 0x7e00 0xd800
129 130 -928 800 71 0 1 [97, 119, 97] 3466 0 0x8700 0x2000
130 130 -926 800 71 0 1 [97, 119, 97] 3466 0 0x8700 0x2000
131 130 -924 800 69 0 1 [134, 183, 131] 3466 0 0x8500 0x4100
132 130 -922 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
133 130 -920 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 130 -918 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 131 -942 802 78 0 1 [97, 119, 97] 3467 0 0x8e00 0x2000
123 131 -940 802 77 0 1 [97, 119, 97] 3466 0 0x8d00 0x2000
124 131 -938 802 77 0 1 [97, 119, 97] 3466 0 0x8d00 0x2000
125 131 -936 802 66 0 1 [134, 183, 131] 3466 0 0x8200 0x4100
126 131 -934 802 62 1 1 [63, 118, 228] 3466 0 0x7e00 0xd800
127 131 -932 802 62 1 1 [63, 118, 228] 3466 0 0x7e00 0xd800
128 131 -930 802 62 1 1 [63, 118, 228] 3466 0 0x7e00 0xd800
129 131 -928 802 62 1 1 [63, 118, 228] 3466 0 0x7e00 0xd800
130 131 -926 802 66 0 1 [134, 183, 131] 3466 0 0x8200 0x4100
131 131 -924 802 68 0 1 [134, 183, 131] 3466 0 0x8400 0x4100
132 131 -922 802 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 131 -920 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 131 -918 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 132 -942 804 79 0 1 [97, 119, 97] 3467 0 0x8f00 0x2000
123 132 -940 804 76 0 1 [97, 119, 97] 3466 0 0x8c00 0x2000
124 132 -938 804 76 0 1 [97, 119, 97] 3466 0 0x8c00 0x2000
125 132 -936 804 76 0 1 [97, 119, 97] 3466 0 0x8c00 0x2000
126 132 -934 804 62 1 1 [63, 118, 228] 3466 0 0x7e00 0xd800
127 132 -932 804 62 1 1 [63, 118, 228] 3466 0 0x7e00 0xd800
128 132 -930 804 62 1 1 [63, 118, 228] 3466 0 0x7e00 0xd800
129 132 -928 804 62 1 1 [63, 118, 228] 3466 0 0x7e00 0xd800
130 132 -926 804 65 0 1 [134, 183, 131] 3466 0 0x8100 0x4100
131 132 -924 804 77 0 1 [97, 119, 97] 3466 0 0x8d00 0x2000
132 132 -922 804 79 0 1 [97, 119, 97] 3466 0 0x8f00 0x2000
133 132 -920 804 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 132 -918 804 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x2000
122 133 -942 806 78 0 1 [97, 119, 97] 3467 0 0x8e00 0x2000
123 133 -940 806 69 0 1 [134, 183, 131] 3466 0 0x8500 0x4100
124 133 -938 806 155 0 1 [134, 183, 131] 3466 0 0xdb00 0x4100
125 133 -936 806 155 0 1 [134, 183, 131] 3466 0 0xdb00 0x4100
126 133 -934 806 155 0 1 [134, 183, 131] 3466 0 0xdb00 0x4100
127 133 -932 806 155 0 1 [134, 183, 131] 3466 0 0xdb00 0x4100
128 133 -930 806 155 0 1 [134, 183, 131] 3466 0 0xdb00 0x4100
129 133 -928 806 155 0 1 [134, 183, 131] 3466 0 0xdb00 0x4100
130 133 -926 806 155 0 1 [134, 183, 131] 3466 0 0xdb00 0x4100
131 133 -924 806 67 0 1 [134, 183, 131] 3466 0 0x8300 0x4100
132 133 -922 806 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 133 -920 806 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 133 -918 806 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x2000
122 134 -942 808 79 0 1 [97, 119, 97] 3467 0 0x8f00 0x2000
123 134 -940 808 68 0 1 [134, 183, 131] 3466 0 0x8400 0x4100
124 134 -938 808 155 0 1 [134, 183, 131] 3466 0 0xdb00 0x4100
125 134 -936 808 180 0 1 [134, 183, 131] 3466 0 0xf400 0x4100
126 134 -934 808 180 0 1 [134, 183, 131] 3466 0 0xf400 0x4100
127 134 -932 808 180 0 1 [134, 183, 131] 3466 0 0xf400 0x4100
128 134 -930 808 180 0 1 [134, 183, 131] 3466 0 0xf400 0x4100
129 134 -928 808 180 0 1 [134, 183, 131] 3466 0 0xf400 0x4100
130 134 -926 808 155 0 1 [134, 183, 131] 3466 0 0xdb00 0x4100
131 134 -924 808 67 0 1 [134, 183, 131] 3466 0 0x8300 0x4100
132 134 -922 808 68 0 1 [134, 183, 131] 18446744073709551615 1 0x8400 0x4100
133 134 -920 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 134 -918 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
```
- edit channel: 0 linkage errors, 0 failed event dispatches (both must be 0)
- phase: 6 parity world boot (17:09:42)
- parity world: view-distance=8 (the voxy leg's server value)
- parity world: joined session 3 (17:10:11)
- parity world: scan populated (cells=65536; the march paints from these buffers)
- parity world tour: 13 vantages (production pack, vd 8, shared structures)
- coverage sweeps: 72 eye passes = 36 positions (12 az x r144/200 + 12 off-az x r172, y=160) x 2 looks (outward+inward), settle-by-laps >= 6 s (q stable 3x1s), deadline 12 s/pos, no shots — the v6.26.0 data-coverage lever + v6.26.6 lottery kill; probe rides parity/view_probe.md
- parity drain-settle TIMEOUT: queue 18972 -> 18904 > 11500 after 150s — continuing anyway; the queue did not drain without eye proximity (priority-lane evidence, the 01/05/10 dip attribution reads from the tour deltas)
- pre-tour pass done: 13 vantages x 10s (the tour now reads the second OR-merge lap; the rim/dip transport race reads from the tour deltas)
- **DAY GUARD:** 02_aerial_south still night after re-pin+reshoot (zenith < 130) — renamed night_02_aerial_south.png; the row pairs as missing, NOT as a renderer diff
- **DAY GUARD:** 03_aerial_north still night after re-pin+reshoot (zenith < 130) — renamed night_03_aerial_north.png; the row pairs as missing, NOT as a renderer diff
- **DAY GUARD:** 06_ground_west still night after re-pin+reshoot (zenith < 130) — renamed night_06_ground_west.png; the row pairs as missing, NOT as a renderer diff
- **DAY GUARD:** 08_back_inside still night after re-pin+reshoot (zenith < 130) — renamed night_08_back_inside.png; the row pairs as missing, NOT as a renderer diff
- **DAY GUARD:** 09_after_mesh still night after re-pin+reshoot (zenith < 130) — renamed night_09_after_mesh.png; the row pairs as missing, NOT as a renderer diff
- parity world tour done: 13 shots, 5 night-guard renames in parity/ (the gate pairs voxy/<name>.png vs parity/<name>.png)
- phase: 7 parity tour done (17:33:14)
- sparse re-beat channel at tour end: streamed=135297 q=0 dropped_stale=114 dropped_undecodable=12487 (frac=8 boot; undecodable-dominant = the rim words die in their decodability window, stale-dominant = edit churn owns the cells)
- **FAIL:** parity world time-of-day uncontrollable: 5 vantage(s) night after re-pin+reshoot — the §7b noon-pin class is broken on this runner; the matrix is infrastructure-blocked, not a renderer verdict (DAY GUARD lines + the feed's time-audit.md carry the evidence)
### parity world stream counters (session 3, tail + maxima)
[c-LoD] stream: tick 13800 65536/65536 cells, lap 1334, cone 77878 zone 24223 dirty 15702 (q 18906) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 9491932c/2126e, edits +18050p/13689c/2929e drop 0, spawned 0 / failed 0, cam0 yaw -180 pitch 5, wish 0 forced 818, native 36488c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 1979085p/4664b fail 0 ~0 kB/s cache 24555/24576 h1536277 m442808 e418253, srbeat 135297s/0q drop 114stale/12487undec, ereb 75356s/0q, sweep 0r/67d
[17:33:05 INFO]: [c-LoD] stream: tick 13800 65536/65536 cells, lap 1334, cone 77878 zone 24223 dirty 15702 (q 18906) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 9491932c/2126e, edits +18050p/13689c/2929e drop 0, spawned 0 / failed 0, cam0 yaw -180 pitch 5, wish 0 forced 818, native 36488c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 1979085p/4664b fail 0 ~0 kB/s cache 24555/24576 h1536277 m442808 e418253, srbeat 135297s/0q drop 114stale/12487undec, ereb 75356s/0q, sweep 0r/67d
- parity stream max cone: 77878
- parity stream max zone: 24223
- parity stream max wire bundle: bundle
### parity world capture lens (debug chain on the §7b world)
lens_red=0 lens_black=0 lens_water=0 lens_dark=5543 lens_zero=0 rot=decoded cam_x_b=? cam_z_b=? az_b=? el_b=? fog=live fog_end_b=? strobe_hits=20 hits_hi=22 mask_hits=21
- parity mesh probe @png_y=2: x306=(74,90,125) x307=(74,90,125) x308=(74,90,125) x309h=(74,90,125) x310c=(74,90,125)
- parity mesh probe @png_y=717: x306=(22,31,25) x307=(23,32,26) x308=(41,55,44) x309h=(41,54,44) x310c=(42,56,45)
- parity mesh engagement: magenta-ish=1504 of 921600 sampled px
- mask8: the un-fog engagement masks shot on the PARITY WORLD (vd 8) — parity_dbg8/01m|07m|10m
- mask8 01m: green=30960 red=0 blue=195712 dim=369840 (px, sampled /16)
- mask8 07m: green=9520 red=0 blue=144768 dim=10720 (px, sampled /16)
- mask8 10m: green=3968 red=0 blue=153264 dim=144608 (px, sampled /16)
- mesh probe @png_y=2: x306=(95,100,109) x307=(95,100,109) x308=(95,100,109) x309h=(95,100,109) x310c=(95,100,109)
- mesh probe @png_y=717: x306=(25,35,29) x307=(26,36,30) x308=(47,62,50) x309h=(47,61,50) x310c=(47,63,50)
- mesh engagement: magenta-ish=1504 of 921600 sampled px (the vanilla-path paint coverage)
- mask8 fog state @png_y=2: raw (300)=(94,99,109) (301)=(95,100,109)
- mask8 fog state decoded: envStart=752 envEnd=792 rdEnd=1744 fogColor=(95,100,109)
- mask8 fog state @png_y=717: raw (300)=(26,38,30) (301)=(28,40,32)
- mask8 fog state decoded: envStart=208 envEnd=304 rdEnd=480 fogColor=(28,40,32)
- buffer dump 10m: marked h=2048/2048 c=2048/2048; height windows r0<142:0 r1:2048 r2>=202:0 maxByte=178
- rim mask: scan populated (cells=65536; the state paint reads the same buffers the march reads)
- phase: 10 rim mask lane (17:38:44)
- rim mask: decode-state frames shot (rim_mask/00|01|12 — GREEN full decode / RED colorless / BLUE heightless; 00 = the leg-1 tower eye, 01 = the near-tower contrast, 12 = the tower re-read; the inherited 07/12 vantages sat 760 blocks outside the streamed disk and 01 sat INSIDE the marker column — their earlier verdicts were classifier noise, purged in v6.33.1)
- rim mask 00: green=672 red=224 blue=0 yellow=0 (px, sampled /16) -> DOMINANT GREEN: rim fully decoded — the fallback is the PAINT path (shade/fog/composite), not data
- rim mask 01: green=2192 red=464 blue=1088 yellow=0 (px, sampled /16) -> DOMINANT GREEN: rim fully decoded — the fallback is the PAINT path (shade/fog/composite), not data
- rim mask 12: green=96 red=224 blue=0 yellow=0 (px, sampled /16) -> no state paint (the march owns too few pixels — check the vantage/scan)

### boot / stream
```
[c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 140203 bytes, sha1 0c7d0f287dc1acbe6e675db9e9f3a874c1264fd3 — zero textures, zero world data, one push per join
[17:36:32 INFO]: [c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 140203 bytes, sha1 0c7d0f287dc1acbe6e675db9e9f3a874c1264fd3 — zero textures, zero world data, one push per join
[c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[17:36:32 INFO]: [c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[c-LoD] stream: tick 1800 65536/65536 cells, lap 693, cone 20784 zone 1356 dirty 11 (q 1032) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1199222c/145e, edits +25p/25c/11e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 362 forced 453, native 20232c/951ch fail 0 (3w q 0) helper on, trees 951ch/fail 0, wire bundle 285994p/395b fail 0 ~35 kB/s cache 13965/24576 h272029 m13965 e0, srbeat 14745s/1314q drop 0stale/32undec, ereb 268s/11q, sweep 0r/99d
[17:39:35 INFO]: [c-LoD] stream: tick 1800 65536/65536 cells, lap 693, cone 20784 zone 1356 dirty 11 (q 1032) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1199222c/145e, edits +25p/25c/11e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 362 forced 453, native 20232c/951ch fail 0 (3w q 0) helper on, trees 951ch/fail 0, wire bundle 285994p/395b fail 0 ~35 kB/s cache 13965/24576 h272029 m13965 e0, srbeat 14745s/1314q drop 0stale/32undec, ereb 268s/11q, sweep 0r/99d
```
### sessions
```
[17:36:43 INFO]: System chat: Stepan joined the game
[17:39:39 INFO]: System chat: Stepan left the game
```
### client log tail
```
Horizon 0.1.0
Horizon renderer: Vulkan has no conformant adapter (Vulkan: only a software adapter (llvmpipe (LLVM 21.1.8, 256 bits))); asking again for the ones wgpu hides as non-conformant
Horizon renderer: trying OpenGL (Vulkan: only a software adapter (llvmpipe (LLVM 21.1.8, 256 bits)))
libEGL warning: DRI3 error: Could not get DRI3 device
libEGL warning: Ensure your X server supports DRI3 to get accelerated rendering
Horizon: ignoring minecraft:ambient_entity_effect: no such vanilla particle in this version
Horizon renderer: OpenGL has no conformant adapter (OpenGL: only a software adapter (llvmpipe (LLVM 21.1.8, 256 bits))); asking again for the ones wgpu hides as non-conformant
libEGL warning: DRI3 error: Could not get DRI3 device
libEGL warning: Ensure your X server supports DRI3 to get accelerated rendering
Horizon renderer: llvmpipe (LLVM 21.1.8, 256 bits) (Vulkan, Cpu)
Horizon renderer: requesting the device (driver llvmpipe Mesa 26.0.8-1ubuntu0.3 (LLVM 21.1.8)) with features [TEXTURE_FORMAT_16BIT_NORM | TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES | PIPELINE_STATISTICS_QUERY | TIMESTAMP_QUERY_INSIDE_ENCODERS | TIMESTAMP_QUERY_INSIDE_PASSES | MAPPABLE_PRIMARY_BUFFERS | MULTI_DRAW_INDIRECT_COUNT | VERTEX_WRITABLE_STORAGE | CLEAR_TEXTURE | TEXTURE_ATOMIC | SHADER_EARLY_DEPTH_TEST | SUBGROUP | TIMESTAMP_QUERY | INDIRECT_FIRST_INSTANCE | RG11B10UFLOAT_RENDERABLE | FLOAT32_FILTERABLE | FLOAT32_BLENDABLE] and the adapter's limits (buffers up to 2147483647 bytes, storage bindings up to 134217728 bytes, 2048 texture layers)
Horizon renderer: device ready; configuring the surface (1280x720 Bgra8Unorm, Mailbox)
Horizon renderer: the chosen graphics API gave no GPU (No Vulkan or OpenGL graphics driver found: Horizon needs the Vulkan loader (libvulkan.so.1) and a Vulkan driver for your GPU (Mesa's, or NVIDIA's), or OpenGL through EGL (Vulkan: only a software adapter (llvmpipe (LLVM 21.1.8, 256 bits)); OpenGL: only a software adapter (llvmpipe (LLVM 21.1.8, 256 bits)))); rendering on the CPU, which is slow
Horizon renderer: HDR output not offered by this display
Horizon renderer: creating the pipelines
Horizon: UI bundle ready after 367 ms
Horizon: block sprites: 1301 layers at 64 px (largest 64 px, budget 256 MiB)
load core tables: 0.79s
Horizon renderer: pipelines ready (multi-draw on, occlusion culling off, GPU culling unavailable, GPU timer on)
load pack tables: 0.93s
Horizon renderer: acquiring the first frame
Horizon: core-ready after 935 ms
Horizon renderer: submitting the first frame
Horizon renderer: presenting the first frame
Horizon: handoff CPU resources prepared on worker in 70 ms
Horizon: loading screen shown after 1068 ms
Horizon: loading the title artwork after 1068 ms
Horizon renderer: llvmpipe (LLVM 21.1.8, 256 bits) (Vulkan, Cpu)
load items: 1.16s
Horizon: resources loaded in 1.2s; 35723 block states, 1658 items, 67 biomes; 3 worker threads
Horizon: rest-ready after 1206 ms
Horizon renderer: rebuilt on the same device (51 shader modules, layouts and pipelines kept)
Horizon renderer: llvmpipe (LLVM 21.1.8, 256 bits) (Vulkan, Cpu)
Horizon renderer: rebuilt on the same device (51 shader modules, layouts and pipelines kept)
Horizon: world renderer ready after 1626 ms
Horizon: no sounds in the active resources; prepare the game again in the launcher (or run horizon-prepare sounds)
Horizon: no sound: audio off: HORIZON_NO_AUDIO is set
Horizon: waiting for world resources
Horizon: world resources joined after 0 ms
Horizon: startup handoff 302 ms (renderer 281 ms, start_game 15 ms)
Horizon: first frame after 463 ms
Horizon: worldgen data loaded in 525.429094ms
Horizon: block sprites: 1301 layers at 64 px (largest 64 px, budget 256 MiB)
Horizon renderer: llvmpipe (LLVM 21.1.8, 256 bits) (Vulkan, Cpu)
Horizon renderer: rebuilt on the same device (51 shader modules, layouts and pipelines kept)
Failed to load post effect clod:lod_terrain: pass 4: reads the target it writes
XIO:  fatal IO error 2 (No such file or directory) on X server ":99"
      after 7456 requests (425 known processed) with 0 events remaining.
--- exceptions (Realms/RTKit noise filtered) ---
(no client log)
```
### server log tail
```
[c-LoD] EB: enter, native_gen=true
[17:39:40 INFO]: [ChunkHolderManager] Saved 1002 block chunks, 976 entity chunks, 0 poi chunks in world 'minecraft:overworld' in 0.84s
[17:39:40 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:overworld'
[17:39:40 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:overworld'
[17:39:40 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[c-LoD] EB: enter, native_gen=true
[17:39:40 INFO]: [ChunkHolderManager] Waiting 60s for chunk system to halt for world 'minecraft:the_nether'
[17:39:40 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:the_nether'
[17:39:40 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_nether'
[17:39:40 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_nether' in 0.00s
[17:39:40 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_nether'
[17:39:40 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_nether'
[17:39:40 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[17:39:40 INFO]: [ChunkHolderManager] Waiting 60s for chunk system to halt for world 'minecraft:the_end'
[17:39:40 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:the_end'
[17:39:40 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_end'
[17:39:40 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_end' in 0.00s
[17:39:40 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_end'
[17:39:40 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_end'
[17:39:40 INFO]: Waiting for all RegionFile I/O tasks to complete...
[17:39:40 INFO]: All RegionFile I/O tasks to complete
[17:39:40 INFO]: [MoonriseCommon] Awaiting termination of worker pool for up to 60s...
[17:39:40 INFO]: [MoonriseCommon] Awaiting termination of I/O pool for up to 60s...
[c-LoD] EB: enter, native_gen=true
[launcher] server exited with code 143
```
### shots
```
-rw-r--r-- 1 runner runner 203588 Nov 18 17:01 11_h165_spark.png
-rw-r--r-- 1 runner runner 179180 Nov 18 17:01 11_h168_spark.png
-rw-r--r-- 1 runner runner 167039 Nov 18 17:01 11_h172_spark.png
-rw-r--r-- 1 runner runner 179458 Nov 18 17:01 11_h176_spark.png
-rw-r--r-- 1 runner runner 172211 Nov 18 17:01 11_h180_spark.png
-rw-r--r-- 1 runner runner 170619 Nov 18 17:01 11_h185_spark.png
-rw-r--r-- 1 runner runner 165008 Nov 18 17:01 12_after_edit_probe0.png
-rw-r--r-- 1 runner runner 186817 Nov 18 17:09 13_after_relog_probe0.png
-rw-r--r-- 1 runner runner 168656 Nov 18 17:09 14_leg2_baseline.png
-rw-r--r-- 1 runner runner 169393 Nov 18 17:09 14_leg2_burst_far.png
-rw-r--r-- 1 runner runner 169231 Nov 18 17:09 14_leg2_burst_greendust.png
-rw-r--r-- 1 runner runner 169091 Nov 18 17:09 14_leg2_burst_near.png
-rw-r--r-- 1 runner runner 153411 Nov 18 17:09 diff_after_relog.png
-rw-r--r-- 1 runner runner 160606 Nov 18 17:01 diff_live_edit.png
-rw-r--r-- 1 runner runner      0 Nov 18 16:57 ffmpeg.err
-rw-r--r-- 1 runner runner   6869 Nov 18 17:01 lens_12_after_edit_probe0.png
-rw-r--r-- 1 runner runner   9095 Nov 18 17:09 lens_13_after_relog_probe0.png
drwxr-xr-x 2 runner runner   4096 Nov 18 17:33 lens_parity
drwxr-xr-x 2 runner runner   4096 Nov 18 17:33 parity
drwxr-xr-x 2 runner runner   4096 Nov 18 17:04 parity_dbg
drwxr-xr-x 2 runner runner   4096 Nov 18 17:36 parity_dbg8
-rw-r--r-- 1 runner runner    546 Nov 18 17:09 probe_counts.txt
drwxr-xr-x 2 runner runner   4096 Nov 18 17:39 rim_mask
-rw-r--r-- 1 runner runner 101378 Nov 18 17:39 server_full.log
-rw-r--r-- 1 runner runner  45256 Nov 18 17:39 summary.md
```

**exit=1** failures=2 leg1=1 leg2=1
