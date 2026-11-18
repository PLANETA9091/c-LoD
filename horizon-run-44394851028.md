- phase: 0 sanity (21:22:18)
### c-LoD visual run (-930,796, marker=red_concrete, vantage -1005,150 yaw 270)
- GL:     Vendor: Mesa (0xffffffff);    Device: llvmpipe (LLVM 21.1.8, 256 bits) (0xffffffff);OpenGL renderer string: llvmpipe (LLVM 21.1.8, 256 bits);OpenGL core profile version string: 4.5 (Core Profile) Mesa 26.0.8-1ubuntu0.3;
- boot + gates OK (21:22:39)
- phase: 1 server booted (21:22:39)
- sky-grid armed
- phase: 2 leg-1 client joined (21:22:50)
- joined session 1 (21:22:50)
- stream live: cells=65536
- phase: 3 scan populated (21:23:13)
- terrain range: 2 terraced mountains at (CX-4,CZ-20)/(CX-2,CZ+16), tops 185/190, grass terraces + stone caps, 66..82 m from the eye (inside the 83.5 m band, chunk edge 64 m)
- scan complete=1 (cells=cells=65536/65536)
- paint settled=1 (wire 552974p)
- baseline try 1: drift=23839 px
- baseline try 2: drift=18207 px
- baseline try 3: drift=16834 px
- baseline try 4: drift=22449 px
- baseline try 5: drift=12832 px
- baseline try 6: drift=9332 px
- **FAIL:** no stable baseline after 6 tries (drift=9332) — the A/B would lie
- lod_silhouette_cols=61 (LOD paint ON SCREEN at 75 m if > 20 — the march-drawn marker column; vanilla cannot render there, chunk edge 64 m; #255 re-measures 49, the ZONE_FAR=48 regression #256 and the control read 0)
- 184 terrain-green px in the sky region (LOD MOUNTAINS on screen if > 5000; the range paints ColLow-green on low terraces via the fallback palette; flat-world baselines and the pack-free control read ~0)
- burst_close white px: 1677 (renderer alive if > 50)
- burst_far white px: 1150 (renderer alive in the marker zone if > 20)
- cut walk 40 m: 70 white px (> 20 = rendered)
- cut walk 60 m: 30 white px (> 20 = rendered)
- cut walk 100 m: 0 white px (> 20 = rendered)
- measured far-render cut: 60 m (G4 flap instrument; the decode proof needs >= 83 m from the y=151.6 eye)
- fill issued: 5x111x5 red_concrete at -930,796 (zone forceloaded ±48)
- phase: 4 marker filled (21:31:16)

### /clod-cells AFTER FILL (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 23 0 0x8400 0x2000
123 122 -940 784 71 0 1 [97, 119, 97] 23 0 0x8700 0x2000
124 122 -938 784 68 0 1 [97, 119, 97] 22 0 0x8400 0x2000
125 122 -936 784 62 1 1 [63, 118, 228] 22 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 21 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 20 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 20 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 202 0 0x8000 0x4100
130 122 -926 784 66 0 1 [134, 183, 131] 202 0 0x8200 0x4100
131 122 -924 784 75 0 1 [97, 119, 97] 202 0 0x8b00 0x2000
132 122 -922 784 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 122 -920 784 75 0 1 [97, 119, 97] 18446744073709551615 1 0x8b00 0x2000
134 122 -918 784 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x2000
122 123 -942 786 70 0 1 [97, 119, 97] 23 0 0x8600 0x2000
123 123 -940 786 65 0 1 [97, 119, 97] 23 0 0x8100 0x2000
124 123 -938 786 65 0 1 [97, 119, 97] 22 0 0x8100 0x2000
125 123 -936 786 62 1 1 [63, 118, 228] 22 0 0x7e00 0xd800
126 123 -934 786 62 1 1 [63, 118, 228] 21 0 0x7e00 0xd800
127 123 -932 786 62 1 1 [63, 118, 228] 21 0 0x7e00 0xd800
128 123 -930 786 62 1 1 [63, 118, 228] 20 0 0x7e00 0xd800
129 123 -928 786 73 0 1 [97, 119, 97] 202 0 0x8900 0x2000
130 123 -926 786 67 0 1 [134, 183, 131] 202 0 0x8300 0x4100
131 123 -924 786 69 0 1 [134, 183, 131] 202 0 0x8500 0x4100
132 123 -922 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 123 -920 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 123 -918 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
122 124 -942 788 72 0 1 [97, 119, 97] 23 0 0x8800 0x2000
123 124 -940 788 70 0 1 [97, 119, 97] 23 0 0x8600 0x2000
124 124 -938 788 68 0 1 [97, 119, 97] 22 0 0x8400 0x2000
125 124 -936 788 62 1 1 [63, 118, 228] 22 0 0x7e00 0xd800
126 124 -934 788 62 1 1 [63, 118, 228] 21 0 0x7e00 0xd800
127 124 -932 788 62 1 1 [63, 118, 228] 21 0 0x7e00 0xd800
128 124 -930 788 73 0 1 [97, 119, 97] 20 0 0x8900 0x2000
129 124 -928 788 76 0 1 [97, 119, 97] 202 0 0x8c00 0x2000
130 124 -926 788 73 0 1 [97, 119, 97] 202 0 0x8900 0x2000
131 124 -924 788 69 0 1 [134, 183, 131] 202 0 0x8500 0x4100
132 124 -922 788 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 124 -920 788 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 124 -918 788 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x2000
122 125 -942 790 74 0 1 [97, 119, 97] 23 0 0x8a00 0x2000
123 125 -940 790 73 0 1 [97, 119, 97] 23 0 0x8900 0x2000
124 125 -938 790 71 0 1 [97, 119, 97] 22 0 0x8700 0x2000
125 125 -936 790 62 1 1 [63, 118, 228] 22 0 0x7e00 0xd800
126 125 -934 790 62 1 1 [63, 118, 228] 21 0 0x7e00 0xd800
127 125 -932 790 62 1 1 [63, 118, 228] 21 0 0x7e00 0xd800
128 125 -930 790 62 1 1 [63, 118, 228] 20 0 0x7e00 0xd800
129 125 -928 790 73 0 1 [97, 119, 97] 202 0 0x8900 0x2000
130 125 -926 790 68 0 1 [134, 183, 131] 202 0 0x8400 0x4100
131 125 -924 790 69 0 1 [134, 183, 131] 202 0 0x8500 0x4100
132 125 -922 790 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
133 125 -920 790 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 125 -918 790 70 0 1 [134, 183, 131] 18446744073709551615 1 0x8600 0x4100
122 126 -942 792 74 0 1 [97, 119, 97] 23 0 0x8a00 0x2000
123 126 -940 792 73 0 1 [97, 119, 97] 23 0 0x8900 0x2000
124 126 -938 792 68 0 1 [97, 119, 97] 22 0 0x8400 0x2000
125 126 -936 792 62 1 1 [63, 118, 228] 22 0 0x7e00 0xd800
126 126 -934 792 62 1 1 [63, 118, 228] 21 0 0x7e00 0xd800
127 126 -932 792 62 1 1 [63, 118, 228] 21 0 0x7e00 0xd800
128 126 -930 792 62 1 1 [63, 118, 228] 20 0 0x7e00 0xd800
129 126 -928 792 67 0 1 [134, 183, 131] 20 0 0x8300 0x4100
130 126 -926 792 74 0 1 [97, 119, 97] 202 0 0x8a00 0x2000
131 126 -924 792 77 0 1 [97, 119, 97] 202 0 0x8d00 0x2000
132 126 -922 792 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 126 -920 792 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3200
134 126 -918 792 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x5000
122 127 -942 794 67 0 1 [134, 183, 131] 23 0 0x8300 0x4100
123 127 -940 794 68 0 1 [134, 183, 131] 23 0 0x8400 0x4100
124 127 -938 794 65 0 1 [134, 183, 131] 22 0 0x8100 0x4100
125 127 -936 794 62 1 1 [63, 118, 228] 22 0 0x7e00 0xd800
126 127 -934 794 62 1 1 [63, 118, 228] 21 0 0x7e00 0xd800
127 127 -932 794 62 1 1 [63, 118, 228] 21 0 0x7e00 0xd800
128 127 -930 794 62 1 1 [63, 118, 228] 20 0 0x7e00 0xd800
129 127 -928 794 66 0 1 [134, 183, 131] 20 0 0x8200 0x4100
130 127 -926 794 74 0 1 [97, 119, 97] 202 0 0x8a00 0x2000
131 127 -924 794 77 0 1 [97, 119, 97] 202 0 0x8d00 0x2000
132 127 -922 794 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 127 -920 794 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3200
134 127 -918 794 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x5000
122 128 -942 796 68 0 1 [134, 183, 131] 23 0 0x8400 0x4100
123 128 -940 796 72 0 1 [97, 119, 97] 23 0 0x8800 0x2000
124 128 -938 796 75 0 1 [97, 119, 97] 22 0 0x8b00 0x2000
125 128 -936 796 72 0 1 [97, 119, 97] 22 0 0x8800 0x2000
126 128 -934 796 62 1 1 [63, 118, 228] 21 0 0x7e00 0xd800
127 128 -932 796 62 1 1 [63, 118, 228] 21 0 0x7e00 0xd800
128 128 -930 796 62 1 1 [63, 118, 228] 20 0 0x7e00 0xd800
129 128 -928 796 66 0 1 [134, 183, 131] 20 0 0x8200 0x4100
130 128 -926 796 68 0 1 [134, 183, 131] 202 0 0x8400 0x4100
131 128 -924 796 74 0 1 [97, 119, 97] 202 0 0x8a00 0x2000
132 128 -922 796 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 128 -920 796 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
134 128 -918 796 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 129 -942 798 69 0 1 [134, 183, 131] 23 0 0x8500 0x4100
123 129 -940 798 75 0 1 [97, 119, 97] 23 0 0x8b00 0x2000
124 129 -938 798 78 0 1 [97, 119, 97] 24 0 0x8e00 0x2000
125 129 -936 798 75 0 1 [97, 119, 97] 22 0 0x8b00 0x2000
126 129 -934 798 62 1 1 [63, 118, 228] 21 0 0x7e00 0xd800
127 129 -932 798 62 1 1 [63, 118, 228] 21 0 0x7e00 0xd800
128 129 -930 798 62 1 1 [63, 118, 228] 20 0 0x7e00 0xd800
129 129 -928 798 71 0 1 [97, 119, 97] 20 0 0x8700 0x2000
130 129 -926 798 71 0 1 [97, 119, 97] 202 0 0x8700 0x2000
131 129 -924 798 69 0 1 [134, 183, 131] 202 0 0x8500 0x4100
132 129 -922 798 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 129 -920 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 129 -918 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 130 -942 800 70 0 1 [134, 183, 131] 23 0 0x8600 0x4100
123 130 -940 800 76 0 1 [97, 119, 97] 23 0 0x8c00 0x2000
124 130 -938 800 76 0 1 [97, 119, 97] 22 0 0x8c00 0x2000
125 130 -936 800 72 0 1 [97, 119, 97] 22 0 0x8800 0x2000
126 130 -934 800 62 1 1 [63, 118, 228] 21 0 0x7e00 0xd800
127 130 -932 800 62 1 1 [63, 118, 228] 21 0 0x7e00 0xd800
128 130 -930 800 62 1 1 [63, 118, 228] 20 0 0x7e00 0xd800
129 130 -928 800 71 0 1 [97, 119, 97] 202 0 0x8700 0x2000
130 130 -926 800 71 0 1 [97, 119, 97] 202 0 0x8700 0x2000
131 130 -924 800 69 0 1 [134, 183, 131] 202 0 0x8500 0x4100
132 130 -922 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
133 130 -920 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 130 -918 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 131 -942 802 78 0 1 [97, 119, 97] 25 0 0x8e00 0x2000
123 131 -940 802 77 0 1 [97, 119, 97] 23 0 0x8d00 0x2000
124 131 -938 802 77 0 1 [97, 119, 97] 22 0 0x8d00 0x2000
125 131 -936 802 66 0 1 [134, 183, 131] 22 0 0x8200 0x4100
126 131 -934 802 62 1 1 [63, 118, 228] 21 0 0x7e00 0xd800
127 131 -932 802 62 1 1 [63, 118, 228] 21 0 0x7e00 0xd800
128 131 -930 802 62 1 1 [63, 118, 228] 20 0 0x7e00 0xd800
129 131 -928 802 62 1 1 [63, 118, 228] 242 0 0x7e00 0xd800
130 131 -926 802 66 0 1 [134, 183, 131] 202 0 0x8200 0x4100
131 131 -924 802 68 0 1 [134, 183, 131] 202 0 0x8400 0x4100
132 131 -922 802 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 131 -920 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 131 -918 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 132 -942 804 79 0 1 [97, 119, 97] 25 0 0x8f00 0x2000
123 132 -940 804 76 0 1 [97, 119, 97] 23 0 0x8c00 0x2000
124 132 -938 804 76 0 1 [97, 119, 97] 22 0 0x8c00 0x2000
125 132 -936 804 76 0 1 [97, 119, 97] 22 0 0x8c00 0x2000
126 132 -934 804 62 1 1 [63, 118, 228] 21 0 0x7e00 0xd800
127 132 -932 804 62 1 1 [63, 118, 228] 21 0 0x7e00 0xd800
128 132 -930 804 62 1 1 [63, 118, 228] 20 0 0x7e00 0xd800
129 132 -928 804 62 1 1 [63, 118, 228] 242 0 0x7e00 0xd800
130 132 -926 804 65 0 1 [134, 183, 131] 202 0 0x8100 0x4100
131 132 -924 804 77 0 1 [97, 119, 97] 202 0 0x8d00 0x2000
132 132 -922 804 79 0 1 [97, 119, 97] 202 0 0x8f00 0x2000
133 132 -920 804 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 132 -918 804 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x2000
122 133 -942 806 78 0 1 [97, 119, 97] 25 0 0x8e00 0x2000
123 133 -940 806 69 0 1 [134, 183, 131] 23 0 0x8500 0x4100
124 133 -938 806 155 0 1 [134, 183, 131] 23 0 0xdb00 0x4100
125 133 -936 806 155 0 1 [134, 183, 131] 23 0 0xdb00 0x4100
126 133 -934 806 155 0 1 [134, 183, 131] 22 0 0xdb00 0x4100
127 133 -932 806 155 0 1 [134, 183, 131] 22 0 0xdb00 0x4100
128 133 -930 806 155 0 1 [134, 183, 131] 21 0 0xdb00 0x4100
129 133 -928 806 155 0 1 [134, 183, 131] 20 0 0xdb00 0x4100
130 133 -926 806 155 0 1 [134, 183, 131] 20 0 0xdb00 0x4100
131 133 -924 806 67 0 1 [134, 183, 131] 202 0 0x8300 0x4100
132 133 -922 806 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 133 -920 806 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 133 -918 806 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x2000
122 134 -942 808 79 0 1 [97, 119, 97] 25 0 0x8f00 0x2000
123 134 -940 808 68 0 1 [134, 183, 131] 23 0 0x8400 0x4100
124 134 -938 808 155 0 1 [134, 183, 131] 23 0 0xdb00 0x4100
125 134 -936 808 180 0 1 [134, 183, 131] 20 0 0xf400 0x4100
126 134 -934 808 180 0 1 [134, 183, 131] 20 0 0xf400 0x4100
127 134 -932 808 180 0 1 [134, 183, 131] 202 0 0xf400 0x4100
128 134 -930 808 180 0 1 [134, 183, 131] 202 0 0xf400 0x4100
129 134 -928 808 180 0 1 [134, 183, 131] 202 0 0xf400 0x4100
130 134 -926 808 155 0 1 [134, 183, 131] 20 0 0xdb00 0x4100
131 134 -924 808 67 0 1 [134, 183, 131] 202 0 0x8300 0x4100
132 134 -922 808 68 0 1 [134, 183, 131] 18446744073709551615 1 0x8400 0x4100
133 134 -920 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 134 -918 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
```
### stream diag after fill:
[c-LoD] stream: tick 5200 65536/65536 cells, lap 832, cone 32230 zone 2137 dirty 935 (q 10279) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1363980c/2388e, edits +5950p/4477c/678e drop 0, spawned 0 / failed 0, cas 0d, cam0 yaw -90 pitch 5, wish 355 forced 157, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 779041p/2359b fail 0 ~74 kB/s cache 13637/24576 h765404 m13637 e0, srbeat 62468s/551q drop 66stale/101undec, ereb 47693s/10q, sweep 0r/90d, cdiag off
[21:31:21 INFO]: [c-LoD] stream: tick 5200 65536/65536 cells, lap 832, cone 32230 zone 2137 dirty 935 (q 10279) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1363980c/2388e, edits +5950p/4477c/678e drop 0, spawned 0 / failed 0, cas 0d, cam0 yaw -90 pitch 5, wish 355 forced 157, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 779041p/2359b fail 0 ~74 kB/s cache 13637/24576 h765404 m13637 e0, srbeat 62468s/551q drop 66stale/101undec, ereb 47693s/10q, sweep 0r/90d, cdiag off
- carrier ladder y=155: 15400 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=165: 14028 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=168: 17996 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=172: 15148 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=176: 13572 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=180: 15324 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=185: 14412 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- probe 0: red=15957 paint=115262 lod_silhouette_cols=91 lens_red=0 lens_black=0 lens_water=0 lens_dark=0 lens_zero=0 rot=decoded cam_x_b=? cam_z_b=? az_b=? el_b=? fog=live fog_end_b=? strobe_hits=22 hits_hi=22 mask_hits=22 (need red>=60 paint>=300 lod>20)
- **LEG 1 PASS** (zero relogs)
- phase: 5 leg-1 probes done (21:31:49)
### stream diag after probes:
[21:31:31 INFO]: [c-LoD] stream: tick 5300 65536/65536 cells, lap 841, cone 33372 zone 2137 dirty 935 (q 10300) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1398620c/2403e, edits +6166p/4644c/749e drop 0, spawned 0 / failed 0, cas 0d, cam0 yaw -90 pitch 5, wish 355 forced 157, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 793128p/2391b fail 0 ~74 kB/s cache 13637/24576 h779491 m13637 e0, srbeat 63099s/552q drop 66stale/101undec, ereb 47773s/10q, sweep 0r/99d, cdiag off
[c-LoD] stream: tick 5400 65536/65536 cells, lap 843, cone 33625 zone 2137 dirty 935 (q 10303) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1428551c/2410e, edits +6276p/4713c/762e drop 0, spawned 0 / failed 0, cas 0d, cam0 yaw -90 pitch 5, wish 355 forced 157, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 799176p/2417b fail 0 ~31 kB/s cache 13637/24576 h785539 m13637 e0, srbeat 63739s/113q drop 66stale/101undec, ereb 47861s/9q, sweep 8r/0d, cdiag off
[21:31:41 INFO]: [c-LoD] stream: tick 5400 65536/65536 cells, lap 843, cone 33625 zone 2137 dirty 935 (q 10303) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1428551c/2410e, edits +6276p/4713c/762e drop 0, spawned 0 / failed 0, cas 0d, cam0 yaw -90 pitch 5, wish 355 forced 157, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 799176p/2417b fail 0 ~31 kB/s cache 13637/24576 h785539 m13637 e0, srbeat 63739s/113q drop 66stale/101undec, ereb 47861s/9q, sweep 8r/0d, cdiag off
- parity debug tour: 13 vantages (session-1 client, DEBUG pack — evidence only, the gate pairs the §7b world session)
- parity debug tour done: 13 shots in parity_dbg/ (the §7b world session feeds the gate)
- joined session 2 (21:34:29)
- leg-2 baseline scene px (x<940): 24860 (0 = the relog client paints no world at all)
- leg-2 renderer probe NEAR (20 m) white px: 1236 raw / 2482 changed-vs-baseline (> 20 = the client renders particles)
- leg-2 renderer probe FAR (80 m, marker zone) white px: 658 raw / 3757 changed-vs-baseline (> 20 = no distance cut; ~0 with NEAR alive = particles beyond the client chunk radius do not render)
- leg-2 carrier probe green dust (80 m): changed-green px 0 / total changed 22164 vs baseline (> 100 green = a stable green carrier exists; total > 0 with green ~0 = the dust renders but fog washes it — a ruler problem, not a carrier problem)
- phase: 8 leg-2 relog probes (21:39:51)
- relog probe 0: red=15951 scene=27484 lens_red=0 lens_black=9 lens_water=0 lens_dark=0 lens_zero=0 rot=decoded cam_x_b=? cam_z_b=? az_b=? el_b=? fog=live fog_end_b=? strobe_hits=22 hits_hi=22 mask_hits=22 (need red >= 60; scene > 0 = the client paints a world at all)
- **LEG 2 PASS** (marker persisted over relog)
- phase: 9 leg-2 done (21:39:54)

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
122 122 -942 784 68 0 1 [97, 119, 97] 0 0 0x8400 0x2000
123 122 -940 784 71 0 1 [97, 119, 97] 0 0 0x8700 0x2000
124 122 -938 784 68 0 1 [97, 119, 97] 0 0 0x8400 0x2000
125 122 -936 784 62 1 1 [63, 118, 228] 0 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 0 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 0 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 0 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 0 0 0x8000 0x4100
130 122 -926 784 66 0 1 [134, 183, 131] 0 0 0x8200 0x4100
131 122 -924 784 75 0 1 [97, 119, 97] 0 0 0x8b00 0x2000
132 122 -922 784 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 122 -920 784 75 0 1 [97, 119, 97] 18446744073709551615 1 0x8b00 0x2000
134 122 -918 784 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x2000
122 123 -942 786 70 0 1 [97, 119, 97] 0 0 0x8600 0x2000
123 123 -940 786 65 0 1 [97, 119, 97] 0 0 0x8100 0x2000
124 123 -938 786 65 0 1 [97, 119, 97] 0 0 0x8100 0x2000
125 123 -936 786 62 1 1 [63, 118, 228] 0 0 0x7e00 0xd800
126 123 -934 786 62 1 1 [63, 118, 228] 0 0 0x7e00 0xd800
127 123 -932 786 62 1 1 [63, 118, 228] 0 0 0x7e00 0xd800
128 123 -930 786 62 1 1 [63, 118, 228] 0 0 0x7e00 0xd800
129 123 -928 786 73 0 1 [97, 119, 97] 0 0 0x8900 0x2000
130 123 -926 786 67 0 1 [134, 183, 131] 0 0 0x8300 0x4100
131 123 -924 786 69 0 1 [134, 183, 131] 0 0 0x8500 0x4100
132 123 -922 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 123 -920 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 123 -918 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
122 124 -942 788 72 0 1 [97, 119, 97] 0 0 0x8800 0x2000
123 124 -940 788 70 0 1 [97, 119, 97] 0 0 0x8600 0x2000
124 124 -938 788 68 0 1 [97, 119, 97] 3484 0 0x8400 0x2000
125 124 -936 788 62 1 1 [63, 118, 228] 3484 0 0x7e00 0xd800
126 124 -934 788 62 1 1 [63, 118, 228] 3484 0 0x7e00 0xd800
127 124 -932 788 62 1 1 [63, 118, 228] 3484 0 0x7e00 0xd800
128 124 -930 788 73 0 1 [97, 119, 97] 3484 0 0x8900 0x2000
129 124 -928 788 76 0 1 [97, 119, 97] 3484 0 0x8c00 0x2000
130 124 -926 788 73 0 1 [97, 119, 97] 3484 0 0x8900 0x2000
131 124 -924 788 69 0 1 [134, 183, 131] 3484 0 0x8500 0x4100
132 124 -922 788 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 124 -920 788 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 124 -918 788 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x2000
122 125 -942 790 74 0 1 [97, 119, 97] 3484 0 0x8a00 0x2000
123 125 -940 790 73 0 1 [97, 119, 97] 3484 0 0x8900 0x2000
124 125 -938 790 71 0 1 [97, 119, 97] 3484 0 0x8700 0x2000
125 125 -936 790 62 1 1 [63, 118, 228] 3484 0 0x7e00 0xd800
126 125 -934 790 62 1 1 [63, 118, 228] 3484 0 0x7e00 0xd800
127 125 -932 790 62 1 1 [63, 118, 228] 3484 0 0x7e00 0xd800
128 125 -930 790 62 1 1 [63, 118, 228] 3484 0 0x7e00 0xd800
129 125 -928 790 73 0 1 [97, 119, 97] 3484 0 0x8900 0x2000
130 125 -926 790 68 0 1 [134, 183, 131] 3484 0 0x8400 0x4100
131 125 -924 790 69 0 1 [134, 183, 131] 3484 0 0x8500 0x4100
132 125 -922 790 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
133 125 -920 790 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 125 -918 790 70 0 1 [134, 183, 131] 18446744073709551615 1 0x8600 0x4100
122 126 -942 792 74 0 1 [97, 119, 97] 3484 0 0x8a00 0x2000
123 126 -940 792 73 0 1 [97, 119, 97] 3484 0 0x8900 0x2000
124 126 -938 792 68 0 1 [97, 119, 97] 3484 0 0x8400 0x2000
125 126 -936 792 62 1 1 [63, 118, 228] 3484 0 0x7e00 0xd800
126 126 -934 792 62 1 1 [63, 118, 228] 3484 0 0x7e00 0xd800
127 126 -932 792 62 1 1 [63, 118, 228] 3484 0 0x7e00 0xd800
128 126 -930 792 62 1 1 [63, 118, 228] 3484 0 0x7e00 0xd800
129 126 -928 792 67 0 1 [134, 183, 131] 3484 0 0x8300 0x4100
130 126 -926 792 74 0 1 [97, 119, 97] 3484 0 0x8a00 0x2000
131 126 -924 792 77 0 1 [97, 119, 97] 3484 0 0x8d00 0x2000
132 126 -922 792 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 126 -920 792 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3200
134 126 -918 792 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x5000
122 127 -942 794 67 0 1 [134, 183, 131] 3484 0 0x8300 0x4100
123 127 -940 794 68 0 1 [134, 183, 131] 3484 0 0x8400 0x4100
124 127 -938 794 65 0 1 [134, 183, 131] 3484 0 0x8100 0x4100
125 127 -936 794 62 1 1 [63, 118, 228] 3484 0 0x7e00 0xd800
126 127 -934 794 62 1 1 [63, 118, 228] 3484 0 0x7e00 0xd800
127 127 -932 794 170 0 1 [142, 33, 33] 3471 0 0xea00 0x0a00
128 127 -930 794 170 0 1 [142, 33, 33] 3471 0 0xea00 0x0a00
129 127 -928 794 170 0 1 [142, 33, 33] 3469 0 0xea00 0x0a00
130 127 -926 794 74 0 1 [97, 119, 97] 3484 0 0x8a00 0x2000
131 127 -924 794 77 0 1 [97, 119, 97] 3484 0 0x8d00 0x2000
132 127 -922 794 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 127 -920 794 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3200
134 127 -918 794 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x5000
122 128 -942 796 68 0 1 [134, 183, 131] 3484 0 0x8400 0x4100
123 128 -940 796 72 0 1 [97, 119, 97] 3484 0 0x8800 0x2000
124 128 -938 796 75 0 1 [97, 119, 97] 3484 0 0x8b00 0x2000
125 128 -936 796 72 0 1 [97, 119, 97] 3484 0 0x8800 0x2000
126 128 -934 796 62 1 1 [63, 118, 228] 3484 0 0x7e00 0xd800
127 128 -932 796 170 0 1 [142, 33, 33] 3471 0 0xea00 0x0a00
128 128 -930 796 170 0 1 [142, 33, 33] 3471 0 0xea00 0x0a00
129 128 -928 796 170 0 1 [142, 33, 33] 3469 0 0xea00 0x0a00
130 128 -926 796 68 0 1 [134, 183, 131] 3484 0 0x8400 0x4100
131 128 -924 796 74 0 1 [97, 119, 97] 3484 0 0x8a00 0x2000
132 128 -922 796 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 128 -920 796 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
134 128 -918 796 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 129 -942 798 69 0 1 [134, 183, 131] 3484 0 0x8500 0x4100
123 129 -940 798 75 0 1 [97, 119, 97] 3484 0 0x8b00 0x2000
124 129 -938 798 78 0 1 [97, 119, 97] 3484 0 0x8e00 0x2000
125 129 -936 798 75 0 1 [97, 119, 97] 3484 0 0x8b00 0x2000
126 129 -934 798 62 1 1 [63, 118, 228] 3484 0 0x7e00 0xd800
127 129 -932 798 170 0 1 [142, 33, 33] 3471 0 0xea00 0x0a00
128 129 -930 798 170 0 1 [142, 33, 33] 3471 0 0xea00 0x0a00
129 129 -928 798 170 0 1 [142, 33, 33] 3469 0 0xea00 0x0a00
130 129 -926 798 71 0 1 [97, 119, 97] 3484 0 0x8700 0x2000
131 129 -924 798 69 0 1 [134, 183, 131] 3484 0 0x8500 0x4100
132 129 -922 798 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 129 -920 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 129 -918 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 130 -942 800 70 0 1 [134, 183, 131] 3484 0 0x8600 0x4100
123 130 -940 800 76 0 1 [97, 119, 97] 3484 0 0x8c00 0x2000
124 130 -938 800 76 0 1 [97, 119, 97] 3484 0 0x8c00 0x2000
125 130 -936 800 72 0 1 [97, 119, 97] 3484 0 0x8800 0x2000
126 130 -934 800 62 1 1 [63, 118, 228] 3484 0 0x7e00 0xd800
127 130 -932 800 62 1 1 [63, 118, 228] 3484 0 0x7e00 0xd800
128 130 -930 800 62 1 1 [63, 118, 228] 3484 0 0x7e00 0xd800
129 130 -928 800 71 0 1 [97, 119, 97] 3484 0 0x8700 0x2000
130 130 -926 800 71 0 1 [97, 119, 97] 3484 0 0x8700 0x2000
131 130 -924 800 69 0 1 [134, 183, 131] 3483 0 0x8500 0x4100
132 130 -922 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
133 130 -920 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 130 -918 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 131 -942 802 78 0 1 [97, 119, 97] 3485 0 0x8e00 0x2000
123 131 -940 802 77 0 1 [97, 119, 97] 3483 0 0x8d00 0x2000
124 131 -938 802 77 0 1 [97, 119, 97] 3483 0 0x8d00 0x2000
125 131 -936 802 66 0 1 [134, 183, 131] 3483 0 0x8200 0x4100
126 131 -934 802 62 1 1 [63, 118, 228] 3483 0 0x7e00 0xd800
127 131 -932 802 62 1 1 [63, 118, 228] 3483 0 0x7e00 0xd800
128 131 -930 802 62 1 1 [63, 118, 228] 3483 0 0x7e00 0xd800
129 131 -928 802 62 1 1 [63, 118, 228] 3483 0 0x7e00 0xd800
130 131 -926 802 66 0 1 [134, 183, 131] 3483 0 0x8200 0x4100
131 131 -924 802 68 0 1 [134, 183, 131] 3483 0 0x8400 0x4100
132 131 -922 802 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 131 -920 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 131 -918 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 132 -942 804 79 0 1 [97, 119, 97] 3485 0 0x8f00 0x2000
123 132 -940 804 76 0 1 [97, 119, 97] 3483 0 0x8c00 0x2000
124 132 -938 804 76 0 1 [97, 119, 97] 3483 0 0x8c00 0x2000
125 132 -936 804 76 0 1 [97, 119, 97] 3483 0 0x8c00 0x2000
126 132 -934 804 62 1 1 [63, 118, 228] 3483 0 0x7e00 0xd800
127 132 -932 804 62 1 1 [63, 118, 228] 3483 0 0x7e00 0xd800
128 132 -930 804 62 1 1 [63, 118, 228] 3483 0 0x7e00 0xd800
129 132 -928 804 62 1 1 [63, 118, 228] 3483 0 0x7e00 0xd800
130 132 -926 804 65 0 1 [134, 183, 131] 3483 0 0x8100 0x4100
131 132 -924 804 77 0 1 [97, 119, 97] 3483 0 0x8d00 0x2000
132 132 -922 804 79 0 1 [97, 119, 97] 3483 0 0x8f00 0x2000
133 132 -920 804 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 132 -918 804 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x2000
122 133 -942 806 78 0 1 [97, 119, 97] 3485 0 0x8e00 0x2000
123 133 -940 806 69 0 1 [134, 183, 131] 3483 0 0x8500 0x4100
124 133 -938 806 155 0 1 [134, 183, 131] 3483 0 0xdb00 0x4100
125 133 -936 806 155 0 1 [134, 183, 131] 3483 0 0xdb00 0x4100
126 133 -934 806 155 0 1 [134, 183, 131] 3483 0 0xdb00 0x4100
127 133 -932 806 155 0 1 [134, 183, 131] 3483 0 0xdb00 0x4100
128 133 -930 806 155 0 1 [134, 183, 131] 3483 0 0xdb00 0x4100
129 133 -928 806 155 0 1 [134, 183, 131] 3483 0 0xdb00 0x4100
130 133 -926 806 155 0 1 [134, 183, 131] 3483 0 0xdb00 0x4100
131 133 -924 806 67 0 1 [134, 183, 131] 3483 0 0x8300 0x4100
132 133 -922 806 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 133 -920 806 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 133 -918 806 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x2000
122 134 -942 808 79 0 1 [97, 119, 97] 3485 0 0x8f00 0x2000
123 134 -940 808 68 0 1 [134, 183, 131] 3483 0 0x8400 0x4100
124 134 -938 808 155 0 1 [134, 183, 131] 3483 0 0xdb00 0x4100
125 134 -936 808 180 0 1 [134, 183, 131] 3483 0 0xf400 0x4100
126 134 -934 808 180 0 1 [134, 183, 131] 3483 0 0xf400 0x4100
127 134 -932 808 180 0 1 [134, 183, 131] 3483 0 0xf400 0x4100
128 134 -930 808 180 0 1 [134, 183, 131] 3483 0 0xf400 0x4100
129 134 -928 808 180 0 1 [134, 183, 131] 3483 0 0xf400 0x4100
130 134 -926 808 155 0 1 [134, 183, 131] 3483 0 0xdb00 0x4100
131 134 -924 808 67 0 1 [134, 183, 131] 3483 0 0x8300 0x4100
132 134 -922 808 68 0 1 [134, 183, 131] 18446744073709551615 1 0x8400 0x4100
133 134 -920 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 134 -918 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
```
- edit channel: 0 linkage errors, 0 failed event dispatches (both must be 0)
- phase: 6 parity world boot (21:39:54)
- parity world: view-distance=8 (the voxy leg's server value)
- parity world: joined session 3 (21:40:19)
- parity world: scan populated (cells=65536; the march paints from these buffers)
- parity world tour: 13 vantages (production pack, vd 8, shared structures)
- coverage sweeps: 72 eye passes = 36 positions (12 az x r144/200 + 12 off-az x r172, y=160) x 2 looks (outward+inward), settle-by-laps >= 6 s (q stable 3x1s), deadline 12 s/pos, no shots — the v6.26.0 data-coverage lever + v6.26.6 lottery kill; probe rides parity/view_probe.md
- parity drain-settle TIMEOUT: queue 16502 -> 16502 > 11500 after 150s — continuing anyway; the queue did not drain without eye proximity (priority-lane evidence, the 01/05/10 dip attribution reads from the tour deltas)
- pre-tour pass done: 13 vantages x 10s (the tour now reads the second OR-merge lap; the rim/dip transport race reads from the tour deltas)
- **DAY GUARD:** 02_aerial_south still night after re-pin+reshoot (zenith < 130) — renamed night_02_aerial_south.png; the row pairs as missing, NOT as a renderer diff
- **DAY GUARD:** 03_aerial_north still night after re-pin+reshoot (zenith < 130) — renamed night_03_aerial_north.png; the row pairs as missing, NOT as a renderer diff
- **DAY GUARD:** 04_aerial_east still night after re-pin+reshoot (zenith < 130) — renamed night_04_aerial_east.png; the row pairs as missing, NOT as a renderer diff
- **DAY GUARD:** 06_ground_west still night after re-pin+reshoot (zenith < 130) — renamed night_06_ground_west.png; the row pairs as missing, NOT as a renderer diff
- **DAY GUARD:** 08_back_inside still night after re-pin+reshoot (zenith < 130) — renamed night_08_back_inside.png; the row pairs as missing, NOT as a renderer diff
- **DAY GUARD:** 09_after_mesh still night after re-pin+reshoot (zenith < 130) — renamed night_09_after_mesh.png; the row pairs as missing, NOT as a renderer diff
- parity world tour done: 13 shots, 6 night-guard renames in parity/ (the gate pairs voxy/<name>.png vs parity/<name>.png)
- phase: 7 parity tour done (22:03:11)
- sparse re-beat channel at tour end: streamed=130004 q=0 dropped_stale=114 dropped_undecodable=10043 (frac=8 boot; undecodable-dominant = the rim words die in their decodability window, stale-dominant = edit churn owns the cells)
- **FAIL:** parity world time-of-day uncontrollable: 6 vantage(s) night after re-pin+reshoot — the §7b noon-pin class is broken on this runner; the matrix is infrastructure-blocked, not a renderer verdict (DAY GUARD lines + the feed's time-audit.md carry the evidence)
### parity world stream counters (session 3, tail + maxima)
[c-LoD] stream: tick 13700 65536/65536 cells, lap 1227, cone 65907 zone 25289 dirty 15523 (q 16416) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 9420325c/2101e, edits +18061p/13673c/2789e drop 0, spawned 0 / failed 0, cas 0d, cam0 yaw -180 pitch 5, wish 0 forced 847, native 33432c/951ch fail 0 (3w q 0) helper on, trees 951ch/fail 0, wire bundle 1854982p/4376b fail 0 ~0 kB/s cache 21806/24576 h1420595 m434387 e412581, srbeat 130004s/0q drop 114stale/10043undec, ereb 75935s/1q, sweep 0r/36d, cdiag off
[22:03:05 INFO]: [c-LoD] stream: tick 13700 65536/65536 cells, lap 1227, cone 65907 zone 25289 dirty 15523 (q 16416) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 9420325c/2101e, edits +18061p/13673c/2789e drop 0, spawned 0 / failed 0, cas 0d, cam0 yaw -180 pitch 5, wish 0 forced 847, native 33432c/951ch fail 0 (3w q 0) helper on, trees 951ch/fail 0, wire bundle 1854982p/4376b fail 0 ~0 kB/s cache 21806/24576 h1420595 m434387 e412581, srbeat 130004s/0q drop 114stale/10043undec, ereb 75935s/1q, sweep 0r/36d, cdiag off
- parity stream max cone: 65907
- parity stream max zone: 25289
- parity stream max wire bundle: bundle
### parity world capture lens (debug chain on the §7b world)
lens_red=0 lens_black=0 lens_water=0 lens_dark=0 lens_zero=0 rot=none scan=0 carriers=0
- parity mesh probe @png_y=2: x306=(74,90,125) x307=(74,90,125) x308=(74,90,125) x309h=(74,90,125) x310c=(74,90,125)
- parity mesh probe @png_y=717: x306=(22,31,25) x307=(23,32,26) x308=(41,55,44) x309h=(41,54,44) x310c=(42,56,45)
- parity mesh engagement: magenta-ish=1504 of 921600 sampled px
