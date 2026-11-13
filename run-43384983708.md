- phase: 0 sanity (07:21:20)
### c-LoD visual run (-930,796, marker=red_concrete, vantage -1005,150 yaw 270)
- GL:     Vendor: Mesa (0xffffffff);    Device: llvmpipe (LLVM 21.1.8, 256 bits) (0xffffffff);OpenGL renderer string: llvmpipe (LLVM 21.1.8, 256 bits);OpenGL core profile version string: 4.5 (Core Profile) Mesa 26.0.8-1ubuntu0.3;
- boot + gates OK (07:21:48)
- phase: 1 server booted (07:21:48)
- sky-grid armed
- phase: 2 leg-1 client joined (07:22:13)
- joined session 1 (07:22:13)
- stream live: cells=65536
- phase: 3 scan populated (07:22:40)
- terrain range: 2 terraced mountains at (CX-4,CZ-20)/(CX-2,CZ+16), tops 185/190, grass terraces + stone caps, 66..82 m from the eye (inside the 83.5 m band, chunk edge 64 m)
- scan complete=1 (cells=cells=65536/65536)
- paint settled=1 (wire 59348p)
- baseline try 1: drift=0 px
- lod_silhouette_cols=10 (LOD paint ON SCREEN at 75 m if > 20 — the march-drawn marker column; vanilla cannot render there, chunk edge 64 m; #255 re-measures 49, the ZONE_FAR=48 regression #256 and the control read 0)
- 1034 terrain-green px in the sky region (LOD MOUNTAINS on screen if > 5000; the range paints ColLow-green on low terraces via the fallback palette; flat-world baselines and the pack-free control read ~0)
- burst_close white px: 485 (renderer alive if > 50)
- burst_far white px: 422 (renderer alive in the marker zone if > 20)
- cut walk 40 m: 0 white px (> 20 = rendered)
- cut walk 60 m: 0 white px (> 20 = rendered)
- cut walk 100 m: 0 white px (> 20 = rendered)
- measured far-render cut: none m (G4 flap instrument; the decode proof needs >= 83 m from the y=151.6 eye)
- fill issued: 5x111x5 red_concrete at -930,796 (zone forceloaded ±48)
- phase: 4 marker filled (07:25:08)

### /clod-cells AFTER FILL (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 1113 0 0x8400 0x3100
123 122 -940 784 71 0 1 [97, 119, 97] 1113 0 0x8700 0x3100
124 122 -938 784 68 0 1 [97, 119, 97] 1113 0 0x8400 0x3100
125 122 -936 784 62 1 1 [63, 118, 228] 1181 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 1179 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 1178 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 1177 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 1177 0 0x8000 0x5c00
130 122 -926 784 66 0 1 [134, 183, 131] 1176 0 0x8200 0x5c00
131 122 -924 784 75 0 1 [97, 119, 97] 1120 0 0x8b00 0x3100
132 122 -922 784 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 122 -920 784 75 0 1 [97, 119, 97] 18446744073709551615 1 0x8b00 0x3100
134 122 -918 784 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x3100
122 123 -942 786 66 0 1 [97, 119, 97] 1113 0 0x8200 0x3100
123 123 -940 786 65 0 1 [97, 119, 97] 1113 0 0x8100 0x3100
124 123 -938 786 65 0 1 [97, 119, 97] 1113 0 0x8100 0x3100
125 123 -936 786 62 1 1 [63, 118, 228] 1181 0 0x7e00 0xd800
```
### stream diag after fill:
[c-LoD] stream: tick 1900 65536/65536 cells, lap 595, cone 3794 zone 768 dirty 896 (q 12354) ewin 243 rbeat 0p/0 rearm 0c/0t rescan 390078c/1404e, edits +5459p/4101c/531e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 299 forced 213, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 59876p/199b fail 0 ~0 kB/s cache 13388/24576 h46488 m13388 e0, srbeat 1970s/0q drop 25stale/0undec
[07:25:08 INFO]: [c-LoD] stream: tick 1900 65536/65536 cells, lap 595, cone 3794 zone 768 dirty 896 (q 12354) ewin 243 rbeat 0p/0 rearm 0c/0t rescan 390078c/1404e, edits +5459p/4101c/531e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 299 forced 213, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 59876p/199b fail 0 ~0 kB/s cache 13388/24576 h46488 m13388 e0, srbeat 1970s/0q drop 25stale/0undec
- probe 0: red=0 paint=0 lod_silhouette_cols=10 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 1: red=0 paint=0 lod_silhouette_cols=10 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 2: red=0 paint=0 lod_silhouette_cols=10 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 3: red=0 paint=0 lod_silhouette_cols=10 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 4: red=0 paint=0 lod_silhouette_cols=10 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 5: red=0 paint=0 lod_silhouette_cols=10 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 6: red=0 paint=0 lod_silhouette_cols=10 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 7: red=0 paint=0 lod_silhouette_cols=10 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 8: red=0 paint=7518 lod_silhouette_cols=10 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 9: red=0 paint=7685 lod_silhouette_cols=10 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 10: red=0 paint=7958 lod_silhouette_cols=10 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 11: red=0 paint=8180 lod_silhouette_cols=10 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 12: red=0 paint=8280 lod_silhouette_cols=10 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 13: red=0 paint=8663 lod_silhouette_cols=10 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 14: red=0 paint=8662 lod_silhouette_cols=10 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 15: red=0 paint=8662 lod_silhouette_cols=10 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 16: red=0 paint=8662 lod_silhouette_cols=10 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 17: red=0 paint=8662 lod_silhouette_cols=10 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 18: red=0 paint=8731 lod_silhouette_cols=10 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 19: red=0 paint=12167 lod_silhouette_cols=10 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 20: red=0 paint=12495 lod_silhouette_cols=10 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 21: red=0 paint=12537 lod_silhouette_cols=10 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 22: red=0 paint=12642 lod_silhouette_cols=10 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 23: red=0 paint=12584 lod_silhouette_cols=10 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- phase: 5 leg-1 probes done (07:28:32)
- **FAIL:** leg-1 never saw the marker in 24 probes (red < 60 everywhere) — the edit never reached the LOD paint; this failure was SILENT before v6.31.1
### stream diag after probes:
[07:28:18 INFO]: [c-LoD] stream: tick 3800 65536/65536 cells, lap 1329, cone 19064 zone 1576 dirty 933 (q 12366) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1006848c/1470e, edits +7080p/5260c/869e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 299 forced 213, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 229309p/602b fail 0 ~123 kB/s cache 13490/24576 h215819 m13490 e0, srbeat 7208s/0q drop 32stale/0undec
[07:28:28 INFO]: [c-LoD] stream: tick 3900 65536/65536 cells, lap 1429, cone 21373 zone 1576 dirty 933 (q 12366) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1036228c/1470e, edits +7080p/5260c/869e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 299 forced 213, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 252675p/666b fail 0 ~123 kB/s cache 13490/24576 h239185 m13490 e0, srbeat 7845s/0q drop 32stale/0undec
[c-LoD] stream: tick 3900 65536/65536 cells, lap 1429, cone 21373 zone 1576 dirty 933 (q 12366) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1036228c/1470e, edits +7080p/5260c/869e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 299 forced 213, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 252675p/666b fail 0 ~123 kB/s cache 13490/24576 h239185 m13490 e0, srbeat 7845s/0q drop 32stale/0undec
- parity debug tour: 13 vantages (session-1 client, DEBUG pack — evidence only, the gate pairs the §7b world session)
- parity debug tour done: 13 shots in parity_dbg/ (the §7b world session feeds the gate)
- joined session 2 (07:31:27)
- leg-2 baseline scene px (x<940): 27612 (0 = the relog client paints no world at all)
- leg-2 renderer probe NEAR (20 m) white px: 576 raw / 576 changed-vs-baseline (> 20 = the client renders particles)
- leg-2 renderer probe FAR (80 m, marker zone) white px: 506 raw / 506 changed-vs-baseline (> 20 = no distance cut; ~0 with NEAR alive = particles beyond the client chunk radius do not render)
- leg-2 carrier probe green dust (80 m): changed-green px 0 / total changed 2966 vs baseline (> 100 green = a stable green carrier exists; total > 0 with green ~0 = the dust renders but fog washes it — a ruler problem, not a carrier problem)
- phase: 8 leg-2 relog probes (07:34:19)
- relog probe 0: red=0 scene=29608 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 1: red=0 scene=33076 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 2: red=0 scene=33064 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 3: red=0 scene=33282 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 4: red=0 scene=33473 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 5: red=0 scene=33846 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 6: red=0 scene=34374 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 7: red=0 scene=34234 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 8: red=0 scene=34450 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 9: red=0 scene=34600 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 10: red=0 scene=38531 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 11: red=0 scene=38802 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- **FAIL:** marker lost after relog — stream state is not persistent/deterministic
- phase: 9 leg-2 done (07:35:57)

### client log forensics (pipeline/particle/atlas — v6.8.2)
```
java.lang.IllegalStateException: Failed to open OpenAL device
	at net.minecraft.client.sounds.SoundEngine.reload(SoundEngine.java:110)
	at net.minecraft.server.packs.resources.SimplePreparableReloadListener.lambda$reload$1(SimplePreparableReloadListener.java:15)
	at net.minecraft.server.packs.resources.SimpleReloadInstance.lambda$prepareTasks$3(SimpleReloadInstance.java:59)
[07:31:24] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[07:31:24] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[07:31:24] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[07:31:24] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[07:31:24] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[07:31:24] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[07:31:24] [Render thread/INFO]: Created: 2048x2048x4 minecraft:textures/atlas/blocks.png-atlas
[07:31:24] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[07:31:24] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[07:31:24] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[07:31:24] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[07:31:24] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
[07:31:31] [Render thread/INFO]: Reloading ResourceManager: vanilla, vanilla, server/00000000/6b1f2f7e-9c0d-4a4b-8b0a-b10d1b4ab100
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
[07:31:33] [Render thread/ERROR]: Error starting SoundSystem. Turning off sounds & music
java.lang.IllegalStateException: Failed to open OpenAL device
	at net.minecraft.client.sounds.SoundEngine.reload(SoundEngine.java:110)
	at net.minecraft.server.packs.resources.SimplePreparableReloadListener.lambda$reload$1(SimplePreparableReloadListener.java:15)
	at net.minecraft.server.packs.resources.SimpleReloadInstance.lambda$prepareTasks$3(SimpleReloadInstance.java:59)
[07:31:33] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[07:31:33] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[07:31:33] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[07:31:33] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[07:31:33] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[07:31:33] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[07:31:33] [Render thread/INFO]: Created: 2048x2048x4 minecraft:textures/atlas/blocks.png-atlas
[07:31:34] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[07:31:34] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[07:31:34] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[07:31:34] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[07:31:34] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
```

### /clod-cells AFTER PROBES (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 5 0 0x8400 0x3100
123 122 -940 784 71 0 1 [97, 119, 97] 14 0 0x8700 0x3100
124 122 -938 784 68 0 1 [97, 119, 97] 14 0 0x8400 0x3100
125 122 -936 784 62 1 1 [63, 118, 228] 56 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 56 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 56 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 55 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 55 0 0x8000 0x5c00
130 122 -926 784 66 0 1 [134, 183, 131] 54 0 0x8200 0x5c00
131 122 -924 784 75 0 1 [97, 119, 97] 0 0 0x8b00 0x3100
132 122 -922 784 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 122 -920 784 75 0 1 [97, 119, 97] 18446744073709551615 1 0x8b00 0x3100
134 122 -918 784 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x3100
122 123 -942 786 66 0 1 [97, 119, 97] 5 0 0x8200 0x3100
123 123 -940 786 65 0 1 [97, 119, 97] 13 0 0x8100 0x3100
124 123 -938 786 65 0 1 [97, 119, 97] 14 0 0x8100 0x3100
125 123 -936 786 62 1 1 [63, 118, 228] 56 0 0x7e00 0xd800
```
- edit channel: 0 linkage errors, 0 failed event dispatches (both must be 0)
- phase: 6 parity world boot (07:35:57)
- parity world: view-distance=8 (the voxy leg's server value)
- parity world: joined session 3 (07:36:43)
- parity world: scan populated (cells=65536; the march paints from these buffers)
- parity world tour: 13 vantages (production pack, vd 8, shared structures)
- coverage sweeps: 72 eye passes = 36 positions (12 az x r144/200 + 12 off-az x r172, y=160) x 2 looks (outward+inward), settle-by-laps >= 6 s (q stable 3x1s), deadline 12 s/pos, no shots — the v6.26.0 data-coverage lever + v6.26.6 lottery kill; probe rides parity/view_probe.md
- parity drain-settle TIMEOUT: queue 27877 -> 27855 > 11500 after 150s — continuing anyway; the queue did not drain without eye proximity (priority-lane evidence, the 01/05/10 dip attribution reads from the tour deltas)
- pre-tour pass done: 13 vantages x 10s (the tour now reads the second OR-merge lap; the rim/dip transport race reads from the tour deltas)
- parity world tour done: 13 shots, 0 night-guard renames in parity/ (the gate pairs voxy/<name>.png vs parity/<name>.png)
- phase: 7 parity tour done (07:58:40)
- sparse re-beat channel at tour end: streamed=45714 q=0 dropped_stale=14 dropped_undecodable=8950 (frac=8 boot; undecodable-dominant = the rim words die in their decodability window, stale-dominant = edit churn owns the cells)
### parity world stream counters (session 3, tail + maxima)
[c-LoD] stream: tick 13300 65536/65536 cells, lap 7994, cone 146636 zone 19660 dirty 8679 (q 27829) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 9052422c/1718e, edits +18072p/13679c/2767e drop 0, spawned 0 / failed 0, cam0 yaw -180 pitch 5, wish 0 forced 917, native 37416c/957ch fail 0 (3w q 0) helper on, trees 957ch/fail 0, wire bundle 1726661p/3627b fail 0 ~0 kB/s cache 24508/24576 h1342803 m383858 e359350, srbeat 45714s/0q drop 14stale/8950undec
[07:58:39 INFO]: [c-LoD] stream: tick 13300 65536/65536 cells, lap 7994, cone 146636 zone 19660 dirty 8679 (q 27829) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 9052422c/1718e, edits +18072p/13679c/2767e drop 0, spawned 0 / failed 0, cam0 yaw -180 pitch 5, wish 0 forced 917, native 37416c/957ch fail 0 (3w q 0) helper on, trees 957ch/fail 0, wire bundle 1726661p/3627b fail 0 ~0 kB/s cache 24508/24576 h1342803 m383858 e359350, srbeat 45714s/0q drop 14stale/8950undec
- parity stream max cone: 146636
- parity stream max zone: 19660
- parity stream max wire bundle: bundle
### parity world capture lens (debug chain on the §7b world)
lens_red=0 lens_black=0 lens_water=0 lens_dark=0 lens_zero=0 rot=none scan=0 carriers=0
- parity mesh probe @png_y=2: x306=(151,184,255) x307=(151,184,255) x308=(151,184,255) x309h=(151,184,255) x310c=(151,184,255)
- parity mesh probe @png_y=717: x306=(44,62,51) x307=(65,89,72) x308=(81,107,87) x309h=(82,108,87) x310c=(84,111,90)
- parity mesh engagement: magenta-ish=0 of 921600 sampled px
- mask8: the un-fog engagement masks shot on the PARITY WORLD (vd 8) — parity_dbg8/01m|07m|10m
- mask8 01m: green=5536 red=0 blue=98496 dim=10176 (px, sampled /16)
- mask8 07m: green=10896 red=0 blue=221904 dim=5888 (px, sampled /16)
- mask8 10m: green=4960 red=96 blue=226176 dim=150832 (px, sampled /16)
- mesh probe @png_y=2: x306=(255,255,255) x307=(13,254,0) x308=(0,254,0) x309h=(4,0,0) x310c=(0,0,0)
- mesh probe @png_y=717: x306=(255,255,255) x307=(13,254,0) x308=(0,254,0) x309h=(4,0,0) x310c=(0,0,0)
- mesh engagement: magenta-ish=0 of 921600 sampled px (the vanilla-path paint coverage)
- mask8 fog state @png_y=2: raw (300)=(151,184,255) (301)=(151,184,255)
- mask8 fog state @png_y=717: raw (300)=(0,128,8) (301)=(179,206,255)
- mask8 fog state decoded: envStart=0 envEnd=1024 rdEnd=128 fogColor=(179,206,255)
- buffer dump 10m: marked h=24/2048 c=0/2048; height windows r0<142:19 r1:0 r2>=202:5 maxByte=255
- buffer dump 10m: RANK-2-WINDOW HEIGHTS (the fake-wall class): (+2,-10)h=145, (+100,-8)h=191, (+100,-6)h=191, (+26,+2)h=149, (+2,+18)h=159
- buffer dump 10m: color idx0(black)=0 water/height mismatches=0 of 0 pairs
- buffer dump 01m: marked h=0/2048 c=0/2048; height windows r0<142:0 r1:0 r2>=202:0 maxByte=0
- buffer dump 01m: color idx0(black)=0 water/height mismatches=0 of 0 pairs
- rim mask: scan populated (cells=65536; the state paint reads the same buffers the march reads)
- phase: 10 rim mask lane (08:03:17)
- rim mask: decode-state frames shot (rim_mask/00|01|12 — GREEN full decode / RED colorless / BLUE heightless; 00 = the leg-1 tower eye, 01 = the near-tower contrast, 12 = the tower re-read; the inherited 07/12 vantages sat 760 blocks outside the streamed disk and 01 sat INSIDE the marker column — their earlier verdicts were classifier noise, purged in v6.33.1)
- rim mask 00: green=1232 red=88288 blue=83728 yellow=2480 (px, sampled /16) -> DOMINANT RED: rim = COLORLESS class — the color words die client-side (pool competition / fog strobe margin / the v6.23 HeightRef capture gate); heights ARE decoded
- rim mask 01: green=6688 red=139792 blue=132112 yellow=7616 (px, sampled /16) -> DOMINANT RED: rim = COLORLESS class — the color words die client-side (pool competition / fog strobe margin / the v6.23 HeightRef capture gate); heights ARE decoded
- rim mask 12: green=7552 red=115056 blue=113136 yellow=12864 (px, sampled /16) -> DOMINANT RED: rim = COLORLESS class — the color words die client-side (pool competition / fog strobe margin / the v6.23 HeightRef capture gate); heights ARE decoded

### boot / stream
```
[c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 127551 bytes, sha1 9044d9e9174ffbeb08caf7427c6d82dfc20eec0c — zero textures, zero world data, one push per join
[c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[08:02:27 INFO]: [c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 127551 bytes, sha1 9044d9e9174ffbeb08caf7427c6d82dfc20eec0c — zero textures, zero world data, one push per join
[08:02:27 INFO]: [c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[c-LoD] stream: tick 800 65536/65536 cells, lap 627, cone 11195 zone 810 dirty 0 (q 4157) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 440637c/531e, edits +7p/7c/2e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 0 forced 932, native 4544c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 126565p/330b fail 0 ~110 kB/s cache 13576/24576 h112989 m13576 e0, srbeat 3934s/351q drop 0stale/195undec
[08:03:55 INFO]: [c-LoD] stream: tick 800 65536/65536 cells, lap 627, cone 11195 zone 810 dirty 0 (q 4157) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 440637c/531e, edits +7p/7c/2e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 0 forced 932, native 4544c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 126565p/330b fail 0 ~110 kB/s cache 13576/24576 h112989 m13576 e0, srbeat 3934s/351q drop 0stale/195undec
```
### sessions
```
[08:02:50 INFO]: System chat: Stepan joined the game
[08:03:58 INFO]: System chat: Stepan left the game
```
### client log tail
```
[08:03:58] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[08:03:58] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[08:03:58] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[08:03:58] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[08:03:58] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[08:03:58] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[08:03:58] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[08:03:58] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[08:03:58] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[08:03:58] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[08:03:58] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[08:03:58] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[08:03:58] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[08:03:58] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[08:03:58] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[08:03:58] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[08:03:58] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[08:03:58] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[08:03:58] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:bundle_delimiter] net.minecraft.network.protocol.game.ClientboundBundleDelimiterPacket -> 1 bytes
[08:03:58] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:move_player_pos] net.minecraft.network.protocol.game.ServerboundMovePlayerPacket$Pos -> 26 bytes
[08:03:58] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[08:03:58] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[08:03:58] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[08:03:58] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[08:03:58] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[08:03:58] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[08:03:58] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[08:03:58] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[08:03:58] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[08:03:58] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
```
### server log tail
```
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[08:03:59 INFO]: [ChunkHolderManager] Saved 997 block chunks, 980 entity chunks, 0 poi chunks in world 'minecraft:overworld' in 0.88s
[08:03:59 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:overworld'
[08:03:59 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:overworld'
[08:03:59 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[08:03:59 INFO]: [ChunkHolderManager] Waiting 60s for chunk system to halt for world 'minecraft:the_nether'
[08:03:59 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:the_nether'
[08:03:59 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_nether'
[08:03:59 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_nether' in 0.00s
[08:03:59 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_nether'
[08:03:59 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_nether'
[08:03:59 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[08:03:59 INFO]: [ChunkHolderManager] Waiting 60s for chunk system to halt for world 'minecraft:the_end'
[08:03:59 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:the_end'
[08:03:59 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_end'
[08:03:59 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_end' in 0.00s
[08:03:59 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_end'
[08:03:59 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_end'
[08:03:59 INFO]: Waiting for all RegionFile I/O tasks to complete...
[08:03:59 INFO]: All RegionFile I/O tasks to complete
[08:03:59 INFO]: [MoonriseCommon] Awaiting termination of worker pool for up to 60s...
[08:03:59 INFO]: [MoonriseCommon] Awaiting termination of I/O pool for up to 60s...
[c-LoD] EB: enter, native_gen=true
[launcher] server exited with code 143
```
### shots
```
-rw-r--r-- 1 runner runner  5571 Nov 13 07:25 lens_12_after_edit_probe4.png
-rw-r--r-- 1 runner runner  5571 Nov 13 07:25 lens_12_after_edit_probe5.png
-rw-r--r-- 1 runner runner  5571 Nov 13 07:26 lens_12_after_edit_probe6.png
-rw-r--r-- 1 runner runner  5571 Nov 13 07:26 lens_12_after_edit_probe7.png
-rw-r--r-- 1 runner runner  6088 Nov 13 07:26 lens_12_after_edit_probe8.png
-rw-r--r-- 1 runner runner  5760 Nov 13 07:26 lens_12_after_edit_probe9.png
-rw-r--r-- 1 runner runner  9194 Nov 13 07:34 lens_13_after_relog_probe0.png
-rw-r--r-- 1 runner runner  9114 Nov 13 07:34 lens_13_after_relog_probe1.png
-rw-r--r-- 1 runner runner  9876 Nov 13 07:35 lens_13_after_relog_probe10.png
-rw-r--r-- 1 runner runner  9090 Nov 13 07:35 lens_13_after_relog_probe11.png
-rw-r--r-- 1 runner runner  9330 Nov 13 07:34 lens_13_after_relog_probe2.png
-rw-r--r-- 1 runner runner  9296 Nov 13 07:34 lens_13_after_relog_probe3.png
-rw-r--r-- 1 runner runner  9417 Nov 13 07:34 lens_13_after_relog_probe4.png
-rw-r--r-- 1 runner runner  9285 Nov 13 07:35 lens_13_after_relog_probe5.png
-rw-r--r-- 1 runner runner  8959 Nov 13 07:35 lens_13_after_relog_probe6.png
-rw-r--r-- 1 runner runner  9229 Nov 13 07:35 lens_13_after_relog_probe7.png
-rw-r--r-- 1 runner runner  9751 Nov 13 07:35 lens_13_after_relog_probe8.png
-rw-r--r-- 1 runner runner  9444 Nov 13 07:35 lens_13_after_relog_probe9.png
drwxr-xr-x 2 runner runner  4096 Nov 13 07:58 lens_parity
drwxr-xr-x 2 runner runner  4096 Nov 13 07:58 parity
drwxr-xr-x 2 runner runner  4096 Nov 13 07:31 parity_dbg
drwxr-xr-x 2 runner runner  4096 Nov 13 08:02 parity_dbg8
-rw-r--r-- 1 runner runner  6738 Nov 13 07:35 probe_counts.txt
drwxr-xr-x 2 runner runner  4096 Nov 13 08:03 rim_mask
-rw-r--r-- 1 runner runner 35057 Nov 13 08:04 summary.md
```

**exit=1** failures=2 leg1=0 leg2=0
