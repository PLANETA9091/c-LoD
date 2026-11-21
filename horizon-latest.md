- phase: 0 sanity (19:06:34)
### c-LoD visual run (-930,796, marker=red_concrete, vantage -1005,150 yaw 270)
- GL:     Vendor: Mesa (0xffffffff);    Device: llvmpipe (LLVM 21.1.8, 256 bits) (0xffffffff);OpenGL renderer string: llvmpipe (LLVM 21.1.8, 256 bits);OpenGL core profile version string: 4.5 (Core Profile) Mesa 26.0.8-1ubuntu0.3;
- boot + gates OK (19:06:54)
- phase: 1 server booted (19:06:54)
- sky-grid armed
- phase: 2 leg-1 client joined (19:07:05)
- joined session 1 (19:07:05)
- stream live: cells=65536
- phase: 3 scan populated (19:07:28)
- terrain range: 2 terraced mountains at (CX-4,CZ-20)/(CX-2,CZ+16), tops 185/190, grass terraces + stone caps, 66..82 m from the eye (inside the 83.5 m band, chunk edge 64 m)
- scan complete=1 (cells=cells=65536/65536)
- paint settled=1 (wire 513183p)
- baseline try 1: drift=893 px (ground structural 424 — the gate reads the GROUND half: v6.59.15j, run 44903108766's drift map split 36552/2168 sky-vs-ground — the sky band's wall blobs vetoed every session while the ground sat at sparkle level; full-frame numbers stay printed)
- lod_silhouette_cols=61 (LOD paint ON SCREEN at 75 m if > 20 — the march-drawn marker column; vanilla cannot render there, chunk edge 64 m; #255 re-measures 49, the ZONE_FAR=48 regression #256 and the control read 0)
- 195 terrain-green px in the sky region (LOD MOUNTAINS on screen if > 5000; the range paints ColLow-green on low terraces via the fallback palette; flat-world baselines and the pack-free control read ~0)
- burst_close white px: 2257 (renderer alive if > 50)
- burst_far white px: 1638 (renderer alive in the marker zone if > 20)
- cut walk 40 m: 67 white px (> 20 = rendered)
- cut walk 60 m: 32 white px (> 20 = rendered)
- cut walk 100 m: 0 white px (> 20 = rendered)
- measured far-render cut: 60 m (G4 flap instrument; the decode proof needs >= 83 m from the y=151.6 eye)
- fill issued: 5x111x5 red_concrete at -930,796 (zone forceloaded ±48)
- phase: 4 marker filled (19:12:41)

### /clod-cells AFTER FILL (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 17 0 0x8400 0x2000
123 122 -940 784 71 0 1 [97, 119, 97] 17 0 0x8700 0x2000
124 122 -938 784 68 0 1 [97, 119, 97] 16 0 0x8400 0x2000
125 122 -936 784 62 1 1 [63, 118, 228] 56 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 55 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 55 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 54 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 457 0 0x8000 0x4100
130 122 -926 784 66 0 1 [134, 183, 131] 457 0 0x8200 0x4100
131 122 -924 784 75 0 1 [97, 119, 97] 457 0 0x8b00 0x2000
132 122 -922 784 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 122 -920 784 75 0 1 [97, 119, 97] 18446744073709551615 1 0x8b00 0x2000
134 122 -918 784 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x2000
122 123 -942 786 70 0 1 [97, 119, 97] 17 0 0x8600 0x2000
123 123 -940 786 65 0 1 [97, 119, 97] 17 0 0x8100 0x2000
124 123 -938 786 65 0 1 [97, 119, 97] 16 0 0x8100 0x2000
125 123 -936 786 62 1 1 [63, 118, 228] 56 0 0x7e00 0xd800
126 123 -934 786 62 1 1 [63, 118, 228] 55 0 0x7e00 0xd800
127 123 -932 786 62 1 1 [63, 118, 228] 55 0 0x7e00 0xd800
128 123 -930 786 62 1 1 [63, 118, 228] 54 0 0x7e00 0xd800
129 123 -928 786 73 0 1 [97, 119, 97] 457 0 0x8900 0x2000
130 123 -926 786 67 0 1 [134, 183, 131] 457 0 0x8300 0x4100
131 123 -924 786 69 0 1 [134, 183, 131] 560 0 0x8500 0x4100
132 123 -922 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 123 -920 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 123 -918 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
122 124 -942 788 72 0 1 [97, 119, 97] 17 0 0x8800 0x2000
123 124 -940 788 70 0 1 [97, 119, 97] 17 0 0x8600 0x2000
124 124 -938 788 68 0 1 [97, 119, 97] 16 0 0x8400 0x2000
125 124 -936 788 62 1 1 [63, 118, 228] 56 0 0x7e00 0xd800
126 124 -934 788 62 1 1 [63, 118, 228] 55 0 0x7e00 0xd800
127 124 -932 788 62 1 1 [63, 118, 228] 55 0 0x7e00 0xd800
128 124 -930 788 73 0 1 [97, 119, 97] 14 0 0x8900 0x2000
129 124 -928 788 76 0 1 [97, 119, 97] 457 0 0x8c00 0x2000
130 124 -926 788 73 0 1 [97, 119, 97] 457 0 0x8900 0x2000
131 124 -924 788 69 0 1 [134, 183, 131] 559 0 0x8500 0x4100
132 124 -922 788 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 124 -920 788 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 124 -918 788 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x2000
122 125 -942 790 74 0 1 [97, 119, 97] 17 0 0x8a00 0x2000
123 125 -940 790 73 0 1 [97, 119, 97] 17 0 0x8900 0x2000
124 125 -938 790 71 0 1 [97, 119, 97] 16 0 0x8700 0x2000
125 125 -936 790 62 1 1 [63, 118, 228] 56 0 0x7e00 0xd800
126 125 -934 790 62 1 1 [63, 118, 228] 55 0 0x7e00 0xd800
127 125 -932 790 62 1 1 [63, 118, 228] 55 0 0x7e00 0xd800
128 125 -930 790 62 1 1 [63, 118, 228] 54 0 0x7e00 0xd800
129 125 -928 790 73 0 1 [97, 119, 97] 14 0 0x8900 0x2000
130 125 -926 790 68 0 1 [134, 183, 131] 457 0 0x8400 0x4100
131 125 -924 790 69 0 1 [134, 183, 131] 559 0 0x8500 0x4100
132 125 -922 790 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
133 125 -920 790 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 125 -918 790 70 0 1 [134, 183, 131] 18446744073709551615 1 0x8600 0x4100
122 126 -942 792 74 0 1 [97, 119, 97] 17 0 0x8a00 0x2000
123 126 -940 792 73 0 1 [97, 119, 97] 17 0 0x8900 0x2000
124 126 -938 792 68 0 1 [97, 119, 97] 16 0 0x8400 0x2000
125 126 -936 792 62 1 1 [63, 118, 228] 56 0 0x7e00 0xd800
126 126 -934 792 62 1 1 [63, 118, 228] 55 0 0x7e00 0xd800
127 126 -932 792 62 1 1 [63, 118, 228] 55 0 0x7e00 0xd800
128 126 -930 792 62 1 1 [63, 118, 228] 54 0 0x7e00 0xd800
129 126 -928 792 67 0 1 [134, 183, 131] 14 0 0x8300 0x4100
130 126 -926 792 74 0 1 [97, 119, 97] 457 0 0x8a00 0x2000
131 126 -924 792 77 0 1 [97, 119, 97] 559 0 0x8d00 0x2000
132 126 -922 792 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 126 -920 792 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3200
134 126 -918 792 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x5000
122 127 -942 794 67 0 1 [134, 183, 131] 17 0 0x8300 0x4100
123 127 -940 794 68 0 1 [134, 183, 131] 17 0 0x8400 0x4100
124 127 -938 794 65 0 1 [134, 183, 131] 16 0 0x8100 0x4100
125 127 -936 794 62 1 1 [63, 118, 228] 56 0 0x7e00 0xd800
126 127 -934 794 62 1 1 [63, 118, 228] 55 0 0x7e00 0xd800
127 127 -932 794 62 1 1 [63, 118, 228] 55 0 0x7e00 0xd800
128 127 -930 794 62 1 1 [63, 118, 228] 54 0 0x7e00 0xd800
129 127 -928 794 66 0 1 [134, 183, 131] 14 0 0x8200 0x4100
130 127 -926 794 74 0 1 [97, 119, 97] 457 0 0x8a00 0x2000
131 127 -924 794 77 0 1 [97, 119, 97] 457 0 0x8d00 0x2000
132 127 -922 794 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 127 -920 794 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3200
134 127 -918 794 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x5000
122 128 -942 796 68 0 1 [134, 183, 131] 17 0 0x8400 0x4100
123 128 -940 796 72 0 1 [97, 119, 97] 17 0 0x8800 0x2000
124 128 -938 796 75 0 1 [97, 119, 97] 16 0 0x8b00 0x2000
125 128 -936 796 72 0 1 [97, 119, 97] 16 0 0x8800 0x2000
126 128 -934 796 62 1 1 [63, 118, 228] 55 0 0x7e00 0xd800
127 128 -932 796 62 1 1 [63, 118, 228] 55 0 0x7e00 0xd800
128 128 -930 796 62 1 1 [63, 118, 228] 54 0 0x7e00 0xd800
129 128 -928 796 66 0 1 [134, 183, 131] 14 0 0x8200 0x4100
130 128 -926 796 68 0 1 [134, 183, 131] 457 0 0x8400 0x4100
131 128 -924 796 74 0 1 [97, 119, 97] 457 0 0x8a00 0x2000
132 128 -922 796 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 128 -920 796 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
134 128 -918 796 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 129 -942 798 69 0 1 [134, 183, 131] 17 0 0x8500 0x4100
123 129 -940 798 75 0 1 [97, 119, 97] 17 0 0x8b00 0x2000
124 129 -938 798 78 0 1 [97, 119, 97] 18 0 0x8e00 0x2000
125 129 -936 798 75 0 1 [97, 119, 97] 16 0 0x8b00 0x2000
126 129 -934 798 62 1 1 [63, 118, 228] 55 0 0x7e00 0xd800
127 129 -932 798 62 1 1 [63, 118, 228] 55 0 0x7e00 0xd800
128 129 -930 798 62 1 1 [63, 118, 228] 54 0 0x7e00 0xd800
129 129 -928 798 71 0 1 [97, 119, 97] 14 0 0x8700 0x2000
130 129 -926 798 71 0 1 [97, 119, 97] 457 0 0x8700 0x2000
131 129 -924 798 69 0 1 [134, 183, 131] 457 0 0x8500 0x4100
132 129 -922 798 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 129 -920 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 129 -918 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 130 -942 800 70 0 1 [134, 183, 131] 17 0 0x8600 0x4100
123 130 -940 800 76 0 1 [97, 119, 97] 17 0 0x8c00 0x2000
124 130 -938 800 76 0 1 [97, 119, 97] 16 0 0x8c00 0x2000
125 130 -936 800 72 0 1 [97, 119, 97] 16 0 0x8800 0x2000
126 130 -934 800 62 1 1 [63, 118, 228] 55 0 0x7e00 0xd800
127 130 -932 800 62 1 1 [63, 118, 228] 55 0 0x7e00 0xd800
128 130 -930 800 62 1 1 [63, 118, 228] 54 0 0x7e00 0xd800
129 130 -928 800 71 0 1 [97, 119, 97] 14 0 0x8700 0x2000
130 130 -926 800 71 0 1 [97, 119, 97] 457 0 0x8700 0x2000
131 130 -924 800 69 0 1 [134, 183, 131] 457 0 0x8500 0x4100
132 130 -922 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
133 130 -920 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 130 -918 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 131 -942 802 78 0 1 [97, 119, 97] 19 0 0x8e00 0x2000
123 131 -940 802 77 0 1 [97, 119, 97] 17 0 0x8d00 0x2000
124 131 -938 802 77 0 1 [97, 119, 97] 16 0 0x8d00 0x2000
125 131 -936 802 66 0 1 [134, 183, 131] 16 0 0x8200 0x4100
126 131 -934 802 62 1 1 [63, 118, 228] 55 0 0x7e00 0xd800
127 131 -932 802 62 1 1 [63, 118, 228] 55 0 0x7e00 0xd800
128 131 -930 802 62 1 1 [63, 118, 228] 54 0 0x7e00 0xd800
129 131 -928 802 62 1 1 [63, 118, 228] 497 0 0x7e00 0xd800
130 131 -926 802 66 0 1 [134, 183, 131] 457 0 0x8200 0x4100
131 131 -924 802 68 0 1 [134, 183, 131] 457 0 0x8400 0x4100
132 131 -922 802 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 131 -920 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 131 -918 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 132 -942 804 79 0 1 [97, 119, 97] 19 0 0x8f00 0x2000
123 132 -940 804 76 0 1 [97, 119, 97] 17 0 0x8c00 0x2000
124 132 -938 804 76 0 1 [97, 119, 97] 16 0 0x8c00 0x2000
125 132 -936 804 76 0 1 [97, 119, 97] 16 0 0x8c00 0x2000
126 132 -934 804 62 1 1 [63, 118, 228] 55 0 0x7e00 0xd800
127 132 -932 804 62 1 1 [63, 118, 228] 55 0 0x7e00 0xd800
128 132 -930 804 62 1 1 [63, 118, 228] 54 0 0x7e00 0xd800
129 132 -928 804 62 1 1 [63, 118, 228] 497 0 0x7e00 0xd800
130 132 -926 804 65 0 1 [134, 183, 131] 457 0 0x8100 0x4100
131 132 -924 804 77 0 1 [97, 119, 97] 457 0 0x8d00 0x2000
132 132 -922 804 79 0 1 [97, 119, 97] 457 0 0x8f00 0x2000
133 132 -920 804 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 132 -918 804 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x2000
122 133 -942 806 78 0 1 [97, 119, 97] 19 0 0x8e00 0x2000
123 133 -940 806 69 0 1 [134, 183, 131] 17 0 0x8500 0x4100
124 133 -938 806 155 0 1 [134, 183, 131] 17 0 0xdb00 0x4100
125 133 -936 806 155 0 1 [134, 183, 131] 17 0 0xdb00 0x4100
126 133 -934 806 155 0 1 [134, 183, 131] 16 0 0xdb00 0x4100
127 133 -932 806 155 0 1 [134, 183, 131] 16 0 0xdb00 0x4100
128 133 -930 806 155 0 1 [134, 183, 131] 15 0 0xdb00 0x4100
129 133 -928 806 155 0 1 [134, 183, 131] 14 0 0xdb00 0x4100
130 133 -926 806 155 0 1 [134, 183, 131] 14 0 0xdb00 0x4100
131 133 -924 806 67 0 1 [134, 183, 131] 457 0 0x8300 0x4100
132 133 -922 806 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 133 -920 806 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 133 -918 806 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x2000
122 134 -942 808 79 0 1 [97, 119, 97] 19 0 0x8f00 0x2000
123 134 -940 808 68 0 1 [134, 183, 131] 17 0 0x8400 0x4100
124 134 -938 808 155 0 1 [134, 183, 131] 17 0 0xdb00 0x4100
125 134 -936 808 180 0 1 [134, 183, 131] 14 0 0xf400 0x4100
126 134 -934 808 180 0 1 [134, 183, 131] 14 0 0xf400 0x4100
127 134 -932 808 180 0 1 [134, 183, 131] 457 0 0xf400 0x4100
128 134 -930 808 180 0 1 [134, 183, 131] 559 0 0xf400 0x4100
129 134 -928 808 180 0 1 [134, 183, 131] 457 0 0xf400 0x4100
130 134 -926 808 155 0 1 [134, 183, 131] 14 0 0xdb00 0x4100
131 134 -924 808 67 0 1 [134, 183, 131] 455 0 0x8300 0x4100
132 134 -922 808 68 0 1 [134, 183, 131] 18446744073709551615 1 0x8400 0x4100
133 134 -920 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 134 -918 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
```
### stream diag after fill:
[c-LoD] stream: tick 3500 65536/65536 cells, lap 714, cone 17313 zone 1231 dirty 945 (q 10522) ewin 173 rbeat 31p/1 rearm 0c/0t rescan 877343c/2732e, edits +6292p/4808c/753e drop 0, spawned 0 / failed 0, cas 0d, cam0 yaw -90 pitch 5, wish 364 forced 148, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 602281p/2007b fail 0 ~2 kB/s cache 13720/24576 h588561 m13720 e0, srbeat 55983s/26q drop 63stale/101undec, ereb 47594s/110q, sweep 0r/35d, cdiag off
[19:12:45 INFO]: [c-LoD] stream: tick 3500 65536/65536 cells, lap 714, cone 17313 zone 1231 dirty 945 (q 10522) ewin 173 rbeat 31p/1 rearm 0c/0t rescan 877343c/2732e, edits +6292p/4808c/753e drop 0, spawned 0 / failed 0, cas 0d, cam0 yaw -90 pitch 5, wish 364 forced 148, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 602281p/2007b fail 0 ~2 kB/s cache 13720/24576 h588561 m13720 e0, srbeat 55983s/26q drop 63stale/101undec, ereb 47594s/110q, sweep 0r/35d, cdiag off
- carrier ladder y=155: 14772 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=165: 14368 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=168: 14192 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=172: 14148 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=176: 14244 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=180: 14080 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- carrier ladder y=185: 14964 strobe-visible px over the marker (120 happy_villager = the transport's own carrier; 0 = the plane's dots cannot strobe there)
- probe 0: red=16043 paint=133595 lod_silhouette_cols=91 lens_red=0 lens_black=0 lens_water=0 lens_dark=0 lens_zero=0 rot=none scan=0 carriers=0 (need red>=60 paint>=300 lod>20)
- **LEG 1 PASS** (zero relogs)
- phase: 5 leg-1 probes done (19:13:14)
### stream diag after probes:
[19:12:55 INFO]: [c-LoD] stream: tick 3600 65536/65536 cells, lap 714, cone 17313 zone 1231 dirty 976 (q 10520) ewin 173 rbeat 31p/0 rearm 0c/0t rescan 913178c/2748e, edits +6479p/4951c/814e drop 0, spawned 0 / failed 0, cas 0d, cam0 yaw -90 pitch 5, wish 364 forced 148, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 603728p/2019b fail 0 ~7 kB/s cache 13765/24576 h589963 m13765 e0, srbeat 56129s/26q drop 79stale/101undec, ereb 47730s/128q, sweep 0r/35d, cdiag off
[c-LoD] stream: tick 3700 65536/65536 cells, lap 714, cone 17313 zone 1231 dirty 1007 (q 10523) ewin 173 rbeat 0p/0 rearm 0c/0t rescan 941927c/2753e, edits +6591p/5022c/828e drop 0, spawned 0 / failed 0, cas 0d, cam0 yaw -90 pitch 5, wish 364 forced 148, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 606199p/2029b fail 0 ~13 kB/s cache 13765/24576 h592434 m13765 e0, srbeat 56403s/26q drop 79stale/101undec, ereb 47978s/128q, sweep 0r/35d, cdiag off
[19:13:05 INFO]: [c-LoD] stream: tick 3700 65536/65536 cells, lap 714, cone 17313 zone 1231 dirty 1007 (q 10523) ewin 173 rbeat 0p/0 rearm 0c/0t rescan 941927c/2753e, edits +6591p/5022c/828e drop 0, spawned 0 / failed 0, cas 0d, cam0 yaw -90 pitch 5, wish 364 forced 148, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 606199p/2029b fail 0 ~13 kB/s cache 13765/24576 h592434 m13765 e0, srbeat 56403s/26q drop 79stale/101undec, ereb 47978s/128q, sweep 0r/35d, cdiag off
- parity debug tour: 13 vantages (session-1 client, DEBUG pack — evidence only, the gate pairs the §7b world session)
- parity debug tour done: 13 shots in parity_dbg/ (the §7b world session feeds the gate)
- joined session 2 (19:15:54)
- leg-2 baseline scene px (x<940): 24909 (0 = the relog client paints no world at all)
- leg-2 renderer probe NEAR (20 m) white px: 863 raw / 2907 changed-vs-baseline (> 20 = the client renders particles)
- leg-2 renderer probe FAR (80 m, marker zone) white px: 305 raw / 5109 changed-vs-baseline (> 20 = no distance cut; ~0 with NEAR alive = particles beyond the client chunk radius do not render)
- leg-2 carrier probe green dust (80 m): changed-green px 0 / total changed 24812 vs baseline (> 100 green = a stable green carrier exists; total > 0 with green ~0 = the dust renders but fog washes it — a ruler problem, not a carrier problem)
- phase: 8 leg-2 relog probes (19:21:16)
- relog probe 0: red=16007 scene=25668 lens_red=0 lens_black=6 lens_water=0 lens_dark=0 lens_zero=0 rot=decoded cam_x_b=? cam_z_b=? az_b=? el_b=? fog=live fog_end_b=? strobe_hits=22 hits_hi=22 mask_hits=22 (need red >= 60; scene > 0 = the client paints a world at all)
- **LEG 2 PASS** (marker persisted over relog)
- phase: 9 leg-2 done (19:21:19)

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
122 122 -942 784 68 0 1 [97, 119, 97] 3470 0 0x8400 0x2000
123 122 -940 784 71 0 1 [97, 119, 97] 3470 0 0x8700 0x2000
124 122 -938 784 68 0 1 [97, 119, 97] 3470 0 0x8400 0x2000
125 122 -936 784 62 1 1 [63, 118, 228] 3470 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 3470 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 3470 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 3470 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 3470 0 0x8000 0x4100
130 122 -926 784 66 0 1 [134, 183, 131] 3470 0 0x8200 0x4100
131 122 -924 784 75 0 1 [97, 119, 97] 3470 0 0x8b00 0x2000
132 122 -922 784 77 0 1 [97, 119, 97] 4036 1 0x8d00 0x2000
133 122 -920 784 75 0 1 [97, 119, 97] 4036 1 0x8b00 0x2000
134 122 -918 784 73 0 1 [97, 119, 97] 4036 1 0x8900 0x2000
122 123 -942 786 70 0 1 [97, 119, 97] 3470 0 0x8600 0x2000
123 123 -940 786 65 0 1 [97, 119, 97] 3470 0 0x8100 0x2000
124 123 -938 786 65 0 1 [97, 119, 97] 3470 0 0x8100 0x2000
125 123 -936 786 62 1 1 [63, 118, 228] 3470 0 0x7e00 0xd800
126 123 -934 786 62 1 1 [63, 118, 228] 3470 0 0x7e00 0xd800
127 123 -932 786 62 1 1 [63, 118, 228] 3470 0 0x7e00 0xd800
128 123 -930 786 62 1 1 [63, 118, 228] 3470 0 0x7e00 0xd800
129 123 -928 786 73 0 1 [97, 119, 97] 3470 0 0x8900 0x2000
130 123 -926 786 67 0 1 [134, 183, 131] 3470 0 0x8300 0x4100
131 123 -924 786 69 0 1 [134, 183, 131] 3470 0 0x8500 0x4100
132 123 -922 786 77 0 1 [97, 119, 97] 4036 1 0x8d00 0x2000
133 123 -920 786 77 0 1 [97, 119, 97] 4036 1 0x8d00 0x2000
134 123 -918 786 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
122 124 -942 788 72 0 1 [97, 119, 97] 3470 0 0x8800 0x2000
123 124 -940 788 70 0 1 [97, 119, 97] 3470 0 0x8600 0x2000
124 124 -938 788 68 0 1 [97, 119, 97] 3470 0 0x8400 0x2000
125 124 -936 788 62 1 1 [63, 118, 228] 3470 0 0x7e00 0xd800
126 124 -934 788 62 1 1 [63, 118, 228] 3470 0 0x7e00 0xd800
127 124 -932 788 62 1 1 [63, 118, 228] 3470 0 0x7e00 0xd800
128 124 -930 788 73 0 1 [97, 119, 97] 3470 0 0x8900 0x2000
129 124 -928 788 76 0 1 [97, 119, 97] 3470 0 0x8c00 0x2000
130 124 -926 788 73 0 1 [97, 119, 97] 3470 0 0x8900 0x2000
131 124 -924 788 69 0 1 [134, 183, 131] 3470 0 0x8500 0x4100
132 124 -922 788 77 0 1 [97, 119, 97] 4036 1 0x8d00 0x2000
133 124 -920 788 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 124 -918 788 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x2000
122 125 -942 790 74 0 1 [97, 119, 97] 3470 0 0x8a00 0x2000
123 125 -940 790 73 0 1 [97, 119, 97] 3470 0 0x8900 0x2000
124 125 -938 790 71 0 1 [97, 119, 97] 3470 0 0x8700 0x2000
125 125 -936 790 62 1 1 [63, 118, 228] 3470 0 0x7e00 0xd800
126 125 -934 790 62 1 1 [63, 118, 228] 3470 0 0x7e00 0xd800
127 125 -932 790 62 1 1 [63, 118, 228] 3470 0 0x7e00 0xd800
128 125 -930 790 62 1 1 [63, 118, 228] 106 0 0x7e00 0xd800
129 125 -928 790 73 0 1 [97, 119, 97] 106 0 0x8900 0x2000
130 125 -926 790 68 0 1 [134, 183, 131] 106 0 0x8400 0x4100
131 125 -924 790 69 0 1 [134, 183, 131] 106 0 0x8500 0x4100
132 125 -922 790 69 0 1 [134, 183, 131] 4036 1 0x8500 0x4100
133 125 -920 790 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 125 -918 790 70 0 1 [134, 183, 131] 18446744073709551615 1 0x8600 0x4100
122 126 -942 792 74 0 1 [97, 119, 97] 3470 0 0x8a00 0x2000
123 126 -940 792 73 0 1 [97, 119, 97] 3470 0 0x8900 0x2000
124 126 -938 792 68 0 1 [97, 119, 97] 3470 0 0x8400 0x2000
125 126 -936 792 62 1 1 [63, 118, 228] 3470 0 0x7e00 0xd800
126 126 -934 792 62 1 1 [63, 118, 228] 3470 0 0x7e00 0xd800
127 126 -932 792 62 1 1 [63, 118, 228] 3470 0 0x7e00 0xd800
128 126 -930 792 62 1 1 [63, 118, 228] 106 0 0x7e00 0xd800
129 126 -928 792 67 0 1 [134, 183, 131] 106 0 0x8300 0x4100
130 126 -926 792 74 0 1 [97, 119, 97] 106 0 0x8a00 0x2000
131 126 -924 792 77 0 1 [97, 119, 97] 3470 0 0x8d00 0x2000
132 126 -922 792 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 126 -920 792 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3200
134 126 -918 792 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x5000
122 127 -942 794 67 0 1 [134, 183, 131] 3470 0 0x8300 0x4100
123 127 -940 794 68 0 1 [134, 183, 131] 3470 0 0x8400 0x4100
124 127 -938 794 65 0 1 [134, 183, 131] 3470 0 0x8100 0x4100
125 127 -936 794 62 1 1 [63, 118, 228] 106 0 0x7e00 0xd800
126 127 -934 794 62 1 1 [63, 118, 228] 3470 0 0x7e00 0xd800
127 127 -932 794 170 0 1 [142, 33, 33] 3470 0 0xea00 0x0a00
128 127 -930 794 170 0 1 [142, 33, 33] 3470 0 0xea00 0x0a00
129 127 -928 794 170 0 1 [142, 33, 33] 106 0 0xea00 0x0a00
130 127 -926 794 74 0 1 [97, 119, 97] 106 0 0x8a00 0x2000
131 127 -924 794 77 0 1 [97, 119, 97] 106 0 0x8d00 0x2000
132 127 -922 794 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 127 -920 794 69 0 1 [113, 167, 77] 18446744073709551615 1 0x8500 0x3200
134 127 -918 794 70 0 1 [219, 211, 160] 18446744073709551615 1 0x8600 0x5000
122 128 -942 796 68 0 1 [134, 183, 131] 3470 0 0x8400 0x4100
123 128 -940 796 72 0 1 [97, 119, 97] 3470 0 0x8800 0x2000
124 128 -938 796 75 0 1 [97, 119, 97] 3470 0 0x8b00 0x2000
125 128 -936 796 72 0 1 [97, 119, 97] 106 0 0x8800 0x2000
126 128 -934 796 62 1 1 [63, 118, 228] 3470 0 0x7e00 0xd800
127 128 -932 796 170 0 1 [142, 33, 33] 3470 0 0xea00 0x0a00
128 128 -930 796 170 0 1 [142, 33, 33] 3470 0 0xea00 0x0a00
129 128 -928 796 170 0 1 [142, 33, 33] 106 0 0xea00 0x0a00
130 128 -926 796 68 0 1 [134, 183, 131] 106 0 0x8400 0x4100
131 128 -924 796 74 0 1 [97, 119, 97] 3470 0 0x8a00 0x2000
132 128 -922 796 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 128 -920 796 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
134 128 -918 796 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 129 -942 798 69 0 1 [134, 183, 131] 3470 0 0x8500 0x4100
123 129 -940 798 75 0 1 [97, 119, 97] 3470 0 0x8b00 0x2000
124 129 -938 798 78 0 1 [97, 119, 97] 3470 0 0x8e00 0x2000
125 129 -936 798 75 0 1 [97, 119, 97] 106 0 0x8b00 0x2000
126 129 -934 798 62 1 1 [63, 118, 228] 3470 0 0x7e00 0xd800
127 129 -932 798 170 0 1 [142, 33, 33] 106 0 0xea00 0x0a00
128 129 -930 798 170 0 1 [142, 33, 33] 106 0 0xea00 0x0a00
129 129 -928 798 170 0 1 [142, 33, 33] 106 0 0xea00 0x0a00
130 129 -926 798 71 0 1 [97, 119, 97] 106 0 0x8700 0x2000
131 129 -924 798 69 0 1 [134, 183, 131] 3470 0 0x8500 0x4100
132 129 -922 798 74 0 1 [97, 119, 97] 18446744073709551615 1 0x8a00 0x2000
133 129 -920 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 129 -918 798 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 130 -942 800 70 0 1 [134, 183, 131] 3469 0 0x8600 0x4100
123 130 -940 800 76 0 1 [97, 119, 97] 3469 0 0x8c00 0x2000
124 130 -938 800 76 0 1 [97, 119, 97] 3469 0 0x8c00 0x2000
125 130 -936 800 72 0 1 [97, 119, 97] 106 0 0x8800 0x2000
126 130 -934 800 62 1 1 [63, 118, 228] 3469 0 0x7e00 0xd800
127 130 -932 800 62 1 1 [63, 118, 228] 3469 0 0x7e00 0xd800
128 130 -930 800 62 1 1 [63, 118, 228] 106 0 0x7e00 0xd800
129 130 -928 800 71 0 1 [97, 119, 97] 106 0 0x8700 0x2000
130 130 -926 800 71 0 1 [97, 119, 97] 106 0 0x8700 0x2000
131 130 -924 800 69 0 1 [134, 183, 131] 106 0 0x8500 0x4100
132 130 -922 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
133 130 -920 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 130 -918 800 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 131 -942 802 78 0 1 [97, 119, 97] 3469 0 0x8e00 0x2000
123 131 -940 802 77 0 1 [97, 119, 97] 3469 0 0x8d00 0x2000
124 131 -938 802 77 0 1 [97, 119, 97] 3469 0 0x8d00 0x2000
125 131 -936 802 66 0 1 [134, 183, 131] 106 0 0x8200 0x4100
126 131 -934 802 62 1 1 [63, 118, 228] 3469 0 0x7e00 0xd800
127 131 -932 802 62 1 1 [63, 118, 228] 3469 0 0x7e00 0xd800
128 131 -930 802 62 1 1 [63, 118, 228] 3469 0 0x7e00 0xd800
129 131 -928 802 62 1 1 [63, 118, 228] 106 0 0x7e00 0xd800
130 131 -926 802 66 0 1 [134, 183, 131] 106 0 0x8200 0x4100
131 131 -924 802 68 0 1 [134, 183, 131] 3469 0 0x8400 0x4100
132 131 -922 802 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 131 -920 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 131 -918 802 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
122 132 -942 804 79 0 1 [97, 119, 97] 3469 0 0x8f00 0x2000
123 132 -940 804 76 0 1 [97, 119, 97] 3469 0 0x8c00 0x2000
124 132 -938 804 76 0 1 [97, 119, 97] 3469 0 0x8c00 0x2000
125 132 -936 804 76 0 1 [97, 119, 97] 3469 0 0x8c00 0x2000
126 132 -934 804 62 1 1 [63, 118, 228] 3469 0 0x7e00 0xd800
127 132 -932 804 62 1 1 [63, 118, 228] 3469 0 0x7e00 0xd800
128 132 -930 804 62 1 1 [63, 118, 228] 3469 0 0x7e00 0xd800
129 132 -928 804 62 1 1 [63, 118, 228] 3469 0 0x7e00 0xd800
130 132 -926 804 65 0 1 [134, 183, 131] 3469 0 0x8100 0x4100
131 132 -924 804 77 0 1 [97, 119, 97] 3469 0 0x8d00 0x2000
132 132 -922 804 79 0 1 [97, 119, 97] 3469 0 0x8f00 0x2000
133 132 -920 804 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
134 132 -918 804 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x2000
122 133 -942 806 78 0 1 [97, 119, 97] 3469 0 0x8e00 0x2000
123 133 -940 806 69 0 1 [134, 183, 131] 3469 0 0x8500 0x4100
124 133 -938 806 155 0 1 [134, 183, 131] 3469 0 0xdb00 0x4100
125 133 -936 806 155 0 1 [134, 183, 131] 3469 0 0xdb00 0x4100
126 133 -934 806 155 0 1 [134, 183, 131] 3469 0 0xdb00 0x4100
127 133 -932 806 155 0 1 [134, 183, 131] 3469 0 0xdb00 0x4100
128 133 -930 806 155 0 1 [134, 183, 131] 3469 0 0xdb00 0x4100
129 133 -928 806 155 0 1 [134, 183, 131] 3469 0 0xdb00 0x4100
130 133 -926 806 155 0 1 [134, 183, 131] 3469 0 0xdb00 0x4100
131 133 -924 806 67 0 1 [134, 183, 131] 3469 0 0x8300 0x4100
132 133 -922 806 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x2000
133 133 -920 806 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 133 -918 806 72 0 1 [97, 119, 97] 18446744073709551615 1 0x8800 0x2000
122 134 -942 808 79 0 1 [97, 119, 97] 3469 0 0x8f00 0x2000
123 134 -940 808 68 0 1 [134, 183, 131] 3469 0 0x8400 0x4100
124 134 -938 808 155 0 1 [134, 183, 131] 3469 0 0xdb00 0x4100
125 134 -936 808 180 0 1 [134, 183, 131] 3469 0 0xf400 0x4100
126 134 -934 808 180 0 1 [134, 183, 131] 3469 0 0xf400 0x4100
127 134 -932 808 180 0 1 [134, 183, 131] 3469 0 0xf400 0x4100
128 134 -930 808 180 0 1 [134, 183, 131] 3469 0 0xf400 0x4100
129 134 -928 808 180 0 1 [134, 183, 131] 3469 0 0xf400 0x4100
130 134 -926 808 155 0 1 [134, 183, 131] 3469 0 0xdb00 0x4100
131 134 -924 808 67 0 1 [134, 183, 131] 3469 0 0x8300 0x4100
132 134 -922 808 68 0 1 [134, 183, 131] 18446744073709551615 1 0x8400 0x4100
133 134 -920 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
134 134 -918 808 69 0 1 [134, 183, 131] 18446744073709551615 1 0x8500 0x4100
```
- edit channel: 0 linkage errors, 0 failed event dispatches (both must be 0)
- phase: 6 parity world boot (19:21:19)
- parity world: view-distance=8 (the voxy leg's server value)
- parity world: joined session 3 (19:21:44)
- parity world: scan populated (cells=65536; the march paints from these buffers)
- parity world tour: 13 vantages (production pack, vd 8, shared structures)
- coverage sweeps: 72 eye passes = 36 positions (12 az x r144/200 + 12 off-az x r172, y=160) x 2 looks (outward+inward), settle-by-laps >= 6 s (q stable 3x1s), deadline 12 s/pos, no shots — the v6.26.0 data-coverage lever + v6.26.6 lottery kill; probe rides parity/view_probe.md
- parity drain-settle TIMEOUT: queue 21237 -> 21237 > 11500 after 150s — continuing anyway; the queue did not drain without eye proximity (priority-lane evidence, the 01/05/10 dip attribution reads from the tour deltas)
- pre-tour pass done: 13 vantages x 10s (the tour now reads the second OR-merge lap; the rim/dip transport race reads from the tour deltas)

### boot / stream
```
[c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 149331 bytes, sha1 1a1510762133205cb7b2b3c45fc21c754410f5b9 — zero textures, zero world data, one push per join
[c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[19:21:33 INFO]: [c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 149331 bytes, sha1 1a1510762133205cb7b2b3c45fc21c754410f5b9 — zero textures, zero world data, one push per join
[19:21:33 INFO]: [c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[c-LoD] stream: tick 9600 65536/65536 cells, lap 1046, cone 42126 zone 17196 dirty 18847 (q 21201) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 6721175c/3781e, edits +18073p/13690c/2926e drop 0, spawned 0 / failed 0, cas 0d, cam0 yaw 90 pitch 15, wish 0 forced 830, native 44764c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 1543573p/3955b fail 0 ~80 kB/s cache 24059/24576 h1148015 m395558 e371499, srbeat 114608s/885q drop 69stale/11565undec, ereb 85051s/15q, sweep 0r/76d, cdiag off
[19:37:39 INFO]: [c-LoD] stream: tick 9600 65536/65536 cells, lap 1046, cone 42126 zone 17196 dirty 18847 (q 21201) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 6721175c/3781e, edits +18073p/13690c/2926e drop 0, spawned 0 / failed 0, cas 0d, cam0 yaw 90 pitch 15, wish 0 forced 830, native 44764c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 1543573p/3955b fail 0 ~80 kB/s cache 24059/24576 h1148015 m395558 e371499, srbeat 114608s/885q drop 69stale/11565undec, ereb 85051s/15q, sweep 0r/76d, cdiag off
```
### sessions
```
[19:21:44 INFO]: System chat: Stepan joined the game
[19:37:47 INFO]: System chat: Stepan left the game
```
### client log tail
```
Horizon 0.1.0
Horizon: ignoring minecraft:ambient_entity_effect: no such vanilla particle in this version
Horizon renderer: Vulkan has no conformant adapter (Vulkan: only a software adapter (llvmpipe (LLVM 21.1.8, 256 bits))); asking again for the ones wgpu hides as non-conformant
Horizon renderer: trying OpenGL (Vulkan: only a software adapter (llvmpipe (LLVM 21.1.8, 256 bits)))
libEGL warning: DRI3 error: Could not get DRI3 device
libEGL warning: Ensure your X server supports DRI3 to get accelerated rendering
Horizon renderer: OpenGL has no conformant adapter (OpenGL: only a software adapter (llvmpipe (LLVM 21.1.8, 256 bits))); asking again for the ones wgpu hides as non-conformant
libEGL warning: DRI3 error: Could not get DRI3 device
libEGL warning: Ensure your X server supports DRI3 to get accelerated rendering
Horizon renderer: llvmpipe (LLVM 21.1.8, 256 bits) (Vulkan, Cpu)
Horizon renderer: requesting the device (driver llvmpipe Mesa 26.0.8-1ubuntu0.3 (LLVM 21.1.8)) with features [TEXTURE_FORMAT_16BIT_NORM | TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES | PIPELINE_STATISTICS_QUERY | TIMESTAMP_QUERY_INSIDE_ENCODERS | TIMESTAMP_QUERY_INSIDE_PASSES | MAPPABLE_PRIMARY_BUFFERS | MULTI_DRAW_INDIRECT_COUNT | VERTEX_WRITABLE_STORAGE | CLEAR_TEXTURE | TEXTURE_ATOMIC | SHADER_EARLY_DEPTH_TEST | SUBGROUP | TIMESTAMP_QUERY | INDIRECT_FIRST_INSTANCE | RG11B10UFLOAT_RENDERABLE | FLOAT32_FILTERABLE | FLOAT32_BLENDABLE] and the adapter's limits (buffers up to 2147483647 bytes, storage bindings up to 134217728 bytes, 2048 texture layers)
Horizon renderer: device ready; configuring the surface (1280x720 Bgra8Unorm, Mailbox)
Horizon renderer: the chosen graphics API gave no GPU (No Vulkan or OpenGL graphics driver found: Horizon needs the Vulkan loader (libvulkan.so.1) and a Vulkan driver for your GPU (Mesa's, or NVIDIA's), or OpenGL through EGL (Vulkan: only a software adapter (llvmpipe (LLVM 21.1.8, 256 bits)); OpenGL: only a software adapter (llvmpipe (LLVM 21.1.8, 256 bits)))); rendering on the CPU, which is slow
Horizon renderer: HDR output not offered by this display
Horizon renderer: creating the pipelines
Horizon: UI bundle ready after 418 ms
Horizon: block sprites: 1301 layers at 64 px (largest 64 px, budget 256 MiB)
load core tables: 0.61s
load pack tables: 0.72s
Horizon: core-ready after 723 ms
Horizon: handoff CPU resources prepared on worker in 88 ms
Horizon renderer: pipelines ready (multi-draw on, occlusion culling off, GPU culling unavailable, GPU timer on)
Horizon renderer: acquiring the first frame
Horizon renderer: submitting the first frame
Horizon renderer: presenting the first frame
load items: 0.91s
Horizon: loading screen shown after 926 ms
Horizon: loading the title artwork after 926 ms
Horizon renderer: llvmpipe (LLVM 21.1.8, 256 bits) (Vulkan, Cpu)
Horizon: resources loaded in 1.0s; 35723 block states, 1658 items, 67 biomes; 3 worker threads
Horizon: rest-ready after 952 ms
Horizon renderer: rebuilt on the same device (51 shader modules, layouts and pipelines kept)
Horizon renderer: llvmpipe (LLVM 21.1.8, 256 bits) (Vulkan, Cpu)
Horizon renderer: rebuilt on the same device (51 shader modules, layouts and pipelines kept)
Horizon: world renderer ready after 1388 ms
Horizon: no sounds in the active resources; prepare the game again in the launcher (or run horizon-prepare sounds)
Horizon: no sound: audio off: HORIZON_NO_AUDIO is set
Horizon: waiting for world resources
Horizon: world resources joined after 0 ms
Horizon: startup handoff 259 ms (renderer 241 ms, start_game 12 ms)
Horizon: first frame after 357 ms
Horizon: worldgen data loaded in 546.852091ms
Horizon: block sprites: 1301 layers at 64 px (largest 64 px, budget 256 MiB)
Horizon renderer: llvmpipe (LLVM 21.1.8, 256 bits) (Vulkan, Cpu)
Horizon renderer: rebuilt on the same device (51 shader modules, layouts and pipelines kept)
Failed to load post effect clod:lod_terrain: pass 4: reads the target it writes
XIO:  fatal IO error 2 (No such file or directory) on X server ":99"
      after 50408 requests (422 known processed) with 0 events remaining.
--- exceptions (Realms/RTKit noise filtered) ---
(no client log)
```
### server log tail
```
[19:37:47 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_nether'
[19:37:47 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_nether' in 0.00s
[19:37:47 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_nether'
[19:37:47 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_nether'
[19:37:47 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[19:37:47 INFO]: [ChunkHolderManager] Waiting 60s for chunk system to halt for world 'minecraft:the_end'
[19:37:47 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:the_end'
[19:37:47 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_end'
[19:37:47 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_end' in 0.00s
[19:37:47 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_end'
[19:37:47 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_end'
[19:37:47 INFO]: Waiting for all RegionFile I/O tasks to complete...
[19:37:47 INFO]: All RegionFile I/O tasks to complete
[19:37:47 INFO]: [MoonriseCommon] Awaiting termination of worker pool for up to 60s...
[19:37:47 INFO]: [MoonriseCommon] Awaiting termination of I/O pool for up to 60s...
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[launcher] server exited with code 143
```
### shots
```
-rw-r--r-- 1 runner runner 168754 Nov 21 19:11 11_baseline_settle.png
-rw-r--r-- 1 runner runner 164845 Nov 21 19:12 11_h155_spark.png
-rw-r--r-- 1 runner runner 165844 Nov 21 19:12 11_h165_spark.png
-rw-r--r-- 1 runner runner 166208 Nov 21 19:12 11_h168_spark.png
-rw-r--r-- 1 runner runner 166334 Nov 21 19:13 11_h172_spark.png
-rw-r--r-- 1 runner runner 166151 Nov 21 19:13 11_h176_spark.png
-rw-r--r-- 1 runner runner 166224 Nov 21 19:13 11_h180_spark.png
-rw-r--r-- 1 runner runner 166138 Nov 21 19:13 11_h185_spark.png
-rw-r--r-- 1 runner runner 165750 Nov 21 19:13 12_after_edit_probe0.png
-rw-r--r-- 1 runner runner 177874 Nov 21 19:21 13_after_relog_probe0.png
-rw-r--r-- 1 runner runner 168784 Nov 21 19:21 14_leg2_baseline.png
-rw-r--r-- 1 runner runner 168246 Nov 21 19:21 14_leg2_burst_far.png
-rw-r--r-- 1 runner runner 167724 Nov 21 19:21 14_leg2_burst_greendust.png
-rw-r--r-- 1 runner runner 168196 Nov 21 19:21 14_leg2_burst_near.png
-rw-r--r-- 1 runner runner 146528 Nov 21 19:21 diff_after_relog.png
-rw-r--r-- 1 runner runner 153237 Nov 21 19:13 diff_live_edit.png
-rw-r--r-- 1 runner runner      0 Nov 21 19:11 ffmpeg.err
-rw-r--r-- 1 runner runner   6612 Nov 21 19:13 lens_12_after_edit_probe0.png
-rw-r--r-- 1 runner runner   9434 Nov 21 19:21 lens_13_after_relog_probe0.png
drwxr-xr-x 2 runner runner   4096 Nov 21 19:21 lens_parity
drwxr-xr-x 2 runner runner   4096 Nov 21 19:37 parity
drwxr-xr-x 2 runner runner   4096 Nov 21 19:15 parity_dbg
-rw-r--r-- 1 runner runner    547 Nov 21 19:21 probe_counts.txt
-rw-r--r-- 1 runner runner 570027 Nov 21 19:37 server_full.log
-rw-r--r-- 1 runner runner  39752 Nov 21 19:37 summary.md
```

**exit=1** failures=0 leg1=1 leg2=1
