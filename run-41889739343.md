### c-LoD visual run (-930,796, marker=red_concrete, vantage -1005,150 yaw 270)
- GL:     Vendor: Mesa (0xffffffff);    Device: llvmpipe (LLVM 20.1.2, 256 bits) (0xffffffff);OpenGL renderer string: llvmpipe (LLVM 20.1.2, 256 bits);OpenGL core profile version string: 4.5 (Core Profile) Mesa 25.2.8-0ubuntu0.24.04.4;
- boot + gates OK (18:44:15)
- sky-grid armed
- joined session 1 (18:44:42)
- stream live: cells=65536
- scan complete=1 (cells=cells=65536/65536)
- paint settled=0 (wire 702298p)
- baseline try 1: drift=473 px
- burst_close white px: 554 (renderer alive if > 50)
- burst_far white px: 398 (renderer alive in the marker zone if > 20)
- fill issued: 5x111x5 red_concrete at -930,796 (zone forceloaded ±48)

### /clod-cells AFTER FILL (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 53 0 0x8400 0x3100
123 122 -940 784 71 0 1 [97, 119, 97] 53 0 0x8700 0x3100
124 122 -938 784 68 0 1 [97, 119, 97] 53 0 0x8400 0x3100
125 122 -936 784 62 1 1 [63, 118, 228] 53 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 53 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 53 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 53 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 23 0 0x8000 0x5c00
130 122 -926 784 66 0 1 [134, 183, 131] 23 0 0x8200 0x5c00
131 122 -924 784 75 0 1 [97, 119, 97] 23 0 0x8b00 0x3100
132 122 -922 784 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 122 -920 784 75 0 1 [97, 119, 97] 18446744073709551615 1 0x8b00 0x3100
134 122 -918 784 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x3100
122 123 -942 786 70 0 1 [97, 119, 97] 53 0 0x8600 0x3100
123 123 -940 786 65 0 1 [97, 119, 97] 53 0 0x8100 0x3100
124 123 -938 786 65 0 1 [97, 119, 97] 53 0 0x8100 0x3100
125 123 -936 786 62 1 1 [63, 118, 228] 53 0 0x7e00 0xd800
```
### stream diag after fill:
[c-LoD] stream: tick 4900 65536/65536 cells, lap 4638, cone 97836 zone 640 dirty 1082 (q 12093) ewin 172 rbeat 21p/1 rearm 0c/0t rescan 1212631c/2021e, edits +3095p/2316c/641e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 311 forced 201, native 65536c/1090ch fail 0 (3w q 0) helper on, trees 1090ch/fail 0, wire bundle 794006p/2948b fail 0 ~18 kB/s cache 15969/24576 h778037 m15969 e0
[18:52:36 INFO]: [c-LoD] stream: tick 4900 65536/65536 cells, lap 4638, cone 97836 zone 640 dirty 1082 (q 12093) ewin 172 rbeat 21p/1 rearm 0c/0t rescan 1212631c/2021e, edits +3095p/2316c/641e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 311 forced 201, native 65536c/1090ch fail 0 (3w q 0) helper on, trees 1090ch/fail 0, wire bundle 794006p/2948b fail 0 ~18 kB/s cache 15969/24576 h778037 m15969 e0
- probe 0: red=0 paint=15081 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=150 paint>=300)
- probe 1: red=0 paint=15081 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=150 paint>=300)
- probe 2: red=0 paint=15085 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=150 paint>=300)
- probe 3: red=0 paint=15081 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=150 paint>=300)
- probe 4: red=0 paint=15081 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=150 paint>=300)
- probe 5: red=0 paint=15081 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=150 paint>=300)
- probe 6: red=0 paint=15081 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=150 paint>=300)
- probe 7: red=0 paint=15081 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=150 paint>=300)
- probe 8: red=0 paint=15081 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=150 paint>=300)
- probe 9: red=0 paint=15081 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=150 paint>=300)
- probe 10: red=0 paint=15081 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=150 paint>=300)
- probe 11: red=0 paint=15081 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=150 paint>=300)
- probe 12: red=0 paint=15081 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=150 paint>=300)
- probe 13: red=0 paint=15081 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=150 paint>=300)
- probe 14: red=0 paint=15081 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=150 paint>=300)
- probe 15: red=0 paint=15081 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=150 paint>=300)
- probe 16: red=0 paint=15065 lens_red=0 lens_black=0 lens_water=0 lens_dark=198436 lens_zero=0 rot=decoded cam_x_b=? cam_z_b=? az_b=? el_b=? strobe_hits=22 mask_hits=22 (need red>=150 paint>=300)
- probe 17: red=0 paint=14928 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=10 mask_hits=30 (need red>=150 paint>=300)
- probe 18: red=0 paint=14904 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=1 mask_hits=30 (need red>=150 paint>=300)
- probe 19: red=0 paint=15029 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=? cam_z_b=? az_b=? el_b=? strobe_hits=22 mask_hits=22 (need red>=150 paint>=300)
- probe 20: red=0 paint=14741 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=7 mask_hits=30 (need red>=150 paint>=300)
- probe 21: red=0 paint=14892 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=150 paint>=300)
- probe 22: red=0 paint=15053 lens_red=0 lens_black=0 lens_water=0 lens_dark=198446 lens_zero=0 rot=decoded cam_x_b=? cam_z_b=? az_b=? el_b=? strobe_hits=22 mask_hits=22 (need red>=150 paint>=300)
- probe 23: red=0 paint=14743 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=5 mask_hits=30 (need red>=150 paint>=300)
### stream diag after probes:
[18:55:37 INFO]: [c-LoD] stream: tick 6700 65536/65536 cells, lap 5143, cone 109211 zone 1280 dirty 1136 (q 12118) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1794124c/2091e, edits +4584p/3351c/961e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 311 forced 201, native 65536c/1090ch fail 0 (3w q 0) helper on, trees 1090ch/fail 0, wire bundle 890180p/3055b fail 0 ~112 kB/s cache 15971/24576 h874209 m15971 e0
[c-LoD] stream: tick 6800 65536/65536 cells, lap 5243, cone 111698 zone 1280 dirty 1136 (q 12119) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1825649c/2092e, edits +4588p/3355c/963e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 311 forced 201, native 65536c/1090ch fail 0 (3w q 0) helper on, trees 1090ch/fail 0, wire bundle 908963p/3074b fail 0 ~99 kB/s cache 15971/24576 h892992 m15971 e0
[18:55:47 INFO]: [c-LoD] stream: tick 6800 65536/65536 cells, lap 5243, cone 111698 zone 1280 dirty 1136 (q 12119) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1825649c/2092e, edits +4588p/3355c/963e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 311 forced 201, native 65536c/1090ch fail 0 (3w q 0) helper on, trees 1090ch/fail 0, wire bundle 908963p/3074b fail 0 ~99 kB/s cache 15971/24576 h892992 m15971 e0
- joined session 2 (18:56:14)
- relog probe 0: red=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need >= 150)
- relog probe 1: red=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=3 mask_hits=30 (need >= 150)
- relog probe 2: red=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=16 mask_hits=30 (need >= 150)
- relog probe 3: red=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=6 mask_hits=30 (need >= 150)
- relog probe 4: red=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=4 mask_hits=30 (need >= 150)
- relog probe 5: red=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=16 mask_hits=30 (need >= 150)
- relog probe 6: red=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=7 mask_hits=30 (need >= 150)
- relog probe 7: red=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=1 mask_hits=30 (need >= 150)
- relog probe 8: red=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=16 mask_hits=30 (need >= 150)
- relog probe 9: red=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=9 mask_hits=30 (need >= 150)
- relog probe 10: red=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=8 mask_hits=30 (need >= 150)
- relog probe 11: red=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=19 mask_hits=30 (need >= 150)
- **FAIL:** marker lost after relog — stream state is not persistent/deterministic

### client log forensics (pipeline/particle/atlas — v6.8.2)
```
ALSA lib confmisc.c:1342:(snd_func_refer) error evaluating name
ALSA lib conf.c:5208:(_snd_config_evaluate) function snd_func_refer returned error: No such file or directory
ALSA lib conf.c:5731:(snd_config_expand) Evaluate error: No such file or directory
[18:56:12] [Render thread/ERROR]: Error starting SoundSystem. Turning off sounds & music
java.lang.IllegalStateException: Failed to open OpenAL device
	at net.minecraft.client.sounds.SoundEngine.reload(SoundEngine.java:110)
	at net.minecraft.server.packs.resources.SimplePreparableReloadListener.lambda$reload$1(SimplePreparableReloadListener.java:15)
	at net.minecraft.server.packs.resources.SimpleReloadInstance.lambda$prepareTasks$3(SimpleReloadInstance.java:59)
