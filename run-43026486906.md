### c-LoD visual run (-930,796, marker=red_concrete, vantage -1005,150 yaw 270)
- GL:     Vendor: Mesa (0xffffffff);    Device: llvmpipe (LLVM 20.1.2, 256 bits) (0xffffffff);OpenGL renderer string: llvmpipe (LLVM 20.1.2, 256 bits);OpenGL core profile version string: 4.5 (Core Profile) Mesa 25.2.8-0ubuntu0.24.04.4;
- boot + gates OK (13:11:31)
- sky-grid armed
- joined session 1 (13:11:58)
- stream live: cells=65536
- terrain range: 2 terraced mountains at (CX-4,CZ-20)/(CX-2,CZ+16), tops 185/190, grass terraces + stone caps, 66..82 m from the eye (inside the 83.5 m band, chunk edge 64 m)
- scan complete=1 (cells=cells=65536/65536)
- paint settled=1 (wire 49182p)
- baseline try 1: drift=4634 px
- baseline try 2: drift=1307 px
- lod_silhouette_cols=14 (LOD paint ON SCREEN at 75 m if > 20 — the march-drawn marker column; vanilla cannot render there, chunk edge 64 m; #255 re-measures 49, the ZONE_FAR=48 regression #256 and the control read 0)
- 743 terrain-green px in the sky region (LOD MOUNTAINS on screen if > 5000; the range paints ColLow-green on low terraces via the fallback palette; flat-world baselines and the pack-free control read ~0)
- burst_close white px: 553 (renderer alive if > 50)
- burst_far white px: 488 (renderer alive in the marker zone if > 20)
- cut walk 40 m: 0 white px (> 20 = rendered)
- cut walk 60 m: 2 white px (> 20 = rendered)
- cut walk 100 m: 0 white px (> 20 = rendered)
- measured far-render cut: none m (G4 flap instrument; the decode proof needs >= 83 m from the y=151.6 eye)
- fill issued: 5x111x5 red_concrete at -930,796 (zone forceloaded ±48)

### /clod-cells AFTER FILL (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 556 0 0x8400 0x3100
123 122 -940 784 71 0 1 [97, 119, 97] 556 0 0x8700 0x3100
124 122 -938 784 68 0 1 [97, 119, 97] 555 0 0x8400 0x3100
125 122 -936 784 62 1 1 [63, 118, 228] 555 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 554 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 554 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 553 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 552 0 0x8000 0x5c00
130 122 -926 784 66 0 1 [134, 183, 131] 552 0 0x8200 0x5c00
131 122 -924 784 75 0 1 [97, 119, 97] 551 0 0x8b00 0x3100
132 122 -922 784 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 122 -920 784 75 0 1 [97, 119, 97] 18446744073709551615 1 0x8b00 0x3100
134 122 -918 784 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x3100
122 123 -942 786 70 0 1 [97, 119, 97] 556 0 0x8600 0x3100
123 123 -940 786 65 0 1 [97, 119, 97] 556 0 0x8100 0x3100
124 123 -938 786 65 0 1 [97, 119, 97] 555 0 0x8100 0x3100
125 123 -936 786 62 1 1 [63, 118, 228] 555 0 0x7e00 0xd800
```
### stream diag after fill:
[c-LoD] stream: tick 2200 65536/65536 cells, lap 916, cone 13324 zone 827 dirty 895 (q 12179) ewin 177 rbeat 10p/0 rearm 0c/0t rescan 482139c/2448e, edits +5386p/4053c/609e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 313 forced 199, native 65536c/1090ch fail 0 (3w q 0) helper on, trees 1090ch/fail 0, wire bundle 120366p/212b fail 0 ~0 kB/s cache 13466/24576 h106900 m13466 e0, srbeat 0s/0q drop 0stale/0undec
[13:15:22 INFO]: [c-LoD] stream: tick 2200 65536/65536 cells, lap 916, cone 13324 zone 827 dirty 895 (q 12179) ewin 177 rbeat 10p/0 rearm 0c/0t rescan 482139c/2448e, edits +5386p/4053c/609e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 313 forced 199, native 65536c/1090ch fail 0 (3w q 0) helper on, trees 1090ch/fail 0, wire bundle 120366p/212b fail 0 ~0 kB/s cache 13466/24576 h106900 m13466 e0, srbeat 0s/0q drop 0stale/0undec
- probe 0: red=0 paint=2990 lod_silhouette_cols=14 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 1: red=0 paint=2990 lod_silhouette_cols=14 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 2: red=0 paint=2990 lod_silhouette_cols=14 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 3: red=0 paint=2860 lod_silhouette_cols=14 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 4: red=0 paint=2816 lod_silhouette_cols=14 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 5: red=0 paint=2990 lod_silhouette_cols=14 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 6: red=0 paint=2990 lod_silhouette_cols=14 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 7: red=0 paint=2990 lod_silhouette_cols=14 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 8: red=0 paint=2990 lod_silhouette_cols=14 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 9: red=0 paint=2885 lod_silhouette_cols=14 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 10: red=0 paint=2936 lod_silhouette_cols=14 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 11: red=0 paint=3212 lod_silhouette_cols=14 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 12: red=0 paint=9704 lod_silhouette_cols=17 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 13: red=0 paint=9811 lod_silhouette_cols=17 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 14: red=0 paint=9811 lod_silhouette_cols=17 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 15: red=0 paint=9811 lod_silhouette_cols=17 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 16: red=0 paint=9811 lod_silhouette_cols=17 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 17: red=0 paint=9811 lod_silhouette_cols=17 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 18: red=0 paint=15714 lod_silhouette_cols=28 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 19: red=0 paint=16065 lod_silhouette_cols=29 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 20: red=0 paint=16223 lod_silhouette_cols=30 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 21: red=0 paint=16213 lod_silhouette_cols=29 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 22: red=0 paint=17489 lod_silhouette_cols=32 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=1 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 23: red=0 paint=17546 lod_silhouette_cols=34 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- **FAIL:** leg-1 never saw the marker in 24 probes (red < 60 everywhere) — the edit never reached the LOD paint; this failure was SILENT before v6.31.1
### stream diag after probes:
[13:18:33 INFO]: [c-LoD] stream: tick 4100 65536/65536 cells, lap 1616, cone 32362 zone 1705 dirty 938 (q 12211) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1094144c/2517e, edits +6854p/5077c/936e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 313 forced 199, native 65536c/1090ch fail 0 (3w q 0) helper on, trees 1090ch/fail 0, wire bundle 278832p/385b fail 0 ~123 kB/s cache 13501/24576 h265331 m13501 e0, srbeat 0s/0q drop 0stale/0undec
[c-LoD] stream: tick 4200 65536/65536 cells, lap 1716, cone 33841 zone 1705 dirty 938 (q 12211) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1128291c/2517e, edits +6855p/5078c/937e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 313 forced 199, native 65536c/1090ch fail 0 (3w q 0) helper on, trees 1090ch/fail 0, wire bundle 290571p/397b fail 0 ~61 kB/s cache 13501/24576 h277070 m13501 e0, srbeat 0s/0q drop 0stale/0undec
[13:18:43 INFO]: [c-LoD] stream: tick 4200 65536/65536 cells, lap 1716, cone 33841 zone 1705 dirty 938 (q 12211) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1128291c/2517e, edits +6855p/5078c/937e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 313 forced 199, native 65536c/1090ch fail 0 (3w q 0) helper on, trees 1090ch/fail 0, wire bundle 290571p/397b fail 0 ~61 kB/s cache 13501/24576 h277070 m13501 e0, srbeat 0s/0q drop 0stale/0undec
- parity debug tour: 13 vantages (session-1 client, DEBUG pack — evidence only, the gate pairs the §7b world session)
- parity debug tour done: 13 shots in parity_dbg/ (the §7b world session feeds the gate)
- joined session 2 (13:21:40)
- leg-2 baseline scene px (x<940): 20924 (0 = the relog client paints no world at all)
- leg-2 renderer probe NEAR (20 m) white px: 474 raw / 474 changed-vs-baseline (> 20 = the client renders particles)
- leg-2 renderer probe FAR (80 m, marker zone) white px: 353 raw / 349 changed-vs-baseline (> 20 = no distance cut; ~0 with NEAR alive = particles beyond the client chunk radius do not render)
- leg-2 carrier probe green dust (80 m): changed-green px 0 / total changed 3109 vs baseline (> 100 green = a stable green carrier exists; total > 0 with green ~0 = the dust renders but fog washes it — a ruler problem, not a carrier problem)
- relog probe 0: red=0 scene=22162 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 1: red=0 scene=23850 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 2: red=0 scene=26053 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 3: red=0 scene=27728 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 4: red=0 scene=28163 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=1 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 5: red=0 scene=28446 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=1 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 6: red=0 scene=28470 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 7: red=0 scene=28667 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 8: red=0 scene=28823 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 9: red=0 scene=28445 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=1 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 10: red=0 scene=28767 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 11: red=0 scene=28649 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- **FAIL:** marker lost after relog — stream state is not persistent/deterministic

### client log forensics (pipeline/particle/atlas — v6.8.2)
```
ALSA lib confmisc.c:1342:(snd_func_refer) error evaluating name
ALSA lib conf.c:5208:(_snd_config_evaluate) function snd_func_refer returned error: No such file or directory
ALSA lib conf.c:5731:(snd_config_expand) Evaluate error: No such file or directory
[13:21:37] [Render thread/ERROR]: Error starting SoundSystem. Turning off sounds & music
java.lang.IllegalStateException: Failed to open OpenAL device
	at net.minecraft.client.sounds.SoundEngine.reload(SoundEngine.java:110)
	at net.minecraft.server.packs.resources.SimplePreparableReloadListener.lambda$reload$1(SimplePreparableReloadListener.java:15)
	at net.minecraft.server.packs.resources.SimpleReloadInstance.lambda$prepareTasks$3(SimpleReloadInstance.java:59)
[13:21:37] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[13:21:37] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[13:21:37] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[13:21:37] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[13:21:37] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[13:21:37] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[13:21:37] [Render thread/INFO]: Created: 2048x2048x4 minecraft:textures/atlas/blocks.png-atlas
[13:21:37] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[13:21:37] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[13:21:37] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[13:21:37] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[13:21:37] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
[13:21:44] [Render thread/INFO]: Reloading ResourceManager: vanilla, vanilla, server/00000000/6b1f2f7e-9c0d-4a4b-8b0a-b10d1b4ab100
ALSA lib conf.c:5208:(_snd_config_evaluate) function snd_func_card_inum returned error: No such file or directory
ALSA lib confmisc.c:422:(snd_func_concat) error evaluating strings
ALSA lib conf.c:5208:(_snd_config_evaluate) function snd_func_concat returned error: No such file or directory
ALSA lib confmisc.c:1342:(snd_func_refer) error evaluating name
ALSA lib conf.c:5208:(_snd_config_evaluate) function snd_func_refer returned error: No such file or directory
ALSA lib conf.c:5731:(snd_config_expand) Evaluate error: No such file or directory
ALSA lib conf.c:5208:(_snd_config_evaluate) function snd_func_card_inum returned error: No such file or directory
ALSA lib confmisc.c:422:(snd_func_concat) error evaluating strings
ALSA lib conf.c:5208:(_snd_config_evaluate) function snd_func_concat returned error: No such file or directory
ALSA lib confmisc.c:1342:(snd_func_refer) error evaluating name
ALSA lib conf.c:5208:(_snd_config_evaluate) function snd_func_refer returned error: No such file or directory
ALSA lib conf.c:5731:(snd_config_expand) Evaluate error: No such file or directory
[13:21:46] [Render thread/ERROR]: Error starting SoundSystem. Turning off sounds & music
java.lang.IllegalStateException: Failed to open OpenAL device
	at net.minecraft.client.sounds.SoundEngine.reload(SoundEngine.java:110)
	at net.minecraft.server.packs.resources.SimplePreparableReloadListener.lambda$reload$1(SimplePreparableReloadListener.java:15)
	at net.minecraft.server.packs.resources.SimpleReloadInstance.lambda$prepareTasks$3(SimpleReloadInstance.java:59)
[13:21:46] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[13:21:46] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[13:21:46] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[13:21:46] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[13:21:46] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[13:21:46] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[13:21:46] [Render thread/INFO]: Created: 2048x2048x4 minecraft:textures/atlas/blocks.png-atlas
[13:21:46] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[13:21:46] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[13:21:46] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[13:21:46] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[13:21:46] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
```

### /clod-cells AFTER PROBES (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 14 0 0x8400 0x3100
123 122 -940 784 71 0 1 [97, 119, 97] 14 0 0x8700 0x3100
124 122 -938 784 68 0 1 [97, 119, 97] 13 0 0x8400 0x3100
125 122 -936 784 62 1 1 [63, 118, 228] 13 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 12 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 12 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 11 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 10 0 0x8000 0x5c00
130 122 -926 784 66 0 1 [134, 183, 131] 10 0 0x8200 0x5c00
131 122 -924 784 75 0 1 [97, 119, 97] 9 0 0x8b00 0x3100
132 122 -922 784 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 122 -920 784 75 0 1 [97, 119, 97] 18446744073709551615 1 0x8b00 0x3100
134 122 -918 784 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x3100
122 123 -942 786 70 0 1 [97, 119, 97] 14 0 0x8600 0x3100
123 123 -940 786 65 0 1 [97, 119, 97] 14 0 0x8100 0x3100
124 123 -938 786 65 0 1 [97, 119, 97] 13 0 0x8100 0x3100
125 123 -936 786 62 1 1 [63, 118, 228] 13 0 0x7e00 0xd800
```
- edit channel: 0 linkage errors, 0 failed event dispatches (both must be 0)
- parity world: view-distance=8 (the voxy leg's server value)
- parity world: joined session 3 (13:26:57)
- parity world: scan populated (cells=65536; the march paints from these buffers)
- parity world tour: 13 vantages (production pack, vd 8, shared structures)
- coverage sweeps: 72 eye passes = 36 positions (12 az x r144/200 + 12 off-az x r172, y=160) x 2 looks (outward+inward), settle-by-laps >= 6 s (q stable 3x1s), deadline 12 s/pos, no shots — the v6.26.0 data-coverage lever + v6.26.6 lottery kill; probe rides parity/view_probe.md
- parity drain-settle TIMEOUT: queue 30515 -> 30516 > 11500 after 150s — continuing anyway; the queue did not drain without eye proximity (priority-lane evidence, the 01/05/10 dip attribution reads from the tour deltas)
- pre-tour pass done: 13 vantages x 10s (the tour now reads the second OR-merge lap; the rim/dip transport race reads from the tour deltas)
- parity world tour done: 13 shots, 0 night-guard renames in parity/ (the gate pairs voxy/<name>.png vs parity/<name>.png)
- sparse re-beat channel at tour end: streamed=49476 q=0 dropped_stale=39 dropped_undecodable=11809 (frac=8 boot; undecodable-dominant = the rim words die in their decodability window, stale-dominant = edit churn owns the cells)
### parity world stream counters (session 3, tail + maxima)
[c-LoD] stream: tick 13300 65536/65536 cells, lap 8292, cone 130255 zone 22363 dirty 15510 (q 30417) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 9062838c/4128e, edits +18058p/13698c/2866e drop 0, spawned 0 / failed 0, cam0 yaw -180 pitch 5, wish 0 forced 939, native 51520c/956ch fail 0 (3w q 0) helper on, trees 956ch/fail 0, wire bundle 1700820p/3600b fail 0 ~0 kB/s cache 24391/24576 h1308399 m392421 e368030, srbeat 49476s/0q drop 39stale/11809undec
[13:48:55 INFO]: [c-LoD] stream: tick 13300 65536/65536 cells, lap 8292, cone 130255 zone 22363 dirty 15510 (q 30417) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 9062838c/4128e, edits +18058p/13698c/2866e drop 0, spawned 0 / failed 0, cam0 yaw -180 pitch 5, wish 0 forced 939, native 51520c/956ch fail 0 (3w q 0) helper on, trees 956ch/fail 0, wire bundle 1700820p/3600b fail 0 ~0 kB/s cache 24391/24576 h1308399 m392421 e368030, srbeat 49476s/0q drop 39stale/11809undec
- parity stream max cone: 130255
- parity stream max zone: 22363
- parity stream max wire bundle: bundle
### parity world capture lens (debug chain on the §7b world)
lens_red=0 lens_black=0 lens_water=0 lens_dark=0 lens_zero=0 rot=none scan=0 carriers=0
- parity mesh probe @png_y=2: x306=(151,184,255) x307=(151,184,255) x308=(151,184,255) x309h=(151,184,255) x310c=(151,184,255)
- parity mesh probe @png_y=717: x306=(61,77,67) x307=(62,79,68) x308=(64,80,70) x309h=(60,73,65) x310c=(43,61,51)
- parity mesh engagement: magenta-ish=0 of 921600 sampled px
- mask8: the un-fog engagement masks shot on the PARITY WORLD (vd 8) — parity_dbg8/01m|07m|10m
- mask8 01m: green=0 red=0 blue=94176 dim=4096 (px, sampled /16)
- mask8 07m: green=11024 red=0 blue=221824 dim=5872 (px, sampled /16)
- mask8 10m: green=0 red=0 blue=216688 dim=4096 (px, sampled /16)
- mesh probe @png_y=2: x306=(255,255,255) x307=(13,254,0) x308=(0,254,0) x309h=(0,0,0) x310c=(0,0,0)
- mesh probe @png_y=717: x306=(255,255,255) x307=(13,254,0) x308=(0,254,0) x309h=(0,0,0) x310c=(0,0,0)
- mesh engagement: magenta-ish=0 of 921600 sampled px (the vanilla-path paint coverage)
- mask8 fog state @png_y=2: raw (300)=(151,184,255) (301)=(151,184,255)
- mask8 fog state @png_y=717: raw (300)=(0,128,8) (301)=(179,206,255)
- mask8 fog state decoded: envStart=0 envEnd=1024 rdEnd=128 fogColor=(179,206,255)
- buffer dump 10m: marked h=0/2048 c=0/2048; height windows r0<142:0 r1:0 r2>=202:0 maxByte=0
- buffer dump 10m: color idx0(black)=0 water/height mismatches=0 of 0 pairs
- buffer dump 01m: marked h=0/2048 c=0/2048; height windows r0<142:0 r1:0 r2>=202:0 maxByte=0
- buffer dump 01m: color idx0(black)=0 water/height mismatches=0 of 0 pairs
- rim mask: scan populated (cells=65536; the state paint reads the same buffers the march reads)
- rim mask: decode-state frames shot (rim_mask/07|01|12 — GREEN full decode / RED colorless / BLUE heightless)
- rim mask 07: green=1200 red=0 blue=416 (px, sampled /16) -> DOMINANT GREEN: rim fully decoded — the fallback is the PAINT path (shade/fog/composite), not data
- rim mask 01: green=144 red=11232 blue=10480 (px, sampled /16) -> DOMINANT RED: rim = COLORLESS class — the color words die client-side (pool competition / fog strobe margin / the v6.23 HeightRef capture gate); heights ARE decoded
- rim mask 12: green=1200 red=0 blue=400 (px, sampled /16) -> DOMINANT GREEN: rim fully decoded — the fallback is the PAINT path (shade/fog/composite), not data

### boot / stream
```
[c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 124538 bytes, sha1 fe5469942cc8784a45bb58789463397df61e3b4f — zero textures, zero world data, one push per join
[13:52:51 INFO]: [c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 124538 bytes, sha1 fe5469942cc8784a45bb58789463397df61e3b4f — zero textures, zero world data, one push per join
[c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[13:52:51 INFO]: [c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[c-LoD] stream: tick 1800 65536/65536 cells, lap 1610, cone 29078 zone 1785 dirty 20 (q 8655) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1095652c/609e, edits +28p/28c/9e drop 0, spawned 0 / failed 0, cam0 yaw -180 pitch 5, wish 0 forced 935, native 9088c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 323731p/721b fail 0 ~0 kB/s cache 24447/24576 h299284 m24447 e0, srbeat 9773s/43q drop 0stale/1567undec
[13:56:00 INFO]: [c-LoD] stream: tick 1800 65536/65536 cells, lap 1610, cone 29078 zone 1785 dirty 20 (q 8655) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1095652c/609e, edits +28p/28c/9e drop 0, spawned 0 / failed 0, cam0 yaw -180 pitch 5, wish 0 forced 935, native 9088c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 323731p/721b fail 0 ~0 kB/s cache 24447/24576 h299284 m24447 e0, srbeat 9773s/43q drop 0stale/1567undec
```
### sessions
```
[13:53:17 INFO]: System chat: Stepan joined the game
[13:56:03 INFO]: System chat: Stepan left the game
```
### client log tail
```
[13:56:02] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[13:56:02] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:entity_position_sync] net.minecraft.network.protocol.game.ClientboundEntityPositionSyncPacket -> 37 bytes
[13:56:02] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:entity_position_sync] net.minecraft.network.protocol.game.ClientboundEntityPositionSyncPacket -> 37 bytes
[13:56:02] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:move_player_pos] net.minecraft.network.protocol.game.ServerboundMovePlayerPacket$Pos -> 26 bytes
[13:56:02] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[13:56:02] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[13:56:02] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:entity_position_sync] net.minecraft.network.protocol.game.ClientboundEntityPositionSyncPacket -> 37 bytes
[13:56:02] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:keep_alive] net.minecraft.network.protocol.common.ClientboundKeepAlivePacket -> 9 bytes
[13:56:02] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:keep_alive] net.minecraft.network.protocol.common.ServerboundKeepAlivePacket -> 9 bytes
[13:56:02] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[13:56:02] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:entity_position_sync] net.minecraft.network.protocol.game.ClientboundEntityPositionSyncPacket -> 37 bytes
[13:56:02] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:entity_position_sync] net.minecraft.network.protocol.game.ClientboundEntityPositionSyncPacket -> 37 bytes
[13:56:02] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[13:56:02] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:set_time] net.minecraft.network.protocol.game.ClientboundSetTimePacket -> 10 bytes
[13:56:02] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:entity_position_sync] net.minecraft.network.protocol.game.ClientboundEntityPositionSyncPacket -> 37 bytes
[13:56:02] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[13:56:02] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:block_update] net.minecraft.network.protocol.game.ClientboundBlockUpdatePacket -> 11 bytes
[13:56:02] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[13:56:02] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:entity_position_sync] net.minecraft.network.protocol.game.ClientboundEntityPositionSyncPacket -> 37 bytes
[13:56:02] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:entity_position_sync] net.minecraft.network.protocol.game.ClientboundEntityPositionSyncPacket -> 37 bytes
[13:56:02] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[13:56:02] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[13:56:02] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:entity_position_sync] net.minecraft.network.protocol.game.ClientboundEntityPositionSyncPacket -> 37 bytes
[13:56:02] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[13:56:02] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:entity_position_sync] net.minecraft.network.protocol.game.ClientboundEntityPositionSyncPacket -> 37 bytes
[13:56:02] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:entity_position_sync] net.minecraft.network.protocol.game.ClientboundEntityPositionSyncPacket -> 37 bytes
[13:56:02] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[13:56:02] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[13:56:03] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:entity_position_sync] net.minecraft.network.protocol.game.ClientboundEntityPositionSyncPacket -> 37 bytes
[13:56:03] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
```
### server log tail
```
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[13:56:04 INFO]: [ChunkHolderManager] Saved 1444 block chunks, 1418 entity chunks, 0 poi chunks in world 'minecraft:overworld' in 1.15s
[13:56:04 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:overworld'
[13:56:04 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:overworld'
[13:56:04 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[13:56:04 INFO]: [ChunkHolderManager] Waiting 60s for chunk system to halt for world 'minecraft:the_nether'
[13:56:04 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:the_nether'
[13:56:04 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_nether'
[13:56:04 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_nether' in 0.00s
[13:56:04 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_nether'
[13:56:04 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_nether'
[13:56:04 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[13:56:04 INFO]: [ChunkHolderManager] Waiting 60s for chunk system to halt for world 'minecraft:the_end'
[13:56:04 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:the_end'
[13:56:04 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_end'
[13:56:04 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_end' in 0.00s
[13:56:04 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_end'
[13:56:04 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_end'
[13:56:04 INFO]: Waiting for all RegionFile I/O tasks to complete...
[13:56:04 INFO]: All RegionFile I/O tasks to complete
[13:56:04 INFO]: [MoonriseCommon] Awaiting termination of worker pool for up to 60s...
[13:56:04 INFO]: [MoonriseCommon] Awaiting termination of I/O pool for up to 60s...
[launcher] server exited with code 143
```
### shots
```
-rw-r--r-- 1 runner runner  6264 Nov 11 13:16 lens_12_after_edit_probe4.png
-rw-r--r-- 1 runner runner  6121 Nov 11 13:16 lens_12_after_edit_probe5.png
-rw-r--r-- 1 runner runner  6121 Nov 11 13:16 lens_12_after_edit_probe6.png
-rw-r--r-- 1 runner runner  6121 Nov 11 13:16 lens_12_after_edit_probe7.png
-rw-r--r-- 1 runner runner  6121 Nov 11 13:16 lens_12_after_edit_probe8.png
-rw-r--r-- 1 runner runner  6954 Nov 11 13:16 lens_12_after_edit_probe9.png
-rw-r--r-- 1 runner runner  6051 Nov 11 13:24 lens_13_after_relog_probe0.png
-rw-r--r-- 1 runner runner  6722 Nov 11 13:24 lens_13_after_relog_probe1.png
-rw-r--r-- 1 runner runner  5980 Nov 11 13:25 lens_13_after_relog_probe10.png
-rw-r--r-- 1 runner runner  6532 Nov 11 13:26 lens_13_after_relog_probe11.png
-rw-r--r-- 1 runner runner  6147 Nov 11 13:24 lens_13_after_relog_probe2.png
-rw-r--r-- 1 runner runner  5802 Nov 11 13:24 lens_13_after_relog_probe3.png
-rw-r--r-- 1 runner runner  6541 Nov 11 13:25 lens_13_after_relog_probe4.png
-rw-r--r-- 1 runner runner  6001 Nov 11 13:25 lens_13_after_relog_probe5.png
-rw-r--r-- 1 runner runner  5985 Nov 11 13:25 lens_13_after_relog_probe6.png
-rw-r--r-- 1 runner runner  6264 Nov 11 13:25 lens_13_after_relog_probe7.png
-rw-r--r-- 1 runner runner  5966 Nov 11 13:25 lens_13_after_relog_probe8.png
-rw-r--r-- 1 runner runner  6539 Nov 11 13:25 lens_13_after_relog_probe9.png
drwxr-xr-x 2 runner runner  4096 Nov 11 13:49 lens_parity
drwxr-xr-x 2 runner runner  4096 Nov 11 13:49 parity
drwxr-xr-x 2 runner runner  4096 Nov 11 13:21 parity_dbg
drwxr-xr-x 2 runner runner  4096 Nov 11 13:52 parity_dbg8
-rw-r--r-- 1 runner runner  6751 Nov 11 13:26 probe_counts.txt
drwxr-xr-x 2 runner runner  4096 Nov 11 13:56 rim_mask
-rw-r--r-- 1 runner runner 34054 Nov 11 13:56 summary.md
```

**exit=1** failures=2 leg1=0 leg2=0
