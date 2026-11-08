### c-LoD visual run (-930,796, marker=red_concrete, vantage -1005,150 yaw 270)
- GL:     Vendor: Mesa (0xffffffff);    Device: llvmpipe (LLVM 20.1.2, 256 bits) (0xffffffff);OpenGL renderer string: llvmpipe (LLVM 20.1.2, 256 bits);OpenGL core profile version string: 4.5 (Core Profile) Mesa 25.2.8-0ubuntu0.24.04.4;
- boot + gates OK (10:34:46)
- sky-grid armed
- joined session 1 (10:35:13)
- stream live: cells=65536
- terrain range: 2 terraced mountains at (CX-4,CZ-20)/(CX-2,CZ+16), tops 185/190, grass terraces + stone caps, 66..82 m from the eye (inside the 83.5 m band, chunk edge 64 m)
- scan complete=1 (cells=cells=65536/65536)
- paint settled=1 (wire 54152p)
- baseline try 1: drift=0 px
- lod_silhouette_cols=0 (LOD paint ON SCREEN at 75 m if > 20 — the march-drawn marker column; vanilla cannot render there, chunk edge 64 m; #255 re-measures 49, the ZONE_FAR=48 regression #256 and the control read 0)
- 0 terrain-green px in the sky region (LOD MOUNTAINS on screen if > 5000; the range paints ColLow-green on low terraces via the fallback palette; flat-world baselines and the pack-free control read ~0)
- burst_close white px: 0 (renderer alive if > 50)
- burst_far white px: 0 (renderer alive in the marker zone if > 20)
- cut walk 40 m: 0 white px (> 20 = rendered)
- cut walk 60 m: 0 white px (> 20 = rendered)
- cut walk 100 m: 0 white px (> 20 = rendered)
- measured far-render cut: none m (G4 flap instrument; the decode proof needs >= 83 m from the y=151.6 eye)
- burst_close retry white px: 0 (preflight passes if > 50)
- preflight lens_zero=0 (1 = capture NEVER committed = no particle EVER rendered)
- preflight stage eyes: rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0
- fill issued: 5x111x5 red_concrete at -930,796 (zone forceloaded ±48)

### /clod-cells AFTER FILL (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 1354 0 0x8400 0x3100
123 122 -940 784 71 0 1 [97, 119, 97] 1354 0 0x8700 0x3100
124 122 -938 784 68 0 1 [97, 119, 97] 1354 0 0x8400 0x3100
125 122 -936 784 62 1 1 [63, 118, 228] 1354 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 1354 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 1347 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 1346 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 1346 0 0x8000 0x5c00
130 122 -926 784 66 0 1 [134, 183, 131] 1345 0 0x8200 0x5c00
131 122 -924 784 75 0 1 [97, 119, 97] 1345 0 0x8b00 0x3100
132 122 -922 784 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 122 -920 784 75 0 1 [97, 119, 97] 18446744073709551615 1 0x8b00 0x3100
134 122 -918 784 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x3100
122 123 -942 786 70 0 1 [97, 119, 97] 1354 0 0x8600 0x3100
123 123 -940 786 65 0 1 [97, 119, 97] 1354 0 0x8100 0x3100
124 123 -938 786 65 0 1 [97, 119, 97] 1354 0 0x8100 0x3100
125 123 -936 786 62 1 1 [63, 118, 228] 1354 0 0x7e00 0xd800
```
### stream diag after fill:
[c-LoD] stream: tick 2100 65536/65536 cells, lap 600, cone 5306 zone 651 dirty 802 (q 12812) ewin 46 rbeat 0p/0 rearm 0c/0t rescan 459449c/873e, edits +5236p/3935c/517e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 284 forced 228, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 54584p/133b fail 0 ~0 kB/s cache 11466/24576 h43118 m11466 e0
[10:38:26 INFO]: [c-LoD] stream: tick 2100 65536/65536 cells, lap 600, cone 5306 zone 651 dirty 802 (q 12812) ewin 46 rbeat 0p/0 rearm 0c/0t rescan 459449c/873e, edits +5236p/3935c/517e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 284 forced 228, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 54584p/133b fail 0 ~0 kB/s cache 11466/24576 h43118 m11466 e0
- probe 0: red=129083 paint=16054 lod_silhouette_cols=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- **LEG 1 PASS** (zero relogs)
### stream diag after probes:
[10:38:26 INFO]: [c-LoD] stream: tick 2100 65536/65536 cells, lap 600, cone 5306 zone 651 dirty 802 (q 12812) ewin 46 rbeat 0p/0 rearm 0c/0t rescan 459449c/873e, edits +5236p/3935c/517e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 284 forced 228, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 54584p/133b fail 0 ~0 kB/s cache 11466/24576 h43118 m11466 e0
[c-LoD] stream: tick 2200 65536/65536 cells, lap 600, cone 5306 zone 651 dirty 802 (q 12822) ewin 206 rbeat 0p/0 rearm 0c/0t rescan 491395c/892e, edits +5664p/4274c/644e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 284 forced 228, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 54584p/133b fail 0 ~0 kB/s cache 11466/24576 h43118 m11466 e0
[10:38:37 INFO]: [c-LoD] stream: tick 2200 65536/65536 cells, lap 600, cone 5306 zone 651 dirty 802 (q 12822) ewin 206 rbeat 0p/0 rearm 0c/0t rescan 491395c/892e, edits +5664p/4274c/644e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 284 forced 228, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 54584p/133b fail 0 ~0 kB/s cache 11466/24576 h43118 m11466 e0
- parity debug tour: 12 vantages (session-1 client, DEBUG pack — evidence only, the gate pairs the §7b world session)
- parity debug tour done: 12 shots in parity_dbg/ (the §7b world session feeds the gate)
- joined session 2 (10:41:20)
- leg-2 baseline scene px (x<940): 128806 (0 = the relog client paints no world at all)
- leg-2 renderer probe NEAR (20 m) white px: 0 raw / 0 changed-vs-baseline (> 20 = the client renders particles)
- leg-2 renderer probe FAR (80 m, marker zone) white px: 0 raw / 0 changed-vs-baseline (> 20 = no distance cut; ~0 with NEAR alive = particles beyond the client chunk radius do not render)
- leg-2 carrier probe green dust (80 m): changed-green px 0 / total changed 2 vs baseline (> 100 green = a stable green carrier exists; total > 0 with green ~0 = the dust renders but fog washes it — a ruler problem, not a carrier problem)
- relog probe 0: red=129085 scene=128808 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red >= 60; scene > 0 = the client paints a world at all)
- **LEG 2 PASS** (marker persisted over relog)

### client log forensics (pipeline/particle/atlas — v6.8.2)
```
ALSA lib confmisc.c:1342:(snd_func_refer) error evaluating name
ALSA lib conf.c:5208:(_snd_config_evaluate) function snd_func_refer returned error: No such file or directory
ALSA lib conf.c:5731:(snd_config_expand) Evaluate error: No such file or directory
[10:41:18] [Render thread/ERROR]: Error starting SoundSystem. Turning off sounds & music
java.lang.IllegalStateException: Failed to open OpenAL device
	at net.minecraft.client.sounds.SoundEngine.reload(SoundEngine.java:110)
	at net.minecraft.server.packs.resources.SimplePreparableReloadListener.lambda$reload$1(SimplePreparableReloadListener.java:15)
	at net.minecraft.server.packs.resources.SimpleReloadInstance.lambda$prepareTasks$3(SimpleReloadInstance.java:59)
[10:41:18] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[10:41:18] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[10:41:18] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[10:41:18] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[10:41:18] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[10:41:18] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[10:41:18] [Render thread/INFO]: Created: 2048x2048x4 minecraft:textures/atlas/blocks.png-atlas
[10:41:18] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[10:41:18] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[10:41:18] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[10:41:18] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[10:41:18] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
[10:41:25] [Render thread/INFO]: Reloading ResourceManager: vanilla, vanilla, server/00000000/6b1f2f7e-9c0d-4a4b-8b0a-b10d1b4ab100
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
[10:41:27] [Render thread/ERROR]: Error starting SoundSystem. Turning off sounds & music
java.lang.IllegalStateException: Failed to open OpenAL device
	at net.minecraft.client.sounds.SoundEngine.reload(SoundEngine.java:110)
	at net.minecraft.server.packs.resources.SimplePreparableReloadListener.lambda$reload$1(SimplePreparableReloadListener.java:15)
	at net.minecraft.server.packs.resources.SimpleReloadInstance.lambda$prepareTasks$3(SimpleReloadInstance.java:59)
[10:41:27] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[10:41:27] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[10:41:27] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[10:41:27] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[10:41:27] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[10:41:27] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[10:41:27] [Render thread/INFO]: Created: 2048x2048x4 minecraft:textures/atlas/blocks.png-atlas
[10:41:27] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[10:41:27] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[10:41:27] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[10:41:27] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[10:41:27] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
```

### /clod-cells AFTER PROBES (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 2557 0 0x8400 0x3100
123 122 -940 784 71 0 1 [97, 119, 97] 2557 0 0x8700 0x3100
124 122 -938 784 68 0 1 [97, 119, 97] 2557 0 0x8400 0x3100
125 122 -936 784 62 1 1 [63, 118, 228] 2557 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 2557 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 2557 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 2557 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 2557 0 0x8000 0x5c00
130 122 -926 784 66 0 1 [134, 183, 131] 2557 0 0x8200 0x5c00
131 122 -924 784 75 0 1 [97, 119, 97] 2557 0 0x8b00 0x3100
132 122 -922 784 77 0 1 [97, 119, 97] 2558 0 0x8d00 0x3100
133 122 -920 784 75 0 1 [97, 119, 97] 2558 0 0x8b00 0x3100
134 122 -918 784 73 0 1 [97, 119, 97] 2558 0 0x8900 0x3100
122 123 -942 786 70 0 1 [97, 119, 97] 2557 0 0x8600 0x3100
123 123 -940 786 65 0 1 [97, 119, 97] 2557 0 0x8100 0x3100
124 123 -938 786 65 0 1 [97, 119, 97] 2557 0 0x8100 0x3100
125 123 -936 786 62 1 1 [63, 118, 228] 2557 0 0x7e00 0xd800
```
- edit channel: 0 linkage errors, 0 failed event dispatches (both must be 0)
- parity world: view-distance=8 (the voxy leg's server value)
- parity world: joined session 3 (10:44:58)
- parity world: scan populated (cells=65536; the march paints from these buffers)
- parity world tour: 12 vantages (production pack, vd 8, shared structures)
- parity world tour done: 12 shots in parity/ (the gate pairs voxy/<name>.png vs parity/<name>.png)
