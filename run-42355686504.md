### c-LoD visual run (-930,796, marker=red_concrete, vantage -1005,150 yaw 270)
- GL:     Vendor: Mesa (0xffffffff);    Device: llvmpipe (LLVM 20.1.2, 256 bits) (0xffffffff);OpenGL renderer string: llvmpipe (LLVM 20.1.2, 256 bits);OpenGL core profile version string: 4.5 (Core Profile) Mesa 25.2.8-0ubuntu0.24.04.4;
- boot + gates OK (08:31:01)
- sky-grid armed
- joined session 1 (08:31:22)
- stream live: cells=65536
- scan complete=1 (cells=cells=65536/65536)
- paint settled=0 (wire 708235p)
- baseline try 1: drift=502 px
- burst_close white px: 524 (renderer alive if > 50)
- burst_far white px: 0 (renderer alive in the marker zone if > 20)
- leg-1 green dust (80 m, leg-2 geometry) changed-green 0 / total 223 vs predust (> 100 green = dust renders on a live client — the leg-2 zero is session-specific; green 0 with total ~0 = dust never renders here, the carrier type IS the root cause)
- leg-1 white dust (20 m NEAR, proven-render spot) changed-white 14 / total 255 vs green-dust shot (> 20 = the dust TYPE renders; green 0 + white > 0 = color-specific break, green 0 + white 0 = dust-type break)
- leg-1 CARRIER (happy_villager) 20 m NEAR: changed-green 0 / total 1756 vs white-dust shot (> 20 = the carrier renders on the healthy client — the leg-2 villager twin's reference; ~0 with end_rod alive = the carrier type itself is dead on BOTH legs — the flap at its extreme, carrier must change)
- leg-1 STREAM-SHAPE end_rod (count=0 x120 NEAR — the engine's exact per-dot packet): changed-white 715 / total 1396 vs the villager NEAR shot (> 20 = the per-dot path renders on the healthy client — the leg-2 twin's reference; ~0 with the bursts alive = the count=0 path is dead on BOTH legs — P0: batch the dots)
- leg-1 STREAM-SHAPE carrier (happy_villager count=0 x120 NEAR): changed-green 0 / total 1126 vs the end_rod stream shot (the healthy twin for the leg-2 per-dot carrier read: ~ 0 green = the 2px color net erases the per-dot carrier hue on a HEALTHY packed client — the expected design read; > 20 = the net misses the per-dot geometry — dot-size/pool class on the healthy leg)
- fill issued: 5x111x5 red_concrete at -930,796 (zone forceloaded ±48)

### /clod-cells AFTER FILL (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 41 0 0x8400 0x3100
123 122 -940 784 71 0 1 [97, 119, 97] 41 0 0x8700 0x3100
124 122 -938 784 68 0 1 [97, 119, 97] 41 0 0x8400 0x3100
125 122 -936 784 62 1 1 [63, 118, 228] 41 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 41 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 35 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 35 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 70 0 0x8000 0x5c00
130 122 -926 784 66 0 1 [134, 183, 131] 70 0 0x8200 0x5c00
131 122 -924 784 75 0 1 [97, 119, 97] 70 0 0x8b00 0x3100
132 122 -922 784 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 122 -920 784 75 0 1 [97, 119, 97] 18446744073709551615 1 0x8b00 0x3100
134 122 -918 784 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x3100
122 123 -942 786 70 0 1 [97, 119, 97] 41 0 0x8600 0x3100
123 123 -940 786 65 0 1 [97, 119, 97] 41 0 0x8100 0x3100
124 123 -938 786 65 0 1 [97, 119, 97] 41 0 0x8100 0x3100
125 123 -936 786 62 1 1 [63, 118, 228] 41 0 0x7e00 0xd800
```
### stream diag after fill:
[c-LoD] stream: tick 5100 65536/65536 cells, lap 4862, cone 102928 zone 640 dirty 1034 (q 11020) ewin 246 rbeat 0p/1 rearm 0c/0t rescan 1284443c/2497e, edits +3593p/2695c/743e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 345 forced 167, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 834262p/2708b fail 0 ~0 kB/s cache 14871/24576 h819391 m14871 e0
[08:39:42 INFO]: [c-LoD] stream: tick 5100 65536/65536 cells, lap 4862, cone 102928 zone 640 dirty 1034 (q 11020) ewin 246 rbeat 0p/1 rearm 0c/0t rescan 1284443c/2497e, edits +3593p/2695c/743e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 345 forced 167, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 834262p/2708b fail 0 ~0 kB/s cache 14871/24576 h819391 m14871 e0
- probe 0: red=1631 paint=15905 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300)
- **LEG 1 PASS** (zero relogs)
### stream diag after probes:
[08:39:32 INFO]: [c-LoD] stream: tick 5000 65536/65536 cells, lap 4862, cone 102928 zone 640 dirty 1021 (q 10987) ewin 246 rbeat 0p/0 rearm 0c/0t rescan 1252898c/2475e, edits +2974p/2213c/543e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 345 forced 167, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 834145p/2707b fail 0 ~103 kB/s cache 14843/24576 h819302 m14843 e0
[c-LoD] stream: tick 5100 65536/65536 cells, lap 4862, cone 102928 zone 640 dirty 1034 (q 11020) ewin 246 rbeat 0p/1 rearm 0c/0t rescan 1284443c/2497e, edits +3593p/2695c/743e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 345 forced 167, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 834262p/2708b fail 0 ~0 kB/s cache 14871/24576 h819391 m14871 e0
[08:39:42 INFO]: [c-LoD] stream: tick 5100 65536/65536 cells, lap 4862, cone 102928 zone 640 dirty 1034 (q 11020) ewin 246 rbeat 0p/1 rearm 0c/0t rescan 1284443c/2497e, edits +3593p/2695c/743e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 345 forced 167, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 834262p/2708b fail 0 ~0 kB/s cache 14871/24576 h819391 m14871 e0
- leg-2 CACHED-PACK CONTROL: removed the server-pack cache (2 files) — leg 2 re-downloads the pack (fresh load path, same as leg 1)
- joined session 2 (08:40:04)
- leg-2 scene dead at first probe (0 px): the arm race hit the relog — tier flip to re-arm the chains (out -> Veil, back -> Full)
- leg-2 tier flip done: chains re-armed post-reload; baseline follows
- leg-2 baseline scene px (x<940): 0 (0 = the relog client paints no world at all)
- leg-2 renderer probe NEAR (20 m) white px: 562 raw / 562 changed-vs-baseline (> 20 = the client renders particles)
- leg-2 renderer probe FAR (80 m, marker zone) white px: 0 raw / 0 changed-vs-baseline (> 20 = no distance cut; ~0 with NEAR alive = particles beyond the client chunk radius do not render)
- leg-2 carrier probe green dust (80 m): changed-green px 0 / total changed 0 vs baseline (> 100 green = a stable green carrier exists; total > 0 with green ~0 = the dust renders but fog washes it — a ruler problem, not a carrier problem)
- leg-2 type matrix white dust NEAR (20 m): changed-white 19 / total 126 vs green-dust shot (> 20 = dust renders on the relog client NEAR — the FAR zero is distance/fog; 0 with the end_rod NEAR burst alive (495/479) = the dust TYPE is unrendered on leg 2)
- leg-2 type matrix happy_villager NEAR (20 m): changed-green 0 / total 126 vs white-dust shot (the old carrier on the relog client; total is the signal — the eraser can eat green)
- leg-2 plane control end_rod at Y=120 (80 m, the hword plane): changed-white 0 / total 0 vs villager shot (> 20 = the planes render — the break is dust-type-only; ~0 with end_rod@150 alive (495/479) = the break rides the plane/depth too)
- leg-2 plane x type cross: white dust at Y=120 (80 m, the hword plane — the end_rod@Y120 burst with ONLY the type swapped): changed-white 0 / total 0 vs the end_rod@Y120 shot (> 20 with end_rod@120 alive = dust renders at the exact engine geometry — the dead stream points at the engine spawn/packet path, not the client renderer; ~0 = the type x plane interaction, move LAYER_Y or the per-plane carrier)
- leg-2 cword plane: white dust at Y=80 (80 m, the cword plane, same burst geometry): changed-white 0 / total 0 vs the dust@Y120 shot (> 20 = the cword plane renders dust too — planes fully exonerated for dust; ~0 with dust@120 > 20 = the Y=80 plane specifically loses (deepest below-horizon) — drop or move it; both ~0 = the whole plane depth loses on leg-2, reversed-Z class)
- leg-2 ERASER TEST: CARRIER (happy_villager) at Y=120 in-band (75 m, the hword plane): changed-green 0 / total 0 vs the dust@Y80 shot (~ 0 = the eraser is ALIVE on leg 2 — stream dots die pre-display; > 20 = the eraser is BLIND on leg 2 — the capture tail is down)
- leg-2 STREAM-SHAPE end_rod (count=0 x120 NEAR — the engine's exact per-dot packet, eraser-immune): changed-white 765 / total 765 vs the villager@Y120 shot (> 20 with the NEAR burst alive = the per-dot path renders — shape exonerated, suspects narrow to type/bundle/cadence; ~0 with the burst alive = the count=0 path is DEAD on the relog client — the stream dies by packet shape, the engine must batch dots)
- leg-2 STREAM-SHAPE carrier (happy_villager count=0 x120 NEAR): changed-green 0 / total 765 vs the end_rod stream shot (read against the leg-1 twin: leg-2 ~ leg-1 = the per-dot carrier path behaves like the healthy leg — carrier exonerated at NEAR; leg-2 green > 20 with leg-1 ~ 0 = the eraser is blind on leg 2 at per-dot geometry; leg-2 ~ 0 with end_rod stream alive = the per-dot CARRIER path specifically dies — type/pool class)
- LEG-2 SCENE DEAD: the relog client paints ZERO scene pixels (x<940) — pure sky + lens panels; the fresh-session capture chain never commits (renderer-dead class, #227/#228); these probes measure a worldless client, not the transport
- relog probe 0: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=1 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 1: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 2: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 3: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 4: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 5: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 6: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 7: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 8: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 9: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 10: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 11: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=4 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- **FAIL:** leg-2 painted no scene in any probe — the fresh-session capture chain never commits (renderer-dead class of #227/#228); the relog verdict is N/A until the chain arms on a fresh client

### client log forensics (pipeline/particle/atlas — v6.8.2)
```
ALSA lib confmisc.c:1342:(snd_func_refer) error evaluating name
ALSA lib conf.c:5208:(_snd_config_evaluate) function snd_func_refer returned error: No such file or directory
ALSA lib conf.c:5731:(snd_config_expand) Evaluate error: No such file or directory
[08:40:03] [Render thread/ERROR]: Error starting SoundSystem. Turning off sounds & music
java.lang.IllegalStateException: Failed to open OpenAL device
	at net.minecraft.client.sounds.SoundEngine.reload(SoundEngine.java:110)
	at net.minecraft.server.packs.resources.SimplePreparableReloadListener.lambda$reload$1(SimplePreparableReloadListener.java:15)
	at net.minecraft.server.packs.resources.SimpleReloadInstance.lambda$prepareTasks$3(SimpleReloadInstance.java:59)
[08:40:03] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[08:40:03] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[08:40:03] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[08:40:03] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[08:40:03] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[08:40:03] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[08:40:03] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/blocks.png-atlas
[08:40:03] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[08:40:03] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[08:40:03] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[08:40:03] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[08:40:03] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
[08:40:09] [Render thread/INFO]: Reloading ResourceManager: vanilla, vanilla, server/00000000/6b1f2f7e-9c0d-4a4b-8b0a-b10d1b4ab100
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
[08:40:11] [Render thread/ERROR]: Error starting SoundSystem. Turning off sounds & music
java.lang.IllegalStateException: Failed to open OpenAL device
	at net.minecraft.client.sounds.SoundEngine.reload(SoundEngine.java:110)
	at net.minecraft.server.packs.resources.SimplePreparableReloadListener.lambda$reload$1(SimplePreparableReloadListener.java:15)
	at net.minecraft.server.packs.resources.SimpleReloadInstance.lambda$prepareTasks$3(SimpleReloadInstance.java:59)
[08:40:11] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[08:40:11] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[08:40:11] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[08:40:11] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[08:40:11] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[08:40:11] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[08:40:11] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/blocks.png-atlas
[08:40:11] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[08:40:11] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[08:40:11] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[08:40:11] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[08:40:11] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
```

### /clod-cells AFTER PROBES (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 21 0 0x8400 0x3100
123 122 -940 784 71 0 1 [97, 119, 97] 21 0 0x8700 0x3100
124 122 -938 784 68 0 1 [97, 119, 97] 21 0 0x8400 0x3100
125 122 -936 784 62 1 1 [63, 118, 228] 21 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 21 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 21 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 21 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 21 0 0x8000 0x5c00
130 122 -926 784 66 0 1 [134, 183, 131] 21 0 0x8200 0x5c00
131 122 -924 784 75 0 1 [97, 119, 97] 21 0 0x8b00 0x3100
132 122 -922 784 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 122 -920 784 75 0 1 [97, 119, 97] 18446744073709551615 1 0x8b00 0x3100
134 122 -918 784 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x3100
122 123 -942 786 70 0 1 [97, 119, 97] 21 0 0x8600 0x3100
123 123 -940 786 65 0 1 [97, 119, 97] 21 0 0x8100 0x3100
124 123 -938 786 65 0 1 [97, 119, 97] 21 0 0x8100 0x3100
125 123 -936 786 62 1 1 [63, 118, 228] 21 0 0x7e00 0xd800
```
- edit channel: 0 linkage errors, 0 failed event dispatches (both must be 0)
