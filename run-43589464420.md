- phase: 0 sanity (11:20:53)
### c-LoD visual run (-930,796, marker=red_concrete, vantage -1005,150 yaw 270)
- GL:     Vendor: Mesa (0xffffffff);    Device: llvmpipe (LLVM 21.1.8, 256 bits) (0xffffffff);OpenGL renderer string: llvmpipe (LLVM 21.1.8, 256 bits);OpenGL core profile version string: 4.5 (Core Profile) Mesa 26.0.8-1ubuntu0.3;
- boot + gates OK (11:21:21)
- phase: 1 server booted (11:21:21)
- sky-grid armed
- phase: 2 leg-1 client joined (11:21:48)
- joined session 1 (11:21:48)
- stream live: cells=65536
- phase: 3 scan populated (11:22:11)
- terrain range: 2 terraced mountains at (CX-4,CZ-20)/(CX-2,CZ+16), tops 185/190, grass terraces + stone caps, 66..82 m from the eye (inside the 83.5 m band, chunk edge 64 m)
- scan complete=1 (cells=cells=65536/65536)
- paint settled=0 (wire 897019p)
- baseline try 1: drift=890 px
- lod_silhouette_cols=35 (LOD paint ON SCREEN at 75 m if > 20 — the march-drawn marker column; vanilla cannot render there, chunk edge 64 m; #255 re-measures 49, the ZONE_FAR=48 regression #256 and the control read 0)
- 236 terrain-green px in the sky region (LOD MOUNTAINS on screen if > 5000; the range paints ColLow-green on low terraces via the fallback palette; flat-world baselines and the pack-free control read ~0)
- burst_close white px: 528 (renderer alive if > 50)
- burst_far white px: 397 (renderer alive in the marker zone if > 20)
- cut walk 40 m: 54 white px (> 20 = rendered)
- cut walk 60 m: 0 white px (> 20 = rendered)
- cut walk 100 m: 0 white px (> 20 = rendered)
- measured far-render cut: 40 m (G4 flap instrument; the decode proof needs >= 83 m from the y=151.6 eye)
- fill issued: 5x111x5 red_concrete at -930,796 (zone forceloaded ±48)
- phase: 4 marker filled (11:30:40)

### /clod-cells AFTER FILL (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 15 0 0x8400 0x3100
123 122 -940 784 71 0 1 [97, 119, 97] 12 0 0x8700 0x3100
124 122 -938 784 68 0 1 [97, 119, 97] 14 0 0x8400 0x3100
125 122 -936 784 62 1 1 [63, 118, 228] 13 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 13 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 12 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 12 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 12 0 0x8000 0x5c00
130 122 -926 784 66 0 1 [134, 183, 131] 11 0 0x8200 0x5c00
131 122 -924 784 75 0 1 [97, 119, 97] 11 0 0x8b00 0x3100
132 122 -922 784 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 122 -920 784 75 0 1 [97, 119, 97] 18446744073709551615 1 0x8b00 0x3100
134 122 -918 784 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x3100
122 123 -942 786 70 0 1 [97, 119, 97] 15 0 0x8600 0x3100
123 123 -940 786 65 0 1 [97, 119, 97] 13 0 0x8100 0x3100
124 123 -938 786 65 0 1 [97, 119, 97] 14 0 0x8100 0x3100
125 123 -936 786 62 1 1 [63, 118, 228] 13 0 0x7e00 0xd800
126 123 -934 786 62 1 1 [63, 118, 228] 13 0 0x7e00 0xd800
127 123 -932 786 62 1 1 [63, 118, 228] 13 0 0x7e00 0xd800
128 123 -930 786 62 1 1 [63, 118, 228] 12 0 0x7e00 0xd800
129 123 -928 786 73 0 1 [97, 119, 97] 3 0 0x8900 0x3100
130 123 -926 786 67 0 1 [134, 183, 131] 11 0 0x8300 0x5c00
131 123 -924 786 69 0 1 [134, 183, 131] 11 0 0x8500 0x5c00
132 123 -922 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 123 -920 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
134 123 -918 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
122 124 -942 788 72 0 1 [97, 119, 97] 15 0 0x8800 0x3100
123 124 -940 788 70 0 1 [97, 119, 97] 13 0 0x8600 0x3100
124 124 -938 788 68 0 1 [97, 119, 97] 14 0 0x8400 0x3100
125 124 -936 788 62 1 1 [63, 118, 228] 14 0 0x7e00 0xd800
126 124 -934 788 62 1 1 [63, 118, 228] 13 0 0x7e00 0xd800
127 124 -932 788 62 1 1 [63, 118, 228] 13 0 0x7e00 0xd800
128 124 -930 788 73 0 1 [97, 119, 97] 12 0 0x8900 0x3100
129 124 -928 788 76 0 1 [97, 119, 97] 10 0 0x8c00 0x3100
130 124 -926 788 73 0 1 [97, 119, 97] 11 0 0x8900 0x3100
131 124 -924 788 69 0 1 [134, 183, 131] 11 0 0x8500 0x5c00
132 124 -922 788 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 124 -920 788 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
134 124 -918 788 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x3100
122 125 -942 790 74 0 1 [97, 119, 97] 15 0 0x8a00 0x3100
123 125 -940 790 73 0 1 [97, 119, 97] 13 0 0x8900 0x3100
124 125 -938 790 71 0 1 [97, 119, 97] 14 0 0x8700 0x3100
125 125 -936 790 62 1 1 [63, 118, 228] 14 0 0x7e00 0xd800
126 125 -934 790 62 1 1 [63, 118, 228] 13 0 0x7e00 0xd800
127 125 -932 790 62 1 1 [63, 118, 228] 13 0 0x7e00 0xd800
128 125 -930 790 62 1 1 [63, 118, 228] 12 0 0x7e00 0xd800
129 125 -928 790 73 0 1 [97, 119, 97] 0 0 0x8900 0x3100
130 125 -926 790 68 0 1 [134, 183, 131] 11 0 0x8400 0x5c00
131 125 -924 790 69 0 1 [134, 183, 131] 11 0 0x8500 0x5c00
132 125 -922 790 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
133 125 -920 790 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
134 125 -918 790 70 0 1 [134, 183, 131] 18446744073709551615 1 0x8600 0x5c00
122 126 -942 792 74 0 1 [97, 119, 97] 15 0 0x8a00 0x3100
123 126 -940 792 73 0 1 [97, 119, 97] 13 0 0x8900 0x3100
124 126 -938 792 68 0 1 [97, 119, 97] 14 0 0x8400 0x3100
125 126 -936 792 62 1 1 [63, 118, 228] 14 0 0x7e00 0xd800
126 126 -934 792 62 1 1 [63, 118, 228] 13 0 0x7e00 0xd800
127 126 -932 792 62 1 1 [63, 118, 228] 13 0 0x7e00 0xd800
128 126 -930 792 62 1 1 [63, 118, 228] 12 0 0x7e00 0xd800
129 126 -928 792 67 0 1 [134, 183, 131] 12 0 0x8300 0x5c00
130 126 -926 792 74 0 1 [97, 119, 97] 11 0 0x8a00 0x3100
131 126 -924 792 77 0 1 [97, 119, 97] 12 0 0x8d00 0x3100
132 126 -922 792 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x3100
133 126 -920 792 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3800
134 126 -918 792 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x8800
122 127 -942 794 67 0 1 [134, 183, 131] 15 0 0x8300 0x5c00
123 127 -940 794 68 0 1 [134, 183, 131] 15 0 0x8400 0x5c00
124 127 -938 794 65 0 1 [134, 183, 131] 14 0 0x8100 0x5c00
125 127 -936 794 62 1 1 [63, 118, 228] 14 0 0x7e00 0xd800
126 127 -934 794 62 1 1 [63, 118, 228] 13 0 0x7e00 0xd800
127 127 -932 794 62 1 1 [63, 118, 228] 13 0 0x7e00 0xd800
128 127 -930 794 62 1 1 [63, 118, 228] 12 0 0x7e00 0xd800
129 127 -928 794 66 0 1 [134, 183, 131] 12 0 0x8200 0x5c00
130 127 -926 794 74 0 1 [97, 119, 97] 11 0 0x8a00 0x3100
131 127 -924 794 77 0 1 [97, 119, 97] 12 0 0x8d00 0x3100
132 127 -922 794 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x3100
133 127 -920 794 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3800
134 127 -918 794 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x8800
122 128 -942 796 68 0 1 [134, 183, 131] 15 0 0x8400 0x5c00
123 128 -940 796 72 0 1 [97, 119, 97] 13 0 0x8800 0x3100
124 128 -938 796 75 0 1 [97, 119, 97] 14 0 0x8b00 0x3100
125 128 -936 796 72 0 1 [97, 119, 97] 14 0 0x8800 0x3100
126 128 -934 796 62 1 1 [63, 118, 228] 13 0 0x7e00 0xd800
127 128 -932 796 62 1 1 [63, 118, 228] 13 0 0x7e00 0xd800
128 128 -930 796 62 1 1 [63, 118, 228] 12 0 0x7e00 0xd800
129 128 -928 796 66 0 1 [134, 183, 131] 12 0 0x8200 0x5c00
130 128 -926 796 68 0 1 [134, 183, 131] 11 0 0x8400 0x5c00
131 128 -924 796 74 0 1 [97, 119, 97] 12 0 0x8a00 0x3100
132 128 -922 796 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 128 -920 796 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x3100
134 128 -918 796 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
122 129 -942 798 69 0 1 [134, 183, 131] 15 0 0x8500 0x5c00
123 129 -940 798 75 0 1 [97, 119, 97] 13 0 0x8b00 0x3100
124 129 -938 798 78 0 1 [97, 119, 97] 13 0 0x8e00 0x3100
125 129 -936 798 75 0 1 [97, 119, 97] 4 0 0x8b00 0x3100
126 129 -934 798 62 1 1 [63, 118, 228] 13 0 0x7e00 0xd800
127 129 -932 798 62 1 1 [63, 118, 228] 13 0 0x7e00 0xd800
128 129 -930 798 62 1 1 [63, 118, 228] 12 0 0x7e00 0xd800
129 129 -928 798 71 0 1 [97, 119, 97] 1 0 0x8700 0x3100
130 129 -926 798 71 0 1 [97, 119, 97] 11 0 0x8700 0x3100
131 129 -924 798 69 0 1 [134, 183, 131] 11 0 0x8500 0x5c00
132 129 -922 798 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x3100
133 129 -920 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
134 129 -918 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
122 130 -942 800 70 0 1 [134, 183, 131] 15 0 0x8600 0x5c00
123 130 -940 800 76 0 1 [97, 119, 97] 13 0 0x8c00 0x3100
124 130 -938 800 76 0 1 [97, 119, 97] 14 0 0x8c00 0x3100
125 130 -936 800 72 0 1 [97, 119, 97] 4 0 0x8800 0x3100
126 130 -934 800 62 1 1 [63, 118, 228] 13 0 0x7e00 0xd800
127 130 -932 800 62 1 1 [63, 118, 228] 13 0 0x7e00 0xd800
128 130 -930 800 62 1 1 [63, 118, 228] 12 0 0x7e00 0xd800
129 130 -928 800 71 0 1 [97, 119, 97] 0 0 0x8700 0x3100
130 130 -926 800 71 0 1 [97, 119, 97] 11 0 0x8700 0x3100
131 130 -924 800 69 0 1 [134, 183, 131] 11 0 0x8500 0x5c00
132 130 -922 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
133 130 -920 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
134 130 -918 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
122 131 -942 802 78 0 1 [97, 119, 97] 3 0 0x8e00 0x3100
123 131 -940 802 77 0 1 [97, 119, 97] 13 0 0x8d00 0x3100
124 131 -938 802 77 0 1 [97, 119, 97] 14 0 0x8d00 0x3100
125 131 -936 802 66 0 1 [134, 183, 131] 14 0 0x8200 0x5c00
126 131 -934 802 62 1 1 [63, 118, 228] 13 0 0x7e00 0xd800
127 131 -932 802 62 1 1 [63, 118, 228] 13 0 0x7e00 0xd800
128 131 -930 802 62 1 1 [63, 118, 228] 12 0 0x7e00 0xd800
129 131 -928 802 62 1 1 [63, 118, 228] 12 0 0x7e00 0xd800
130 131 -926 802 66 0 1 [134, 183, 131] 11 0 0x8200 0x5c00
131 131 -924 802 68 0 1 [134, 183, 131] 11 0 0x8400 0x5c00
132 131 -922 802 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 131 -920 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
134 131 -918 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
122 132 -942 804 79 0 1 [97, 119, 97] 6 0 0x8f00 0x3100
123 132 -940 804 76 0 1 [97, 119, 97] 13 0 0x8c00 0x3100
124 132 -938 804 76 0 1 [97, 119, 97] 14 0 0x8c00 0x3100
125 132 -936 804 76 0 1 [97, 119, 97] 4 0 0x8c00 0x3100
126 132 -934 804 62 1 1 [63, 118, 228] 13 0 0x7e00 0xd800
127 132 -932 804 62 1 1 [63, 118, 228] 13 0 0x7e00 0xd800
128 132 -930 804 62 1 1 [63, 118, 228] 12 0 0x7e00 0xd800
129 132 -928 804 62 1 1 [63, 118, 228] 12 0 0x7e00 0xd800
130 132 -926 804 65 0 1 [134, 183, 131] 11 0 0x8100 0x5c00
131 132 -924 804 77 0 1 [97, 119, 97] 11 0 0x8d00 0x3100
132 132 -922 804 79 0 1 [97, 119, 97] 10 0 0x8f00 0x3100
133 132 -920 804 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
134 132 -918 804 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x3100
122 133 -942 806 78 0 1 [97, 119, 97] 6 0 0x8e00 0x3100
123 133 -940 806 69 0 1 [134, 183, 131] 15 0 0x8500 0x5c00
124 133 -938 806 155 0 1 [134, 183, 131] 15 0 0xdb00 0x5c00
125 133 -936 806 155 0 1 [134, 183, 131] 15 0 0xdb00 0x5c00
126 133 -934 806 155 0 1 [134, 183, 131] 14 0 0xdb00 0x5c00
127 133 -932 806 155 0 1 [134, 183, 131] 13 0 0xdb00 0x5c00
128 133 -930 806 155 0 1 [134, 183, 131] 13 0 0xdb00 0x5c00
129 133 -928 806 155 0 1 [134, 183, 131] 12 0 0xdb00 0x5c00
130 133 -926 806 155 0 1 [134, 183, 131] 12 0 0xdb00 0x5c00
131 133 -924 806 67 0 1 [134, 183, 131] 11 0 0x8300 0x5c00
132 133 -922 806 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 133 -920 806 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
134 133 -918 806 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x3100
122 134 -942 808 79 0 1 [97, 119, 97] 6 0 0x8f00 0x3100
123 134 -940 808 68 0 1 [134, 183, 131] 14 0 0x8400 0x5c00
124 134 -938 808 155 0 1 [134, 183, 131] 15 0 0xdb00 0x5c00
125 134 -936 808 180 0 1 [134, 183, 131] 18446744073709551615 1 0xf400 0x5c00
126 134 -934 808 180 0 1 [134, 183, 131] 18446744073709551615 1 0xf400 0x5c00
127 134 -932 808 180 0 1 [134, 183, 131] 18446744073709551615 1 0xf400 0x5c00
128 134 -930 808 180 0 1 [134, 183, 131] 18446744073709551615 1 0xf400 0x5c00
129 134 -928 808 180 0 1 [134, 183, 131] 4838 1 0xf400 0x5c00
130 134 -926 808 155 0 1 [134, 183, 131] 12 0 0xdb00 0x5c00
131 134 -924 808 67 0 1 [134, 183, 131] 11 0 0x8300 0x5c00
132 134 -922 808 68 0 1 [134, 183, 131] 18446744073709551615 1 0x8400 0x5c00
133 134 -920 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
134 134 -918 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
```
### stream diag after fill:
[c-LoD] stream: tick 5500 65536/65536 cells, lap 3889, cone 69319 zone 1554 dirty 896 (q 11895) ewin 194 rbeat 9p/1 rearm 0c/0t rescan 1429746c/2105e, edits +6019p/4544c/669e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 328 forced 184, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 1108381p/3620b fail 0 ~2 kB/s cache 13255/24576 h1095126 m13255 e0, srbeat 67529s/9q drop 27stale/0undec, ereb 47316s/9q
[11:30:44 INFO]: [c-LoD] stream: tick 5500 65536/65536 cells, lap 3889, cone 69319 zone 1554 dirty 896 (q 11895) ewin 194 rbeat 9p/1 rearm 0c/0t rescan 1429746c/2105e, edits +6019p/4544c/669e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 328 forced 184, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 1108381p/3620b fail 0 ~2 kB/s cache 13255/24576 h1095126 m13255 e0, srbeat 67529s/9q drop 27stale/0undec, ereb 47316s/9q
- carrier ladder y=155: 18568 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=165: 18568 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=168: 18568 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=172: 18568 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=176: 18568 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=180: 18568 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=185: 18568 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- probe 0: red=0 paint=4297 lod_silhouette_cols=37 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 1: red=0 paint=4297 lod_silhouette_cols=37 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 2: red=0 paint=4678 lod_silhouette_cols=37 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 3: red=0 paint=4557 lod_silhouette_cols=37 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 4: red=0 paint=4632 lod_silhouette_cols=36 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 5: red=0 paint=4472 lod_silhouette_cols=37 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 6: red=0 paint=6162 lod_silhouette_cols=37 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 7: red=0 paint=6110 lod_silhouette_cols=36 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 8: red=0 paint=5897 lod_silhouette_cols=37 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 9: red=0 paint=5848 lod_silhouette_cols=37 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 10: red=0 paint=5848 lod_silhouette_cols=37 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 11: red=0 paint=5848 lod_silhouette_cols=37 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 12: red=0 paint=5848 lod_silhouette_cols=37 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 13: red=0 paint=6003 lod_silhouette_cols=37 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 14: red=0 paint=5942 lod_silhouette_cols=37 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 15: red=0 paint=6046 lod_silhouette_cols=37 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 16: red=0 paint=6332 lod_silhouette_cols=37 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 17: red=0 paint=6458 lod_silhouette_cols=36 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 18: red=0 paint=6363 lod_silhouette_cols=37 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 19: red=0 paint=6423 lod_silhouette_cols=36 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 20: red=0 paint=6325 lod_silhouette_cols=37 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 21: red=0 paint=6511 lod_silhouette_cols=37 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 22: red=0 paint=7884 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 23: red=0 paint=8024 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 24: red=0 paint=7734 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 25: red=0 paint=7953 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 26: red=0 paint=7784 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 27: red=0 paint=7944 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 28: red=0 paint=7878 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 29: red=0 paint=7993 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 30: red=0 paint=7805 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 31: red=0 paint=8072 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 32: red=0 paint=7911 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 33: red=0 paint=8120 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 34: red=0 paint=7930 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 35: red=0 paint=8055 lod_silhouette_cols=40 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- phase: 5 leg-1 probes done (11:36:40)
- **FAIL:** leg-1 never saw the marker in 24 probes (red < 60 everywhere) — the edit never reached the LOD paint; this failure was SILENT before v6.31.1
### stream diag after probes:
[11:36:25 INFO]: [c-LoD] stream: tick 8900 65536/65536 cells, lap 6436, cone 121799 zone 1844 dirty 937 (q 11926) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 2528398c/2168e, edits +7406p/5502c/958e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 328 forced 184, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 1654542p/5059b fail 0 ~123 kB/s cache 13266/24576 h1641276 m13266 e0, srbeat 83581s/511q drop 33stale/0undec, ereb 48558s/0q
[c-LoD] stream: tick 9000 65536/65536 cells, lap 6536, cone 123710 zone 1844 dirty 937 (q 11926) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 2564065c/2169e, edits +7407p/5503c/959e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 328 forced 184, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 1674145p/5116b fail 0 ~103 kB/s cache 13266/24576 h1660879 m13266 e0, srbeat 84219s/453q drop 33stale/0undec, ereb 48558s/0q
[11:36:35 INFO]: [c-LoD] stream: tick 9000 65536/65536 cells, lap 6536, cone 123710 zone 1844 dirty 937 (q 11926) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 2564065c/2169e, edits +7407p/5503c/959e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 328 forced 184, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 1674145p/5116b fail 0 ~103 kB/s cache 13266/24576 h1660879 m13266 e0, srbeat 84219s/453q drop 33stale/0undec, ereb 48558s/0q
- parity debug tour: 13 vantages (session-1 client, DEBUG pack — evidence only, the gate pairs the §7b world session)
- parity debug tour done: 13 shots in parity_dbg/ (the §7b world session feeds the gate)
- joined session 2 (11:39:33)
- leg-2 baseline scene px (x<940): 22890 (0 = the relog client paints no world at all)
- leg-2 renderer probe NEAR (20 m) white px: 0 raw / 0 changed-vs-baseline (> 20 = the client renders particles)
- leg-2 renderer probe FAR (80 m, marker zone) white px: 438 raw / 431 changed-vs-baseline (> 20 = no distance cut; ~0 with NEAR alive = particles beyond the client chunk radius do not render)
- leg-2 carrier probe green dust (80 m): changed-green px 0 / total changed 3179 vs baseline (> 100 green = a stable green carrier exists; total > 0 with green ~0 = the dust renders but fog washes it — a ruler problem, not a carrier problem)
- phase: 8 leg-2 relog probes (11:42:24)
- relog probe 0: red=0 scene=23158 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 1: red=0 scene=23096 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 2: red=0 scene=25623 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 3: red=0 scene=25515 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 4: red=0 scene=25611 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 5: red=0 scene=26487 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 6: red=0 scene=26608 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 7: red=0 scene=26605 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 8: red=0 scene=26700 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 9: red=0 scene=31211 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 10: red=0 scene=31952 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 11: red=0 scene=34831 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 12: red=0 scene=34833 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 13: red=0 scene=34833 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 14: red=0 scene=34833 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 15: red=0 scene=34833 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 16: red=0 scene=34833 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 17: red=0 scene=34753 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 18: red=0 scene=34643 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 19: red=0 scene=37703 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- **FAIL:** marker lost after relog — stream state is not persistent/deterministic
- phase: 9 leg-2 done (11:45:26)

### client log forensics (pipeline/particle/atlas — v6.8.2)
```
java.lang.IllegalStateException: Failed to open OpenAL device
	at net.minecraft.client.sounds.SoundEngine.reload(SoundEngine.java:110)
	at net.minecraft.server.packs.resources.SimplePreparableReloadListener.lambda$reload$1(SimplePreparableReloadListener.java:15)
	at net.minecraft.server.packs.resources.SimpleReloadInstance.lambda$prepareTasks$3(SimpleReloadInstance.java:59)
[11:39:30] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[11:39:30] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[11:39:30] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[11:39:30] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[11:39:30] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[11:39:30] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[11:39:30] [Render thread/INFO]: Created: 2048x2048x4 minecraft:textures/atlas/blocks.png-atlas
[11:39:30] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[11:39:30] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[11:39:30] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[11:39:30] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[11:39:30] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
[11:39:37] [Render thread/INFO]: Reloading ResourceManager: vanilla, vanilla, server/00000000/6b1f2f7e-9c0d-4a4b-8b0a-b10d1b4ab100
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
[11:39:39] [Render thread/ERROR]: Error starting SoundSystem. Turning off sounds & music
java.lang.IllegalStateException: Failed to open OpenAL device
	at net.minecraft.client.sounds.SoundEngine.reload(SoundEngine.java:110)
	at net.minecraft.server.packs.resources.SimplePreparableReloadListener.lambda$reload$1(SimplePreparableReloadListener.java:15)
	at net.minecraft.server.packs.resources.SimpleReloadInstance.lambda$prepareTasks$3(SimpleReloadInstance.java:59)
[11:39:39] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[11:39:39] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[11:39:39] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[11:39:39] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[11:39:39] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[11:39:39] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[11:39:39] [Render thread/INFO]: Created: 2048x2048x4 minecraft:textures/atlas/blocks.png-atlas
[11:39:39] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[11:39:39] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[11:39:39] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[11:39:39] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[11:39:39] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
```

### /clod-cells AFTER PROBES (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 48 0 0x8400 0x3100
123 122 -940 784 71 0 1 [97, 119, 97] 47 0 0x8700 0x3100
124 122 -938 784 68 0 1 [97, 119, 97] 51 0 0x8400 0x3100
125 122 -936 784 62 1 1 [63, 118, 228] 59 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 57 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 55 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 54 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 54 0 0x8000 0x5c00
130 122 -926 784 66 0 1 [134, 183, 131] 54 0 0x8200 0x5c00
131 122 -924 784 75 0 1 [97, 119, 97] 44 0 0x8b00 0x3100
132 122 -922 784 77 0 1 [97, 119, 97] 4352 0 0x8d00 0x3100
133 122 -920 784 75 0 1 [97, 119, 97] 4352 0 0x8b00 0x3100
134 122 -918 784 73 0 1 [97, 119, 97] 4352 0 0x8900 0x3100
122 123 -942 786 70 0 1 [97, 119, 97] 48 0 0x8600 0x3100
123 123 -940 786 65 0 1 [97, 119, 97] 47 0 0x8100 0x3100
124 123 -938 786 65 0 1 [97, 119, 97] 51 0 0x8100 0x3100
125 123 -936 786 62 1 1 [63, 118, 228] 59 0 0x7e00 0xd800
126 123 -934 786 62 1 1 [63, 118, 228] 57 0 0x7e00 0xd800
127 123 -932 786 62 1 1 [63, 118, 228] 55 0 0x7e00 0xd800
128 123 -930 786 62 1 1 [63, 118, 228] 55 0 0x7e00 0xd800
129 123 -928 786 73 0 1 [97, 119, 97] 46 0 0x8900 0x3100
130 123 -926 786 67 0 1 [134, 183, 131] 54 0 0x8300 0x5c00
131 123 -924 786 69 0 1 [134, 183, 131] 53 0 0x8500 0x5c00
132 123 -922 786 77 0 1 [97, 119, 97] 4352 0 0x8d00 0x3100
133 123 -920 786 77 0 1 [97, 119, 97] 4352 0 0x8d00 0x3100
134 123 -918 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
122 124 -942 788 72 0 1 [97, 119, 97] 48 0 0x8800 0x3100
123 124 -940 788 70 0 1 [97, 119, 97] 47 0 0x8600 0x3100
124 124 -938 788 68 0 1 [97, 119, 97] 51 0 0x8400 0x3100
125 124 -936 788 62 1 1 [63, 118, 228] 59 0 0x7e00 0xd800
126 124 -934 788 62 1 1 [63, 118, 228] 57 0 0x7e00 0xd800
127 124 -932 788 62 1 1 [63, 118, 228] 55 0 0x7e00 0xd800
128 124 -930 788 73 0 1 [97, 119, 97] 55 0 0x8900 0x3100
129 124 -928 788 76 0 1 [97, 119, 97] 46 0 0x8c00 0x3100
130 124 -926 788 73 0 1 [97, 119, 97] 45 0 0x8900 0x3100
131 124 -924 788 69 0 1 [134, 183, 131] 53 0 0x8500 0x5c00
132 124 -922 788 77 0 1 [97, 119, 97] 4352 0 0x8d00 0x3100
133 124 -920 788 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
134 124 -918 788 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x3100
122 125 -942 790 74 0 1 [97, 119, 97] 48 0 0x8a00 0x3100
123 125 -940 790 73 0 1 [97, 119, 97] 47 0 0x8900 0x3100
124 125 -938 790 71 0 1 [97, 119, 97] 51 0 0x8700 0x3100
125 125 -936 790 62 1 1 [63, 118, 228] 59 0 0x7e00 0xd800
126 125 -934 790 62 1 1 [63, 118, 228] 58 0 0x7e00 0xd800
127 125 -932 790 62 1 1 [63, 118, 228] 55 0 0x7e00 0xd800
128 125 -930 790 62 1 1 [63, 118, 228] 55 0 0x7e00 0xd800
129 125 -928 790 73 0 1 [97, 119, 97] 46 0 0x8900 0x3100
130 125 -926 790 68 0 1 [134, 183, 131] 54 0 0x8400 0x5c00
131 125 -924 790 69 0 1 [134, 183, 131] 53 0 0x8500 0x5c00
132 125 -922 790 69 0 1 [134, 183, 131] 4352 0 0x8500 0x5c00
133 125 -920 790 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
134 125 -918 790 70 0 1 [134, 183, 131] 18446744073709551615 1 0x8600 0x5c00
122 126 -942 792 74 0 1 [97, 119, 97] 48 0 0x8a00 0x3100
123 126 -940 792 73 0 1 [97, 119, 97] 47 0 0x8900 0x3100
124 126 -938 792 68 0 1 [97, 119, 97] 50 0 0x8400 0x3100
125 126 -936 792 62 1 1 [63, 118, 228] 59 0 0x7e00 0xd800
126 126 -934 792 62 1 1 [63, 118, 228] 58 0 0x7e00 0xd800
127 126 -932 792 62 1 1 [63, 118, 228] 55 0 0x7e00 0xd800
128 126 -930 792 62 1 1 [63, 118, 228] 55 0 0x7e00 0xd800
129 126 -928 792 67 0 1 [134, 183, 131] 54 0 0x8300 0x5c00
130 126 -926 792 74 0 1 [97, 119, 97] 45 0 0x8a00 0x3100
131 126 -924 792 77 0 1 [97, 119, 97] 44 0 0x8d00 0x3100
132 126 -922 792 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x3100
133 126 -920 792 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3800
134 126 -918 792 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x8800
122 127 -942 794 67 0 1 [134, 183, 131] 56 0 0x8300 0x5c00
123 127 -940 794 68 0 1 [134, 183, 131] 56 0 0x8400 0x5c00
124 127 -938 794 65 0 1 [134, 183, 131] 55 0 0x8100 0x5c00
125 127 -936 794 62 1 1 [63, 118, 228] 0 0 0x7e00 0xd800
126 127 -934 794 62 1 1 [63, 118, 228] 58 0 0x7e00 0xd800
127 127 -932 794 170 0 1 [142, 33, 33] 55 0 0xea00 0x0200
128 127 -930 794 170 0 1 [142, 33, 33] 55 0 0xea00 0x0200
129 127 -928 794 170 0 1 [142, 33, 33] 46 0 0xea00 0x0200
130 127 -926 794 74 0 1 [97, 119, 97] 45 0 0x8a00 0x3100
131 127 -924 794 77 0 1 [97, 119, 97] 43 0 0x8d00 0x3100
132 127 -922 794 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x3100
133 127 -920 794 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3800
134 127 -918 794 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x8800
122 128 -942 796 68 0 1 [134, 183, 131] 56 0 0x8400 0x5c00
123 128 -940 796 72 0 1 [97, 119, 97] 47 0 0x8800 0x3100
124 128 -938 796 75 0 1 [97, 119, 97] 50 0 0x8b00 0x3100
125 128 -936 796 72 0 1 [97, 119, 97] 52 0 0x8800 0x3100
126 128 -934 796 62 1 1 [63, 118, 228] 58 0 0x7e00 0xd800
127 128 -932 796 170 0 1 [142, 33, 33] 55 0 0xea00 0x0200
128 128 -930 796 170 0 1 [142, 33, 33] 55 0 0xea00 0x0200
129 128 -928 796 170 0 1 [142, 33, 33] 46 0 0xea00 0x0200
130 128 -926 796 68 0 1 [134, 183, 131] 54 0 0x8400 0x5c00
131 128 -924 796 74 0 1 [97, 119, 97] 45 0 0x8a00 0x3100
132 128 -922 796 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 128 -920 796 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x3100
134 128 -918 796 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
122 129 -942 798 69 0 1 [134, 183, 131] 56 0 0x8500 0x5c00
123 129 -940 798 75 0 1 [97, 119, 97] 47 0 0x8b00 0x3100
124 129 -938 798 78 0 1 [97, 119, 97] 57 0 0x8e00 0x3100
125 129 -936 798 75 0 1 [97, 119, 97] 54 0 0x8b00 0x3100
126 129 -934 798 62 1 1 [63, 118, 228] 58 0 0x7e00 0xd800
127 129 -932 798 170 0 1 [142, 33, 33] 55 0 0xea00 0x0200
128 129 -930 798 170 0 1 [142, 33, 33] 55 0 0xea00 0x0200
129 129 -928 798 170 0 1 [142, 33, 33] 46 0 0xea00 0x0200
130 129 -926 798 71 0 1 [97, 119, 97] 45 0 0x8700 0x3100
131 129 -924 798 69 0 1 [134, 183, 131] 53 0 0x8500 0x5c00
132 129 -922 798 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x3100
133 129 -920 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
134 129 -918 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
122 130 -942 800 70 0 1 [134, 183, 131] 56 0 0x8600 0x5c00
123 130 -940 800 76 0 1 [97, 119, 97] 47 0 0x8c00 0x3100
124 130 -938 800 76 0 1 [97, 119, 97] 51 0 0x8c00 0x3100
125 130 -936 800 72 0 1 [97, 119, 97] 54 0 0x8800 0x3100
126 130 -934 800 62 1 1 [63, 118, 228] 58 0 0x7e00 0xd800
127 130 -932 800 62 1 1 [63, 118, 228] 55 0 0x7e00 0xd800
128 130 -930 800 62 1 1 [63, 118, 228] 55 0 0x7e00 0xd800
129 130 -928 800 71 0 1 [97, 119, 97] 46 0 0x8700 0x3100
130 130 -926 800 71 0 1 [97, 119, 97] 45 0 0x8700 0x3100
131 130 -924 800 69 0 1 [134, 183, 131] 53 0 0x8500 0x5c00
132 130 -922 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
133 130 -920 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
134 130 -918 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
122 131 -942 802 78 0 1 [97, 119, 97] 49 0 0x8e00 0x3100
123 131 -940 802 77 0 1 [97, 119, 97] 47 0 0x8d00 0x3100
124 131 -938 802 77 0 1 [97, 119, 97] 51 0 0x8d00 0x3100
125 131 -936 802 66 0 1 [134, 183, 131] 59 0 0x8200 0x5c00
126 131 -934 802 62 1 1 [63, 118, 228] 58 0 0x7e00 0xd800
127 131 -932 802 62 1 1 [63, 118, 228] 55 0 0x7e00 0xd800
128 131 -930 802 62 1 1 [63, 118, 228] 55 0 0x7e00 0xd800
129 131 -928 802 62 1 1 [63, 118, 228] 54 0 0x7e00 0xd800
130 131 -926 802 66 0 1 [134, 183, 131] 54 0 0x8200 0x5c00
131 131 -924 802 68 0 1 [134, 183, 131] 53 0 0x8400 0x5c00
132 131 -922 802 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 131 -920 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
134 131 -918 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
122 132 -942 804 79 0 1 [97, 119, 97] 49 0 0x8f00 0x3100
123 132 -940 804 76 0 1 [97, 119, 97] 47 0 0x8c00 0x3100
124 132 -938 804 76 0 1 [97, 119, 97] 51 0 0x8c00 0x3100
125 132 -936 804 76 0 1 [97, 119, 97] 54 0 0x8c00 0x3100
126 132 -934 804 62 1 1 [63, 118, 228] 57 0 0x7e00 0xd800
127 132 -932 804 62 1 1 [63, 118, 228] 55 0 0x7e00 0xd800
128 132 -930 804 62 1 1 [63, 118, 228] 55 0 0x7e00 0xd800
129 132 -928 804 62 1 1 [63, 118, 228] 54 0 0x7e00 0xd800
130 132 -926 804 65 0 1 [134, 183, 131] 54 0 0x8100 0x5c00
131 132 -924 804 77 0 1 [97, 119, 97] 44 0 0x8d00 0x3100
132 132 -922 804 79 0 1 [97, 119, 97] 46 0 0x8f00 0x3100
133 132 -920 804 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
134 132 -918 804 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x3100
122 133 -942 806 78 0 1 [97, 119, 97] 49 0 0x8e00 0x3100
123 133 -940 806 69 0 1 [134, 183, 131] 56 0 0x8500 0x5c00
124 133 -938 806 155 0 1 [134, 183, 131] 56 0 0xdb00 0x5c00
125 133 -936 806 155 0 1 [134, 183, 131] 56 0 0xdb00 0x5c00
126 133 -934 806 155 0 1 [134, 183, 131] 55 0 0xdb00 0x5c00
127 133 -932 806 155 0 1 [134, 183, 131] 59 0 0xdb00 0x5c00
128 133 -930 806 155 0 1 [134, 183, 131] 56 0 0xdb00 0x5c00
129 133 -928 806 155 0 1 [134, 183, 131] 55 0 0xdb00 0x5c00
130 133 -926 806 155 0 1 [134, 183, 131] 54 0 0xdb00 0x5c00
131 133 -924 806 67 0 1 [134, 183, 131] 53 0 0x8300 0x5c00
132 133 -922 806 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 133 -920 806 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
134 133 -918 806 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x3100
122 134 -942 808 79 0 1 [97, 119, 97] 48 0 0x8f00 0x3100
123 134 -940 808 68 0 1 [134, 183, 131] 56 0 0x8400 0x5c00
124 134 -938 808 155 0 1 [134, 183, 131] 56 0 0xdb00 0x5c00
125 134 -936 808 180 0 1 [134, 183, 131] 4939 0 0xf400 0x5c00
126 134 -934 808 180 0 1 [134, 183, 131] 4939 0 0xf400 0x5c00
127 134 -932 808 180 0 1 [134, 183, 131] 4939 0 0xf400 0x5c00
128 134 -930 808 180 0 1 [134, 183, 131] 4939 0 0xf400 0x5c00
129 134 -928 808 180 0 1 [134, 183, 131] 4938 0 0xf400 0x5c00
130 134 -926 808 155 0 1 [134, 183, 131] 54 0 0xdb00 0x5c00
131 134 -924 808 67 0 1 [134, 183, 131] 53 0 0x8300 0x5c00
132 134 -922 808 68 0 1 [134, 183, 131] 18446744073709551615 1 0x8400 0x5c00
133 134 -920 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
134 134 -918 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
```
- edit channel: 0 linkage errors, 0 failed event dispatches (both must be 0)
- phase: 6 parity world boot (11:45:27)
- parity world: view-distance=8 (the voxy leg's server value)
- parity world: joined session 3 (11:46:08)
- parity world: scan populated (cells=65536; the march paints from these buffers)
- parity world tour: 13 vantages (production pack, vd 8, shared structures)
- coverage sweeps: 72 eye passes = 36 positions (12 az x r144/200 + 12 off-az x r172, y=160) x 2 looks (outward+inward), settle-by-laps >= 6 s (q stable 3x1s), deadline 12 s/pos, no shots — the v6.26.0 data-coverage lever + v6.26.6 lottery kill; probe rides parity/view_probe.md
- parity drain-settle TIMEOUT: queue 21685 -> 21685 > 11500 after 150s — continuing anyway; the queue did not drain without eye proximity (priority-lane evidence, the 01/05/10 dip attribution reads from the tour deltas)
- pre-tour pass done: 13 vantages x 10s (the tour now reads the second OR-merge lap; the rim/dip transport race reads from the tour deltas)
- parity world tour done: 13 shots, 0 night-guard renames in parity/ (the gate pairs voxy/<name>.png vs parity/<name>.png)
- phase: 7 parity tour done (12:08:03)
- sparse re-beat channel at tour end: streamed=84839 q=0 dropped_stale=23 dropped_undecodable=13969 (frac=8 boot; undecodable-dominant = the rim words die in their decodability window, stale-dominant = edit churn owns the cells)
### parity world stream counters (session 3, tail + maxima)
[c-LoD] stream: tick 13200 65536/65536 cells, lap 8763, cone 146743 zone 24283 dirty 9721 (q 21683) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 9019552c/1722e, edits +18050p/13678c/2740e drop 0, spawned 0 / failed 0, cam0 yaw -180 pitch 5, wish 0 forced 905, native 34244c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 2051533p/5462b fail 0 ~0 kB/s cache 24067/24576 h1626004 m425529 e401462, srbeat 84839s/0q drop 23stale/13969undec, ereb 38479s/4q
[12:07:58 INFO]: [c-LoD] stream: tick 13200 65536/65536 cells, lap 8763, cone 146743 zone 24283 dirty 9721 (q 21683) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 9019552c/1722e, edits +18050p/13678c/2740e drop 0, spawned 0 / failed 0, cam0 yaw -180 pitch 5, wish 0 forced 905, native 34244c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 2051533p/5462b fail 0 ~0 kB/s cache 24067/24576 h1626004 m425529 e401462, srbeat 84839s/0q drop 23stale/13969undec, ereb 38479s/4q
- parity stream max cone: 146743
- parity stream max zone: 24283
- parity stream max wire bundle: bundle
### parity world capture lens (debug chain on the §7b world)
lens_red=0 lens_black=0 lens_water=0 lens_dark=0 lens_zero=0 rot=none scan=0 carriers=0
- parity mesh probe @png_y=2: x306=(151,184,255) x307=(151,184,255) x308=(151,184,255) x309h=(151,184,255) x310c=(151,184,255)
- parity mesh probe @png_y=717: x306=(41,59,48) x307=(44,62,50) x308=(44,61,50) x309h=(45,64,52) x310c=(46,65,52)
- parity mesh engagement: magenta-ish=48 of 921600 sampled px
- mask8: the un-fog engagement masks shot on the PARITY WORLD (vd 8) — parity_dbg8/01m|07m|10m
- mask8 01m: green=464 red=0 blue=98352 dim=6848 (px, sampled /16)
- mask8 07m: green=11920 red=0 blue=221296 dim=5744 (px, sampled /16)
- mask8 10m: green=3936 red=192 blue=213216 dim=223872 (px, sampled /16)
- mesh probe @png_y=2: x306=(255,255,255) x307=(13,254,0) x308=(0,254,0) x309h=(68,0,0) x310c=(0,0,0)
- mesh probe @png_y=717: x306=(255,255,255) x307=(13,254,0) x308=(0,254,0) x309h=(68,0,0) x310c=(0,0,0)
- mesh engagement: magenta-ish=0 of 921600 sampled px (the vanilla-path paint coverage)
- mask8 fog state @png_y=2: raw (300)=(151,184,255) (301)=(151,184,255)
- mask8 fog state @png_y=717: raw (300)=(0,128,8) (301)=(179,206,255)
- mask8 fog state decoded: envStart=0 envEnd=1024 rdEnd=128 fogColor=(179,206,255)
- buffer dump 10m: marked h=20/2048 c=0/2048; height windows r0<142:10 r1:1 r2>=202:9 maxByte=225
- buffer dump 10m: RANK-2-WINDOW HEIGHTS (the fake-wall class): (+2,-24)h=159, (+6,-24)h=159, (+6,-20)h=159, (+0,-18)h=158, (+8,-2)h=143, (+0,+24)h=144, (+4,+24)h=150, (+2,+26)h=151, (-2,+28)h=161
- buffer dump 10m: color idx0(black)=0 water/height mismatches=0 of 0 pairs
- buffer dump 01m: marked h=0/2048 c=0/2048; height windows r0<142:0 r1:0 r2>=202:0 maxByte=0
- buffer dump 01m: color idx0(black)=0 water/height mismatches=0 of 0 pairs
- rim mask: scan populated (cells=65536; the state paint reads the same buffers the march reads)
- phase: 10 rim mask lane (12:12:34)
- rim mask: decode-state frames shot (rim_mask/00|01|12 — GREEN full decode / RED colorless / BLUE heightless; 00 = the leg-1 tower eye, 01 = the near-tower contrast, 12 = the tower re-read; the inherited 07/12 vantages sat 760 blocks outside the streamed disk and 01 sat INSIDE the marker column — their earlier verdicts were classifier noise, purged in v6.33.1)
- rim mask 00: green=9248 red=93184 blue=95120 yellow=12688 (px, sampled /16) -> DOMINANT BLUE: rim = HEIGHTS NEVER DECODED — a data-coverage class: the state paint cannot even see the cells
- rim mask 01: green=19472 red=130032 blue=132416 yellow=14960 (px, sampled /16) -> DOMINANT BLUE: rim = HEIGHTS NEVER DECODED — a data-coverage class: the state paint cannot even see the cells
- rim mask 12: green=18064 red=106240 blue=113632 yellow=20016 (px, sampled /16) -> DOMINANT BLUE: rim = HEIGHTS NEVER DECODED — a data-coverage class: the state paint cannot even see the cells

### boot / stream
```
[c-LoD] [12:11:45 INFO]: [c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 127551 bytes, sha1 c7293c2a24265e83ebf9025127549657b5234c3b — zero textures, zero world data, one push per join
[12:11:45 INFO]: [c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
c-LoD pack finalized (ONE-TIME, static): shader-only, 127551 bytes, sha1 c7293c2a24265e83ebf9025127549657b5234c3b — zero textures, zero world data, one push per join
[c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[c-LoD] stream: tick 900 65536/65536 cells, lap 734, cone 13834 zone 855 dirty 3 (q 1023) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 519575c/63e, edits +8p/8c/2e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 34 forced 880, native 1296c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 130534p/476b fail 0 ~75 kB/s cache 13955/24576 h116579 m13955 e0, srbeat 4244s/910q drop 0stale/817undec, ereb 87s/3q
[12:13:24 INFO]: [c-LoD] stream: tick 900 65536/65536 cells, lap 734, cone 13834 zone 855 dirty 3 (q 1023) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 519575c/63e, edits +8p/8c/2e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 34 forced 880, native 1296c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 130534p/476b fail 0 ~75 kB/s cache 13955/24576 h116579 m13955 e0, srbeat 4244s/910q drop 0stale/817undec, ereb 87s/3q
```
### sessions
```
[12:12:09 INFO]: System chat: Stepan joined the game
[12:13:29 INFO]: System chat: Stepan left the game
```
### client log tail
```
[12:13:29] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[12:13:29] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[12:13:29] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[12:13:29] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[12:13:29] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[12:13:29] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[12:13:29] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[12:13:29] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[12:13:29] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[12:13:29] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[12:13:29] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[12:13:29] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[12:13:29] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[12:13:29] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[12:13:29] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[12:13:29] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[12:13:29] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[12:13:29] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[12:13:29] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[12:13:29] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[12:13:29] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[12:13:29] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[12:13:29] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[12:13:29] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[12:13:29] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[12:13:29] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[12:13:29] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[12:13:29] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[12:13:29] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[12:13:29] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:bundle_delimiter] net.minecraft.network.protocol.game.ClientboundBundleDelimiterPacket -> 1 bytes
```
### server log tail
```
[c-LoD] EB: enter, native_gen=true
[12:13:30 INFO]: [ChunkHolderManager] Saved 999 block chunks, 975 entity chunks, 0 poi chunks in world 'minecraft:overworld' in 1.03s
[12:13:30 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:overworld'
[12:13:30 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:overworld'
[12:13:30 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[12:13:30 INFO]: [ChunkHolderManager] Waiting 60s for chunk system to halt for world 'minecraft:the_nether'
[12:13:30 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:the_nether'
[12:13:30 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_nether'
[12:13:30 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_nether' in 0.00s
[12:13:30 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_nether'
[12:13:30 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_nether'
[12:13:30 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[12:13:30 INFO]: [ChunkHolderManager] Waiting 60s for chunk system to halt for world 'minecraft:the_end'
[12:13:30 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:the_end'
[12:13:30 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_end'
[12:13:30 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_end' in 0.00s
[12:13:30 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_end'
[12:13:30 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_end'
[12:13:30 INFO]: Waiting for all RegionFile I/O tasks to complete...
[12:13:30 INFO]: All RegionFile I/O tasks to complete
[12:13:30 INFO]: [MoonriseCommon] Awaiting termination of worker pool for up to 60s...
[12:13:30 INFO]: [MoonriseCommon] Awaiting termination of I/O pool for up to 60s...
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[launcher] server exited with code 143
```
### shots
```
-rw-r--r-- 1 runner runner   6527 Nov 14 11:44 lens_13_after_relog_probe11.png
-rw-r--r-- 1 runner runner   6531 Nov 14 11:44 lens_13_after_relog_probe12.png
-rw-r--r-- 1 runner runner   6531 Nov 14 11:44 lens_13_after_relog_probe13.png
-rw-r--r-- 1 runner runner   6531 Nov 14 11:44 lens_13_after_relog_probe14.png
-rw-r--r-- 1 runner runner   6531 Nov 14 11:44 lens_13_after_relog_probe15.png
-rw-r--r-- 1 runner runner   6531 Nov 14 11:44 lens_13_after_relog_probe16.png
-rw-r--r-- 1 runner runner   6664 Nov 14 11:45 lens_13_after_relog_probe17.png
-rw-r--r-- 1 runner runner   7231 Nov 14 11:45 lens_13_after_relog_probe18.png
-rw-r--r-- 1 runner runner   7242 Nov 14 11:45 lens_13_after_relog_probe19.png
-rw-r--r-- 1 runner runner   5958 Nov 14 11:42 lens_13_after_relog_probe2.png
-rw-r--r-- 1 runner runner   6112 Nov 14 11:42 lens_13_after_relog_probe3.png
-rw-r--r-- 1 runner runner   5993 Nov 14 11:43 lens_13_after_relog_probe4.png
-rw-r--r-- 1 runner runner   7468 Nov 14 11:43 lens_13_after_relog_probe5.png
-rw-r--r-- 1 runner runner   7300 Nov 14 11:43 lens_13_after_relog_probe6.png
-rw-r--r-- 1 runner runner   7078 Nov 14 11:43 lens_13_after_relog_probe7.png
-rw-r--r-- 1 runner runner   6696 Nov 14 11:43 lens_13_after_relog_probe8.png
-rw-r--r-- 1 runner runner   7165 Nov 14 11:43 lens_13_after_relog_probe9.png
drwxr-xr-x 2 runner runner   4096 Nov 14 12:08 lens_parity
drwxr-xr-x 2 runner runner   4096 Nov 14 12:08 parity
drwxr-xr-x 2 runner runner   4096 Nov 14 11:39 parity_dbg
drwxr-xr-x 2 runner runner   4096 Nov 14 12:11 parity_dbg8
-rw-r--r-- 1 runner runner  10385 Nov 14 11:45 probe_counts.txt
drwxr-xr-x 2 runner runner   4096 Nov 14 12:13 rim_mask
-rw-r--r-- 1 runner runner  60232 Nov 14 12:13 server_full.log
-rw-r--r-- 1 runner runner  59727 Nov 14 12:13 summary.md
```

**exit=1** failures=2 leg1=0 leg2=0