[18:56:12] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[18:56:12] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[18:56:12] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[18:56:12] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[18:56:12] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[18:56:12] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[18:56:12] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/blocks.png-atlas
[18:56:12] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[18:56:12] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[18:56:12] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[18:56:12] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[18:56:12] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
[18:56:19] [Render thread/INFO]: Reloading ResourceManager: vanilla, vanilla, server/00000000/6b1f2f7e-9c0d-4a4b-8b0a-b10d1b4ab100
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
[18:56:21] [Render thread/ERROR]: Error starting SoundSystem. Turning off sounds & music
java.lang.IllegalStateException: Failed to open OpenAL device
	at net.minecraft.client.sounds.SoundEngine.reload(SoundEngine.java:110)
	at net.minecraft.server.packs.resources.SimplePreparableReloadListener.lambda$reload$1(SimplePreparableReloadListener.java:15)
	at net.minecraft.server.packs.resources.SimpleReloadInstance.lambda$prepareTasks$3(SimpleReloadInstance.java:59)
[18:56:21] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[18:56:21] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[18:56:21] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[18:56:21] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[18:56:21] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[18:56:21] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[18:56:21] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/blocks.png-atlas
[18:56:21] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[18:56:21] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[18:56:21] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[18:56:21] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[18:56:21] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
```

### /clod-cells AFTER PROBES (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 54 0 0x8400 0x3100
123 122 -940 784 71 0 1 [97, 119, 97] 54 0 0x8700 0x3100
124 122 -938 784 68 0 1 [97, 119, 97] 53 0 0x8400 0x3100
125 122 -936 784 62 1 1 [63, 118, 228] 53 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 52 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 52 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 51 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 51 0 0x8000 0x5c00
130 122 -926 784 66 0 1 [134, 183, 131] 50 0 0x8200 0x5c00
131 122 -924 784 75 0 1 [97, 119, 97] 49 0 0x8b00 0x3100
132 122 -922 784 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 122 -920 784 75 0 1 [97, 119, 97] 18446744073709551615 1 0x8b00 0x3100
134 122 -918 784 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x3100
122 123 -942 786 70 0 1 [97, 119, 97] 55 0 0x8600 0x3100
123 123 -940 786 65 0 1 [97, 119, 97] 54 0 0x8100 0x3100
124 123 -938 786 65 0 1 [97, 119, 97] 54 0 0x8100 0x3100
125 123 -936 786 62 1 1 [63, 118, 228] 53 0 0x7e00 0xd800
```
- edit channel: 0 linkage errors, 0 failed event dispatches (both must be 0)
