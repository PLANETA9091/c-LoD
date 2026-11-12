- phase: 0 sanity (15:58:41)
### c-LoD visual run (-930,796, marker=red_concrete, vantage -1005,150 yaw 270)
- GL:     Vendor: Mesa (0xffffffff);    Device: llvmpipe (LLVM 21.1.8, 256 bits) (0xffffffff);OpenGL renderer string: llvmpipe (LLVM 21.1.8, 256 bits);OpenGL core profile version string: 4.5 (Core Profile) Mesa 26.0.8-1ubuntu0.3;
- boot + gates OK (15:59:01)
- phase: 1 server booted (15:59:02)
- sky-grid armed
- phase: 2 leg-1 client joined (15:59:25)
- joined session 1 (15:59:25)
- stream live: cells=65536
- phase: 3 scan populated (15:59:47)
- terrain range: 2 terraced mountains at (CX-4,CZ-20)/(CX-2,CZ+16), tops 185/190, grass terraces + stone caps, 66..82 m from the eye (inside the 83.5 m band, chunk edge 64 m)
- scan complete=1 (cells=cells=65536/65536)
- paint settled=1 (wire 60202p)
- baseline try 1: drift=0 px
- lod_silhouette_cols=16 (LOD paint ON SCREEN at 75 m if > 20 — the march-drawn marker column; vanilla cannot render there, chunk edge 64 m; #255 re-measures 49, the ZONE_FAR=48 regression #256 and the control read 0)
- 0 terrain-green px in the sky region (LOD MOUNTAINS on screen if > 5000; the range paints ColLow-green on low terraces via the fallback palette; flat-world baselines and the pack-free control read ~0)
- burst_close white px: 527 (renderer alive if > 50)
- burst_far white px: 133 (renderer alive in the marker zone if > 20)
- cut walk 40 m: 76 white px (> 20 = rendered)
- cut walk 60 m: 0 white px (> 20 = rendered)
- cut walk 100 m: 0 white px (> 20 = rendered)
- measured far-render cut: 40 m (G4 flap instrument; the decode proof needs >= 83 m from the y=151.6 eye)
- fill issued: 5x111x5 red_concrete at -930,796 (zone forceloaded ±48)
- phase: 4 marker filled (16:02:15)

### /clod-cells AFTER FILL (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 19 0 0x8400 0x3100
123 122 -940 784 71 0 1 [97, 119, 97] 19 0 0x8700 0x3100
124 122 -938 784 68 0 1 [97, 119, 97] 18 0 0x8400 0x3100
125 122 -936 784 62 1 1 [63, 118, 228] 18 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 17 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 17 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 16 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 15 0 0x8000 0x5c00
130 122 -926 784 66 0 1 [134, 183, 131] 15 0 0x8200 0x5c00
131 122 -924 784 75 0 1 [97, 119, 97] 14 0 0x8b00 0x3100
132 122 -922 784 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 122 -920 784 75 0 1 [97, 119, 97] 18446744073709551615 1 0x8b00 0x3100
134 122 -918 784 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x3100
122 123 -942 786 70 0 1 [97, 119, 97] 19 0 0x8600 0x3100
123 123 -940 786 65 0 1 [97, 119, 97] 19 0 0x8100 0x3100
124 123 -938 786 65 0 1 [97, 119, 97] 18 0 0x8100 0x3100
125 123 -936 786 62 1 1 [63, 118, 228] 18 0 0x7e00 0xd800
```
### stream diag after fill:
[c-LoD] stream: tick 1900 65536/65536 cells, lap 752, cone 9592 zone 1509 dirty 873 (q 11070) ewin 178 rbeat 9p/1 rearm 0c/0t rescan 419974c/2787e, edits +6264p/4733c/685e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 343 forced 169, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 95885p/181b fail 0 ~0 kB/s cache 13326/24576 h82559 m13326 e0, srbeat 0s/0q drop 0stale/0undec
[16:02:21 INFO]: [c-LoD] stream: tick 1900 65536/65536 cells, lap 752, cone 9592 zone 1509 dirty 873 (q 11070) ewin 178 rbeat 9p/1 rearm 0c/0t rescan 419974c/2787e, edits +6264p/4733c/685e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 343 forced 169, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 95885p/181b fail 0 ~0 kB/s cache 13326/24576 h82559 m13326 e0, srbeat 0s/0q drop 0stale/0undec
- probe 0: red=0 paint=4426 lod_silhouette_cols=20 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 1: red=0 paint=4426 lod_silhouette_cols=20 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 2: red=0 paint=4426 lod_silhouette_cols=20 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 3: red=0 paint=4426 lod_silhouette_cols=20 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 4: red=0 paint=4426 lod_silhouette_cols=20 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 5: red=0 paint=4426 lod_silhouette_cols=20 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 6: red=0 paint=4426 lod_silhouette_cols=20 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 7: red=0 paint=4426 lod_silhouette_cols=20 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 8: red=0 paint=4426 lod_silhouette_cols=20 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 9: red=0 paint=4426 lod_silhouette_cols=20 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 10: red=0 paint=4426 lod_silhouette_cols=20 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 11: red=0 paint=4426 lod_silhouette_cols=20 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 12: red=0 paint=4426 lod_silhouette_cols=20 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 13: red=0 paint=4426 lod_silhouette_cols=20 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 14: red=0 paint=4426 lod_silhouette_cols=20 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 15: red=0 paint=4426 lod_silhouette_cols=20 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 16: red=0 paint=4426 lod_silhouette_cols=20 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 17: red=0 paint=4426 lod_silhouette_cols=20 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 18: red=0 paint=4426 lod_silhouette_cols=20 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 19: red=0 paint=4426 lod_silhouette_cols=20 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 20: red=0 paint=4426 lod_silhouette_cols=20 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 21: red=0 paint=4426 lod_silhouette_cols=20 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 22: red=0 paint=4426 lod_silhouette_cols=20 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 23: red=0 paint=4598 lod_silhouette_cols=20 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- phase: 5 leg-1 probes done (16:05:28)
- **FAIL:** leg-1 never saw the marker in 24 probes (red < 60 everywhere) — the edit never reached the LOD paint; this failure was SILENT before v6.31.1
### stream diag after probes:
[16:05:12 INFO]: [c-LoD] stream: tick 3600 65536/65536 cells, lap 752, cone 9592 zone 1509 dirty 917 (q 11100) ewin 58 rbeat 0p/0 rearm 0c/0t rescan 965513c/2846e, edits +7552p/5618c/938e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 343 forced 169, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 96281p/192b fail 0 ~0 kB/s cache 13348/24576 h82933 m13348 e0, srbeat 0s/0q drop 0stale/0undec
[c-LoD] stream: tick 3700 65536/65536 cells, lap 791, cone 10373 zone 2249 dirty 917 (q 11100) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1001470c/2846e, edits +7552p/5618c/938e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 343 forced 169, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 108346p/205b fail 0 ~63 kB/s cache 13348/24576 h94998 m13348 e0, srbeat 0s/0q drop 0stale/0undec
[16:05:22 INFO]: [c-LoD] stream: tick 3700 65536/65536 cells, lap 791, cone 10373 zone 2249 dirty 917 (q 11100) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1001470c/2846e, edits +7552p/5618c/938e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 343 forced 169, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 108346p/205b fail 0 ~63 kB/s cache 13348/24576 h94998 m13348 e0, srbeat 0s/0q drop 0stale/0undec
- parity debug tour: 13 vantages (session-1 client, DEBUG pack — evidence only, the gate pairs the §7b world session)
- parity debug tour done: 13 shots in parity_dbg/ (the §7b world session feeds the gate)
- joined session 2 (16:08:19)
- leg-2 baseline scene px (x<940): 20765 (0 = the relog client paints no world at all)
- leg-2 renderer probe NEAR (20 m) white px: 559 raw / 559 changed-vs-baseline (> 20 = the client renders particles)
- leg-2 renderer probe FAR (80 m, marker zone) white px: 344 raw / 344 changed-vs-baseline (> 20 = no distance cut; ~0 with NEAR alive = particles beyond the client chunk radius do not render)
- leg-2 carrier probe green dust (80 m): changed-green px 0 / total changed 9270 vs baseline (> 100 green = a stable green carrier exists; total > 0 with green ~0 = the dust renders but fog washes it — a ruler problem, not a carrier problem)
- phase: 8 leg-2 relog probes (16:11:09)
- relog probe 0: red=0 scene=26554 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 1: red=0 scene=27931 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 2: red=0 scene=28382 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 3: red=0 scene=28531 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 4: red=0 scene=28860 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 5: red=0 scene=33401 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 6: red=0 scene=33683 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 7: red=0 scene=33735 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 8: red=0 scene=33563 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 9: red=0 scene=33825 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 10: red=0 scene=36679 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 11: red=0 scene=39411 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- **FAIL:** marker lost after relog — stream state is not persistent/deterministic
- phase: 9 leg-2 done (16:12:42)

### client log forensics (pipeline/particle/atlas — v6.8.2)
```
java.lang.IllegalStateException: Failed to open OpenAL device
	at net.minecraft.client.sounds.SoundEngine.reload(SoundEngine.java:110)
	at net.minecraft.server.packs.resources.SimplePreparableReloadListener.lambda$reload$1(SimplePreparableReloadListener.java:15)
	at net.minecraft.server.packs.resources.SimpleReloadInstance.lambda$prepareTasks$3(SimpleReloadInstance.java:59)
[16:08:16] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[16:08:16] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[16:08:16] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[16:08:16] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[16:08:16] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[16:08:16] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[16:08:16] [Render thread/INFO]: Created: 2048x2048x4 minecraft:textures/atlas/blocks.png-atlas
[16:08:17] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[16:08:17] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[16:08:17] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[16:08:17] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[16:08:17] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
[16:08:24] [Render thread/INFO]: Reloading ResourceManager: vanilla, vanilla, server/00000000/6b1f2f7e-9c0d-4a4b-8b0a-b10d1b4ab100
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
[16:08:26] [Render thread/ERROR]: Error starting SoundSystem. Turning off sounds & music
java.lang.IllegalStateException: Failed to open OpenAL device
	at net.minecraft.client.sounds.SoundEngine.reload(SoundEngine.java:110)
	at net.minecraft.server.packs.resources.SimplePreparableReloadListener.lambda$reload$1(SimplePreparableReloadListener.java:15)
	at net.minecraft.server.packs.resources.SimpleReloadInstance.lambda$prepareTasks$3(SimpleReloadInstance.java:59)
[16:08:26] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[16:08:26] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[16:08:26] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[16:08:26] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[16:08:26] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[16:08:26] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[16:08:26] [Render thread/INFO]: Created: 2048x2048x4 minecraft:textures/atlas/blocks.png-atlas
[16:08:26] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[16:08:26] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[16:08:26] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[16:08:26] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[16:08:26] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
```

### /clod-cells AFTER PROBES (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 59 0 0x8400 0x3100
123 122 -940 784 71 0 1 [97, 119, 97] 59 0 0x8700 0x3100
124 122 -938 784 68 0 1 [97, 119, 97] 58 0 0x8400 0x3100
125 122 -936 784 62 1 1 [63, 118, 228] 58 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 57 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 57 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 56 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 55 0 0x8000 0x5c00
130 122 -926 784 66 0 1 [134, 183, 131] 55 0 0x8200 0x5c00
131 122 -924 784 75 0 1 [97, 119, 97] 54 0 0x8b00 0x3100
132 122 -922 784 77 0 1 [97, 119, 97] 3468 0 0x8d00 0x3100
133 122 -920 784 75 0 1 [97, 119, 97] 3468 0 0x8b00 0x3100
134 122 -918 784 73 0 1 [97, 119, 97] 3468 0 0x8900 0x3100
122 123 -942 786 70 0 1 [97, 119, 97] 59 0 0x8600 0x3100
123 123 -940 786 65 0 1 [97, 119, 97] 59 0 0x8100 0x3100
124 123 -938 786 65 0 1 [97, 119, 97] 58 0 0x8100 0x3100
125 123 -936 786 62 1 1 [63, 118, 228] 58 0 0x7e00 0xd800
```
- edit channel: 0 linkage errors, 0 failed event dispatches (both must be 0)
- phase: 6 parity world boot (16:12:43)
- parity world: view-distance=8 (the voxy leg's server value)
- parity world: joined session 3 (16:13:20)
- parity world: scan populated (cells=65536; the march paints from these buffers)
- parity world tour: 13 vantages (production pack, vd 8, shared structures)
- coverage sweeps: 72 eye passes = 36 positions (12 az x r144/200 + 12 off-az x r172, y=160) x 2 looks (outward+inward), settle-by-laps >= 6 s (q stable 3x1s), deadline 12 s/pos, no shots — the v6.26.0 data-coverage lever + v6.26.6 lottery kill; probe rides parity/view_probe.md
- parity drain-settle TIMEOUT: queue 24657 -> 24657 > 11500 after 150s — continuing anyway; the queue did not drain without eye proximity (priority-lane evidence, the 01/05/10 dip attribution reads from the tour deltas)
- pre-tour pass done: 13 vantages x 10s (the tour now reads the second OR-merge lap; the rim/dip transport race reads from the tour deltas)
- parity world tour done: 13 shots, 0 night-guard renames in parity/ (the gate pairs voxy/<name>.png vs parity/<name>.png)
- phase: 7 parity tour done (16:35:09)
- sparse re-beat channel at tour end: streamed=45794 q=0 dropped_stale=18 dropped_undecodable=7716 (frac=8 boot; undecodable-dominant = the rim words die in their decodability window, stale-dominant = edit churn owns the cells)
### parity world stream counters (session 3, tail + maxima)
[c-LoD] stream: tick 13100 65536/65536 cells, lap 8438, cone 139044 zone 22426 dirty 9287 (q 24636) ewin 0 rbeat 0p/1 rearm 0c/0t rescan 8983389c/1765e, edits +18056p/13690c/2890e drop 0, spawned 0 / failed 0, cam0 yaw -180 pitch 5, wish 0 forced 892, native 35968c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 1663046p/3385b fail 0 ~0 kB/s cache 24238/24576 h1272561 m390485 e366247, srbeat 45794s/0q drop 18stale/7716undec
[16:35:02 INFO]: [c-LoD] stream: tick 13100 65536/65536 cells, lap 8438, cone 139044 zone 22426 dirty 9287 (q 24636) ewin 0 rbeat 0p/1 rearm 0c/0t rescan 8983389c/1765e, edits +18056p/13690c/2890e drop 0, spawned 0 / failed 0, cam0 yaw -180 pitch 5, wish 0 forced 892, native 35968c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 1663046p/3385b fail 0 ~0 kB/s cache 24238/24576 h1272561 m390485 e366247, srbeat 45794s/0q drop 18stale/7716undec
- parity stream max cone: 139044
- parity stream max zone: 22426
- parity stream max wire bundle: bundle
### parity world capture lens (debug chain on the §7b world)
lens_red=0 lens_black=0 lens_water=0 lens_dark=0 lens_zero=0 rot=none scan=0 carriers=0
- parity mesh probe @png_y=2: x306=(151,184,255) x307=(151,184,255) x308=(151,184,255) x309h=(151,184,255) x310c=(151,184,255)
- parity mesh probe @png_y=717: x306=(44,62,51) x307=(65,89,72) x308=(81,107,87) x309h=(82,108,87) x310c=(84,111,90)
- parity mesh engagement: magenta-ish=0 of 921600 sampled px
- mask8: the un-fog engagement masks shot on the PARITY WORLD (vd 8) — parity_dbg8/01m|07m|10m
- mask8 01m: green=4944 red=0 blue=102048 dim=258496 (px, sampled /16)
- mask8 07m: green=11072 red=0 blue=221776 dim=5872 (px, sampled /16)
- mask8 10m: green=4224 red=160 blue=211056 dim=215824 (px, sampled /16)
- mesh probe @png_y=2: x306=(255,255,255) x307=(13,254,0) x308=(0,254,0) x309h=(0,0,0) x310c=(0,0,0)
- mesh probe @png_y=717: x306=(255,255,255) x307=(13,254,0) x308=(0,254,0) x309h=(0,0,0) x310c=(0,0,0)
- mesh engagement: magenta-ish=144 of 921600 sampled px (the vanilla-path paint coverage)
- mask8 fog state @png_y=2: raw (300)=(151,184,255) (301)=(151,184,255)
- mask8 fog state @png_y=717: raw (300)=(0,128,8) (301)=(179,206,255)
- mask8 fog state decoded: envStart=0 envEnd=1024 rdEnd=128 fogColor=(179,206,255)
- buffer dump 10m: marked h=29/2048 c=1/2048; height windows r0<142:17 r1:1 r2>=202:11 maxByte=255
- buffer dump 10m: RANK-2-WINDOW HEIGHTS (the fake-wall class): (+20,-32)h=177, (+26,-32)h=161, (+86,-28)h=191, (+2,-10)h=163, (+8,-10)h=169, (+110,-4)h=191, (+100,-2)h=191, (+26,+2)h=149, (+100,+2)h=191, (+2,+18)h=159
- rim mask: scan populated (cells=65536; the state paint reads the same buffers the march reads)
- phase: 10 rim mask lane (16:39:30)
- rim mask: decode-state frames shot (rim_mask/00|01|12 — GREEN full decode / RED colorless / BLUE heightless; 00 = the leg-1 tower eye, 01 = the near-tower contrast, 12 = the tower re-read; the inherited 07/12 vantages sat 760 blocks outside the streamed disk and 01 sat INSIDE the marker column — their earlier verdicts were classifier noise, purged in v6.33.1)
- rim mask 00: green=2624 red=89792 blue=84912 yellow=3968 (px, sampled /16) -> DOMINANT RED: rim = COLORLESS class — the color words die client-side (pool competition / fog strobe margin / the v6.23 HeightRef capture gate); heights ARE decoded
- rim mask 01: green=2880 red=149184 blue=135216 yellow=9152 (px, sampled /16) -> DOMINANT RED: rim = COLORLESS class — the color words die client-side (pool competition / fog strobe margin / the v6.23 HeightRef capture gate); heights ARE decoded
- rim mask 12: green=9936 red=120400 blue=117984 yellow=12560 (px, sampled /16) -> DOMINANT RED: rim = COLORLESS class — the color words die client-side (pool competition / fog strobe margin / the v6.23 HeightRef capture gate); heights ARE decoded

### boot / stream
```
[c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 126319 bytes, sha1 fcf296bd4e39808e449a94caf952146e90e31cd7 — zero textures, zero world data, one push per join
[c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[16:38:46 INFO]: [c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 126319 bytes, sha1 fcf296bd4e39808e449a94caf952146e90e31cd7 — zero textures, zero world data, one push per join
[16:38:46 INFO]: [c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[16:40:06 INFO]: [c-LoD] stream: tick 700 65536/65536 cells, lap 557, cone 10025 zone 818 dirty 0 (q 1176) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 395504c/17e, edits +8p/8c/5e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 142 forced 715, native 10160c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 114155p/282b fail 0 ~123 kB/s cache 13547/24576 h100608 m13547 e0, srbeat 3501s/200q drop 0stale/404undec
[c-LoD] stream: tick 700 65536/65536 cells, lap 557, cone 10025 zone 818 dirty 0 (q 1176) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 395504c/17e, edits +8p/8c/5e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 142 forced 715, native 10160c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 114155p/282b fail 0 ~123 kB/s cache 13547/24576 h100608 m13547 e0, srbeat 3501s/200q drop 0stale/404undec
```
### sessions
```
[16:39:08 INFO]: System chat: Stepan joined the game
[16:40:11 INFO]: System chat: Stepan left the game
```
### client log tail
```
[16:40:11] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[16:40:11] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[16:40:11] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[16:40:11] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[16:40:11] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[16:40:11] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[16:40:11] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[16:40:11] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[16:40:11] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[16:40:11] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[16:40:11] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[16:40:11] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[16:40:11] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[16:40:11] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[16:40:11] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[16:40:11] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[16:40:11] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[16:40:11] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[16:40:11] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[16:40:11] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[16:40:11] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[16:40:11] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[16:40:11] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[16:40:11] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[16:40:11] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[16:40:11] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:bundle_delimiter] net.minecraft.network.protocol.game.ClientboundBundleDelimiterPacket -> 1 bytes
[16:40:11] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[16:40:11] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[16:40:11] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[16:40:11] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
```
### server log tail
```
[16:40:12 INFO]: [ChunkHolderManager] Saved 1000 block chunks, 974 entity chunks, 0 poi chunks in world 'minecraft:overworld' in 0.75s
[16:40:12 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:overworld'
[16:40:12 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:overworld'
[16:40:12 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[16:40:12 INFO]: [ChunkHolderManager] Waiting 60s for chunk system to halt for world 'minecraft:the_nether'
[16:40:12 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:the_nether'
[16:40:12 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_nether'
[16:40:12 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_nether' in 0.00s
[16:40:12 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_nether'
[16:40:12 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_nether'
[16:40:12 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[16:40:12 INFO]: [ChunkHolderManager] Waiting 60s for chunk system to halt for world 'minecraft:the_end'
[16:40:12 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:the_end'
[16:40:12 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_end'
[16:40:12 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_end' in 0.00s
[16:40:12 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_end'
[16:40:12 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_end'
[16:40:12 INFO]: Waiting for all RegionFile I/O tasks to complete...
[16:40:12 INFO]: All RegionFile I/O tasks to complete
[16:40:12 INFO]: [MoonriseCommon] Awaiting termination of worker pool for up to 60s...
[16:40:12 INFO]: [MoonriseCommon] Awaiting termination of I/O pool for up to 60s...
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[launcher] server exited with code 143
```
### shots
```
-rw-r--r-- 1 runner runner  5382 Nov 12 16:02 lens_12_after_edit_probe4.png
-rw-r--r-- 1 runner runner  5382 Nov 12 16:03 lens_12_after_edit_probe5.png
-rw-r--r-- 1 runner runner  5382 Nov 12 16:03 lens_12_after_edit_probe6.png
-rw-r--r-- 1 runner runner  5382 Nov 12 16:03 lens_12_after_edit_probe7.png
-rw-r--r-- 1 runner runner  5382 Nov 12 16:03 lens_12_after_edit_probe8.png
-rw-r--r-- 1 runner runner  5382 Nov 12 16:03 lens_12_after_edit_probe9.png
-rw-r--r-- 1 runner runner  7321 Nov 12 16:11 lens_13_after_relog_probe0.png
-rw-r--r-- 1 runner runner  7301 Nov 12 16:11 lens_13_after_relog_probe1.png
-rw-r--r-- 1 runner runner  7469 Nov 12 16:12 lens_13_after_relog_probe10.png
-rw-r--r-- 1 runner runner  7554 Nov 12 16:12 lens_13_after_relog_probe11.png
-rw-r--r-- 1 runner runner  7755 Nov 12 16:11 lens_13_after_relog_probe2.png
-rw-r--r-- 1 runner runner  7539 Nov 12 16:11 lens_13_after_relog_probe3.png
-rw-r--r-- 1 runner runner  7357 Nov 12 16:11 lens_13_after_relog_probe4.png
-rw-r--r-- 1 runner runner  8214 Nov 12 16:11 lens_13_after_relog_probe5.png
-rw-r--r-- 1 runner runner  7619 Nov 12 16:11 lens_13_after_relog_probe6.png
-rw-r--r-- 1 runner runner  7427 Nov 12 16:12 lens_13_after_relog_probe7.png
-rw-r--r-- 1 runner runner  7640 Nov 12 16:12 lens_13_after_relog_probe8.png
-rw-r--r-- 1 runner runner  7744 Nov 12 16:12 lens_13_after_relog_probe9.png
drwxr-xr-x 2 runner runner  4096 Nov 12 16:35 lens_parity
drwxr-xr-x 2 runner runner  4096 Nov 12 16:35 parity
drwxr-xr-x 2 runner runner  4096 Nov 12 16:07 parity_dbg
drwxr-xr-x 2 runner runner  4096 Nov 12 16:38 parity_dbg8
-rw-r--r-- 1 runner runner  6757 Nov 12 16:12 probe_counts.txt
drwxr-xr-x 2 runner runner  4096 Nov 12 16:40 rim_mask
-rw-r--r-- 1 runner runner 34833 Nov 12 16:40 summary.md
```

**exit=1** failures=2 leg1=0 leg2=0
