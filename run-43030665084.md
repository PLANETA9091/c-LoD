### c-LoD visual run (-930,796, marker=red_concrete, vantage -1005,150 yaw 270)
- GL:     Vendor: Mesa (0xffffffff);    Device: llvmpipe (LLVM 21.1.8, 256 bits) (0xffffffff);OpenGL renderer string: llvmpipe (LLVM 21.1.8, 256 bits);OpenGL core profile version string: 4.5 (Core Profile) Mesa 26.0.8-1ubuntu0.3;
- boot + gates OK (13:40:42)
- sky-grid armed
- joined session 1 (13:41:02)
- stream live: cells=65536
- terrain range: 2 terraced mountains at (CX-4,CZ-20)/(CX-2,CZ+16), tops 185/190, grass terraces + stone caps, 66..82 m from the eye (inside the 83.5 m band, chunk edge 64 m)
- scan complete=1 (cells=cells=65536/65536)
- paint settled=1 (wire 60491p)
- baseline try 1: drift=396 px
- lod_silhouette_cols=14 (LOD paint ON SCREEN at 75 m if > 20 — the march-drawn marker column; vanilla cannot render there, chunk edge 64 m; #255 re-measures 49, the ZONE_FAR=48 regression #256 and the control read 0)
- 11 terrain-green px in the sky region (LOD MOUNTAINS on screen if > 5000; the range paints ColLow-green on low terraces via the fallback palette; flat-world baselines and the pack-free control read ~0)
- burst_close white px: 489 (renderer alive if > 50)
- burst_far white px: 0 (renderer alive in the marker zone if > 20)
- cut walk 40 m: 67 white px (> 20 = rendered)
- cut walk 60 m: 2 white px (> 20 = rendered)
- cut walk 100 m: 0 white px (> 20 = rendered)
- measured far-render cut: 40 m (G4 flap instrument; the decode proof needs >= 83 m from the y=151.6 eye)
- fill issued: 5x111x5 red_concrete at -930,796 (zone forceloaded ±48)

### /clod-cells AFTER FILL (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 575 0 0x8400 0x3100
123 122 -940 784 71 0 1 [97, 119, 97] 575 0 0x8700 0x3100
124 122 -938 784 68 0 1 [97, 119, 97] 574 0 0x8400 0x3100
125 122 -936 784 62 1 1 [63, 118, 228] 574 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 573 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 572 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 572 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 571 0 0x8000 0x5c00
130 122 -926 784 66 0 1 [134, 183, 131] 571 0 0x8200 0x5c00
131 122 -924 784 75 0 1 [97, 119, 97] 570 0 0x8b00 0x3100
132 122 -922 784 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 122 -920 784 75 0 1 [97, 119, 97] 18446744073709551615 1 0x8b00 0x3100
134 122 -918 784 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x3100
122 123 -942 786 70 0 1 [97, 119, 97] 575 0 0x8600 0x3100
123 123 -940 786 65 0 1 [97, 119, 97] 575 0 0x8100 0x3100
124 123 -938 786 65 0 1 [97, 119, 97] 574 0 0x8100 0x3100
125 123 -936 786 62 1 1 [63, 118, 228] 574 0 0x7e00 0xd800
```
### stream diag after fill:
[c-LoD] stream: tick 1800 65536/65536 cells, lap 683, cone 8833 zone 825 dirty 889 (q 12180) ewin 167 rbeat 9p/1 rearm 0c/0t rescan 396691c/2895e, edits +6099p/4569c/592e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 340 forced 172, native 65536c/1090ch fail 0 (3w q 0) helper on, trees 1090ch/fail 0, wire bundle 84620p/165b fail 0 ~0 kB/s cache 13380/24576 h71240 m13380 e0, srbeat 0s/0q drop 0stale/0undec
[13:43:52 INFO]: [c-LoD] stream: tick 1800 65536/65536 cells, lap 683, cone 8833 zone 825 dirty 889 (q 12180) ewin 167 rbeat 9p/1 rearm 0c/0t rescan 396691c/2895e, edits +6099p/4569c/592e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 340 forced 172, native 65536c/1090ch fail 0 (3w q 0) helper on, trees 1090ch/fail 0, wire bundle 84620p/165b fail 0 ~0 kB/s cache 13380/24576 h71240 m13380 e0, srbeat 0s/0q drop 0stale/0undec
- probe 0: red=0 paint=1866 lod_silhouette_cols=16 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 1: red=0 paint=1866 lod_silhouette_cols=16 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 2: red=0 paint=1866 lod_silhouette_cols=16 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 3: red=0 paint=1866 lod_silhouette_cols=16 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 4: red=0 paint=1866 lod_silhouette_cols=16 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 5: red=0 paint=3816 lod_silhouette_cols=20 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 6: red=0 paint=4477 lod_silhouette_cols=20 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 7: red=0 paint=4227 lod_silhouette_cols=20 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 8: red=0 paint=4227 lod_silhouette_cols=20 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 9: red=0 paint=4227 lod_silhouette_cols=20 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 10: red=0 paint=4227 lod_silhouette_cols=20 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 11: red=0 paint=4227 lod_silhouette_cols=20 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 12: red=0 paint=4741 lod_silhouette_cols=20 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 13: red=0 paint=5664 lod_silhouette_cols=20 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 14: red=0 paint=6223 lod_silhouette_cols=20 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 15: red=0 paint=6223 lod_silhouette_cols=20 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 16: red=0 paint=6223 lod_silhouette_cols=20 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 17: red=0 paint=6223 lod_silhouette_cols=20 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 18: red=0 paint=6223 lod_silhouette_cols=20 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 19: red=0 paint=6223 lod_silhouette_cols=20 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 20: red=0 paint=6428 lod_silhouette_cols=20 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=1 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 21: red=0 paint=6929 lod_silhouette_cols=20 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 22: red=0 paint=7030 lod_silhouette_cols=20 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 23: red=0 paint=7060 lod_silhouette_cols=20 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- **FAIL:** leg-1 never saw the marker in 24 probes (red < 60 everywhere) — the edit never reached the LOD paint; this failure was SILENT before v6.31.1
### stream diag after probes:
[13:46:42 INFO]: [c-LoD] stream: tick 3500 65536/65536 cells, lap 1093, cone 20449 zone 1692 dirty 938 (q 12225) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 949956c/2960e, edits +7637p/5663c/933e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 340 forced 172, native 65536c/1090ch fail 0 (3w q 0) helper on, trees 1090ch/fail 0, wire bundle 184100p/278b fail 0 ~123 kB/s cache 13409/24576 h170691 m13409 e0, srbeat 0s/0q drop 0stale/0undec
[c-LoD] stream: tick 3600 65536/65536 cells, lap 1193, cone 23399 zone 1692 dirty 938 (q 12225) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 978208c/2961e, edits +7640p/5666c/934e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 340 forced 172, native 65536c/1090ch fail 0 (3w q 0) helper on, trees 1090ch/fail 0, wire bundle 207504p/302b fail 0 ~123 kB/s cache 13409/24576 h194095 m13409 e0, srbeat 0s/0q drop 0stale/0undec
[13:46:52 INFO]: [c-LoD] stream: tick 3600 65536/65536 cells, lap 1193, cone 23399 zone 1692 dirty 938 (q 12225) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 978208c/2961e, edits +7640p/5666c/934e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 340 forced 172, native 65536c/1090ch fail 0 (3w q 0) helper on, trees 1090ch/fail 0, wire bundle 207504p/302b fail 0 ~123 kB/s cache 13409/24576 h194095 m13409 e0, srbeat 0s/0q drop 0stale/0undec
- parity debug tour: 13 vantages (session-1 client, DEBUG pack — evidence only, the gate pairs the §7b world session)
- parity debug tour done: 13 shots in parity_dbg/ (the §7b world session feeds the gate)
- joined session 2 (13:49:46)
- leg-2 baseline scene px (x<940): 23914 (0 = the relog client paints no world at all)
- leg-2 renderer probe NEAR (20 m) white px: 536 raw / 536 changed-vs-baseline (> 20 = the client renders particles)
- leg-2 renderer probe FAR (80 m, marker zone) white px: 0 raw / 0 changed-vs-baseline (> 20 = no distance cut; ~0 with NEAR alive = particles beyond the client chunk radius do not render)
- leg-2 carrier probe green dust (80 m): changed-green px 0 / total changed 1885 vs baseline (> 100 green = a stable green carrier exists; total > 0 with green ~0 = the dust renders but fog washes it — a ruler problem, not a carrier problem)
- relog probe 0: red=0 scene=23754 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 1: red=0 scene=23776 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 2: red=0 scene=25841 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 3: red=0 scene=26757 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 4: red=0 scene=26805 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 5: red=0 scene=26837 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 6: red=0 scene=26812 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=1 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 7: red=0 scene=26959 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 8: red=0 scene=27062 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 9: red=0 scene=27086 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 10: red=0 scene=26797 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 11: red=0 scene=30768 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- **FAIL:** marker lost after relog — stream state is not persistent/deterministic

### client log forensics (pipeline/particle/atlas — v6.8.2)
```
java.lang.IllegalStateException: Failed to open OpenAL device
	at net.minecraft.client.sounds.SoundEngine.reload(SoundEngine.java:110)
	at net.minecraft.server.packs.resources.SimplePreparableReloadListener.lambda$reload$1(SimplePreparableReloadListener.java:15)
	at net.minecraft.server.packs.resources.SimpleReloadInstance.lambda$prepareTasks$3(SimpleReloadInstance.java:59)
[13:49:43] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[13:49:43] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[13:49:43] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[13:49:43] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[13:49:43] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[13:49:43] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[13:49:43] [Render thread/INFO]: Created: 2048x2048x4 minecraft:textures/atlas/blocks.png-atlas
[13:49:43] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[13:49:43] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[13:49:43] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[13:49:43] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[13:49:43] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
[13:49:51] [Render thread/INFO]: Reloading ResourceManager: vanilla, vanilla, server/00000000/6b1f2f7e-9c0d-4a4b-8b0a-b10d1b4ab100
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
[13:49:52] [Render thread/ERROR]: Error starting SoundSystem. Turning off sounds & music
java.lang.IllegalStateException: Failed to open OpenAL device
	at net.minecraft.client.sounds.SoundEngine.reload(SoundEngine.java:110)
	at net.minecraft.server.packs.resources.SimplePreparableReloadListener.lambda$reload$1(SimplePreparableReloadListener.java:15)
	at net.minecraft.server.packs.resources.SimpleReloadInstance.lambda$prepareTasks$3(SimpleReloadInstance.java:59)
[13:49:52] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[13:49:52] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[13:49:52] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[13:49:52] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[13:49:52] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[13:49:52] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[13:49:52] [Render thread/INFO]: Created: 2048x2048x4 minecraft:textures/atlas/blocks.png-atlas
[13:49:52] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[13:49:52] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[13:49:52] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[13:49:52] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[13:49:52] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
```

### /clod-cells AFTER PROBES (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 43 0 0x8400 0x3100
123 122 -940 784 71 0 1 [97, 119, 97] 43 0 0x8700 0x3100
124 122 -938 784 68 0 1 [97, 119, 97] 42 0 0x8400 0x3100
125 122 -936 784 62 1 1 [63, 118, 228] 42 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 41 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 40 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 40 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 39 0 0x8000 0x5c00
130 122 -926 784 66 0 1 [134, 183, 131] 39 0 0x8200 0x5c00
131 122 -924 784 75 0 1 [97, 119, 97] 38 0 0x8b00 0x3100
132 122 -922 784 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 122 -920 784 75 0 1 [97, 119, 97] 18446744073709551615 1 0x8b00 0x3100
134 122 -918 784 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x3100
122 123 -942 786 70 0 1 [97, 119, 97] 43 0 0x8600 0x3100
123 123 -940 786 65 0 1 [97, 119, 97] 43 0 0x8100 0x3100
124 123 -938 786 65 0 1 [97, 119, 97] 42 0 0x8100 0x3100
125 123 -936 786 62 1 1 [63, 118, 228] 42 0 0x7e00 0xd800
```
- edit channel: 0 linkage errors, 0 failed event dispatches (both must be 0)
- parity world: view-distance=8 (the voxy leg's server value)
- parity world: joined session 3 (13:54:40)
- parity world: scan populated (cells=65536; the march paints from these buffers)
- parity world tour: 13 vantages (production pack, vd 8, shared structures)
- coverage sweeps: 72 eye passes = 36 positions (12 az x r144/200 + 12 off-az x r172, y=160) x 2 looks (outward+inward), settle-by-laps >= 6 s (q stable 3x1s), deadline 12 s/pos, no shots — the v6.26.0 data-coverage lever + v6.26.6 lottery kill; probe rides parity/view_probe.md
- parity drain-settle TIMEOUT: queue 13776 -> 13771 > 11500 after 150s — continuing anyway; the queue did not drain without eye proximity (priority-lane evidence, the 01/05/10 dip attribution reads from the tour deltas)
- pre-tour pass done: 13 vantages x 10s (the tour now reads the second OR-merge lap; the rim/dip transport race reads from the tour deltas)
- parity world tour done: 13 shots, 0 night-guard renames in parity/ (the gate pairs voxy/<name>.png vs parity/<name>.png)
- sparse re-beat channel at tour end: streamed=52163 q=0 dropped_stale=55 dropped_undecodable=11192 (frac=8 boot; undecodable-dominant = the rim words die in their decodability window, stale-dominant = edit churn owns the cells)
### parity world stream counters (session 3, tail + maxima)
[c-LoD] stream: tick 13000 65536/65536 cells, lap 9183, cone 164064 zone 23706 dirty 6413 (q 13758) ewin 0 rbeat 0p/1 rearm 0c/0t rescan 8958200c/1173e, edits +18116p/13723c/2847e drop 0, spawned 0 / failed 0, cam0 yaw -180 pitch 5, wish 0 forced 865, native 21248c/950ch fail 0 (3w q 0) helper on, trees 950ch/fail 0, wire bundle 1897705p/3939b fail 0 ~0 kB/s cache 24284/24576 h1493444 m404261 e379977, srbeat 52163s/0q drop 55stale/11192undec
[14:16:14 INFO]: [c-LoD] stream: tick 13000 65536/65536 cells, lap 9183, cone 164064 zone 23706 dirty 6413 (q 13758) ewin 0 rbeat 0p/1 rearm 0c/0t rescan 8958200c/1173e, edits +18116p/13723c/2847e drop 0, spawned 0 / failed 0, cam0 yaw -180 pitch 5, wish 0 forced 865, native 21248c/950ch fail 0 (3w q 0) helper on, trees 950ch/fail 0, wire bundle 1897705p/3939b fail 0 ~0 kB/s cache 24284/24576 h1493444 m404261 e379977, srbeat 52163s/0q drop 55stale/11192undec
- parity stream max cone: 164064
- parity stream max zone: 23706
- parity stream max wire bundle: bundle
### parity world capture lens (debug chain on the §7b world)
lens_red=0 lens_black=0 lens_water=0 lens_dark=0 lens_zero=0 rot=none scan=0 carriers=0
- parity mesh probe @png_y=2: x306=(151,184,255) x307=(151,184,255) x308=(151,184,255) x309h=(151,184,255) x310c=(151,184,255)
- parity mesh probe @png_y=717: x306=(56,70,62) x307=(62,77,67) x308=(62,77,67) x309h=(58,70,64) x310c=(66,75,71)
- parity mesh engagement: magenta-ish=0 of 921600 sampled px
- mask8: the un-fog engagement masks shot on the PARITY WORLD (vd 8) — parity_dbg8/01m|07m|10m
- mask8 01m: green=9696 red=80 blue=107824 dim=334544 (px, sampled /16)
- mask8 07m: green=10880 red=0 blue=221920 dim=5888 (px, sampled /16)
- mask8 10m: green=16096 red=2704 blue=217120 dim=234592 (px, sampled /16)
- mesh probe @png_y=2: x306=(255,255,255) x307=(13,254,0) x308=(0,254,0) x309h=(4,0,0) x310c=(0,0,0)
- mesh probe @png_y=717: x306=(255,255,255) x307=(13,254,0) x308=(0,254,0) x309h=(4,0,0) x310c=(0,0,0)
- mesh engagement: magenta-ish=1088 of 921600 sampled px (the vanilla-path paint coverage)
- mask8 fog state @png_y=2: raw (300)=(151,184,255) (301)=(151,184,255)
- mask8 fog state @png_y=717: raw (300)=(29,36,31) (301)=(30,37,32)
- mask8 fog state decoded: envStart=232 envEnd=288 rdEnd=496 fogColor=(30,37,32)
- buffer dump 10m: marked h=34/2048 c=1/2048; height windows r0<142:22 r1:0 r2>=202:12 maxByte=255
- buffer dump 10m: RANK-2-WINDOW HEIGHTS (the fake-wall class): (+34,-32)h=177, (+86,-28)h=191, (+104,-26)h=191, (+2,-10)h=169, (+110,-4)h=191, (+30,+0)h=147, (+100,+0)h=191, (+26,+2)h=149, (+104,+2)h=191, (+20,+18)h=155
- rim mask: scan populated (cells=65536; the state paint reads the same buffers the march reads)
- rim mask: decode-state frames shot (rim_mask/07|01|12 — GREEN full decode / RED colorless / BLUE heightless)
- rim mask 07: green=1200 red=0 blue=416 (px, sampled /16) -> DOMINANT GREEN: rim fully decoded — the fallback is the PAINT path (shade/fog/composite), not data
- rim mask 01: green=416 red=6688 blue=8832 (px, sampled /16) -> DOMINANT BLUE: rim = HEIGHTS NEVER DECODED — a data-coverage class: the state paint cannot even see the cells
- rim mask 12: green=1200 red=0 blue=400 (px, sampled /16) -> DOMINANT GREEN: rim fully decoded — the fallback is the PAINT path (shade/fog/composite), not data

### boot / stream
```
[c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 124538 bytes, sha1 fe5469942cc8784a45bb58789463397df61e3b4f — zero textures, zero world data, one push per join
[c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[14:19:42 INFO]: [c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 124538 bytes, sha1 fe5469942cc8784a45bb58789463397df61e3b4f — zero textures, zero world data, one push per join
[14:19:42 INFO]: [c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[c-LoD] stream: tick 1700 65536/65536 cells, lap 1591, cone 27780 zone 1787 dirty 18 (q 1933) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1080890c/121e, edits +19p/19c/6e drop 0, spawned 0 / failed 0, cam0 yaw -180 pitch 5, wish 382 forced 481, native 22448c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 312535p/700b fail 0 ~0 kB/s cache 24339/24576 h288196 m24339 e0, srbeat 9672s/0q drop 0stale/966undec
[14:22:44 INFO]: [c-LoD] stream: tick 1700 65536/65536 cells, lap 1591, cone 27780 zone 1787 dirty 18 (q 1933) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1080890c/121e, edits +19p/19c/6e drop 0, spawned 0 / failed 0, cam0 yaw -180 pitch 5, wish 382 forced 481, native 22448c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 312535p/700b fail 0 ~0 kB/s cache 24339/24576 h288196 m24339 e0, srbeat 9672s/0q drop 0stale/966undec
```
### sessions
```
[14:20:03 INFO]: System chat: Stepan joined the game
[14:22:45 INFO]: System chat: Stepan left the game
```
### client log tail
```
[14:22:44] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:entity_position_sync] net.minecraft.network.protocol.game.ClientboundEntityPositionSyncPacket -> 37 bytes
[14:22:44] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:entity_position_sync] net.minecraft.network.protocol.game.ClientboundEntityPositionSyncPacket -> 37 bytes
[14:22:44] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[14:22:44] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[14:22:44] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[14:22:44] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[14:22:44] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:entity_position_sync] net.minecraft.network.protocol.game.ClientboundEntityPositionSyncPacket -> 37 bytes
[14:22:44] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:entity_position_sync] net.minecraft.network.protocol.game.ClientboundEntityPositionSyncPacket -> 37 bytes
[14:22:44] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:entity_position_sync] net.minecraft.network.protocol.game.ClientboundEntityPositionSyncPacket -> 37 bytes
[14:22:44] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[14:22:44] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[14:22:45] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[14:22:45] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:entity_position_sync] net.minecraft.network.protocol.game.ClientboundEntityPositionSyncPacket -> 37 bytes
[14:22:45] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:entity_position_sync] net.minecraft.network.protocol.game.ClientboundEntityPositionSyncPacket -> 37 bytes
[14:22:45] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:entity_position_sync] net.minecraft.network.protocol.game.ClientboundEntityPositionSyncPacket -> 37 bytes
[14:22:45] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[14:22:45] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:section_blocks_update] net.minecraft.network.protocol.game.ClientboundSectionBlocksUpdatePacket -> 18 bytes
[14:22:45] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[14:22:45] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[14:22:45] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:set_time] net.minecraft.network.protocol.game.ClientboundSetTimePacket -> 10 bytes
[14:22:45] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:entity_position_sync] net.minecraft.network.protocol.game.ClientboundEntityPositionSyncPacket -> 37 bytes
[14:22:45] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:entity_position_sync] net.minecraft.network.protocol.game.ClientboundEntityPositionSyncPacket -> 37 bytes
[14:22:45] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:entity_position_sync] net.minecraft.network.protocol.game.ClientboundEntityPositionSyncPacket -> 37 bytes
[14:22:45] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[14:22:45] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[14:22:45] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:entity_position_sync] net.minecraft.network.protocol.game.ClientboundEntityPositionSyncPacket -> 37 bytes
[14:22:45] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:entity_position_sync] net.minecraft.network.protocol.game.ClientboundEntityPositionSyncPacket -> 37 bytes
[14:22:45] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:entity_position_sync] net.minecraft.network.protocol.game.ClientboundEntityPositionSyncPacket -> 37 bytes
[14:22:45] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[14:22:45] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
```
### server log tail
```
[14:22:46 INFO]: [ChunkHolderManager] Saved 1441 block chunks, 1415 entity chunks, 0 poi chunks in world 'minecraft:overworld' in 0.72s
[14:22:46 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:overworld'
[14:22:46 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:overworld'
[14:22:46 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[14:22:46 INFO]: [ChunkHolderManager] Waiting 60s for chunk system to halt for world 'minecraft:the_nether'
[14:22:46 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:the_nether'
[14:22:46 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_nether'
[14:22:46 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_nether' in 0.00s
[14:22:46 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_nether'
[14:22:46 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_nether'
[14:22:46 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[14:22:46 INFO]: [ChunkHolderManager] Waiting 60s for chunk system to halt for world 'minecraft:the_end'
[14:22:46 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:the_end'
[14:22:46 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_end'
[14:22:46 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_end' in 0.00s
[14:22:46 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_end'
[14:22:46 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_end'
[14:22:46 INFO]: Waiting for all RegionFile I/O tasks to complete...
[14:22:46 INFO]: All RegionFile I/O tasks to complete
[14:22:46 INFO]: [MoonriseCommon] Awaiting termination of worker pool for up to 60s...
[14:22:46 INFO]: [MoonriseCommon] Awaiting termination of I/O pool for up to 60s...
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[launcher] server exited with code 143
```
### shots
```
-rw-r--r-- 1 runner runner  3557 Nov 11 13:44 lens_12_after_edit_probe4.png
-rw-r--r-- 1 runner runner  3792 Nov 11 13:44 lens_12_after_edit_probe5.png
-rw-r--r-- 1 runner runner  3779 Nov 11 13:44 lens_12_after_edit_probe6.png
-rw-r--r-- 1 runner runner  3735 Nov 11 13:44 lens_12_after_edit_probe7.png
-rw-r--r-- 1 runner runner  3735 Nov 11 13:44 lens_12_after_edit_probe8.png
-rw-r--r-- 1 runner runner  3735 Nov 11 13:45 lens_12_after_edit_probe9.png
-rw-r--r-- 1 runner runner  3440 Nov 11 13:52 lens_13_after_relog_probe0.png
-rw-r--r-- 1 runner runner  3406 Nov 11 13:52 lens_13_after_relog_probe1.png
-rw-r--r-- 1 runner runner  4484 Nov 11 13:53 lens_13_after_relog_probe10.png
-rw-r--r-- 1 runner runner  4569 Nov 11 13:53 lens_13_after_relog_probe11.png
-rw-r--r-- 1 runner runner  4178 Nov 11 13:52 lens_13_after_relog_probe2.png
-rw-r--r-- 1 runner runner  3853 Nov 11 13:52 lens_13_after_relog_probe3.png
-rw-r--r-- 1 runner runner  3746 Nov 11 13:53 lens_13_after_relog_probe4.png
-rw-r--r-- 1 runner runner  3645 Nov 11 13:53 lens_13_after_relog_probe5.png
-rw-r--r-- 1 runner runner  4430 Nov 11 13:53 lens_13_after_relog_probe6.png
-rw-r--r-- 1 runner runner  4121 Nov 11 13:53 lens_13_after_relog_probe7.png
-rw-r--r-- 1 runner runner  4012 Nov 11 13:53 lens_13_after_relog_probe8.png
-rw-r--r-- 1 runner runner  4041 Nov 11 13:53 lens_13_after_relog_probe9.png
drwxr-xr-x 2 runner runner  4096 Nov 11 14:16 lens_parity
drwxr-xr-x 2 runner runner  4096 Nov 11 14:16 parity
drwxr-xr-x 2 runner runner  4096 Nov 11 13:49 parity_dbg
drwxr-xr-x 2 runner runner  4096 Nov 11 14:19 parity_dbg8
-rw-r--r-- 1 runner runner  6734 Nov 11 13:54 probe_counts.txt
drwxr-xr-x 2 runner runner  4096 Nov 11 14:22 rim_mask
-rw-r--r-- 1 runner runner 34079 Nov 11 14:22 summary.md
```

**exit=1** failures=2 leg1=0 leg2=0
