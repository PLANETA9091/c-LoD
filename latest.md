- phase: 0 sanity (19:39:43)
### c-LoD visual run (-930,796, marker=red_concrete, vantage -1005,150 yaw 270)
- GL:     Vendor: Mesa (0xffffffff);    Device: llvmpipe (LLVM 21.1.8, 256 bits) (0xffffffff);OpenGL renderer string: llvmpipe (LLVM 21.1.8, 256 bits);OpenGL core profile version string: 4.5 (Core Profile) Mesa 26.0.8-1ubuntu0.3;
- boot + gates OK (19:40:11)
- phase: 1 server booted (19:40:11)
- sky-grid armed
- phase: 2 leg-1 client joined (19:40:38)
- joined session 1 (19:40:38)
- stream live: cells=65536
- phase: 3 scan populated (19:41:02)
- terrain range: 2 terraced mountains at (CX-4,CZ-20)/(CX-2,CZ+16), tops 185/190, grass terraces + stone caps, 66..82 m from the eye (inside the 83.5 m band, chunk edge 64 m)
- scan complete=1 (cells=cells=65536/65536)
- paint settled=1 (wire 442967p)
- baseline try 1: drift=3799 px
- baseline try 2: drift=0 px
- lod_silhouette_cols=31 (LOD paint ON SCREEN at 75 m if > 20 — the march-drawn marker column; vanilla cannot render there, chunk edge 64 m; #255 re-measures 49, the ZONE_FAR=48 regression #256 and the control read 0)
- 0 terrain-green px in the sky region (LOD MOUNTAINS on screen if > 5000; the range paints ColLow-green on low terraces via the fallback palette; flat-world baselines and the pack-free control read ~0)
- burst_close white px: 489 (renderer alive if > 50)
- burst_far white px: 430 (renderer alive in the marker zone if > 20)
- cut walk 40 m: 0 white px (> 20 = rendered)
- cut walk 60 m: 2 white px (> 20 = rendered)
- cut walk 100 m: 0 white px (> 20 = rendered)
- measured far-render cut: none m (G4 flap instrument; the decode proof needs >= 83 m from the y=151.6 eye)
- fill issued: 5x111x5 red_concrete at -930,796 (zone forceloaded ±48)
- phase: 4 marker filled (19:46:37)

### /clod-cells AFTER FILL (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 21 0 0x8400 0x3100
123 122 -940 784 71 0 1 [97, 119, 97] 20 0 0x8700 0x3100
124 122 -938 784 68 0 1 [97, 119, 97] 29 0 0x8400 0x3100
125 122 -936 784 62 1 1 [63, 118, 228] 29 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 28 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 28 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 27 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 26 0 0x8000 0x5c00
130 122 -926 784 66 0 1 [134, 183, 131] 26 0 0x8200 0x5c00
131 122 -924 784 75 0 1 [97, 119, 97] 23 0 0x8b00 0x3100
132 122 -922 784 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 122 -920 784 75 0 1 [97, 119, 97] 18446744073709551615 1 0x8b00 0x3100
134 122 -918 784 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x3100
122 123 -942 786 70 0 1 [97, 119, 97] 21 0 0x8600 0x3100
123 123 -940 786 65 0 1 [97, 119, 97] 20 0 0x8100 0x3100
124 123 -938 786 65 0 1 [97, 119, 97] 28 0 0x8100 0x3100
125 123 -936 786 62 1 1 [63, 118, 228] 29 0 0x7e00 0xd800
126 123 -934 786 62 1 1 [63, 118, 228] 28 0 0x7e00 0xd800
127 123 -932 786 62 1 1 [63, 118, 228] 28 0 0x7e00 0xd800
128 123 -930 786 62 1 1 [63, 118, 228] 27 0 0x7e00 0xd800
129 123 -928 786 73 0 1 [97, 119, 97] 27 0 0x8900 0x3100
130 123 -926 786 67 0 1 [134, 183, 131] 26 0 0x8300 0x5c00
131 123 -924 786 69 0 1 [134, 183, 131] 25 0 0x8500 0x5c00
132 123 -922 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 123 -920 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
134 123 -918 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
122 124 -942 788 72 0 1 [97, 119, 97] 21 0 0x8800 0x3100
123 124 -940 788 70 0 1 [97, 119, 97] 20 0 0x8600 0x3100
124 124 -938 788 68 0 1 [97, 119, 97] 28 0 0x8400 0x3100
125 124 -936 788 62 1 1 [63, 118, 228] 29 0 0x7e00 0xd800
126 124 -934 788 62 1 1 [63, 118, 228] 28 0 0x7e00 0xd800
127 124 -932 788 62 1 1 [63, 118, 228] 28 0 0x7e00 0xd800
128 124 -930 788 73 0 1 [97, 119, 97] 27 0 0x8900 0x3100
129 124 -928 788 76 0 1 [97, 119, 97] 27 0 0x8c00 0x3100
130 124 -926 788 73 0 1 [97, 119, 97] 18 0 0x8900 0x3100
131 124 -924 788 69 0 1 [134, 183, 131] 26 0 0x8500 0x5c00
132 124 -922 788 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 124 -920 788 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
134 124 -918 788 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x3100
122 125 -942 790 74 0 1 [97, 119, 97] 21 0 0x8a00 0x3100
123 125 -940 790 73 0 1 [97, 119, 97] 20 0 0x8900 0x3100
124 125 -938 790 71 0 1 [97, 119, 97] 28 0 0x8700 0x3100
125 125 -936 790 62 1 1 [63, 118, 228] 29 0 0x7e00 0xd800
126 125 -934 790 62 1 1 [63, 118, 228] 28 0 0x7e00 0xd800
127 125 -932 790 62 1 1 [63, 118, 228] 28 0 0x7e00 0xd800
128 125 -930 790 62 1 1 [63, 118, 228] 27 0 0x7e00 0xd800
129 125 -928 790 73 0 1 [97, 119, 97] 27 0 0x8900 0x3100
130 125 -926 790 68 0 1 [134, 183, 131] 26 0 0x8400 0x5c00
131 125 -924 790 69 0 1 [134, 183, 131] 26 0 0x8500 0x5c00
132 125 -922 790 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
133 125 -920 790 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
134 125 -918 790 70 0 1 [134, 183, 131] 18446744073709551615 1 0x8600 0x5c00
122 126 -942 792 74 0 1 [97, 119, 97] 21 0 0x8a00 0x3100
123 126 -940 792 73 0 1 [97, 119, 97] 20 0 0x8900 0x3100
124 126 -938 792 68 0 1 [97, 119, 97] 28 0 0x8400 0x3100
125 126 -936 792 62 1 1 [63, 118, 228] 29 0 0x7e00 0xd800
126 126 -934 792 62 1 1 [63, 118, 228] 28 0 0x7e00 0xd800
127 126 -932 792 62 1 1 [63, 118, 228] 28 0 0x7e00 0xd800
128 126 -930 792 62 1 1 [63, 118, 228] 27 0 0x7e00 0xd800
129 126 -928 792 67 0 1 [134, 183, 131] 27 0 0x8300 0x5c00
130 126 -926 792 74 0 1 [97, 119, 97] 18 0 0x8a00 0x3100
131 126 -924 792 77 0 1 [97, 119, 97] 17 0 0x8d00 0x3100
132 126 -922 792 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x3100
133 126 -920 792 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3800
134 126 -918 792 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x8800
122 127 -942 794 67 0 1 [134, 183, 131] 30 0 0x8300 0x5c00
123 127 -940 794 68 0 1 [134, 183, 131] 30 0 0x8400 0x5c00
124 127 -938 794 65 0 1 [134, 183, 131] 29 0 0x8100 0x5c00
125 127 -936 794 62 1 1 [63, 118, 228] 29 0 0x7e00 0xd800
126 127 -934 794 62 1 1 [63, 118, 228] 28 0 0x7e00 0xd800
127 127 -932 794 62 1 1 [63, 118, 228] 28 0 0x7e00 0xd800
128 127 -930 794 62 1 1 [63, 118, 228] 27 0 0x7e00 0xd800
129 127 -928 794 66 0 1 [134, 183, 131] 27 0 0x8200 0x5c00
130 127 -926 794 74 0 1 [97, 119, 97] 18 0 0x8a00 0x3100
131 127 -924 794 77 0 1 [97, 119, 97] 17 0 0x8d00 0x3100
132 127 -922 794 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x3100
133 127 -920 794 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3800
134 127 -918 794 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x8800
122 128 -942 796 68 0 1 [134, 183, 131] 30 0 0x8400 0x5c00
123 128 -940 796 72 0 1 [97, 119, 97] 20 0 0x8800 0x3100
124 128 -938 796 75 0 1 [97, 119, 97] 28 0 0x8b00 0x3100
125 128 -936 796 72 0 1 [97, 119, 97] 29 0 0x8800 0x3100
126 128 -934 796 62 1 1 [63, 118, 228] 28 0 0x7e00 0xd800
127 128 -932 796 62 1 1 [63, 118, 228] 28 0 0x7e00 0xd800
128 128 -930 796 62 1 1 [63, 118, 228] 27 0 0x7e00 0xd800
129 128 -928 796 66 0 1 [134, 183, 131] 27 0 0x8200 0x5c00
130 128 -926 796 68 0 1 [134, 183, 131] 26 0 0x8400 0x5c00
131 128 -924 796 74 0 1 [97, 119, 97] 17 0 0x8a00 0x3100
132 128 -922 796 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 128 -920 796 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x3100
134 128 -918 796 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
122 129 -942 798 69 0 1 [134, 183, 131] 30 0 0x8500 0x5c00
123 129 -940 798 75 0 1 [97, 119, 97] 20 0 0x8b00 0x3100
124 129 -938 798 78 0 1 [97, 119, 97] 31 0 0x8e00 0x3100
125 129 -936 798 75 0 1 [97, 119, 97] 29 0 0x8b00 0x3100
126 129 -934 798 62 1 1 [63, 118, 228] 28 0 0x7e00 0xd800
127 129 -932 798 62 1 1 [63, 118, 228] 28 0 0x7e00 0xd800
128 129 -930 798 62 1 1 [63, 118, 228] 27 0 0x7e00 0xd800
129 129 -928 798 71 0 1 [97, 119, 97] 27 0 0x8700 0x3100
130 129 -926 798 71 0 1 [97, 119, 97] 18 0 0x8700 0x3100
131 129 -924 798 69 0 1 [134, 183, 131] 26 0 0x8500 0x5c00
132 129 -922 798 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x3100
133 129 -920 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
134 129 -918 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
122 130 -942 800 70 0 1 [134, 183, 131] 30 0 0x8600 0x5c00
123 130 -940 800 76 0 1 [97, 119, 97] 20 0 0x8c00 0x3100
124 130 -938 800 76 0 1 [97, 119, 97] 28 0 0x8c00 0x3100
125 130 -936 800 72 0 1 [97, 119, 97] 29 0 0x8800 0x3100
126 130 -934 800 62 1 1 [63, 118, 228] 28 0 0x7e00 0xd800
127 130 -932 800 62 1 1 [63, 118, 228] 28 0 0x7e00 0xd800
128 130 -930 800 62 1 1 [63, 118, 228] 27 0 0x7e00 0xd800
129 130 -928 800 71 0 1 [97, 119, 97] 27 0 0x8700 0x3100
130 130 -926 800 71 0 1 [97, 119, 97] 18 0 0x8700 0x3100
131 130 -924 800 69 0 1 [134, 183, 131] 26 0 0x8500 0x5c00
132 130 -922 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
133 130 -920 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
134 130 -918 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
122 131 -942 802 78 0 1 [97, 119, 97] 22 0 0x8e00 0x3100
123 131 -940 802 77 0 1 [97, 119, 97] 20 0 0x8d00 0x3100
124 131 -938 802 77 0 1 [97, 119, 97] 28 0 0x8d00 0x3100
125 131 -936 802 66 0 1 [134, 183, 131] 29 0 0x8200 0x5c00
126 131 -934 802 62 1 1 [63, 118, 228] 28 0 0x7e00 0xd800
127 131 -932 802 62 1 1 [63, 118, 228] 28 0 0x7e00 0xd800
128 131 -930 802 62 1 1 [63, 118, 228] 27 0 0x7e00 0xd800
129 131 -928 802 62 1 1 [63, 118, 228] 27 0 0x7e00 0xd800
130 131 -926 802 66 0 1 [134, 183, 131] 26 0 0x8200 0x5c00
131 131 -924 802 68 0 1 [134, 183, 131] 26 0 0x8400 0x5c00
132 131 -922 802 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 131 -920 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
134 131 -918 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
122 132 -942 804 79 0 1 [97, 119, 97] 22 0 0x8f00 0x3100
123 132 -940 804 76 0 1 [97, 119, 97] 20 0 0x8c00 0x3100
124 132 -938 804 76 0 1 [97, 119, 97] 28 0 0x8c00 0x3100
125 132 -936 804 76 0 1 [97, 119, 97] 29 0 0x8c00 0x3100
126 132 -934 804 62 1 1 [63, 118, 228] 28 0 0x7e00 0xd800
127 132 -932 804 62 1 1 [63, 118, 228] 28 0 0x7e00 0xd800
128 132 -930 804 62 1 1 [63, 118, 228] 27 0 0x7e00 0xd800
129 132 -928 804 62 1 1 [63, 118, 228] 27 0 0x7e00 0xd800
130 132 -926 804 65 0 1 [134, 183, 131] 26 0 0x8100 0x5c00
131 132 -924 804 77 0 1 [97, 119, 97] 17 0 0x8d00 0x3100
132 132 -922 804 79 0 1 [97, 119, 97] 18 0 0x8f00 0x3100
133 132 -920 804 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
134 132 -918 804 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x3100
122 133 -942 806 78 0 1 [97, 119, 97] 22 0 0x8e00 0x3100
123 133 -940 806 69 0 1 [134, 183, 131] 30 0 0x8500 0x5c00
124 133 -938 806 155 0 1 [134, 183, 131] 30 0 0xdb00 0x5c00
125 133 -936 806 155 0 1 [134, 183, 131] 30 0 0xdb00 0x5c00
126 133 -934 806 155 0 1 [134, 183, 131] 29 0 0xdb00 0x5c00
127 133 -932 806 155 0 1 [134, 183, 131] 29 0 0xdb00 0x5c00
128 133 -930 806 155 0 1 [134, 183, 131] 28 0 0xdb00 0x5c00
129 133 -928 806 155 0 1 [134, 183, 131] 28 0 0xdb00 0x5c00
130 133 -926 806 155 0 1 [134, 183, 131] 27 0 0xdb00 0x5c00
131 133 -924 806 67 0 1 [134, 183, 131] 25 0 0x8300 0x5c00
132 133 -922 806 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 133 -920 806 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
134 133 -918 806 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x3100
122 134 -942 808 79 0 1 [97, 119, 97] 22 0 0x8f00 0x3100
123 134 -940 808 68 0 1 [134, 183, 131] 30 0 0x8400 0x5c00
124 134 -938 808 155 0 1 [134, 183, 131] 30 0 0xdb00 0x5c00
125 134 -936 808 180 0 1 [134, 183, 131] 18446744073709551615 1 0xf400 0x5c00
126 134 -934 808 180 0 1 [134, 183, 131] 18446744073709551615 1 0xf400 0x5c00
127 134 -932 808 180 0 1 [134, 183, 131] 18446744073709551615 1 0xf400 0x5c00
128 134 -930 808 180 0 1 [134, 183, 131] 18446744073709551615 1 0xf400 0x5c00
129 134 -928 808 180 0 1 [134, 183, 131] 3129 1 0xf400 0x5c00
130 134 -926 808 155 0 1 [134, 183, 131] 27 0 0xdb00 0x5c00
131 134 -924 808 67 0 1 [134, 183, 131] 25 0 0x8300 0x5c00
132 134 -922 808 68 0 1 [134, 183, 131] 18446744073709551615 1 0x8400 0x5c00
133 134 -920 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
134 134 -918 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
```
### stream diag after fill:
[c-LoD] stream: tick 3800 65536/65536 cells, lap 1260, cone 17005 zone 1372 dirty 865 (q 12417) ewin 194 rbeat 9p/1 rearm 0c/0t rescan 941106c/1554e, edits +4742p/3593c/624e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 285 forced 227, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 567068p/2082b fail 0 ~57 kB/s cache 13069/24576 h553999 m13069 e0, srbeat 51918s/9q drop 32stale/0undec, ereb 45267s/10q
[19:46:42 INFO]: [c-LoD] stream: tick 3800 65536/65536 cells, lap 1260, cone 17005 zone 1372 dirty 865 (q 12417) ewin 194 rbeat 9p/1 rearm 0c/0t rescan 941106c/1554e, edits +4742p/3593c/624e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 285 forced 227, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 567068p/2082b fail 0 ~57 kB/s cache 13069/24576 h553999 m13069 e0, srbeat 51918s/9q drop 32stale/0undec, ereb 45267s/10q
- carrier ladder y=155: 14800 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=165: 14800 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=168: 14800 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=172: 14800 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=176: 14800 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=180: 14800 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=185: 14800 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- probe 0: red=1 paint=4722 lod_silhouette_cols=31 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 1: red=1 paint=4722 lod_silhouette_cols=31 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 2: red=1 paint=5820 lod_silhouette_cols=30 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 3: red=1 paint=5770 lod_silhouette_cols=30 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 4: red=1 paint=5819 lod_silhouette_cols=30 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 5: red=1 paint=5669 lod_silhouette_cols=30 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 6: red=1 paint=5859 lod_silhouette_cols=30 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 7: red=1 paint=5760 lod_silhouette_cols=30 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 8: red=1 paint=5561 lod_silhouette_cols=30 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 9: red=1 paint=5539 lod_silhouette_cols=31 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 10: red=1 paint=5539 lod_silhouette_cols=31 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 11: red=1 paint=5539 lod_silhouette_cols=31 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 12: red=1 paint=5539 lod_silhouette_cols=31 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 13: red=1 paint=5900 lod_silhouette_cols=30 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 14: red=1 paint=5697 lod_silhouette_cols=30 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 15: red=1 paint=5799 lod_silhouette_cols=30 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 16: red=1 paint=5619 lod_silhouette_cols=31 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 17: red=1 paint=6035 lod_silhouette_cols=31 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 18: red=1 paint=5982 lod_silhouette_cols=30 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 19: red=1 paint=6181 lod_silhouette_cols=30 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 20: red=1 paint=6207 lod_silhouette_cols=30 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 21: red=1 paint=6159 lod_silhouette_cols=30 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 22: red=1 paint=6134 lod_silhouette_cols=30 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 23: red=1 paint=6020 lod_silhouette_cols=30 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 24: red=1 paint=6430 lod_silhouette_cols=30 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 25: red=1 paint=6381 lod_silhouette_cols=30 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 26: red=1 paint=8306 lod_silhouette_cols=30 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 27: red=1 paint=12075 lod_silhouette_cols=30 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 28: red=1 paint=12122 lod_silhouette_cols=30 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 29: red=1 paint=12095 lod_silhouette_cols=30 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 30: red=1 paint=13062 lod_silhouette_cols=30 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 31: red=1 paint=18605 lod_silhouette_cols=31 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 32: red=1 paint=18811 lod_silhouette_cols=31 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 33: red=1 paint=18594 lod_silhouette_cols=31 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 34: red=1 paint=18720 lod_silhouette_cols=31 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 35: red=1 paint=18614 lod_silhouette_cols=31 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- phase: 5 leg-1 probes done (19:52:41)
- **FAIL:** leg-1 never saw the marker in 24 probes (red < 60 everywhere) — the edit never reached the LOD paint; this failure was SILENT before v6.31.1
### stream diag after probes:
[19:52:23 INFO]: [c-LoD] stream: tick 7200 65536/65536 cells, lap 3807, cone 68904 zone 1527 dirty 906 (q 12423) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 2039794c/1620e, edits +6040p/4483c/868e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 285 forced 227, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 1128530p/3613b fail 0 ~122 kB/s cache 13082/24576 h1115448 m13082 e0, srbeat 70415s/40q drop 38stale/0undec, ereb 46364s/0q
[c-LoD] stream: tick 7300 65536/65536 cells, lap 3907, cone 71182 zone 1527 dirty 906 (q 12423) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 2077365c/1620e, edits +6041p/4484c/869e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 285 forced 227, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 1152140p/3672b fail 0 ~124 kB/s cache 13082/24576 h1139058 m13082 e0, srbeat 71097s/24q drop 38stale/0undec, ereb 46364s/0q
[19:52:33 INFO]: [c-LoD] stream: tick 7300 65536/65536 cells, lap 3907, cone 71182 zone 1527 dirty 906 (q 12423) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 2077365c/1620e, edits +6041p/4484c/869e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 285 forced 227, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 1152140p/3672b fail 0 ~124 kB/s cache 13082/24576 h1139058 m13082 e0, srbeat 71097s/24q drop 38stale/0undec, ereb 46364s/0q
- parity debug tour: 13 vantages (session-1 client, DEBUG pack — evidence only, the gate pairs the §7b world session)
- parity debug tour done: 13 shots in parity_dbg/ (the §7b world session feeds the gate)
- joined session 2 (19:55:36)
- leg-2 baseline scene px (x<940): 24065 (0 = the relog client paints no world at all)
- leg-2 renderer probe NEAR (20 m) white px: 0 raw / 0 changed-vs-baseline (> 20 = the client renders particles)
- leg-2 renderer probe FAR (80 m, marker zone) white px: 449 raw / 449 changed-vs-baseline (> 20 = no distance cut; ~0 with NEAR alive = particles beyond the client chunk radius do not render)
- leg-2 carrier probe green dust (80 m): changed-green px 0 / total changed 13092 vs baseline (> 100 green = a stable green carrier exists; total > 0 with green ~0 = the dust renders but fog washes it — a ruler problem, not a carrier problem)
- phase: 8 leg-2 relog probes (19:58:28)
- relog probe 0: red=0 scene=34281 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 1: red=0 scene=34107 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 2: red=0 scene=36954 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 3: red=0 scene=36827 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 4: red=0 scene=37504 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 5: red=0 scene=39460 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 6: red=0 scene=39644 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 7: red=0 scene=39656 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 8: red=0 scene=39747 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 9: red=0 scene=39688 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 10: red=0 scene=39466 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 11: red=0 scene=39794 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 12: red=0 scene=39947 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 13: red=0 scene=40754 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 14: red=0 scene=40515 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 15: red=0 scene=40695 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 16: red=0 scene=40475 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 17: red=0 scene=40530 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 18: red=0 scene=40590 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 19: red=0 scene=40907 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- **FAIL:** marker lost after relog — stream state is not persistent/deterministic
- phase: 9 leg-2 done (20:01:32)

### client log forensics (pipeline/particle/atlas — v6.8.2)
```
java.lang.IllegalStateException: Failed to open OpenAL device
	at net.minecraft.client.sounds.SoundEngine.reload(SoundEngine.java:110)
	at net.minecraft.server.packs.resources.SimplePreparableReloadListener.lambda$reload$1(SimplePreparableReloadListener.java:15)
	at net.minecraft.server.packs.resources.SimpleReloadInstance.lambda$prepareTasks$3(SimpleReloadInstance.java:59)
[19:55:32] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[19:55:32] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[19:55:32] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[19:55:32] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[19:55:32] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[19:55:32] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[19:55:32] [Render thread/INFO]: Created: 2048x2048x4 minecraft:textures/atlas/blocks.png-atlas
[19:55:33] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[19:55:33] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[19:55:33] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[19:55:33] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[19:55:33] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
[19:55:40] [Render thread/INFO]: Reloading ResourceManager: vanilla, vanilla, server/00000000/6b1f2f7e-9c0d-4a4b-8b0a-b10d1b4ab100
ALSA lib confmisc.c:855:(parse_card) [error.core] cannot find card '0'
ALSA lib conf.c:5211:(_snd_config_evaluate) [error.core] function snd_func_card_inum returned error: No such file or directory
ALSA lib confmisc.c:422:(snd_func_concat) [error.core] error evaluating strings
ALSA lib conf.c:5211:(_snd_config_evaluate) [error.core] function snd_func_concat returned error: No such file or directory
ALSA lib confmisc.c:1342:(snd_func_refer) [error.core] error evaluating name
ALSA lib conf.c:5211:(_snd_config_evaluate) [error.core] function snd_func_refer returned error: No such file or directory
ALSA lib conf.c:5734:(snd_config_expand) [error.core] Evaluate error: No such file or directory
ALSA lib pcm.c:2722:(snd_pcm_open_noupdate) [error.pcm] Unknown PCM default
ALSA lib confmisc.c:855:(parse_card) [error.core] cannot find card '0'
ALSA lib conf.c:5211:(_snd_config_evaluate) [error.core] function snd_func_card_inum returned error: No such file or directory
ALSA lib confmisc.c:422:(snd_func_concat) [error.core] error evaluating strings
ALSA lib conf.c:5211:(_snd_config_evaluate) [error.core] function snd_func_concat returned error: No such file or directory
ALSA lib confmisc.c:1342:(snd_func_refer) [error.core] error evaluating name
ALSA lib conf.c:5211:(_snd_config_evaluate) [error.core] function snd_func_refer returned error: No such file or directory
ALSA lib conf.c:5734:(snd_config_expand) [error.core] Evaluate error: No such file or directory
ALSA lib pcm.c:2722:(snd_pcm_open_noupdate) [error.pcm] Unknown PCM default
[19:55:42] [Render thread/ERROR]: Error starting SoundSystem. Turning off sounds & music
java.lang.IllegalStateException: Failed to open OpenAL device
	at net.minecraft.client.sounds.SoundEngine.reload(SoundEngine.java:110)
	at net.minecraft.server.packs.resources.SimplePreparableReloadListener.lambda$reload$1(SimplePreparableReloadListener.java:15)
	at net.minecraft.server.packs.resources.SimpleReloadInstance.lambda$prepareTasks$3(SimpleReloadInstance.java:59)
[19:55:42] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[19:55:42] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[19:55:42] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[19:55:42] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[19:55:42] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[19:55:42] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[19:55:42] [Render thread/INFO]: Created: 2048x2048x4 minecraft:textures/atlas/blocks.png-atlas
[19:55:42] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[19:55:42] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[19:55:42] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[19:55:42] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[19:55:42] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
```

### /clod-cells AFTER PROBES (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 35 0 0x8400 0x3100
123 122 -940 784 71 0 1 [97, 119, 97] 36 0 0x8700 0x3100
124 122 -938 784 68 0 1 [97, 119, 97] 36 0 0x8400 0x3100
125 122 -936 784 62 1 1 [63, 118, 228] 18 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 18 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 17 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 17 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 16 0 0x8000 0x5c00
130 122 -926 784 66 0 1 [134, 183, 131] 16 0 0x8200 0x5c00
131 122 -924 784 75 0 1 [97, 119, 97] 22 0 0x8b00 0x3100
132 122 -922 784 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 122 -920 784 75 0 1 [97, 119, 97] 18446744073709551615 1 0x8b00 0x3100
134 122 -918 784 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x3100
122 123 -942 786 70 0 1 [97, 119, 97] 40 0 0x8600 0x3100
123 123 -940 786 65 0 1 [97, 119, 97] 40 0 0x8100 0x3100
124 123 -938 786 65 0 1 [97, 119, 97] 40 0 0x8100 0x3100
125 123 -936 786 62 1 1 [63, 118, 228] 18 0 0x7e00 0xd800
126 123 -934 786 62 1 1 [63, 118, 228] 18 0 0x7e00 0xd800
127 123 -932 786 62 1 1 [63, 118, 228] 17 0 0x7e00 0xd800
128 123 -930 786 62 1 1 [63, 118, 228] 17 0 0x7e00 0xd800
129 123 -928 786 73 0 1 [97, 119, 97] 40 0 0x8900 0x3100
130 123 -926 786 67 0 1 [134, 183, 131] 16 0 0x8300 0x5c00
131 123 -924 786 69 0 1 [134, 183, 131] 16 0 0x8500 0x5c00
132 123 -922 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 123 -920 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
134 123 -918 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
122 124 -942 788 72 0 1 [97, 119, 97] 40 0 0x8800 0x3100
123 124 -940 788 70 0 1 [97, 119, 97] 40 0 0x8600 0x3100
124 124 -938 788 68 0 1 [97, 119, 97] 40 0 0x8400 0x3100
125 124 -936 788 62 1 1 [63, 118, 228] 18 0 0x7e00 0xd800
126 124 -934 788 62 1 1 [63, 118, 228] 18 0 0x7e00 0xd800
127 124 -932 788 62 1 1 [63, 118, 228] 17 0 0x7e00 0xd800
128 124 -930 788 73 0 1 [97, 119, 97] 40 0 0x8900 0x3100
129 124 -928 788 76 0 1 [97, 119, 97] 40 0 0x8c00 0x3100
130 124 -926 788 73 0 1 [97, 119, 97] 40 0 0x8900 0x3100
131 124 -924 788 69 0 1 [134, 183, 131] 16 0 0x8500 0x5c00
132 124 -922 788 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 124 -920 788 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
134 124 -918 788 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x3100
122 125 -942 790 74 0 1 [97, 119, 97] 42 0 0x8a00 0x3100
123 125 -940 790 73 0 1 [97, 119, 97] 43 0 0x8900 0x3100
124 125 -938 790 71 0 1 [97, 119, 97] 43 0 0x8700 0x3100
125 125 -936 790 62 1 1 [63, 118, 228] 18 0 0x7e00 0xd800
126 125 -934 790 62 1 1 [63, 118, 228] 18 0 0x7e00 0xd800
127 125 -932 790 62 1 1 [63, 118, 228] 17 0 0x7e00 0xd800
128 125 -930 790 62 1 1 [63, 118, 228] 17 0 0x7e00 0xd800
129 125 -928 790 73 0 1 [97, 119, 97] 43 0 0x8900 0x3100
130 125 -926 790 68 0 1 [134, 183, 131] 16 0 0x8400 0x5c00
131 125 -924 790 69 0 1 [134, 183, 131] 16 0 0x8500 0x5c00
132 125 -922 790 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
133 125 -920 790 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
134 125 -918 790 70 0 1 [134, 183, 131] 18446744073709551615 1 0x8600 0x5c00
122 126 -942 792 74 0 1 [97, 119, 97] 42 0 0x8a00 0x3100
123 126 -940 792 73 0 1 [97, 119, 97] 42 0 0x8900 0x3100
124 126 -938 792 68 0 1 [97, 119, 97] 43 0 0x8400 0x3100
125 126 -936 792 62 1 1 [63, 118, 228] 18 0 0x7e00 0xd800
126 126 -934 792 62 1 1 [63, 118, 228] 18 0 0x7e00 0xd800
127 126 -932 792 62 1 1 [63, 118, 228] 18 0 0x7e00 0xd800
128 126 -930 792 62 1 1 [63, 118, 228] 17 0 0x7e00 0xd800
129 126 -928 792 67 0 1 [134, 183, 131] 17 0 0x8300 0x5c00
130 126 -926 792 74 0 1 [97, 119, 97] 43 0 0x8a00 0x3100
131 126 -924 792 77 0 1 [97, 119, 97] 43 0 0x8d00 0x3100
132 126 -922 792 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x3100
133 126 -920 792 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3800
134 126 -918 792 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x8800
122 127 -942 794 67 0 1 [134, 183, 131] 20 0 0x8300 0x5c00
123 127 -940 794 68 0 1 [134, 183, 131] 19 0 0x8400 0x5c00
124 127 -938 794 65 0 1 [134, 183, 131] 19 0 0x8100 0x5c00
125 127 -936 794 62 1 1 [63, 118, 228] 18 0 0x7e00 0xd800
126 127 -934 794 62 1 1 [63, 118, 228] 18 0 0x7e00 0xd800
127 127 -932 794 170 0 1 [142, 33, 33] 43 0 0xea00 0x0200
128 127 -930 794 170 0 1 [142, 33, 33] 43 0 0xea00 0x0200
129 127 -928 794 170 0 1 [142, 33, 33] 43 0 0xea00 0x0200
130 127 -926 794 74 0 1 [97, 119, 97] 43 0 0x8a00 0x3100
131 127 -924 794 77 0 1 [97, 119, 97] 43 0 0x8d00 0x3100
132 127 -922 794 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x3100
133 127 -920 794 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3800
134 127 -918 794 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x8800
122 128 -942 796 68 0 1 [134, 183, 131] 20 0 0x8400 0x5c00
123 128 -940 796 72 0 1 [97, 119, 97] 42 0 0x8800 0x3100
124 128 -938 796 75 0 1 [97, 119, 97] 43 0 0x8b00 0x3100
125 128 -936 796 72 0 1 [97, 119, 97] 43 0 0x8800 0x3100
126 128 -934 796 62 1 1 [63, 118, 228] 18 0 0x7e00 0xd800
127 128 -932 796 170 0 1 [142, 33, 33] 43 0 0xea00 0x0200
128 128 -930 796 170 0 1 [142, 33, 33] 43 0 0xea00 0x0200
129 128 -928 796 170 0 1 [142, 33, 33] 43 0 0xea00 0x0200
130 128 -926 796 68 0 1 [134, 183, 131] 16 0 0x8400 0x5c00
131 128 -924 796 74 0 1 [97, 119, 97] 30 0 0x8a00 0x3100
132 128 -922 796 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 128 -920 796 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x3100
134 128 -918 796 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
122 129 -942 798 69 0 1 [134, 183, 131] 20 0 0x8500 0x5c00
123 129 -940 798 75 0 1 [97, 119, 97] 30 0 0x8b00 0x3100
124 129 -938 798 78 0 1 [97, 119, 97] 30 0 0x8e00 0x3100
125 129 -936 798 75 0 1 [97, 119, 97] 30 0 0x8b00 0x3100
126 129 -934 798 62 1 1 [63, 118, 228] 18 0 0x7e00 0xd800
127 129 -932 798 170 0 1 [142, 33, 33] 30 0 0xea00 0x0200
128 129 -930 798 170 0 1 [142, 33, 33] 30 0 0xea00 0x0200
129 129 -928 798 170 0 1 [142, 33, 33] 30 0 0xea00 0x0200
130 129 -926 798 71 0 1 [97, 119, 97] 30 0 0x8700 0x3100
131 129 -924 798 69 0 1 [134, 183, 131] 16 0 0x8500 0x5c00
132 129 -922 798 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x3100
133 129 -920 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
134 129 -918 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
122 130 -942 800 70 0 1 [134, 183, 131] 20 0 0x8600 0x5c00
123 130 -940 800 76 0 1 [97, 119, 97] 30 0 0x8c00 0x3100
124 130 -938 800 76 0 1 [97, 119, 97] 30 0 0x8c00 0x3100
125 130 -936 800 72 0 1 [97, 119, 97] 29 0 0x8800 0x3100
126 130 -934 800 62 1 1 [63, 118, 228] 18 0 0x7e00 0xd800
127 130 -932 800 62 1 1 [63, 118, 228] 18 0 0x7e00 0xd800
128 130 -930 800 62 1 1 [63, 118, 228] 17 0 0x7e00 0xd800
129 130 -928 800 71 0 1 [97, 119, 97] 29 0 0x8700 0x3100
130 130 -926 800 71 0 1 [97, 119, 97] 29 0 0x8700 0x3100
131 130 -924 800 69 0 1 [134, 183, 131] 16 0 0x8500 0x5c00
132 130 -922 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
133 130 -920 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
134 130 -918 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
122 131 -942 802 78 0 1 [97, 119, 97] 29 0 0x8e00 0x3100
123 131 -940 802 77 0 1 [97, 119, 97] 29 0 0x8d00 0x3100
124 131 -938 802 77 0 1 [97, 119, 97] 29 0 0x8d00 0x3100
125 131 -936 802 66 0 1 [134, 183, 131] 18 0 0x8200 0x5c00
126 131 -934 802 62 1 1 [63, 118, 228] 18 0 0x7e00 0xd800
127 131 -932 802 62 1 1 [63, 118, 228] 17 0 0x7e00 0xd800
128 131 -930 802 62 1 1 [63, 118, 228] 17 0 0x7e00 0xd800
129 131 -928 802 62 1 1 [63, 118, 228] 16 0 0x7e00 0xd800
130 131 -926 802 66 0 1 [134, 183, 131] 16 0 0x8200 0x5c00
131 131 -924 802 68 0 1 [134, 183, 131] 16 0 0x8400 0x5c00
132 131 -922 802 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 131 -920 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
134 131 -918 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
122 132 -942 804 79 0 1 [97, 119, 97] 29 0 0x8f00 0x3100
123 132 -940 804 76 0 1 [97, 119, 97] 29 0 0x8c00 0x3100
124 132 -938 804 76 0 1 [97, 119, 97] 29 0 0x8c00 0x3100
125 132 -936 804 76 0 1 [97, 119, 97] 29 0 0x8c00 0x3100
126 132 -934 804 62 1 1 [63, 118, 228] 18 0 0x7e00 0xd800
127 132 -932 804 62 1 1 [63, 118, 228] 17 0 0x7e00 0xd800
128 132 -930 804 62 1 1 [63, 118, 228] 17 0 0x7e00 0xd800
129 132 -928 804 62 1 1 [63, 118, 228] 16 0 0x7e00 0xd800
130 132 -926 804 65 0 1 [134, 183, 131] 16 0 0x8100 0x5c00
131 132 -924 804 77 0 1 [97, 119, 97] 29 0 0x8d00 0x3100
132 132 -922 804 79 0 1 [97, 119, 97] 29 0 0x8f00 0x3100
133 132 -920 804 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
134 132 -918 804 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x3100
122 133 -942 806 78 0 1 [97, 119, 97] 29 0 0x8e00 0x3100
123 133 -940 806 69 0 1 [134, 183, 131] 19 0 0x8500 0x5c00
124 133 -938 806 155 0 1 [134, 183, 131] 19 0 0xdb00 0x5c00
125 133 -936 806 155 0 1 [134, 183, 131] 19 0 0xdb00 0x5c00
126 133 -934 806 155 0 1 [134, 183, 131] 19 0 0xdb00 0x5c00
127 133 -932 806 155 0 1 [134, 183, 131] 18 0 0xdb00 0x5c00
128 133 -930 806 155 0 1 [134, 183, 131] 18 0 0xdb00 0x5c00
129 133 -928 806 155 0 1 [134, 183, 131] 17 0 0xdb00 0x5c00
130 133 -926 806 155 0 1 [134, 183, 131] 17 0 0xdb00 0x5c00
131 133 -924 806 67 0 1 [134, 183, 131] 15 0 0x8300 0x5c00
132 133 -922 806 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 133 -920 806 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
134 133 -918 806 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x3100
122 134 -942 808 79 0 1 [97, 119, 97] 29 0 0x8f00 0x3100
123 134 -940 808 68 0 1 [134, 183, 131] 19 0 0x8400 0x5c00
124 134 -938 808 155 0 1 [134, 183, 131] 19 0 0xdb00 0x5c00
125 134 -936 808 180 0 1 [134, 183, 131] 4912 0 0xf400 0x5c00
126 134 -934 808 180 0 1 [134, 183, 131] 4912 0 0xf400 0x5c00
127 134 -932 808 180 0 1 [134, 183, 131] 4912 0 0xf400 0x5c00
128 134 -930 808 180 0 1 [134, 183, 131] 4911 0 0xf400 0x5c00
129 134 -928 808 180 0 1 [134, 183, 131] 4911 0 0xf400 0x5c00
130 134 -926 808 155 0 1 [134, 183, 131] 17 0 0xdb00 0x5c00
131 134 -924 808 67 0 1 [134, 183, 131] 15 0 0x8300 0x5c00
132 134 -922 808 68 0 1 [134, 183, 131] 18446744073709551615 1 0x8400 0x5c00
133 134 -920 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
134 134 -918 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
```
- edit channel: 0 linkage errors, 0 failed event dispatches (both must be 0)
- phase: 6 parity world boot (20:01:33)
- parity world: view-distance=8 (the voxy leg's server value)
- parity world: joined session 3 (20:02:19)
- parity world: scan populated (cells=65536; the march paints from these buffers)
- parity world tour: 13 vantages (production pack, vd 8, shared structures)
- coverage sweeps: 72 eye passes = 36 positions (12 az x r144/200 + 12 off-az x r172, y=160) x 2 looks (outward+inward), settle-by-laps >= 6 s (q stable 3x1s), deadline 12 s/pos, no shots — the v6.26.0 data-coverage lever + v6.26.6 lottery kill; probe rides parity/view_probe.md
- parity drain-settle TIMEOUT: queue 16696 -> 16696 > 11500 after 150s — continuing anyway; the queue did not drain without eye proximity (priority-lane evidence, the 01/05/10 dip attribution reads from the tour deltas)
- pre-tour pass done: 13 vantages x 10s (the tour now reads the second OR-merge lap; the rim/dip transport race reads from the tour deltas)
- parity world tour done: 13 shots, 0 night-guard renames in parity/ (the gate pairs voxy/<name>.png vs parity/<name>.png)
- phase: 7 parity tour done (20:24:40)
- sparse re-beat channel at tour end: streamed=82145 q=0 dropped_stale=22 dropped_undecodable=13025 (frac=8 boot; undecodable-dominant = the rim words die in their decodability window, stale-dominant = edit churn owns the cells)
### parity world stream counters (session 3, tail + maxima)
[c-LoD] stream: tick 13500 65536/65536 cells, lap 8849, cone 144475 zone 24349 dirty 8533 (q 16634) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 9190994c/1209e, edits +18206p/13817c/2827e drop 0, spawned 0 / failed 0, cam0 yaw -180 pitch 5, wish 0 forced 940, native 26944c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 2006163p/5287b fail 0 ~0 kB/s cache 18962/24576 h1583287 m422876 e403914, srbeat 82145s/0q drop 22stale/13025undec, ereb 37095s/13q
[20:24:35 INFO]: [c-LoD] stream: tick 13500 65536/65536 cells, lap 8849, cone 144475 zone 24349 dirty 8533 (q 16634) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 9190994c/1209e, edits +18206p/13817c/2827e drop 0, spawned 0 / failed 0, cam0 yaw -180 pitch 5, wish 0 forced 940, native 26944c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 2006163p/5287b fail 0 ~0 kB/s cache 18962/24576 h1583287 m422876 e403914, srbeat 82145s/0q drop 22stale/13025undec, ereb 37095s/13q
- parity stream max cone: 144475
- parity stream max zone: 24349
- parity stream max wire bundle: bundle
### parity world capture lens (debug chain on the §7b world)
lens_red=0 lens_black=0 lens_water=0 lens_dark=0 lens_zero=0 rot=none scan=0 carriers=0
- parity mesh probe @png_y=2: x306=(151,184,255) x307=(151,184,255) x308=(151,184,255) x309h=(151,184,255) x310c=(151,184,255)
- parity mesh probe @png_y=717: x306=(44,62,51) x307=(65,89,72) x308=(81,107,87) x309h=(82,108,87) x310c=(84,111,90)
- parity mesh engagement: magenta-ish=0 of 921600 sampled px
- mask8: the un-fog engagement masks shot on the PARITY WORLD (vd 8) — parity_dbg8/01m|07m|10m
- mask8 01m: green=8032 red=0 blue=100464 dim=6528 (px, sampled /16)
- mask8 07m: green=12064 red=0 blue=221360 dim=5808 (px, sampled /16)
- mask8 10m: green=7104 red=112 blue=226688 dim=162528 (px, sampled /16)
- mesh probe @png_y=2: x306=(255,255,255) x307=(13,254,0) x308=(0,254,0) x309h=(0,0,0) x310c=(0,0,0)
- mesh probe @png_y=717: x306=(255,255,255) x307=(13,254,0) x308=(0,254,0) x309h=(0,0,0) x310c=(0,0,0)
- mesh engagement: magenta-ish=0 of 921600 sampled px (the vanilla-path paint coverage)
- mask8 fog state @png_y=2: raw (300)=(151,184,255) (301)=(151,184,255)
- mask8 fog state @png_y=717: raw (300)=(0,128,8) (301)=(179,206,255)
- mask8 fog state decoded: envStart=0 envEnd=1024 rdEnd=128 fogColor=(179,206,255)
- buffer dump 10m: marked h=19/2048 c=1/2048; height windows r0<142:16 r1:1 r2>=202:2 maxByte=222
- buffer dump 10m: RANK-2-WINDOW HEIGHTS (the fake-wall class): (+0,-18)h=158, (+2,+26)h=151
- rim mask: scan populated (cells=65536; the state paint reads the same buffers the march reads)
- phase: 10 rim mask lane (20:29:20)
- rim mask: decode-state frames shot (rim_mask/00|01|12 — GREEN full decode / RED colorless / BLUE heightless; 00 = the leg-1 tower eye, 01 = the near-tower contrast, 12 = the tower re-read; the inherited 07/12 vantages sat 760 blocks outside the streamed disk and 01 sat INSIDE the marker column — their earlier verdicts were classifier noise, purged in v6.33.1)
- rim mask 00: green=7424 red=83664 blue=94784 yellow=17856 (px, sampled /16) -> DOMINANT BLUE: rim = HEIGHTS NEVER DECODED — a data-coverage class: the state paint cannot even see the cells
- rim mask 01: green=24976 red=118096 blue=133040 yellow=23040 (px, sampled /16) -> DOMINANT BLUE: rim = HEIGHTS NEVER DECODED — a data-coverage class: the state paint cannot even see the cells
- rim mask 12: green=19520 red=103488 blue=115648 yellow=21520 (px, sampled /16) -> DOMINANT BLUE: rim = HEIGHTS NEVER DECODED — a data-coverage class: the state paint cannot even see the cells

### boot / stream
```
[c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 127551 bytes, sha1 c7293c2a24265e83ebf9025127549657b5234c3b — zero textures, zero world data, one push per join
[20:28:30 INFO]: [c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 127551 bytes, sha1 c7293c2a24265e83ebf9025127549657b5234c3b — zero textures, zero world data, one push per join
c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[20:28:30 INFO]: [c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[c-LoD] stream: tick 900 65536/65536 cells, lap 729, cone 13831 zone 857 dirty 22 (q 10308) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 525023c/1903e, edits +12p/12c/6e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 218 forced 708, native 15567c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 136547p/492b fail 0 ~75 kB/s cache 14041/24576 h122506 m14041 e0, srbeat 4892s/1056q drop 0stale/518undec, ereb 639s/22q
[20:30:10 INFO]: [c-LoD] stream: tick 900 65536/65536 cells, lap 729, cone 13831 zone 857 dirty 22 (q 10308) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 525023c/1903e, edits +12p/12c/6e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 218 forced 708, native 15567c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 136547p/492b fail 0 ~75 kB/s cache 14041/24576 h122506 m14041 e0, srbeat 4892s/1056q drop 0stale/518undec, ereb 639s/22q
```
### sessions
```
[20:28:55 INFO]: System chat: Stepan joined the game
[20:30:15 INFO]: System chat: Stepan left the game
```
### client log tail
```
[20:30:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[20:30:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[20:30:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[20:30:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[20:30:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[20:30:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[20:30:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[20:30:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[20:30:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[20:30:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[20:30:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[20:30:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[20:30:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[20:30:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[20:30:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[20:30:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[20:30:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[20:30:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[20:30:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[20:30:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[20:30:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[20:30:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[20:30:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[20:30:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[20:30:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[20:30:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[20:30:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[20:30:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[20:30:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[20:30:15] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:bundle_delimiter] net.minecraft.network.protocol.game.ClientboundBundleDelimiterPacket -> 1 bytes
```
### server log tail
```
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[20:30:16 INFO]: [ChunkHolderManager] Saved 998 block chunks, 979 entity chunks, 0 poi chunks in world 'minecraft:overworld' in 0.87s
[20:30:16 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:overworld'
[20:30:16 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:overworld'
[20:30:16 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[20:30:16 INFO]: [ChunkHolderManager] Waiting 60s for chunk system to halt for world 'minecraft:the_nether'
[20:30:16 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:the_nether'
[20:30:16 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_nether'
[20:30:16 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_nether' in 0.00s
[20:30:16 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_nether'
[20:30:16 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_nether'
[20:30:16 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[20:30:16 INFO]: [ChunkHolderManager] Waiting 60s for chunk system to halt for world 'minecraft:the_end'
[20:30:16 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:the_end'
[20:30:16 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_end'
[20:30:16 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_end' in 0.00s
[20:30:16 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_end'
[20:30:16 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_end'
[20:30:16 INFO]: Waiting for all RegionFile I/O tasks to complete...
[20:30:16 INFO]: All RegionFile I/O tasks to complete
[20:30:16 INFO]: [MoonriseCommon] Awaiting termination of worker pool for up to 60s...
[20:30:16 INFO]: [MoonriseCommon] Awaiting termination of I/O pool for up to 60s...
[launcher] server exited with code 143
```
### shots
```
-rw-r--r-- 1 runner runner   7171 Nov 13 20:00 lens_13_after_relog_probe10.png
-rw-r--r-- 1 runner runner   7385 Nov 13 20:00 lens_13_after_relog_probe11.png
-rw-r--r-- 1 runner runner   7300 Nov 13 20:00 lens_13_after_relog_probe12.png
-rw-r--r-- 1 runner runner   6958 Nov 13 20:00 lens_13_after_relog_probe13.png
-rw-r--r-- 1 runner runner   7272 Nov 13 20:00 lens_13_after_relog_probe14.png
-rw-r--r-- 1 runner runner   8428 Nov 13 20:00 lens_13_after_relog_probe15.png
-rw-r--r-- 1 runner runner   8649 Nov 13 20:00 lens_13_after_relog_probe16.png
-rw-r--r-- 1 runner runner   8768 Nov 13 20:01 lens_13_after_relog_probe17.png
-rw-r--r-- 1 runner runner   9047 Nov 13 20:01 lens_13_after_relog_probe18.png
-rw-r--r-- 1 runner runner   8623 Nov 13 20:01 lens_13_after_relog_probe19.png
-rw-r--r-- 1 runner runner   6505 Nov 13 19:58 lens_13_after_relog_probe2.png
-rw-r--r-- 1 runner runner   6836 Nov 13 19:58 lens_13_after_relog_probe3.png
-rw-r--r-- 1 runner runner   7264 Nov 13 19:59 lens_13_after_relog_probe4.png
-rw-r--r-- 1 runner runner   7420 Nov 13 19:59 lens_13_after_relog_probe5.png
-rw-r--r-- 1 runner runner   7391 Nov 13 19:59 lens_13_after_relog_probe6.png
-rw-r--r-- 1 runner runner   6904 Nov 13 19:59 lens_13_after_relog_probe7.png
-rw-r--r-- 1 runner runner   7105 Nov 13 19:59 lens_13_after_relog_probe8.png
-rw-r--r-- 1 runner runner   7186 Nov 13 19:59 lens_13_after_relog_probe9.png
drwxr-xr-x 2 runner runner   4096 Nov 13 20:24 lens_parity
drwxr-xr-x 2 runner runner   4096 Nov 13 20:24 parity
drwxr-xr-x 2 runner runner   4096 Nov 13 19:55 parity_dbg
drwxr-xr-x 2 runner runner   4096 Nov 13 20:28 parity_dbg8
-rw-r--r-- 1 runner runner  10395 Nov 13 20:01 probe_counts.txt
drwxr-xr-x 2 runner runner   4096 Nov 13 20:30 rim_mask
-rw-r--r-- 1 runner runner  59550 Nov 13 20:30 summary.md
```

**exit=1** failures=2 leg1=0 leg2=0
