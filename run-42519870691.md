### c-LoD visual run (-930,796, marker=red_concrete, vantage -1005,150 yaw 270)
- GL:     Vendor: Mesa (0xffffffff);    Device: llvmpipe (LLVM 20.1.2, 256 bits) (0xffffffff);OpenGL renderer string: llvmpipe (LLVM 20.1.2, 256 bits);OpenGL core profile version string: 4.5 (Core Profile) Mesa 25.2.8-0ubuntu0.24.04.4;
- boot + gates OK (20:25:02)
- sky-grid armed
- joined session 1 (20:25:23)
- stream live: cells=65536
- terrain range: 2 terraced mountains at (CX-4,CZ-20)/(CX-2,CZ+16), tops 185/190, grass terraces + stone caps, 66..82 m from the eye (inside the 83.5 m band, chunk edge 64 m)
- scan complete=1 (cells=cells=65536/65536)
- paint settled=1 (wire 54300p)
- baseline try 1: drift=0 px
- lod_silhouette_cols=0 (LOD paint ON SCREEN at 75 m if > 20 — the march-drawn marker column; vanilla cannot render there, chunk edge 64 m; #255 re-measures 49, the ZONE_FAR=48 regression #256 and the control read 0)
- 0 terrain-green px in the sky region (LOD MOUNTAINS on screen if > 5000; the range paints ColLow-green on low terraces via the fallback palette; flat-world baselines and the pack-free control read ~0)
- burst_close white px: 534 (renderer alive if > 50)
- burst_far white px: 0 (renderer alive in the marker zone if > 20)
- cut walk 40 m: 64 white px (> 20 = rendered)
- cut walk 60 m: 0 white px (> 20 = rendered)
- cut walk 100 m: 0 white px (> 20 = rendered)
- measured far-render cut: 40 m (G4 flap instrument; the decode proof needs >= 83 m from the y=151.6 eye)
- fill issued: 5x111x5 red_concrete at -930,796 (zone forceloaded ±48)

### /clod-cells AFTER FILL (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 1104 0 0x8400 0x3100
123 122 -940 784 71 0 1 [97, 119, 97] 1099 0 0x8700 0x3100
124 122 -938 784 68 0 1 [97, 119, 97] 1099 0 0x8400 0x3100
125 122 -936 784 62 1 1 [63, 118, 228] 1099 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 1098 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 1098 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 1097 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 1097 0 0x8000 0x5c00
130 122 -926 784 66 0 1 [134, 183, 131] 1096 0 0x8200 0x5c00
131 122 -924 784 75 0 1 [97, 119, 97] 1096 0 0x8b00 0x3100
132 122 -922 784 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 122 -920 784 75 0 1 [97, 119, 97] 18446744073709551615 1 0x8b00 0x3100
134 122 -918 784 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x3100
122 123 -942 786 70 0 1 [97, 119, 97] 1104 0 0x8600 0x3100
123 123 -940 786 65 0 1 [97, 119, 97] 1099 0 0x8100 0x3100
124 123 -938 786 65 0 1 [97, 119, 97] 1099 0 0x8100 0x3100
125 123 -936 786 62 1 1 [63, 118, 228] 1099 0 0x7e00 0xd800
```
### stream diag after fill:
[c-LoD] stream: tick 1800 65536/65536 cells, lap 600, cone 5362 zone 646 dirty 759 (q 10673) ewin 214 rbeat 0p/1 rearm 0c/0t rescan 390706c/2854e, edits +5266p/3969c/564e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 351 forced 161, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 54660p/142b fail 0 ~0 kB/s cache 11447/24576 h43213 m11447 e0
[20:28:11 INFO]: [c-LoD] stream: tick 1800 65536/65536 cells, lap 600, cone 5362 zone 646 dirty 759 (q 10673) ewin 214 rbeat 0p/1 rearm 0c/0t rescan 390706c/2854e, edits +5266p/3969c/564e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 351 forced 161, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 54660p/142b fail 0 ~0 kB/s cache 11447/24576 h43213 m11447 e0
- probe 0: red=0 paint=0 lod_silhouette_cols=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 1: red=0 paint=0 lod_silhouette_cols=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 2: red=0 paint=0 lod_silhouette_cols=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 3: red=0 paint=411 lod_silhouette_cols=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=198114 lens_zero=0 rot=decoded cam_x_b=? cam_z_b=? az_b=? el_b=? strobe_hits=22 mask_hits=22 (need red>=60 paint>=300 lod>20)
- probe 4: red=0 paint=384 lod_silhouette_cols=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=6 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 5: red=0 paint=192 lod_silhouette_cols=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=2 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 6: red=0 paint=402 lod_silhouette_cols=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=198114 lens_zero=0 rot=decoded cam_x_b=? cam_z_b=? az_b=? el_b=? strobe_hits=22 mask_hits=22 (need red>=60 paint>=300 lod>20)
- probe 7: red=0 paint=0 lod_silhouette_cols=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 8: red=0 paint=0 lod_silhouette_cols=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 9: red=0 paint=0 lod_silhouette_cols=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 10: red=0 paint=0 lod_silhouette_cols=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 11: red=0 paint=0 lod_silhouette_cols=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 12: red=0 paint=0 lod_silhouette_cols=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 13: red=0 paint=401 lod_silhouette_cols=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=198114 lens_zero=0 rot=decoded cam_x_b=? cam_z_b=? az_b=? el_b=? strobe_hits=22 mask_hits=22 (need red>=60 paint>=300 lod>20)
- probe 14: red=0 paint=384 lod_silhouette_cols=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=11 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 15: red=0 paint=0 lod_silhouette_cols=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 16: red=0 paint=0 lod_silhouette_cols=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 17: red=0 paint=0 lod_silhouette_cols=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 18: red=0 paint=0 lod_silhouette_cols=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 19: red=0 paint=0 lod_silhouette_cols=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 20: red=0 paint=384 lod_silhouette_cols=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=16 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 21: red=0 paint=384 lod_silhouette_cols=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=4 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 22: red=0 paint=0 lod_silhouette_cols=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 23: red=0 paint=399 lod_silhouette_cols=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=198114 lens_zero=0 rot=decoded cam_x_b=? cam_z_b=? az_b=? el_b=? strobe_hits=22 mask_hits=22 (need red>=60 paint>=300 lod>20)
### stream diag after probes:
[20:31:12 INFO]: [c-LoD] stream: tick 3600 65536/65536 cells, lap 1163, cone 18712 zone 1286 dirty 786 (q 10714) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 972193c/2927e, edits +6990p/5204c/962e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 351 forced 161, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 166317p/265b fail 0 ~56 kB/s cache 11512/24576 h154805 m11512 e0
[c-LoD] stream: tick 3700 65536/65536 cells, lap 1263, cone 21382 zone 1286 dirty 786 (q 10714) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1004623c/2927e, edits +6990p/5204c/962e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 351 forced 161, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 187583p/287b fail 0 ~112 kB/s cache 11512/24576 h176071 m11512 e0
[20:31:22 INFO]: [c-LoD] stream: tick 3700 65536/65536 cells, lap 1263, cone 21382 zone 1286 dirty 786 (q 10714) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1004623c/2927e, edits +6990p/5204c/962e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 351 forced 161, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 187583p/287b fail 0 ~112 kB/s cache 11512/24576 h176071 m11512 e0
- parity debug tour: 12 vantages (session-1 client, DEBUG pack — evidence only, the gate pairs the §7b world session)
- parity debug tour done: 12 shots in parity_dbg/ (the §7b world session feeds the gate)
- joined session 2 (20:34:06)
- leg-2 baseline scene px (x<940): 0 (0 = the relog client paints no world at all)
- leg-2 renderer probe NEAR (20 m) white px: 586 raw / 586 changed-vs-baseline (> 20 = the client renders particles)
- leg-2 renderer probe FAR (80 m, marker zone) white px: 0 raw / 0 changed-vs-baseline (> 20 = no distance cut; ~0 with NEAR alive = particles beyond the client chunk radius do not render)
- leg-2 carrier probe green dust (80 m): changed-green px 0 / total changed 4 vs baseline (> 100 green = a stable green carrier exists; total > 0 with green ~0 = the dust renders but fog washes it — a ruler problem, not a carrier problem)
- LEG-2 SCENE DEAD: the relog client paints ZERO scene pixels (x<940) — pure sky + lens panels; the fresh-session capture chain never commits (renderer-dead class, #227/#228); these probes measure a worldless client, not the transport
- relog probe 0: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=2 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 1: red=0 scene=42 lens_red=0 lens_black=0 lens_water=0 lens_dark=198114 lens_zero=0 rot=decoded cam_x_b=? cam_z_b=? az_b=? el_b=? strobe_hits=22 mask_hits=22 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 2: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=6 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 3: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=4 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 4: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 5: red=0 scene=10 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=? cam_z_b=? az_b=? el_b=? strobe_hits=22 mask_hits=22 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 6: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=6 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 7: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 8: red=0 scene=26 lens_red=0 lens_black=0 lens_water=0 lens_dark=198114 lens_zero=0 rot=decoded cam_x_b=? cam_z_b=? az_b=? el_b=? strobe_hits=22 mask_hits=22 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 9: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=13 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 10: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 11: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red >= 60; scene > 0 = the client paints a world at all)
- **FAIL:** marker lost after relog — stream state is not persistent/deterministic

### client log forensics (pipeline/particle/atlas — v6.8.2)
```
ALSA lib confmisc.c:1342:(snd_func_refer) error evaluating name
ALSA lib conf.c:5208:(_snd_config_evaluate) function snd_func_refer returned error: No such file or directory
ALSA lib conf.c:5731:(snd_config_expand) Evaluate error: No such file or directory
[20:34:03] [Render thread/ERROR]: Error starting SoundSystem. Turning off sounds & music
java.lang.IllegalStateException: Failed to open OpenAL device
	at net.minecraft.client.sounds.SoundEngine.reload(SoundEngine.java:110)
	at net.minecraft.server.packs.resources.SimplePreparableReloadListener.lambda$reload$1(SimplePreparableReloadListener.java:15)
	at net.minecraft.server.packs.resources.SimpleReloadInstance.lambda$prepareTasks$3(SimpleReloadInstance.java:59)
[20:34:03] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[20:34:03] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[20:34:03] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[20:34:03] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[20:34:03] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[20:34:03] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[20:34:03] [Render thread/INFO]: Created: 2048x2048x4 minecraft:textures/atlas/blocks.png-atlas
[20:34:03] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[20:34:03] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[20:34:03] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[20:34:03] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[20:34:03] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
[20:34:10] [Render thread/INFO]: Reloading ResourceManager: vanilla, vanilla, server/00000000/6b1f2f7e-9c0d-4a4b-8b0a-b10d1b4ab100
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
[20:34:12] [Render thread/ERROR]: Error starting SoundSystem. Turning off sounds & music
java.lang.IllegalStateException: Failed to open OpenAL device
	at net.minecraft.client.sounds.SoundEngine.reload(SoundEngine.java:110)
	at net.minecraft.server.packs.resources.SimplePreparableReloadListener.lambda$reload$1(SimplePreparableReloadListener.java:15)
	at net.minecraft.server.packs.resources.SimpleReloadInstance.lambda$prepareTasks$3(SimpleReloadInstance.java:59)
[20:34:12] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[20:34:12] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[20:34:12] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[20:34:12] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[20:34:12] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[20:34:12] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[20:34:12] [Render thread/INFO]: Created: 2048x2048x4 minecraft:textures/atlas/blocks.png-atlas
[20:34:12] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[20:34:12] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[20:34:12] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[20:34:12] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[20:34:12] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
```

### /clod-cells AFTER PROBES (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 44 0 0x8400 0x3100
123 122 -940 784 71 0 1 [97, 119, 97] 44 0 0x8700 0x3100
124 122 -938 784 68 0 1 [97, 119, 97] 43 0 0x8400 0x3100
125 122 -936 784 62 1 1 [63, 118, 228] 43 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 42 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 42 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 41 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 41 0 0x8000 0x5c00
130 122 -926 784 66 0 1 [134, 183, 131] 40 0 0x8200 0x5c00
131 122 -924 784 75 0 1 [97, 119, 97] 39 0 0x8b00 0x3100
132 122 -922 784 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 122 -920 784 75 0 1 [97, 119, 97] 18446744073709551615 1 0x8b00 0x3100
134 122 -918 784 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x3100
122 123 -942 786 70 0 1 [97, 119, 97] 45 0 0x8600 0x3100
123 123 -940 786 65 0 1 [97, 119, 97] 44 0 0x8100 0x3100
124 123 -938 786 65 0 1 [97, 119, 97] 44 0 0x8100 0x3100
125 123 -936 786 62 1 1 [63, 118, 228] 43 0 0x7e00 0xd800
```
- edit channel: 0 linkage errors, 0 failed event dispatches (both must be 0)
- parity world: view-distance=8 (the voxy leg's server value)
- parity world: joined session 3 (20:39:09)
- parity world: scan populated (cells=65536; the march paints from these buffers)
- parity world tour: 12 vantages (production pack, vd 8, shared structures)
- parity world tour done: 12 shots in parity/ (the gate pairs voxy/<name>.png vs parity/<name>.png)
### parity world stream counters (session 3, tail + maxima)
[20:43:16 INFO]: [c-LoD] stream: tick 2600 65536/65536 cells, lap 2333, cone 62435 zone 4984 dirty 190 (q 20902) ewin 0 rbeat 0p/1 rearm 0c/0t rescan 1717683c/1239e, edits +81p/79c/12e drop 0, spawned 0 / failed 0, cam0 yaw 90 pitch 89, wish 416 forced 522, native 43136c/955ch fail 0 (3w q 0) helper on, trees 955ch/fail 0, wire bundle 526462p/598b fail 0 ~55 kB/s cache 22743/24576 h411559 m114903 e92160
[c-LoD] [20:43:26 INFO]: [c-LoD] stream: tick 2700 65536/65536 cells, lap 2433, cone 63508 zone 4984 dirty 190 (q 20902) ewin 0 rbeat 0p/1 rearm 0c/0t rescan 1797123c/1239e, edits +82p/80c/12e drop 0, spawned 0 / failed 0, cam0 yaw -180 pitch 5, wish 416 forced 522, native 43136c/955ch fail 0 (3w q 0) helper on, trees 955ch/fail 0, wire bundle 526462p/598b fail 0 ~0 kB/s cache 22743/24576 h411559 m114903 e92160
- parity stream max cone: 63508
- parity stream max zone: 4984
- parity stream max wire bundle: bundle
### parity world capture lens (debug chain on the §7b world)
lens_red=0 lens_black=0 lens_water=0 lens_dark=0 lens_zero=0 rot=none scan=0 carriers=0
- parity mesh probe @png_y=2: x306=(151,184,255) x307=(151,184,255) x308=(151,184,255) x309h=(151,184,255) x310c=(151,184,255)
- parity mesh probe @png_y=717: x306=(44,62,51) x307=(46,65,53) x308=(83,109,89) x309h=(83,109,89) x310c=(48,156,192)
- parity mesh engagement: magenta-ish=1904 of 921600 sampled px
- mask8: the un-fog engagement masks shot on the PARITY WORLD (vd 8) — parity_dbg8/01m|07m|10m
- mask8 01m: green=3200 red=3296 blue=112448 dim=258736 (px, sampled /16)
- mask8 07m: green=11920 red=0 blue=221360 dim=5824 (px, sampled /16)
- mask8 10m: green=9616 red=2976 blue=208816 dim=136176 (px, sampled /16)
- mesh probe @png_y=2: x306=(255,255,255) x307=(13,254,0) x308=(0,254,0) x309h=(195,0,0) x310c=(0,0,0)
- mesh probe @png_y=717: x306=(255,255,255) x307=(13,254,0) x308=(0,254,0) x309h=(195,0,0) x310c=(0,0,0)
- mesh engagement: magenta-ish=23664 of 921600 sampled px (the vanilla-path paint coverage)
- mask8 fog state @png_y=2: raw (300)=(151,184,255) (301)=(151,184,255)
- mask8 fog state @png_y=717: raw (300)=(100,13,105) (301)=(187,30,212)
- mask8 fog state decoded: envStart=800 envEnd=104 rdEnd=1680 fogColor=(187,30,212)
