- phase: 0 sanity (18:27:32)
### c-LoD visual run (-930,796, marker=red_concrete, vantage -1005,150 yaw 270)
- GL:     Vendor: Mesa (0xffffffff);    Device: llvmpipe (LLVM 21.1.8, 256 bits) (0xffffffff);OpenGL renderer string: llvmpipe (LLVM 21.1.8, 256 bits);OpenGL core profile version string: 4.5 (Core Profile) Mesa 26.0.8-1ubuntu0.3;
- boot + gates OK (18:27:47)
- phase: 1 server booted (18:27:47)
- sky-grid armed
- phase: 2 leg-1 client joined (18:27:58)
- joined session 1 (18:27:58)
- stream live: cells=65536
- phase: 3 scan populated (18:28:21)
- terrain range: 2 terraced mountains at (CX-4,CZ-20)/(CX-2,CZ+16), tops 185/190, grass terraces + stone caps, 66..82 m from the eye (inside the 83.5 m band, chunk edge 64 m)
- scan complete=1 (cells=cells=65536/65536)
- paint settled=1 (wire 553291p)
- baseline try 1: drift=23025 px
- baseline try 2: drift=22739 px
- baseline try 3: drift=18326 px
- baseline try 4: drift=26006 px
- baseline try 5: drift=11057 px
- baseline try 6: drift=13500 px
- **FAIL:** no stable baseline after 6 tries (drift=13500) — the A/B would lie
- lod_silhouette_cols=61 (LOD paint ON SCREEN at 75 m if > 20 — the march-drawn marker column; vanilla cannot render there, chunk edge 64 m; #255 re-measures 49, the ZONE_FAR=48 regression #256 and the control read 0)
- 3985 terrain-green px in the sky region (LOD MOUNTAINS on screen if > 5000; the range paints ColLow-green on low terraces via the fallback palette; flat-world baselines and the pack-free control read ~0)
- burst_close white px: 1733 (renderer alive if > 50)
- burst_far white px: 1150 (renderer alive in the marker zone if > 20)
- cut walk 40 m: 73 white px (> 20 = rendered)
- cut walk 60 m: 32 white px (> 20 = rendered)
- cut walk 100 m: 0 white px (> 20 = rendered)
- measured far-render cut: 60 m (G4 flap instrument; the decode proof needs >= 83 m from the y=151.6 eye)
- fill issued: 5x111x5 red_concrete at -930,796 (zone forceloaded ±48)
- phase: 4 marker filled (18:36:24)

### /clod-cells AFTER FILL (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 68 0 0x8400 0x2000
123 122 -940 784 71 0 1 [97, 119, 97] 68 0 0x8700 0x2000
124 122 -938 784 68 0 1 [97, 119, 97] 67 0 0x8400 0x2000
125 122 -936 784 62 1 1 [63, 118, 228] 107 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 106 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 105 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 105 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 2 0 0x8000 0x4100
130 122 -926 784 66 0 1 [134, 183, 131] 2 0 0x8200 0x4100
131 122 -924 784 75 0 1 [97, 119, 97] 2 0 0x8b00 0x2000
132 122 -922 784 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 122 -920 784 75 0 1 [97, 119, 97] 18446744073709551615 1 0x8b00 0x2000
134 122 -918 784 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x2000
122 123 -942 786 70 0 1 [97, 119, 97] 68 0 0x8600 0x2000
123 123 -940 786 65 0 1 [97, 119, 97] 68 0 0x8100 0x2000
124 123 -938 786 65 0 1 [97, 119, 97] 67 0 0x8100 0x2000
125 123 -936 786 62 1 1 [63, 118, 228] 107 0 0x7e00 0xd800
126 123 -934 786 62 1 1 [63, 118, 228] 106 0 0x7e00 0xd800
127 123 -932 786 62 1 1 [63, 118, 228] 106 0 0x7e00 0xd800
128 123 -930 786 62 1 1 [63, 118, 228] 105 0 0x7e00 0xd800
129 123 -928 786 73 0 1 [97, 119, 97] 2 0 0x8900 0x2000
130 123 -926 786 67 0 1 [134, 183, 131] 2 0 0x8300 0x4100
131 123 -924 786 69 0 1 [134, 183, 131] 2 0 0x8500 0x4100
132 123 -922 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 123 -920 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 123 -918 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
122 124 -942 788 72 0 1 [97, 119, 97] 68 0 0x8800 0x2000
123 124 -940 788 70 0 1 [97, 119, 97] 68 0 0x8600 0x2000
124 124 -938 788 68 0 1 [97, 119, 97] 67 0 0x8400 0x2000
125 124 -936 788 62 1 1 [63, 118, 228] 107 0 0x7e00 0xd800
126 124 -934 788 62 1 1 [63, 118, 228] 106 0 0x7e00 0xd800
127 124 -932 788 62 1 1 [63, 118, 228] 106 0 0x7e00 0xd800
128 124 -930 788 73 0 1 [97, 119, 97] 65 0 0x8900 0x2000
129 124 -928 788 76 0 1 [97, 119, 97] 2 0 0x8c00 0x2000
130 124 -926 788 73 0 1 [97, 119, 97] 2 0 0x8900 0x2000
131 124 -924 788 69 0 1 [134, 183, 131] 2 0 0x8500 0x4100
132 124 -922 788 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 124 -920 788 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 124 -918 788 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x2000
122 125 -942 790 74 0 1 [97, 119, 97] 68 0 0x8a00 0x2000
123 125 -940 790 73 0 1 [97, 119, 97] 68 0 0x8900 0x2000
124 125 -938 790 71 0 1 [97, 119, 97] 67 0 0x8700 0x2000
125 125 -936 790 62 1 1 [63, 118, 228] 107 0 0x7e00 0xd800
126 125 -934 790 62 1 1 [63, 118, 228] 106 0 0x7e00 0xd800
127 125 -932 790 62 1 1 [63, 118, 228] 106 0 0x7e00 0xd800
128 125 -930 790 62 1 1 [63, 118, 228] 105 0 0x7e00 0xd800
129 125 -928 790 73 0 1 [97, 119, 97] 65 0 0x8900 0x2000
130 125 -926 790 68 0 1 [134, 183, 131] 2 0 0x8400 0x4100
131 125 -924 790 69 0 1 [134, 183, 131] 2 0 0x8500 0x4100
132 125 -922 790 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
133 125 -920 790 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 125 -918 790 70 0 1 [134, 183, 131] 18446744073709551615 1 0x8600 0x4100
122 126 -942 792 74 0 1 [97, 119, 97] 68 0 0x8a00 0x2000
123 126 -940 792 73 0 1 [97, 119, 97] 68 0 0x8900 0x2000
124 126 -938 792 68 0 1 [97, 119, 97] 67 0 0x8400 0x2000
125 126 -936 792 62 1 1 [63, 118, 228] 107 0 0x7e00 0xd800
126 126 -934 792 62 1 1 [63, 118, 228] 106 0 0x7e00 0xd800
127 126 -932 792 62 1 1 [63, 118, 228] 106 0 0x7e00 0xd800
128 126 -930 792 62 1 1 [63, 118, 228] 105 0 0x7e00 0xd800
129 126 -928 792 67 0 1 [134, 183, 131] 65 0 0x8300 0x4100
130 126 -926 792 74 0 1 [97, 119, 97] 2 0 0x8a00 0x2000
131 126 -924 792 77 0 1 [97, 119, 97] 2 0 0x8d00 0x2000
132 126 -922 792 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 126 -920 792 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3200
134 126 -918 792 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x5000
122 127 -942 794 67 0 1 [134, 183, 131] 68 0 0x8300 0x4100
123 127 -940 794 68 0 1 [134, 183, 131] 68 0 0x8400 0x4100
124 127 -938 794 65 0 1 [134, 183, 131] 67 0 0x8100 0x4100
125 127 -936 794 62 1 1 [63, 118, 228] 107 0 0x7e00 0xd800
126 127 -934 794 62 1 1 [63, 118, 228] 106 0 0x7e00 0xd800
127 127 -932 794 62 1 1 [63, 118, 228] 106 0 0x7e00 0xd800
128 127 -930 794 62 1 1 [63, 118, 228] 105 0 0x7e00 0xd800
129 127 -928 794 66 0 1 [134, 183, 131] 65 0 0x8200 0x4100
130 127 -926 794 74 0 1 [97, 119, 97] 2 0 0x8a00 0x2000
131 127 -924 794 77 0 1 [97, 119, 97] 2 0 0x8d00 0x2000
132 127 -922 794 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 127 -920 794 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3200
134 127 -918 794 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x5000
122 128 -942 796 68 0 1 [134, 183, 131] 0 0 0x8400 0x4100
123 128 -940 796 72 0 1 [97, 119, 97] 68 0 0x8800 0x2000
124 128 -938 796 75 0 1 [97, 119, 97] 67 0 0x8b00 0x2000
125 128 -936 796 72 0 1 [97, 119, 97] 67 0 0x8800 0x2000
126 128 -934 796 62 1 1 [63, 118, 228] 106 0 0x7e00 0xd800
127 128 -932 796 62 1 1 [63, 118, 228] 106 0 0x7e00 0xd800
128 128 -930 796 62 1 1 [63, 118, 228] 105 0 0x7e00 0xd800
129 128 -928 796 66 0 1 [134, 183, 131] 65 0 0x8200 0x4100
130 128 -926 796 68 0 1 [134, 183, 131] 2 0 0x8400 0x4100
131 128 -924 796 74 0 1 [97, 119, 97] 2 0 0x8a00 0x2000
132 128 -922 796 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 128 -920 796 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
134 128 -918 796 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 129 -942 798 69 0 1 [134, 183, 131] 68 0 0x8500 0x4100
123 129 -940 798 75 0 1 [97, 119, 97] 68 0 0x8b00 0x2000
124 129 -938 798 78 0 1 [97, 119, 97] 0 0 0x8e00 0x2000
125 129 -936 798 75 0 1 [97, 119, 97] 67 0 0x8b00 0x2000
126 129 -934 798 62 1 1 [63, 118, 228] 106 0 0x7e00 0xd800
127 129 -932 798 62 1 1 [63, 118, 228] 106 0 0x7e00 0xd800
128 129 -930 798 62 1 1 [63, 118, 228] 105 0 0x7e00 0xd800
129 129 -928 798 71 0 1 [97, 119, 97] 65 0 0x8700 0x2000
130 129 -926 798 71 0 1 [97, 119, 97] 2 0 0x8700 0x2000
131 129 -924 798 69 0 1 [134, 183, 131] 2 0 0x8500 0x4100
132 129 -922 798 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 129 -920 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 129 -918 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 130 -942 800 70 0 1 [134, 183, 131] 68 0 0x8600 0x4100
123 130 -940 800 76 0 1 [97, 119, 97] 68 0 0x8c00 0x2000
124 130 -938 800 76 0 1 [97, 119, 97] 67 0 0x8c00 0x2000
125 130 -936 800 72 0 1 [97, 119, 97] 67 0 0x8800 0x2000
126 130 -934 800 62 1 1 [63, 118, 228] 106 0 0x7e00 0xd800
127 130 -932 800 62 1 1 [63, 118, 228] 106 0 0x7e00 0xd800
128 130 -930 800 62 1 1 [63, 118, 228] 105 0 0x7e00 0xd800
129 130 -928 800 71 0 1 [97, 119, 97] 65 0 0x8700 0x2000
130 130 -926 800 71 0 1 [97, 119, 97] 2 0 0x8700 0x2000
131 130 -924 800 69 0 1 [122, 122, 122] 2 0 0x8500 0x2a00
132 130 -922 800 69 0 1 [122, 122, 122] 18446744073709551615 0 0x8500 0x2a00
133 130 -920 800 69 0 1 [122, 122, 122] 18446744073709551615 0 0x8500 0x2a00
134 130 -918 800 69 0 1 [122, 122, 122] 18446744073709551615 0 0x8500 0x2a00
122 131 -942 802 78 0 1 [97, 119, 97] 1 0 0x8e00 0x2000
123 131 -940 802 77 0 1 [97, 119, 97] 68 0 0x8d00 0x2000
124 131 -938 802 77 0 1 [97, 119, 97] 67 0 0x8d00 0x2000
125 131 -936 802 66 0 1 [134, 183, 131] 67 0 0x8200 0x4100
126 131 -934 802 62 1 1 [63, 118, 228] 106 0 0x7e00 0xd800
127 131 -932 802 62 1 1 [63, 118, 228] 106 0 0x7e00 0xd800
128 131 -930 802 62 1 1 [63, 118, 228] 105 0 0x7e00 0xd800
129 131 -928 802 62 1 1 [63, 118, 228] 2 0 0x7e00 0xd800
130 131 -926 802 66 0 1 [122, 122, 122] 2 0 0x8200 0x2a00
131 131 -924 802 68 0 1 [122, 122, 122] 2 0 0x8400 0x2a00
132 131 -922 802 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 131 -920 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 131 -918 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 132 -942 804 79 0 1 [97, 119, 97] 1 0 0x8f00 0x2000
123 132 -940 804 76 0 1 [97, 119, 97] 68 0 0x8c00 0x2000
124 132 -938 804 76 0 1 [97, 119, 97] 67 0 0x8c00 0x2000
125 132 -936 804 76 0 1 [97, 119, 97] 67 0 0x8c00 0x2000
126 132 -934 804 62 1 1 [63, 118, 228] 106 0 0x7e00 0xd800
127 132 -932 804 62 1 1 [63, 118, 228] 106 0 0x7e00 0xd800
128 132 -930 804 62 1 1 [63, 118, 228] 105 0 0x7e00 0xd800
129 132 -928 804 62 1 1 [63, 118, 228] 2 0 0x7e00 0xd800
130 132 -926 804 65 0 1 [122, 122, 122] 2 0 0x8100 0x2a00
131 132 -924 804 77 0 1 [97, 119, 97] 2 0 0x8d00 0x2000
132 132 -922 804 79 0 1 [97, 119, 97] 2 0 0x8f00 0x2000
133 132 -920 804 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 132 -918 804 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x2000
122 133 -942 806 78 0 1 [97, 119, 97] 1 0 0x8e00 0x2000
123 133 -940 806 69 0 1 [134, 183, 131] 68 0 0x8500 0x4100
124 133 -938 806 155 0 1 [134, 183, 131] 68 0 0xdb00 0x4100
125 133 -936 806 155 0 1 [134, 183, 131] 68 0 0xdb00 0x4100
126 133 -934 806 155 0 1 [134, 183, 131] 67 0 0xdb00 0x4100
127 133 -932 806 155 0 1 [134, 183, 131] 67 0 0xdb00 0x4100
128 133 -930 806 155 0 1 [134, 183, 131] 66 0 0xdb00 0x4100
129 133 -928 806 155 0 1 [134, 183, 131] 65 0 0xdb00 0x4100
130 133 -926 806 155 0 1 [134, 183, 131] 65 0 0xdb00 0x4100
131 133 -924 806 67 0 1 [122, 122, 122] 2 0 0x8300 0x2a00
132 133 -922 806 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 133 -920 806 69 0 1 [122, 122, 122] 18446744073709551615 0 0x8500 0x2a00
134 133 -918 806 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x2000
122 134 -942 808 79 0 1 [97, 119, 97] 1 0 0x8f00 0x2000
123 134 -940 808 68 0 1 [134, 183, 131] 68 0 0x8400 0x4100
124 134 -938 808 155 0 1 [134, 183, 131] 68 0 0xdb00 0x4100
125 134 -936 808 180 0 1 [134, 183, 131] 65 0 0xf400 0x4100
126 134 -934 808 180 0 1 [134, 183, 131] 65 0 0xf400 0x4100
127 134 -932 808 180 0 1 [134, 183, 131] 2 0 0xf400 0x4100
128 134 -930 808 180 0 1 [134, 183, 131] 2 0 0xf400 0x4100
129 134 -928 808 180 0 1 [134, 183, 131] 2 0 0xf400 0x4100
130 134 -926 808 155 0 1 [134, 183, 131] 65 0 0xdb00 0x4100
131 134 -924 808 67 0 1 [122, 122, 122] 2 0 0x8300 0x2a00
132 134 -922 808 68 0 1 [134, 183, 131] 18446744073709551615 1 0x8400 0x4100
133 134 -920 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 134 -918 808 69 0 1 [122, 122, 122] 18446744073709551615 0 0x8500 0x2a00
```
### stream diag after fill:
[c-LoD] stream: tick 5200 65536/65536 cells, lap 819, cone 30621 zone 2636 dirty 936 (q 11291) ewin 180 rbeat 9p/1 rearm 0c/0t rescan 1369655c/3176e, edits +6175p/4682c/744e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 342 forced 170, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 796101p/2398b fail 0 ~63 kB/s cache 13623/24576 h782478 m13623 e0, srbeat 65689s/9q drop 58stale/112undec, ereb 49474s/9q, sweep 0r/86d
[18:36:30 INFO]: [c-LoD] stream: tick 5200 65536/65536 cells, lap 819, cone 30621 zone 2636 dirty 936 (q 11291) ewin 180 rbeat 9p/1 rearm 0c/0t rescan 1369655c/3176e, edits +6175p/4682c/744e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 342 forced 170, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 796101p/2398b fail 0 ~63 kB/s cache 13623/24576 h782478 m13623 e0, srbeat 65689s/9q drop 58stale/112undec, ereb 49474s/9q, sweep 0r/86d
- carrier ladder y=155: 13544 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=165: 13972 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=168: 13612 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=172: 13516 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=176: 13292 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=180: 13880 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=185: 13672 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- probe 0: red=16054 paint=117253 lod_silhouette_cols=91 lens_red=0 lens_black=0 lens_water=0 lens_dark=0 lens_zero=0 rot=none scan=0 carriers=0 (need red>=60 paint>=300 lod>20)
- **LEG 1 PASS** (zero relogs)
- phase: 5 leg-1 probes done (18:36:56)
### stream diag after probes:
[c-LoD] stream: tick 5300 65536/65536 cells, lap 819, cone 30621 zone 2636 dirty 945 (q 11308) ewin 180 rbeat 9p/0 rearm 0c/0t rescan 1403001c/3191e, edits +6376p/4833c/805e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 342 forced 170, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 796830p/2408b fail 0 ~3 kB/s cache 13623/24576 h783207 m13623 e0, srbeat 65770s/9q drop 58stale/112undec, ereb 49546s/9q, sweep 0r/86d
[c-LoD] stream: tick 5400 65536/65536 cells, lap 819, cone 30621 zone 2636 dirty 954 (q 11311) ewin 180 rbeat 0p/0 rearm 0c/0t rescan 1434244c/3198e, edits +6473p/4895c/819e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 342 forced 170, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 797559p/2418b fail 0 ~3 kB/s cache 13623/24576 h783936 m13623 e0, srbeat 65851s/9q drop 58stale/112undec, ereb 49618s/9q, sweep 0r/86d
[18:36:50 INFO]: [c-LoD] stream: tick 5400 65536/65536 cells, lap 819, cone 30621 zone 2636 dirty 954 (q 11311) ewin 180 rbeat 0p/0 rearm 0c/0t rescan 1434244c/3198e, edits +6473p/4895c/819e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 342 forced 170, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 797559p/2418b fail 0 ~3 kB/s cache 13623/24576 h783936 m13623 e0, srbeat 65851s/9q drop 58stale/112undec, ereb 49618s/9q, sweep 0r/86d
- parity debug tour: 13 vantages (session-1 client, DEBUG pack — evidence only, the gate pairs the §7b world session)
- parity debug tour done: 13 shots in parity_dbg/ (the §7b world session feeds the gate)
- joined session 2 (18:39:36)
- leg-2 baseline scene px (x<940): 24864 (0 = the relog client paints no world at all)
- leg-2 renderer probe NEAR (20 m) white px: 1338 raw / 2532 changed-vs-baseline (> 20 = the client renders particles)
- leg-2 renderer probe FAR (80 m, marker zone) white px: 651 raw / 3689 changed-vs-baseline (> 20 = no distance cut; ~0 with NEAR alive = particles beyond the client chunk radius do not render)
- leg-2 carrier probe green dust (80 m): changed-green px 0 / total changed 22014 vs baseline (> 100 green = a stable green carrier exists; total > 0 with green ~0 = the dust renders but fog washes it — a ruler problem, not a carrier problem)
- phase: 8 leg-2 relog probes (18:44:59)
- relog probe 0: red=16054 scene=24866 lens_red=0 lens_black=0 lens_water=0 lens_dark=0 lens_zero=0 rot=none scan=0 carriers=0 (need red >= 60; scene > 0 = the client paints a world at all)
- **LEG 2 PASS** (marker persisted over relog)
- phase: 9 leg-2 done (18:45:01)

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
122 122 -942 784 68 0 1 [97, 119, 97] 9 0 0x8400 0x2000
123 122 -940 784 71 0 1 [97, 119, 97] 9 0 0x8700 0x2000
124 122 -938 784 68 0 1 [97, 119, 97] 8 0 0x8400 0x2000
125 122 -936 784 62 1 1 [63, 118, 228] 8 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 8 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 8 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 8 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 8 0 0x8000 0x4100
130 122 -926 784 66 0 1 [134, 183, 131] 8 0 0x8200 0x4100
131 122 -924 784 75 0 1 [97, 119, 97] 8 0 0x8b00 0x2000
132 122 -922 784 77 0 1 [97, 119, 97] 4053 1 0x8d00 0x2000
133 122 -920 784 75 0 1 [97, 119, 97] 4053 1 0x8b00 0x2000
134 122 -918 784 73 0 1 [97, 119, 97] 4053 1 0x8900 0x2000
122 123 -942 786 70 0 1 [97, 119, 97] 9 0 0x8600 0x2000
123 123 -940 786 65 0 1 [97, 119, 97] 9 0 0x8100 0x2000
124 123 -938 786 65 0 1 [97, 119, 97] 8 0 0x8100 0x2000
125 123 -936 786 62 1 1 [63, 118, 228] 8 0 0x7e00 0xd800
126 123 -934 786 62 1 1 [63, 118, 228] 8 0 0x7e00 0xd800
127 123 -932 786 62 1 1 [63, 118, 228] 8 0 0x7e00 0xd800
128 123 -930 786 62 1 1 [63, 118, 228] 8 0 0x7e00 0xd800
129 123 -928 786 73 0 1 [97, 119, 97] 8 0 0x8900 0x2000
130 123 -926 786 67 0 1 [134, 183, 131] 8 0 0x8300 0x4100
131 123 -924 786 69 0 1 [134, 183, 131] 8 0 0x8500 0x4100
132 123 -922 786 77 0 1 [97, 119, 97] 4052 1 0x8d00 0x2000
133 123 -920 786 77 0 1 [97, 119, 97] 4052 1 0x8d00 0x2000
134 123 -918 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
122 124 -942 788 72 0 1 [97, 119, 97] 9 0 0x8800 0x2000
123 124 -940 788 70 0 1 [97, 119, 97] 9 0 0x8600 0x2000
124 124 -938 788 68 0 1 [97, 119, 97] 9 0 0x8400 0x2000
125 124 -936 788 62 1 1 [63, 118, 228] 8 0 0x7e00 0xd800
126 124 -934 788 62 1 1 [63, 118, 228] 8 0 0x7e00 0xd800
127 124 -932 788 62 1 1 [63, 118, 228] 8 0 0x7e00 0xd800
128 124 -930 788 73 0 1 [97, 119, 97] 8 0 0x8900 0x2000
129 124 -928 788 76 0 1 [97, 119, 97] 8 0 0x8c00 0x2000
130 124 -926 788 73 0 1 [97, 119, 97] 8 0 0x8900 0x2000
131 124 -924 788 69 0 1 [134, 183, 131] 8 0 0x8500 0x4100
132 124 -922 788 77 0 1 [97, 119, 97] 4052 1 0x8d00 0x2000
133 124 -920 788 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 124 -918 788 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x2000
122 125 -942 790 74 0 1 [97, 119, 97] 9 0 0x8a00 0x2000
123 125 -940 790 73 0 1 [97, 119, 97] 9 0 0x8900 0x2000
124 125 -938 790 71 0 1 [97, 119, 97] 9 0 0x8700 0x2000
125 125 -936 790 62 1 1 [63, 118, 228] 8 0 0x7e00 0xd800
126 125 -934 790 62 1 1 [63, 118, 228] 8 0 0x7e00 0xd800
127 125 -932 790 62 1 1 [63, 118, 228] 8 0 0x7e00 0xd800
128 125 -930 790 62 1 1 [63, 118, 228] 8 0 0x7e00 0xd800
129 125 -928 790 73 0 1 [97, 119, 97] 8 0 0x8900 0x2000
130 125 -926 790 68 0 1 [134, 183, 131] 8 0 0x8400 0x4100
131 125 -924 790 69 0 1 [134, 183, 131] 8 0 0x8500 0x4100
132 125 -922 790 69 0 1 [134, 183, 131] 4052 1 0x8500 0x4100
133 125 -920 790 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 125 -918 790 70 0 1 [134, 183, 131] 18446744073709551615 1 0x8600 0x4100
122 126 -942 792 74 0 1 [97, 119, 97] 9 0 0x8a00 0x2000
123 126 -940 792 73 0 1 [97, 119, 97] 9 0 0x8900 0x2000
124 126 -938 792 68 0 1 [97, 119, 97] 9 0 0x8400 0x2000
125 126 -936 792 62 1 1 [63, 118, 228] 8 0 0x7e00 0xd800
126 126 -934 792 62 1 1 [63, 118, 228] 8 0 0x7e00 0xd800
127 126 -932 792 62 1 1 [63, 118, 228] 8 0 0x7e00 0xd800
128 126 -930 792 62 1 1 [63, 118, 228] 8 0 0x7e00 0xd800
129 126 -928 792 67 0 1 [134, 183, 131] 8 0 0x8300 0x4100
130 126 -926 792 74 0 1 [97, 119, 97] 8 0 0x8a00 0x2000
131 126 -924 792 77 0 1 [97, 119, 97] 8 0 0x8d00 0x2000
132 126 -922 792 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 126 -920 792 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3200
134 126 -918 792 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x5000
122 127 -942 794 67 0 1 [134, 183, 131] 9 0 0x8300 0x4100
123 127 -940 794 68 0 1 [134, 183, 131] 9 0 0x8400 0x4100
124 127 -938 794 65 0 1 [134, 183, 131] 9 0 0x8100 0x4100
125 127 -936 794 62 1 1 [63, 118, 228] 8 0 0x7e00 0xd800
126 127 -934 794 62 1 1 [63, 118, 228] 8 0 0x7e00 0xd800
127 127 -932 794 170 0 1 [142, 33, 33] 8 0 0xea00 0x0a00
128 127 -930 794 170 0 1 [142, 33, 33] 8 0 0xea00 0x0a00
129 127 -928 794 170 0 1 [142, 33, 33] 8 0 0xea00 0x0a00
130 127 -926 794 74 0 1 [97, 119, 97] 8 0 0x8a00 0x2000
131 127 -924 794 77 0 1 [97, 119, 97] 8 0 0x8d00 0x2000
132 127 -922 794 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 127 -920 794 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3200
134 127 -918 794 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x5000
122 128 -942 796 68 0 1 [134, 183, 131] 9 0 0x8400 0x4100
123 128 -940 796 72 0 1 [97, 119, 97] 9 0 0x8800 0x2000
124 128 -938 796 75 0 1 [97, 119, 97] 9 0 0x8b00 0x2000
125 128 -936 796 72 0 1 [97, 119, 97] 8 0 0x8800 0x2000
126 128 -934 796 62 1 1 [63, 118, 228] 8 0 0x7e00 0xd800
127 128 -932 796 170 0 1 [142, 33, 33] 8 0 0xea00 0x0a00
128 128 -930 796 170 0 1 [142, 33, 33] 8 0 0xea00 0x0a00
129 128 -928 796 170 0 1 [142, 33, 33] 8 0 0xea00 0x0a00
130 128 -926 796 68 0 1 [134, 183, 131] 8 0 0x8400 0x4100
131 128 -924 796 74 0 1 [97, 119, 97] 8 0 0x8a00 0x2000
132 128 -922 796 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 128 -920 796 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
134 128 -918 796 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 129 -942 798 69 0 1 [134, 183, 131] 9 0 0x8500 0x4100
123 129 -940 798 75 0 1 [97, 119, 97] 9 0 0x8b00 0x2000
124 129 -938 798 78 0 1 [97, 119, 97] 10 0 0x8e00 0x2000
125 129 -936 798 75 0 1 [97, 119, 97] 8 0 0x8b00 0x2000
126 129 -934 798 62 1 1 [63, 118, 228] 8 0 0x7e00 0xd800
127 129 -932 798 170 0 1 [142, 33, 33] 8 0 0xea00 0x0a00
128 129 -930 798 170 0 1 [142, 33, 33] 8 0 0xea00 0x0a00
129 129 -928 798 170 0 1 [142, 33, 33] 8 0 0xea00 0x0a00
130 129 -926 798 71 0 1 [97, 119, 97] 8 0 0x8700 0x2000
131 129 -924 798 69 0 1 [134, 183, 131] 8 0 0x8500 0x4100
132 129 -922 798 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 129 -920 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 129 -918 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 130 -942 800 70 0 1 [134, 183, 131] 9 0 0x8600 0x4100
123 130 -940 800 76 0 1 [97, 119, 97] 9 0 0x8c00 0x2000
124 130 -938 800 76 0 1 [97, 119, 97] 9 0 0x8c00 0x2000
125 130 -936 800 72 0 1 [97, 119, 97] 8 0 0x8800 0x2000
126 130 -934 800 62 1 1 [63, 118, 228] 8 0 0x7e00 0xd800
127 130 -932 800 62 1 1 [63, 118, 228] 8 0 0x7e00 0xd800
128 130 -930 800 62 1 1 [63, 118, 228] 8 0 0x7e00 0xd800
129 130 -928 800 71 0 1 [97, 119, 97] 8 0 0x8700 0x2000
130 130 -926 800 71 0 1 [97, 119, 97] 8 0 0x8700 0x2000
131 130 -924 800 69 0 1 [122, 122, 122] 8 0 0x8500 0x2a00
132 130 -922 800 69 0 1 [122, 122, 122] 18446744073709551615 0 0x8500 0x2a00
133 130 -920 800 69 0 1 [122, 122, 122] 18446744073709551615 0 0x8500 0x2a00
134 130 -918 800 69 0 1 [122, 122, 122] 18446744073709551615 0 0x8500 0x2a00
122 131 -942 802 78 0 1 [97, 119, 97] 10 0 0x8e00 0x2000
123 131 -940 802 77 0 1 [97, 119, 97] 9 0 0x8d00 0x2000
124 131 -938 802 77 0 1 [97, 119, 97] 9 0 0x8d00 0x2000
125 131 -936 802 66 0 1 [134, 183, 131] 8 0 0x8200 0x4100
126 131 -934 802 62 1 1 [63, 118, 228] 8 0 0x7e00 0xd800
127 131 -932 802 62 1 1 [63, 118, 228] 8 0 0x7e00 0xd800
128 131 -930 802 62 1 1 [63, 118, 228] 8 0 0x7e00 0xd800
129 131 -928 802 62 1 1 [63, 118, 228] 8 0 0x7e00 0xd800
130 131 -926 802 66 0 1 [122, 122, 122] 8 0 0x8200 0x2a00
131 131 -924 802 68 0 1 [122, 122, 122] 8 0 0x8400 0x2a00
132 131 -922 802 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 131 -920 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 131 -918 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 132 -942 804 79 0 1 [97, 119, 97] 10 0 0x8f00 0x2000
123 132 -940 804 76 0 1 [97, 119, 97] 9 0 0x8c00 0x2000
124 132 -938 804 76 0 1 [97, 119, 97] 9 0 0x8c00 0x2000
125 132 -936 804 76 0 1 [97, 119, 97] 8 0 0x8c00 0x2000
126 132 -934 804 62 1 1 [63, 118, 228] 8 0 0x7e00 0xd800
127 132 -932 804 62 1 1 [63, 118, 228] 8 0 0x7e00 0xd800
128 132 -930 804 62 1 1 [63, 118, 228] 8 0 0x7e00 0xd800
129 132 -928 804 62 1 1 [63, 118, 228] 8 0 0x7e00 0xd800
130 132 -926 804 65 0 1 [122, 122, 122] 8 0 0x8100 0x2a00
131 132 -924 804 77 0 1 [97, 119, 97] 8 0 0x8d00 0x2000
132 132 -922 804 79 0 1 [97, 119, 97] 8 0 0x8f00 0x2000
133 132 -920 804 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 132 -918 804 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x2000
122 133 -942 806 78 0 1 [97, 119, 97] 10 0 0x8e00 0x2000
123 133 -940 806 69 0 1 [134, 183, 131] 9 0 0x8500 0x4100
124 133 -938 806 155 0 1 [134, 183, 131] 9 0 0xdb00 0x4100
125 133 -936 806 155 0 1 [134, 183, 131] 9 0 0xdb00 0x4100
126 133 -934 806 155 0 1 [134, 183, 131] 8 0 0xdb00 0x4100
127 133 -932 806 155 0 1 [134, 183, 131] 8 0 0xdb00 0x4100
128 133 -930 806 155 0 1 [134, 183, 131] 8 0 0xdb00 0x4100
129 133 -928 806 155 0 1 [134, 183, 131] 8 0 0xdb00 0x4100
130 133 -926 806 155 0 1 [134, 183, 131] 8 0 0xdb00 0x4100
131 133 -924 806 67 0 1 [122, 122, 122] 8 0 0x8300 0x2a00
132 133 -922 806 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 133 -920 806 69 0 1 [122, 122, 122] 18446744073709551615 0 0x8500 0x2a00
134 133 -918 806 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x2000
122 134 -942 808 79 0 1 [97, 119, 97] 10 0 0x8f00 0x2000
123 134 -940 808 68 0 1 [134, 183, 131] 9 0 0x8400 0x4100
124 134 -938 808 155 0 1 [134, 183, 131] 9 0 0xdb00 0x4100
125 134 -936 808 180 0 1 [134, 183, 131] 8 0 0xf400 0x4100
126 134 -934 808 180 0 1 [134, 183, 131] 8 0 0xf400 0x4100
127 134 -932 808 180 0 1 [134, 183, 131] 8 0 0xf400 0x4100
128 134 -930 808 180 0 1 [134, 183, 131] 8 0 0xf400 0x4100
129 134 -928 808 180 0 1 [134, 183, 131] 8 0 0xf400 0x4100
130 134 -926 808 155 0 1 [134, 183, 131] 8 0 0xdb00 0x4100
131 134 -924 808 67 0 1 [122, 122, 122] 8 0 0x8300 0x2a00
132 134 -922 808 68 0 1 [134, 183, 131] 18446744073709551615 1 0x8400 0x4100
133 134 -920 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 134 -918 808 69 0 1 [122, 122, 122] 18446744073709551615 0 0x8500 0x2a00
```
- edit channel: 0 linkage errors, 0 failed event dispatches (both must be 0)
- phase: 6 parity world boot (18:45:01)
- parity world: view-distance=8 (the voxy leg's server value)
- parity world: joined session 3 (18:45:24)
- parity world: scan populated (cells=65536; the march paints from these buffers)
- parity world tour: 13 vantages (production pack, vd 8, shared structures)
- coverage sweeps: 72 eye passes = 36 positions (12 az x r144/200 + 12 off-az x r172, y=160) x 2 looks (outward+inward), settle-by-laps >= 6 s (q stable 3x1s), deadline 12 s/pos, no shots — the v6.26.0 data-coverage lever + v6.26.6 lottery kill; probe rides parity/view_probe.md
- parity drain-settle TIMEOUT: queue 18301 -> 18301 > 11500 after 150s — continuing anyway; the queue did not drain without eye proximity (priority-lane evidence, the 01/05/10 dip attribution reads from the tour deltas)
- pre-tour pass done: 13 vantages x 10s (the tour now reads the second OR-merge lap; the rim/dip transport race reads from the tour deltas)
- **DAY GUARD:** 02_aerial_south still night after re-pin+reshoot (zenith < 130) — renamed night_02_aerial_south.png; the row pairs as missing, NOT as a renderer diff
- **DAY GUARD:** 03_aerial_north still night after re-pin+reshoot (zenith < 130) — renamed night_03_aerial_north.png; the row pairs as missing, NOT as a renderer diff
- **DAY GUARD:** 04_aerial_east still night after re-pin+reshoot (zenith < 130) — renamed night_04_aerial_east.png; the row pairs as missing, NOT as a renderer diff
- **DAY GUARD:** 06_ground_west still night after re-pin+reshoot (zenith < 130) — renamed night_06_ground_west.png; the row pairs as missing, NOT as a renderer diff
- **DAY GUARD:** 08_back_inside still night after re-pin+reshoot (zenith < 130) — renamed night_08_back_inside.png; the row pairs as missing, NOT as a renderer diff
- **DAY GUARD:** 09_after_mesh still night after re-pin+reshoot (zenith < 130) — renamed night_09_after_mesh.png; the row pairs as missing, NOT as a renderer diff
- parity world tour done: 13 shots, 6 night-guard renames in parity/ (the gate pairs voxy/<name>.png vs parity/<name>.png)
- phase: 7 parity tour done (19:08:15)
- sparse re-beat channel at tour end: streamed=157639 q=0 dropped_stale=94 dropped_undecodable=13395 (frac=8 boot; undecodable-dominant = the rim words die in their decodability window, stale-dominant = edit churn owns the cells)
- **FAIL:** parity world time-of-day uncontrollable: 6 vantage(s) night after re-pin+reshoot — the §7b noon-pin class is broken on this runner; the matrix is infrastructure-blocked, not a renderer verdict (DAY GUARD lines + the feed's time-audit.md carry the evidence)
### parity world stream counters (session 3, tail + maxima)
[c-LoD] stream: tick 13700 65536/65536 cells, lap 1294, cone 69960 zone 26065 dirty 15631 (q 18251) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 9432730c/2486e, edits +18082p/13711c/2886e drop 0, spawned 0 / failed 0, cam0 yaw -180 pitch 5, wish 0 forced 827, native 35136c/950ch fail 0 (3w q 0) helper on, trees 950ch/fail 0, wire bundle 2108011p/5226b fail 0 ~0 kB/s cache 21747/24576 h1639934 m468077 e446330, srbeat 157639s/0q drop 94stale/13395undec, ereb 102812s/1q, sweep 0r/95d
[19:08:12 INFO]: [c-LoD] stream: tick 13700 65536/65536 cells, lap 1294, cone 69960 zone 26065 dirty 15631 (q 18251) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 9432730c/2486e, edits +18082p/13711c/2886e drop 0, spawned 0 / failed 0, cam0 yaw -180 pitch 5, wish 0 forced 827, native 35136c/950ch fail 0 (3w q 0) helper on, trees 950ch/fail 0, wire bundle 2108011p/5226b fail 0 ~0 kB/s cache 21747/24576 h1639934 m468077 e446330, srbeat 157639s/0q drop 94stale/13395undec, ereb 102812s/1q, sweep 0r/95d
- parity stream max cone: 69960
- parity stream max zone: 26065
- parity stream max wire bundle: bundle
### parity world capture lens (debug chain on the §7b world)
lens_red=0 lens_black=0 lens_water=0 lens_dark=0 lens_zero=0 rot=none scan=0 carriers=0
- parity mesh probe @png_y=2: x306=(74,90,125) x307=(74,90,125) x308=(74,90,125) x309h=(74,90,125) x310c=(74,90,125)
- parity mesh probe @png_y=717: x306=(22,31,25) x307=(23,32,26) x308=(41,55,44) x309h=(41,54,44) x310c=(42,56,45)
- parity mesh engagement: magenta-ish=1504 of 921600 sampled px
- mask8: the un-fog engagement masks shot on the PARITY WORLD (vd 8) — parity_dbg8/01m|07m|10m
- mask8 01m: green=26304 red=0 blue=200480 dim=372864 (px, sampled /16)
- mask8 07m: green=9488 red=0 blue=149856 dim=10880 (px, sampled /16)
- mask8 10m: green=4128 red=0 blue=134656 dim=144352 (px, sampled /16)
- mesh probe @png_y=2: x306=(83,102,141) x307=(83,102,141) x308=(83,102,141) x309h=(83,102,141) x310c=(83,102,141)
- mesh probe @png_y=717: x306=(25,35,29) x307=(26,36,30) x308=(47,62,50) x309h=(47,61,50) x310c=(47,63,50)
- mesh engagement: magenta-ish=1504 of 921600 sampled px (the vanilla-path paint coverage)
- mask8 fog state @png_y=2: raw (300)=(104,109,119) (301)=(105,110,120)
- mask8 fog state decoded: envStart=832 envEnd=872 rdEnd=1904 fogColor=(105,110,120)
- mask8 fog state @png_y=717: raw (300)=(26,38,30) (301)=(28,40,32)
- mask8 fog state decoded: envStart=208 envEnd=304 rdEnd=480 fogColor=(28,40,32)
- buffer dump 10m: marked h=2048/2048 c=2048/2048; height windows r0<142:0 r1:2048 r2>=202:0 maxByte=178
- rim mask: scan populated (cells=65536; the state paint reads the same buffers the march reads)
- phase: 10 rim mask lane (19:13:34)
- rim mask: decode-state frames shot (rim_mask/00|01|12 — GREEN full decode / RED colorless / BLUE heightless; 00 = the leg-1 tower eye, 01 = the near-tower contrast, 12 = the tower re-read; the inherited 07/12 vantages sat 760 blocks outside the streamed disk and 01 sat INSIDE the marker column — their earlier verdicts were classifier noise, purged in v6.33.1)
- rim mask 00: green=560 red=160 blue=0 yellow=0 (px, sampled /16) -> DOMINANT GREEN: rim fully decoded — the fallback is the PAINT path (shade/fog/composite), not data
- rim mask 01: green=0 red=272 blue=1120 yellow=0 (px, sampled /16) -> DOMINANT BLUE: rim = HEIGHTS NEVER DECODED — a data-coverage class: the state paint cannot even see the cells
- rim mask 12: green=1376 red=160 blue=0 yellow=0 (px, sampled /16) -> DOMINANT GREEN: rim fully decoded — the fallback is the PAINT path (shade/fog/composite), not data

### boot / stream
```
[c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 143229 bytes, sha1 d25a1161c1d1c6b6f86482ec981b01c009043b33 — zero textures, zero world data, one push per join
[19:11:24 INFO]: [c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 143229 bytes, sha1 d25a1161c1d1c6b6f86482ec981b01c009043b33 — zero textures, zero world data, one push per join
[19:11:24 INFO]: [c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[c-LoD] stream: tick 1800 65536/65536 cells, lap 692, cone 21139 zone 1334 dirty 23 (q 9347) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1223079c/2553e, edits +21p/21c/7e drop 0, spawned 0 / failed 0, cam0 none, wish 357 forced 315, native 36837c/951ch fail 0 (3w q 0) helper on, trees 951ch/fail 0, wire bundle 286751p/431b fail 0 ~36 kB/s cache 14003/24576 h272748 m14003 e0, srbeat 14982s/862q drop 0stale/1321undec, ereb 1147s/7q, sweep 0r/84d
[19:14:29 INFO]: [c-LoD] stream: tick 1800 65536/65536 cells, lap 692, cone 21139 zone 1334 dirty 23 (q 9347) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1223079c/2553e, edits +21p/21c/7e drop 0, spawned 0 / failed 0, cam0 none, wish 357 forced 315, native 36837c/951ch fail 0 (3w q 0) helper on, trees 951ch/fail 0, wire bundle 286751p/431b fail 0 ~36 kB/s cache 14003/24576 h272748 m14003 e0, srbeat 14982s/862q drop 0stale/1321undec, ereb 1147s/7q, sweep 0r/84d
```
### sessions
```
[19:11:34 INFO]: System chat: Stepan joined the game
[19:14:28 INFO]: System chat: Stepan left the game
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
Horizon: UI bundle ready after 314 ms
Horizon: block sprites: 1301 layers at 64 px (largest 64 px, budget 256 MiB)
load core tables: 0.63s
Horizon renderer: pipelines ready (multi-draw on, occlusion culling off, GPU culling unavailable, GPU timer on)
Horizon renderer: acquiring the first frame
Horizon renderer: submitting the first frame
Horizon renderer: presenting the first frame
Horizon: loading screen shown after 735 ms
Horizon: loading the title artwork after 735 ms
Horizon renderer: llvmpipe (LLVM 21.1.8, 256 bits) (Vulkan, Cpu)
load pack tables: 0.76s
Horizon: core-ready after 767 ms
Horizon: handoff CPU resources prepared on worker in 76 ms
load items: 0.93s
Horizon: resources loaded in 1.0s; 35723 block states, 1658 items, 67 biomes; 3 worker threads
Horizon: rest-ready after 968 ms
Horizon renderer: rebuilt on the same device (51 shader modules, layouts and pipelines kept)
Horizon renderer: llvmpipe (LLVM 21.1.8, 256 bits) (Vulkan, Cpu)
Horizon renderer: rebuilt on the same device (51 shader modules, layouts and pipelines kept)
Horizon: world renderer ready after 1263 ms
Horizon: no sounds in the active resources; prepare the game again in the launcher (or run horizon-prepare sounds)
Horizon: no sound: audio off: HORIZON_NO_AUDIO is set
Horizon: waiting for world resources
Horizon: world resources joined after 0 ms
Horizon: startup handoff 262 ms (renderer 241 ms, start_game 15 ms)
Horizon: first frame after 421 ms
Horizon: worldgen data loaded in 479.633532ms
Horizon: block sprites: 1301 layers at 64 px (largest 64 px, budget 256 MiB)
Horizon renderer: llvmpipe (LLVM 21.1.8, 256 bits) (Vulkan, Cpu)
Horizon renderer: rebuilt on the same device (51 shader modules, layouts and pipelines kept)
Failed to load post effect clod:lod_terrain: pass 4: reads the target it writes
XIO:  fatal IO error 2 (No such file or directory) on X server ":99"
      after 10465 requests (425 known processed) with 0 events remaining.
--- exceptions (Realms/RTKit noise filtered) ---
(no client log)
```
### server log tail
```
[19:14:29 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:the_nether'
[19:14:29 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_nether'
[19:14:29 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_nether' in 0.00s
[19:14:29 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_nether'
[19:14:29 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_nether'
[19:14:29 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[19:14:29 INFO]: [ChunkHolderManager] Waiting 60s for chunk system to halt for world 'minecraft:the_end'
[19:14:29 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:the_end'
[19:14:29 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_end'
[19:14:29 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_end' in 0.00s
[19:14:29 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_end'
[19:14:29 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_end'
[19:14:29 INFO]: Waiting for all RegionFile I/O tasks to complete...
[19:14:29 INFO]: All RegionFile I/O tasks to complete
[19:14:29 INFO]: [MoonriseCommon] Awaiting termination of worker pool for up to 60s...
[19:14:29 INFO]: [MoonriseCommon] Awaiting termination of I/O pool for up to 60s...
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[c-LoD] engine tick alive: tick 1800
[c-LoD] EB: enter, native_gen=true
[19:14:29 INFO]: [c-LoD] engine tick alive: tick 1800
[c-LoD] stream: tick 1800 65536/65536 cells, lap 692, cone 21139 zone 1334 dirty 23 (q 9347) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1223079c/2553e, edits +21p/21c/7e drop 0, spawned 0 / failed 0, cam0 none, wish 357 forced 315, native 36837c/951ch fail 0 (3w q 0) helper on, trees 951ch/fail 0, wire bundle 286751p/431b fail 0 ~36 kB/s cache 14003/24576 h272748 m14003 e0, srbeat 14982s/862q drop 0stale/1321undec, ereb 1147s/7q, sweep 0r/84d
[19:14:29 INFO]: [c-LoD] stream: tick 1800 65536/65536 cells, lap 692, cone 21139 zone 1334 dirty 23 (q 9347) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1223079c/2553e, edits +21p/21c/7e drop 0, spawned 0 / failed 0, cam0 none, wish 357 forced 315, native 36837c/951ch fail 0 (3w q 0) helper on, trees 951ch/fail 0, wire bundle 286751p/431b fail 0 ~36 kB/s cache 14003/24576 h272748 m14003 e0, srbeat 14982s/862q drop 0stale/1321undec, ereb 1147s/7q, sweep 0r/84d
[launcher] server exited with code 143
```
### shots
```
-rw-r--r-- 1 runner runner 163341 Nov 18 18:36 11_h165_spark.png
-rw-r--r-- 1 runner runner 163297 Nov 18 18:36 11_h168_spark.png
-rw-r--r-- 1 runner runner 163407 Nov 18 18:36 11_h172_spark.png
-rw-r--r-- 1 runner runner 163387 Nov 18 18:36 11_h176_spark.png
-rw-r--r-- 1 runner runner 163334 Nov 18 18:36 11_h180_spark.png
-rw-r--r-- 1 runner runner 163492 Nov 18 18:36 11_h185_spark.png
-rw-r--r-- 1 runner runner 163090 Nov 18 18:36 12_after_edit_probe0.png
-rw-r--r-- 1 runner runner 168100 Nov 18 18:45 13_after_relog_probe0.png
-rw-r--r-- 1 runner runner 167990 Nov 18 18:44 14_leg2_baseline.png
-rw-r--r-- 1 runner runner 168410 Nov 18 18:44 14_leg2_burst_far.png
-rw-r--r-- 1 runner runner 168012 Nov 18 18:44 14_leg2_burst_greendust.png
-rw-r--r-- 1 runner runner 168317 Nov 18 18:44 14_leg2_burst_near.png
-rw-r--r-- 1 runner runner 155073 Nov 18 18:45 diff_after_relog.png
-rw-r--r-- 1 runner runner 168759 Nov 18 18:36 diff_live_edit.png
-rw-r--r-- 1 runner runner      0 Nov 18 18:32 ffmpeg.err
-rw-r--r-- 1 runner runner   6793 Nov 18 18:36 lens_12_after_edit_probe0.png
-rw-r--r-- 1 runner runner   7554 Nov 18 18:45 lens_13_after_relog_probe0.png
drwxr-xr-x 2 runner runner   4096 Nov 18 19:08 lens_parity
drwxr-xr-x 2 runner runner   4096 Nov 18 19:08 parity
drwxr-xr-x 2 runner runner   4096 Nov 18 18:39 parity_dbg
drwxr-xr-x 2 runner runner   4096 Nov 18 19:11 parity_dbg8
-rw-r--r-- 1 runner runner    469 Nov 18 18:45 probe_counts.txt
drwxr-xr-x 2 runner runner   4096 Nov 18 19:14 rim_mask
-rw-r--r-- 1 runner runner 100259 Nov 18 19:14 server_full.log
-rw-r--r-- 1 runner runner  45253 Nov 18 19:14 summary.md
```

**exit=1** failures=2 leg1=1 leg2=1
