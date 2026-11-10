### c-LoD visual run (-930,796, marker=red_concrete, vantage -1005,150 yaw 270)
- GL:     Vendor: Mesa (0xffffffff);    Device: llvmpipe (LLVM 20.1.2, 256 bits) (0xffffffff);OpenGL renderer string: llvmpipe (LLVM 20.1.2, 256 bits);OpenGL core profile version string: 4.5 (Core Profile) Mesa 25.2.8-0ubuntu0.24.04.4;
- boot + gates OK (16:22:05)
- sky-grid armed
- joined session 1 (16:22:32)
- stream live: cells=65536
- terrain range: 2 terraced mountains at (CX-4,CZ-20)/(CX-2,CZ+16), tops 185/190, grass terraces + stone caps, 66..82 m from the eye (inside the 83.5 m band, chunk edge 64 m)
- scan complete=1 (cells=cells=65536/65536)
- paint settled=1 (wire 57609p)
- baseline try 1: drift=503 px
- lod_silhouette_cols=19 (LOD paint ON SCREEN at 75 m if > 20 — the march-drawn marker column; vanilla cannot render there, chunk edge 64 m; #255 re-measures 49, the ZONE_FAR=48 regression #256 and the control read 0)
- 0 terrain-green px in the sky region (LOD MOUNTAINS on screen if > 5000; the range paints ColLow-green on low terraces via the fallback palette; flat-world baselines and the pack-free control read ~0)
- burst_close white px: 581 (renderer alive if > 50)
- burst_far white px: 429 (renderer alive in the marker zone if > 20)
- cut walk 40 m: 75 white px (> 20 = rendered)
- cut walk 60 m: 0 white px (> 20 = rendered)
- cut walk 100 m: 0 white px (> 20 = rendered)
- measured far-render cut: 40 m (G4 flap instrument; the decode proof needs >= 83 m from the y=151.6 eye)
- fill issued: 5x111x5 red_concrete at -930,796 (zone forceloaded ±48)

### /clod-cells AFTER FILL (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 98 0 0x8400 0x3100
123 122 -940 784 71 0 1 [97, 119, 97] 98 0 0x8700 0x3100
124 122 -938 784 68 0 1 [97, 119, 97] 97 0 0x8400 0x3100
125 122 -936 784 62 1 1 [63, 118, 228] 97 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 96 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 96 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 95 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 95 0 0x8000 0x5c00
130 122 -926 784 66 0 1 [134, 183, 131] 94 0 0x8200 0x5c00
131 122 -924 784 75 0 1 [97, 119, 97] 94 0 0x8b00 0x3100
132 122 -922 784 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 122 -920 784 75 0 1 [97, 119, 97] 18446744073709551615 1 0x8b00 0x3100
134 122 -918 784 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x3100
122 123 -942 786 66 0 1 [97, 119, 97] 98 0 0x8200 0x3100
123 123 -940 786 65 0 1 [97, 119, 97] 98 0 0x8100 0x3100
124 123 -938 786 65 0 1 [97, 119, 97] 97 0 0x8100 0x3100
125 123 -936 786 62 1 1 [63, 118, 228] 97 0 0x7e00 0xd800
```
### stream diag after fill:
[c-LoD] stream: tick 1900 65536/65536 cells, lap 871, cone 12634 zone 780 dirty 863 (q 12499) ewin 211 rbeat 0p/1 rearm 0c/0t rescan 404101c/981e, edits +5426p/4086c/615e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 292 forced 220, native 65536c/1090ch fail 0 (3w q 0) helper on, trees 1090ch/fail 0, wire bundle 114085p/190b fail 0 ~0 kB/s cache 12786/24576 h101299 m12786 e0
[16:25:27 INFO]: [c-LoD] stream: tick 1900 65536/65536 cells, lap 871, cone 12634 zone 780 dirty 863 (q 12499) ewin 211 rbeat 0p/1 rearm 0c/0t rescan 404101c/981e, edits +5426p/4086c/615e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 292 forced 220, native 65536c/1090ch fail 0 (3w q 0) helper on, trees 1090ch/fail 0, wire bundle 114085p/190b fail 0 ~0 kB/s cache 12786/24576 h101299 m12786 e0
- probe 0: red=0 paint=272 lod_silhouette_cols=19 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 1: red=0 paint=272 lod_silhouette_cols=19 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 2: red=0 paint=272 lod_silhouette_cols=19 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 3: red=0 paint=272 lod_silhouette_cols=19 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 4: red=0 paint=546 lod_silhouette_cols=18 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 5: red=0 paint=272 lod_silhouette_cols=19 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 6: red=0 paint=272 lod_silhouette_cols=19 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 7: red=0 paint=272 lod_silhouette_cols=19 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 8: red=0 paint=272 lod_silhouette_cols=19 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 9: red=0 paint=272 lod_silhouette_cols=19 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 10: red=0 paint=2469 lod_silhouette_cols=24 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=1 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 11: red=0 paint=3046 lod_silhouette_cols=25 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 12: red=0 paint=4615 lod_silhouette_cols=26 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 13: red=0 paint=4292 lod_silhouette_cols=28 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=11 (need red>=60 paint>=300 lod>20)
- probe 14: red=0 paint=4099 lod_silhouette_cols=28 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 15: red=0 paint=4099 lod_silhouette_cols=28 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 16: red=0 paint=4099 lod_silhouette_cols=28 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 17: red=0 paint=4099 lod_silhouette_cols=28 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 18: red=0 paint=4805 lod_silhouette_cols=26 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 19: red=0 paint=4929 lod_silhouette_cols=27 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 20: red=0 paint=7265 lod_silhouette_cols=26 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 21: red=0 paint=7152 lod_silhouette_cols=27 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 22: red=0 paint=7197 lod_silhouette_cols=27 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 23: red=0 paint=7346 lod_silhouette_cols=27 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=1 mask_hits=30 (need red>=60 paint>=300 lod>20)
### stream diag after probes:
[16:28:27 INFO]: [c-LoD] stream: tick 3700 65536/65536 cells, lap 1441, cone 27866 zone 951 dirty 892 (q 12494) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 985629c/1044e, edits +6850p/5079c/895e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 292 forced 220, native 65536c/1090ch fail 0 (3w q 0) helper on, trees 1090ch/fail 0, wire bundle 236092p/320b fail 0 ~112 kB/s cache 12833/24576 h223259 m12833 e0
[c-LoD] stream: tick 3800 65536/65536 cells, lap 1541, cone 30682 zone 951 dirty 892 (q 12494) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1020521c/1044e, edits +6850p/5079c/895e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 292 forced 220, native 65536c/1090ch fail 0 (3w q 0) helper on, trees 1090ch/fail 0, wire bundle 258342p/342b fail 0 ~117 kB/s cache 12833/24576 h245509 m12833 e0
[16:28:37 INFO]: [c-LoD] stream: tick 3800 65536/65536 cells, lap 1541, cone 30682 zone 951 dirty 892 (q 12494) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1020521c/1044e, edits +6850p/5079c/895e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 292 forced 220, native 65536c/1090ch fail 0 (3w q 0) helper on, trees 1090ch/fail 0, wire bundle 258342p/342b fail 0 ~117 kB/s cache 12833/24576 h245509 m12833 e0
- parity debug tour: 13 vantages (session-1 client, DEBUG pack — evidence only, the gate pairs the §7b world session)
- parity debug tour done: 13 shots in parity_dbg/ (the §7b world session feeds the gate)
- joined session 2 (16:31:39)
- leg-2 baseline scene px (x<940): 24861 (0 = the relog client paints no world at all)
- leg-2 renderer probe NEAR (20 m) white px: 582 raw / 582 changed-vs-baseline (> 20 = the client renders particles)
- leg-2 renderer probe FAR (80 m, marker zone) white px: 420 raw / 412 changed-vs-baseline (> 20 = no distance cut; ~0 with NEAR alive = particles beyond the client chunk radius do not render)
- leg-2 carrier probe green dust (80 m): changed-green px 0 / total changed 1612 vs baseline (> 100 green = a stable green carrier exists; total > 0 with green ~0 = the dust renders but fog washes it — a ruler problem, not a carrier problem)
- relog probe 0: red=0 scene=24912 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 1: red=0 scene=24766 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=1 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 2: red=0 scene=28425 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 3: red=0 scene=28533 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 4: red=0 scene=31036 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 5: red=0 scene=31252 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 6: red=0 scene=31313 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 7: red=0 scene=32033 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 8: red=0 scene=32168 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=1 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 9: red=0 scene=32282 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 10: red=0 scene=36189 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=1 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 11: red=0 scene=36473 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- **FAIL:** marker lost after relog — stream state is not persistent/deterministic

### client log forensics (pipeline/particle/atlas — v6.8.2)
```
ALSA lib confmisc.c:1342:(snd_func_refer) error evaluating name
ALSA lib conf.c:5208:(_snd_config_evaluate) function snd_func_refer returned error: No such file or directory
ALSA lib conf.c:5731:(snd_config_expand) Evaluate error: No such file or directory
[16:31:37] [Render thread/ERROR]: Error starting SoundSystem. Turning off sounds & music
java.lang.IllegalStateException: Failed to open OpenAL device
	at net.minecraft.client.sounds.SoundEngine.reload(SoundEngine.java:110)
	at net.minecraft.server.packs.resources.SimplePreparableReloadListener.lambda$reload$1(SimplePreparableReloadListener.java:15)
	at net.minecraft.server.packs.resources.SimpleReloadInstance.lambda$prepareTasks$3(SimpleReloadInstance.java:59)
[16:31:37] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[16:31:37] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[16:31:37] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[16:31:37] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[16:31:37] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[16:31:37] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[16:31:37] [Render thread/INFO]: Created: 2048x2048x4 minecraft:textures/atlas/blocks.png-atlas
[16:31:37] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[16:31:37] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[16:31:38] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[16:31:38] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[16:31:38] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
[16:31:44] [Render thread/INFO]: Reloading ResourceManager: vanilla, vanilla, server/00000000/6b1f2f7e-9c0d-4a4b-8b0a-b10d1b4ab100
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
[16:31:46] [Render thread/ERROR]: Error starting SoundSystem. Turning off sounds & music
java.lang.IllegalStateException: Failed to open OpenAL device
	at net.minecraft.client.sounds.SoundEngine.reload(SoundEngine.java:110)
	at net.minecraft.server.packs.resources.SimplePreparableReloadListener.lambda$reload$1(SimplePreparableReloadListener.java:15)
	at net.minecraft.server.packs.resources.SimpleReloadInstance.lambda$prepareTasks$3(SimpleReloadInstance.java:59)
[16:31:46] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[16:31:46] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[16:31:46] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[16:31:46] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[16:31:46] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[16:31:46] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[16:31:46] [Render thread/INFO]: Created: 2048x2048x4 minecraft:textures/atlas/blocks.png-atlas
[16:31:47] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[16:31:47] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[16:31:47] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[16:31:47] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[16:31:47] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
```

### /clod-cells AFTER PROBES (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 56 0 0x8400 0x3100
123 122 -940 784 71 0 1 [97, 119, 97] 56 0 0x8700 0x3100
124 122 -938 784 68 0 1 [97, 119, 97] 55 0 0x8400 0x3100
125 122 -936 784 62 1 1 [63, 118, 228] 55 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 54 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 54 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 53 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 53 0 0x8000 0x5c00
130 122 -926 784 66 0 1 [134, 183, 131] 52 0 0x8200 0x5c00
131 122 -924 784 75 0 1 [97, 119, 97] 52 0 0x8b00 0x3100
132 122 -922 784 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 122 -920 784 75 0 1 [97, 119, 97] 18446744073709551615 1 0x8b00 0x3100
134 122 -918 784 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x3100
122 123 -942 786 66 0 1 [97, 119, 97] 56 0 0x8200 0x3100
123 123 -940 786 65 0 1 [97, 119, 97] 56 0 0x8100 0x3100
124 123 -938 786 65 0 1 [97, 119, 97] 55 0 0x8100 0x3100
125 123 -936 786 62 1 1 [63, 118, 228] 55 0 0x7e00 0xd800
```
- edit channel: 0 linkage errors, 0 failed event dispatches (both must be 0)
- parity world: view-distance=8 (the voxy leg's server value)
- parity world: joined session 3 (16:36:55)
- parity world: scan populated (cells=65536; the march paints from these buffers)
- parity world tour: 13 vantages (production pack, vd 8, shared structures)
- coverage sweeps: 72 eye passes = 36 positions (12 az x r144/200 + 12 off-az x r172, y=160) x 2 looks (outward+inward), settle 6 s, no shots — the v6.26.0 data-coverage lever; outer disk rho 32..256 decodes before the tour
- parity drain-settle TIMEOUT: queue 28851 -> 28833 > 14500 after 150s — continuing anyway; the queue did not drain without eye proximity (priority-lane evidence, the 01/05/10 dip attribution reads from the tour deltas)
- parity world tour done: 13 shots, 0 night-guard renames in parity/ (the gate pairs voxy/<name>.png vs parity/<name>.png)
### parity world stream counters (session 3, tail + maxima)
[c-LoD] stream: tick 11300 65536/65536 cells, lap 7358, cone 147278 zone 16484 dirty 8018 (q 28817) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 7692118c/1914e, edits +18030p/13661c/2816e drop 0, spawned 0 / failed 0, cam0 yaw -180 pitch 5, wish 0 forced 932, native 38016c/955ch fail 0 (3w q 0) helper on, trees 955ch/fail 0, wire bundle 1331373p/1597b fail 0 ~0 kB/s cache 23882/24576 h1025688 m305685 e281803
[16:55:30 INFO]: [c-LoD] stream: tick 11300 65536/65536 cells, lap 7358, cone 147278 zone 16484 dirty 8018 (q 28817) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 7692118c/1914e, edits +18030p/13661c/2816e drop 0, spawned 0 / failed 0, cam0 yaw -180 pitch 5, wish 0 forced 932, native 38016c/955ch fail 0 (3w q 0) helper on, trees 955ch/fail 0, wire bundle 1331373p/1597b fail 0 ~0 kB/s cache 23882/24576 h1025688 m305685 e281803
- parity stream max cone: 147278
- parity stream max zone: 16484
- parity stream max wire bundle: bundle
### parity world capture lens (debug chain on the §7b world)
lens_red=0 lens_black=0 lens_water=0 lens_dark=0 lens_zero=0 rot=none scan=0 carriers=0
- parity mesh probe @png_y=2: x306=(151,184,255) x307=(151,184,255) x308=(151,184,255) x309h=(151,184,255) x310c=(151,184,255)
- parity mesh probe @png_y=717: x306=(86,94,89) x307=(66,76,70) x308=(66,77,71) x309h=(73,81,77) x310c=(61,72,66)
- parity mesh engagement: magenta-ish=2432 of 921600 sampled px
- mask8: the un-fog engagement masks shot on the PARITY WORLD (vd 8) — parity_dbg8/01m|07m|10m
- mask8 01m: green=7248 red=0 blue=103456 dim=8416 (px, sampled /16)
- mask8 07m: green=11040 red=0 blue=221824 dim=5888 (px, sampled /16)
- mask8 10m: green=3888 red=48 blue=221920 dim=132784 (px, sampled /16)
- mesh probe @png_y=2: x306=(255,255,255) x307=(13,254,0) x308=(0,254,0) x309h=(0,0,0) x310c=(0,0,0)
- mesh probe @png_y=717: x306=(255,255,255) x307=(13,254,0) x308=(0,254,0) x309h=(0,0,0) x310c=(0,0,0)
- mesh engagement: magenta-ish=0 of 921600 sampled px (the vanilla-path paint coverage)
- mask8 fog state @png_y=2: raw (300)=(151,184,255) (301)=(151,184,255)
- mask8 fog state @png_y=717: raw (300)=(0,128,8) (301)=(179,206,255)
- mask8 fog state decoded: envStart=0 envEnd=1024 rdEnd=128 fogColor=(179,206,255)
- buffer dump 10m: marked h=12/2048 c=0/2048; height windows r0<142:12 r1:0 r2>=202:0 maxByte=123
- buffer dump 10m: color idx0(black)=0 water/height mismatches=0 of 0 pairs
- buffer dump 01m: marked h=0/2048 c=0/2048; height windows r0<142:0 r1:0 r2>=202:0 maxByte=0
- buffer dump 01m: color idx0(black)=0 water/height mismatches=0 of 0 pairs
