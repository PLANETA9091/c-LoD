### c-LoD visual run (-930,796, marker=red_concrete, vantage -1005,150 yaw 270)
- GL:     Vendor: Mesa (0xffffffff);    Device: llvmpipe (LLVM 20.1.2, 256 bits) (0xffffffff);OpenGL renderer string: llvmpipe (LLVM 20.1.2, 256 bits);OpenGL core profile version string: 4.5 (Core Profile) Mesa 25.2.8-0ubuntu0.24.04.4;
- boot + gates OK (10:59:21)
- sky-grid armed
- joined session 1 (10:59:48)
- stream live: cells=65536
- terrain range: 2 terraced mountains at (CX-4,CZ-20)/(CX-2,CZ+16), tops 185/190, grass terraces + stone caps, 66..82 m from the eye (inside the 83.5 m band, chunk edge 64 m)
- scan complete=1 (cells=cells=65536/65536)
- paint settled=1 (wire 60504p)
- baseline try 1: drift=0 px
- lod_silhouette_cols=13 (LOD paint ON SCREEN at 75 m if > 20 — the march-drawn marker column; vanilla cannot render there, chunk edge 64 m; #255 re-measures 49, the ZONE_FAR=48 regression #256 and the control read 0)
- 11 terrain-green px in the sky region (LOD MOUNTAINS on screen if > 5000; the range paints ColLow-green on low terraces via the fallback palette; flat-world baselines and the pack-free control read ~0)
- burst_close white px: 0 (renderer alive if > 50)
- burst_far white px: 462 (renderer alive in the marker zone if > 20)
- cut walk 40 m: 68 white px (> 20 = rendered)
- cut walk 60 m: 2 white px (> 20 = rendered)
- cut walk 100 m: 0 white px (> 20 = rendered)
- measured far-render cut: 40 m (G4 flap instrument; the decode proof needs >= 83 m from the y=151.6 eye)
- burst_close retry white px: 530 (preflight passes if > 50)
- fill issued: 5x111x5 red_concrete at -930,796 (zone forceloaded ±48)

### /clod-cells AFTER FILL (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 48 0 0x8400 0x3100
123 122 -940 784 71 0 1 [97, 119, 97] 48 0 0x8700 0x3100
124 122 -938 784 68 0 1 [97, 119, 97] 47 0 0x8400 0x3100
125 122 -936 784 62 1 1 [63, 118, 228] 47 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 46 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 45 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 45 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 44 0 0x8000 0x5c00
130 122 -926 784 66 0 1 [134, 183, 131] 44 0 0x8200 0x5c00
131 122 -924 784 75 0 1 [97, 119, 97] 43 0 0x8b00 0x3100
132 122 -922 784 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 122 -920 784 75 0 1 [97, 119, 97] 18446744073709551615 1 0x8b00 0x3100
134 122 -918 784 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x3100
122 123 -942 786 70 0 1 [97, 119, 97] 48 0 0x8600 0x3100
123 123 -940 786 65 0 1 [97, 119, 97] 48 0 0x8100 0x3100
124 123 -938 786 65 0 1 [97, 119, 97] 47 0 0x8100 0x3100
125 123 -936 786 62 1 1 [63, 118, 228] 47 0 0x7e00 0xd800
```
### stream diag after fill:
[c-LoD] stream: tick 2100 65536/65536 cells, lap 981, cone 15482 zone 1517 dirty 898 (q 12363) ewin 233 rbeat 0p/0 rearm 0c/0t rescan 459943c/1549e, edits +4667p/3524c/550e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 307 forced 205, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 142944p/245b fail 0 ~61 kB/s cache 13393/24576 h129551 m13393 e0, srbeat 0s/0q drop 0stale/0undec
[11:03:03 INFO]: [c-LoD] stream: tick 2100 65536/65536 cells, lap 981, cone 15482 zone 1517 dirty 898 (q 12363) ewin 233 rbeat 0p/0 rearm 0c/0t rescan 459943c/1549e, edits +4667p/3524c/550e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 307 forced 205, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 142944p/245b fail 0 ~61 kB/s cache 13393/24576 h129551 m13393 e0, srbeat 0s/0q drop 0stale/0undec
- probe 0: red=0 paint=8438 lod_silhouette_cols=16 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 1: red=0 paint=8438 lod_silhouette_cols=16 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 2: red=0 paint=8438 lod_silhouette_cols=16 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 3: red=0 paint=8438 lod_silhouette_cols=16 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 4: red=0 paint=8438 lod_silhouette_cols=16 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 5: red=0 paint=8438 lod_silhouette_cols=16 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 6: red=0 paint=8438 lod_silhouette_cols=16 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 7: red=0 paint=8438 lod_silhouette_cols=16 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 8: red=0 paint=8438 lod_silhouette_cols=16 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 9: red=0 paint=8438 lod_silhouette_cols=16 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 10: red=0 paint=8438 lod_silhouette_cols=16 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 11: red=0 paint=8774 lod_silhouette_cols=16 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 12: red=0 paint=8850 lod_silhouette_cols=16 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 13: red=0 paint=8541 lod_silhouette_cols=16 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 14: red=0 paint=8541 lod_silhouette_cols=16 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 15: red=0 paint=8541 lod_silhouette_cols=16 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 16: red=0 paint=8541 lod_silhouette_cols=16 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 17: red=0 paint=8541 lod_silhouette_cols=16 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 18: red=0 paint=9574 lod_silhouette_cols=16 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 19: red=0 paint=9852 lod_silhouette_cols=16 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 20: red=0 paint=10315 lod_silhouette_cols=16 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 21: red=0 paint=10233 lod_silhouette_cols=16 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=1 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 22: red=0 paint=10290 lod_silhouette_cols=16 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=1 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 23: red=0 paint=10288 lod_silhouette_cols=16 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=1 mask_hits=30 (need red>=60 paint>=300 lod>20)
- **FAIL:** leg-1 never saw the marker in 24 probes (red < 60 everywhere) — the edit never reached the LOD paint; this failure was SILENT before v6.31.1
### stream diag after probes:
[11:06:13 INFO]: [c-LoD] stream: tick 4000 65536/65536 cells, lap 1525, cone 29501 zone 2322 dirty 952 (q 12392) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1075307c/1614e, edits +6241p/4643c/903e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 307 forced 205, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 261050p/377b fail 0 ~96 kB/s cache 13490/24576 h247560 m13490 e0, srbeat 0s/0q drop 0stale/0undec
[c-LoD] stream: tick 4100 65536/65536 cells, lap 1625, cone 32443 zone 2322 dirty 952 (q 12392) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1106076c/1614e, edits +6241p/4643c/903e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 307 forced 205, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 284392p/401b fail 0 ~123 kB/s cache 13490/24576 h270902 m13490 e0, srbeat 0s/0q drop 0stale/0undec
[11:06:23 INFO]: [c-LoD] stream: tick 4100 65536/65536 cells, lap 1625, cone 32443 zone 2322 dirty 952 (q 12392) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1106076c/1614e, edits +6241p/4643c/903e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 307 forced 205, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 284392p/401b fail 0 ~123 kB/s cache 13490/24576 h270902 m13490 e0, srbeat 0s/0q drop 0stale/0undec
- parity debug tour: 13 vantages (session-1 client, DEBUG pack — evidence only, the gate pairs the §7b world session)
- parity debug tour done: 13 shots in parity_dbg/ (the §7b world session feeds the gate)
- joined session 2 (11:09:22)
- leg-2 baseline scene px (x<940): 0 (0 = the relog client paints no world at all)
- leg-2 renderer probe NEAR (20 m) white px: 0 raw / 0 changed-vs-baseline (> 20 = the client renders particles)
- leg-2 renderer probe FAR (80 m, marker zone) white px: 461 raw / 461 changed-vs-baseline (> 20 = no distance cut; ~0 with NEAR alive = particles beyond the client chunk radius do not render)
- leg-2 carrier probe green dust (80 m): changed-green px 0 / total changed 117 vs baseline (> 100 green = a stable green carrier exists; total > 0 with green ~0 = the dust renders but fog washes it — a ruler problem, not a carrier problem)
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
[11:09:18] [Render thread/ERROR]: Error starting SoundSystem. Turning off sounds & music
java.lang.IllegalStateException: Failed to open OpenAL device
	at net.minecraft.client.sounds.SoundEngine.reload(SoundEngine.java:110)
	at net.minecraft.server.packs.resources.SimplePreparableReloadListener.lambda$reload$1(SimplePreparableReloadListener.java:15)
	at net.minecraft.server.packs.resources.SimpleReloadInstance.lambda$prepareTasks$3(SimpleReloadInstance.java:59)
[11:09:18] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[11:09:18] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[11:09:18] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[11:09:18] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[11:09:18] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[11:09:18] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[11:09:18] [Render thread/INFO]: Created: 2048x2048x4 minecraft:textures/atlas/blocks.png-atlas
[11:09:18] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[11:09:18] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[11:09:18] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[11:09:18] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[11:09:18] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
[11:09:26] [Render thread/INFO]: Reloading ResourceManager: vanilla, vanilla, server/00000000/6b1f2f7e-9c0d-4a4b-8b0a-b10d1b4ab100
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
[11:09:28] [Render thread/ERROR]: Error starting SoundSystem. Turning off sounds & music
java.lang.IllegalStateException: Failed to open OpenAL device
	at net.minecraft.client.sounds.SoundEngine.reload(SoundEngine.java:110)
	at net.minecraft.server.packs.resources.SimplePreparableReloadListener.lambda$reload$1(SimplePreparableReloadListener.java:15)
	at net.minecraft.server.packs.resources.SimpleReloadInstance.lambda$prepareTasks$3(SimpleReloadInstance.java:59)
[11:09:28] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[11:09:28] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[11:09:28] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[11:09:28] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[11:09:28] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[11:09:28] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[11:09:28] [Render thread/INFO]: Created: 2048x2048x4 minecraft:textures/atlas/blocks.png-atlas
[11:09:28] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[11:09:28] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[11:09:28] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[11:09:28] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[11:09:28] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
```

### /clod-cells AFTER PROBES (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 4220 0 0x8400 0x3100
123 122 -940 784 71 0 1 [97, 119, 97] 4220 0 0x8700 0x3100
124 122 -938 784 68 0 1 [97, 119, 97] 4219 0 0x8400 0x3100
125 122 -936 784 62 1 1 [63, 118, 228] 4219 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 4218 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 4217 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 4217 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 4216 0 0x8000 0x5c00
130 122 -926 784 66 0 1 [134, 183, 131] 4216 0 0x8200 0x5c00
131 122 -924 784 75 0 1 [97, 119, 97] 4215 0 0x8b00 0x3100
132 122 -922 784 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 122 -920 784 75 0 1 [97, 119, 97] 18446744073709551615 1 0x8b00 0x3100
134 122 -918 784 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x3100
122 123 -942 786 70 0 1 [97, 119, 97] 4220 0 0x8600 0x3100
123 123 -940 786 65 0 1 [97, 119, 97] 4220 0 0x8100 0x3100
124 123 -938 786 65 0 1 [97, 119, 97] 4219 0 0x8100 0x3100
125 123 -936 786 62 1 1 [63, 118, 228] 4219 0 0x7e00 0xd800
```
- edit channel: 0 linkage errors, 0 failed event dispatches (both must be 0)
- parity world: view-distance=8 (the voxy leg's server value)
- parity world: joined session 3 (11:14:38)
- parity world: scan populated (cells=65536; the march paints from these buffers)
- parity world tour: 13 vantages (production pack, vd 8, shared structures)
- coverage sweeps: 72 eye passes = 36 positions (12 az x r144/200 + 12 off-az x r172, y=160) x 2 looks (outward+inward), settle-by-laps >= 6 s (q stable 3x1s), deadline 12 s/pos, no shots — the v6.26.0 data-coverage lever + v6.26.6 lottery kill; probe rides parity/view_probe.md
- parity drain-settle TIMEOUT: queue 20632 -> 20626 > 11500 after 150s — continuing anyway; the queue did not drain without eye proximity (priority-lane evidence, the 01/05/10 dip attribution reads from the tour deltas)
- pre-tour pass done: 13 vantages x 10s (the tour now reads the second OR-merge lap; the rim/dip transport race reads from the tour deltas)
- **DAY GUARD:** 00_summit_rise still night after re-pin+reshoot (zenith < 130) — renamed night_00_summit_rise.png; the row pairs as missing, NOT as a renderer diff
- **DAY GUARD:** 01_aerial_west still night after re-pin+reshoot (zenith < 130) — renamed night_01_aerial_west.png; the row pairs as missing, NOT as a renderer diff
- **DAY GUARD:** 02_aerial_south still night after re-pin+reshoot (zenith < 130) — renamed night_02_aerial_south.png; the row pairs as missing, NOT as a renderer diff
- **DAY GUARD:** 03_aerial_north still night after re-pin+reshoot (zenith < 130) — renamed night_03_aerial_north.png; the row pairs as missing, NOT as a renderer diff
- **DAY GUARD:** 04_aerial_east still night after re-pin+reshoot (zenith < 130) — renamed night_04_aerial_east.png; the row pairs as missing, NOT as a renderer diff
- **DAY GUARD:** 05_oblique still night after re-pin+reshoot (zenith < 130) — renamed night_05_oblique.png; the row pairs as missing, NOT as a renderer diff
- **DAY GUARD:** 06_ground_west still night after re-pin+reshoot (zenith < 130) — renamed night_06_ground_west.png; the row pairs as missing, NOT as a renderer diff
- parity world tour done: 13 shots, 7 night-guard renames in parity/ (the gate pairs voxy/<name>.png vs parity/<name>.png)
- sparse re-beat channel at tour end: streamed=56768 q=0 dropped_stale=26 dropped_undecodable=12860 (frac=8 boot; undecodable-dominant = the rim words die in their decodability window, stale-dominant = edit churn owns the cells)
- **FAIL:** parity world time-of-day uncontrollable: 7 vantage(s) night after re-pin+reshoot — the §7b noon-pin class is broken on this runner; the matrix is infrastructure-blocked, not a renderer verdict (DAY GUARD lines + the feed's time-audit.md carry the evidence)
### parity world stream counters (session 3, tail + maxima)
[c-LoD] stream: tick 14100 65536/65536 cells, lap 8822, cone 169809 zone 21528 dirty 8923 (q 20578) ewin 0 rbeat 0p/1 rearm 0c/0t rescan 9616502c/1609e, edits +18134p/13747c/2885e drop 0, spawned 0 / failed 0, cam0 yaw -180 pitch 5, wish 0 forced 935, native 31552c/955ch fail 0 (3w q 0) helper on, trees 955ch/fail 0, wire bundle 2000643p/4307b fail 0 ~0 kB/s cache 24416/24576 h1616530 m384113 e359697, srbeat 56768s/0q drop 26stale/12860undec
[11:37:55 INFO]: [c-LoD] stream: tick 14100 65536/65536 cells, lap 8822, cone 169809 zone 21528 dirty 8923 (q 20578) ewin 0 rbeat 0p/1 rearm 0c/0t rescan 9616502c/1609e, edits +18134p/13747c/2885e drop 0, spawned 0 / failed 0, cam0 yaw -180 pitch 5, wish 0 forced 935, native 31552c/955ch fail 0 (3w q 0) helper on, trees 955ch/fail 0, wire bundle 2000643p/4307b fail 0 ~0 kB/s cache 24416/24576 h1616530 m384113 e359697, srbeat 56768s/0q drop 26stale/12860undec
- parity stream max cone: 169809
- parity stream max zone: 21528
- parity stream max wire bundle: bundle
### parity world capture lens (debug chain on the §7b world)
lens_red=? lens_black=? lens_water=? lens_dark=? lens_zero=? rot=? scan=? carriers=?
- mask8: the un-fog engagement masks shot on the PARITY WORLD (vd 8) — parity_dbg8/01m|07m|10m
- mask8 01m: green=688 red=0 blue=101520 dim=16336 (px, sampled /16)
- mask8 07m: green=11776 red=0 blue=221392 dim=5808 (px, sampled /16)
- mask8 10m: green=6208 red=176 blue=216064 dim=196464 (px, sampled /16)
- mesh probe @png_y=2: x306=(255,255,255) x307=(13,254,0) x308=(0,254,0) x309h=(64,0,0) x310c=(0,0,0)
- mesh probe @png_y=717: x306=(255,255,255) x307=(13,254,0) x308=(0,254,0) x309h=(64,0,0) x310c=(0,0,0)
- mesh engagement: magenta-ish=0 of 921600 sampled px (the vanilla-path paint coverage)
- mask8 fog state @png_y=2: raw (300)=(151,184,255) (301)=(151,184,255)
- mask8 fog state @png_y=717: raw (300)=(0,128,8) (301)=(179,206,255)
- mask8 fog state decoded: envStart=0 envEnd=1024 rdEnd=128 fogColor=(179,206,255)
- buffer dump 10m: marked h=19/2048 c=1/2048; height windows r0<142:15 r1:2 r2>=202:2 maxByte=225
- buffer dump 10m: RANK-2-WINDOW HEIGHTS (the fake-wall class): (+80,-30)h=159, (+50,-8)h=161
- rim mask: scan populated (cells=65536; the state paint reads the same buffers the march reads)
- rim mask: decode-state frames shot (rim_mask/07|01|12 — GREEN full decode / RED colorless / BLUE heightless)
- rim mask 07: green=1200 red=0 blue=400 (px, sampled /16) -> DOMINANT GREEN: rim fully decoded — the fallback is the PAINT path (shade/fog/composite), not data
- rim mask 01: green=0 red=0 blue=224 (px, sampled /16) -> no state paint (the march owns too few pixels — check the vantage/scan)
- rim mask 12: green=1200 red=0 blue=400 (px, sampled /16) -> DOMINANT GREEN: rim fully decoded — the fallback is the PAINT path (shade/fog/composite), not data

### boot / stream
```
[c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 124538 bytes, sha1 fc4488f48f619479794bcfab610955e8245b980c — zero textures, zero world data, one push per join
[c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[11:41:46 INFO]: [c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 124538 bytes, sha1 fc4488f48f619479794bcfab610955e8245b980c — zero textures, zero world data, one push per join
[11:41:46 INFO]: [c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[c-LoD] stream: tick 1800 65536/65536 cells, lap 1365, cone 24440 zone 727 dirty 23 (q 4509) ewin 101 rbeat 0p/0 rearm 0c/0t rescan 1097481c/796e, edits +22p/22c/10e drop 0, spawned 0 / failed 0, cam0 yaw -180 pitch 5, wish 14 forced 935, native 4864c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 267330p/623b fail 0 ~0 kB/s cache 11900/24576 h255430 m11900 e0, srbeat 8398s/0q drop 0stale/696undec
[11:44:54 INFO]: [c-LoD] stream: tick 1800 65536/65536 cells, lap 1365, cone 24440 zone 727 dirty 23 (q 4509) ewin 101 rbeat 0p/0 rearm 0c/0t rescan 1097481c/796e, edits +22p/22c/10e drop 0, spawned 0 / failed 0, cam0 yaw -180 pitch 5, wish 14 forced 935, native 4864c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 267330p/623b fail 0 ~0 kB/s cache 11900/24576 h255430 m11900 e0, srbeat 8398s/0q drop 0stale/696undec
```
### sessions
```
[11:42:10 INFO]: System chat: Stepan joined the game
[11:44:55 INFO]: System chat: Stepan left the game
```
### client log tail
```
[11:44:54] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[11:44:55] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:set_time] net.minecraft.network.protocol.game.ClientboundSetTimePacket -> 10 bytes
[11:44:55] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:move_player_pos] net.minecraft.network.protocol.game.ServerboundMovePlayerPacket$Pos -> 26 bytes
[11:44:55] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[11:44:55] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:entity_position_sync] net.minecraft.network.protocol.game.ClientboundEntityPositionSyncPacket -> 37 bytes
[11:44:55] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:entity_position_sync] net.minecraft.network.protocol.game.ClientboundEntityPositionSyncPacket -> 37 bytes
[11:44:55] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:keep_alive] net.minecraft.network.protocol.common.ClientboundKeepAlivePacket -> 9 bytes
[11:44:55] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:keep_alive] net.minecraft.network.protocol.common.ServerboundKeepAlivePacket -> 9 bytes
[11:44:55] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[11:44:55] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:entity_position_sync] net.minecraft.network.protocol.game.ClientboundEntityPositionSyncPacket -> 37 bytes
[11:44:55] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[11:44:55] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[11:44:55] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:entity_position_sync] net.minecraft.network.protocol.game.ClientboundEntityPositionSyncPacket -> 37 bytes
[11:44:55] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:entity_position_sync] net.minecraft.network.protocol.game.ClientboundEntityPositionSyncPacket -> 37 bytes
[11:44:55] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[11:44:55] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:entity_position_sync] net.minecraft.network.protocol.game.ClientboundEntityPositionSyncPacket -> 37 bytes
[11:44:55] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[11:44:55] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[11:44:55] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:entity_position_sync] net.minecraft.network.protocol.game.ClientboundEntityPositionSyncPacket -> 37 bytes
[11:44:55] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:entity_position_sync] net.minecraft.network.protocol.game.ClientboundEntityPositionSyncPacket -> 37 bytes
[11:44:55] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[11:44:55] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[11:44:55] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:entity_position_sync] net.minecraft.network.protocol.game.ClientboundEntityPositionSyncPacket -> 37 bytes
[11:44:55] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[11:44:55] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:entity_position_sync] net.minecraft.network.protocol.game.ClientboundEntityPositionSyncPacket -> 37 bytes
[11:44:55] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:entity_position_sync] net.minecraft.network.protocol.game.ClientboundEntityPositionSyncPacket -> 37 bytes
[11:44:55] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[11:44:55] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[11:44:55] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:entity_position_sync] net.minecraft.network.protocol.game.ClientboundEntityPositionSyncPacket -> 37 bytes
[11:44:55] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
```
### server log tail
```
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[11:44:56 INFO]: [ChunkHolderManager] Saved 1443 block chunks, 1420 entity chunks, 0 poi chunks in world 'minecraft:overworld' in 1.23s
[11:44:56 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:overworld'
[11:44:56 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:overworld'
[11:44:56 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[11:44:56 INFO]: [ChunkHolderManager] Waiting 60s for chunk system to halt for world 'minecraft:the_nether'
[11:44:56 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:the_nether'
[11:44:56 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_nether'
[11:44:56 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_nether' in 0.00s
[11:44:56 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_nether'
[11:44:56 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_nether'
[11:44:56 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[11:44:56 INFO]: [ChunkHolderManager] Waiting 60s for chunk system to halt for world 'minecraft:the_end'
[11:44:56 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:the_end'
[11:44:56 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_end'
[11:44:56 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_end' in 0.00s
[11:44:56 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_end'
[11:44:56 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_end'
[11:44:56 INFO]: Waiting for all RegionFile I/O tasks to complete...
[11:44:56 INFO]: All RegionFile I/O tasks to complete
[11:44:56 INFO]: [MoonriseCommon] Awaiting termination of worker pool for up to 60s...
[11:44:56 INFO]: [MoonriseCommon] Awaiting termination of I/O pool for up to 60s...
[launcher] server exited with code 143
```
### shots
```
-rw-r--r-- 1 runner runner  4073 Nov 11 11:03 lens_12_after_edit_probe4.png
-rw-r--r-- 1 runner runner  4073 Nov 11 11:03 lens_12_after_edit_probe5.png
-rw-r--r-- 1 runner runner  4073 Nov 11 11:04 lens_12_after_edit_probe6.png
-rw-r--r-- 1 runner runner  4073 Nov 11 11:04 lens_12_after_edit_probe7.png
-rw-r--r-- 1 runner runner  4073 Nov 11 11:04 lens_12_after_edit_probe8.png
-rw-r--r-- 1 runner runner  4073 Nov 11 11:04 lens_12_after_edit_probe9.png
-rw-r--r-- 1 runner runner  2653 Nov 11 11:12 lens_13_after_relog_probe0.png
-rw-r--r-- 1 runner runner  2653 Nov 11 11:12 lens_13_after_relog_probe1.png
-rw-r--r-- 1 runner runner  2653 Nov 11 11:13 lens_13_after_relog_probe10.png
-rw-r--r-- 1 runner runner  2653 Nov 11 11:13 lens_13_after_relog_probe11.png
-rw-r--r-- 1 runner runner  2653 Nov 11 11:12 lens_13_after_relog_probe2.png
-rw-r--r-- 1 runner runner  2653 Nov 11 11:12 lens_13_after_relog_probe3.png
-rw-r--r-- 1 runner runner  2653 Nov 11 11:12 lens_13_after_relog_probe4.png
-rw-r--r-- 1 runner runner  2653 Nov 11 11:12 lens_13_after_relog_probe5.png
-rw-r--r-- 1 runner runner  2653 Nov 11 11:13 lens_13_after_relog_probe6.png
-rw-r--r-- 1 runner runner  2653 Nov 11 11:13 lens_13_after_relog_probe7.png
-rw-r--r-- 1 runner runner  2653 Nov 11 11:13 lens_13_after_relog_probe8.png
-rw-r--r-- 1 runner runner  2653 Nov 11 11:13 lens_13_after_relog_probe9.png
drwxr-xr-x 2 runner runner  4096 Nov 11 11:14 lens_parity
drwxr-xr-x 2 runner runner  4096 Nov 11 11:37 parity
drwxr-xr-x 2 runner runner  4096 Nov 11 11:08 parity_dbg
drwxr-xr-x 2 runner runner  4096 Nov 11 11:41 parity_dbg8
-rw-r--r-- 1 runner runner  6676 Nov 11 11:13 probe_counts.txt
drwxr-xr-x 2 runner runner  4096 Nov 11 11:44 rim_mask
-rw-r--r-- 1 runner runner 35269 Nov 11 11:44 summary.md
```

**exit=1** failures=3 leg1=0 leg2=0
