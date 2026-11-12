- phase: 0 sanity (13:28:33)
### c-LoD visual run (-930,796, marker=red_concrete, vantage -1005,150 yaw 270)
- GL:     Vendor: Mesa (0xffffffff);    Device: llvmpipe (LLVM 21.1.8, 256 bits) (0xffffffff);OpenGL renderer string: llvmpipe (LLVM 21.1.8, 256 bits);OpenGL core profile version string: 4.5 (Core Profile) Mesa 26.0.8-1ubuntu0.3;
- boot + gates OK (13:29:01)
- phase: 1 server booted (13:29:01)
- sky-grid armed
- phase: 2 leg-1 client joined (13:29:28)
- joined session 1 (13:29:28)
- stream live: cells=65536
- phase: 3 scan populated (13:29:51)
- terrain range: 2 terraced mountains at (CX-4,CZ-20)/(CX-2,CZ+16), tops 185/190, grass terraces + stone caps, 66..82 m from the eye (inside the 83.5 m band, chunk edge 64 m)
- scan complete=1 (cells=cells=65536/65536)
- paint settled=1 (wire 60613p)
- baseline try 1: drift=0 px
- lod_silhouette_cols=17 (LOD paint ON SCREEN at 75 m if > 20 — the march-drawn marker column; vanilla cannot render there, chunk edge 64 m; #255 re-measures 49, the ZONE_FAR=48 regression #256 and the control read 0)
- 0 terrain-green px in the sky region (LOD MOUNTAINS on screen if > 5000; the range paints ColLow-green on low terraces via the fallback palette; flat-world baselines and the pack-free control read ~0)
- burst_close white px: 504 (renderer alive if > 50)
- burst_far white px: 441 (renderer alive in the marker zone if > 20)
- cut walk 40 m: 0 white px (> 20 = rendered)
- cut walk 60 m: 0 white px (> 20 = rendered)
- cut walk 100 m: 0 white px (> 20 = rendered)
- measured far-render cut: none m (G4 flap instrument; the decode proof needs >= 83 m from the y=151.6 eye)
- fill issued: 5x111x5 red_concrete at -930,796 (zone forceloaded ±48)
- phase: 4 marker filled (13:32:20)

### /clod-cells AFTER FILL (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 86 0 0x8400 0x3100
123 122 -940 784 71 0 1 [97, 119, 97] 86 0 0x8700 0x3100
124 122 -938 784 68 0 1 [97, 119, 97] 85 0 0x8400 0x3100
125 122 -936 784 62 1 1 [63, 118, 228] 85 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 84 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 84 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 83 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 82 0 0x8000 0x5c00
130 122 -926 784 66 0 1 [134, 183, 131] 82 0 0x8200 0x5c00
131 122 -924 784 75 0 1 [97, 119, 97] 81 0 0x8b00 0x3100
132 122 -922 784 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 122 -920 784 75 0 1 [97, 119, 97] 18446744073709551615 1 0x8b00 0x3100
134 122 -918 784 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x3100
122 123 -942 786 70 0 1 [97, 119, 97] 86 0 0x8600 0x3100
123 123 -940 786 65 0 1 [97, 119, 97] 86 0 0x8100 0x3100
124 123 -938 786 65 0 1 [97, 119, 97] 85 0 0x8100 0x3100
125 123 -936 786 62 1 1 [63, 118, 228] 85 0 0x7e00 0xd800
```
### stream diag after fill:
[c-LoD] stream: tick 1900 65536/65536 cells, lap 767, cone 9637 zone 1524 dirty 892 (q 12643) ewin 214 rbeat 0p/0 rearm 0c/0t rescan 403369c/2036e, edits +5600p/4213c/598e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 294 forced 218, native 65536c/1090ch fail 0 (3w q 0) helper on, trees 1090ch/fail 0, wire bundle 96556p/183b fail 0 ~61 kB/s cache 13330/24576 h83226 m13330 e0, srbeat 0s/0q drop 0stale/0undec
[13:32:22 INFO]: [c-LoD] stream: tick 1900 65536/65536 cells, lap 767, cone 9637 zone 1524 dirty 892 (q 12643) ewin 214 rbeat 0p/0 rearm 0c/0t rescan 403369c/2036e, edits +5600p/4213c/598e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 294 forced 218, native 65536c/1090ch fail 0 (3w q 0) helper on, trees 1090ch/fail 0, wire bundle 96556p/183b fail 0 ~61 kB/s cache 13330/24576 h83226 m13330 e0, srbeat 0s/0q drop 0stale/0undec
- probe 0: red=0 paint=3484 lod_silhouette_cols=23 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 1: red=0 paint=3484 lod_silhouette_cols=23 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 2: red=0 paint=3484 lod_silhouette_cols=23 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 3: red=0 paint=3484 lod_silhouette_cols=23 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 4: red=0 paint=3484 lod_silhouette_cols=23 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 5: red=0 paint=5541 lod_silhouette_cols=24 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 6: red=0 paint=10499 lod_silhouette_cols=24 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 7: red=0 paint=10715 lod_silhouette_cols=24 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 8: red=0 paint=13533 lod_silhouette_cols=33 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 9: red=0 paint=14429 lod_silhouette_cols=33 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 10: red=0 paint=14725 lod_silhouette_cols=33 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 11: red=0 paint=14589 lod_silhouette_cols=33 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 12: red=0 paint=14909 lod_silhouette_cols=33 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 13: red=0 paint=14692 lod_silhouette_cols=33 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 14: red=0 paint=14692 lod_silhouette_cols=33 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 15: red=0 paint=14692 lod_silhouette_cols=33 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 16: red=0 paint=14692 lod_silhouette_cols=33 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 17: red=0 paint=14692 lod_silhouette_cols=33 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 18: red=0 paint=14997 lod_silhouette_cols=33 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 19: red=0 paint=14956 lod_silhouette_cols=33 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 20: red=0 paint=15103 lod_silhouette_cols=33 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 21: red=0 paint=17415 lod_silhouette_cols=33 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 22: red=0 paint=18291 lod_silhouette_cols=35 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 23: red=0 paint=18371 lod_silhouette_cols=35 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- phase: 5 leg-1 probes done (13:35:43)
- **FAIL:** leg-1 never saw the marker in 24 probes (red < 60 everywhere) — the edit never reached the LOD paint; this failure was SILENT before v6.31.1
### stream diag after probes:
[13:35:32 INFO]: [c-LoD] stream: tick 3800 65536/65536 cells, lap 1794, cone 35787 zone 1642 dirty 942 (q 12641) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1017403c/2093e, edits +7125p/5289c/908e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 294 forced 218, native 65536c/1090ch fail 0 (3w q 0) helper on, trees 1090ch/fail 0, wire bundle 304448p/402b fail 0 ~99 kB/s cache 13422/24576 h291026 m13422 e0, srbeat 0s/0q drop 0stale/0undec
[c-LoD] stream: tick 3900 65536/65536 cells, lap 1894, cone 37719 zone 1642 dirty 942 (q 12642) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1049489c/2093e, edits +7128p/5292c/909e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 294 forced 218, native 65536c/1090ch fail 0 (3w q 0) helper on, trees 1090ch/fail 0, wire bundle 320780p/419b fail 0 ~86 kB/s cache 13422/24576 h307358 m13422 e0, srbeat 0s/0q drop 0stale/0undec
[13:35:42 INFO]: [c-LoD] stream: tick 3900 65536/65536 cells, lap 1894, cone 37719 zone 1642 dirty 942 (q 12642) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1049489c/2093e, edits +7128p/5292c/909e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 294 forced 218, native 65536c/1090ch fail 0 (3w q 0) helper on, trees 1090ch/fail 0, wire bundle 320780p/419b fail 0 ~86 kB/s cache 13422/24576 h307358 m13422 e0, srbeat 0s/0q drop 0stale/0undec
- parity debug tour: 13 vantages (session-1 client, DEBUG pack — evidence only, the gate pairs the §7b world session)
- parity debug tour done: 13 shots in parity_dbg/ (the §7b world session feeds the gate)
- joined session 2 (13:38:36)
- leg-2 baseline scene px (x<940): 30645 (0 = the relog client paints no world at all)
- leg-2 renderer probe NEAR (20 m) white px: 0 raw / 0 changed-vs-baseline (> 20 = the client renders particles)
- leg-2 renderer probe FAR (80 m, marker zone) white px: 476 raw / 476 changed-vs-baseline (> 20 = no distance cut; ~0 with NEAR alive = particles beyond the client chunk radius do not render)
- leg-2 carrier probe green dust (80 m): changed-green px 0 / total changed 13351 vs baseline (> 100 green = a stable green carrier exists; total > 0 with green ~0 = the dust renders but fog washes it — a ruler problem, not a carrier problem)
- phase: 8 leg-2 relog probes (13:41:28)
- relog probe 0: red=0 scene=31630 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 1: red=0 scene=32424 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 2: red=0 scene=33130 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 3: red=0 scene=33579 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 4: red=0 scene=37046 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 5: red=0 scene=37475 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 6: red=0 scene=37848 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 7: red=0 scene=37451 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 8: red=0 scene=37754 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 9: red=0 scene=38474 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 10: red=0 scene=38706 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 11: red=0 scene=39027 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- **FAIL:** marker lost after relog — stream state is not persistent/deterministic
- phase: 9 leg-2 done (13:43:05)

### client log forensics (pipeline/particle/atlas — v6.8.2)
```
java.lang.IllegalStateException: Failed to open OpenAL device
	at net.minecraft.client.sounds.SoundEngine.reload(SoundEngine.java:110)
	at net.minecraft.server.packs.resources.SimplePreparableReloadListener.lambda$reload$1(SimplePreparableReloadListener.java:15)
	at net.minecraft.server.packs.resources.SimpleReloadInstance.lambda$prepareTasks$3(SimpleReloadInstance.java:59)
[13:38:33] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[13:38:33] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[13:38:33] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[13:38:33] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[13:38:33] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[13:38:33] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[13:38:33] [Render thread/INFO]: Created: 2048x2048x4 minecraft:textures/atlas/blocks.png-atlas
[13:38:33] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[13:38:33] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[13:38:33] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[13:38:33] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[13:38:33] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
[13:38:41] [Render thread/INFO]: Reloading ResourceManager: vanilla, vanilla, server/00000000/6b1f2f7e-9c0d-4a4b-8b0a-b10d1b4ab100
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
[13:38:43] [Render thread/ERROR]: Error starting SoundSystem. Turning off sounds & music
java.lang.IllegalStateException: Failed to open OpenAL device
	at net.minecraft.client.sounds.SoundEngine.reload(SoundEngine.java:110)
	at net.minecraft.server.packs.resources.SimplePreparableReloadListener.lambda$reload$1(SimplePreparableReloadListener.java:15)
	at net.minecraft.server.packs.resources.SimpleReloadInstance.lambda$prepareTasks$3(SimpleReloadInstance.java:59)
[13:38:43] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[13:38:43] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[13:38:43] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[13:38:43] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[13:38:43] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[13:38:43] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[13:38:43] [Render thread/INFO]: Created: 2048x2048x4 minecraft:textures/atlas/blocks.png-atlas
[13:38:43] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[13:38:43] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[13:38:43] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[13:38:43] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[13:38:43] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
```

### /clod-cells AFTER PROBES (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 38 0 0x8400 0x3100
123 122 -940 784 71 0 1 [97, 119, 97] 38 0 0x8700 0x3100
124 122 -938 784 68 0 1 [97, 119, 97] 37 0 0x8400 0x3100
125 122 -936 784 62 1 1 [63, 118, 228] 37 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 36 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 36 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 35 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 34 0 0x8000 0x5c00
130 122 -926 784 66 0 1 [134, 183, 131] 34 0 0x8200 0x5c00
131 122 -924 784 75 0 1 [97, 119, 97] 33 0 0x8b00 0x3100
132 122 -922 784 77 0 1 [97, 119, 97] 3550 0 0x8d00 0x3100
133 122 -920 784 75 0 1 [97, 119, 97] 3550 0 0x8b00 0x3100
134 122 -918 784 73 0 1 [97, 119, 97] 3550 0 0x8900 0x3100
122 123 -942 786 70 0 1 [97, 119, 97] 38 0 0x8600 0x3100
123 123 -940 786 65 0 1 [97, 119, 97] 38 0 0x8100 0x3100
124 123 -938 786 65 0 1 [97, 119, 97] 37 0 0x8100 0x3100
125 123 -936 786 62 1 1 [63, 118, 228] 37 0 0x7e00 0xd800
```
- edit channel: 0 linkage errors, 0 failed event dispatches (both must be 0)
- phase: 6 parity world boot (13:43:06)
- parity world: view-distance=8 (the voxy leg's server value)
- parity world: joined session 3 (13:43:50)
- parity world: scan populated (cells=65536; the march paints from these buffers)
- parity world tour: 13 vantages (production pack, vd 8, shared structures)
- coverage sweeps: 72 eye passes = 36 positions (12 az x r144/200 + 12 off-az x r172, y=160) x 2 looks (outward+inward), settle-by-laps >= 6 s (q stable 3x1s), deadline 12 s/pos, no shots — the v6.26.0 data-coverage lever + v6.26.6 lottery kill; probe rides parity/view_probe.md
- parity drain-settle TIMEOUT: queue 21555 -> 21555 > 11500 after 150s — continuing anyway; the queue did not drain without eye proximity (priority-lane evidence, the 01/05/10 dip attribution reads from the tour deltas)
- pre-tour pass done: 13 vantages x 10s (the tour now reads the second OR-merge lap; the rim/dip transport race reads from the tour deltas)
- parity world tour done: 13 shots, 0 night-guard renames in parity/ (the gate pairs voxy/<name>.png vs parity/<name>.png)
- phase: 7 parity tour done (14:05:55)
- sparse re-beat channel at tour end: streamed=48772 q=0 dropped_stale=31 dropped_undecodable=10357 (frac=8 boot; undecodable-dominant = the rim words die in their decodability window, stale-dominant = edit churn owns the cells)
### parity world stream counters (session 3, tail + maxima)
[c-LoD] stream: tick 13300 65536/65536 cells, lap 8409, cone 153099 zone 21670 dirty 8315 (q 21547) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 9078272c/2106e, edits +18190p/13784c/2842e drop 0, spawned 0 / failed 0, cam0 yaw -180 pitch 5, wish 0 forced 905, native 32064c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 1804673p/3777b fail 0 ~0 kB/s cache 22459/24576 h1409505 m395168 e372709, srbeat 48772s/0q drop 31stale/10357undec
[14:05:48 INFO]: [c-LoD] stream: tick 13300 65536/65536 cells, lap 8409, cone 153099 zone 21670 dirty 8315 (q 21547) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 9078272c/2106e, edits +18190p/13784c/2842e drop 0, spawned 0 / failed 0, cam0 yaw -180 pitch 5, wish 0 forced 905, native 32064c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 1804673p/3777b fail 0 ~0 kB/s cache 22459/24576 h1409505 m395168 e372709, srbeat 48772s/0q drop 31stale/10357undec
- parity stream max cone: 153099
- parity stream max zone: 21670
- parity stream max wire bundle: bundle
### parity world capture lens (debug chain on the §7b world)
lens_red=0 lens_black=0 lens_water=0 lens_dark=0 lens_zero=0 rot=none scan=0 carriers=0
- parity mesh probe @png_y=2: x306=(151,184,255) x307=(151,184,255) x308=(151,184,255) x309h=(151,184,255) x310c=(151,184,255)
- parity mesh probe @png_y=717: x306=(46,66,53) x307=(59,76,64) x308=(49,70,56) x309h=(47,66,54) x310c=(44,63,51)
- parity mesh engagement: magenta-ish=16 of 921600 sampled px
- mask8: the un-fog engagement masks shot on the PARITY WORLD (vd 8) — parity_dbg8/01m|07m|10m
- mask8 01m: green=352 red=0 blue=96400 dim=5664 (px, sampled /16)
- mask8 07m: green=11056 red=0 blue=221808 dim=5872 (px, sampled /16)
- mask8 10m: green=3456 red=128 blue=222448 dim=160112 (px, sampled /16)
- mesh probe @png_y=2: x306=(255,255,255) x307=(13,254,0) x308=(0,254,0) x309h=(5,0,0) x310c=(0,0,0)
- mesh probe @png_y=717: x306=(255,255,255) x307=(13,254,0) x308=(0,254,0) x309h=(5,0,0) x310c=(0,0,0)
- mesh engagement: magenta-ish=0 of 921600 sampled px (the vanilla-path paint coverage)
- mask8 fog state @png_y=2: raw (300)=(151,184,255) (301)=(151,184,255)
- mask8 fog state @png_y=717: raw (300)=(179,206,255) (301)=(179,206,255)
- mask8 fog state decoded: envStart=1432 envEnd=1648 rdEnd=4080 fogColor=(179,206,255)
- buffer dump 10m: marked h=40/2048 c=0/2048; height windows r0<142:26 r1:1 r2>=202:13 maxByte=255
- buffer dump 10m: RANK-2-WINDOW HEIGHTS (the fake-wall class): (+108,-30)h=187, (+108,-26)h=191, (+110,-16)h=191, (+2,-14)h=159, (+106,-12)h=191, (+2,-10)h=171, (+8,-10)h=171, (+104,-8)h=191, (+106,-6)h=191, (+110,-4)h=191
- buffer dump 10m: color idx0(black)=0 water/height mismatches=0 of 0 pairs
- buffer dump 01m: marked h=0/2048 c=0/2048; height windows r0<142:0 r1:0 r2>=202:0 maxByte=0
- buffer dump 01m: color idx0(black)=0 water/height mismatches=0 of 0 pairs
- rim mask: scan populated (cells=65536; the state paint reads the same buffers the march reads)
- phase: 10 rim mask lane (14:10:28)
- rim mask: decode-state frames shot (rim_mask/00|01|12 — GREEN full decode / RED colorless / BLUE heightless; 00 = the leg-1 tower eye, 01 = the near-tower contrast, 12 = the tower re-read; the inherited 07/12 vantages sat 760 blocks outside the streamed disk and 01 sat INSIDE the marker column — their earlier verdicts were classifier noise, purged in v6.33.1)
- rim mask 00: green=5808 red=85984 blue=86480 yellow=6880 (px, sampled /16) -> DOMINANT BLUE: rim = HEIGHTS NEVER DECODED — a data-coverage class: the state paint cannot even see the cells
- rim mask 01: green=9872 red=136496 blue=140656 yellow=12192 (px, sampled /16) -> DOMINANT BLUE: rim = HEIGHTS NEVER DECODED — a data-coverage class: the state paint cannot even see the cells
- rim mask 12: green=18320 red=114672 blue=115888 yellow=11040 (px, sampled /16) -> DOMINANT BLUE: rim = HEIGHTS NEVER DECODED — a data-coverage class: the state paint cannot even see the cells

### boot / stream
```
[c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 125238 bytes, sha1 2295c3cd8c335c0a9bf4250f8f6e91548b387bcf — zero textures, zero world data, one push per join
[c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[14:09:40 INFO]: [c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 125238 bytes, sha1 2295c3cd8c335c0a9bf4250f8f6e91548b387bcf — zero textures, zero world data, one push per join
[14:09:40 INFO]: [c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[c-LoD] stream: tick 800 65536/65536 cells, lap 629, cone 11591 zone 810 dirty 26 (q 1044) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 428620c/36e, edits +8p/8c/2e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 0 forced 940, native 1088c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 130240p/320b fail 0 ~102 kB/s cache 13703/24576 h116537 m13703 e0, srbeat 3945s/702q drop 0stale/194undec
[14:11:08 INFO]: [c-LoD] stream: tick 800 65536/65536 cells, lap 629, cone 11591 zone 810 dirty 26 (q 1044) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 428620c/36e, edits +8p/8c/2e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 0 forced 940, native 1088c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 130240p/320b fail 0 ~102 kB/s cache 13703/24576 h116537 m13703 e0, srbeat 3945s/702q drop 0stale/194undec
```
### sessions
```
[14:10:03 INFO]: System chat: Stepan joined the game
[14:11:10 INFO]: System chat: Stepan left the game
```
### client log tail
```
[14:11:09] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[14:11:09] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[14:11:09] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[14:11:09] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[14:11:09] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[14:11:09] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[14:11:09] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[14:11:09] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[14:11:09] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[14:11:09] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[14:11:09] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[14:11:09] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[14:11:09] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[14:11:09] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[14:11:09] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[14:11:09] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[14:11:09] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[14:11:09] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[14:11:09] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[14:11:09] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[14:11:09] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[14:11:09] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[14:11:09] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[14:11:09] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:bundle_delimiter] net.minecraft.network.protocol.game.ClientboundBundleDelimiterPacket -> 1 bytes
[14:11:09] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[14:11:09] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[14:11:09] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[14:11:09] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[14:11:09] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[14:11:09] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
```
### server log tail
```
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[14:11:11 INFO]: [ChunkHolderManager] Saved 996 block chunks, 977 entity chunks, 0 poi chunks in world 'minecraft:overworld' in 1.11s
[14:11:11 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:overworld'
[14:11:11 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:overworld'
[14:11:11 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[14:11:11 INFO]: [ChunkHolderManager] Waiting 60s for chunk system to halt for world 'minecraft:the_nether'
[14:11:11 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:the_nether'
[14:11:11 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_nether'
[14:11:11 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_nether' in 0.00s
[14:11:11 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_nether'
[14:11:11 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_nether'
[14:11:11 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[14:11:11 INFO]: [ChunkHolderManager] Waiting 60s for chunk system to halt for world 'minecraft:the_end'
[14:11:11 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:the_end'
[14:11:11 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_end'
[14:11:11 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_end' in 0.00s
[14:11:11 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_end'
[14:11:11 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_end'
[14:11:11 INFO]: Waiting for all RegionFile I/O tasks to complete...
[14:11:11 INFO]: All RegionFile I/O tasks to complete
[14:11:11 INFO]: [MoonriseCommon] Awaiting termination of worker pool for up to 60s...
[14:11:11 INFO]: [MoonriseCommon] Awaiting termination of I/O pool for up to 60s...
[c-LoD] EB: enter, native_gen=true
[launcher] server exited with code 143
```
### shots
```
-rw-r--r-- 1 runner runner   3506 Nov 12 13:33 lens_12_after_edit_probe4.png
-rw-r--r-- 1 runner runner   3823 Nov 12 13:33 lens_12_after_edit_probe5.png
-rw-r--r-- 1 runner runner   3646 Nov 12 13:33 lens_12_after_edit_probe6.png
-rw-r--r-- 1 runner runner   4019 Nov 12 13:33 lens_12_after_edit_probe7.png
-rw-r--r-- 1 runner runner   4087 Nov 12 13:33 lens_12_after_edit_probe8.png
-rw-r--r-- 1 runner runner   3858 Nov 12 13:33 lens_12_after_edit_probe9.png
-rw-r--r-- 1 runner runner   4028 Nov 12 13:41 lens_13_after_relog_probe0.png
-rw-r--r-- 1 runner runner   3859 Nov 12 13:41 lens_13_after_relog_probe1.png
-rw-r--r-- 1 runner runner   4345 Nov 12 13:42 lens_13_after_relog_probe10.png
-rw-r--r-- 1 runner runner   4456 Nov 12 13:43 lens_13_after_relog_probe11.png
-rw-r--r-- 1 runner runner   4300 Nov 12 13:41 lens_13_after_relog_probe2.png
-rw-r--r-- 1 runner runner   3962 Nov 12 13:41 lens_13_after_relog_probe3.png
-rw-r--r-- 1 runner runner   4179 Nov 12 13:42 lens_13_after_relog_probe4.png
-rw-r--r-- 1 runner runner   4278 Nov 12 13:42 lens_13_after_relog_probe5.png
-rw-r--r-- 1 runner runner   4028 Nov 12 13:42 lens_13_after_relog_probe6.png
-rw-r--r-- 1 runner runner   4286 Nov 12 13:42 lens_13_after_relog_probe7.png
-rw-r--r-- 1 runner runner   4077 Nov 12 13:42 lens_13_after_relog_probe8.png
-rw-r--r-- 1 runner runner   3992 Nov 12 13:42 lens_13_after_relog_probe9.png
drwxr-xr-x 2 runner runner   4096 Nov 12 14:05 lens_parity
drwxr-xr-x 2 runner runner   4096 Nov 12 14:05 parity
drwxr-xr-x 2 runner runner   4096 Nov 12 13:38 parity_dbg
drwxr-xr-x 2 runner runner   4096 Nov 12 14:09 parity_dbg8
-rw-r--r-- 1 runner runner   6762 Nov 12 13:43 probe_counts.txt
drwxr-xr-x 2 runner runner   4096 Nov 12 14:11 rim_mask
-rw-r--r-- 1 runner runner  34936 Nov 12 14:11 summary.md
```

**exit=1** failures=2 leg1=0 leg2=0
