### c-LoD visual run (-930,796, marker=red_concrete, vantage -1005,150 yaw 270)
- GL:     Vendor: Mesa (0xffffffff);    Device: llvmpipe (LLVM 20.1.2, 256 bits) (0xffffffff);OpenGL renderer string: llvmpipe (LLVM 20.1.2, 256 bits);OpenGL core profile version string: 4.5 (Core Profile) Mesa 25.2.8-0ubuntu0.24.04.4;
- boot + gates OK (17:34:33)
- sky-grid armed
- joined session 1 (17:34:54)
- stream live: cells=65536
- terrain range: 2 terraced mountains at (CX-4,CZ-20)/(CX-2,CZ+16), tops 185/190, grass terraces + stone caps, 66..82 m from the eye (inside the 83.5 m band, chunk edge 64 m)
- scan complete=1 (cells=cells=65536/65536)
- paint settled=1 (wire 57731p)
- baseline try 1: drift=0 px
- lod_silhouette_cols=15 (LOD paint ON SCREEN at 75 m if > 20 — the march-drawn marker column; vanilla cannot render there, chunk edge 64 m; #255 re-measures 49, the ZONE_FAR=48 regression #256 and the control read 0)
- 0 terrain-green px in the sky region (LOD MOUNTAINS on screen if > 5000; the range paints ColLow-green on low terraces via the fallback palette; flat-world baselines and the pack-free control read ~0)
- burst_close white px: 536 (renderer alive if > 50)
- burst_far white px: 139 (renderer alive in the marker zone if > 20)
- cut walk 40 m: 78 white px (> 20 = rendered)
- cut walk 60 m: 0 white px (> 20 = rendered)
- cut walk 100 m: 0 white px (> 20 = rendered)
- measured far-render cut: 40 m (G4 flap instrument; the decode proof needs >= 83 m from the y=151.6 eye)
- fill issued: 5x111x5 red_concrete at -930,796 (zone forceloaded ±48)

### /clod-cells AFTER FILL (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 15 0 0x8400 0x3100
123 122 -940 784 71 0 1 [97, 119, 97] 15 0 0x8700 0x3100
124 122 -938 784 68 0 1 [97, 119, 97] 14 0 0x8400 0x3100
125 122 -936 784 62 1 1 [63, 118, 228] 14 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 13 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 13 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 12 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 12 0 0x8000 0x5c00
130 122 -926 784 66 0 1 [134, 183, 131] 11 0 0x8200 0x5c00
131 122 -924 784 75 0 1 [97, 119, 97] 11 0 0x8b00 0x3100
132 122 -922 784 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 122 -920 784 75 0 1 [97, 119, 97] 18446744073709551615 1 0x8b00 0x3100
134 122 -918 784 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x3100
122 123 -942 786 70 0 1 [97, 119, 97] 15 0 0x8600 0x3100
123 123 -940 786 65 0 1 [97, 119, 97] 15 0 0x8100 0x3100
124 123 -938 786 65 0 1 [97, 119, 97] 14 0 0x8100 0x3100
125 123 -936 786 62 1 1 [63, 118, 228] 14 0 0x7e00 0xd800
```
### stream diag after fill:
[c-LoD] stream: tick 1800 65536/65536 cells, lap 743, cone 9241 zone 1441 dirty 798 (q 12495) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 390725c/2684e, edits +5230p/3923c/535e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 322 forced 190, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 91866p/179b fail 0 ~118 kB/s cache 12613/24576 h79253 m12613 e0
[17:37:43 INFO]: [c-LoD] stream: tick 1800 65536/65536 cells, lap 743, cone 9241 zone 1441 dirty 798 (q 12495) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 390725c/2684e, edits +5230p/3923c/535e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 322 forced 190, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 91866p/179b fail 0 ~118 kB/s cache 12613/24576 h79253 m12613 e0
- probe 0: red=0 paint=8408 lod_silhouette_cols=17 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 1: red=0 paint=11376 lod_silhouette_cols=23 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 2: red=0 paint=12433 lod_silhouette_cols=24 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=1 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 3: red=0 paint=13176 lod_silhouette_cols=25 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 4: red=0 paint=13166 lod_silhouette_cols=25 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=6 (need red>=60 paint>=300 lod>20)
- probe 5: red=0 paint=12974 lod_silhouette_cols=25 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 6: red=0 paint=12974 lod_silhouette_cols=25 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 7: red=0 paint=12974 lod_silhouette_cols=25 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 8: red=0 paint=12974 lod_silhouette_cols=25 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 9: red=0 paint=12974 lod_silhouette_cols=25 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 10: red=0 paint=13341 lod_silhouette_cols=25 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 11: red=0 paint=13434 lod_silhouette_cols=24 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 12: red=0 paint=13997 lod_silhouette_cols=25 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 13: red=0 paint=13815 lod_silhouette_cols=25 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 14: red=0 paint=13815 lod_silhouette_cols=25 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 15: red=0 paint=13815 lod_silhouette_cols=25 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 16: red=0 paint=13815 lod_silhouette_cols=25 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 17: red=0 paint=13815 lod_silhouette_cols=25 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 18: red=0 paint=13815 lod_silhouette_cols=25 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 19: red=0 paint=14094 lod_silhouette_cols=24 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 20: red=0 paint=14277 lod_silhouette_cols=24 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=1 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 21: red=0 paint=15784 lod_silhouette_cols=25 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 22: red=0 paint=15824 lod_silhouette_cols=25 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 23: red=0 paint=15868 lod_silhouette_cols=25 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
### stream diag after probes:
[17:40:44 INFO]: [c-LoD] stream: tick 3600 65536/65536 cells, lap 1623, cone 30421 zone 1541 dirty 824 (q 12544) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 972224c/2759e, edits +6952p/5156c/950e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 322 forced 190, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 260270p/367b fail 0 ~58 kB/s cache 12670/24576 h247600 m12670 e0
[c-LoD] stream: tick 3700 65536/65536 cells, lap 1723, cone 33245 zone 1541 dirty 824 (q 12544) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1004557c/2759e, edits +6953p/5157c/950e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 322 forced 190, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 282586p/391b fail 0 ~117 kB/s cache 12670/24576 h269916 m12670 e0
[17:40:54 INFO]: [c-LoD] stream: tick 3700 65536/65536 cells, lap 1723, cone 33245 zone 1541 dirty 824 (q 12544) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1004557c/2759e, edits +6953p/5157c/950e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 322 forced 190, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 282586p/391b fail 0 ~117 kB/s cache 12670/24576 h269916 m12670 e0
- parity debug tour: 13 vantages (session-1 client, DEBUG pack — evidence only, the gate pairs the §7b world session)
- parity debug tour done: 13 shots in parity_dbg/ (the §7b world session feeds the gate)
- joined session 2 (17:43:51)
- leg-2 baseline scene px (x<940): 24282 (0 = the relog client paints no world at all)
- leg-2 renderer probe NEAR (20 m) white px: 536 raw / 536 changed-vs-baseline (> 20 = the client renders particles)
- leg-2 renderer probe FAR (80 m, marker zone) white px: 116 raw / 116 changed-vs-baseline (> 20 = no distance cut; ~0 with NEAR alive = particles beyond the client chunk radius do not render)
- leg-2 carrier probe green dust (80 m): changed-green px 0 / total changed 599 vs baseline (> 100 green = a stable green carrier exists; total > 0 with green ~0 = the dust renders but fog washes it — a ruler problem, not a carrier problem)
- relog probe 0: red=0 scene=24338 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 1: red=0 scene=24372 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=17 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 2: red=0 scene=25837 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 3: red=0 scene=26316 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 4: red=0 scene=26337 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 5: red=0 scene=26158 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=1 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 6: red=0 scene=31086 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 7: red=0 scene=31321 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 8: red=0 scene=31234 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 9: red=0 scene=33040 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=1 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 10: red=0 scene=34610 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 11: red=0 scene=34807 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=6 (need red >= 60; scene > 0 = the client paints a world at all)
- **FAIL:** marker lost after relog — stream state is not persistent/deterministic

### client log forensics (pipeline/particle/atlas — v6.8.2)
```
ALSA lib confmisc.c:1342:(snd_func_refer) error evaluating name
ALSA lib conf.c:5208:(_snd_config_evaluate) function snd_func_refer returned error: No such file or directory
ALSA lib conf.c:5731:(snd_config_expand) Evaluate error: No such file or directory
[17:43:49] [Render thread/ERROR]: Error starting SoundSystem. Turning off sounds & music
java.lang.IllegalStateException: Failed to open OpenAL device
	at net.minecraft.client.sounds.SoundEngine.reload(SoundEngine.java:110)
	at net.minecraft.server.packs.resources.SimplePreparableReloadListener.lambda$reload$1(SimplePreparableReloadListener.java:15)
	at net.minecraft.server.packs.resources.SimpleReloadInstance.lambda$prepareTasks$3(SimpleReloadInstance.java:59)
[17:43:49] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[17:43:49] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[17:43:49] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[17:43:49] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[17:43:49] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[17:43:49] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[17:43:49] [Render thread/INFO]: Created: 2048x2048x4 minecraft:textures/atlas/blocks.png-atlas
[17:43:49] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[17:43:49] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[17:43:49] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[17:43:49] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[17:43:49] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
[17:43:55] [Render thread/INFO]: Reloading ResourceManager: vanilla, vanilla, server/00000000/6b1f2f7e-9c0d-4a4b-8b0a-b10d1b4ab100
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
[17:43:57] [Render thread/ERROR]: Error starting SoundSystem. Turning off sounds & music
java.lang.IllegalStateException: Failed to open OpenAL device
	at net.minecraft.client.sounds.SoundEngine.reload(SoundEngine.java:110)
	at net.minecraft.server.packs.resources.SimplePreparableReloadListener.lambda$reload$1(SimplePreparableReloadListener.java:15)
	at net.minecraft.server.packs.resources.SimpleReloadInstance.lambda$prepareTasks$3(SimpleReloadInstance.java:59)
[17:43:57] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[17:43:57] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[17:43:57] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[17:43:57] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[17:43:57] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[17:43:57] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[17:43:57] [Render thread/INFO]: Created: 2048x2048x4 minecraft:textures/atlas/blocks.png-atlas
[17:43:57] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[17:43:57] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[17:43:57] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[17:43:57] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[17:43:57] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
```

### /clod-cells AFTER PROBES (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 46 0 0x8400 0x3100
123 122 -940 784 71 0 1 [97, 119, 97] 46 0 0x8700 0x3100
124 122 -938 784 68 0 1 [97, 119, 97] 45 0 0x8400 0x3100
125 122 -936 784 62 1 1 [63, 118, 228] 45 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 44 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 44 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 43 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 43 0 0x8000 0x5c00
130 122 -926 784 66 0 1 [134, 183, 131] 42 0 0x8200 0x5c00
131 122 -924 784 75 0 1 [97, 119, 97] 42 0 0x8b00 0x3100
132 122 -922 784 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 122 -920 784 75 0 1 [97, 119, 97] 18446744073709551615 1 0x8b00 0x3100
134 122 -918 784 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x3100
122 123 -942 786 70 0 1 [97, 119, 97] 46 0 0x8600 0x3100
123 123 -940 786 65 0 1 [97, 119, 97] 46 0 0x8100 0x3100
124 123 -938 786 65 0 1 [97, 119, 97] 45 0 0x8100 0x3100
125 123 -936 786 62 1 1 [63, 118, 228] 45 0 0x7e00 0xd800
```
- edit channel: 0 linkage errors, 0 failed event dispatches (both must be 0)
- parity world: view-distance=8 (the voxy leg's server value)
- parity world: joined session 3 (17:48:52)
- parity world: scan populated (cells=65536; the march paints from these buffers)
- parity world tour: 13 vantages (production pack, vd 8, shared structures)
- coverage sweeps: 72 eye passes = 36 positions (12 az x r144/200 + 12 off-az x r172, y=160) x 2 looks (outward+inward), settle 6 s, no shots — the v6.26.0 data-coverage lever; outer disk rho 32..256 decodes before the tour
- parity drain-settle TIMEOUT: queue 19509 -> 19509 > 11500 after 150s — continuing anyway; the queue did not drain without eye proximity (priority-lane evidence, the 01/05/10 dip attribution reads from the tour deltas)
- **DAY GUARD:** 12_far_north_re still night after re-pin+reshoot (zenith < 130) — renamed night_12_far_north_re.png; the row pairs as missing, NOT as a renderer diff
- parity world tour done: 13 shots, 1 night-guard renames in parity/ (the gate pairs voxy/<name>.png vs parity/<name>.png)
- **FAIL:** parity world time-of-day uncontrollable: 1 vantage(s) night after re-pin+reshoot — the §7b noon-pin class is broken on this runner; the matrix is infrastructure-blocked, not a renderer verdict (DAY GUARD lines + the feed's time-audit.md carry the evidence)
### parity world stream counters (session 3, tail + maxima)
[c-LoD] stream: tick 11400 65536/65536 cells, lap 7136, cone 131342 zone 16097 dirty 7530 (q 19436) ewin 0 rbeat 0p/1 rearm 0c/0t rescan 7720626c/1386e, edits +17974p/13662c/2974e drop 0, spawned 0 / failed 0, cam0 yaw -180 pitch 5, wish 0 forced 936, native 28160c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 1189644p/1317b fail 0 ~0 kB/s cache 22612/24576 h902379 m287265 e264653
[18:07:43 INFO]: [c-LoD] stream: tick 11400 65536/65536 cells, lap 7136, cone 131342 zone 16097 dirty 7530 (q 19436) ewin 0 rbeat 0p/1 rearm 0c/0t rescan 7720626c/1386e, edits +17974p/13662c/2974e drop 0, spawned 0 / failed 0, cam0 yaw -180 pitch 5, wish 0 forced 936, native 28160c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 1189644p/1317b fail 0 ~0 kB/s cache 22612/24576 h902379 m287265 e264653
- parity stream max cone: 131342
- parity stream max zone: 16097
- parity stream max wire bundle: bundle
### parity world capture lens (debug chain on the §7b world)
lens_red=0 lens_black=0 lens_water=0 lens_dark=0 lens_zero=0 rot=none scan=0 carriers=0
- parity mesh probe @png_y=2: x306=(151,184,255) x307=(151,184,255) x308=(151,184,255) x309h=(151,184,255) x310c=(151,184,255)
- parity mesh probe @png_y=717: x306=(86,89,89) x307=(78,85,81) x308=(78,85,82) x309h=(88,92,91) x310c=(103,103,104)
- parity mesh engagement: magenta-ish=0 of 921600 sampled px
- mask8: the un-fog engagement masks shot on the PARITY WORLD (vd 8) — parity_dbg8/01m|07m|10m
- mask8 01m: green=0 red=61824 blue=0 dim=857776 (px, sampled /16)
- mask8 07m: green=32 red=0 blue=1744 dim=839056 (px, sampled /16)
- mask8 10m: green=16 red=112 blue=0 dim=857360 (px, sampled /16)
- mesh probe @png_y=2: x306=(255,255,255) x307=(13,254,0) x308=(0,254,0) x309h=(20,0,0) x310c=(0,0,0)
- mesh probe @png_y=717: x306=(255,255,255) x307=(13,254,0) x308=(0,254,0) x309h=(20,0,0) x310c=(0,0,0)
- mesh engagement: magenta-ish=400 of 921600 sampled px (the vanilla-path paint coverage)
- mask8 fog state @png_y=2: raw (300)=(42,46,58) (301)=(42,46,58)
- mask8 fog state decoded: envStart=336 envEnd=368 rdEnd=928 fogColor=(42,46,58)
- mask8 fog state @png_y=717: raw (300)=(0,12,8) (301)=(41,46,65)
- mask8 fog state decoded: envStart=0 envEnd=96 rdEnd=128 fogColor=(41,46,65)
- buffer dump 10m: marked h=10/2048 c=0/2048; height windows r0<142:10 r1:0 r2>=202:0 maxByte=127
- buffer dump 10m: color idx0(black)=0 water/height mismatches=0 of 0 pairs
- buffer dump 01m: marked h=0/2048 c=0/2048; height windows r0<142:0 r1:0 r2>=202:0 maxByte=0
- buffer dump 01m: color idx0(black)=0 water/height mismatches=0 of 0 pairs
