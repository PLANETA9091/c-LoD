- phase: 0 sanity (17:38:26)
### c-LoD visual run (-930,796, marker=red_concrete, vantage -1005,150 yaw 270)
- GL:     Vendor: Mesa (0xffffffff);    Device: llvmpipe (LLVM 21.1.8, 256 bits) (0xffffffff);OpenGL renderer string: llvmpipe (LLVM 21.1.8, 256 bits);OpenGL core profile version string: 4.5 (Core Profile) Mesa 26.0.8-1ubuntu0.3;
- boot + gates OK (17:38:47)
- phase: 1 server booted (17:38:47)
- sky-grid armed
- phase: 2 leg-1 client joined (17:38:58)
- joined session 1 (17:38:58)
- stream live: cells=65536
- phase: 3 scan populated (17:39:21)
- terrain range: 2 terraced mountains at (CX-4,CZ-20)/(CX-2,CZ+16), tops 185/190, grass terraces + stone caps, 66..82 m from the eye (inside the 83.5 m band, chunk edge 64 m)
- scan complete=1 (cells=cells=65536/65536)
- paint settled=1 (wire 549090p)
- baseline try 1: drift=18965 px
- baseline try 2: drift=20397 px
- baseline try 3: drift=21049 px
- baseline try 4: drift=22633 px
- baseline try 5: drift=22883 px
- baseline try 6: drift=15924 px
- **FAIL:** no stable baseline after 6 tries (drift=15924) — the A/B would lie
- lod_silhouette_cols=61 (LOD paint ON SCREEN at 75 m if > 20 — the march-drawn marker column; vanilla cannot render there, chunk edge 64 m; #255 re-measures 49, the ZONE_FAR=48 regression #256 and the control read 0)
- 4930 terrain-green px in the sky region (LOD MOUNTAINS on screen if > 5000; the range paints ColLow-green on low terraces via the fallback palette; flat-world baselines and the pack-free control read ~0)
- burst_close white px: 1721 (renderer alive if > 50)
- burst_far white px: 1108 (renderer alive in the marker zone if > 20)
- cut walk 40 m: 72 white px (> 20 = rendered)
- cut walk 60 m: 30 white px (> 20 = rendered)
- cut walk 100 m: 0 white px (> 20 = rendered)
- measured far-render cut: 60 m (G4 flap instrument; the decode proof needs >= 83 m from the y=151.6 eye)
- fill issued: 5x111x5 red_concrete at -930,796 (zone forceloaded ±48)
- phase: 4 marker filled (17:47:04)

### /clod-cells AFTER FILL (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 60 0 0x8400 0x2000
123 122 -940 784 71 0 1 [97, 119, 97] 60 0 0x8700 0x2000
124 122 -938 784 68 0 1 [97, 119, 97] 59 0 0x8400 0x2000
125 122 -936 784 62 1 1 [63, 118, 228] 99 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 98 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 98 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 97 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 610 0 0x8000 0x4100
130 122 -926 784 66 0 1 [134, 183, 131] 610 0 0x8200 0x4100
131 122 -924 784 75 0 1 [97, 119, 97] 610 0 0x8b00 0x2000
132 122 -922 784 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 122 -920 784 75 0 1 [97, 119, 97] 18446744073709551615 1 0x8b00 0x2000
134 122 -918 784 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x2000
122 123 -942 786 70 0 1 [97, 119, 97] 60 0 0x8600 0x2000
123 123 -940 786 65 0 1 [97, 119, 97] 60 0 0x8100 0x2000
124 123 -938 786 65 0 1 [97, 119, 97] 59 0 0x8100 0x2000
125 123 -936 786 62 1 1 [63, 118, 228] 99 0 0x7e00 0xd800
126 123 -934 786 62 1 1 [63, 118, 228] 98 0 0x7e00 0xd800
127 123 -932 786 62 1 1 [63, 118, 228] 98 0 0x7e00 0xd800
128 123 -930 786 62 1 1 [63, 118, 228] 97 0 0x7e00 0xd800
129 123 -928 786 73 0 1 [97, 119, 97] 610 0 0x8900 0x2000
130 123 -926 786 67 0 1 [134, 183, 131] 610 0 0x8300 0x4100
131 123 -924 786 69 0 1 [134, 183, 131] 65 0 0x8500 0x4100
132 123 -922 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 123 -920 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 123 -918 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
122 124 -942 788 72 0 1 [97, 119, 97] 60 0 0x8800 0x2000
123 124 -940 788 70 0 1 [97, 119, 97] 60 0 0x8600 0x2000
124 124 -938 788 68 0 1 [97, 119, 97] 59 0 0x8400 0x2000
125 124 -936 788 62 1 1 [63, 118, 228] 99 0 0x7e00 0xd800
126 124 -934 788 62 1 1 [63, 118, 228] 98 0 0x7e00 0xd800
127 124 -932 788 62 1 1 [63, 118, 228] 98 0 0x7e00 0xd800
128 124 -930 788 73 0 1 [97, 119, 97] 57 0 0x8900 0x2000
129 124 -928 788 76 0 1 [97, 119, 97] 610 0 0x8c00 0x2000
130 124 -926 788 73 0 1 [97, 119, 97] 610 0 0x8900 0x2000
131 124 -924 788 69 0 1 [134, 183, 131] 65 0 0x8500 0x4100
132 124 -922 788 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 124 -920 788 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 124 -918 788 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x2000
122 125 -942 790 74 0 1 [97, 119, 97] 60 0 0x8a00 0x2000
123 125 -940 790 73 0 1 [97, 119, 97] 60 0 0x8900 0x2000
124 125 -938 790 71 0 1 [97, 119, 97] 59 0 0x8700 0x2000
125 125 -936 790 62 1 1 [63, 118, 228] 99 0 0x7e00 0xd800
126 125 -934 790 62 1 1 [63, 118, 228] 98 0 0x7e00 0xd800
127 125 -932 790 62 1 1 [63, 118, 228] 98 0 0x7e00 0xd800
128 125 -930 790 62 1 1 [63, 118, 228] 97 0 0x7e00 0xd800
129 125 -928 790 73 0 1 [97, 119, 97] 57 0 0x8900 0x2000
130 125 -926 790 68 0 1 [134, 183, 131] 610 0 0x8400 0x4100
131 125 -924 790 69 0 1 [134, 183, 131] 65 0 0x8500 0x4100
132 125 -922 790 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
133 125 -920 790 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 125 -918 790 70 0 1 [134, 183, 131] 18446744073709551615 1 0x8600 0x4100
122 126 -942 792 74 0 1 [97, 119, 97] 60 0 0x8a00 0x2000
123 126 -940 792 73 0 1 [97, 119, 97] 60 0 0x8900 0x2000
124 126 -938 792 68 0 1 [97, 119, 97] 59 0 0x8400 0x2000
125 126 -936 792 62 1 1 [63, 118, 228] 99 0 0x7e00 0xd800
126 126 -934 792 62 1 1 [63, 118, 228] 98 0 0x7e00 0xd800
127 126 -932 792 62 1 1 [63, 118, 228] 98 0 0x7e00 0xd800
128 126 -930 792 62 1 1 [63, 118, 228] 97 0 0x7e00 0xd800
129 126 -928 792 67 0 1 [134, 183, 131] 57 0 0x8300 0x4100
130 126 -926 792 74 0 1 [97, 119, 97] 610 0 0x8a00 0x2000
131 126 -924 792 77 0 1 [97, 119, 97] 65 0 0x8d00 0x2000
132 126 -922 792 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 126 -920 792 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3200
134 126 -918 792 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x5000
122 127 -942 794 67 0 1 [134, 183, 131] 61 0 0x8300 0x4100
123 127 -940 794 68 0 1 [134, 183, 131] 60 0 0x8400 0x4100
124 127 -938 794 65 0 1 [134, 183, 131] 59 0 0x8100 0x4100
125 127 -936 794 62 1 1 [63, 118, 228] 99 0 0x7e00 0xd800
126 127 -934 794 62 1 1 [63, 118, 228] 98 0 0x7e00 0xd800
127 127 -932 794 62 1 1 [63, 118, 228] 98 0 0x7e00 0xd800
128 127 -930 794 62 1 1 [63, 118, 228] 97 0 0x7e00 0xd800
129 127 -928 794 66 0 1 [134, 183, 131] 57 0 0x8200 0x4100
130 127 -926 794 74 0 1 [97, 119, 97] 610 0 0x8a00 0x2000
131 127 -924 794 77 0 1 [97, 119, 97] 610 0 0x8d00 0x2000
132 127 -922 794 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 127 -920 794 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3200
134 127 -918 794 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x5000
122 128 -942 796 68 0 1 [134, 183, 131] 61 0 0x8400 0x4100
123 128 -940 796 72 0 1 [97, 119, 97] 60 0 0x8800 0x2000
124 128 -938 796 75 0 1 [97, 119, 97] 59 0 0x8b00 0x2000
125 128 -936 796 72 0 1 [97, 119, 97] 59 0 0x8800 0x2000
126 128 -934 796 62 1 1 [63, 118, 228] 98 0 0x7e00 0xd800
127 128 -932 796 62 1 1 [63, 118, 228] 98 0 0x7e00 0xd800
128 128 -930 796 62 1 1 [63, 118, 228] 97 0 0x7e00 0xd800
129 128 -928 796 66 0 1 [134, 183, 131] 57 0 0x8200 0x4100
130 128 -926 796 68 0 1 [134, 183, 131] 610 0 0x8400 0x4100
131 128 -924 796 74 0 1 [97, 119, 97] 65 0 0x8a00 0x2000
132 128 -922 796 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 128 -920 796 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
134 128 -918 796 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 129 -942 798 69 0 1 [134, 183, 131] 61 0 0x8500 0x4100
123 129 -940 798 75 0 1 [97, 119, 97] 60 0 0x8b00 0x2000
124 129 -938 798 78 0 1 [97, 119, 97] 61 0 0x8e00 0x2000
125 129 -936 798 75 0 1 [97, 119, 97] 59 0 0x8b00 0x2000
126 129 -934 798 62 1 1 [63, 118, 228] 98 0 0x7e00 0xd800
127 129 -932 798 62 1 1 [63, 118, 228] 98 0 0x7e00 0xd800
128 129 -930 798 62 1 1 [63, 118, 228] 97 0 0x7e00 0xd800
129 129 -928 798 71 0 1 [97, 119, 97] 57 0 0x8700 0x2000
130 129 -926 798 71 0 1 [97, 119, 97] 65 0 0x8700 0x2000
131 129 -924 798 69 0 1 [134, 183, 131] 65 0 0x8500 0x4100
132 129 -922 798 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 129 -920 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 129 -918 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 130 -942 800 70 0 1 [134, 183, 131] 60 0 0x8600 0x4100
123 130 -940 800 76 0 1 [97, 119, 97] 60 0 0x8c00 0x2000
124 130 -938 800 76 0 1 [97, 119, 97] 59 0 0x8c00 0x2000
125 130 -936 800 72 0 1 [97, 119, 97] 59 0 0x8800 0x2000
126 130 -934 800 62 1 1 [63, 118, 228] 98 0 0x7e00 0xd800
127 130 -932 800 62 1 1 [63, 118, 228] 98 0 0x7e00 0xd800
128 130 -930 800 62 1 1 [63, 118, 228] 97 0 0x7e00 0xd800
129 130 -928 800 71 0 1 [97, 119, 97] 57 0 0x8700 0x2000
130 130 -926 800 71 0 1 [97, 119, 97] 610 0 0x8700 0x2000
131 130 -924 800 69 0 1 [134, 183, 131] 610 0 0x8500 0x4100
132 130 -922 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
133 130 -920 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 130 -918 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 131 -942 802 78 0 1 [97, 119, 97] 62 0 0x8e00 0x2000
123 131 -940 802 77 0 1 [97, 119, 97] 60 0 0x8d00 0x2000
124 131 -938 802 77 0 1 [97, 119, 97] 59 0 0x8d00 0x2000
125 131 -936 802 66 0 1 [134, 183, 131] 59 0 0x8200 0x4100
126 131 -934 802 62 1 1 [63, 118, 228] 98 0 0x7e00 0xd800
127 131 -932 802 62 1 1 [63, 118, 228] 98 0 0x7e00 0xd800
128 131 -930 802 62 1 1 [63, 118, 228] 97 0 0x7e00 0xd800
129 131 -928 802 62 1 1 [63, 118, 228] 650 0 0x7e00 0xd800
130 131 -926 802 66 0 1 [134, 183, 131] 610 0 0x8200 0x4100
131 131 -924 802 68 0 1 [134, 183, 131] 610 0 0x8400 0x4100
132 131 -922 802 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 131 -920 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 131 -918 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 132 -942 804 79 0 1 [97, 119, 97] 62 0 0x8f00 0x2000
123 132 -940 804 76 0 1 [97, 119, 97] 60 0 0x8c00 0x2000
124 132 -938 804 76 0 1 [97, 119, 97] 59 0 0x8c00 0x2000
125 132 -936 804 76 0 1 [97, 119, 97] 59 0 0x8c00 0x2000
126 132 -934 804 62 1 1 [63, 118, 228] 98 0 0x7e00 0xd800
127 132 -932 804 62 1 1 [63, 118, 228] 98 0 0x7e00 0xd800
128 132 -930 804 62 1 1 [63, 118, 228] 97 0 0x7e00 0xd800
129 132 -928 804 62 1 1 [63, 118, 228] 650 0 0x7e00 0xd800
130 132 -926 804 65 0 1 [134, 183, 131] 610 0 0x8100 0x4100
131 132 -924 804 77 0 1 [97, 119, 97] 610 0 0x8d00 0x2000
132 132 -922 804 79 0 1 [97, 119, 97] 610 0 0x8f00 0x2000
133 132 -920 804 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 132 -918 804 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x2000
122 133 -942 806 78 0 1 [97, 119, 97] 62 0 0x8e00 0x2000
123 133 -940 806 69 0 1 [134, 183, 131] 60 0 0x8500 0x4100
124 133 -938 806 155 0 1 [134, 183, 131] 60 0 0xdb00 0x4100
125 133 -936 806 155 0 1 [134, 183, 131] 60 0 0xdb00 0x4100
126 133 -934 806 155 0 1 [134, 183, 131] 59 0 0xdb00 0x4100
127 133 -932 806 155 0 1 [134, 183, 131] 59 0 0xdb00 0x4100
128 133 -930 806 155 0 1 [134, 183, 131] 58 0 0xdb00 0x4100
129 133 -928 806 155 0 1 [134, 183, 131] 57 0 0xdb00 0x4100
130 133 -926 806 155 0 1 [134, 183, 131] 57 0 0xdb00 0x4100
131 133 -924 806 67 0 1 [134, 183, 131] 610 0 0x8300 0x4100
132 133 -922 806 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 133 -920 806 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 133 -918 806 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x2000
122 134 -942 808 79 0 1 [97, 119, 97] 62 0 0x8f00 0x2000
123 134 -940 808 68 0 1 [134, 183, 131] 60 0 0x8400 0x4100
124 134 -938 808 155 0 1 [134, 183, 131] 60 0 0xdb00 0x4100
125 134 -936 808 180 0 1 [134, 183, 131] 57 0 0xf400 0x4100
126 134 -934 808 180 0 1 [134, 183, 131] 57 0 0xf400 0x4100
127 134 -932 808 180 0 1 [134, 183, 131] 65 0 0xf400 0x4100
128 134 -930 808 180 0 1 [134, 183, 131] 65 0 0xf400 0x4100
129 134 -928 808 180 0 1 [134, 183, 131] 610 0 0xf400 0x4100
130 134 -926 808 155 0 1 [134, 183, 131] 57 0 0xdb00 0x4100
131 134 -924 808 67 0 1 [134, 183, 131] 610 0 0x8300 0x4100
132 134 -922 808 68 0 1 [134, 183, 131] 18446744073709551615 1 0x8400 0x4100
133 134 -920 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 134 -918 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
```
### stream diag after fill:
[c-LoD] stream: tick 5000 65536/65536 cells, lap 874, cone 36516 zone 2353 dirty 933 (q 11009) ewin 190 rbeat 9p/1 rearm 0c/0t rescan 1311381c/3329e, edits +6126p/4639c/699e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 351 forced 161, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 843228p/2210b fail 0 ~97 kB/s cache 13603/24576 h829625 m13603 e0, srbeat 66074s/9q drop 61stale/156undec, ereb 48332s/9q, sweep 0r/84d
[17:47:10 INFO]: [c-LoD] stream: tick 5000 65536/65536 cells, lap 874, cone 36516 zone 2353 dirty 933 (q 11009) ewin 190 rbeat 9p/1 rearm 0c/0t rescan 1311381c/3329e, edits +6126p/4639c/699e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 351 forced 161, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 843228p/2210b fail 0 ~97 kB/s cache 13603/24576 h829625 m13603 e0, srbeat 66074s/9q drop 61stale/156undec, ereb 48332s/9q, sweep 0r/84d
- carrier ladder y=155: 13420 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=165: 13504 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=168: 13660 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=172: 13632 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=176: 13848 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=180: 13344 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=185: 13616 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- probe 0: red=16054 paint=119510 lod_silhouette_cols=91 lens_red=0 lens_black=0 lens_water=0 lens_dark=0 lens_zero=0 rot=none scan=0 carriers=0 (need red>=60 paint>=300 lod>20)
- **LEG 1 PASS** (zero relogs)
- phase: 5 leg-1 probes done (17:47:36)
### stream diag after probes:
[17:47:20 INFO]: [c-LoD] stream: tick 5100 65536/65536 cells, lap 874, cone 36516 zone 2353 dirty 942 (q 11026) ewin 190 rbeat 9p/0 rearm 0c/0t rescan 1341726c/3346e, edits +6313p/4782c/761e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 351 forced 161, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 843957p/2220b fail 0 ~3 kB/s cache 13603/24576 h830354 m13603 e0, srbeat 66155s/9q drop 61stale/156undec, ereb 48404s/9q, sweep 0r/84d
[c-LoD] stream: tick 5200 65536/65536 cells, lap 874, cone 36516 zone 2353 dirty 951 (q 11029) ewin 190 rbeat 0p/0 rearm 0c/0t rescan 1375956c/3350e, edits +6424p/4852c/774e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 351 forced 161, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 844686p/2230b fail 0 ~3 kB/s cache 13603/24576 h831083 m13603 e0, srbeat 66236s/9q drop 61stale/156undec, ereb 48476s/9q, sweep 0r/84d
[17:47:30 INFO]: [c-LoD] stream: tick 5200 65536/65536 cells, lap 874, cone 36516 zone 2353 dirty 951 (q 11029) ewin 190 rbeat 0p/0 rearm 0c/0t rescan 1375956c/3350e, edits +6424p/4852c/774e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 351 forced 161, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 844686p/2230b fail 0 ~3 kB/s cache 13603/24576 h831083 m13603 e0, srbeat 66236s/9q drop 61stale/156undec, ereb 48476s/9q, sweep 0r/84d
- parity debug tour: 13 vantages (session-1 client, DEBUG pack — evidence only, the gate pairs the §7b world session)
- parity debug tour done: 13 shots in parity_dbg/ (the §7b world session feeds the gate)
- joined session 2 (17:50:16)
- leg-2 baseline scene px (x<940): 24860 (0 = the relog client paints no world at all)
- leg-2 renderer probe NEAR (20 m) white px: 2110 raw / 2649 changed-vs-baseline (> 20 = the client renders particles)
- leg-2 renderer probe FAR (80 m, marker zone) white px: 1611 raw / 4431 changed-vs-baseline (> 20 = no distance cut; ~0 with NEAR alive = particles beyond the client chunk radius do not render)
- leg-2 carrier probe green dust (80 m): changed-green px 0 / total changed 25020 vs baseline (> 100 green = a stable green carrier exists; total > 0 with green ~0 = the dust renders but fog washes it — a ruler problem, not a carrier problem)
- phase: 8 leg-2 relog probes (17:55:39)
- relog probe 0: red=16054 scene=24863 lens_red=0 lens_black=0 lens_water=0 lens_dark=0 lens_zero=0 rot=none scan=0 carriers=0 (need red >= 60; scene > 0 = the client paints a world at all)
- **LEG 2 PASS** (marker persisted over relog)
- phase: 9 leg-2 done (17:55:41)

### client log forensics (pipeline/particle/atlas — v6.8.2)
```
Horizon: ignoring minecraft:ambient_entity_effect: no such vanilla particle in this version
Horizon renderer: Vulkan has no conformant adapter (Vulkan: only a software adapter (llvmpipe (LLVM 21.1.8, 256 bits))); asking again for the ones wgpu hides as non-conformant
Horizon renderer: trying OpenGL (Vulkan: only a software adapter (llvmpipe (LLVM 21.1.8, 256 bits)))
libEGL warning: DRI3 error: Could not get DRI3 device
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
122 122 -942 784 68 0 1 [97, 119, 97] 4 0 0x8400 0x2000
123 122 -940 784 71 0 1 [97, 119, 97] 4 0 0x8700 0x2000
124 122 -938 784 68 0 1 [97, 119, 97] 4 0 0x8400 0x2000
125 122 -936 784 62 1 1 [63, 118, 228] 4 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 4 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 4 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 4 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 4 0 0x8000 0x4100
130 122 -926 784 66 0 1 [134, 183, 131] 4 0 0x8200 0x4100
131 122 -924 784 75 0 1 [97, 119, 97] 4 0 0x8b00 0x2000
132 122 -922 784 77 0 1 [97, 119, 97] 4053 1 0x8d00 0x2000
133 122 -920 784 75 0 1 [97, 119, 97] 4053 1 0x8b00 0x2000
134 122 -918 784 73 0 1 [97, 119, 97] 4053 1 0x8900 0x2000
122 123 -942 786 70 0 1 [97, 119, 97] 4 0 0x8600 0x2000
123 123 -940 786 65 0 1 [97, 119, 97] 4 0 0x8100 0x2000
124 123 -938 786 65 0 1 [97, 119, 97] 4 0 0x8100 0x2000
125 123 -936 786 62 1 1 [63, 118, 228] 4 0 0x7e00 0xd800
126 123 -934 786 62 1 1 [63, 118, 228] 4 0 0x7e00 0xd800
127 123 -932 786 62 1 1 [63, 118, 228] 4 0 0x7e00 0xd800
128 123 -930 786 62 1 1 [63, 118, 228] 4 0 0x7e00 0xd800
129 123 -928 786 73 0 1 [97, 119, 97] 4 0 0x8900 0x2000
130 123 -926 786 67 0 1 [134, 183, 131] 4 0 0x8300 0x4100
131 123 -924 786 69 0 1 [134, 183, 131] 4 0 0x8500 0x4100
132 123 -922 786 77 0 1 [97, 119, 97] 4053 1 0x8d00 0x2000
133 123 -920 786 77 0 1 [97, 119, 97] 4053 1 0x8d00 0x2000
134 123 -918 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
122 124 -942 788 72 0 1 [97, 119, 97] 4 0 0x8800 0x2000
123 124 -940 788 70 0 1 [97, 119, 97] 4 0 0x8600 0x2000
124 124 -938 788 68 0 1 [97, 119, 97] 4 0 0x8400 0x2000
125 124 -936 788 62 1 1 [63, 118, 228] 4 0 0x7e00 0xd800
126 124 -934 788 62 1 1 [63, 118, 228] 4 0 0x7e00 0xd800
127 124 -932 788 62 1 1 [63, 118, 228] 3 0 0x7e00 0xd800
128 124 -930 788 73 0 1 [97, 119, 97] 3 0 0x8900 0x2000
129 124 -928 788 76 0 1 [97, 119, 97] 3 0 0x8c00 0x2000
130 124 -926 788 73 0 1 [97, 119, 97] 3 0 0x8900 0x2000
131 124 -924 788 69 0 1 [134, 183, 131] 3 0 0x8500 0x4100
132 124 -922 788 77 0 1 [97, 119, 97] 4053 1 0x8d00 0x2000
133 124 -920 788 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 124 -918 788 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x2000
122 125 -942 790 74 0 1 [97, 119, 97] 4 0 0x8a00 0x2000
123 125 -940 790 73 0 1 [97, 119, 97] 3 0 0x8900 0x2000
124 125 -938 790 71 0 1 [97, 119, 97] 3 0 0x8700 0x2000
125 125 -936 790 62 1 1 [63, 118, 228] 3 0 0x7e00 0xd800
126 125 -934 790 62 1 1 [63, 118, 228] 3 0 0x7e00 0xd800
127 125 -932 790 62 1 1 [63, 118, 228] 3 0 0x7e00 0xd800
128 125 -930 790 62 1 1 [63, 118, 228] 3 0 0x7e00 0xd800
129 125 -928 790 73 0 1 [97, 119, 97] 3 0 0x8900 0x2000
130 125 -926 790 68 0 1 [134, 183, 131] 3 0 0x8400 0x4100
131 125 -924 790 69 0 1 [134, 183, 131] 3 0 0x8500 0x4100
132 125 -922 790 69 0 1 [134, 183, 131] 4053 1 0x8500 0x4100
133 125 -920 790 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 125 -918 790 70 0 1 [134, 183, 131] 18446744073709551615 1 0x8600 0x4100
122 126 -942 792 74 0 1 [97, 119, 97] 4 0 0x8a00 0x2000
123 126 -940 792 73 0 1 [97, 119, 97] 3 0 0x8900 0x2000
124 126 -938 792 68 0 1 [97, 119, 97] 3 0 0x8400 0x2000
125 126 -936 792 62 1 1 [63, 118, 228] 3 0 0x7e00 0xd800
126 126 -934 792 62 1 1 [63, 118, 228] 3 0 0x7e00 0xd800
127 126 -932 792 62 1 1 [63, 118, 228] 3 0 0x7e00 0xd800
128 126 -930 792 62 1 1 [63, 118, 228] 3 0 0x7e00 0xd800
129 126 -928 792 67 0 1 [134, 183, 131] 3 0 0x8300 0x4100
130 126 -926 792 74 0 1 [97, 119, 97] 3 0 0x8a00 0x2000
131 126 -924 792 77 0 1 [97, 119, 97] 3 0 0x8d00 0x2000
132 126 -922 792 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 126 -920 792 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3200
134 126 -918 792 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x5000
122 127 -942 794 67 0 1 [134, 183, 131] 4 0 0x8300 0x4100
123 127 -940 794 68 0 1 [134, 183, 131] 3 0 0x8400 0x4100
124 127 -938 794 65 0 1 [134, 183, 131] 3 0 0x8100 0x4100
125 127 -936 794 62 1 1 [63, 118, 228] 3 0 0x7e00 0xd800
126 127 -934 794 62 1 1 [63, 118, 228] 3 0 0x7e00 0xd800
127 127 -932 794 170 0 1 [142, 33, 33] 3 0 0xea00 0x0a00
128 127 -930 794 170 0 1 [142, 33, 33] 3 0 0xea00 0x0a00
129 127 -928 794 170 0 1 [142, 33, 33] 3 0 0xea00 0x0a00
130 127 -926 794 74 0 1 [97, 119, 97] 3 0 0x8a00 0x2000
131 127 -924 794 77 0 1 [97, 119, 97] 3 0 0x8d00 0x2000
132 127 -922 794 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 127 -920 794 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3200
134 127 -918 794 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x5000
122 128 -942 796 68 0 1 [134, 183, 131] 4 0 0x8400 0x4100
123 128 -940 796 72 0 1 [97, 119, 97] 3 0 0x8800 0x2000
124 128 -938 796 75 0 1 [97, 119, 97] 3 0 0x8b00 0x2000
125 128 -936 796 72 0 1 [97, 119, 97] 3 0 0x8800 0x2000
126 128 -934 796 62 1 1 [63, 118, 228] 3 0 0x7e00 0xd800
127 128 -932 796 170 0 1 [142, 33, 33] 3 0 0xea00 0x0a00
128 128 -930 796 170 0 1 [142, 33, 33] 3 0 0xea00 0x0a00
129 128 -928 796 170 0 1 [142, 33, 33] 3 0 0xea00 0x0a00
130 128 -926 796 68 0 1 [134, 183, 131] 3 0 0x8400 0x4100
131 128 -924 796 74 0 1 [97, 119, 97] 3 0 0x8a00 0x2000
132 128 -922 796 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 128 -920 796 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
134 128 -918 796 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 129 -942 798 69 0 1 [134, 183, 131] 4 0 0x8500 0x4100
123 129 -940 798 75 0 1 [97, 119, 97] 3 0 0x8b00 0x2000
124 129 -938 798 78 0 1 [97, 119, 97] 4 0 0x8e00 0x2000
125 129 -936 798 75 0 1 [97, 119, 97] 3 0 0x8b00 0x2000
126 129 -934 798 62 1 1 [63, 118, 228] 3 0 0x7e00 0xd800
127 129 -932 798 170 0 1 [142, 33, 33] 3 0 0xea00 0x0a00
128 129 -930 798 170 0 1 [142, 33, 33] 3 0 0xea00 0x0a00
129 129 -928 798 170 0 1 [142, 33, 33] 3 0 0xea00 0x0a00
130 129 -926 798 71 0 1 [97, 119, 97] 3 0 0x8700 0x2000
131 129 -924 798 69 0 1 [134, 183, 131] 3 0 0x8500 0x4100
132 129 -922 798 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 129 -920 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 129 -918 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 130 -942 800 70 0 1 [134, 183, 131] 4 0 0x8600 0x4100
123 130 -940 800 76 0 1 [97, 119, 97] 3 0 0x8c00 0x2000
124 130 -938 800 76 0 1 [97, 119, 97] 3 0 0x8c00 0x2000
125 130 -936 800 72 0 1 [97, 119, 97] 3 0 0x8800 0x2000
126 130 -934 800 62 1 1 [63, 118, 228] 3 0 0x7e00 0xd800
127 130 -932 800 62 1 1 [63, 118, 228] 3 0 0x7e00 0xd800
128 130 -930 800 62 1 1 [63, 118, 228] 3 0 0x7e00 0xd800
129 130 -928 800 71 0 1 [97, 119, 97] 3 0 0x8700 0x2000
130 130 -926 800 71 0 1 [97, 119, 97] 3 0 0x8700 0x2000
131 130 -924 800 69 0 1 [134, 183, 131] 3 0 0x8500 0x4100
132 130 -922 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
133 130 -920 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 130 -918 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 131 -942 802 78 0 1 [97, 119, 97] 5 0 0x8e00 0x2000
123 131 -940 802 77 0 1 [97, 119, 97] 3 0 0x8d00 0x2000
124 131 -938 802 77 0 1 [97, 119, 97] 3 0 0x8d00 0x2000
125 131 -936 802 66 0 1 [134, 183, 131] 3 0 0x8200 0x4100
126 131 -934 802 62 1 1 [63, 118, 228] 3 0 0x7e00 0xd800
127 131 -932 802 62 1 1 [63, 118, 228] 3 0 0x7e00 0xd800
128 131 -930 802 62 1 1 [63, 118, 228] 3 0 0x7e00 0xd800
129 131 -928 802 62 1 1 [63, 118, 228] 3 0 0x7e00 0xd800
130 131 -926 802 66 0 1 [134, 183, 131] 3 0 0x8200 0x4100
131 131 -924 802 68 0 1 [134, 183, 131] 3 0 0x8400 0x4100
132 131 -922 802 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 131 -920 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 131 -918 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 132 -942 804 79 0 1 [97, 119, 97] 5 0 0x8f00 0x2000
123 132 -940 804 76 0 1 [97, 119, 97] 3 0 0x8c00 0x2000
124 132 -938 804 76 0 1 [97, 119, 97] 3 0 0x8c00 0x2000
125 132 -936 804 76 0 1 [97, 119, 97] 3 0 0x8c00 0x2000
126 132 -934 804 62 1 1 [63, 118, 228] 3 0 0x7e00 0xd800
127 132 -932 804 62 1 1 [63, 118, 228] 3 0 0x7e00 0xd800
128 132 -930 804 62 1 1 [63, 118, 228] 3 0 0x7e00 0xd800
129 132 -928 804 62 1 1 [63, 118, 228] 3 0 0x7e00 0xd800
130 132 -926 804 65 0 1 [134, 183, 131] 3 0 0x8100 0x4100
131 132 -924 804 77 0 1 [97, 119, 97] 3 0 0x8d00 0x2000
132 132 -922 804 79 0 1 [97, 119, 97] 3 0 0x8f00 0x2000
133 132 -920 804 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 132 -918 804 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x2000
122 133 -942 806 78 0 1 [97, 119, 97] 5 0 0x8e00 0x2000
123 133 -940 806 69 0 1 [134, 183, 131] 3 0 0x8500 0x4100
124 133 -938 806 155 0 1 [134, 183, 131] 4 0 0xdb00 0x4100
125 133 -936 806 155 0 1 [134, 183, 131] 3 0 0xdb00 0x4100
126 133 -934 806 155 0 1 [134, 183, 131] 3 0 0xdb00 0x4100
127 133 -932 806 155 0 1 [134, 183, 131] 3 0 0xdb00 0x4100
128 133 -930 806 155 0 1 [134, 183, 131] 3 0 0xdb00 0x4100
129 133 -928 806 155 0 1 [134, 183, 131] 3 0 0xdb00 0x4100
130 133 -926 806 155 0 1 [134, 183, 131] 3 0 0xdb00 0x4100
131 133 -924 806 67 0 1 [134, 183, 131] 3 0 0x8300 0x4100
132 133 -922 806 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 133 -920 806 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 133 -918 806 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x2000
122 134 -942 808 79 0 1 [97, 119, 97] 5 0 0x8f00 0x2000
123 134 -940 808 68 0 1 [134, 183, 131] 3 0 0x8400 0x4100
124 134 -938 808 155 0 1 [134, 183, 131] 4 0 0xdb00 0x4100
125 134 -936 808 180 0 1 [134, 183, 131] 3 0 0xf400 0x4100
126 134 -934 808 180 0 1 [134, 183, 131] 3 0 0xf400 0x4100
127 134 -932 808 180 0 1 [134, 183, 131] 3 0 0xf400 0x4100
128 134 -930 808 180 0 1 [134, 183, 131] 3 0 0xf400 0x4100
129 134 -928 808 180 0 1 [134, 183, 131] 3 0 0xf400 0x4100
130 134 -926 808 155 0 1 [134, 183, 131] 3 0 0xdb00 0x4100
131 134 -924 808 67 0 1 [134, 183, 131] 3 0 0x8300 0x4100
132 134 -922 808 68 0 1 [134, 183, 131] 18446744073709551615 1 0x8400 0x4100
133 134 -920 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 134 -918 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
```
- edit channel: 0 linkage errors, 0 failed event dispatches (both must be 0)
- phase: 6 parity world boot (17:55:41)
- parity world: view-distance=8 (the voxy leg's server value)
- parity world: joined session 3 (17:56:07)
- parity world: scan populated (cells=65536; the march paints from these buffers)
- parity world tour: 13 vantages (production pack, vd 8, shared structures)
- coverage sweeps: 72 eye passes = 36 positions (12 az x r144/200 + 12 off-az x r172, y=160) x 2 looks (outward+inward), settle-by-laps >= 6 s (q stable 3x1s), deadline 12 s/pos, no shots — the v6.26.0 data-coverage lever + v6.26.6 lottery kill; probe rides parity/view_probe.md
- parity drain-settle TIMEOUT: queue 18789 -> 18790 > 11500 after 150s — continuing anyway; the queue did not drain without eye proximity (priority-lane evidence, the 01/05/10 dip attribution reads from the tour deltas)
- pre-tour pass done: 13 vantages x 10s (the tour now reads the second OR-merge lap; the rim/dip transport race reads from the tour deltas)
- **DAY GUARD:** 02_aerial_south still night after re-pin+reshoot (zenith < 130) — renamed night_02_aerial_south.png; the row pairs as missing, NOT as a renderer diff
- **DAY GUARD:** 03_aerial_north still night after re-pin+reshoot (zenith < 130) — renamed night_03_aerial_north.png; the row pairs as missing, NOT as a renderer diff
- **DAY GUARD:** 04_aerial_east still night after re-pin+reshoot (zenith < 130) — renamed night_04_aerial_east.png; the row pairs as missing, NOT as a renderer diff
- **DAY GUARD:** 06_ground_west still night after re-pin+reshoot (zenith < 130) — renamed night_06_ground_west.png; the row pairs as missing, NOT as a renderer diff
- **DAY GUARD:** 08_back_inside still night after re-pin+reshoot (zenith < 130) — renamed night_08_back_inside.png; the row pairs as missing, NOT as a renderer diff
- **DAY GUARD:** 09_after_mesh still night after re-pin+reshoot (zenith < 130) — renamed night_09_after_mesh.png; the row pairs as missing, NOT as a renderer diff
- parity world tour done: 13 shots, 6 night-guard renames in parity/ (the gate pairs voxy/<name>.png vs parity/<name>.png)
- phase: 7 parity tour done (18:19:07)
- sparse re-beat channel at tour end: streamed=151975 q=0 dropped_stale=58 dropped_undecodable=6241 (frac=8 boot; undecodable-dominant = the rim words die in their decodability window, stale-dominant = edit churn owns the cells)
- **FAIL:** parity world time-of-day uncontrollable: 6 vantage(s) night after re-pin+reshoot — the §7b noon-pin class is broken on this runner; the matrix is infrastructure-blocked, not a renderer verdict (DAY GUARD lines + the feed's time-audit.md carry the evidence)
### parity world stream counters (session 3, tail + maxima)
[c-LoD] stream: tick 13800 65536/65536 cells, lap 1290, cone 71447 zone 24705 dirty 16911 (q 18753) ewin 0 rbeat 0p/1 rearm 0c/0t rescan 9500050c/3243e, edits +17874p/13539c/2902e drop 0, spawned 0 / failed 0, cam0 yaw -180 pitch 5, wish 0 forced 752, native 41483c/955ch fail 0 (3w q 0) helper on, trees 955ch/fail 0, wire bundle 2075267p/4763b fail 0 ~0 kB/s cache 24528/24576 h1621720 m453547 e429019, srbeat 151975s/0q drop 58stale/6241undec, ereb 88680s/0q, sweep 0r/13d
[18:19:03 INFO]: [c-LoD] stream: tick 13800 65536/65536 cells, lap 1290, cone 71447 zone 24705 dirty 16911 (q 18753) ewin 0 rbeat 0p/1 rearm 0c/0t rescan 9500050c/3243e, edits +17874p/13539c/2902e drop 0, spawned 0 / failed 0, cam0 yaw -180 pitch 5, wish 0 forced 752, native 41483c/955ch fail 0 (3w q 0) helper on, trees 955ch/fail 0, wire bundle 2075267p/4763b fail 0 ~0 kB/s cache 24528/24576 h1621720 m453547 e429019, srbeat 151975s/0q drop 58stale/6241undec, ereb 88680s/0q, sweep 0r/13d
- parity stream max cone: 71447
- parity stream max zone: 24705
- parity stream max wire bundle: bundle
### parity world capture lens (debug chain on the §7b world)
lens_red=0 lens_black=0 lens_water=0 lens_dark=5808 lens_zero=0 rot=decoded cam_x_b=? cam_z_b=? az_b=? el_b=? fog=live fog_end_b=? strobe_hits=21 hits_hi=22 mask_hits=21
- parity mesh probe @png_y=2: x306=(74,90,125) x307=(74,90,125) x308=(74,90,125) x309h=(74,90,125) x310c=(74,90,125)
- parity mesh probe @png_y=717: x306=(22,31,25) x307=(23,32,26) x308=(40,53,43) x309h=(40,53,43) x310c=(41,55,44)
- parity mesh engagement: magenta-ish=1504 of 921600 sampled px
- mask8: the un-fog engagement masks shot on the PARITY WORLD (vd 8) — parity_dbg8/01m|07m|10m
- mask8 01m: green=25904 red=0 blue=201712 dim=374544 (px, sampled /16)
- mask8 07m: green=9664 red=0 blue=149248 dim=10848 (px, sampled /16)
- mask8 10m: green=4096 red=0 blue=131824 dim=144528 (px, sampled /16)
- mesh probe @png_y=2: x306=(83,102,141) x307=(83,102,141) x308=(83,102,141) x309h=(83,102,141) x310c=(83,102,141)
- mesh probe @png_y=717: x306=(25,35,29) x307=(26,36,30) x308=(45,60,49) x309h=(45,60,49) x310c=(47,62,50)
- mesh engagement: magenta-ish=1504 of 921600 sampled px (the vanilla-path paint coverage)
- mask8 fog state @png_y=2: raw (300)=(82,100,139) (301)=(83,101,140)
- mask8 fog state decoded: envStart=656 envEnd=800 rdEnd=2224 fogColor=(83,101,140)
- mask8 fog state @png_y=717: raw (300)=(26,38,30) (301)=(28,40,32)
- mask8 fog state decoded: envStart=208 envEnd=304 rdEnd=480 fogColor=(28,40,32)
- buffer dump 10m: marked h=2048/2048 c=2048/2048; height windows r0<142:0 r1:2048 r2>=202:0 maxByte=178
- rim mask: scan populated (cells=65536; the state paint reads the same buffers the march reads)
- phase: 10 rim mask lane (18:24:33)
