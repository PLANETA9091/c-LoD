### c-LoD visual run (-930,796, marker=red_concrete, vantage -1005,150 yaw 270)
- GL:     Vendor: Mesa (0xffffffff);    Device: llvmpipe (LLVM 20.1.2, 256 bits) (0xffffffff);OpenGL renderer string: llvmpipe (LLVM 20.1.2, 256 bits);OpenGL core profile version string: 4.5 (Core Profile) Mesa 25.2.8-0ubuntu0.24.04.4;
- boot + gates OK (18:22:39)
- sky-grid armed
- joined session 1 (18:22:58)
- stream live: cells=65536
- scan complete=1 (cells=cells=65536/65536)
- paint settled=0 (wire 709306p)
- baseline try 1: drift=386 px
- burst_close white px: 546 (renderer alive if > 50)
- burst_far white px: 0 (renderer alive in the marker zone if > 20)
- fill issued: 5x111x5 red_concrete at -930,796 (zone forceloaded ±48)

### /clod-cells AFTER FILL (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 56 0 0x8400 0x3100
123 122 -940 784 71 0 1 [97, 119, 97] 56 0 0x8700 0x3100
124 122 -938 784 68 0 1 [97, 119, 97] 56 0 0x8400 0x3100
125 122 -936 784 62 1 1 [63, 118, 228] 56 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 56 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 56 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 56 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 81 0 0x8000 0x5c00
130 122 -926 784 66 0 1 [134, 183, 131] 76 0 0x8200 0x5c00
131 122 -924 784 75 0 1 [97, 119, 97] 76 0 0x8b00 0x3100
132 122 -922 784 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 122 -920 784 75 0 1 [97, 119, 97] 18446744073709551615 1 0x8b00 0x3100
134 122 -918 784 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x3100
122 123 -942 786 70 0 1 [97, 119, 97] 56 0 0x8600 0x3100
123 123 -940 786 65 0 1 [97, 119, 97] 56 0 0x8100 0x3100
124 123 -938 786 65 0 1 [97, 119, 97] 56 0 0x8100 0x3100
125 123 -936 786 62 1 1 [63, 118, 228] 56 0 0x7e00 0xd800
```
### stream diag after fill:
[c-LoD] stream: tick 4800 65536/65536 cells, lap 4631, cone 97413 zone 640 dirty 1014 (q 12258) ewin 206 rbeat 0p/0 rearm 0c/0t rescan 1204796c/3533e, edits +4803p/3559c/633e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 344 forced 168, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 790090p/2646b fail 0 ~52 kB/s cache 14631/24576 h775459 m14631 e0
[18:30:47 INFO]: [c-LoD] stream: tick 4800 65536/65536 cells, lap 4631, cone 97413 zone 640 dirty 1014 (q 12258) ewin 206 rbeat 0p/0 rearm 0c/0t rescan 1204796c/3533e, edits +4803p/3559c/633e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 344 forced 168, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 790090p/2646b fail 0 ~52 kB/s cache 14631/24576 h775459 m14631 e0
- probe 0: red=1631 paint=15097 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=150 paint>=300)
- **LEG 1 PASS** (zero relogs)
### stream diag after probes:
[18:30:37 INFO]: [c-LoD] stream: tick 4700 65536/65536 cells, lap 4586, cone 96170 zone 640 dirty 1014 (q 11394) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1171219c/2928e, edits +4272p/3174c/481e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 348 forced 164, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 780193p/2619b fail 0 ~93 kB/s cache 14631/24576 h765562 m14631 e0
[c-LoD] stream: tick 4800 65536/65536 cells, lap 4631, cone 97413 zone 640 dirty 1014 (q 12258) ewin 206 rbeat 0p/0 rearm 0c/0t rescan 1204796c/3533e, edits +4803p/3559c/633e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 344 forced 168, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 790090p/2646b fail 0 ~52 kB/s cache 14631/24576 h775459 m14631 e0
[18:30:47 INFO]: [c-LoD] stream: tick 4800 65536/65536 cells, lap 4631, cone 97413 zone 640 dirty 1014 (q 12258) ewin 206 rbeat 0p/0 rearm 0c/0t rescan 1204796c/3533e, edits +4803p/3559c/633e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 344 forced 168, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 790090p/2646b fail 0 ~52 kB/s cache 14631/24576 h775459 m14631 e0
- joined session 2 (18:31:15)
- relog probe 0: red=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need >= 150)
- relog probe 1: red=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need >= 150)
- relog probe 2: red=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need >= 150)
- relog probe 3: red=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need >= 150)
- relog probe 4: red=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need >= 150)
- relog probe 5: red=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need >= 150)
- relog probe 6: red=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need >= 150)
- relog probe 7: red=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need >= 150)
- relog probe 8: red=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need >= 150)
- relog probe 9: red=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need >= 150)
- relog probe 10: red=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need >= 150)
- relog probe 11: red=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need >= 150)
- **FAIL:** marker lost after relog — stream state is not persistent/deterministic

### client log forensics (pipeline/particle/atlas — v6.8.2)
```
ALSA lib confmisc.c:1342:(snd_func_refer) error evaluating name
ALSA lib conf.c:5208:(_snd_config_evaluate) function snd_func_refer returned error: No such file or directory
ALSA lib conf.c:5731:(snd_config_expand) Evaluate error: No such file or directory
[18:31:12] [Render thread/ERROR]: Error starting SoundSystem. Turning off sounds & music
java.lang.IllegalStateException: Failed to open OpenAL device
	at net.minecraft.client.sounds.SoundEngine.reload(SoundEngine.java:110)
	at net.minecraft.server.packs.resources.SimplePreparableReloadListener.lambda$reload$1(SimplePreparableReloadListener.java:15)
	at net.minecraft.server.packs.resources.SimpleReloadInstance.lambda$prepareTasks$3(SimpleReloadInstance.java:59)
