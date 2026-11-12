- phase: 0 sanity (16:51:32)
### c-LoD visual run (-930,796, marker=red_concrete, vantage -1005,150 yaw 270)
- GL:     Vendor: Mesa (0xffffffff);    Device: llvmpipe (LLVM 21.1.8, 256 bits) (0xffffffff);OpenGL renderer string: llvmpipe (LLVM 21.1.8, 256 bits);OpenGL core profile version string: 4.5 (Core Profile) Mesa 26.0.8-1ubuntu0.3;
- boot + gates OK (16:51:55)
- phase: 1 server booted (16:51:55)
- sky-grid armed
- phase: 2 leg-1 client joined (16:52:13)
- joined session 1 (16:52:13)
- stream live: cells=65536
- phase: 3 scan populated (16:52:35)
- terrain range: 2 terraced mountains at (CX-4,CZ-20)/(CX-2,CZ+16), tops 185/190, grass terraces + stone caps, 66..82 m from the eye (inside the 83.5 m band, chunk edge 64 m)
- scan complete=1 (cells=cells=65536/65536)
- paint settled=1 (wire 60554p)
- baseline try 1: drift=0 px
- lod_silhouette_cols=19 (LOD paint ON SCREEN at 75 m if > 20 — the march-drawn marker column; vanilla cannot render there, chunk edge 64 m; #255 re-measures 49, the ZONE_FAR=48 regression #256 and the control read 0)
- 11 terrain-green px in the sky region (LOD MOUNTAINS on screen if > 5000; the range paints ColLow-green on low terraces via the fallback palette; flat-world baselines and the pack-free control read ~0)
- burst_close white px: 495 (renderer alive if > 50)
- burst_far white px: 0 (renderer alive in the marker zone if > 20)
- cut walk 40 m: 69 white px (> 20 = rendered)
- cut walk 60 m: 1 white px (> 20 = rendered)
- cut walk 100 m: 0 white px (> 20 = rendered)
- measured far-render cut: 40 m (G4 flap instrument; the decode proof needs >= 83 m from the y=151.6 eye)
- fill issued: 5x111x5 red_concrete at -930,796 (zone forceloaded ±48)
- phase: 4 marker filled (16:55:02)

### /clod-cells AFTER FILL (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 1126 0 0x8400 0x3100
123 122 -940 784 71 0 1 [97, 119, 97] 1122 0 0x8700 0x3100
124 122 -938 784 68 0 1 [97, 119, 97] 1122 0 0x8400 0x3100
125 122 -936 784 62 1 1 [63, 118, 228] 1121 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 1121 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 1120 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 1120 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 1119 0 0x8000 0x5c00
130 122 -926 784 66 0 1 [134, 183, 131] 1119 0 0x8200 0x5c00
131 122 -924 784 75 0 1 [97, 119, 97] 1118 0 0x8b00 0x3100
132 122 -922 784 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 122 -920 784 75 0 1 [97, 119, 97] 18446744073709551615 1 0x8b00 0x3100
134 122 -918 784 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x3100
122 123 -942 786 70 0 1 [97, 119, 97] 1126 0 0x8600 0x3100
123 123 -940 786 65 0 1 [97, 119, 97] 1122 0 0x8100 0x3100
124 123 -938 786 65 0 1 [97, 119, 97] 1122 0 0x8100 0x3100
125 123 -936 786 62 1 1 [63, 118, 228] 1121 0 0x7e00 0xd800
```
### stream diag after fill:
[c-LoD] stream: tick 1800 65536/65536 cells, lap 598, cone 5904 zone 768 dirty 880 (q 10827) ewin 186 rbeat 9p/0 rearm 0c/0t rescan 398118c/2658e, edits +5819p/4379c/577e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 350 forced 162, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 60815p/143b fail 0 ~0 kB/s cache 13311/24576 h47504 m13311 e0, srbeat 0s/0q drop 0stale/0undec
[16:55:03 INFO]: [c-LoD] stream: tick 1800 65536/65536 cells, lap 598, cone 5904 zone 768 dirty 880 (q 10827) ewin 186 rbeat 9p/0 rearm 0c/0t rescan 398118c/2658e, edits +5819p/4379c/577e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 350 forced 162, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 60815p/143b fail 0 ~0 kB/s cache 13311/24576 h47504 m13311 e0, srbeat 0s/0q drop 0stale/0undec
- probe 0: red=0 paint=2452 lod_silhouette_cols=14 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 1: red=0 paint=2452 lod_silhouette_cols=14 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 2: red=0 paint=2452 lod_silhouette_cols=14 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 3: red=0 paint=2452 lod_silhouette_cols=14 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 4: red=0 paint=2455 lod_silhouette_cols=14 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 5: red=0 paint=2452 lod_silhouette_cols=14 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 6: red=0 paint=2452 lod_silhouette_cols=14 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 7: red=0 paint=2452 lod_silhouette_cols=14 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 8: red=0 paint=2452 lod_silhouette_cols=14 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 9: red=0 paint=2452 lod_silhouette_cols=14 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 10: red=0 paint=4272 lod_silhouette_cols=20 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 11: red=0 paint=6297 lod_silhouette_cols=20 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 12: red=0 paint=12342 lod_silhouette_cols=21 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 13: red=0 paint=12421 lod_silhouette_cols=21 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 14: red=0 paint=12415 lod_silhouette_cols=21 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 15: red=0 paint=12415 lod_silhouette_cols=21 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 16: red=0 paint=12415 lod_silhouette_cols=21 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 17: red=0 paint=12415 lod_silhouette_cols=21 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 18: red=0 paint=12415 lod_silhouette_cols=21 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 19: red=0 paint=12535 lod_silhouette_cols=20 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 20: red=0 paint=14126 lod_silhouette_cols=23 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 21: red=0 paint=14623 lod_silhouette_cols=25 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 22: red=0 paint=14614 lod_silhouette_cols=25 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 23: red=0 paint=14902 lod_silhouette_cols=25 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- phase: 5 leg-1 probes done (16:58:10)
- **FAIL:** leg-1 never saw the marker in 24 probes (red < 60 everywhere) — the edit never reached the LOD paint; this failure was SILENT before v6.31.1
### stream diag after probes:
[16:57:54 INFO]: [c-LoD] stream: tick 3500 65536/65536 cells, lap 1089, cone 18508 zone 1580 dirty 919 (q 10863) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 946219c/2718e, edits +7402p/5502c/924e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 350 forced 162, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 167610p/260b fail 0 ~123 kB/s cache 13341/24576 h154269 m13341 e0, srbeat 0s/0q drop 0stale/0undec
[c-LoD] stream: tick 3600 65536/65536 cells, lap 1189, cone 21464 zone 1580 dirty 919 (q 10863) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 979629c/2718e, edits +7405p/5505c/924e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 350 forced 162, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 191062p/284b fail 0 ~123 kB/s cache 13341/24576 h177721 m13341 e0, srbeat 0s/0q drop 0stale/0undec
[16:58:04 INFO]: [c-LoD] stream: tick 3600 65536/65536 cells, lap 1189, cone 21464 zone 1580 dirty 919 (q 10863) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 979629c/2718e, edits +7405p/5505c/924e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 350 forced 162, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 191062p/284b fail 0 ~123 kB/s cache 13341/24576 h177721 m13341 e0, srbeat 0s/0q drop 0stale/0undec
- parity debug tour: 13 vantages (session-1 client, DEBUG pack — evidence only, the gate pairs the §7b world session)
- parity debug tour done: 13 shots in parity_dbg/ (the §7b world session feeds the gate)
- joined session 2 (17:00:58)
- leg-2 baseline scene px (x<940): 15039 (0 = the relog client paints no world at all)
- leg-2 renderer probe NEAR (20 m) white px: 510 raw / 510 changed-vs-baseline (> 20 = the client renders particles)
- leg-2 renderer probe FAR (80 m, marker zone) white px: 0 raw / 0 changed-vs-baseline (> 20 = no distance cut; ~0 with NEAR alive = particles beyond the client chunk radius do not render)
- leg-2 carrier probe green dust (80 m): changed-green px 0 / total changed 12326 vs baseline (> 100 green = a stable green carrier exists; total > 0 with green ~0 = the dust renders but fog washes it — a ruler problem, not a carrier problem)
- phase: 8 leg-2 relog probes (17:03:48)
- relog probe 0: red=0 scene=23144 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 1: red=0 scene=23192 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 2: red=0 scene=24511 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 3: red=0 scene=29657 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 4: red=0 scene=29716 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 5: red=0 scene=29993 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 6: red=0 scene=29664 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 7: red=0 scene=30170 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 8: red=0 scene=33864 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 9: red=0 scene=33687 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 10: red=0 scene=37663 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 11: red=0 scene=38359 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- **FAIL:** marker lost after relog — stream state is not persistent/deterministic
- phase: 9 leg-2 done (17:05:18)

### client log forensics (pipeline/particle/atlas — v6.8.2)
```
java.lang.IllegalStateException: Failed to open OpenAL device
	at net.minecraft.client.sounds.SoundEngine.reload(SoundEngine.java:110)
	at net.minecraft.server.packs.resources.SimplePreparableReloadListener.lambda$reload$1(SimplePreparableReloadListener.java:15)
	at net.minecraft.server.packs.resources.SimpleReloadInstance.lambda$prepareTasks$3(SimpleReloadInstance.java:59)
[17:00:56] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[17:00:56] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[17:00:56] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[17:00:56] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[17:00:56] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[17:00:56] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[17:00:56] [Render thread/INFO]: Created: 2048x2048x4 minecraft:textures/atlas/blocks.png-atlas
[17:00:56] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[17:00:56] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[17:00:56] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[17:00:56] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[17:00:56] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
[17:01:03] [Render thread/INFO]: Reloading ResourceManager: vanilla, vanilla, server/00000000/6b1f2f7e-9c0d-4a4b-8b0a-b10d1b4ab100
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
[17:01:05] [Render thread/ERROR]: Error starting SoundSystem. Turning off sounds & music
java.lang.IllegalStateException: Failed to open OpenAL device
	at net.minecraft.client.sounds.SoundEngine.reload(SoundEngine.java:110)
	at net.minecraft.server.packs.resources.SimplePreparableReloadListener.lambda$reload$1(SimplePreparableReloadListener.java:15)
	at net.minecraft.server.packs.resources.SimpleReloadInstance.lambda$prepareTasks$3(SimpleReloadInstance.java:59)
[17:01:05] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[17:01:05] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[17:01:05] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[17:01:05] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[17:01:05] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[17:01:05] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[17:01:05] [Render thread/INFO]: Created: 2048x2048x4 minecraft:textures/atlas/blocks.png-atlas
[17:01:05] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[17:01:05] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[17:01:05] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[17:01:05] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[17:01:05] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
```

### /clod-cells AFTER PROBES (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 33 0 0x8400 0x3100
123 122 -940 784 71 0 1 [97, 119, 97] 33 0 0x8700 0x3100
124 122 -938 784 68 0 1 [97, 119, 97] 32 0 0x8400 0x3100
125 122 -936 784 62 1 1 [63, 118, 228] 32 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 31 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 30 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 30 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 29 0 0x8000 0x5c00
130 122 -926 784 66 0 1 [134, 183, 131] 29 0 0x8200 0x5c00
131 122 -924 784 75 0 1 [97, 119, 97] 28 0 0x8b00 0x3100
132 122 -922 784 77 0 1 [97, 119, 97] 3381 0 0x8d00 0x3100
133 122 -920 784 75 0 1 [97, 119, 97] 3381 0 0x8b00 0x3100
134 122 -918 784 73 0 1 [97, 119, 97] 3381 0 0x8900 0x3100
122 123 -942 786 70 0 1 [97, 119, 97] 33 0 0x8600 0x3100
123 123 -940 786 65 0 1 [97, 119, 97] 33 0 0x8100 0x3100
124 123 -938 786 65 0 1 [97, 119, 97] 32 0 0x8100 0x3100
125 123 -936 786 62 1 1 [63, 118, 228] 32 0 0x7e00 0xd800
```
- edit channel: 0 linkage errors, 0 failed event dispatches (both must be 0)
- phase: 6 parity world boot (17:05:18)
- parity world: view-distance=8 (the voxy leg's server value)
- parity world: joined session 3 (17:05:52)
- parity world: scan populated (cells=65536; the march paints from these buffers)
- parity world tour: 13 vantages (production pack, vd 8, shared structures)
- coverage sweeps: 72 eye passes = 36 positions (12 az x r144/200 + 12 off-az x r172, y=160) x 2 looks (outward+inward), settle-by-laps >= 6 s (q stable 3x1s), deadline 12 s/pos, no shots — the v6.26.0 data-coverage lever + v6.26.6 lottery kill; probe rides parity/view_probe.md
- parity drain-settle TIMEOUT: queue 16943 -> 16938 > 11500 after 150s — continuing anyway; the queue did not drain without eye proximity (priority-lane evidence, the 01/05/10 dip attribution reads from the tour deltas)
- pre-tour pass done: 13 vantages x 10s (the tour now reads the second OR-merge lap; the rim/dip transport race reads from the tour deltas)
- parity world tour done: 13 shots, 0 night-guard renames in parity/ (the gate pairs voxy/<name>.png vs parity/<name>.png)
- phase: 7 parity tour done (17:27:35)
- sparse re-beat channel at tour end: streamed=54178 q=0 dropped_stale=23 dropped_undecodable=11662 (frac=8 boot; undecodable-dominant = the rim words die in their decodability window, stale-dominant = edit churn owns the cells)
### parity world stream counters (session 3, tail + maxima)
[c-LoD] stream: tick 13100 65536/65536 cells, lap 9173, cone 163436 zone 24217 dirty 7614 (q 16891) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 8989512c/1392e, edits +17878p/13556c/2856e drop 0, spawned 0 / failed 0, cam0 yaw -180 pitch 5, wish 0 forced 936, native 24384c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 1930340p/4007b fail 0 ~0 kB/s cache 24364/24576 h1506029 m424311 e399947, srbeat 54178s/0q drop 23stale/11662undec
[17:27:35 INFO]: [c-LoD] stream: tick 13100 65536/65536 cells, lap 9173, cone 163436 zone 24217 dirty 7614 (q 16891) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 8989512c/1392e, edits +17878p/13556c/2856e drop 0, spawned 0 / failed 0, cam0 yaw -180 pitch 5, wish 0 forced 936, native 24384c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 1930340p/4007b fail 0 ~0 kB/s cache 24364/24576 h1506029 m424311 e399947, srbeat 54178s/0q drop 23stale/11662undec
- parity stream max cone: 163436
- parity stream max zone: 24217
- parity stream max wire bundle: bundle
### parity world capture lens (debug chain on the §7b world)
lens_red=0 lens_black=0 lens_water=0 lens_dark=0 lens_zero=0 rot=none scan=0 carriers=0
- parity mesh probe @png_y=2: x306=(151,184,255) x307=(151,184,255) x308=(151,184,255) x309h=(151,184,255) x310c=(151,184,255)
- parity mesh probe @png_y=717: x306=(44,62,51) x307=(47,67,54) x308=(43,61,50) x309h=(41,58,48) x310c=(49,70,56)
- parity mesh engagement: magenta-ish=48 of 921600 sampled px
- mask8: the un-fog engagement masks shot on the PARITY WORLD (vd 8) — parity_dbg8/01m|07m|10m
- mask8 01m: green=8592 red=48 blue=97200 dim=346608 (px, sampled /16)
- mask8 07m: green=10960 red=0 blue=221744 dim=5776 (px, sampled /16)
- mask8 10m: green=6448 red=560 blue=207376 dim=132320 (px, sampled /16)
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
- phase: 10 rim mask lane (17:31:37)
- rim mask: decode-state frames shot (rim_mask/00|01|12 — GREEN full decode / RED colorless / BLUE heightless; 00 = the leg-1 tower eye, 01 = the near-tower contrast, 12 = the tower re-read; the inherited 07/12 vantages sat 760 blocks outside the streamed disk and 01 sat INSIDE the marker column — their earlier verdicts were classifier noise, purged in v6.33.1)
- rim mask 00: green=6768 red=92128 blue=87072 yellow=2688 (px, sampled /16) -> DOMINANT RED: rim = COLORLESS class — the color words die client-side (pool competition / fog strobe margin / the v6.23 HeightRef capture gate); heights ARE decoded
- rim mask 01: green=9584 red=150304 blue=141024 yellow=8704 (px, sampled /16) -> DOMINANT RED: rim = COLORLESS class — the color words die client-side (pool competition / fog strobe margin / the v6.23 HeightRef capture gate); heights ARE decoded
- rim mask 12: green=11952 red=122208 blue=118528 yellow=9440 (px, sampled /16) -> DOMINANT RED: rim = COLORLESS class — the color words die client-side (pool competition / fog strobe margin / the v6.23 HeightRef capture gate); heights ARE decoded

### boot / stream
```
[c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 127551 bytes, sha1 9044d9e9174ffbeb08caf7427c6d82dfc20eec0c — zero textures, zero world data, one push per join
[17:30:58 INFO]: [c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 127551 bytes, sha1 9044d9e9174ffbeb08caf7427c6d82dfc20eec0c — zero textures, zero world data, one push per join
[c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[17:30:58 INFO]: [c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[c-LoD] stream: tick 700 65536/65536 cells, lap 594, cone 11073 zone 816 dirty 22 (q 3234) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 417447c/376e, edits +13p/13c/8e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 396 forced 438, native 19740c/952ch fail 0 (3w q 0) helper on, trees 952ch/fail 0, wire bundle 122064p/314b fail 0 ~111 kB/s cache 13637/24576 h108427 m13637 e0, srbeat 3421s/481q drop 0stale/670undec
[17:32:18 INFO]: [c-LoD] stream: tick 700 65536/65536 cells, lap 594, cone 11073 zone 816 dirty 22 (q 3234) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 417447c/376e, edits +13p/13c/8e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 396 forced 438, native 19740c/952ch fail 0 (3w q 0) helper on, trees 952ch/fail 0, wire bundle 122064p/314b fail 0 ~111 kB/s cache 13637/24576 h108427 m13637 e0, srbeat 3421s/481q drop 0stale/670undec
```
### sessions
```
[17:31:16 INFO]: System chat: Stepan joined the game
[17:32:18 INFO]: System chat: Stepan left the game
```
### client log tail
```
[17:32:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[17:32:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[17:32:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[17:32:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[17:32:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[17:32:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[17:32:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[17:32:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[17:32:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[17:32:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[17:32:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[17:32:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[17:32:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[17:32:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[17:32:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[17:32:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[17:32:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[17:32:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[17:32:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[17:32:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[17:32:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[17:32:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[17:32:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[17:32:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[17:32:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[17:32:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[17:32:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[17:32:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[17:32:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[17:32:18] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:bundle_delimiter] net.minecraft.network.protocol.game.ClientboundBundleDelimiterPacket -> 1 bytes
```
### server log tail
```
[17:32:19 INFO]: [ChunkHolderManager] Saved 997 block chunks, 976 entity chunks, 0 poi chunks in world 'minecraft:overworld' in 0.63s
[17:32:19 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:overworld'
[17:32:19 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:overworld'
[17:32:19 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[17:32:19 INFO]: [ChunkHolderManager] Waiting 60s for chunk system to halt for world 'minecraft:the_nether'
[17:32:19 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:the_nether'
[17:32:19 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_nether'
[17:32:19 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_nether' in 0.00s
[17:32:19 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_nether'
[17:32:19 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_nether'
[17:32:19 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[17:32:19 INFO]: [ChunkHolderManager] Waiting 60s for chunk system to halt for world 'minecraft:the_end'
[17:32:19 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:the_end'
[17:32:19 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_end'
[17:32:19 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_end' in 0.00s
[17:32:19 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_end'
[17:32:19 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_end'
[17:32:19 INFO]: Waiting for all RegionFile I/O tasks to complete...
[17:32:19 INFO]: All RegionFile I/O tasks to complete
[17:32:19 INFO]: [MoonriseCommon] Awaiting termination of worker pool for up to 60s...
[17:32:19 INFO]: [MoonriseCommon] Awaiting termination of I/O pool for up to 60s...
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[launcher] server exited with code 143
```
### shots
```
-rw-r--r-- 1 runner runner  3381 Nov 12 16:55 lens_12_after_edit_probe4.png
-rw-r--r-- 1 runner runner  3381 Nov 12 16:55 lens_12_after_edit_probe5.png
-rw-r--r-- 1 runner runner  3381 Nov 12 16:55 lens_12_after_edit_probe6.png
-rw-r--r-- 1 runner runner  3381 Nov 12 16:56 lens_12_after_edit_probe7.png
-rw-r--r-- 1 runner runner  3381 Nov 12 16:56 lens_12_after_edit_probe8.png
-rw-r--r-- 1 runner runner  3381 Nov 12 16:56 lens_12_after_edit_probe9.png
-rw-r--r-- 1 runner runner  3835 Nov 12 17:03 lens_13_after_relog_probe0.png
-rw-r--r-- 1 runner runner  3844 Nov 12 17:03 lens_13_after_relog_probe1.png
-rw-r--r-- 1 runner runner  4262 Nov 12 17:05 lens_13_after_relog_probe10.png
-rw-r--r-- 1 runner runner  4072 Nov 12 17:05 lens_13_after_relog_probe11.png
-rw-r--r-- 1 runner runner  4165 Nov 12 17:04 lens_13_after_relog_probe2.png
-rw-r--r-- 1 runner runner  4082 Nov 12 17:04 lens_13_after_relog_probe3.png
-rw-r--r-- 1 runner runner  3962 Nov 12 17:04 lens_13_after_relog_probe4.png
-rw-r--r-- 1 runner runner  3917 Nov 12 17:04 lens_13_after_relog_probe5.png
-rw-r--r-- 1 runner runner  4291 Nov 12 17:04 lens_13_after_relog_probe6.png
-rw-r--r-- 1 runner runner  4069 Nov 12 17:04 lens_13_after_relog_probe7.png
-rw-r--r-- 1 runner runner  3956 Nov 12 17:04 lens_13_after_relog_probe8.png
-rw-r--r-- 1 runner runner  3937 Nov 12 17:04 lens_13_after_relog_probe9.png
drwxr-xr-x 2 runner runner  4096 Nov 12 17:27 lens_parity
drwxr-xr-x 2 runner runner  4096 Nov 12 17:27 parity
drwxr-xr-x 2 runner runner  4096 Nov 12 17:00 parity_dbg
drwxr-xr-x 2 runner runner  4096 Nov 12 17:30 parity_dbg8
-rw-r--r-- 1 runner runner  6766 Nov 12 17:05 probe_counts.txt
drwxr-xr-x 2 runner runner  4096 Nov 12 17:32 rim_mask
-rw-r--r-- 1 runner runner 34904 Nov 12 17:32 summary.md
```

**exit=1** failures=2 leg1=0 leg2=0
