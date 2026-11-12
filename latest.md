- phase: 0 sanity (12:31:03)
### c-LoD visual run (-930,796, marker=red_concrete, vantage -1005,150 yaw 270)
- GL:     Vendor: Mesa (0xffffffff);    Device: llvmpipe (LLVM 21.1.8, 256 bits) (0xffffffff);OpenGL renderer string: llvmpipe (LLVM 21.1.8, 256 bits);OpenGL core profile version string: 4.5 (Core Profile) Mesa 26.0.8-1ubuntu0.3;
- boot + gates OK (12:31:34)
- phase: 1 server booted (12:31:34)
- sky-grid armed
- phase: 2 leg-1 client joined (12:32:01)
- joined session 1 (12:32:01)
- stream live: cells=65536
- phase: 3 scan populated (12:32:24)
- terrain range: 2 terraced mountains at (CX-4,CZ-20)/(CX-2,CZ+16), tops 185/190, grass terraces + stone caps, 66..82 m from the eye (inside the 83.5 m band, chunk edge 64 m)
- scan complete=1 (cells=cells=65536/65536)
- paint settled=1 (wire 60559p)
- baseline try 1: drift=0 px
- lod_silhouette_cols=10 (LOD paint ON SCREEN at 75 m if > 20 — the march-drawn marker column; vanilla cannot render there, chunk edge 64 m; #255 re-measures 49, the ZONE_FAR=48 regression #256 and the control read 0)
- 0 terrain-green px in the sky region (LOD MOUNTAINS on screen if > 5000; the range paints ColLow-green on low terraces via the fallback palette; flat-world baselines and the pack-free control read ~0)
- burst_close white px: 0 (renderer alive if > 50)
- burst_far white px: 476 (renderer alive in the marker zone if > 20)
- cut walk 40 m: 0 white px (> 20 = rendered)
- cut walk 60 m: 1 white px (> 20 = rendered)
- cut walk 100 m: 0 white px (> 20 = rendered)
- measured far-render cut: none m (G4 flap instrument; the decode proof needs >= 83 m from the y=151.6 eye)
- burst_close retry white px: 0 (preflight passes if > 50)
- preflight lens_zero=0 (1 = capture NEVER committed = no particle EVER rendered)
- preflight stage eyes: rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=1 mask_hits=30
- fill issued: 5x111x5 red_concrete at -930,796 (zone forceloaded ±48)
- phase: 4 marker filled (12:35:17)

### /clod-cells AFTER FILL (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 36 0 0x8400 0x3100
123 122 -940 784 71 0 1 [97, 119, 97] 36 0 0x8700 0x3100
124 122 -938 784 68 0 1 [97, 119, 97] 35 0 0x8400 0x3100
125 122 -936 784 62 1 1 [63, 118, 228] 35 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 34 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 33 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 33 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 32 0 0x8000 0x5c00
130 122 -926 784 66 0 1 [134, 183, 131] 32 0 0x8200 0x5c00
131 122 -924 784 75 0 1 [97, 119, 97] 31 0 0x8b00 0x3100
132 122 -922 784 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 122 -920 784 75 0 1 [97, 119, 97] 18446744073709551615 1 0x8b00 0x3100
134 122 -918 784 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x3100
122 123 -942 786 70 0 1 [97, 119, 97] 36 0 0x8600 0x3100
123 123 -940 786 65 0 1 [97, 119, 97] 36 0 0x8100 0x3100
124 123 -938 786 65 0 1 [97, 119, 97] 35 0 0x8100 0x3100
125 123 -936 786 62 1 1 [63, 118, 228] 35 0 0x7e00 0xd800
```
### stream diag after fill:
[c-LoD] stream: tick 2100 65536/65536 cells, lap 1003, cone 15528 zone 1522 dirty 889 (q 12663) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 457749c/1755e, edits +5157p/3876c/507e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 297 forced 215, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 143191p/247b fail 0 ~118 kB/s cache 13343/24576 h129848 m13343 e0, srbeat 0s/0q drop 0stale/0undec
[12:35:15 INFO]: [c-LoD] stream: tick 2100 65536/65536 cells, lap 1003, cone 15528 zone 1522 dirty 889 (q 12663) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 457749c/1755e, edits +5157p/3876c/507e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 297 forced 215, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 143191p/247b fail 0 ~118 kB/s cache 13343/24576 h129848 m13343 e0, srbeat 0s/0q drop 0stale/0undec
- probe 0: red=0 paint=8106 lod_silhouette_cols=16 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 1: red=0 paint=8106 lod_silhouette_cols=16 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 2: red=0 paint=8106 lod_silhouette_cols=16 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 3: red=0 paint=8106 lod_silhouette_cols=16 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 4: red=0 paint=8106 lod_silhouette_cols=16 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 5: red=0 paint=9055 lod_silhouette_cols=18 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=1 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 6: red=0 paint=17018 lod_silhouette_cols=24 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 7: red=0 paint=20185 lod_silhouette_cols=24 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 8: red=0 paint=20207 lod_silhouette_cols=24 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 9: red=0 paint=21278 lod_silhouette_cols=25 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 10: red=0 paint=21748 lod_silhouette_cols=24 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 11: red=0 paint=21999 lod_silhouette_cols=24 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=1 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 12: red=0 paint=21976 lod_silhouette_cols=24 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 13: red=0 paint=21780 lod_silhouette_cols=25 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 14: red=0 paint=21780 lod_silhouette_cols=25 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 15: red=0 paint=21780 lod_silhouette_cols=25 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 16: red=0 paint=21780 lod_silhouette_cols=25 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 17: red=0 paint=21780 lod_silhouette_cols=25 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 18: red=0 paint=23753 lod_silhouette_cols=27 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=1 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 19: red=0 paint=23406 lod_silhouette_cols=28 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 20: red=0 paint=23401 lod_silhouette_cols=28 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 21: red=0 paint=25780 lod_silhouette_cols=33 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 22: red=0 paint=25767 lod_silhouette_cols=32 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 23: red=0 paint=25812 lod_silhouette_cols=32 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=1 mask_hits=30 (need red>=60 paint>=300 lod>20)
- phase: 5 leg-1 probes done (12:38:40)
- **FAIL:** leg-1 never saw the marker in 24 probes (red < 60 everywhere) — the edit never reached the LOD paint; this failure was SILENT before v6.31.1
### stream diag after probes:
[c-LoD] stream: tick 4000 65536/65536 cells, lap 1983, cone 40620 zone 1640 dirty 940 (q 12679) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1070549c/1837e, edits +6886p/5113c/905e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 297 forced 215, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 343558p/459b fail 0 ~123 kB/s cache 13444/24576 h330114 m13444 e0, srbeat 0s/0q drop 0stale/0undec
[c-LoD] stream: tick 4100 65536/65536 cells, lap 2083, cone 43248 zone 1640 dirty 940 (q 12679) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1103848c/1837e, edits +6887p/5114c/905e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 297 forced 215, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 363402p/479b fail 0 ~104 kB/s cache 13444/24576 h349958 m13444 e0, srbeat 0s/0q drop 0stale/0undec
[12:38:35 INFO]: [c-LoD] stream: tick 4100 65536/65536 cells, lap 2083, cone 43248 zone 1640 dirty 940 (q 12679) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1103848c/1837e, edits +6887p/5114c/905e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 297 forced 215, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 363402p/479b fail 0 ~104 kB/s cache 13444/24576 h349958 m13444 e0, srbeat 0s/0q drop 0stale/0undec
- parity debug tour: 13 vantages (session-1 client, DEBUG pack — evidence only, the gate pairs the §7b world session)
- parity debug tour done: 13 shots in parity_dbg/ (the §7b world session feeds the gate)
- joined session 2 (12:41:33)
- leg-2 baseline scene px (x<940): 15634 (0 = the relog client paints no world at all)
- leg-2 renderer probe NEAR (20 m) white px: 489 raw / 489 changed-vs-baseline (> 20 = the client renders particles)
- leg-2 renderer probe FAR (80 m, marker zone) white px: 418 raw / 418 changed-vs-baseline (> 20 = no distance cut; ~0 with NEAR alive = particles beyond the client chunk radius do not render)
- leg-2 carrier probe green dust (80 m): changed-green px 0 / total changed 6579 vs baseline (> 100 green = a stable green carrier exists; total > 0 with green ~0 = the dust renders but fog washes it — a ruler problem, not a carrier problem)
- phase: 8 leg-2 relog probes (12:44:26)
- relog probe 0: red=0 scene=15923 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=1 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 1: red=0 scene=19557 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=1 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 2: red=0 scene=21931 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 3: red=0 scene=22069 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 4: red=0 scene=22165 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 5: red=0 scene=31036 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 6: red=0 scene=32867 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 7: red=0 scene=33573 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 8: red=0 scene=34082 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 9: red=0 scene=34567 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 10: red=0 scene=37123 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=1 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 11: red=0 scene=37557 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- **FAIL:** marker lost after relog — stream state is not persistent/deterministic
- phase: 9 leg-2 done (12:46:04)

### client log forensics (pipeline/particle/atlas — v6.8.2)
```
java.lang.IllegalStateException: Failed to open OpenAL device
	at net.minecraft.client.sounds.SoundEngine.reload(SoundEngine.java:110)
	at net.minecraft.server.packs.resources.SimplePreparableReloadListener.lambda$reload$1(SimplePreparableReloadListener.java:15)
	at net.minecraft.server.packs.resources.SimpleReloadInstance.lambda$prepareTasks$3(SimpleReloadInstance.java:59)
[12:41:31] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[12:41:31] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[12:41:31] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[12:41:31] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[12:41:31] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[12:41:31] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[12:41:31] [Render thread/INFO]: Created: 2048x2048x4 minecraft:textures/atlas/blocks.png-atlas
[12:41:31] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[12:41:32] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[12:41:32] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[12:41:32] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[12:41:32] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
[12:41:38] [Render thread/INFO]: Reloading ResourceManager: vanilla, vanilla, server/00000000/6b1f2f7e-9c0d-4a4b-8b0a-b10d1b4ab100
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
[12:41:40] [Render thread/ERROR]: Error starting SoundSystem. Turning off sounds & music
java.lang.IllegalStateException: Failed to open OpenAL device
	at net.minecraft.client.sounds.SoundEngine.reload(SoundEngine.java:110)
	at net.minecraft.server.packs.resources.SimplePreparableReloadListener.lambda$reload$1(SimplePreparableReloadListener.java:15)
	at net.minecraft.server.packs.resources.SimpleReloadInstance.lambda$prepareTasks$3(SimpleReloadInstance.java:59)
[12:41:40] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[12:41:40] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[12:41:40] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[12:41:40] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[12:41:40] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[12:41:40] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[12:41:40] [Render thread/INFO]: Created: 2048x2048x4 minecraft:textures/atlas/blocks.png-atlas
[12:41:41] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[12:41:41] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[12:41:41] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[12:41:41] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[12:41:41] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
```

### /clod-cells AFTER PROBES (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 55 0 0x8400 0x3100
123 122 -940 784 71 0 1 [97, 119, 97] 55 0 0x8700 0x3100
124 122 -938 784 68 0 1 [97, 119, 97] 54 0 0x8400 0x3100
125 122 -936 784 62 1 1 [63, 118, 228] 54 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 53 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 52 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 52 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 51 0 0x8000 0x5c00
130 122 -926 784 66 0 1 [134, 183, 131] 51 0 0x8200 0x5c00
131 122 -924 784 75 0 1 [97, 119, 97] 50 0 0x8b00 0x3100
132 122 -922 784 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 122 -920 784 75 0 1 [97, 119, 97] 18446744073709551615 1 0x8b00 0x3100
134 122 -918 784 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x3100
122 123 -942 786 70 0 1 [97, 119, 97] 55 0 0x8600 0x3100
123 123 -940 786 65 0 1 [97, 119, 97] 55 0 0x8100 0x3100
124 123 -938 786 65 0 1 [97, 119, 97] 54 0 0x8100 0x3100
125 123 -936 786 62 1 1 [63, 118, 228] 54 0 0x7e00 0xd800
```
- edit channel: 0 linkage errors, 0 failed event dispatches (both must be 0)
- phase: 6 parity world boot (12:46:04)
- parity world: view-distance=8 (the voxy leg's server value)
- parity world: joined session 3 (12:46:50)
- parity world: scan populated (cells=65536; the march paints from these buffers)
- parity world tour: 13 vantages (production pack, vd 8, shared structures)
- coverage sweeps: 72 eye passes = 36 positions (12 az x r144/200 + 12 off-az x r172, y=160) x 2 looks (outward+inward), settle-by-laps >= 6 s (q stable 3x1s), deadline 12 s/pos, no shots — the v6.26.0 data-coverage lever + v6.26.6 lottery kill; probe rides parity/view_probe.md
- parity drain-settle TIMEOUT: queue 19677 -> 19672 > 11500 after 150s — continuing anyway; the queue did not drain without eye proximity (priority-lane evidence, the 01/05/10 dip attribution reads from the tour deltas)
- pre-tour pass done: 13 vantages x 10s (the tour now reads the second OR-merge lap; the rim/dip transport race reads from the tour deltas)
- parity world tour done: 13 shots, 0 night-guard renames in parity/ (the gate pairs voxy/<name>.png vs parity/<name>.png)
- phase: 7 parity tour done (13:09:08)
- sparse re-beat channel at tour end: streamed=48435 q=0 dropped_stale=19 dropped_undecodable=10898 (frac=8 boot; undecodable-dominant = the rim words die in their decodability window, stale-dominant = edit churn owns the cells)
### parity world stream counters (session 3, tail + maxima)
[c-LoD] stream: tick 13500 65536/65536 cells, lap 9170, cone 154859 zone 21750 dirty 7813 (q 19631) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 9171366c/1447e, edits +18032p/13670c/2764e drop 0, spawned 0 / failed 0, cam0 yaw -180 pitch 5, wish 0 forced 946, native 29888c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 1791892p/3750b fail 0 ~0 kB/s cache 24109/24576 h1420590 m371302 e347193, srbeat 48435s/0q drop 19stale/10898undec
[13:09:06 INFO]: [c-LoD] stream: tick 13500 65536/65536 cells, lap 9170, cone 154859 zone 21750 dirty 7813 (q 19631) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 9171366c/1447e, edits +18032p/13670c/2764e drop 0, spawned 0 / failed 0, cam0 yaw -180 pitch 5, wish 0 forced 946, native 29888c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 1791892p/3750b fail 0 ~0 kB/s cache 24109/24576 h1420590 m371302 e347193, srbeat 48435s/0q drop 19stale/10898undec
- parity stream max cone: 154859
- parity stream max zone: 21750
- parity stream max wire bundle: bundle
### parity world capture lens (debug chain on the §7b world)
lens_red=0 lens_black=0 lens_water=0 lens_dark=0 lens_zero=0 rot=none scan=0 carriers=0
- parity mesh probe @png_y=2: x306=(151,184,255) x307=(151,184,255) x308=(151,184,255) x309h=(151,184,255) x310c=(151,184,255)
- parity mesh probe @png_y=717: x306=(61,77,67) x307=(62,79,68) x308=(64,80,70) x309h=(55,68,61) x310c=(43,61,51)
- parity mesh engagement: magenta-ish=0 of 921600 sampled px
- mask8: the un-fog engagement masks shot on the PARITY WORLD (vd 8) — parity_dbg8/01m|07m|10m
- mask8 01m: green=144 red=0 blue=94576 dim=6112 (px, sampled /16)
- mask8 07m: green=10816 red=0 blue=220976 dim=5856 (px, sampled /16)
- mask8 10m: green=5952 red=96 blue=226464 dim=142432 (px, sampled /16)
- mesh probe @png_y=2: x306=(255,255,255) x307=(13,254,0) x308=(0,254,0) x309h=(0,0,0) x310c=(0,0,0)
- mesh probe @png_y=717: x306=(255,255,255) x307=(13,254,0) x308=(0,254,0) x309h=(0,0,0) x310c=(0,0,0)
- mesh engagement: magenta-ish=0 of 921600 sampled px (the vanilla-path paint coverage)
- mask8 fog state @png_y=2: raw (300)=(151,184,255) (301)=(151,184,255)
- mask8 fog state @png_y=717: raw (300)=(0,128,8) (301)=(179,206,255)
- mask8 fog state decoded: envStart=0 envEnd=1024 rdEnd=128 fogColor=(179,206,255)
- buffer dump 10m: marked h=26/2048 c=0/2048; height windows r0<142:15 r1:2 r2>=202:9 maxByte=255
- buffer dump 10m: RANK-2-WINDOW HEIGHTS (the fake-wall class): (+104,-26)h=191, (+100,-8)h=191, (+110,-4)h=191, (+104,-2)h=191, (+26,+2)h=149, (+104,+2)h=191, (+26,+18)h=149, (-4,+24)h=140, (+30,+26)h=173
- buffer dump 10m: color idx0(black)=0 water/height mismatches=0 of 0 pairs
- buffer dump 01m: marked h=0/2048 c=0/2048; height windows r0<142:0 r1:0 r2>=202:0 maxByte=0
- buffer dump 01m: color idx0(black)=0 water/height mismatches=0 of 0 pairs
- rim mask: scan populated (cells=65536; the state paint reads the same buffers the march reads)
- phase: 10 rim mask lane (13:15:24)
- rim mask: decode-state frames shot (rim_mask/00|01|12 — GREEN full decode / RED colorless / BLUE heightless; 00 = the leg-1 tower eye, 01 = the near-tower contrast, 12 = the tower re-read; the inherited 07/12 vantages sat 760 blocks outside the streamed disk and 01 sat INSIDE the marker column — their earlier verdicts were classifier noise, purged in v6.33.1)
- rim mask 00: green=16576 red=103296 blue=91312 (px, sampled /16) -> DOMINANT RED: rim = COLORLESS class — the color words die client-side (pool competition / fog strobe margin / the v6.23 HeightRef capture gate); heights ARE decoded
- rim mask 01: green=10384 red=158688 blue=153072 (px, sampled /16) -> DOMINANT RED: rim = COLORLESS class — the color words die client-side (pool competition / fog strobe margin / the v6.23 HeightRef capture gate); heights ARE decoded
- rim mask 12: green=23776 red=130496 blue=119632 (px, sampled /16) -> DOMINANT RED: rim = COLORLESS class — the color words die client-side (pool competition / fog strobe margin / the v6.23 HeightRef capture gate); heights ARE decoded

### boot / stream
```
[c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 124538 bytes, sha1 fe5469942cc8784a45bb58789463397df61e3b4f — zero textures, zero world data, one push per join
[13:12:54 INFO]: [c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 124538 bytes, sha1 fe5469942cc8784a45bb58789463397df61e3b4f — zero textures, zero world data, one push per join
[13:12:54 INFO]: [c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[c-LoD] stream: tick 1800 65536/65536 cells, lap 1630, cone 31490 zone 850 dirty 18 (q 887) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1132473c/8e, edits +24p/24c/10e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 0 forced 942, native 896c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 344417p/867b fail 0 ~93 kB/s cache 13690/24576 h330727 m13690 e0, srbeat 10952s/510q drop 0stale/197undec
[13:16:03 INFO]: [c-LoD] stream: tick 1800 65536/65536 cells, lap 1630, cone 31490 zone 850 dirty 18 (q 887) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1132473c/8e, edits +24p/24c/10e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 0 forced 942, native 896c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 344417p/867b fail 0 ~93 kB/s cache 13690/24576 h330727 m13690 e0, srbeat 10952s/510q drop 0stale/197undec
```
### sessions
```
[13:13:18 INFO]: System chat: Stepan joined the game
[13:16:06 INFO]: System chat: Stepan left the game
```
### client log tail
```
[13:16:05] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[13:16:05] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[13:16:05] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[13:16:05] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[13:16:05] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[13:16:05] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[13:16:05] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[13:16:05] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[13:16:05] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[13:16:05] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[13:16:05] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[13:16:05] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[13:16:05] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[13:16:05] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[13:16:05] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[13:16:05] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[13:16:05] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[13:16:05] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:bundle_delimiter] net.minecraft.network.protocol.game.ClientboundBundleDelimiterPacket -> 1 bytes
[13:16:05] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:move_player_pos] net.minecraft.network.protocol.game.ServerboundMovePlayerPacket$Pos -> 26 bytes
[13:16:05] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[13:16:05] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[13:16:05] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[13:16:05] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[13:16:05] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[13:16:05] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[13:16:05] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[13:16:05] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[13:16:05] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[13:16:05] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[13:16:06] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
```
### server log tail
```
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[13:16:07 INFO]: [ChunkHolderManager] Saved 1001 block chunks, 974 entity chunks, 0 poi chunks in world 'minecraft:overworld' in 0.90s
[13:16:07 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:overworld'
[13:16:07 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:overworld'
[13:16:07 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[13:16:07 INFO]: [ChunkHolderManager] Waiting 60s for chunk system to halt for world 'minecraft:the_nether'
[13:16:07 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:the_nether'
[13:16:07 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_nether'
[13:16:07 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_nether' in 0.00s
[13:16:07 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_nether'
[13:16:07 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_nether'
[13:16:07 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[13:16:07 INFO]: [ChunkHolderManager] Waiting 60s for chunk system to halt for world 'minecraft:the_end'
[13:16:07 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:the_end'
[13:16:07 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_end'
[13:16:07 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_end' in 0.00s
[13:16:07 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_end'
[13:16:07 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_end'
[13:16:07 INFO]: Waiting for all RegionFile I/O tasks to complete...
[13:16:07 INFO]: All RegionFile I/O tasks to complete
[13:16:07 INFO]: [MoonriseCommon] Awaiting termination of worker pool for up to 60s...
[13:16:07 INFO]: [MoonriseCommon] Awaiting termination of I/O pool for up to 60s...
[c-LoD] EB: enter, native_gen=true
[launcher] server exited with code 143
```
### shots
```
-rw-r--r-- 1 runner runner  3908 Nov 12 12:35 lens_12_after_edit_probe4.png
-rw-r--r-- 1 runner runner  4272 Nov 12 12:36 lens_12_after_edit_probe5.png
-rw-r--r-- 1 runner runner  3964 Nov 12 12:36 lens_12_after_edit_probe6.png
-rw-r--r-- 1 runner runner  4322 Nov 12 12:36 lens_12_after_edit_probe7.png
-rw-r--r-- 1 runner runner  4579 Nov 12 12:36 lens_12_after_edit_probe8.png
-rw-r--r-- 1 runner runner  4351 Nov 12 12:36 lens_12_after_edit_probe9.png
-rw-r--r-- 1 runner runner  3286 Nov 12 12:44 lens_13_after_relog_probe0.png
-rw-r--r-- 1 runner runner  3374 Nov 12 12:44 lens_13_after_relog_probe1.png
-rw-r--r-- 1 runner runner  4355 Nov 12 12:45 lens_13_after_relog_probe10.png
-rw-r--r-- 1 runner runner  4020 Nov 12 12:45 lens_13_after_relog_probe11.png
-rw-r--r-- 1 runner runner  3434 Nov 12 12:44 lens_13_after_relog_probe2.png
-rw-r--r-- 1 runner runner  3333 Nov 12 12:44 lens_13_after_relog_probe3.png
-rw-r--r-- 1 runner runner  3712 Nov 12 12:45 lens_13_after_relog_probe4.png
-rw-r--r-- 1 runner runner  3768 Nov 12 12:45 lens_13_after_relog_probe5.png
-rw-r--r-- 1 runner runner  3716 Nov 12 12:45 lens_13_after_relog_probe6.png
-rw-r--r-- 1 runner runner  3996 Nov 12 12:45 lens_13_after_relog_probe7.png
-rw-r--r-- 1 runner runner  3831 Nov 12 12:45 lens_13_after_relog_probe8.png
-rw-r--r-- 1 runner runner  3904 Nov 12 12:45 lens_13_after_relog_probe9.png
drwxr-xr-x 2 runner runner  4096 Nov 12 13:09 lens_parity
drwxr-xr-x 2 runner runner  4096 Nov 12 13:09 parity
drwxr-xr-x 2 runner runner  4096 Nov 12 12:41 parity_dbg
drwxr-xr-x 2 runner runner  4096 Nov 12 13:12 parity_dbg8
-rw-r--r-- 1 runner runner  6765 Nov 12 12:45 probe_counts.txt
drwxr-xr-x 2 runner runner  4096 Nov 12 13:16 rim_mask
-rw-r--r-- 1 runner runner 35304 Nov 12 13:16 summary.md
```

**exit=1** failures=2 leg1=0 leg2=0
