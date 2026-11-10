### c-LoD visual run (-930,796, marker=red_concrete, vantage -1005,150 yaw 270)
- GL:     Vendor: Mesa (0xffffffff);    Device: llvmpipe (LLVM 20.1.2, 256 bits) (0xffffffff);OpenGL renderer string: llvmpipe (LLVM 20.1.2, 256 bits);OpenGL core profile version string: 4.5 (Core Profile) Mesa 25.2.8-0ubuntu0.24.04.4;
- boot + gates OK (19:47:05)
- sky-grid armed
- joined session 1 (19:47:30)
- stream live: cells=65536
- terrain range: 2 terraced mountains at (CX-4,CZ-20)/(CX-2,CZ+16), tops 185/190, grass terraces + stone caps, 66..82 m from the eye (inside the 83.5 m band, chunk edge 64 m)
- scan complete=1 (cells=cells=65536/65536)
- paint settled=1 (wire 59152p)
- baseline try 1: drift=449 px
- lod_silhouette_cols=25 (LOD paint ON SCREEN at 75 m if > 20 — the march-drawn marker column; vanilla cannot render there, chunk edge 64 m; #255 re-measures 49, the ZONE_FAR=48 regression #256 and the control read 0)
- 0 terrain-green px in the sky region (LOD MOUNTAINS on screen if > 5000; the range paints ColLow-green on low terraces via the fallback palette; flat-world baselines and the pack-free control read ~0)
- burst_close white px: 544 (renderer alive if > 50)
- burst_far white px: 408 (renderer alive in the marker zone if > 20)
- cut walk 40 m: 77 white px (> 20 = rendered)
- cut walk 60 m: 0 white px (> 20 = rendered)
- cut walk 100 m: 0 white px (> 20 = rendered)
- measured far-render cut: 40 m (G4 flap instrument; the decode proof needs >= 83 m from the y=151.6 eye)
- fill issued: 5x111x5 red_concrete at -930,796 (zone forceloaded ±48)

### /clod-cells AFTER FILL (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 53 0 0x8400 0x3100
123 122 -940 784 71 0 1 [97, 119, 97] 53 0 0x8700 0x3100
124 122 -938 784 68 0 1 [97, 119, 97] 52 0 0x8400 0x3100
125 122 -936 784 62 1 1 [63, 118, 228] 52 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 51 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 51 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 50 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 50 0 0x8000 0x5c00
130 122 -926 784 66 0 1 [134, 183, 131] 49 0 0x8200 0x5c00
131 122 -924 784 75 0 1 [97, 119, 97] 49 0 0x8b00 0x3100
132 122 -922 784 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 122 -920 784 75 0 1 [97, 119, 97] 18446744073709551615 1 0x8b00 0x3100
134 122 -918 784 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x3100
122 123 -942 786 70 0 1 [97, 119, 97] 53 0 0x8600 0x3100
123 123 -940 786 65 0 1 [97, 119, 97] 53 0 0x8100 0x3100
124 123 -938 786 65 0 1 [97, 119, 97] 52 0 0x8100 0x3100
125 123 -936 786 62 1 1 [63, 118, 228] 52 0 0x7e00 0xd800
```
### stream diag after fill:
[c-LoD] stream: tick 1900 65536/65536 cells, lap 1016, cone 16402 zone 790 dirty 796 (q 11554) ewin 0 rbeat 0p/1 rearm 0c/0t rescan 403629c/1089e, edits +4234p/3175c/554e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 310 forced 202, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 143672p/226b fail 0 ~61 kB/s cache 12595/24576 h131077 m12595 e0
[19:50:24 INFO]: [c-LoD] stream: tick 1900 65536/65536 cells, lap 1016, cone 16402 zone 790 dirty 796 (q 11554) ewin 0 rbeat 0p/1 rearm 0c/0t rescan 403629c/1089e, edits +4234p/3175c/554e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 310 forced 202, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 143672p/226b fail 0 ~61 kB/s cache 12595/24576 h131077 m12595 e0
- probe 0: red=0 paint=10639 lod_silhouette_cols=32 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 1: red=0 paint=10756 lod_silhouette_cols=33 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 2: red=0 paint=11553 lod_silhouette_cols=33 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 3: red=0 paint=11455 lod_silhouette_cols=33 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 4: red=0 paint=11956 lod_silhouette_cols=33 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 5: red=0 paint=11714 lod_silhouette_cols=33 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 6: red=0 paint=11628 lod_silhouette_cols=33 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 7: red=0 paint=12237 lod_silhouette_cols=33 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 8: red=0 paint=12034 lod_silhouette_cols=33 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 9: red=0 paint=12354 lod_silhouette_cols=33 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 10: red=0 paint=12405 lod_silhouette_cols=34 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 11: red=0 paint=12295 lod_silhouette_cols=34 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 12: red=0 paint=12276 lod_silhouette_cols=34 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 13: red=0 paint=12086 lod_silhouette_cols=34 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 14: red=0 paint=12086 lod_silhouette_cols=34 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 15: red=0 paint=12086 lod_silhouette_cols=34 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 16: red=0 paint=12086 lod_silhouette_cols=34 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 17: red=0 paint=12086 lod_silhouette_cols=34 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 18: red=0 paint=13341 lod_silhouette_cols=37 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 19: red=0 paint=13118 lod_silhouette_cols=37 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 20: red=0 paint=13159 lod_silhouette_cols=37 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 21: red=0 paint=13400 lod_silhouette_cols=37 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 22: red=0 paint=13254 lod_silhouette_cols=37 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 23: red=0 paint=14093 lod_silhouette_cols=38 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
### stream diag after probes:
[19:53:35 INFO]: [c-LoD] stream: tick 3800 65536/65536 cells, lap 2456, cone 52877 zone 812 dirty 819 (q 11573) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1020998c/1156e, edits +5719p/4214c/881e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 310 forced 202, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 432853p/531b fail 0 ~121 kB/s cache 12638/24576 h420215 m12638 e0
[c-LoD] stream: tick 3900 65536/65536 cells, lap 2556, cone 54336 zone 812 dirty 819 (q 11573) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1049778c/1157e, edits +5720p/4215c/882e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 310 forced 202, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 444405p/543b fail 0 ~60 kB/s cache 12638/24576 h431767 m12638 e0
[19:53:45 INFO]: [c-LoD] stream: tick 3900 65536/65536 cells, lap 2556, cone 54336 zone 812 dirty 819 (q 11573) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1049778c/1157e, edits +5720p/4215c/882e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 310 forced 202, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 444405p/543b fail 0 ~60 kB/s cache 12638/24576 h431767 m12638 e0
- parity debug tour: 13 vantages (session-1 client, DEBUG pack — evidence only, the gate pairs the §7b world session)
- parity debug tour done: 13 shots in parity_dbg/ (the §7b world session feeds the gate)
- joined session 2 (19:56:42)
- leg-2 baseline scene px (x<940): 0 (0 = the relog client paints no world at all)
- leg-2 renderer probe NEAR (20 m) white px: 513 raw / 513 changed-vs-baseline (> 20 = the client renders particles)
- leg-2 renderer probe FAR (80 m, marker zone) white px: 393 raw / 381 changed-vs-baseline (> 20 = no distance cut; ~0 with NEAR alive = particles beyond the client chunk radius do not render)
- leg-2 carrier probe green dust (80 m): changed-green px 0 / total changed 0 vs baseline (> 100 green = a stable green carrier exists; total > 0 with green ~0 = the dust renders but fog washes it — a ruler problem, not a carrier problem)
- LEG-2 SCENE DEAD: the relog client paints ZERO scene pixels (x<940) — pure sky + lens panels; the fresh-session capture chain never commits (renderer-dead class, #227/#228); these probes measure a worldless client, not the transport
- relog probe 0: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 1: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 2: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 3: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 4: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 5: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 6: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 7: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 8: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 9: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 10: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 11: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red >= 60; scene > 0 = the client paints a world at all)
- **FAIL:** leg-2 painted no scene in any probe — the fresh-session capture chain never commits (renderer-dead class of #227/#228); the relog verdict is N/A until the chain arms on a fresh client

### client log forensics (pipeline/particle/atlas — v6.8.2)
```
ALSA lib confmisc.c:1342:(snd_func_refer) error evaluating name
ALSA lib conf.c:5208:(_snd_config_evaluate) function snd_func_refer returned error: No such file or directory
ALSA lib conf.c:5731:(snd_config_expand) Evaluate error: No such file or directory
[19:56:39] [Render thread/ERROR]: Error starting SoundSystem. Turning off sounds & music
java.lang.IllegalStateException: Failed to open OpenAL device
	at net.minecraft.client.sounds.SoundEngine.reload(SoundEngine.java:110)
	at net.minecraft.server.packs.resources.SimplePreparableReloadListener.lambda$reload$1(SimplePreparableReloadListener.java:15)
	at net.minecraft.server.packs.resources.SimpleReloadInstance.lambda$prepareTasks$3(SimpleReloadInstance.java:59)
[19:56:39] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[19:56:39] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[19:56:39] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[19:56:39] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[19:56:39] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[19:56:39] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[19:56:39] [Render thread/INFO]: Created: 2048x2048x4 minecraft:textures/atlas/blocks.png-atlas
[19:56:39] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[19:56:39] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[19:56:39] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[19:56:39] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[19:56:39] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
[19:56:47] [Render thread/INFO]: Reloading ResourceManager: vanilla, vanilla, server/00000000/6b1f2f7e-9c0d-4a4b-8b0a-b10d1b4ab100
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
[19:56:49] [Render thread/ERROR]: Error starting SoundSystem. Turning off sounds & music
java.lang.IllegalStateException: Failed to open OpenAL device
	at net.minecraft.client.sounds.SoundEngine.reload(SoundEngine.java:110)
	at net.minecraft.server.packs.resources.SimplePreparableReloadListener.lambda$reload$1(SimplePreparableReloadListener.java:15)
	at net.minecraft.server.packs.resources.SimpleReloadInstance.lambda$prepareTasks$3(SimpleReloadInstance.java:59)
[19:56:49] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[19:56:49] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[19:56:49] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[19:56:49] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[19:56:49] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[19:56:49] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[19:56:49] [Render thread/INFO]: Created: 2048x2048x4 minecraft:textures/atlas/blocks.png-atlas
[19:56:49] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[19:56:49] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[19:56:49] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[19:56:49] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[19:56:49] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
```

### /clod-cells AFTER PROBES (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 2999 0 0x8400 0x3100
123 122 -940 784 71 0 1 [97, 119, 97] 2999 0 0x8700 0x3100
124 122 -938 784 68 0 1 [97, 119, 97] 2999 0 0x8400 0x3100
125 122 -936 784 62 1 1 [63, 118, 228] 2999 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 2999 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 2999 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 2999 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 2999 0 0x8000 0x5c00
130 122 -926 784 66 0 1 [134, 183, 131] 2999 0 0x8200 0x5c00
131 122 -924 784 75 0 1 [97, 119, 97] 2999 0 0x8b00 0x3100
132 122 -922 784 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 122 -920 784 75 0 1 [97, 119, 97] 18446744073709551615 1 0x8b00 0x3100
134 122 -918 784 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x3100
122 123 -942 786 70 0 1 [97, 119, 97] 3000 0 0x8600 0x3100
123 123 -940 786 65 0 1 [97, 119, 97] 2999 0 0x8100 0x3100
124 123 -938 786 65 0 1 [97, 119, 97] 2999 0 0x8100 0x3100
125 123 -936 786 62 1 1 [63, 118, 228] 2999 0 0x7e00 0xd800
```
- edit channel: 0 linkage errors, 0 failed event dispatches (both must be 0)
- parity world: view-distance=8 (the voxy leg's server value)
- parity world: joined session 3 (20:01:56)
- parity world: scan populated (cells=65536; the march paints from these buffers)
- parity world tour: 13 vantages (production pack, vd 8, shared structures)
- coverage sweeps: 72 eye passes = 36 positions (12 az x r144/200 + 12 off-az x r172, y=160) x 2 looks (outward+inward), settle-by-laps >= 6 s (q stable 3x1s), deadline 12 s/pos, no shots — the v6.26.0 data-coverage lever + v6.26.6 lottery kill; probe rides parity/view_probe.md
- parity drain-settle TIMEOUT: queue 27983 -> 27983 > 11500 after 150s — continuing anyway; the queue did not drain without eye proximity (priority-lane evidence, the 01/05/10 dip attribution reads from the tour deltas)
- pre-tour pass done: 13 vantages x 10s (the tour now reads the second OR-merge lap; the rim/dip transport race reads from the tour deltas)
- parity world tour done: 13 shots, 0 night-guard renames in parity/ (the gate pairs voxy/<name>.png vs parity/<name>.png)
### parity world stream counters (session 3, tail + maxima)
[c-LoD] stream: tick 13100 65536/65536 cells, lap 9569, cone 193478 zone 24928 dirty 13231 (q 27962) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 8936327c/3002e, edits +18132p/13738c/2940e drop 0, spawned 0 / failed 0, cam0 yaw -180 pitch 5, wish 0 forced 932, native 45184c/957ch fail 0 (3w q 0) helper on, trees 957ch/fail 0, wire bundle 1787321p/2103b fail 0 ~0 kB/s cache 23491/24576 h1315787 m471534 e448043
[20:23:34 INFO]: [c-LoD] stream: tick 13100 65536/65536 cells, lap 9569, cone 193478 zone 24928 dirty 13231 (q 27962) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 8936327c/3002e, edits +18132p/13738c/2940e drop 0, spawned 0 / failed 0, cam0 yaw -180 pitch 5, wish 0 forced 932, native 45184c/957ch fail 0 (3w q 0) helper on, trees 957ch/fail 0, wire bundle 1787321p/2103b fail 0 ~0 kB/s cache 23491/24576 h1315787 m471534 e448043
- parity stream max cone: 193478
- parity stream max zone: 24928
- parity stream max wire bundle: bundle
### parity world capture lens (debug chain on the §7b world)
lens_red=0 lens_black=0 lens_water=0 lens_dark=0 lens_zero=0 rot=none scan=0 carriers=0
- parity mesh probe @png_y=2: x306=(151,184,255) x307=(151,184,255) x308=(151,184,255) x309h=(151,184,255) x310c=(151,184,255)
- parity mesh probe @png_y=717: x306=(61,77,67) x307=(62,79,68) x308=(64,80,70) x309h=(55,68,61) x310c=(43,61,51)
- parity mesh engagement: magenta-ish=272 of 921600 sampled px
- mask8: the un-fog engagement masks shot on the PARITY WORLD (vd 8) — parity_dbg8/01m|07m|10m
- mask8 01m: green=4320 red=0 blue=97824 dim=8288 (px, sampled /16)
- mask8 07m: green=11056 red=0 blue=221808 dim=5888 (px, sampled /16)
- mask8 10m: green=4128 red=96 blue=222832 dim=146208 (px, sampled /16)
- mesh probe @png_y=2: x306=(255,255,255) x307=(13,254,0) x308=(0,254,0) x309h=(0,0,0) x310c=(0,0,0)
- mesh probe @png_y=717: x306=(255,255,255) x307=(13,254,0) x308=(0,254,0) x309h=(0,0,0) x310c=(0,0,0)
- mesh engagement: magenta-ish=0 of 921600 sampled px (the vanilla-path paint coverage)
- mask8 fog state @png_y=2: raw (300)=(151,184,255) (301)=(151,184,255)
- mask8 fog state @png_y=717: raw (300)=(0,128,8) (301)=(179,206,255)
- mask8 fog state decoded: envStart=0 envEnd=1024 rdEnd=128 fogColor=(179,206,255)
- buffer dump 10m: marked h=32/2048 c=1/2048; height windows r0<142:30 r1:0 r2>=202:2 maxByte=255
- buffer dump 10m: RANK-2-WINDOW HEIGHTS (the fake-wall class): (+108,-30)h=187, (+106,-12)h=191