[18:31:12] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[18:31:12] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[18:31:12] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[18:31:12] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[18:31:12] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[18:31:12] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[18:31:12] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/blocks.png-atlas
[18:31:12] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[18:31:12] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[18:31:12] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[18:31:12] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[18:31:12] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
[18:31:18] [Render thread/INFO]: Reloading ResourceManager: vanilla, vanilla, server/00000000/6b1f2f7e-9c0d-4a4b-8b0a-b10d1b4ab100
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
[18:31:20] [Render thread/ERROR]: Error starting SoundSystem. Turning off sounds & music
java.lang.IllegalStateException: Failed to open OpenAL device
	at net.minecraft.client.sounds.SoundEngine.reload(SoundEngine.java:110)
	at net.minecraft.server.packs.resources.SimplePreparableReloadListener.lambda$reload$1(SimplePreparableReloadListener.java:15)
	at net.minecraft.server.packs.resources.SimpleReloadInstance.lambda$prepareTasks$3(SimpleReloadInstance.java:59)
[18:31:20] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[18:31:20] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[18:31:20] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[18:31:20] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[18:31:20] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[18:31:20] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[18:31:20] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/blocks.png-atlas
[18:31:20] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[18:31:20] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[18:31:20] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[18:31:20] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[18:31:20] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
```

### /clod-cells AFTER PROBES (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 2656 0 0x8400 0x3100
123 122 -940 784 71 0 1 [97, 119, 97] 2656 0 0x8700 0x3100
124 122 -938 784 68 0 1 [97, 119, 97] 2656 0 0x8400 0x3100
125 122 -936 784 62 1 1 [63, 118, 228] 2656 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 2656 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 2656 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 2656 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 2681 0 0x8000 0x5c00
130 122 -926 784 66 0 1 [134, 183, 131] 2676 0 0x8200 0x5c00
131 122 -924 784 75 0 1 [97, 119, 97] 2676 0 0x8b00 0x3100
132 122 -922 784 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 122 -920 784 75 0 1 [97, 119, 97] 18446744073709551615 1 0x8b00 0x3100
134 122 -918 784 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x3100
122 123 -942 786 70 0 1 [97, 119, 97] 2656 0 0x8600 0x3100
123 123 -940 786 65 0 1 [97, 119, 97] 2656 0 0x8100 0x3100
124 123 -938 786 65 0 1 [97, 119, 97] 2656 0 0x8100 0x3100
125 123 -936 786 62 1 1 [63, 118, 228] 2656 0 0x7e00 0xd800
```
- edit channel: 0 linkage errors, 0 failed event dispatches (both must be 0)
