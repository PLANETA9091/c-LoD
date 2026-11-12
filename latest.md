- phase: 0 sanity (18:25:21)
### c-LoD visual run (-930,796, marker=red_concrete, vantage -1005,150 yaw 270)
- GL:     Vendor: Mesa (0xffffffff);    Device: llvmpipe (LLVM 21.1.8, 256 bits) (0xffffffff);OpenGL renderer string: llvmpipe (LLVM 21.1.8, 256 bits);OpenGL core profile version string: 4.5 (Core Profile) Mesa 26.0.8-1ubuntu0.3;
- boot + gates OK (18:25:40)
- phase: 1 server booted (18:25:40)
- sky-grid armed
- phase: 2 leg-1 client joined (18:26:03)
- joined session 1 (18:26:04)
- stream live: cells=65536
- phase: 3 scan populated (18:26:26)
- terrain range: 2 terraced mountains at (CX-4,CZ-20)/(CX-2,CZ+16), tops 185/190, grass terraces + stone caps, 66..82 m from the eye (inside the 83.5 m band, chunk edge 64 m)
- scan complete=1 (cells=cells=65536/65536)
- paint settled=1 (wire 65093p)
- baseline try 1: drift=5187 px
- baseline try 2: drift=7144 px
- baseline try 3: drift=0 px
- lod_silhouette_cols=25 (LOD paint ON SCREEN at 75 m if > 20 — the march-drawn marker column; vanilla cannot render there, chunk edge 64 m; #255 re-measures 49, the ZONE_FAR=48 regression #256 and the control read 0)
- 12 terrain-green px in the sky region (LOD MOUNTAINS on screen if > 5000; the range paints ColLow-green on low terraces via the fallback palette; flat-world baselines and the pack-free control read ~0)
- burst_close white px: 529 (renderer alive if > 50)
- burst_far white px: 334 (renderer alive in the marker zone if > 20)
- cut walk 40 m: 68 white px (> 20 = rendered)
- cut walk 60 m: 0 white px (> 20 = rendered)
- cut walk 100 m: 0 white px (> 20 = rendered)
- measured far-render cut: 40 m (G4 flap instrument; the decode proof needs >= 83 m from the y=151.6 eye)
- fill issued: 5x111x5 red_concrete at -930,796 (zone forceloaded ±48)
- phase: 4 marker filled (18:29:45)

### /clod-cells AFTER FILL (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 48 0 0x8400 0x3100
123 122 -940 784 71 0 1 [97, 119, 97] 48 0 0x8700 0x3100
124 122 -938 784 68 0 1 [97, 119, 97] 47 0 0x8400 0x3100
125 122 -936 784 62 1 1 [63, 118, 228] 107 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 106 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 105 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 105 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 104 0 0x8000 0x5c00
130 122 -926 784 66 0 1 [134, 183, 131] 104 0 0x8200 0x5c00
131 122 -924 784 75 0 1 [97, 119, 97] 43 0 0x8b00 0x3100
132 122 -922 784 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 122 -920 784 75 0 1 [97, 119, 97] 18446744073709551615 1 0x8b00 0x3100
134 122 -918 784 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x3100
122 123 -942 786 70 0 1 [97, 119, 97] 48 0 0x8600 0x3100
123 123 -940 786 65 0 1 [97, 119, 97] 48 0 0x8100 0x3100
124 123 -938 786 65 0 1 [97, 119, 97] 47 0 0x8100 0x3100
125 123 -936 786 62 1 1 [63, 118, 228] 107 0 0x7e00 0xd800
```
### stream diag after fill:
[c-LoD] stream: tick 2400 65536/65536 cells, lap 1172, cone 17813 zone 884 dirty 891 (q 10913) ewin 185 rbeat 10p/0 rearm 0c/0t rescan 559995c/1909e, edits +6131p/4622c/700e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 342 forced 170, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 201095p/424b fail 0 ~0 kB/s cache 13376/24576 h187719 m13376 e0, srbeat 5639s/9q drop 37stale/0undec
[18:29:51 INFO]: [c-LoD] stream: tick 2400 65536/65536 cells, lap 1172, cone 17813 zone 884 dirty 891 (q 10913) ewin 185 rbeat 10p/0 rearm 0c/0t rescan 559995c/1909e, edits +6131p/4622c/700e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 342 forced 170, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 201095p/424b fail 0 ~0 kB/s cache 13376/24576 h187719 m13376 e0, srbeat 5639s/9q drop 37stale/0undec
- probe 0: red=0 paint=937 lod_silhouette_cols=25 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 1: red=0 paint=937 lod_silhouette_cols=25 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 2: red=0 paint=937 lod_silhouette_cols=25 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 3: red=0 paint=937 lod_silhouette_cols=25 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 4: red=0 paint=1645 lod_silhouette_cols=27 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 5: red=0 paint=1656 lod_silhouette_cols=26 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 6: red=0 paint=1805 lod_silhouette_cols=27 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 7: red=0 paint=1817 lod_silhouette_cols=27 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 8: red=0 paint=1770 lod_silhouette_cols=27 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 9: red=0 paint=1729 lod_silhouette_cols=27 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 10: red=0 paint=2355 lod_silhouette_cols=27 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 11: red=0 paint=9925 lod_silhouette_cols=33 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 12: red=0 paint=10062 lod_silhouette_cols=33 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 13: red=0 paint=9927 lod_silhouette_cols=33 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 14: red=0 paint=9927 lod_silhouette_cols=33 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 15: red=0 paint=9927 lod_silhouette_cols=33 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 16: red=0 paint=9927 lod_silhouette_cols=33 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 17: red=0 paint=9927 lod_silhouette_cols=33 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 18: red=0 paint=9927 lod_silhouette_cols=33 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 19: red=0 paint=10689 lod_silhouette_cols=33 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 20: red=0 paint=10754 lod_silhouette_cols=32 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 21: red=0 paint=10582 lod_silhouette_cols=33 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 22: red=0 paint=12523 lod_silhouette_cols=33 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 23: red=0 paint=12597 lod_silhouette_cols=32 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- phase: 5 leg-1 probes done (18:32:59)
- **FAIL:** leg-1 never saw the marker in 24 probes (red < 60 everywhere) — the edit never reached the LOD paint; this failure was SILENT before v6.31.1
### stream diag after probes:
[18:32:42 INFO]: [c-LoD] stream: tick 4100 65536/65536 cells, lap 2128, cone 38888 zone 1058 dirty 924 (q 10939) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1109169c/1964e, edits +7442p/5519c/961e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 342 forced 170, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 415445p/797b fail 0 ~125 kB/s cache 13387/24576 h402058 m13387 e0, srbeat 11368s/0q drop 43stale/0undec
[c-LoD] stream: tick 4200 65536/65536 cells, lap 2228, cone 41270 zone 1058 dirty 924 (q 10939) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1141522c/1964e, edits +7443p/5520c/961e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 342 forced 170, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 438721p/833b fail 0 ~122 kB/s cache 13387/24576 h425334 m13387 e0, srbeat 11920s/0q drop 43stale/0undec
[18:32:52 INFO]: [c-LoD] stream: tick 4200 65536/65536 cells, lap 2228, cone 41270 zone 1058 dirty 924 (q 10939) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1141522c/1964e, edits +7443p/5520c/961e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 342 forced 170, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 438721p/833b fail 0 ~122 kB/s cache 13387/24576 h425334 m13387 e0, srbeat 11920s/0q drop 43stale/0undec
- parity debug tour: 13 vantages (session-1 client, DEBUG pack — evidence only, the gate pairs the §7b world session)
- parity debug tour done: 13 shots in parity_dbg/ (the §7b world session feeds the gate)
- joined session 2 (18:35:50)
- leg-2 baseline scene px (x<940): 16443 (0 = the relog client paints no world at all)
- leg-2 renderer probe NEAR (20 m) white px: 504 raw / 504 changed-vs-baseline (> 20 = the client renders particles)
- leg-2 renderer probe FAR (80 m, marker zone) white px: 130 raw / 130 changed-vs-baseline (> 20 = no distance cut; ~0 with NEAR alive = particles beyond the client chunk radius do not render)
- leg-2 carrier probe green dust (80 m): changed-green px 0 / total changed 10484 vs baseline (> 100 green = a stable green carrier exists; total > 0 with green ~0 = the dust renders but fog washes it — a ruler problem, not a carrier problem)
- phase: 8 leg-2 relog probes (18:38:40)
- relog probe 0: red=0 scene=27061 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 1: red=0 scene=27586 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 2: red=0 scene=27314 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 3: red=0 scene=28293 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 4: red=0 scene=28230 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 5: red=0 scene=30715 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 6: red=0 scene=30981 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 7: red=0 scene=30963 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 8: red=0 scene=31006 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 9: red=0 scene=30934 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 10: red=0 scene=31050 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 11: red=0 scene=31154 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- **FAIL:** marker lost after relog — stream state is not persistent/deterministic
- phase: 9 leg-2 done (18:40:13)

### client log forensics (pipeline/particle/atlas — v6.8.2)
```
java.lang.IllegalStateException: Failed to open OpenAL device
	at net.minecraft.client.sounds.SoundEngine.reload(SoundEngine.java:110)
	at net.minecraft.server.packs.resources.SimplePreparableReloadListener.lambda$reload$1(SimplePreparableReloadListener.java:15)
	at net.minecraft.server.packs.resources.SimpleReloadInstance.lambda$prepareTasks$3(SimpleReloadInstance.java:59)
[18:35:47] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[18:35:47] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[18:35:47] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[18:35:47] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[18:35:47] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[18:35:47] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[18:35:47] [Render thread/INFO]: Created: 2048x2048x4 minecraft:textures/atlas/blocks.png-atlas
[18:35:47] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[18:35:47] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[18:35:47] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[18:35:47] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[18:35:47] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
[18:35:54] [Render thread/INFO]: Reloading ResourceManager: vanilla, vanilla, server/00000000/6b1f2f7e-9c0d-4a4b-8b0a-b10d1b4ab100
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
[18:35:55] [Render thread/ERROR]: Error starting SoundSystem. Turning off sounds & music
java.lang.IllegalStateException: Failed to open OpenAL device
	at net.minecraft.client.sounds.SoundEngine.reload(SoundEngine.java:110)
	at net.minecraft.server.packs.resources.SimplePreparableReloadListener.lambda$reload$1(SimplePreparableReloadListener.java:15)
	at net.minecraft.server.packs.resources.SimpleReloadInstance.lambda$prepareTasks$3(SimpleReloadInstance.java:59)
[18:35:55] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[18:35:55] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[18:35:55] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[18:35:55] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[18:35:55] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[18:35:55] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[18:35:55] [Render thread/INFO]: Created: 2048x2048x4 minecraft:textures/atlas/blocks.png-atlas
[18:35:56] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[18:35:56] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[18:35:56] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[18:35:56] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[18:35:56] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
```

### /clod-cells AFTER PROBES (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 59 0 0x8400 0x3100
123 122 -940 784 71 0 1 [97, 119, 97] 59 0 0x8700 0x3100
124 122 -938 784 68 0 1 [97, 119, 97] 59 0 0x8400 0x3100
125 122 -936 784 62 1 1 [63, 118, 228] 35 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 34 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 34 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 34 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 33 0 0x8000 0x5c00
130 122 -926 784 66 0 1 [134, 183, 131] 33 0 0x8200 0x5c00
131 122 -924 784 75 0 1 [97, 119, 97] 58 0 0x8b00 0x3100
132 122 -922 784 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 122 -920 784 75 0 1 [97, 119, 97] 18446744073709551615 1 0x8b00 0x3100
134 122 -918 784 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x3100
122 123 -942 786 70 0 1 [97, 119, 97] 59 0 0x8600 0x3100
123 123 -940 786 65 0 1 [97, 119, 97] 59 0 0x8100 0x3100
124 123 -938 786 65 0 1 [97, 119, 97] 59 0 0x8100 0x3100
125 123 -936 786 62 1 1 [63, 118, 228] 35 0 0x7e00 0xd800
```
- edit channel: 0 linkage errors, 0 failed event dispatches (both must be 0)
- phase: 6 parity world boot (18:40:13)
- parity world: view-distance=8 (the voxy leg's server value)
- parity world: joined session 3 (18:40:51)
- parity world: scan populated (cells=65536; the march paints from these buffers)
- parity world tour: 13 vantages (production pack, vd 8, shared structures)
- coverage sweeps: 72 eye passes = 36 positions (12 az x r144/200 + 12 off-az x r172, y=160) x 2 looks (outward+inward), settle-by-laps >= 6 s (q stable 3x1s), deadline 12 s/pos, no shots — the v6.26.0 data-coverage lever + v6.26.6 lottery kill; probe rides parity/view_probe.md
- parity drain-settle TIMEOUT: queue 20564 -> 20561 > 11500 after 150s — continuing anyway; the queue did not drain without eye proximity (priority-lane evidence, the 01/05/10 dip attribution reads from the tour deltas)
- pre-tour pass done: 13 vantages x 10s (the tour now reads the second OR-merge lap; the rim/dip transport race reads from the tour deltas)
- parity world tour done: 13 shots, 0 night-guard renames in parity/ (the gate pairs voxy/<name>.png vs parity/<name>.png)
- phase: 7 parity tour done (19:02:43)
- sparse re-beat channel at tour end: streamed=38995 q=0 dropped_stale=25 dropped_undecodable=4646 (frac=8 boot; undecodable-dominant = the rim words die in their decodability window, stale-dominant = edit churn owns the cells)
### parity world stream counters (session 3, tail + maxima)
[c-LoD] stream: tick 13200 65536/65536 cells, lap 7743, cone 124517 zone 19048 dirty 6829 (q 20555) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 9013803c/1744e, edits +18100p/13722c/2851e drop 0, spawned 0 / failed 0, cam0 yaw -180 pitch 5, wish 0 forced 915, native 28224c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 1443868p/2056b fail 0 ~0 kB/s cache 21523/24576 h1110869 m332999 e311476, srbeat 38995s/0q drop 25stale/4646undec
[19:02:43 INFO]: [c-LoD] stream: tick 13200 65536/65536 cells, lap 7743, cone 124517 zone 19048 dirty 6829 (q 20555) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 9013803c/1744e, edits +18100p/13722c/2851e drop 0, spawned 0 / failed 0, cam0 yaw -180 pitch 5, wish 0 forced 915, native 28224c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 1443868p/2056b fail 0 ~0 kB/s cache 21523/24576 h1110869 m332999 e311476, srbeat 38995s/0q drop 25stale/4646undec
- parity stream max cone: 124517
- parity stream max zone: 19048
- parity stream max wire bundle: bundle
### parity world capture lens (debug chain on the §7b world)
lens_red=0 lens_black=0 lens_water=0 lens_dark=0 lens_zero=0 rot=none scan=0 carriers=0
- parity mesh probe @png_y=2: x306=(151,184,255) x307=(151,184,255) x308=(151,184,255) x309h=(151,184,255) x310c=(151,184,255)
- parity mesh probe @png_y=717: x306=(44,62,51) x307=(66,90,72) x308=(83,109,89) x309h=(83,109,89) x310c=(85,112,90)
- parity mesh engagement: magenta-ish=0 of 921600 sampled px
- mask8: the un-fog engagement masks shot on the PARITY WORLD (vd 8) — parity_dbg8/01m|07m|10m
- mask8 01m: green=4784 red=0 blue=100384 dim=240368 (px, sampled /16)
- mask8 07m: green=11056 red=0 blue=221808 dim=5872 (px, sampled /16)
- mask8 10m: green=5088 red=144 blue=215920 dim=220928 (px, sampled /16)
- mesh probe @png_y=2: x306=(255,255,255) x307=(13,254,0) x308=(0,254,0) x309h=(4,0,0) x310c=(0,0,0)
- mesh probe @png_y=717: x306=(255,255,255) x307=(13,254,0) x308=(0,254,0) x309h=(4,0,0) x310c=(0,0,0)
- mesh engagement: magenta-ish=1136 of 921600 sampled px (the vanilla-path paint coverage)
- mask8 fog state @png_y=2: raw (300)=(151,184,255) (301)=(151,184,255)
- mask8 fog state @png_y=717: raw (300)=(0,128,8) (301)=(179,206,255)
- mask8 fog state decoded: envStart=0 envEnd=1024 rdEnd=128 fogColor=(179,206,255)
- buffer dump 10m: marked h=27/2048 c=0/2048; height windows r0<142:20 r1:0 r2>=202:7 maxByte=255
- buffer dump 10m: RANK-2-WINDOW HEIGHTS (the fake-wall class): (+86,-28)h=191, (+2,-10)h=162, (+110,-4)h=191, (+104,-2)h=191, (+26,+2)h=151, (+104,+2)h=191, (-2,+24)h=140
- buffer dump 10m: color idx0(black)=0 water/height mismatches=0 of 0 pairs
- buffer dump 01m: marked h=0/2048 c=0/2048; height windows r0<142:0 r1:0 r2>=202:0 maxByte=0
- buffer dump 01m: color idx0(black)=0 water/height mismatches=0 of 0 pairs
- rim mask: scan populated (cells=65536; the state paint reads the same buffers the march reads)
- phase: 10 rim mask lane (19:07:00)
- rim mask: decode-state frames shot (rim_mask/00|01|12 — GREEN full decode / RED colorless / BLUE heightless; 00 = the leg-1 tower eye, 01 = the near-tower contrast, 12 = the tower re-read; the inherited 07/12 vantages sat 760 blocks outside the streamed disk and 01 sat INSIDE the marker column — their earlier verdicts were classifier noise, purged in v6.33.1)
- rim mask 00: green=1248 red=89248 blue=87152 yellow=4704 (px, sampled /16) -> DOMINANT RED: rim = COLORLESS class — the color words die client-side (pool competition / fog strobe margin / the v6.23 HeightRef capture gate); heights ARE decoded
- rim mask 01: green=5760 red=140160 blue=134288 yellow=8528 (px, sampled /16) -> DOMINANT RED: rim = COLORLESS class — the color words die client-side (pool competition / fog strobe margin / the v6.23 HeightRef capture gate); heights ARE decoded
- rim mask 12: green=11504 red=115984 blue=112272 yellow=11504 (px, sampled /16) -> DOMINANT RED: rim = COLORLESS class — the color words die client-side (pool competition / fog strobe margin / the v6.23 HeightRef capture gate); heights ARE decoded

### boot / stream
```
[19:06:15 INFO]: [c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 127551 bytes, sha1 9044d9e9174ffbeb08caf7427c6d82dfc20eec0c — zero textures, zero world data, one push per join
[19:06:15 INFO]: [c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 127551 bytes, sha1 9044d9e9174ffbeb08caf7427c6d82dfc20eec0c — zero textures, zero world data, one push per join
[c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[19:07:34 INFO]: [c-LoD] stream: tick 700 65536/65536 cells, lap 566, cone 10295 zone 843 dirty 18 (q 882) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 396982c/53e, edits +8p/8c/4e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 65 forced 821, native 4208c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 119240p/158b fail 0 ~119 kB/s cache 13585/24576 h105655 m13585 e0, srbeat 3876s/197q drop 0stale/0undec
[c-LoD] stream: tick 700 65536/65536 cells, lap 566, cone 10295 zone 843 dirty 18 (q 882) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 396982c/53e, edits +8p/8c/4e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 65 forced 821, native 4208c/954ch fail 0 (3w q 0) helper on, trees 954ch/fail 0, wire bundle 119240p/158b fail 0 ~119 kB/s cache 13585/24576 h105655 m13585 e0, srbeat 3876s/197q drop 0stale/0undec
```
### sessions
```
[19:06:35 INFO]: System chat: Stepan joined the game
[19:07:41 INFO]: System chat: Stepan left the game
```
### client log tail
```
[19:07:40] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[19:07:40] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[19:07:40] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[19:07:40] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[19:07:40] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[19:07:40] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:move_player_pos] net.minecraft.network.protocol.game.ServerboundMovePlayerPacket$Pos -> 26 bytes
[19:07:40] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[19:07:40] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[19:07:40] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[19:07:40] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[19:07:40] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[19:07:40] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[19:07:40] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[19:07:40] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[19:07:40] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[19:07:40] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[19:07:40] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:keep_alive] net.minecraft.network.protocol.common.ClientboundKeepAlivePacket -> 9 bytes
[19:07:40] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:keep_alive] net.minecraft.network.protocol.common.ServerboundKeepAlivePacket -> 9 bytes
[19:07:40] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:set_time] net.minecraft.network.protocol.game.ClientboundSetTimePacket -> 10 bytes
[19:07:41] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:entity_event] net.minecraft.network.protocol.game.ClientboundEntityEventPacket -> 6 bytes
[19:07:41] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[19:07:41] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[19:07:41] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[19:07:41] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[19:07:41] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[19:07:41] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[19:07:41] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[19:07:41] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[19:07:41] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[19:07:41] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
```
### server log tail
```
[19:07:42 INFO]: [ChunkHolderManager] Saved 997 block chunks, 975 entity chunks, 0 poi chunks in world 'minecraft:overworld' in 0.72s
[19:07:42 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:overworld'
[19:07:42 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:overworld'
[19:07:42 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[19:07:42 INFO]: [ChunkHolderManager] Waiting 60s for chunk system to halt for world 'minecraft:the_nether'
[19:07:42 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:the_nether'
[19:07:42 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_nether'
[19:07:42 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_nether' in 0.00s
[19:07:42 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_nether'
[19:07:42 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_nether'
[19:07:42 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[19:07:42 INFO]: [ChunkHolderManager] Waiting 60s for chunk system to halt for world 'minecraft:the_end'
[19:07:42 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:the_end'
[19:07:42 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_end'
[19:07:42 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_end' in 0.00s
[19:07:42 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_end'
[19:07:42 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_end'
[19:07:42 INFO]: Waiting for all RegionFile I/O tasks to complete...
[19:07:42 INFO]: All RegionFile I/O tasks to complete
[19:07:42 INFO]: [MoonriseCommon] Awaiting termination of worker pool for up to 60s...
[19:07:42 INFO]: [MoonriseCommon] Awaiting termination of I/O pool for up to 60s...
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[launcher] server exited with code 143
```
### shots
```
-rw-r--r-- 1 runner runner  4179 Nov 12 18:30 lens_12_after_edit_probe4.png
-rw-r--r-- 1 runner runner  4401 Nov 12 18:30 lens_12_after_edit_probe5.png
-rw-r--r-- 1 runner runner  4123 Nov 12 18:30 lens_12_after_edit_probe6.png
-rw-r--r-- 1 runner runner  4333 Nov 12 18:30 lens_12_after_edit_probe7.png
-rw-r--r-- 1 runner runner  4174 Nov 12 18:30 lens_12_after_edit_probe8.png
-rw-r--r-- 1 runner runner  4436 Nov 12 18:31 lens_12_after_edit_probe9.png
-rw-r--r-- 1 runner runner  3650 Nov 12 18:38 lens_13_after_relog_probe0.png
-rw-r--r-- 1 runner runner  4256 Nov 12 18:38 lens_13_after_relog_probe1.png
-rw-r--r-- 1 runner runner  4596 Nov 12 18:39 lens_13_after_relog_probe10.png
-rw-r--r-- 1 runner runner  4686 Nov 12 18:40 lens_13_after_relog_probe11.png
-rw-r--r-- 1 runner runner  4336 Nov 12 18:38 lens_13_after_relog_probe2.png
-rw-r--r-- 1 runner runner  4214 Nov 12 18:39 lens_13_after_relog_probe3.png
-rw-r--r-- 1 runner runner  4541 Nov 12 18:39 lens_13_after_relog_probe4.png
-rw-r--r-- 1 runner runner  4268 Nov 12 18:39 lens_13_after_relog_probe5.png
-rw-r--r-- 1 runner runner  4230 Nov 12 18:39 lens_13_after_relog_probe6.png
-rw-r--r-- 1 runner runner  4484 Nov 12 18:39 lens_13_after_relog_probe7.png
-rw-r--r-- 1 runner runner  4520 Nov 12 18:39 lens_13_after_relog_probe8.png
-rw-r--r-- 1 runner runner  4423 Nov 12 18:39 lens_13_after_relog_probe9.png
drwxr-xr-x 2 runner runner  4096 Nov 12 19:02 lens_parity
drwxr-xr-x 2 runner runner  4096 Nov 12 19:02 parity
drwxr-xr-x 2 runner runner  4096 Nov 12 18:35 parity_dbg
drwxr-xr-x 2 runner runner  4096 Nov 12 19:05 parity_dbg8
-rw-r--r-- 1 runner runner  6760 Nov 12 18:40 probe_counts.txt
drwxr-xr-x 2 runner runner  4096 Nov 12 19:07 rim_mask
-rw-r--r-- 1 runner runner 35074 Nov 12 19:07 summary.md
```

**exit=1** failures=2 leg1=0 leg2=0
