- phase: 0 sanity (20:09:45)
### c-LoD visual run (-930,796, marker=red_concrete, vantage -1005,150 yaw 270)
- GL:     Vendor: Mesa (0xffffffff);    Device: llvmpipe (LLVM 21.1.8, 256 bits) (0xffffffff);OpenGL renderer string: llvmpipe (LLVM 21.1.8, 256 bits);OpenGL core profile version string: 4.5 (Core Profile) Mesa 26.0.8-1ubuntu0.3;
- boot + gates OK (20:10:09)
- phase: 1 server booted (20:10:09)
- sky-grid armed
- phase: 2 leg-1 client joined (20:10:22)
- joined session 1 (20:10:22)
- stream live: cells=65536
- phase: 3 scan populated (20:10:45)
- terrain range: 2 terraced mountains at (CX-4,CZ-20)/(CX-2,CZ+16), tops 185/190, grass terraces + stone caps, 66..82 m from the eye (inside the 83.5 m band, chunk edge 64 m)
- scan complete=1 (cells=cells=65536/65536)
- paint settled=0 (wire 845984p)
- baseline try 1: drift=11182 px
- baseline try 2: drift=17643 px
- baseline try 3: drift=16734 px
- baseline try 4: drift=14402 px
- baseline try 5: drift=14170 px
- baseline try 6: drift=17512 px
- **FAIL:** no stable baseline after 6 tries (drift=17512) — the A/B would lie
- lod_silhouette_cols=61 (LOD paint ON SCREEN at 75 m if > 20 — the march-drawn marker column; vanilla cannot render there, chunk edge 64 m; #255 re-measures 49, the ZONE_FAR=48 regression #256 and the control read 0)
- 312 terrain-green px in the sky region (LOD MOUNTAINS on screen if > 5000; the range paints ColLow-green on low terraces via the fallback palette; flat-world baselines and the pack-free control read ~0)
- burst_close white px: 2755 (renderer alive if > 50)
- burst_far white px: 2313 (renderer alive in the marker zone if > 20)
- cut walk 40 m: 72 white px (> 20 = rendered)
- cut walk 60 m: 31 white px (> 20 = rendered)
- cut walk 100 m: 0 white px (> 20 = rendered)
- measured far-render cut: 60 m (G4 flap instrument; the decode proof needs >= 83 m from the y=151.6 eye)
- fill issued: 5x111x5 red_concrete at -930,796 (zone forceloaded ±48)
- phase: 4 marker filled (20:21:29)

### /clod-cells AFTER FILL (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 48 0 0x8400 0x2000
123 122 -940 784 71 0 1 [97, 119, 97] 48 0 0x8700 0x2000
124 122 -938 784 68 0 1 [97, 119, 97] 47 0 0x8400 0x2000
125 122 -936 784 62 1 1 [63, 118, 228] 87 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 86 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 85 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 85 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 158 0 0x8000 0x4100
130 122 -926 784 66 0 1 [134, 183, 131] 158 0 0x8200 0x4100
131 122 -924 784 75 0 1 [97, 119, 97] 158 0 0x8b00 0x2000
132 122 -922 784 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 122 -920 784 75 0 1 [97, 119, 97] 18446744073709551615 1 0x8b00 0x2000
134 122 -918 784 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x2000
122 123 -942 786 70 0 1 [97, 119, 97] 48 0 0x8600 0x2000
123 123 -940 786 65 0 1 [97, 119, 97] 48 0 0x8100 0x2000
124 123 -938 786 65 0 1 [97, 119, 97] 47 0 0x8100 0x2000
125 123 -936 786 62 1 1 [63, 118, 228] 87 0 0x7e00 0xd800
126 123 -934 786 62 1 1 [63, 118, 228] 86 0 0x7e00 0xd800
127 123 -932 786 62 1 1 [63, 118, 228] 85 0 0x7e00 0xd800
128 123 -930 786 62 1 1 [63, 118, 228] 85 0 0x7e00 0xd800
129 123 -928 786 73 0 1 [97, 119, 97] 158 0 0x8900 0x2000
130 123 -926 786 67 0 1 [134, 183, 131] 158 0 0x8300 0x4100
131 123 -924 786 69 0 1 [134, 183, 131] 155 0 0x8500 0x4100
132 123 -922 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 123 -920 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 123 -918 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
122 124 -942 788 72 0 1 [97, 119, 97] 48 0 0x8800 0x2000
123 124 -940 788 70 0 1 [97, 119, 97] 48 0 0x8600 0x2000
124 124 -938 788 68 0 1 [97, 119, 97] 47 0 0x8400 0x2000
125 124 -936 788 62 1 1 [63, 118, 228] 87 0 0x7e00 0xd800
126 124 -934 788 62 1 1 [63, 118, 228] 86 0 0x7e00 0xd800
127 124 -932 788 62 1 1 [63, 118, 228] 86 0 0x7e00 0xd800
128 124 -930 788 73 0 1 [97, 119, 97] 45 0 0x8900 0x2000
129 124 -928 788 76 0 1 [97, 119, 97] 155 0 0x8c00 0x2000
130 124 -926 788 73 0 1 [97, 119, 97] 155 0 0x8900 0x2000
131 124 -924 788 69 0 1 [134, 183, 131] 160 0 0x8500 0x4100
132 124 -922 788 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 124 -920 788 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 124 -918 788 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x2000
122 125 -942 790 74 0 1 [97, 119, 97] 48 0 0x8a00 0x2000
123 125 -940 790 73 0 1 [97, 119, 97] 48 0 0x8900 0x2000
124 125 -938 790 71 0 1 [97, 119, 97] 47 0 0x8700 0x2000
125 125 -936 790 62 1 1 [63, 118, 228] 87 0 0x7e00 0xd800
126 125 -934 790 62 1 1 [63, 118, 228] 86 0 0x7e00 0xd800
127 125 -932 790 62 1 1 [63, 118, 228] 86 0 0x7e00 0xd800
128 125 -930 790 62 1 1 [63, 118, 228] 85 0 0x7e00 0xd800
129 125 -928 790 73 0 1 [97, 119, 97] 160 0 0x8900 0x2000
130 125 -926 790 68 0 1 [134, 183, 131] 160 0 0x8400 0x4100
131 125 -924 790 69 0 1 [134, 183, 131] 160 0 0x8500 0x4100
132 125 -922 790 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
133 125 -920 790 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 125 -918 790 70 0 1 [134, 183, 131] 18446744073709551615 1 0x8600 0x4100
122 126 -942 792 74 0 1 [97, 119, 97] 48 0 0x8a00 0x2000
123 126 -940 792 73 0 1 [97, 119, 97] 48 0 0x8900 0x2000
124 126 -938 792 68 0 1 [97, 119, 97] 47 0 0x8400 0x2000
125 126 -936 792 62 1 1 [63, 118, 228] 87 0 0x7e00 0xd800
126 126 -934 792 62 1 1 [63, 118, 228] 86 0 0x7e00 0xd800
127 126 -932 792 62 1 1 [63, 118, 228] 86 0 0x7e00 0xd800
128 126 -930 792 62 1 1 [63, 118, 228] 85 0 0x7e00 0xd800
129 126 -928 792 67 0 1 [134, 183, 131] 160 0 0x8300 0x4100
130 126 -926 792 74 0 1 [97, 119, 97] 160 0 0x8a00 0x2000
131 126 -924 792 77 0 1 [97, 119, 97] 160 0 0x8d00 0x2000
132 126 -922 792 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 126 -920 792 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3200
134 126 -918 792 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x5000
122 127 -942 794 67 0 1 [134, 183, 131] 49 0 0x8300 0x4100
123 127 -940 794 68 0 1 [134, 183, 131] 48 0 0x8400 0x4100
124 127 -938 794 65 0 1 [134, 183, 131] 48 0 0x8100 0x4100
125 127 -936 794 62 1 1 [63, 118, 228] 87 0 0x7e00 0xd800
126 127 -934 794 62 1 1 [63, 118, 228] 86 0 0x7e00 0xd800
127 127 -932 794 170 0 1 [142, 33, 33] 3 0 0xea00 0x0a00
128 127 -930 794 170 0 1 [142, 33, 33] 3 0 0xea00 0x0a00
129 127 -928 794 170 0 1 [142, 33, 33] 1 0 0xea00 0x0a00
130 127 -926 794 74 0 1 [97, 119, 97] 160 0 0x8a00 0x2000
131 127 -924 794 77 0 1 [97, 119, 97] 160 0 0x8d00 0x2000
132 127 -922 794 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 127 -920 794 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3200
134 127 -918 794 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x5000
122 128 -942 796 68 0 1 [134, 183, 131] 49 0 0x8400 0x4100
123 128 -940 796 72 0 1 [97, 119, 97] 48 0 0x8800 0x2000
124 128 -938 796 75 0 1 [97, 119, 97] 48 0 0x8b00 0x2000
125 128 -936 796 72 0 1 [97, 119, 97] 47 0 0x8800 0x2000
126 128 -934 796 62 1 1 [63, 118, 228] 86 0 0x7e00 0xd800
127 128 -932 796 170 0 1 [142, 33, 33] 3 0 0xea00 0x0a00
128 128 -930 796 170 0 1 [142, 33, 33] 3 0 0xea00 0x0a00
129 128 -928 796 170 0 1 [142, 33, 33] 1 0 0xea00 0x0a00
130 128 -926 796 68 0 1 [134, 183, 131] 160 0 0x8400 0x4100
131 128 -924 796 74 0 1 [97, 119, 97] 160 0 0x8a00 0x2000
132 128 -922 796 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 128 -920 796 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
134 128 -918 796 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 129 -942 798 69 0 1 [134, 183, 131] 48 0 0x8500 0x4100
123 129 -940 798 75 0 1 [97, 119, 97] 48 0 0x8b00 0x2000
124 129 -938 798 78 0 1 [97, 119, 97] 49 0 0x8e00 0x2000
125 129 -936 798 75 0 1 [97, 119, 97] 47 0 0x8b00 0x2000
126 129 -934 798 62 1 1 [63, 118, 228] 86 0 0x7e00 0xd800
127 129 -932 798 170 0 1 [142, 33, 33] 3 0 0xea00 0x0a00
128 129 -930 798 170 0 1 [142, 33, 33] 3 0 0xea00 0x0a00
129 129 -928 798 170 0 1 [142, 33, 33] 11 0 0xea00 0x0a00
130 129 -926 798 71 0 1 [97, 119, 97] 160 0 0x8700 0x2000
131 129 -924 798 69 0 1 [134, 183, 131] 160 0 0x8500 0x4100
132 129 -922 798 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 129 -920 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 129 -918 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 130 -942 800 70 0 1 [134, 183, 131] 48 0 0x8600 0x4100
123 130 -940 800 76 0 1 [97, 119, 97] 48 0 0x8c00 0x2000
124 130 -938 800 76 0 1 [97, 119, 97] 47 0 0x8c00 0x2000
125 130 -936 800 72 0 1 [97, 119, 97] 47 0 0x8800 0x2000
126 130 -934 800 62 1 1 [63, 118, 228] 86 0 0x7e00 0xd800
127 130 -932 800 62 1 1 [63, 118, 228] 86 0 0x7e00 0xd800
128 130 -930 800 62 1 1 [63, 118, 228] 85 0 0x7e00 0xd800
129 130 -928 800 71 0 1 [97, 119, 97] 160 0 0x8700 0x2000
130 130 -926 800 71 0 1 [97, 119, 97] 160 0 0x8700 0x2000
131 130 -924 800 69 0 1 [134, 183, 131] 160 0 0x8500 0x4100
132 130 -922 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
133 130 -920 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 130 -918 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 131 -942 802 78 0 1 [97, 119, 97] 50 0 0x8e00 0x2000
123 131 -940 802 77 0 1 [97, 119, 97] 48 0 0x8d00 0x2000
124 131 -938 802 77 0 1 [97, 119, 97] 47 0 0x8d00 0x2000
125 131 -936 802 66 0 1 [134, 183, 131] 47 0 0x8200 0x4100
126 131 -934 802 62 1 1 [63, 118, 228] 86 0 0x7e00 0xd800
127 131 -932 802 62 1 1 [63, 118, 228] 86 0 0x7e00 0xd800
128 131 -930 802 62 1 1 [63, 118, 228] 85 0 0x7e00 0xd800
129 131 -928 802 62 1 1 [63, 118, 228] 200 0 0x7e00 0xd800
130 131 -926 802 66 0 1 [134, 183, 131] 160 0 0x8200 0x4100
131 131 -924 802 68 0 1 [134, 183, 131] 160 0 0x8400 0x4100
132 131 -922 802 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 131 -920 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 131 -918 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 132 -942 804 79 0 1 [97, 119, 97] 50 0 0x8f00 0x2000
123 132 -940 804 76 0 1 [97, 119, 97] 48 0 0x8c00 0x2000
124 132 -938 804 76 0 1 [97, 119, 97] 47 0 0x8c00 0x2000
125 132 -936 804 76 0 1 [97, 119, 97] 47 0 0x8c00 0x2000
126 132 -934 804 62 1 1 [63, 118, 228] 86 0 0x7e00 0xd800
127 132 -932 804 62 1 1 [63, 118, 228] 86 0 0x7e00 0xd800
128 132 -930 804 62 1 1 [63, 118, 228] 85 0 0x7e00 0xd800
129 132 -928 804 62 1 1 [63, 118, 228] 200 0 0x7e00 0xd800
130 132 -926 804 65 0 1 [134, 183, 131] 160 0 0x8100 0x4100
131 132 -924 804 77 0 1 [97, 119, 97] 160 0 0x8d00 0x2000
132 132 -922 804 79 0 1 [97, 119, 97] 160 0 0x8f00 0x2000
133 132 -920 804 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 132 -918 804 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x2000
122 133 -942 806 78 0 1 [97, 119, 97] 50 0 0x8e00 0x2000
123 133 -940 806 69 0 1 [134, 183, 131] 48 0 0x8500 0x4100
124 133 -938 806 155 0 1 [134, 183, 131] 47 0 0xdb00 0x4100
125 133 -936 806 155 0 1 [134, 183, 131] 47 0 0xdb00 0x4100
126 133 -934 806 155 0 1 [134, 183, 131] 46 0 0xdb00 0x4100
127 133 -932 806 155 0 1 [134, 183, 131] 45 0 0xdb00 0x4100
128 133 -930 806 155 0 1 [134, 183, 131] 45 0 0xdb00 0x4100
129 133 -928 806 155 0 1 [134, 183, 131] 160 0 0xdb00 0x4100
130 133 -926 806 155 0 1 [134, 183, 131] 160 0 0xdb00 0x4100
131 133 -924 806 67 0 1 [134, 183, 131] 160 0 0x8300 0x4100
132 133 -922 806 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 133 -920 806 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 133 -918 806 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x2000
122 134 -942 808 79 0 1 [97, 119, 97] 50 0 0x8f00 0x2000
123 134 -940 808 68 0 1 [134, 183, 131] 48 0 0x8400 0x4100
124 134 -938 808 155 0 1 [134, 183, 131] 47 0 0xdb00 0x4100
125 134 -936 808 180 0 1 [134, 183, 131] 47 0 0xf400 0x4100
126 134 -934 808 180 0 1 [134, 183, 131] 46 0 0xf400 0x4100
127 134 -932 808 180 0 1 [134, 183, 131] 45 0 0xf400 0x4100
128 134 -930 808 180 0 1 [134, 183, 131] 45 0 0xf400 0x4100
129 134 -928 808 180 0 1 [134, 183, 131] 160 0 0xf400 0x4100
130 134 -926 808 155 0 1 [134, 183, 131] 160 0 0xdb00 0x4100
131 134 -924 808 67 0 1 [134, 183, 131] 160 0 0x8300 0x4100
132 134 -922 808 68 0 1 [134, 183, 131] 18446744073709551615 1 0x8400 0x4100
133 134 -920 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 134 -918 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
```
### stream diag after fill:
[c-LoD] stream: tick 6800 65536/65536 cells, lap 959, cone 48001 zone 3193 dirty 1244 (q 16003) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1802881c/971e, edits +3116p/2337c/527e drop 0, spawned 0 / failed 0, cas 0d, cam0 yaw -90 pitch 5, wish 235 forced 277, native 65536c/1091ch fail 0 (3w q 0) helper on, trees 1091ch/fail 0, wire bundle 1143655p/2998b fail 0 ~82 kB/s cache 15439/24576 h1128216 m15439 e0, srbeat 91379s/9q drop 109stale/120undec, ereb 58466s/10q, sweep 0r/16d, cdiag off
[20:21:31 INFO]: [c-LoD] stream: tick 6800 65536/65536 cells, lap 959, cone 48001 zone 3193 dirty 1244 (q 16003) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1802881c/971e, edits +3116p/2337c/527e drop 0, spawned 0 / failed 0, cas 0d, cam0 yaw -90 pitch 5, wish 235 forced 277, native 65536c/1091ch fail 0 (3w q 0) helper on, trees 1091ch/fail 0, wire bundle 1143655p/2998b fail 0 ~82 kB/s cache 15439/24576 h1128216 m15439 e0, srbeat 91379s/9q drop 109stale/120undec, ereb 58466s/10q, sweep 0r/16d, cdiag off
- carrier ladder y=155: 13828 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=165: 11980 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=168: 15752 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=172: 15008 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=176: 12092 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=180: 15992 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=185: 12696 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- probe 0: red=16008 paint=131793 lod_silhouette_cols=91 lens_red=0 lens_black=0 lens_water=0 lens_dark=0 lens_zero=0 rot=none scan=0 carriers=0 (need red>=60 paint>=300 lod>20)
- **LEG 1 PASS** (zero relogs)
- phase: 5 leg-1 probes done (20:22:02)
### stream diag after probes:
[20:21:51 INFO]: [c-LoD] stream: tick 7000 65536/65536 cells, lap 977, cone 50285 zone 3197 dirty 1244 (q 16007) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1867436c/996e, edits +3619p/2728c/666e drop 0, spawned 0 / failed 0, cas 0d, cam0 yaw -90 pitch 5, wish 235 forced 277, native 65536c/1091ch fail 0 (3w q 0) helper on, trees 1091ch/fail 0, wire bundle 1175117p/3086b fail 0 ~82 kB/s cache 15439/24576 h1159678 m15439 e0, srbeat 93053s/0q drop 109stale/120undec, ereb 58628s/10q, sweep 0r/34d, cdiag off
[c-LoD] stream: tick 7100 65536/65536 cells, lap 986, cone 51427 zone 3197 dirty 1244 (q 16008) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1900598c/1003e, edits +3743p/2801c/684e drop 0, spawned 0 / failed 0, cas 0d, cam0 yaw -90 pitch 5, wish 235 forced 277, native 65536c/1091ch fail 0 (3w q 0) helper on, trees 1091ch/fail 0, wire bundle 1190852p/3133b fail 0 ~83 kB/s cache 15439/24576 h1175413 m15439 e0, srbeat 93891s/0q drop 109stale/120undec, ereb 58716s/10q, sweep 0r/43d, cdiag off
[20:22:01 INFO]: [c-LoD] stream: tick 7100 65536/65536 cells, lap 986, cone 51427 zone 3197 dirty 1244 (q 16008) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1900598c/1003e, edits +3743p/2801c/684e drop 0, spawned 0 / failed 0, cas 0d, cam0 yaw -90 pitch 5, wish 235 forced 277, native 65536c/1091ch fail 0 (3w q 0) helper on, trees 1091ch/fail 0, wire bundle 1190852p/3133b fail 0 ~83 kB/s cache 15439/24576 h1175413 m15439 e0, srbeat 93891s/0q drop 109stale/120undec, ereb 58716s/10q, sweep 0r/43d, cdiag off
- parity debug tour: 13 vantages (session-1 client, DEBUG pack — evidence only, the gate pairs the §7b world session)
- parity debug tour done: 13 shots in parity_dbg/ (the §7b world session feeds the gate)

### boot / stream
```
[c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 153603 bytes, sha1 2867a7a3d6fcecee18045acb46d0b51c4e117fa4 — zero textures, zero world data, one push per join
[c-LoD] c-LoD streaming world: world[20:10:06 INFO]: [c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 153603 bytes, sha1 2867a7a3d6fcecee18045acb46d0b51c4e117fa4 — zero textures, zero world data, one push per join
[c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[20:10:06 INFO]: [c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[c-LoD] stream: tick 14000 65536/65536 cells, lap 1511, cone 57401 zone 5461 dirty 4209 (q 15223) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 3964065c/1149e, edits +4851p/3576c/879e drop 0, spawned 0 / failed 0, cas 0d, cam0 yaw -180 pitch 5, wish 187 forced 329, native 65536c/1091ch fail 0 (3w q 0) helper on, trees 1091ch/fail 0, wire bundle 1517865p/4048b fail 0 ~0 kB/s cache 18976/24576 h1446915 m70950 e51974, srbeat 129308s/0q drop 112stale/1395undec, ereb 87439s/0q, sweep 0r/24d, cdiag off
[20:33:33 INFO]: [c-LoD] stream: tick 14000 65536/65536 cells, lap 1511, cone 57401 zone 5461 dirty 4209 (q 15223) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 3964065c/1149e, edits +4851p/3576c/879e drop 0, spawned 0 / failed 0, cas 0d, cam0 yaw -180 pitch 5, wish 187 forced 329, native 65536c/1091ch fail 0 (3w q 0) helper on, trees 1091ch/fail 0, wire bundle 1517865p/4048b fail 0 ~0 kB/s cache 18976/24576 h1446915 m70950 e51974, srbeat 129308s/0q drop 112stale/1395undec, ereb 87439s/0q, sweep 0r/24d, cdiag off
```
### sessions
```
[20:10:22 INFO]: System chat: Stepan joined the game
```
### client log tail
```
Horizon 0.1.0
Horizon renderer: Vulkan has no conformant adapter (Vulkan: only a software adapter (llvmpipe (LLVM 21.1.8, 256 bits))); asking again for the ones wgpu hides as non-conformant
Horizon renderer: trying OpenGL (Vulkan: only a software adapter (llvmpipe (LLVM 21.1.8, 256 bits)))
Horizon: ignoring minecraft:ambient_entity_effect: no such vanilla particle in this version
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
Horizon: UI bundle ready after 827 ms
Horizon: block sprites: 1301 layers at 64 px (largest 64 px, budget 256 MiB)
load core tables: 1.23s
load pack tables: 1.46s
Horizon: core-ready after 1475 ms
Horizon: handoff CPU resources prepared on worker in 148 ms
Horizon renderer: pipelines ready (multi-draw on, occlusion culling off, GPU culling unavailable, GPU timer on)
Horizon renderer: acquiring the first frame
Horizon renderer: submitting the first frame
load items: 1.81s
Horizon renderer: presenting the first frame
Horizon: resources loaded in 1.9s; 35723 block states, 1658 items, 67 biomes; 3 worker threads
Horizon: rest-ready after 1873 ms
Horizon: loading screen shown after 1878 ms
Horizon: loading the title artwork after 1878 ms
Horizon renderer: llvmpipe (LLVM 21.1.8, 256 bits) (Vulkan, Cpu)
Horizon renderer: rebuilt on the same device (51 shader modules, layouts and pipelines kept)
Horizon renderer: llvmpipe (LLVM 21.1.8, 256 bits) (Vulkan, Cpu)
Horizon renderer: rebuilt on the same device (51 shader modules, layouts and pipelines kept)
Horizon: world renderer ready after 2737 ms
Horizon: no sounds in the active resources; prepare the game again in the launcher (or run horizon-prepare sounds)
Horizon: no sound: audio off: HORIZON_NO_AUDIO is set
Horizon: waiting for world resources
Horizon: world resources joined after 0 ms
Horizon: startup handoff 498 ms (renderer 461 ms, start_game 22 ms)
Horizon: first frame after 421 ms
Horizon: worldgen data loaded in 419.578414ms
Horizon: disconnected: End of stream
XIO:  fatal IO error 2 (No such file or directory) on X server ":99"
      after 43683 requests (321 known processed) with 0 events remaining.
--- exceptions (Realms/RTKit noise filtered) ---
43:Horizon: disconnected: End of stream
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
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[20:33:42 INFO]: Stopping server
[launcher] server exited with code 143
```
### shots
```
total 3508
drwxr-xr-x 3 runner runner   4096 Nov 19 20:33 .
drwxr-xr-x 3 runner runner   4096 Nov 19 19:48 ..
-rw-r--r-- 1 runner runner 164890 Nov 19 20:20 09_burst_close.png
-rw-r--r-- 1 runner runner 159288 Nov 19 20:20 09b_burst_far.png
-rw-r--r-- 1 runner runner 174570 Nov 19 20:21 09w100.png
-rw-r--r-- 1 runner runner 159616 Nov 19 20:21 09w40.png
-rw-r--r-- 1 runner runner 159794 Nov 19 20:21 09w60.png
-rw-r--r-- 1 runner runner 159976 Nov 19 20:20 10_before_edit.png
-rw-r--r-- 1 runner runner 188038 Nov 19 20:20 11_baseline_settle.png
-rw-r--r-- 1 runner runner 190989 Nov 19 20:21 11_h155_spark.png
-rw-r--r-- 1 runner runner 166827 Nov 19 20:21 11_h165_spark.png
-rw-r--r-- 1 runner runner 200759 Nov 19 20:21 11_h168_spark.png
-rw-r--r-- 1 runner runner 196406 Nov 19 20:21 11_h172_spark.png
-rw-r--r-- 1 runner runner 169274 Nov 19 20:21 11_h176_spark.png
-rw-r--r-- 1 runner runner 202783 Nov 19 20:21 11_h180_spark.png
-rw-r--r-- 1 runner runner 173852 Nov 19 20:21 11_h185_spark.png
-rw-r--r-- 1 runner runner 185921 Nov 19 20:22 12_after_edit_probe0.png
-rw-r--r-- 1 runner runner 158581 Nov 19 20:22 diff_live_edit.png
-rw-r--r-- 1 runner runner      0 Nov 19 20:18 ffmpeg.err
-rw-r--r-- 1 runner runner  11108 Nov 19 20:22 lens_12_after_edit_probe0.png
drwxr-xr-x 2 runner runner   4096 Nov 19 20:24 parity_dbg
-rw-r--r-- 1 runner runner    141 Nov 19 20:22 probe_counts.txt
-rw-r--r-- 1 runner runner 693731 Nov 19 20:33 server_full.log
-rw-r--r-- 1 runner runner  23302 Nov 19 20:33 summary.md
```

**exit=1** failures=1 leg1=1 leg2=0
