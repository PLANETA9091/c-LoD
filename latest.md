- phase: 0 sanity (02:32:15)
### c-LoD visual run (-930,796, marker=red_concrete, vantage -1005,150 yaw 270)
- GL:     Vendor: Mesa (0xffffffff);    Device: llvmpipe (LLVM 21.1.8, 256 bits) (0xffffffff);OpenGL renderer string: llvmpipe (LLVM 21.1.8, 256 bits);OpenGL core profile version string: 4.5 (Core Profile) Mesa 26.0.8-1ubuntu0.3;
- boot + gates OK (02:32:47)
- phase: 1 server booted (02:32:47)
- sky-grid armed
- phase: 2 leg-1 client joined (02:33:14)
- joined session 1 (02:33:14)
- stream live: cells=65536
- phase: 3 scan populated (02:33:38)
- terrain range: 2 terraced mountains at (CX-4,CZ-20)/(CX-2,CZ+16), tops 185/190, grass terraces + stone caps, 66..82 m from the eye (inside the 83.5 m band, chunk edge 64 m)
- scan complete=1 (cells=cells=65536/65536)
- paint settled=1 (wire 479450p)
- baseline try 1: drift=397 px
- lod_silhouette_cols=37 (LOD paint ON SCREEN at 75 m if > 20 — the march-drawn marker column; vanilla cannot render there, chunk edge 64 m; #255 re-measures 49, the ZONE_FAR=48 regression #256 and the control read 0)
- 11 terrain-green px in the sky region (LOD MOUNTAINS on screen if > 5000; the range paints ColLow-green on low terraces via the fallback palette; flat-world baselines and the pack-free control read ~0)
- burst_close white px: 0 (renderer alive if > 50)
- burst_far white px: 418 (renderer alive in the marker zone if > 20)
- cut walk 40 m: 72 white px (> 20 = rendered)
- cut walk 60 m: 0 white px (> 20 = rendered)
- cut walk 100 m: 0 white px (> 20 = rendered)
- measured far-render cut: 40 m (G4 flap instrument; the decode proof needs >= 83 m from the y=151.6 eye)
- burst_close retry white px: 546 (preflight passes if > 50)
- fill issued: 5x111x5 red_concrete at -930,796 (zone forceloaded ±48)
- phase: 4 marker filled (02:38:49)

### /clod-cells AFTER FILL (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 53 0 0x8400 0x3100
123 122 -940 784 71 0 1 [97, 119, 97] 54 0 0x8700 0x3100
124 122 -938 784 68 0 1 [97, 119, 97] 44 0 0x8400 0x3100
125 122 -936 784 62 1 1 [63, 118, 228] 53 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 52 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 52 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 51 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 51 0 0x8000 0x5c00
130 122 -926 784 66 0 1 [134, 183, 131] 50 0 0x8200 0x5c00
131 122 -924 784 75 0 1 [97, 119, 97] 41 0 0x8b00 0x3100
132 122 -922 784 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 122 -920 784 75 0 1 [97, 119, 97] 18446744073709551615 1 0x8b00 0x3100
134 122 -918 784 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x3100
122 123 -942 786 70 0 1 [97, 119, 97] 53 0 0x8600 0x3100
123 123 -940 786 65 0 1 [97, 119, 97] 54 0 0x8100 0x3100
124 123 -938 786 65 0 1 [97, 119, 97] 44 0 0x8100 0x3100
125 123 -936 786 62 1 1 [63, 118, 228] 53 0 0x7e00 0xd800
126 123 -934 786 62 1 1 [63, 118, 228] 52 0 0x7e00 0xd800
127 123 -932 786 62 1 1 [63, 118, 228] 52 0 0x7e00 0xd800
128 123 -930 786 62 1 1 [63, 118, 228] 51 0 0x7e00 0xd800
129 123 -928 786 73 0 1 [97, 119, 97] 52 0 0x8900 0x3100
130 123 -926 786 67 0 1 [134, 183, 131] 51 0 0x8300 0x5c00
131 123 -924 786 69 0 1 [134, 183, 131] 50 0 0x8500 0x5c00
132 123 -922 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 123 -920 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
134 123 -918 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
122 124 -942 788 72 0 1 [97, 119, 97] 53 0 0x8800 0x3100
123 124 -940 788 70 0 1 [97, 119, 97] 54 0 0x8600 0x3100
124 124 -938 788 68 0 1 [97, 119, 97] 44 0 0x8400 0x3100
125 124 -936 788 62 1 1 [63, 118, 228] 53 0 0x7e00 0xd800
126 124 -934 788 62 1 1 [63, 118, 228] 52 0 0x7e00 0xd800
127 124 -932 788 62 1 1 [63, 118, 228] 52 0 0x7e00 0xd800
128 124 -930 788 73 0 1 [97, 119, 97] 51 0 0x8900 0x3100
129 124 -928 788 76 0 1 [97, 119, 97] 52 0 0x8c00 0x3100
130 124 -926 788 73 0 1 [97, 119, 97] 42 0 0x8900 0x3100
131 124 -924 788 69 0 1 [134, 183, 131] 50 0 0x8500 0x5c00
132 124 -922 788 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 124 -920 788 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
134 124 -918 788 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x3100
122 125 -942 790 74 0 1 [97, 119, 97] 53 0 0x8a00 0x3100
123 125 -940 790 73 0 1 [97, 119, 97] 54 0 0x8900 0x3100
124 125 -938 790 71 0 1 [97, 119, 97] 44 0 0x8700 0x3100
125 125 -936 790 62 1 1 [63, 118, 228] 53 0 0x7e00 0xd800
126 125 -934 790 62 1 1 [63, 118, 228] 53 0 0x7e00 0xd800
127 125 -932 790 62 1 1 [63, 118, 228] 52 0 0x7e00 0xd800
128 125 -930 790 62 1 1 [63, 118, 228] 52 0 0x7e00 0xd800
129 125 -928 790 73 0 1 [97, 119, 97] 52 0 0x8900 0x3100
130 125 -926 790 68 0 1 [134, 183, 131] 51 0 0x8400 0x5c00
131 125 -924 790 69 0 1 [134, 183, 131] 50 0 0x8500 0x5c00
132 125 -922 790 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
133 125 -920 790 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
134 125 -918 790 70 0 1 [134, 183, 131] 18446744073709551615 1 0x8600 0x5c00
122 126 -942 792 74 0 1 [97, 119, 97] 53 0 0x8a00 0x3100
123 126 -940 792 73 0 1 [97, 119, 97] 54 0 0x8900 0x3100
124 126 -938 792 68 0 1 [97, 119, 97] 44 0 0x8400 0x3100
125 126 -936 792 62 1 1 [63, 118, 228] 53 0 0x7e00 0xd800
126 126 -934 792 62 1 1 [63, 118, 228] 53 0 0x7e00 0xd800
127 126 -932 792 62 1 1 [63, 118, 228] 52 0 0x7e00 0xd800
128 126 -930 792 62 1 1 [63, 118, 228] 52 0 0x7e00 0xd800
129 126 -928 792 67 0 1 [134, 183, 131] 51 0 0x8300 0x5c00
130 126 -926 792 74 0 1 [97, 119, 97] 42 0 0x8a00 0x3100
131 126 -924 792 77 0 1 [97, 119, 97] 41 0 0x8d00 0x3100
132 126 -922 792 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x3100
133 126 -920 792 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3800
134 126 -918 792 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x8800
122 127 -942 794 67 0 1 [134, 183, 131] 54 0 0x8300 0x5c00
123 127 -940 794 68 0 1 [134, 183, 131] 54 0 0x8400 0x5c00
124 127 -938 794 65 0 1 [134, 183, 131] 53 0 0x8100 0x5c00
125 127 -936 794 62 1 1 [63, 118, 228] 53 0 0x7e00 0xd800
126 127 -934 794 62 1 1 [63, 118, 228] 53 0 0x7e00 0xd800
127 127 -932 794 170 0 1 [142, 33, 33] 52 1 0xea00 0x0200
128 127 -930 794 170 0 1 [142, 33, 33] 52 1 0xea00 0x0200
129 127 -928 794 170 0 1 [142, 33, 33] 51 1 0xea00 0x0200
130 127 -926 794 74 0 1 [97, 119, 97] 42 0 0x8a00 0x3100
131 127 -924 794 77 0 1 [97, 119, 97] 41 0 0x8d00 0x3100
132 127 -922 794 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x3100
133 127 -920 794 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3800
134 127 -918 794 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x8800
122 128 -942 796 68 0 1 [134, 183, 131] 54 0 0x8400 0x5c00
123 128 -940 796 72 0 1 [97, 119, 97] 54 0 0x8800 0x3100
124 128 -938 796 75 0 1 [97, 119, 97] 45 0 0x8b00 0x3100
125 128 -936 796 72 0 1 [97, 119, 97] 44 0 0x8800 0x3100
126 128 -934 796 62 1 1 [63, 118, 228] 53 0 0x7e00 0xd800
127 128 -932 796 170 0 1 [142, 33, 33] 52 1 0xea00 0x0200
128 128 -930 796 170 0 1 [142, 33, 33] 52 1 0xea00 0x0200
129 128 -928 796 170 0 1 [142, 33, 33] 51 1 0xea00 0x0200
130 128 -926 796 68 0 1 [134, 183, 131] 51 0 0x8400 0x5c00
131 128 -924 796 74 0 1 [97, 119, 97] 41 0 0x8a00 0x3100
132 128 -922 796 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 128 -920 796 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x3100
134 128 -918 796 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
122 129 -942 798 69 0 1 [134, 183, 131] 54 0 0x8500 0x5c00
123 129 -940 798 75 0 1 [97, 119, 97] 54 0 0x8b00 0x3100
124 129 -938 798 78 0 1 [97, 119, 97] 48 0 0x8e00 0x3100
125 129 -936 798 75 0 1 [97, 119, 97] 42 0 0x8b00 0x3100
126 129 -934 798 62 1 1 [63, 118, 228] 53 0 0x7e00 0xd800
127 129 -932 798 170 0 1 [142, 33, 33] 52 1 0xea00 0x0200
128 129 -930 798 170 0 1 [142, 33, 33] 52 1 0xea00 0x0200
129 129 -928 798 170 0 1 [142, 33, 33] 52 1 0xea00 0x0200
130 129 -926 798 71 0 1 [97, 119, 97] 42 0 0x8700 0x3100
131 129 -924 798 69 0 1 [134, 183, 131] 50 0 0x8500 0x5c00
132 129 -922 798 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x3100
133 129 -920 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
134 129 -918 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
122 130 -942 800 70 0 1 [134, 183, 131] 54 0 0x8600 0x5c00
123 130 -940 800 76 0 1 [97, 119, 97] 54 0 0x8c00 0x3100
124 130 -938 800 76 0 1 [97, 119, 97] 44 0 0x8c00 0x3100
125 130 -936 800 72 0 1 [97, 119, 97] 42 0 0x8800 0x3100
126 130 -934 800 62 1 1 [63, 118, 228] 53 0 0x7e00 0xd800
127 130 -932 800 62 1 1 [63, 118, 228] 52 0 0x7e00 0xd800
128 130 -930 800 62 1 1 [63, 118, 228] 52 0 0x7e00 0xd800
129 130 -928 800 71 0 1 [97, 119, 97] 52 0 0x8700 0x3100
130 130 -926 800 71 0 1 [97, 119, 97] 42 0 0x8700 0x3100
131 130 -924 800 69 0 1 [134, 183, 131] 50 0 0x8500 0x5c00
132 130 -922 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
133 130 -920 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
134 130 -918 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
122 131 -942 802 78 0 1 [97, 119, 97] 46 0 0x8e00 0x3100
123 131 -940 802 77 0 1 [97, 119, 97] 54 0 0x8d00 0x3100
124 131 -938 802 77 0 1 [97, 119, 97] 44 0 0x8d00 0x3100
125 131 -936 802 66 0 1 [134, 183, 131] 53 0 0x8200 0x5c00
126 131 -934 802 62 1 1 [63, 118, 228] 53 0 0x7e00 0xd800
127 131 -932 802 62 1 1 [63, 118, 228] 52 0 0x7e00 0xd800
128 131 -930 802 62 1 1 [63, 118, 228] 52 0 0x7e00 0xd800
129 131 -928 802 62 1 1 [63, 118, 228] 51 0 0x7e00 0xd800
130 131 -926 802 66 0 1 [134, 183, 131] 51 0 0x8200 0x5c00
131 131 -924 802 68 0 1 [134, 183, 131] 50 0 0x8400 0x5c00
132 131 -922 802 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 131 -920 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
134 131 -918 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
122 132 -942 804 79 0 1 [97, 119, 97] 45 0 0x8f00 0x3100
123 132 -940 804 76 0 1 [97, 119, 97] 54 0 0x8c00 0x3100
124 132 -938 804 76 0 1 [97, 119, 97] 44 0 0x8c00 0x3100
125 132 -936 804 76 0 1 [97, 119, 97] 62 0 0x8c00 0x3100
126 132 -934 804 62 1 1 [63, 118, 228] 52 0 0x7e00 0xd800
127 132 -932 804 62 1 1 [63, 118, 228] 52 0 0x7e00 0xd800
128 132 -930 804 62 1 1 [63, 118, 228] 51 0 0x7e00 0xd800
129 132 -928 804 62 1 1 [63, 118, 228] 51 0 0x7e00 0xd800
130 132 -926 804 65 0 1 [134, 183, 131] 51 0 0x8100 0x5c00
131 132 -924 804 77 0 1 [97, 119, 97] 41 0 0x8d00 0x3100
132 132 -922 804 79 0 1 [97, 119, 97] 50 0 0x8f00 0x3100
133 132 -920 804 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
134 132 -918 804 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x3100
122 133 -942 806 78 0 1 [97, 119, 97] 45 0 0x8e00 0x3100
123 133 -940 806 69 0 1 [134, 183, 131] 54 0 0x8500 0x5c00
124 133 -938 806 155 0 1 [134, 183, 131] 54 0 0xdb00 0x5c00
125 133 -936 806 155 0 1 [134, 183, 131] 54 0 0xdb00 0x5c00
126 133 -934 806 155 0 1 [134, 183, 131] 53 0 0xdb00 0x5c00
127 133 -932 806 155 0 1 [134, 183, 131] 53 0 0xdb00 0x5c00
128 133 -930 806 155 0 1 [134, 183, 131] 52 0 0xdb00 0x5c00
129 133 -928 806 155 0 1 [134, 183, 131] 52 0 0xdb00 0x5c00
130 133 -926 806 155 0 1 [134, 183, 131] 51 0 0xdb00 0x5c00
131 133 -924 806 67 0 1 [134, 183, 131] 50 0 0x8300 0x5c00
132 133 -922 806 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 133 -920 806 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
134 133 -918 806 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x3100
122 134 -942 808 79 0 1 [97, 119, 97] 45 0 0x8f00 0x3100
123 134 -940 808 68 0 1 [134, 183, 131] 54 0 0x8400 0x5c00
124 134 -938 808 155 0 1 [134, 183, 131] 54 0 0xdb00 0x5c00
125 134 -936 808 180 0 1 [134, 183, 131] 18446744073709551615 1 0xf400 0x5c00
126 134 -934 808 180 0 1 [134, 183, 131] 18446744073709551615 1 0xf400 0x5c00
127 134 -932 808 180 0 1 [134, 183, 131] 18446744073709551615 1 0xf400 0x5c00
128 134 -930 808 180 0 1 [134, 183, 131] 18446744073709551615 1 0xf400 0x5c00
129 134 -928 808 180 0 1 [134, 183, 131] 2978 1 0xf400 0x5c00
130 134 -926 808 155 0 1 [134, 183, 131] 51 0 0xdb00 0x5c00
131 134 -924 808 67 0 1 [134, 183, 131] 50 0 0x8300 0x5c00
132 134 -922 808 68 0 1 [134, 183, 131] 18446744073709551615 1 0x8400 0x5c00
133 134 -920 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
134 134 -918 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
```
### stream diag after fill:
[02:38:49 INFO]: [c-LoD] stream: tick 3500 65536/65536 cells, lap 1927, cone 26563 zone 1225 dirty 896 (q 11557) ewin 238 rbeat 0p/0 rearm 0c/0t rescan 858557c/1539e, edits +4634p/3492c/516e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 313 forced 199, native 65536c/1090ch fail 0 (3w q 0) helper on, trees 1090ch/fail 0, wire bundle 671090p/2150b fail 0 ~86 kB/s cache 13235/24576 h657855 m13235 e0, srbeat 55526s/0q drop 45stale/0undec, ereb 47218s/0q
[c-LoD] stream: tick 3500 65536/65536 cells, lap 1927, cone 26563 zone 1225 dirty 896 (q 11557) ewin 238 rbeat 0p/0 rearm 0c/0t rescan 858557c/1539e, edits +4634p/3492c/516e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 313 forced 199, native 65536c/1090ch fail 0 (3w q 0) helper on, trees 1090ch/fail 0, wire bundle 671090p/2150b fail 0 ~86 kB/s cache 13235/24576 h657855 m13235 e0, srbeat 55526s/0q drop 45stale/0undec, ereb 47218s/0q
- carrier ladder y=155: 13512 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=165: 13512 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=168: 13512 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=172: 13512 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=176: 13512 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=180: 13512 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=185: 13512 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- probe 0: red=258 paint=998 lod_silhouette_cols=37 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- **LEG 1 PASS** (zero relogs)
- phase: 5 leg-1 probes done (02:39:25)
### stream diag after probes:
[02:39:09 INFO]: [c-LoD] stream: tick 3700 65536/65536 cells, lap 1927, cone 26563 zone 1225 dirty 914 (q 11577) ewin 238 rbeat 0p/0 rearm 0c/0t rescan 923098c/1561e, edits +5250p/3972c/706e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 313 forced 199, native 65536c/1090ch fail 0 (3w q 0) helper on, trees 1090ch/fail 0, wire bundle 672332p/2167b fail 0 ~3 kB/s cache 13316/24576 h659016 m13316 e0, srbeat 55661s/0q drop 45stale/0undec, ereb 47335s/9q
[c-LoD] stream: tick 3800 65536/65536 cells, lap 1927, cone 26563 zone 1225 dirty 923 (q 11571) ewin 138 rbeat 0p/0 rearm 0c/0t rescan 955697c/1568e, edits +5354p/4038c/722e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 313 forced 199, native 65536c/1090ch fail 0 (3w q 0) helper on, trees 1090ch/fail 0, wire bundle 673061p/2177b fail 0 ~3 kB/s cache 13316/24576 h659745 m13316 e0, srbeat 55742s/0q drop 45stale/0undec, ereb 47407s/9q
[02:39:19 INFO]: [c-LoD] stream: tick 3800 65536/65536 cells, lap 1927, cone 26563 zone 1225 dirty 923 (q 11571) ewin 138 rbeat 0p/0 rearm 0c/0t rescan 955697c/1568e, edits +5354p/4038c/722e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 313 forced 199, native 65536c/1090ch fail 0 (3w q 0) helper on, trees 1090ch/fail 0, wire bundle 673061p/2177b fail 0 ~3 kB/s cache 13316/24576 h659745 m13316 e0, srbeat 55742s/0q drop 45stale/0undec, ereb 47407s/9q
- parity debug tour: 13 vantages (session-1 client, DEBUG pack — evidence only, the gate pairs the §7b world session)
- parity debug tour done: 13 shots in parity_dbg/ (the §7b world session feeds the gate)
- joined session 2 (02:42:20)
- leg-2 baseline scene px (x<940): 28080 (0 = the relog client paints no world at all)
- leg-2 renderer probe NEAR (20 m) white px: 500 raw / 500 changed-vs-baseline (> 20 = the client renders particles)
- leg-2 renderer probe FAR (80 m, marker zone) white px: 448 raw / 448 changed-vs-baseline (> 20 = no distance cut; ~0 with NEAR alive = particles beyond the client chunk radius do not render)
- leg-2 carrier probe green dust (80 m): changed-green px 0 / total changed 3018 vs baseline (> 100 green = a stable green carrier exists; total > 0 with green ~0 = the dust renders but fog washes it — a ruler problem, not a carrier problem)
- phase: 8 leg-2 relog probes (02:45:12)
- relog probe 0: red=0 scene=28402 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 1: red=0 scene=28298 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 2: red=0 scene=29702 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 3: red=0 scene=29536 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 4: red=0 scene=30220 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 5: red=0 scene=38635 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 6: red=0 scene=39012 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 7: red=0 scene=38951 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 8: red=0 scene=43159 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 9: red=0 scene=43350 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 10: red=0 scene=43456 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 11: red=0 scene=43424 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 12: red=0 scene=43338 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 13: red=0 scene=43545 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 14: red=0 scene=43435 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 15: red=0 scene=43592 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 16: red=0 scene=43319 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 17: red=0 scene=45987 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 18: red=0 scene=45841 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 19: red=0 scene=46181 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- **FAIL:** marker lost after relog — stream state is not persistent/deterministic
- phase: 9 leg-2 done (02:48:15)

### client log forensics (pipeline/particle/atlas — v6.8.2)
```
java.lang.IllegalStateException: Failed to open OpenAL device
	at net.minecraft.client.sounds.SoundEngine.reload(SoundEngine.java:110)
	at net.minecraft.server.packs.resources.SimplePreparableReloadListener.lambda$reload$1(SimplePreparableReloadListener.java:15)
	at net.minecraft.server.packs.resources.SimpleReloadInstance.lambda$prepareTasks$3(SimpleReloadInstance.java:59)
[02:42:16] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[02:42:16] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[02:42:16] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[02:42:16] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[02:42:16] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[02:42:16] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[02:42:16] [Render thread/INFO]: Created: 2048x2048x4 minecraft:textures/atlas/blocks.png-atlas
[02:42:17] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[02:42:17] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[02:42:17] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[02:42:17] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[02:42:17] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
[02:42:24] [Render thread/INFO]: Reloading ResourceManager: vanilla, vanilla, server/00000000/6b1f2f7e-9c0d-4a4b-8b0a-b10d1b4ab100
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
[02:42:26] [Render thread/ERROR]: Error starting SoundSystem. Turning off sounds & music
java.lang.IllegalStateException: Failed to open OpenAL device
	at net.minecraft.client.sounds.SoundEngine.reload(SoundEngine.java:110)
	at net.minecraft.server.packs.resources.SimplePreparableReloadListener.lambda$reload$1(SimplePreparableReloadListener.java:15)
	at net.minecraft.server.packs.resources.SimpleReloadInstance.lambda$prepareTasks$3(SimpleReloadInstance.java:59)
[02:42:26] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[02:42:26] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[02:42:26] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[02:42:26] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[02:42:26] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[02:42:26] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[02:42:26] [Render thread/INFO]: Created: 2048x2048x4 minecraft:textures/atlas/blocks.png-atlas
[02:42:27] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[02:42:27] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[02:42:27] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[02:42:27] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[02:42:27] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
```

### /clod-cells AFTER PROBES (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 43 0 0x8400 0x3100
123 122 -940 784 71 0 1 [97, 119, 97] 43 0 0x8700 0x3100
124 122 -938 784 68 0 1 [97, 119, 97] 43 0 0x8400 0x3100
125 122 -936 784 62 1 1 [63, 118, 228] 35 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 34 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 34 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 33 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 33 0 0x8000 0x5c00
130 122 -926 784 66 0 1 [134, 183, 131] 33 0 0x8200 0x5c00
131 122 -924 784 75 0 1 [97, 119, 97] 55 0 0x8b00 0x3100
132 122 -922 784 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 122 -920 784 75 0 1 [97, 119, 97] 18446744073709551615 1 0x8b00 0x3100
134 122 -918 784 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x3100
122 123 -942 786 70 0 1 [97, 119, 97] 54 0 0x8600 0x3100
123 123 -940 786 65 0 1 [97, 119, 97] 55 0 0x8100 0x3100
124 123 -938 786 65 0 1 [97, 119, 97] 55 0 0x8100 0x3100
125 123 -936 786 62 1 1 [63, 118, 228] 35 0 0x7e00 0xd800
126 123 -934 786 62 1 1 [63, 118, 228] 34 0 0x7e00 0xd800
127 123 -932 786 62 1 1 [63, 118, 228] 34 0 0x7e00 0xd800
128 123 -930 786 62 1 1 [63, 118, 228] 34 0 0x7e00 0xd800
129 123 -928 786 73 0 1 [97, 119, 97] 55 0 0x8900 0x3100
130 123 -926 786 67 0 1 [134, 183, 131] 33 0 0x8300 0x5c00
131 123 -924 786 69 0 1 [134, 183, 131] 32 0 0x8500 0x5c00
132 123 -922 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 123 -920 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
134 123 -918 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
122 124 -942 788 72 0 1 [97, 119, 97] 54 0 0x8800 0x3100
123 124 -940 788 70 0 1 [97, 119, 97] 55 0 0x8600 0x3100
124 124 -938 788 68 0 1 [97, 119, 97] 55 0 0x8400 0x3100
125 124 -936 788 62 1 1 [63, 118, 228] 35 0 0x7e00 0xd800
126 124 -934 788 62 1 1 [63, 118, 228] 34 0 0x7e00 0xd800
127 124 -932 788 62 1 1 [63, 118, 228] 34 0 0x7e00 0xd800
128 124 -930 788 73 0 1 [97, 119, 97] 55 0 0x8900 0x3100
129 124 -928 788 76 0 1 [97, 119, 97] 55 0 0x8c00 0x3100
130 124 -926 788 73 0 1 [97, 119, 97] 55 0 0x8900 0x3100
131 124 -924 788 69 0 1 [134, 183, 131] 32 0 0x8500 0x5c00
132 124 -922 788 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 124 -920 788 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
134 124 -918 788 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x3100
122 125 -942 790 74 0 1 [97, 119, 97] 54 0 0x8a00 0x3100
123 125 -940 790 73 0 1 [97, 119, 97] 52 0 0x8900 0x3100
124 125 -938 790 71 0 1 [97, 119, 97] 55 0 0x8700 0x3100
125 125 -936 790 62 1 1 [63, 118, 228] 35 0 0x7e00 0xd800
126 125 -934 790 62 1 1 [63, 118, 228] 35 0 0x7e00 0xd800
127 125 -932 790 62 1 1 [63, 118, 228] 34 0 0x7e00 0xd800
128 125 -930 790 62 1 1 [63, 118, 228] 34 0 0x7e00 0xd800
129 125 -928 790 73 0 1 [97, 119, 97] 55 0 0x8900 0x3100
130 125 -926 790 68 0 1 [134, 183, 131] 33 0 0x8400 0x5c00
131 125 -924 790 69 0 1 [134, 183, 131] 32 0 0x8500 0x5c00
132 125 -922 790 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
133 125 -920 790 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
134 125 -918 790 70 0 1 [134, 183, 131] 18446744073709551615 1 0x8600 0x5c00
122 126 -942 792 74 0 1 [97, 119, 97] 54 0 0x8a00 0x3100
123 126 -940 792 73 0 1 [97, 119, 97] 52 0 0x8900 0x3100
124 126 -938 792 68 0 1 [97, 119, 97] 55 0 0x8400 0x3100
125 126 -936 792 62 1 1 [63, 118, 228] 35 0 0x7e00 0xd800
126 126 -934 792 62 1 1 [63, 118, 228] 35 0 0x7e00 0xd800
127 126 -932 792 62 1 1 [63, 118, 228] 34 0 0x7e00 0xd800
128 126 -930 792 62 1 1 [63, 118, 228] 34 0 0x7e00 0xd800
129 126 -928 792 67 0 1 [134, 183, 131] 33 0 0x8300 0x5c00
130 126 -926 792 74 0 1 [97, 119, 97] 55 0 0x8a00 0x3100
131 126 -924 792 77 0 1 [97, 119, 97] 55 0 0x8d00 0x3100
132 126 -922 792 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x3100
133 126 -920 792 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3800
134 126 -918 792 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x8800
122 127 -942 794 67 0 1 [134, 183, 131] 36 0 0x8300 0x5c00
123 127 -940 794 68 0 1 [134, 183, 131] 36 0 0x8400 0x5c00
124 127 -938 794 65 0 1 [134, 183, 131] 35 0 0x8100 0x5c00
125 127 -936 794 62 1 1 [63, 118, 228] 35 0 0x7e00 0xd800
126 127 -934 794 62 1 1 [63, 118, 228] 35 0 0x7e00 0xd800
127 127 -932 794 170 0 1 [142, 33, 33] 34 0 0xea00 0x0200
128 127 -930 794 170 0 1 [142, 33, 33] 34 0 0xea00 0x0200
129 127 -928 794 170 0 1 [142, 33, 33] 34 0 0xea00 0x0200
130 127 -926 794 74 0 1 [97, 119, 97] 55 0 0x8a00 0x3100
131 127 -924 794 77 0 1 [97, 119, 97] 55 0 0x8d00 0x3100
132 127 -922 794 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x3100
133 127 -920 794 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3800
134 127 -918 794 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x8800
122 128 -942 796 68 0 1 [134, 183, 131] 36 0 0x8400 0x5c00
123 128 -940 796 72 0 1 [97, 119, 97] 52 0 0x8800 0x3100
124 128 -938 796 75 0 1 [97, 119, 97] 55 0 0x8b00 0x3100
125 128 -936 796 72 0 1 [97, 119, 97] 55 0 0x8800 0x3100
126 128 -934 796 62 1 1 [63, 118, 228] 35 0 0x7e00 0xd800
127 128 -932 796 170 0 1 [142, 33, 33] 34 0 0xea00 0x0200
128 128 -930 796 170 0 1 [142, 33, 33] 34 0 0xea00 0x0200
129 128 -928 796 170 0 1 [142, 33, 33] 34 0 0xea00 0x0200
130 128 -926 796 68 0 1 [134, 183, 131] 33 0 0x8400 0x5c00
131 128 -924 796 74 0 1 [97, 119, 97] 56 0 0x8a00 0x3100
132 128 -922 796 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 128 -920 796 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x3100
134 128 -918 796 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
122 129 -942 798 69 0 1 [134, 183, 131] 36 0 0x8500 0x5c00
123 129 -940 798 75 0 1 [97, 119, 97] 52 0 0x8b00 0x3100
124 129 -938 798 78 0 1 [97, 119, 97] 53 0 0x8e00 0x3100
125 129 -936 798 75 0 1 [97, 119, 97] 56 0 0x8b00 0x3100
126 129 -934 798 62 1 1 [63, 118, 228] 35 0 0x7e00 0xd800
127 129 -932 798 170 0 1 [142, 33, 33] 34 0 0xea00 0x0200
128 129 -930 798 170 0 1 [142, 33, 33] 34 0 0xea00 0x0200
129 129 -928 798 170 0 1 [142, 33, 33] 34 0 0xea00 0x0200
130 129 -926 798 71 0 1 [97, 119, 97] 56 0 0x8700 0x3100
131 129 -924 798 69 0 1 [134, 183, 131] 32 0 0x8500 0x5c00
132 129 -922 798 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x3100
133 129 -920 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
134 129 -918 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
122 130 -942 800 70 0 1 [134, 183, 131] 36 0 0x8600 0x5c00
123 130 -940 800 76 0 1 [97, 119, 97] 52 0 0x8c00 0x3100
124 130 -938 800 76 0 1 [97, 119, 97] 56 0 0x8c00 0x3100
125 130 -936 800 72 0 1 [97, 119, 97] 56 0 0x8800 0x3100
126 130 -934 800 62 1 1 [63, 118, 228] 35 0 0x7e00 0xd800
127 130 -932 800 62 1 1 [63, 118, 228] 34 0 0x7e00 0xd800
128 130 -930 800 62 1 1 [63, 118, 228] 34 0 0x7e00 0xd800
129 130 -928 800 71 0 1 [97, 119, 97] 56 0 0x8700 0x3100
130 130 -926 800 71 0 1 [97, 119, 97] 56 0 0x8700 0x3100
131 130 -924 800 69 0 1 [134, 183, 131] 32 0 0x8500 0x5c00
132 130 -922 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
133 130 -920 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
134 130 -918 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
122 131 -942 802 78 0 1 [97, 119, 97] 43 0 0x8e00 0x3100
123 131 -940 802 77 0 1 [97, 119, 97] 55 0 0x8d00 0x3100
124 131 -938 802 77 0 1 [97, 119, 97] 56 0 0x8d00 0x3100
125 131 -936 802 66 0 1 [134, 183, 131] 35 0 0x8200 0x5c00
126 131 -934 802 62 1 1 [63, 118, 228] 34 0 0x7e00 0xd800
127 131 -932 802 62 1 1 [63, 118, 228] 34 0 0x7e00 0xd800
128 131 -930 802 62 1 1 [63, 118, 228] 34 0 0x7e00 0xd800
129 131 -928 802 62 1 1 [63, 118, 228] 33 0 0x7e00 0xd800
130 131 -926 802 66 0 1 [134, 183, 131] 33 0 0x8200 0x5c00
131 131 -924 802 68 0 1 [134, 183, 131] 32 0 0x8400 0x5c00
132 131 -922 802 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 131 -920 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
134 131 -918 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
122 132 -942 804 79 0 1 [97, 119, 97] 44 0 0x8f00 0x3100
123 132 -940 804 76 0 1 [97, 119, 97] 55 0 0x8c00 0x3100
124 132 -938 804 76 0 1 [97, 119, 97] 56 0 0x8c00 0x3100
125 132 -936 804 76 0 1 [97, 119, 97] 56 0 0x8c00 0x3100
126 132 -934 804 62 1 1 [63, 118, 228] 34 0 0x7e00 0xd800
127 132 -932 804 62 1 1 [63, 118, 228] 34 0 0x7e00 0xd800
128 132 -930 804 62 1 1 [63, 118, 228] 34 0 0x7e00 0xd800
129 132 -928 804 62 1 1 [63, 118, 228] 33 0 0x7e00 0xd800
130 132 -926 804 65 0 1 [134, 183, 131] 33 0 0x8100 0x5c00
131 132 -924 804 77 0 1 [97, 119, 97] 56 0 0x8d00 0x3100
132 132 -922 804 79 0 1 [97, 119, 97] 56 0 0x8f00 0x3100
133 132 -920 804 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
134 132 -918 804 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x3100
122 133 -942 806 78 0 1 [97, 119, 97] 44 0 0x8e00 0x3100
123 133 -940 806 69 0 1 [134, 183, 131] 36 0 0x8500 0x5c00
124 133 -938 806 155 0 1 [134, 183, 131] 36 0 0xdb00 0x5c00
125 133 -936 806 155 0 1 [134, 183, 131] 36 0 0xdb00 0x5c00
126 133 -934 806 155 0 1 [134, 183, 131] 35 0 0xdb00 0x5c00
127 133 -932 806 155 0 1 [134, 183, 131] 35 0 0xdb00 0x5c00
128 133 -930 806 155 0 1 [134, 183, 131] 34 0 0xdb00 0x5c00
129 133 -928 806 155 0 1 [134, 183, 131] 34 0 0xdb00 0x5c00
130 133 -926 806 155 0 1 [134, 183, 131] 33 0 0xdb00 0x5c00
131 133 -924 806 67 0 1 [134, 183, 131] 32 0 0x8300 0x5c00
132 133 -922 806 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 133 -920 806 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
134 133 -918 806 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x3100
122 134 -942 808 79 0 1 [97, 119, 97] 44 0 0x8f00 0x3100
123 134 -940 808 68 0 1 [134, 183, 131] 36 0 0x8400 0x5c00
124 134 -938 808 155 0 1 [134, 183, 131] 36 0 0xdb00 0x5c00
125 134 -936 808 180 0 1 [134, 183, 131] 4926 0 0xf400 0x5c00
126 134 -934 808 180 0 1 [134, 183, 131] 4926 0 0xf400 0x5c00
127 134 -932 808 180 0 1 [134, 183, 131] 4926 0 0xf400 0x5c00
128 134 -930 808 180 0 1 [134, 183, 131] 4926 0 0xf400 0x5c00
129 134 -928 808 180 0 1 [134, 183, 131] 4925 0 0xf400 0x5c00
130 134 -926 808 155 0 1 [134, 183, 131] 33 0 0xdb00 0x5c00
131 134 -924 808 67 0 1 [134, 183, 131] 32 0 0x8300 0x5c00
132 134 -922 808 68 0 1 [134, 183, 131] 18446744073709551615 1 0x8400 0x5c00
133 134 -920 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
134 134 -918 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x5c00
```
- edit channel: 0 linkage errors, 0 failed event dispatches (both must be 0)
- phase: 6 parity world boot (02:48:16)
- parity world: view-distance=8 (the voxy leg's server value)
- parity world: joined session 3 (02:49:02)
- parity world: scan populated (cells=65536; the march paints from these buffers)
- parity world tour: 13 vantages (production pack, vd 8, shared structures)
- coverage sweeps: 72 eye passes = 36 positions (12 az x r144/200 + 12 off-az x r172, y=160) x 2 looks (outward+inward), settle-by-laps >= 6 s (q stable 3x1s), deadline 12 s/pos, no shots — the v6.26.0 data-coverage lever + v6.26.6 lottery kill; probe rides parity/view_probe.md
- parity drain-settle TIMEOUT: queue 22521 -> 22521 > 11500 after 150s — continuing anyway; the queue did not drain without eye proximity (priority-lane evidence, the 01/05/10 dip attribution reads from the tour deltas)
- pre-tour pass done: 13 vantages x 10s (the tour now reads the second OR-merge lap; the rim/dip transport race reads from the tour deltas)
- parity world tour done: 13 shots, 0 night-guard renames in parity/ (the gate pairs voxy/<name>.png vs parity/<name>.png)
- phase: 7 parity tour done (03:11:15)
- sparse re-beat channel at tour end: streamed=77416 q=0 dropped_stale=28 dropped_undecodable=9671 (frac=8 boot; undecodable-dominant = the rim words die in their decodability window, stale-dominant = edit churn owns the cells)
### parity world stream counters (session 3, tail + maxima)
[c-LoD] stream: tick 13400 65536/65536 cells, lap 8469, cone 151052 zone 19269 dirty 6882 (q 22500) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 9132354c/1339e, edits +17971p/13626c/2804e drop 0, spawned 0 / failed 0, cam0 yaw -180 pitch 5, wish 0 forced 917, native 31040c/955ch fail 0 (3w q 0) helper on, trees 955ch/fail 0, wire bundle 1978653p/4917b fail 0 ~0 kB/s cache 22912/24576 h1626761 m351892 e328980, srbeat 77416s/0q drop 28stale/9671undec, ereb 30763s/4q
[03:11:09 INFO]: [c-LoD] stream: tick 13400 65536/65536 cells, lap 8469, cone 151052 zone 19269 dirty 6882 (q 22500) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 9132354c/1339e, edits +17971p/13626c/2804e drop 0, spawned 0 / failed 0, cam0 yaw -180 pitch 5, wish 0 forced 917, native 31040c/955ch fail 0 (3w q 0) helper on, trees 955ch/fail 0, wire bundle 1978653p/4917b fail 0 ~0 kB/s cache 22912/24576 h1626761 m351892 e328980, srbeat 77416s/0q drop 28stale/9671undec, ereb 30763s/4q
- parity stream max cone: 151052
- parity stream max zone: 19269
- parity stream max wire bundle: bundle
### parity world capture lens (debug chain on the §7b world)
lens_red=0 lens_black=0 lens_water=0 lens_dark=0 lens_zero=0 rot=none scan=0 carriers=0
- parity mesh probe @png_y=2: x306=(151,184,255) x307=(151,184,255) x308=(151,184,255) x309h=(151,184,255) x310c=(151,184,255)
- parity mesh probe @png_y=717: x306=(44,62,51) x307=(65,89,72) x308=(81,107,87) x309h=(82,108,87) x310c=(84,111,90)
- parity mesh engagement: magenta-ish=144 of 921600 sampled px
- mask8: the un-fog engagement masks shot on the PARITY WORLD (vd 8) — parity_dbg8/01m|07m|10m
- mask8 01m: green=5968 red=0 blue=98272 dim=5152 (px, sampled /16)
- mask8 07m: green=11568 red=0 blue=221440 dim=5808 (px, sampled /16)
- mask8 10m: green=8160 red=144 blue=230880 dim=153040 (px, sampled /16)
- mesh probe @png_y=2: x306=(255,255,255) x307=(13,254,0) x308=(0,254,0) x309h=(0,0,0) x310c=(0,0,0)
- mesh probe @png_y=717: x306=(255,255,255) x307=(13,254,0) x308=(0,254,0) x309h=(0,0,0) x310c=(0,0,0)
- mesh engagement: magenta-ish=0 of 921600 sampled px (the vanilla-path paint coverage)
- mask8 fog state @png_y=2: raw (300)=(151,184,255) (301)=(151,184,255)
- mask8 fog state @png_y=717: raw (300)=(0,128,8) (301)=(179,206,255)
- mask8 fog state decoded: envStart=0 envEnd=1024 rdEnd=128 fogColor=(179,206,255)
- buffer dump 10m: marked h=28/2048 c=1/2048; height windows r0<142:18 r1:2 r2>=202:8 maxByte=255
- buffer dump 10m: RANK-2-WINDOW HEIGHTS (the fake-wall class): (+2,-26)h=160, (+0,-18)h=158, (+106,-18)h=191, (+12,+0)h=160, (-8,+24)h=145, (+4,+24)h=151, (+2,+26)h=151, (+4,+26)h=150
- rim mask: scan populated (cells=65536; the state paint reads the same buffers the march reads)
- phase: 10 rim mask lane (03:15:51)
- rim mask: decode-state frames shot (rim_mask/00|01|12 — GREEN full decode / RED colorless / BLUE heightless; 00 = the leg-1 tower eye, 01 = the near-tower contrast, 12 = the tower re-read; the inherited 07/12 vantages sat 760 blocks outside the streamed disk and 01 sat INSIDE the marker column — their earlier verdicts were classifier noise, purged in v6.33.1)
- rim mask 00: green=10448 red=83696 blue=88640 yellow=10176 (px, sampled /16) -> DOMINANT BLUE: rim = HEIGHTS NEVER DECODED — a data-coverage class: the state paint cannot even see the cells
- rim mask 01: green=21456 red=110784 blue=118736 yellow=24176 (px, sampled /16) -> DOMINANT BLUE: rim = HEIGHTS NEVER DECODED — a data-coverage class: the state paint cannot even see the cells
- rim mask 12: green=18864 red=103824 blue=107136 yellow=11088 (px, sampled /16) -> DOMINANT BLUE: rim = HEIGHTS NEVER DECODED — a data-coverage class: the state paint cannot even see the cells

### boot / stream
```
[c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 127551 bytes, sha1 c7293c2a24265e83ebf9025127549657b5234c3b — zero textures, zero world data, one push per join
[c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[03:15:00 INFO]: [c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 127551 bytes, sha1 c7293c2a24265e83ebf9025127549657b5234c3b — zero textures, zero world data, one push per join
[03:15:00 INFO]: [c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[c-LoD] stream: tick 900 65536/65536 cells, lap 737, cone 14028 zone 866 dirty 19 (q 7497) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 516109c/364e, edits +11p/11c/5e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 0 forced 950, native 8146c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 140561p/478b fail 0 ~75 kB/s cache 14090/24576 h126471 m14090 e0, srbeat 4970s/1062q drop 0stale/468undec, ereb 576s/19q
[03:16:40 INFO]: [c-LoD] stream: tick 900 65536/65536 cells, lap 737, cone 14028 zone 866 dirty 19 (q 7497) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 516109c/364e, edits +11p/11c/5e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 0 forced 950, native 8146c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 140561p/478b fail 0 ~75 kB/s cache 14090/24576 h126471 m14090 e0, srbeat 4970s/1062q drop 0stale/468undec, ereb 576s/19q
```
### sessions
```
[03:15:25 INFO]: System chat: Stepan joined the game
[03:16:46 INFO]: System chat: Stepan left the game
```
### client log tail
```
[03:16:46] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[03:16:46] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[03:16:46] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[03:16:46] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[03:16:46] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[03:16:46] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[03:16:46] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[03:16:46] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[03:16:46] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[03:16:46] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[03:16:46] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[03:16:46] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[03:16:46] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[03:16:46] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[03:16:46] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[03:16:46] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[03:16:46] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[03:16:46] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[03:16:46] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[03:16:46] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[03:16:46] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[03:16:46] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[03:16:46] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[03:16:46] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[03:16:46] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[03:16:46] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[03:16:46] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[03:16:46] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:bundle_delimiter] net.minecraft.network.protocol.game.ClientboundBundleDelimiterPacket -> 1 bytes
[03:16:46] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:set_time] net.minecraft.network.protocol.game.ClientboundSetTimePacket -> 10 bytes
[03:16:46] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
```
### server log tail
```
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[03:16:47 INFO]: [ChunkHolderManager] Saved 1002 block chunks, 976 entity chunks, 0 poi chunks in world 'minecraft:overworld' in 1.14s
[03:16:47 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:overworld'
[03:16:47 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:overworld'
[03:16:47 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[03:16:47 INFO]: [ChunkHolderManager] Waiting 60s for chunk system to halt for world 'minecraft:the_nether'
[03:16:47 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:the_nether'
[03:16:47 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_nether'
[03:16:47 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_nether' in 0.00s
[03:16:47 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_nether'
[03:16:47 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_nether'
[03:16:47 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[03:16:47 INFO]: [ChunkHolderManager] Waiting 60s for chunk system to halt for world 'minecraft:the_end'
[03:16:47 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:the_end'
[03:16:47 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_end'
[03:16:47 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_end' in 0.00s
[03:16:47 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_end'
[03:16:47 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_end'
[03:16:47 INFO]: Waiting for all RegionFile I/O tasks to complete...
[03:16:47 INFO]: All RegionFile I/O tasks to complete
[03:16:47 INFO]: [MoonriseCommon] Awaiting termination of worker pool for up to 60s...
[03:16:47 INFO]: [MoonriseCommon] Awaiting termination of I/O pool for up to 60s...
[launcher] server exited with code 143
```
### shots
```
-rw-r--r-- 1 runner runner   4573 Nov 14 02:46 lens_13_after_relog_probe11.png
-rw-r--r-- 1 runner runner   4517 Nov 14 02:47 lens_13_after_relog_probe12.png
-rw-r--r-- 1 runner runner   4672 Nov 14 02:47 lens_13_after_relog_probe13.png
-rw-r--r-- 1 runner runner   4624 Nov 14 02:47 lens_13_after_relog_probe14.png
-rw-r--r-- 1 runner runner   4752 Nov 14 02:47 lens_13_after_relog_probe15.png
-rw-r--r-- 1 runner runner   4871 Nov 14 02:47 lens_13_after_relog_probe16.png
-rw-r--r-- 1 runner runner   4853 Nov 14 02:47 lens_13_after_relog_probe17.png
-rw-r--r-- 1 runner runner   4826 Nov 14 02:47 lens_13_after_relog_probe18.png
-rw-r--r-- 1 runner runner   4777 Nov 14 02:48 lens_13_after_relog_probe19.png
-rw-r--r-- 1 runner runner   4173 Nov 14 02:45 lens_13_after_relog_probe2.png
-rw-r--r-- 1 runner runner   4202 Nov 14 02:45 lens_13_after_relog_probe3.png
-rw-r--r-- 1 runner runner   4541 Nov 14 02:45 lens_13_after_relog_probe4.png
-rw-r--r-- 1 runner runner   4687 Nov 14 02:46 lens_13_after_relog_probe5.png
-rw-r--r-- 1 runner runner   4549 Nov 14 02:46 lens_13_after_relog_probe6.png
-rw-r--r-- 1 runner runner   4604 Nov 14 02:46 lens_13_after_relog_probe7.png
-rw-r--r-- 1 runner runner   4722 Nov 14 02:46 lens_13_after_relog_probe8.png
-rw-r--r-- 1 runner runner   4791 Nov 14 02:46 lens_13_after_relog_probe9.png
drwxr-xr-x 2 runner runner   4096 Nov 14 03:11 lens_parity
drwxr-xr-x 2 runner runner   4096 Nov 14 03:11 parity
drwxr-xr-x 2 runner runner   4096 Nov 14 02:41 parity_dbg
drwxr-xr-x 2 runner runner   4096 Nov 14 03:14 parity_dbg8
-rw-r--r-- 1 runner runner   3819 Nov 14 02:48 probe_counts.txt
drwxr-xr-x 2 runner runner   4096 Nov 14 03:16 rim_mask
-rw-r--r-- 1 runner runner  60963 Nov 14 03:16 server_full.log
-rw-r--r-- 1 runner runner  51643 Nov 14 03:16 summary.md
```

**exit=1** failures=1 leg1=1 leg2=0
