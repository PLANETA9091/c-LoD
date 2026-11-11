- phase: 0 sanity (20:01:36)
### c-LoD visual run (-930,796, marker=red_concrete, vantage -1005,150 yaw 270)
- GL:     Vendor: Mesa (0xffffffff);    Device: llvmpipe (LLVM 21.1.8, 256 bits) (0xffffffff);OpenGL renderer string: llvmpipe (LLVM 21.1.8, 256 bits);OpenGL core profile version string: 4.5 (Core Profile) Mesa 26.0.8-1ubuntu0.3;
- boot + gates OK (20:01:57)
- phase: 1 server booted (20:01:57)
- sky-grid armed
- phase: 2 leg-1 client joined (20:02:19)
- joined session 1 (20:02:19)
- stream live: cells=65536
- phase: 3 scan populated (20:02:41)
- terrain range: 2 terraced mountains at (CX-4,CZ-20)/(CX-2,CZ+16), tops 185/190, grass terraces + stone caps, 66..82 m from the eye (inside the 83.5 m band, chunk edge 64 m)
- scan complete=1 (cells=cells=65536/65536)
- paint settled=1 (wire 60300p)
- baseline try 1: drift=4398 px
- baseline try 2: drift=3405 px
- baseline try 3: drift=5237 px
- baseline try 4: drift=1391 px
- lod_silhouette_cols=24 (LOD paint ON SCREEN at 75 m if > 20 — the march-drawn marker column; vanilla cannot render there, chunk edge 64 m; #255 re-measures 49, the ZONE_FAR=48 regression #256 and the control read 0)
- 249 terrain-green px in the sky region (LOD MOUNTAINS on screen if > 5000; the range paints ColLow-green on low terraces via the fallback palette; flat-world baselines and the pack-free control read ~0)
- burst_close white px: 555 (renderer alive if > 50)
- burst_far white px: 360 (renderer alive in the marker zone if > 20)
- cut walk 40 m: 72 white px (> 20 = rendered)
- cut walk 60 m: 0 white px (> 20 = rendered)
- cut walk 100 m: 0 white px (> 20 = rendered)
- measured far-render cut: 40 m (G4 flap instrument; the decode proof needs >= 83 m from the y=151.6 eye)
- fill issued: 5x111x5 red_concrete at -930,796 (zone forceloaded ±48)
- phase: 4 marker filled (20:06:25)

### /clod-cells AFTER FILL (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 35 0 0x8400 0x3100
123 122 -940 784 71 0 1 [97, 119, 97] 35 0 0x8700 0x3100
124 122 -938 784 68 0 1 [97, 119, 97] 34 0 0x8400 0x3100
125 122 -936 784 62 1 1 [63, 118, 228] 34 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 33 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 33 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 32 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 91 0 0x8000 0x5c00
130 122 -926 784 66 0 1 [134, 183, 131] 91 0 0x8200 0x5c00
131 122 -924 784 75 0 1 [97, 119, 97] 90 0 0x8b00 0x3100
132 122 -922 784 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 122 -920 784 75 0 1 [97, 119, 97] 18446744073709551615 1 0x8b00 0x3100
134 122 -918 784 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x3100
122 123 -942 786 70 0 1 [97, 119, 97] 35 0 0x8600 0x3100
123 123 -940 786 65 0 1 [97, 119, 97] 35 0 0x8100 0x3100
124 123 -938 786 65 0 1 [97, 119, 97] 34 0 0x8100 0x3100
125 123 -936 786 62 1 1 [63, 118, 228] 34 0 0x7e00 0xd800
```
### stream diag after fill:
[c-LoD] stream: tick 2600 65536/65536 cells, lap 1520, cone 30604 zone 939 dirty 883 (q 11982) ewin 229 rbeat 0p/0 rearm 0c/0t rescan 615514c/1133e, edits +6032p/4529c/567e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 301 forced 211, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 258090p/352b fail 0 ~109 kB/s cache 13277/24576 h244813 m13277 e0, srbeat 0s/0q drop 0stale/0undec
[20:06:26 INFO]: [c-LoD] stream: tick 2600 65536/65536 cells, lap 1520, cone 30604 zone 939 dirty 883 (q 11982) ewin 229 rbeat 0p/0 rearm 0c/0t rescan 615514c/1133e, edits +6032p/4529c/567e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 301 forced 211, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 258090p/352b fail 0 ~109 kB/s cache 13277/24576 h244813 m13277 e0, srbeat 0s/0q drop 0stale/0undec
- probe 0: red=0 paint=3036 lod_silhouette_cols=28 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 1: red=0 paint=3036 lod_silhouette_cols=28 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 2: red=0 paint=3036 lod_silhouette_cols=28 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 3: red=0 paint=3036 lod_silhouette_cols=28 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 4: red=0 paint=3036 lod_silhouette_cols=28 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 5: red=0 paint=3191 lod_silhouette_cols=26 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 6: red=0 paint=3052 lod_silhouette_cols=27 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 7: red=0 paint=3021 lod_silhouette_cols=28 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 8: red=0 paint=3368 lod_silhouette_cols=26 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 9: red=0 paint=3114 lod_silhouette_cols=27 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=1 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 10: red=0 paint=5360 lod_silhouette_cols=27 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 11: red=0 paint=5536 lod_silhouette_cols=27 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 12: red=0 paint=7157 lod_silhouette_cols=31 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 13: red=0 paint=7110 lod_silhouette_cols=31 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 14: red=0 paint=7110 lod_silhouette_cols=31 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 15: red=0 paint=7110 lod_silhouette_cols=31 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 16: red=0 paint=7110 lod_silhouette_cols=31 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 17: red=0 paint=7110 lod_silhouette_cols=31 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 18: red=0 paint=7110 lod_silhouette_cols=31 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 19: red=0 paint=7465 lod_silhouette_cols=30 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 20: red=0 paint=7653 lod_silhouette_cols=30 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 21: red=0 paint=8433 lod_silhouette_cols=30 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 22: red=0 paint=8837 lod_silhouette_cols=30 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 23: red=0 paint=8823 lod_silhouette_cols=31 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- phase: 5 leg-1 probes done (20:09:39)
- **FAIL:** leg-1 never saw the marker in 24 probes (red < 60 everywhere) — the edit never reached the LOD paint; this failure was SILENT before v6.31.1
### stream diag after probes:
[20:09:27 INFO]: [c-LoD] stream: tick 4400 65536/65536 cells, lap 2432, cone 54188 zone 1053 dirty 933 (q 11978) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1196985c/1191e, edits +7593p/5630c/889e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 301 forced 211, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 446458p/552b fail 0 ~123 kB/s cache 13369/24576 h433089 m13369 e0, srbeat 0s/0q drop 0stale/0undec
[20:09:37 INFO]: [c-LoD] stream: tick 4500 65536/65536 cells, lap 2532, cone 55662 zone 1053 dirty 933 (q 11978) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1227842c/1191e, edits +7594p/5631c/889e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 301 forced 211, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 458146p/564b fail 0 ~61 kB/s cache 13369/24576 h444777 m13369 e0, srbeat 0s/0q drop 0stale/0undec
[c-LoD] stream: tick 4500 65536/65536 cells, lap 2532, cone 55662 zone 1053 dirty 933 (q 11978) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1227842c/1191e, edits +7594p/5631c/889e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 301 forced 211, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 458146p/564b fail 0 ~61 kB/s cache 13369/24576 h444777 m13369 e0, srbeat 0s/0q drop 0stale/0undec
- parity debug tour: 13 vantages (session-1 client, DEBUG pack — evidence only, the gate pairs the §7b world session)
- parity debug tour done: 13 shots in parity_dbg/ (the §7b world session feeds the gate)
- joined session 2 (20:12:30)
- leg-2 baseline scene px (x<940): 13673 (0 = the relog client paints no world at all)
- leg-2 renderer probe NEAR (20 m) white px: 524 raw / 524 changed-vs-baseline (> 20 = the client renders particles)
- leg-2 renderer probe FAR (80 m, marker zone) white px: 106 raw / 106 changed-vs-baseline (> 20 = no distance cut; ~0 with NEAR alive = particles beyond the client chunk radius do not render)
- leg-2 carrier probe green dust (80 m): changed-green px 0 / total changed 5359 vs baseline (> 100 green = a stable green carrier exists; total > 0 with green ~0 = the dust renders but fog washes it — a ruler problem, not a carrier problem)
- phase: 8 leg-2 relog probes (20:15:19)
- relog probe 0: red=0 scene=17723 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 1: red=0 scene=17747 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=21 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 2: red=0 scene=21403 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 3: red=0 scene=22485 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 4: red=0 scene=22481 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 5: red=0 scene=24454 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 6: red=0 scene=24593 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 7: red=0 scene=26915 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 8: red=0 scene=28630 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=1 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 9: red=0 scene=29088 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 10: red=0 scene=29166 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 11: red=0 scene=29585 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=10 (need red >= 60; scene > 0 = the client paints a world at all)
- **FAIL:** marker lost after relog — stream state is not persistent/deterministic
- phase: 9 leg-2 done (20:16:53)

### client log forensics (pipeline/particle/atlas — v6.8.2)
```
java.lang.IllegalStateException: Failed to open OpenAL device
	at net.minecraft.client.sounds.SoundEngine.reload(SoundEngine.java:110)
	at net.minecraft.server.packs.resources.SimplePreparableReloadListener.lambda$reload$1(SimplePreparableReloadListener.java:15)
	at net.minecraft.server.packs.resources.SimpleReloadInstance.lambda$prepareTasks$3(SimpleReloadInstance.java:59)
[20:12:26] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[20:12:26] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[20:12:26] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[20:12:26] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[20:12:26] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[20:12:26] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[20:12:26] [Render thread/INFO]: Created: 2048x2048x4 minecraft:textures/atlas/blocks.png-atlas
[20:12:27] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[20:12:27] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[20:12:27] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[20:12:27] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[20:12:27] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
[20:12:34] [Render thread/INFO]: Reloading ResourceManager: vanilla, vanilla, server/00000000/6b1f2f7e-9c0d-4a4b-8b0a-b10d1b4ab100
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
[20:12:35] [Render thread/ERROR]: Error starting SoundSystem. Turning off sounds & music
java.lang.IllegalStateException: Failed to open OpenAL device
	at net.minecraft.client.sounds.SoundEngine.reload(SoundEngine.java:110)
	at net.minecraft.server.packs.resources.SimplePreparableReloadListener.lambda$reload$1(SimplePreparableReloadListener.java:15)
	at net.minecraft.server.packs.resources.SimpleReloadInstance.lambda$prepareTasks$3(SimpleReloadInstance.java:59)
[20:12:35] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[20:12:35] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[20:12:35] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[20:12:35] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[20:12:35] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[20:12:35] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[20:12:35] [Render thread/INFO]: Created: 2048x2048x4 minecraft:textures/atlas/blocks.png-atlas
[20:12:35] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[20:12:35] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[20:12:35] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[20:12:35] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[20:12:35] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
```

### /clod-cells AFTER PROBES (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 53 0 0x8400 0x3100
123 122 -940 784 71 0 1 [97, 119, 97] 53 0 0x8700 0x3100
124 122 -938 784 68 0 1 [97, 119, 97] 52 0 0x8400 0x3100
125 122 -936 784 62 1 1 [63, 118, 228] 52 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 51 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 51 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 50 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 49 0 0x8000 0x5c00
130 122 -926 784 66 0 1 [134, 183, 131] 49 0 0x8200 0x5c00
131 122 -924 784 75 0 1 [97, 119, 97] 48 0 0x8b00 0x3100
132 122 -922 784 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 122 -920 784 75 0 1 [97, 119, 97] 18446744073709551615 1 0x8b00 0x3100
134 122 -918 784 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x3100
122 123 -942 786 70 0 1 [97, 119, 97] 53 0 0x8600 0x3100
123 123 -940 786 65 0 1 [97, 119, 97] 53 0 0x8100 0x3100
124 123 -938 786 65 0 1 [97, 119, 97] 52 0 0x8100 0x3100
125 123 -936 786 62 1 1 [63, 118, 228] 52 0 0x7e00 0xd800
```
- edit channel: 0 linkage errors, 0 failed event dispatches (both must be 0)
- phase: 6 parity world boot (20:16:54)
- parity world: view-distance=8 (the voxy leg's server value)
- parity world: joined session 3 (20:17:31)
- parity world: scan populated (cells=65536; the march paints from these buffers)
- parity world tour: 13 vantages (production pack, vd 8, shared structures)
- coverage sweeps: 72 eye passes = 36 positions (12 az x r144/200 + 12 off-az x r172, y=160) x 2 looks (outward+inward), settle-by-laps >= 6 s (q stable 3x1s), deadline 12 s/pos, no shots — the v6.26.0 data-coverage lever + v6.26.6 lottery kill; probe rides parity/view_probe.md
- parity drain-settle TIMEOUT: queue 19866 -> 19866 > 11500 after 150s — continuing anyway; the queue did not drain without eye proximity (priority-lane evidence, the 01/05/10 dip attribution reads from the tour deltas)
- pre-tour pass done: 13 vantages x 10s (the tour now reads the second OR-merge lap; the rim/dip transport race reads from the tour deltas)
- parity world tour done: 13 shots, 0 night-guard renames in parity/ (the gate pairs voxy/<name>.png vs parity/<name>.png)
- phase: 7 parity tour done (20:39:09)
- sparse re-beat channel at tour end: streamed=50013 q=0 dropped_stale=23 dropped_undecodable=12795 (frac=8 boot; undecodable-dominant = the rim words die in their decodability window, stale-dominant = edit churn owns the cells)
### parity world stream counters (session 3, tail + maxima)
[c-LoD] stream: tick 13000 65536/65536 cells, lap 8443, cone 151625 zone 23632 dirty 9699 (q 19815) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 8894677c/1939e, edits +18123p/13738c/2942e drop 0, spawned 0 / failed 0, cam0 yaw -180 pitch 5, wish 0 forced 931, native 31232c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 1832220p/3806b fail 0 ~0 kB/s cache 22899/24576 h1393458 m438762 e415863, srbeat 50013s/0q drop 23stale/12795undec
[20:39:02 INFO]: [c-LoD] stream: tick 13000 65536/65536 cells, lap 8443, cone 151625 zone 23632 dirty 9699 (q 19815) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 8894677c/1939e, edits +18123p/13738c/2942e drop 0, spawned 0 / failed 0, cam0 yaw -180 pitch 5, wish 0 forced 931, native 31232c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 1832220p/3806b fail 0 ~0 kB/s cache 22899/24576 h1393458 m438762 e415863, srbeat 50013s/0q drop 23stale/12795undec
- parity stream max cone: 151625
- parity stream max zone: 23632
- parity stream max wire bundle: bundle
### parity world capture lens (debug chain on the §7b world)
lens_red=0 lens_black=0 lens_water=0 lens_dark=0 lens_zero=0 rot=none scan=0 carriers=0
- parity mesh probe @png_y=2: x306=(151,184,255) x307=(151,184,255) x308=(151,184,255) x309h=(151,184,255) x310c=(151,184,255)
- parity mesh probe @png_y=717: x306=(61,77,67) x307=(62,79,68) x308=(64,80,70) x309h=(55,68,61) x310c=(43,61,51)
- parity mesh engagement: magenta-ish=272 of 921600 sampled px
- mask8: the un-fog engagement masks shot on the PARITY WORLD (vd 8) — parity_dbg8/01m|07m|10m
- mask8 01m: green=3792 red=0 blue=111952 dim=207408 (px, sampled /16)
- mask8 07m: green=11072 red=0 blue=221792 dim=5872 (px, sampled /16)
- mask8 10m: green=8288 red=176 blue=217648 dim=225984 (px, sampled /16)
- mesh probe @png_y=2: x306=(255,255,255) x307=(13,254,0) x308=(0,254,0) x309h=(4,0,0) x310c=(0,0,0)
- mesh probe @png_y=717: x306=(255,255,255) x307=(13,254,0) x308=(0,254,0) x309h=(4,0,0) x310c=(0,0,0)
- mesh engagement: magenta-ish=32 of 921600 sampled px (the vanilla-path paint coverage)
- mask8 fog state @png_y=2: raw (300)=(151,184,255) (301)=(151,184,255)
- mask8 fog state @png_y=717: raw (300)=(179,206,255) (301)=(179,206,255)
- mask8 fog state decoded: envStart=1432 envEnd=1648 rdEnd=4080 fogColor=(179,206,255)
- buffer dump 10m: marked h=27/2048 c=1/2048; height windows r0<142:16 r1:1 r2>=202:10 maxByte=255
- buffer dump 10m: RANK-2-WINDOW HEIGHTS (the fake-wall class): (+26,-32)h=161, (+86,-28)h=191, (+64,-26)h=177, (+100,-8)h=191, (+100,-6)h=191, (+110,-4)h=191, (+106,+0)h=188, (+100,+2)h=191, (+26,+18)h=149, (+2,+22)h=159
- rim mask: scan populated (cells=65536; the state paint reads the same buffers the march reads)
- phase: 10 rim mask lane (20:45:02)
- rim mask: decode-state frames shot (rim_mask/00|01|12 — GREEN full decode / RED colorless / BLUE heightless; 00 = the leg-1 tower eye, 01 = the near-tower contrast, 12 = the tower re-read; the inherited 07/12 vantages sat 760 blocks outside the streamed disk and 01 sat INSIDE the marker column — their earlier verdicts were classifier noise, purged in v6.33.1)
- rim mask 00: green=12800 red=103920 blue=94704 (px, sampled /16) -> DOMINANT RED: rim = COLORLESS class — the color words die client-side (pool competition / fog strobe margin / the v6.23 HeightRef capture gate); heights ARE decoded
- rim mask 01: green=12592 red=160528 blue=149760 (px, sampled /16) -> DOMINANT RED: rim = COLORLESS class — the color words die client-side (pool competition / fog strobe margin / the v6.23 HeightRef capture gate); heights ARE decoded
- rim mask 12: green=22112 red=128752 blue=116896 (px, sampled /16) -> DOMINANT RED: rim = COLORLESS class — the color words die client-side (pool competition / fog strobe margin / the v6.23 HeightRef capture gate); heights ARE decoded

### boot / stream
```
[20:42:38 INFO]: [c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 124538 bytes, sha1 fe5469942cc8784a45bb58789463397df61e3b4f — zero textures, zero world data, one push per join
[20:42:38 INFO]: [c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 124538 bytes, sha1 fe5469942cc8784a45bb58789463397df61e3b4f — zero textures, zero world data, one push per join
[c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[c-LoD] stream: tick 1700 65536/65536 cells, lap 1577, cone 30028 zone 852 dirty 18 (q 1012) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1116593c/6e, edits +22p/22c/8e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 263 forced 637, native 11954c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 332780p/797b fail 0 ~122 kB/s cache 13607/24576 h319173 m13607 e0, srbeat 10930s/96q drop 0stale/179undec
[20:45:38 INFO]: [c-LoD] stream: tick 1700 65536/65536 cells, lap 1577, cone 30028 zone 852 dirty 18 (q 1012) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1116593c/6e, edits +22p/22c/8e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 263 forced 637, native 11954c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 332780p/797b fail 0 ~122 kB/s cache 13607/24576 h319173 m13607 e0, srbeat 10930s/96q drop 0stale/179undec
```
### sessions
```
[20:42:59 INFO]: System chat: Stepan joined the game
[20:45:43 INFO]: System chat: Stepan left the game
```
### client log tail
```
[20:45:43] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[20:45:43] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[20:45:43] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[20:45:43] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[20:45:43] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[20:45:43] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[20:45:43] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[20:45:43] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[20:45:43] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[20:45:43] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:bundle_delimiter] net.minecraft.network.protocol.game.ClientboundBundleDelimiterPacket -> 1 bytes
[20:45:43] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[20:45:43] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[20:45:43] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[20:45:43] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[20:45:43] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[20:45:43] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[20:45:43] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[20:45:43] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[20:45:43] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[20:45:43] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[20:45:43] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[20:45:43] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[20:45:43] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[20:45:43] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[20:45:43] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[20:45:43] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[20:45:43] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[20:45:43] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[20:45:43] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[20:45:43] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
```
### server log tail
```
[c-LoD] EB: enter, native_gen=true
[20:45:44 INFO]: [ChunkHolderManager] Saved 1005 block chunks, 979 entity chunks, 0 poi chunks in world 'minecraft:overworld' in 1.01s
[20:45:44 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:overworld'
[20:45:44 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:overworld'
[20:45:44 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[20:45:44 INFO]: [ChunkHolderManager] Waiting 60s for chunk system to halt for world 'minecraft:the_nether'
[20:45:44 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:the_nether'
[20:45:44 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_nether'
[20:45:44 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_nether' in 0.00s
[20:45:44 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_nether'
[20:45:44 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_nether'
[20:45:44 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[20:45:44 INFO]: [ChunkHolderManager] Waiting 60s for chunk system to halt for world 'minecraft:the_end'
[20:45:44 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:the_end'
[20:45:44 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_end'
[20:45:44 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_end' in 0.00s
[20:45:44 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_end'
[20:45:44 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_end'
[20:45:44 INFO]: Waiting for all RegionFile I/O tasks to complete...
[20:45:44 INFO]: All RegionFile I/O tasks to complete
[20:45:44 INFO]: [MoonriseCommon] Awaiting termination of worker pool for up to 60s...
[20:45:44 INFO]: [MoonriseCommon] Awaiting termination of I/O pool for up to 60s...
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[launcher] server exited with code 143
```
### shots
```
-rw-r--r-- 1 runner runner   5362 Nov 11 20:07 lens_12_after_edit_probe4.png
-rw-r--r-- 1 runner runner   6104 Nov 11 20:07 lens_12_after_edit_probe5.png
-rw-r--r-- 1 runner runner   5533 Nov 11 20:07 lens_12_after_edit_probe6.png
-rw-r--r-- 1 runner runner   5375 Nov 11 20:07 lens_12_after_edit_probe7.png
-rw-r--r-- 1 runner runner   6072 Nov 11 20:07 lens_12_after_edit_probe8.png
-rw-r--r-- 1 runner runner   5501 Nov 11 20:07 lens_12_after_edit_probe9.png
-rw-r--r-- 1 runner runner   6704 Nov 11 20:15 lens_13_after_relog_probe0.png
-rw-r--r-- 1 runner runner   6418 Nov 11 20:15 lens_13_after_relog_probe1.png
-rw-r--r-- 1 runner runner   6837 Nov 11 20:16 lens_13_after_relog_probe10.png
-rw-r--r-- 1 runner runner   6646 Nov 11 20:16 lens_13_after_relog_probe11.png
-rw-r--r-- 1 runner runner   7141 Nov 11 20:15 lens_13_after_relog_probe2.png
-rw-r--r-- 1 runner runner   6595 Nov 11 20:15 lens_13_after_relog_probe3.png
-rw-r--r-- 1 runner runner   6588 Nov 11 20:15 lens_13_after_relog_probe4.png
-rw-r--r-- 1 runner runner   7219 Nov 11 20:16 lens_13_after_relog_probe5.png
-rw-r--r-- 1 runner runner   6808 Nov 11 20:16 lens_13_after_relog_probe6.png
-rw-r--r-- 1 runner runner   6672 Nov 11 20:16 lens_13_after_relog_probe7.png
-rw-r--r-- 1 runner runner   7282 Nov 11 20:16 lens_13_after_relog_probe8.png
-rw-r--r-- 1 runner runner   7012 Nov 11 20:16 lens_13_after_relog_probe9.png
drwxr-xr-x 2 runner runner   4096 Nov 11 20:39 lens_parity
drwxr-xr-x 2 runner runner   4096 Nov 11 20:39 parity
drwxr-xr-x 2 runner runner   4096 Nov 11 20:12 parity_dbg
drwxr-xr-x 2 runner runner   4096 Nov 11 20:42 parity_dbg8
-rw-r--r-- 1 runner runner   6746 Nov 11 20:16 probe_counts.txt
drwxr-xr-x 2 runner runner   4096 Nov 11 20:45 rim_mask
-rw-r--r-- 1 runner runner  34879 Nov 11 20:45 summary.md
```

**exit=1** failures=2 leg1=0 leg2=0
