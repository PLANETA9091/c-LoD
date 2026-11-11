- phase: 0 sanity (18:13:47)
### c-LoD visual run (-930,796, marker=red_concrete, vantage -1005,150 yaw 270)
- GL:     Vendor: Mesa (0xffffffff);    Device: llvmpipe (LLVM 21.1.8, 256 bits) (0xffffffff);OpenGL renderer string: llvmpipe (LLVM 21.1.8, 256 bits);OpenGL core profile version string: 4.5 (Core Profile) Mesa 26.0.8-1ubuntu0.3;
- boot + gates OK (18:14:09)
- phase: 1 server booted (18:14:09)
- sky-grid armed
- phase: 2 leg-1 client joined (18:14:28)
- joined session 1 (18:14:28)
- stream live: cells=65536
- phase: 3 scan populated (18:14:50)
- terrain range: 2 terraced mountains at (CX-4,CZ-20)/(CX-2,CZ+16), tops 185/190, grass terraces + stone caps, 66..82 m from the eye (inside the 83.5 m band, chunk edge 64 m)
- scan complete=1 (cells=cells=65536/65536)
- paint settled=1 (wire 60603p)
- baseline try 1: drift=0 px
- lod_silhouette_cols=15 (LOD paint ON SCREEN at 75 m if > 20 — the march-drawn marker column; vanilla cannot render there, chunk edge 64 m; #255 re-measures 49, the ZONE_FAR=48 regression #256 and the control read 0)
- 1903 terrain-green px in the sky region (LOD MOUNTAINS on screen if > 5000; the range paints ColLow-green on low terraces via the fallback palette; flat-world baselines and the pack-free control read ~0)
- burst_close white px: 553 (renderer alive if > 50)
- burst_far white px: 140 (renderer alive in the marker zone if > 20)
- cut walk 40 m: 70 white px (> 20 = rendered)
- cut walk 60 m: 2 white px (> 20 = rendered)
- cut walk 100 m: 0 white px (> 20 = rendered)
- measured far-render cut: 40 m (G4 flap instrument; the decode proof needs >= 83 m from the y=151.6 eye)
- fill issued: 5x111x5 red_concrete at -930,796 (zone forceloaded ±48)
- phase: 4 marker filled (18:17:18)

### /clod-cells AFTER FILL (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 1115 0 0x8400 0x3100
123 122 -940 784 71 0 1 [97, 119, 97] 1112 0 0x8700 0x3100
124 122 -938 784 68 0 1 [97, 119, 97] 1111 0 0x8400 0x3100
125 122 -936 784 62 1 1 [63, 118, 228] 1111 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 1110 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 1109 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 1109 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 1108 0 0x8000 0x5c00
130 122 -926 784 66 0 1 [134, 183, 131] 1108 0 0x8200 0x5c00
131 122 -924 784 75 0 1 [97, 119, 97] 1107 0 0x8b00 0x3100
132 122 -922 784 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 122 -920 784 75 0 1 [97, 119, 97] 18446744073709551615 1 0x8b00 0x3100
134 122 -918 784 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x3100
122 123 -942 786 65 0 1 [97, 119, 97] 1115 0 0x8100 0x3100
123 123 -940 786 65 0 1 [97, 119, 97] 1112 0 0x8100 0x3100
124 123 -938 786 65 0 1 [97, 119, 97] 1111 0 0x8100 0x3100
125 123 -936 786 62 1 1 [63, 118, 228] 1111 0 0x7e00 0xd800
```
### stream diag after fill:
[c-LoD] stream: tick 1800 65536/65536 cells, lap 595, cone 5894 zone 768 dirty 895 (q 9967) ewin 198 rbeat 12p/0 rearm 0c/0t rescan 394659c/2642e, edits +5984p/4500c/565e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 350 forced 162, native 65536c/1091ch fail 0 (3w q 0) helper on, trees 1091ch/fail 0, wire bundle 60891p/147b fail 0 ~0 kB/s cache 13448/24576 h47443 m13448 e0, srbeat 0s/0q drop 0stale/0undec
[18:17:18 INFO]: [c-LoD] stream: tick 1800 65536/65536 cells, lap 595, cone 5894 zone 768 dirty 895 (q 9967) ewin 198 rbeat 12p/0 rearm 0c/0t rescan 394659c/2642e, edits +5984p/4500c/565e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 350 forced 162, native 65536c/1091ch fail 0 (3w q 0) helper on, trees 1091ch/fail 0, wire bundle 60891p/147b fail 0 ~0 kB/s cache 13448/24576 h47443 m13448 e0, srbeat 0s/0q drop 0stale/0undec
- probe 0: red=0 paint=277 lod_silhouette_cols=15 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 1: red=0 paint=277 lod_silhouette_cols=15 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 2: red=0 paint=277 lod_silhouette_cols=15 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 3: red=0 paint=277 lod_silhouette_cols=15 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 4: red=0 paint=1413 lod_silhouette_cols=15 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 5: red=0 paint=3107 lod_silhouette_cols=19 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 6: red=0 paint=2867 lod_silhouette_cols=19 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 7: red=0 paint=2867 lod_silhouette_cols=19 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 8: red=0 paint=2867 lod_silhouette_cols=19 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 9: red=0 paint=2867 lod_silhouette_cols=19 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 10: red=0 paint=2867 lod_silhouette_cols=19 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 11: red=0 paint=3371 lod_silhouette_cols=19 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 12: red=0 paint=3340 lod_silhouette_cols=19 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 13: red=0 paint=3481 lod_silhouette_cols=19 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 14: red=0 paint=3264 lod_silhouette_cols=19 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 15: red=0 paint=3264 lod_silhouette_cols=19 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 16: red=0 paint=3266 lod_silhouette_cols=19 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 17: red=0 paint=3264 lod_silhouette_cols=19 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 18: red=0 paint=3264 lod_silhouette_cols=19 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 19: red=0 paint=3264 lod_silhouette_cols=19 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 20: red=0 paint=3264 lod_silhouette_cols=19 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 21: red=0 paint=3264 lod_silhouette_cols=19 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 22: red=0 paint=3264 lod_silhouette_cols=19 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 23: red=0 paint=3264 lod_silhouette_cols=19 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- phase: 5 leg-1 probes done (18:20:29)
- **FAIL:** leg-1 never saw the marker in 24 probes (red < 60 everywhere) — the edit never reached the LOD paint; this failure was SILENT before v6.31.1
### stream diag after probes:
[18:20:18 INFO]: [c-LoD] stream: tick 3600 65536/65536 cells, lap 869, cone 14045 zone 1575 dirty 938 (q 10008) ewin 156 rbeat 0p/0 rearm 0c/0t rescan 976119c/2711e, edits +7645p/5687c/951e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 350 forced 162, native 65536c/1091ch fail 0 (3w q 0) helper on, trees 1091ch/fail 0, wire bundle 132346p/232b fail 0 ~0 kB/s cache 13484/24576 h118862 m13484 e0, srbeat 0s/0q drop 0stale/0undec
[c-LoD] stream: tick 3700 65536/65536 cells, lap 869, cone 14045 zone 1575 dirty 938 (q 10008) ewin 56 rbeat 0p/0 rearm 0c/0t rescan 1009584c/2711e, edits +7645p/5687c/951e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 350 forced 162, native 65536c/1091ch fail 0 (3w q 0) helper on, trees 1091ch/fail 0, wire bundle 132346p/232b fail 0 ~0 kB/s cache 13484/24576 h118862 m13484 e0, srbeat 0s/0q drop 0stale/0undec
[18:20:28 INFO]: [c-LoD] stream: tick 3700 65536/65536 cells, lap 869, cone 14045 zone 1575 dirty 938 (q 10008) ewin 56 rbeat 0p/0 rearm 0c/0t rescan 1009584c/2711e, edits +7645p/5687c/951e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 350 forced 162, native 65536c/1091ch fail 0 (3w q 0) helper on, trees 1091ch/fail 0, wire bundle 132346p/232b fail 0 ~0 kB/s cache 13484/24576 h118862 m13484 e0, srbeat 0s/0q drop 0stale/0undec
- parity debug tour: 13 vantages (session-1 client, DEBUG pack — evidence only, the gate pairs the §7b world session)
- parity debug tour done: 13 shots in parity_dbg/ (the §7b world session feeds the gate)
- joined session 2 (18:23:17)
- leg-2 baseline scene px (x<940): 25681 (0 = the relog client paints no world at all)
- leg-2 renderer probe NEAR (20 m) white px: 563 raw / 563 changed-vs-baseline (> 20 = the client renders particles)
- leg-2 renderer probe FAR (80 m, marker zone) white px: 127 raw / 127 changed-vs-baseline (> 20 = no distance cut; ~0 with NEAR alive = particles beyond the client chunk radius do not render)
- leg-2 carrier probe green dust (80 m): changed-green px 0 / total changed 3819 vs baseline (> 100 green = a stable green carrier exists; total > 0 with green ~0 = the dust renders but fog washes it — a ruler problem, not a carrier problem)
- phase: 8 leg-2 relog probes (18:26:07)
- relog probe 0: red=0 scene=26770 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 1: red=0 scene=27596 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 2: red=0 scene=28580 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 3: red=0 scene=28766 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 4: red=0 scene=28830 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 5: red=0 scene=33294 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 6: red=0 scene=33284 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 7: red=0 scene=33379 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 8: red=0 scene=33426 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 9: red=0 scene=33244 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 10: red=0 scene=33512 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 11: red=0 scene=38428 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- **FAIL:** marker lost after relog — stream state is not persistent/deterministic
- phase: 9 leg-2 done (18:27:39)

### client log forensics (pipeline/particle/atlas — v6.8.2)
```
java.lang.IllegalStateException: Failed to open OpenAL device
	at net.minecraft.client.sounds.SoundEngine.reload(SoundEngine.java:110)
	at net.minecraft.server.packs.resources.SimplePreparableReloadListener.lambda$reload$1(SimplePreparableReloadListener.java:15)
	at net.minecraft.server.packs.resources.SimpleReloadInstance.lambda$prepareTasks$3(SimpleReloadInstance.java:59)
[18:23:15] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[18:23:15] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[18:23:15] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[18:23:15] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[18:23:15] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[18:23:15] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[18:23:15] [Render thread/INFO]: Created: 2048x2048x4 minecraft:textures/atlas/blocks.png-atlas
[18:23:15] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[18:23:15] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[18:23:15] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[18:23:15] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[18:23:15] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
[18:23:22] [Render thread/INFO]: Reloading ResourceManager: vanilla, vanilla, server/00000000/6b1f2f7e-9c0d-4a4b-8b0a-b10d1b4ab100
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
[18:23:24] [Render thread/ERROR]: Error starting SoundSystem. Turning off sounds & music
java.lang.IllegalStateException: Failed to open OpenAL device
	at net.minecraft.client.sounds.SoundEngine.reload(SoundEngine.java:110)
	at net.minecraft.server.packs.resources.SimplePreparableReloadListener.lambda$reload$1(SimplePreparableReloadListener.java:15)
	at net.minecraft.server.packs.resources.SimpleReloadInstance.lambda$prepareTasks$3(SimpleReloadInstance.java:59)
[18:23:24] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[18:23:24] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[18:23:24] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[18:23:24] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[18:23:24] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[18:23:24] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[18:23:24] [Render thread/INFO]: Created: 2048x2048x4 minecraft:textures/atlas/blocks.png-atlas
[18:23:24] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[18:23:24] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[18:23:24] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[18:23:24] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[18:23:24] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
```

### /clod-cells AFTER PROBES (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 28 0 0x8400 0x3100
123 122 -940 784 71 0 1 [97, 119, 97] 28 0 0x8700 0x3100
124 122 -938 784 68 0 1 [97, 119, 97] 27 0 0x8400 0x3100
125 122 -936 784 62 1 1 [63, 118, 228] 27 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 26 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 25 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 25 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 24 0 0x8000 0x5c00
130 122 -926 784 66 0 1 [134, 183, 131] 24 0 0x8200 0x5c00
131 122 -924 784 75 0 1 [97, 119, 97] 23 0 0x8b00 0x3100
132 122 -922 784 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 122 -920 784 75 0 1 [97, 119, 97] 18446744073709551615 1 0x8b00 0x3100
134 122 -918 784 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x3100
122 123 -942 786 65 0 1 [97, 119, 97] 28 0 0x8100 0x3100
123 123 -940 786 65 0 1 [97, 119, 97] 28 0 0x8100 0x3100
124 123 -938 786 65 0 1 [97, 119, 97] 27 0 0x8100 0x3100
125 123 -936 786 62 1 1 [63, 118, 228] 27 0 0x7e00 0xd800
```
- edit channel: 0 linkage errors, 0 failed event dispatches (both must be 0)
- phase: 6 parity world boot (18:27:39)
- parity world: view-distance=8 (the voxy leg's server value)
- parity world: joined session 3 (18:28:14)
- parity world: scan populated (cells=65536; the march paints from these buffers)
- parity world tour: 13 vantages (production pack, vd 8, shared structures)
- coverage sweeps: 72 eye passes = 36 positions (12 az x r144/200 + 12 off-az x r172, y=160) x 2 looks (outward+inward), settle-by-laps >= 6 s (q stable 3x1s), deadline 12 s/pos, no shots — the v6.26.0 data-coverage lever + v6.26.6 lottery kill; probe rides parity/view_probe.md
- parity drain-settle TIMEOUT: queue 24948 -> 24948 > 11500 after 150s — continuing anyway; the queue did not drain without eye proximity (priority-lane evidence, the 01/05/10 dip attribution reads from the tour deltas)
- pre-tour pass done: 13 vantages x 10s (the tour now reads the second OR-merge lap; the rim/dip transport race reads from the tour deltas)
- parity world tour done: 13 shots, 0 night-guard renames in parity/ (the gate pairs voxy/<name>.png vs parity/<name>.png)
- phase: 7 parity tour done (18:50:12)
- sparse re-beat channel at tour end: streamed=41880 q=0 dropped_stale=50 dropped_undecodable=7439 (frac=8 boot; undecodable-dominant = the rim words die in their decodability window, stale-dominant = edit churn owns the cells)
### parity world stream counters (session 3, tail + maxima)
[c-LoD] stream: tick 13200 65536/65536 cells, lap 7806, cone 123579 zone 21203 dirty 10118 (q 24910) ewin 0 rbeat 0p/1 rearm 0c/0t rescan 9051571c/1531e, edits +17905p/13558c/2734e drop 0, spawned 0 / failed 0, cam0 yaw -180 pitch 5, wish 0 forced 902, native 36720c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 1512673p/3040b fail 0 ~0 kB/s cache 20270/24576 h1172895 m339778 e319508, srbeat 41880s/0q drop 50stale/7439undec
[18:50:07 INFO]: [c-LoD] stream: tick 13200 65536/65536 cells, lap 7806, cone 123579 zone 21203 dirty 10118 (q 24910) ewin 0 rbeat 0p/1 rearm 0c/0t rescan 9051571c/1531e, edits +17905p/13558c/2734e drop 0, spawned 0 / failed 0, cam0 yaw -180 pitch 5, wish 0 forced 902, native 36720c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 1512673p/3040b fail 0 ~0 kB/s cache 20270/24576 h1172895 m339778 e319508, srbeat 41880s/0q drop 50stale/7439undec
- parity stream max cone: 123579
- parity stream max zone: 21203
- parity stream max wire bundle: bundle
### parity world capture lens (debug chain on the §7b world)
lens_red=0 lens_black=0 lens_water=0 lens_dark=0 lens_zero=0 rot=none scan=0 carriers=0
- parity mesh probe @png_y=2: x306=(151,184,255) x307=(151,184,255) x308=(151,184,255) x309h=(151,184,255) x310c=(151,184,255)
- parity mesh probe @png_y=717: x306=(46,66,53) x307=(59,76,64) x308=(62,80,67) x309h=(57,74,63) x310c=(47,67,54)
- parity mesh engagement: magenta-ish=0 of 921600 sampled px
- mask8: the un-fog engagement masks shot on the PARITY WORLD (vd 8) — parity_dbg8/01m|07m|10m
- mask8 01m: green=3760 red=0 blue=111648 dim=252016 (px, sampled /16)
- mask8 07m: green=10880 red=0 blue=221920 dim=5888 (px, sampled /16)
- mask8 10m: green=2064 red=32 blue=212560 dim=117888 (px, sampled /16)
- mesh probe @png_y=2: x306=(255,255,255) x307=(13,254,0) x308=(0,254,0) x309h=(0,0,0) x310c=(0,0,0)
- mesh probe @png_y=717: x306=(255,255,255) x307=(13,254,0) x308=(0,254,0) x309h=(0,0,0) x310c=(0,0,0)
- mesh engagement: magenta-ish=256 of 921600 sampled px (the vanilla-path paint coverage)
- mask8 fog state @png_y=2: raw (300)=(151,184,255) (301)=(151,184,255)
- mask8 fog state @png_y=717: raw (300)=(60,68,74) (301)=(25,32,28)
- mask8 fog state decoded: envStart=480 envEnd=544 rdEnd=1184 fogColor=(25,32,28)
- buffer dump 10m: marked h=0/2048 c=0/2048; height windows r0<142:0 r1:0 r2>=202:0 maxByte=0
- buffer dump 10m: color idx0(black)=0 water/height mismatches=0 of 0 pairs
- buffer dump 01m: marked h=0/2048 c=0/2048; height windows r0<142:0 r1:0 r2>=202:0 maxByte=0
- buffer dump 01m: color idx0(black)=0 water/height mismatches=0 of 0 pairs
- rim mask: scan populated (cells=65536; the state paint reads the same buffers the march reads)
- phase: 10 rim mask lane (18:56:01)
- rim mask: decode-state frames shot (rim_mask/00|07|01|12 — GREEN full decode / RED colorless / BLUE heightless; 00 = the leg-1 tower eye)
- rim mask 07: green=1200 red=0 blue=416 (px, sampled /16) -> DOMINANT GREEN: rim fully decoded — the fallback is the PAINT path (shade/fog/composite), not data
- rim mask 01: green=16 red=752 blue=2800 (px, sampled /16) -> DOMINANT BLUE: rim = HEIGHTS NEVER DECODED — a data-coverage class: the state paint cannot even see the cells
- rim mask 12: green=1200 red=0 blue=400 (px, sampled /16) -> DOMINANT GREEN: rim fully decoded — the fallback is the PAINT path (shade/fog/composite), not data

### boot / stream
```
c-LoD pack finalized (ONE-TIME, static): shader-only, 124538 bytes, sha1 fe5469942cc8784a45bb58789463397df61e3b4f — zero textures, zero world data, one push per join
[18:53:40 INFO]: [c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 124538 bytes, sha1 fe5469942cc8784a45bb58789463397df61e3b4f — zero textures, zero world data, one push per join
[c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[18:53:40 INFO]: [c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[18:56:48 INFO]: [c-LoD] stream: tick 1800 65536/65536 cells, lap 1484, cone 24448 zone 730 dirty 22 (q 1080) ewin 161 rbeat 0p/0 rearm 0c/0t rescan 1154945c/5e, edits +26p/26c/10e drop 0, spawned 0 / failed 0, cam0 yaw -180 pitch 5, wish 198 forced 709, native 9888c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 267076p/626b fail 0 ~0 kB/s cache 11917/24576 h255159 m11917 e0, srbeat 8350s/0q drop 0stale/608undec
[c-LoD] stream: tick 1800 65536/65536 cells, lap 1484, cone 24448 zone 730 dirty 22 (q 1080) ewin 161 rbeat 0p/0 rearm 0c/0t rescan 1154945c/5e, edits +26p/26c/10e drop 0, spawned 0 / failed 0, cam0 yaw -180 pitch 5, wish 198 forced 709, native 9888c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 267076p/626b fail 0 ~0 kB/s cache 11917/24576 h255159 m11917 e0, srbeat 8350s/0q drop 0stale/608undec
```
### sessions
```
[18:53:58 INFO]: System chat: Stepan joined the game
[18:56:56 INFO]: System chat: Stepan left the game
```
### client log tail
```
[18:56:55] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[18:56:55] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:section_blocks_update] net.minecraft.network.protocol.game.ClientboundSectionBlocksUpdatePacket -> 18 bytes
[18:56:55] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[18:56:55] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:entity_position_sync] net.minecraft.network.protocol.game.ClientboundEntityPositionSyncPacket -> 37 bytes
[18:56:55] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:entity_position_sync] net.minecraft.network.protocol.game.ClientboundEntityPositionSyncPacket -> 37 bytes
[18:56:55] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:entity_position_sync] net.minecraft.network.protocol.game.ClientboundEntityPositionSyncPacket -> 37 bytes
[18:56:55] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[18:56:55] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[18:56:55] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:move_player_pos] net.minecraft.network.protocol.game.ServerboundMovePlayerPacket$Pos -> 26 bytes
[18:56:55] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[18:56:55] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[18:56:55] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:entity_position_sync] net.minecraft.network.protocol.game.ClientboundEntityPositionSyncPacket -> 37 bytes
[18:56:55] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:entity_position_sync] net.minecraft.network.protocol.game.ClientboundEntityPositionSyncPacket -> 37 bytes
[18:56:55] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:entity_position_sync] net.minecraft.network.protocol.game.ClientboundEntityPositionSyncPacket -> 37 bytes
[18:56:55] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[18:56:55] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[18:56:56] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:entity_position_sync] net.minecraft.network.protocol.game.ClientboundEntityPositionSyncPacket -> 37 bytes
[18:56:56] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:entity_position_sync] net.minecraft.network.protocol.game.ClientboundEntityPositionSyncPacket -> 37 bytes
[18:56:56] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:entity_position_sync] net.minecraft.network.protocol.game.ClientboundEntityPositionSyncPacket -> 37 bytes
[18:56:56] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[18:56:56] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:keep_alive] net.minecraft.network.protocol.common.ClientboundKeepAlivePacket -> 9 bytes
[18:56:56] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:keep_alive] net.minecraft.network.protocol.common.ServerboundKeepAlivePacket -> 9 bytes
[18:56:56] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[18:56:56] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[18:56:56] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:set_time] net.minecraft.network.protocol.game.ClientboundSetTimePacket -> 10 bytes
[18:56:56] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:entity_position_sync] net.minecraft.network.protocol.game.ClientboundEntityPositionSyncPacket -> 37 bytes
[18:56:56] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:entity_position_sync] net.minecraft.network.protocol.game.ClientboundEntityPositionSyncPacket -> 37 bytes
[18:56:56] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:entity_position_sync] net.minecraft.network.protocol.game.ClientboundEntityPositionSyncPacket -> 37 bytes
[18:56:56] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[18:56:56] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
```
### server log tail
```
[c-LoD] EB: enter, native_gen=true
[18:56:57 INFO]: [ChunkHolderManager] Saved 1447 block chunks, 1417 entity chunks, 0 poi chunks in world 'minecraft:overworld' in 1.02s
[18:56:57 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:overworld'
[18:56:57 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:overworld'
[18:56:57 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[18:56:57 INFO]: [ChunkHolderManager] Waiting 60s for chunk system to halt for world 'minecraft:the_nether'
[18:56:57 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:the_nether'
[18:56:57 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_nether'
[18:56:57 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_nether' in 0.00s
[18:56:57 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_nether'
[18:56:57 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_nether'
[18:56:57 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[18:56:57 INFO]: [ChunkHolderManager] Waiting 60s for chunk system to halt for world 'minecraft:the_end'
[18:56:57 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:the_end'
[18:56:57 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_end'
[18:56:57 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_end' in 0.00s
[18:56:57 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_end'
[18:56:57 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_end'
[18:56:57 INFO]: Waiting for all RegionFile I/O tasks to complete...
[18:56:57 INFO]: All RegionFile I/O tasks to complete
[18:56:57 INFO]: [MoonriseCommon] Awaiting termination of worker pool for up to 60s...
[18:56:57 INFO]: [MoonriseCommon] Awaiting termination of I/O pool for up to 60s...
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[launcher] server exited with code 143
```
### shots
```
-rw-r--r-- 1 runner runner  4044 Nov 11 18:17 lens_12_after_edit_probe4.png
-rw-r--r-- 1 runner runner  3908 Nov 11 18:18 lens_12_after_edit_probe5.png
-rw-r--r-- 1 runner runner  3801 Nov 11 18:18 lens_12_after_edit_probe6.png
-rw-r--r-- 1 runner runner  3801 Nov 11 18:18 lens_12_after_edit_probe7.png
-rw-r--r-- 1 runner runner  3801 Nov 11 18:18 lens_12_after_edit_probe8.png
-rw-r--r-- 1 runner runner  3801 Nov 11 18:18 lens_12_after_edit_probe9.png
-rw-r--r-- 1 runner runner  3779 Nov 11 18:26 lens_13_after_relog_probe0.png
-rw-r--r-- 1 runner runner  3572 Nov 11 18:26 lens_13_after_relog_probe1.png
-rw-r--r-- 1 runner runner  3949 Nov 11 18:27 lens_13_after_relog_probe10.png
-rw-r--r-- 1 runner runner  3848 Nov 11 18:27 lens_13_after_relog_probe11.png
-rw-r--r-- 1 runner runner  4099 Nov 11 18:26 lens_13_after_relog_probe2.png
-rw-r--r-- 1 runner runner  3818 Nov 11 18:26 lens_13_after_relog_probe3.png
-rw-r--r-- 1 runner runner  4024 Nov 11 18:26 lens_13_after_relog_probe4.png
-rw-r--r-- 1 runner runner  4094 Nov 11 18:26 lens_13_after_relog_probe5.png
-rw-r--r-- 1 runner runner  4076 Nov 11 18:26 lens_13_after_relog_probe6.png
-rw-r--r-- 1 runner runner  3829 Nov 11 18:27 lens_13_after_relog_probe7.png
-rw-r--r-- 1 runner runner  3692 Nov 11 18:27 lens_13_after_relog_probe8.png
-rw-r--r-- 1 runner runner  4115 Nov 11 18:27 lens_13_after_relog_probe9.png
drwxr-xr-x 2 runner runner  4096 Nov 11 18:50 lens_parity
drwxr-xr-x 2 runner runner  4096 Nov 11 18:50 parity
drwxr-xr-x 2 runner runner  4096 Nov 11 18:22 parity_dbg
drwxr-xr-x 2 runner runner  4096 Nov 11 18:53 parity_dbg8
-rw-r--r-- 1 runner runner  6733 Nov 11 18:27 probe_counts.txt
drwxr-xr-x 2 runner runner  4096 Nov 11 18:56 rim_mask
-rw-r--r-- 1 runner runner 34499 Nov 11 18:56 summary.md
```

**exit=1** failures=2 leg1=0 leg2=0
