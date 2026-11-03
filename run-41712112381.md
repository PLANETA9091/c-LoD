### c-LoD visual run (-930,796, marker=red_concrete, vantage -1005,150 yaw 270)
- GL:     Vendor: Mesa (0xffffffff);    Device: llvmpipe (LLVM 20.1.2, 256 bits) (0xffffffff);OpenGL renderer string: llvmpipe (LLVM 20.1.2, 256 bits);OpenGL core profile version string: 4.5 (Core Profile) Mesa 25.2.8-0ubuntu0.24.04.4;
- boot + gates OK (20:25:07)
- sky-grid armed
- joined session 1 (20:25:29)
- stream live: cells=65536
- scan complete=1 (cells=cells=65536/65536)
- paint settled=0 (wire 707658p)
- baseline try 1: drift=439 px
- burst_close white px: 546 (renderer alive if > 50)
- burst_far white px: 0 (renderer alive in the marker zone if > 20)
- fill issued: 5x111x5 red_concrete at -930,796 (zone forceloaded ±48)

### /clod-cells AFTER FILL (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 97 0 0x8400 0x3100
123 122 -940 784 71 0 1 [97, 119, 97] 97 0 0x8700 0x3100
124 122 -938 784 68 0 1 [97, 119, 97] 97 0 0x8400 0x3100
125 122 -936 784 62 1 1 [63, 118, 228] 97 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 97 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 97 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 97 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 63 0 0x8000 0x5c00
130 122 -926 784 66 0 1 [134, 183, 131] 62 0 0x8200 0x5c00
131 122 -924 784 75 0 1 [97, 119, 97] 62 0 0x8b00 0x3100
132 122 -922 784 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 122 -920 784 75 0 1 [97, 119, 97] 18446744073709551615 1 0x8b00 0x3100
134 122 -918 784 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x3100
122 123 -942 786 70 0 1 [97, 119, 97] 97 0 0x8600 0x3100
123 123 -940 786 65 0 1 [97, 119, 97] 97 0 0x8100 0x3100
124 123 -938 786 65 0 1 [97, 119, 97] 97 0 0x8100 0x3100
125 123 -936 786 62 1 1 [63, 118, 228] 97 0 0x7e00 0xd800
```
### stream diag after fill:
[c-LoD] stream: tick 4800 65536/65536 cells, lap 4625, cone 97367 zone 640 dirty 976 (q 10764) ewin 221 rbeat 0p/0 rescan 1195428c/2901e, edits +4062p/2984c/615e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 358 forced 154, native 65536c/1090ch fail 0 (3w q 0) helper on, trees 1090ch/fail 0, wire bundle 789442p/2786b fail 0 ~56 kB/s cache 15615/24576 h773827 m15615 e0
[20:33:17 INFO]: [c-LoD] stream: tick 4800 65536/65536 cells, lap 4625, cone 97367 zone 640 dirty 976 (q 10764) ewin 221 rbeat 0p/0 rescan 1195428c/2901e, edits +4062p/2984c/615e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 358 forced 154, native 65536c/1090ch fail 0 (3w q 0) helper on, trees 1090ch/fail 0, wire bundle 789442p/2786b fail 0 ~56 kB/s cache 15615/24576 h773827 m15615 e0
- probe 0: red=0 paint=14900 lens_red=0 lens_black=762 lens_water=0 lens_dark=200208 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=150 paint>=300)
- probe 1: red=0 paint=14901 lens_red=0 lens_black=762 lens_water=0 lens_dark=200208 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=150 paint>=300)
- probe 2: red=0 paint=14900 lens_red=0 lens_black=762 lens_water=0 lens_dark=200208 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=150 paint>=300)
- probe 3: red=0 paint=14900 lens_red=0 lens_black=762 lens_water=0 lens_dark=200208 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=150 paint>=300)
- probe 4: red=0 paint=14900 lens_red=0 lens_black=762 lens_water=0 lens_dark=200208 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=150 paint>=300)
- probe 5: red=0 paint=14900 lens_red=0 lens_black=762 lens_water=0 lens_dark=200208 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=150 paint>=300)
- probe 6: red=0 paint=14900 lens_red=0 lens_black=762 lens_water=0 lens_dark=200208 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=150 paint>=300)
- probe 7: red=0 paint=14900 lens_red=0 lens_black=762 lens_water=0 lens_dark=200208 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=150 paint>=300)
- probe 8: red=0 paint=14900 lens_red=0 lens_black=762 lens_water=0 lens_dark=200208 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=150 paint>=300)
- probe 9: red=0 paint=14900 lens_red=0 lens_black=762 lens_water=0 lens_dark=200208 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=150 paint>=300)
- probe 10: red=0 paint=14900 lens_red=0 lens_black=762 lens_water=0 lens_dark=200208 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=150 paint>=300)
- probe 11: red=0 paint=14900 lens_red=0 lens_black=762 lens_water=0 lens_dark=200208 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=150 paint>=300)
- probe 12: red=0 paint=14900 lens_red=0 lens_black=762 lens_water=0 lens_dark=200208 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=150 paint>=300)
- probe 13: red=0 paint=14900 lens_red=0 lens_black=762 lens_water=0 lens_dark=200208 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=150 paint>=300)
- probe 14: red=0 paint=15090 lens_red=0 lens_black=570 lens_water=0 lens_dark=200526 lens_zero=0 rot=decoded cam_x_b=? cam_z_b=? az_b=? el_b=? strobe_hits=176 mask_hits=176 (need red>=150 paint>=300)
- probe 15: red=0 paint=14954 lens_red=0 lens_black=570 lens_water=0 lens_dark=200208 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=4 mask_hits=204 (need red>=150 paint>=300)
- probe 16: red=0 paint=14913 lens_red=0 lens_black=570 lens_water=0 lens_dark=200208 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=2 mask_hits=54 (need red>=150 paint>=300)
- probe 17: red=0 paint=14900 lens_red=0 lens_black=762 lens_water=0 lens_dark=200208 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=150 paint>=300)
- probe 18: red=0 paint=14900 lens_red=0 lens_black=762 lens_water=0 lens_dark=200208 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=150 paint>=300)
- probe 19: red=0 paint=14900 lens_red=0 lens_black=762 lens_water=0 lens_dark=200208 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=150 paint>=300)
- probe 20: red=0 paint=14900 lens_red=0 lens_black=762 lens_water=0 lens_dark=200208 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=150 paint>=300)
- probe 21: red=0 paint=14900 lens_red=0 lens_black=762 lens_water=0 lens_dark=200208 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=150 paint>=300)
- probe 22: red=0 paint=14900 lens_red=0 lens_black=762 lens_water=0 lens_dark=200208 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=150 paint>=300)
- probe 23: red=0 paint=14900 lens_red=0 lens_black=762 lens_water=0 lens_dark=200208 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=150 paint>=300)
### stream diag after probes:
[20:36:17 INFO]: [c-LoD] stream: tick 6600 65536/65536 cells, lap 4856, cone 102067 zone 1280 dirty 1056 (q 10800) ewin 36 rbeat 0p/0 rescan 1776894c/2989e, edits +5903p/4304c/1055e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 358 forced 154, native 65536c/1090ch fail 0 (3w q 0) helper on, trees 1090ch/fail 0, wire bundle 832698p/2840b fail 0 ~0 kB/s cache 15711/24576 h816987 m15711 e0
[c-LoD] stream: tick 6700 65536/65536 cells, lap 4856, cone 102067 zone 1280 dirty 1057 (q 10799) ewin 196 rbeat 1p/1 rescan 1809971c/2989e, edits +5903p/4304c/1055e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 358 forced 154, native 65536c/1090ch fail 0 (3w q 0) helper on, trees 1090ch/fail 0, wire bundle 832707p/2841b fail 0 ~0 kB/s cache 15711/24576 h816996 m15711 e0
[20:36:27 INFO]: [c-LoD] stream: tick 6700 65536/65536 cells, lap 4856, cone 102067 zone 1280 dirty 1057 (q 10799) ewin 196 rbeat 1p/1 rescan 1809971c/2989e, edits +5903p/4304c/1055e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 358 forced 154, native 65536c/1090ch fail 0 (3w q 0) helper on, trees 1090ch/fail 0, wire bundle 832707p/2841b fail 0 ~0 kB/s cache 15711/24576 h816996 m15711 e0
- joined session 2 (20:36:48)
- relog probe 0: red=0 lens_red=0 lens_black=762 lens_water=0 lens_dark=200208 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need >= 150)
- relog probe 1: red=0 lens_red=0 lens_black=762 lens_water=0 lens_dark=200208 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need >= 150)
- relog probe 2: red=0 lens_red=0 lens_black=570 lens_water=0 lens_dark=200208 lens_zero=0 rot=decoded cam_x_b=? cam_z_b=? az_b=? el_b=? strobe_hits=176 mask_hits=176 (need >= 150)
- relog probe 3: red=0 lens_red=0 lens_black=570 lens_water=0 lens_dark=200208 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=4 mask_hits=240 (need >= 150)
- relog probe 4: red=0 lens_red=0 lens_black=570 lens_water=0 lens_dark=200208 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=3 mask_hits=83 (need >= 150)
- relog probe 5: red=0 lens_red=0 lens_black=762 lens_water=0 lens_dark=200208 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need >= 150)
- relog probe 6: red=0 lens_red=0 lens_black=570 lens_water=0 lens_dark=200526 lens_zero=0 rot=decoded cam_x_b=? cam_z_b=? az_b=? el_b=? strobe_hits=176 mask_hits=176 (need >= 150)
- relog probe 7: red=0 lens_red=0 lens_black=570 lens_water=0 lens_dark=200208 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=6 mask_hits=179 (need >= 150)
- relog probe 8: red=0 lens_red=0 lens_black=570 lens_water=0 lens_dark=200208 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=71 (need >= 150)
- relog probe 9: red=0 lens_red=0 lens_black=762 lens_water=0 lens_dark=200208 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need >= 150)
- relog probe 10: red=0 lens_red=0 lens_black=570 lens_water=0 lens_dark=200208 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=6 mask_hits=240 (need >= 150)
- relog probe 11: red=0 lens_red=0 lens_black=570 lens_water=0 lens_dark=200208 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=2 mask_hits=154 (need >= 150)
- **FAIL:** marker lost after relog — stream state is not persistent/deterministic

### client log forensics (pipeline/particle/atlas — v6.8.2)
```
ALSA lib confmisc.c:1342:(snd_func_refer) error evaluating name
ALSA lib conf.c:5208:(_snd_config_evaluate) function snd_func_refer returned error: No such file or directory
ALSA lib conf.c:5731:(snd_config_expand) Evaluate error: No such file or directory
[20:36:45] [Render thread/ERROR]: Error starting SoundSystem. Turning off sounds & music
java.lang.IllegalStateException: Failed to open OpenAL device
	at net.minecraft.client.sounds.SoundEngine.reload(SoundEngine.java:110)
	at net.minecraft.server.packs.resources.SimplePreparableReloadListener.lambda$reload$1(SimplePreparableReloadListener.java:15)
	at net.minecraft.server.packs.resources.SimpleReloadInstance.lambda$prepareTasks$3(SimpleReloadInstance.java:59)
[20:36:45] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[20:36:45] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[20:36:45] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[20:36:45] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[20:36:45] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[20:36:45] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[20:36:45] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/blocks.png-atlas
[20:36:45] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[20:36:45] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[20:36:45] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[20:36:45] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[20:36:45] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
[20:36:51] [Render thread/INFO]: Reloading ResourceManager: vanilla, vanilla, server/00000000/6b1f2f7e-9c0d-4a4b-8b0a-b10d1b4ab100
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
[20:36:53] [Render thread/ERROR]: Error starting SoundSystem. Turning off sounds & music
java.lang.IllegalStateException: Failed to open OpenAL device
	at net.minecraft.client.sounds.SoundEngine.reload(SoundEngine.java:110)
	at net.minecraft.server.packs.resources.SimplePreparableReloadListener.lambda$reload$1(SimplePreparableReloadListener.java:15)
	at net.minecraft.server.packs.resources.SimpleReloadInstance.lambda$prepareTasks$3(SimpleReloadInstance.java:59)
[20:36:53] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[20:36:53] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[20:36:53] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[20:36:53] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[20:36:53] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[20:36:53] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[20:36:53] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/blocks.png-atlas
[20:36:53] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[20:36:53] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[20:36:53] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[20:36:53] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[20:36:53] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
```

### /clod-cells AFTER PROBES (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 37 0 0x8400 0x3100
123 122 -940 784 71 0 1 [97, 119, 97] 37 0 0x8700 0x3100
124 122 -938 784 68 0 1 [97, 119, 97] 36 0 0x8400 0x3100
125 122 -936 784 62 1 1 [63, 118, 228] 36 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 35 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 35 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 34 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 34 0 0x8000 0x5c00
130 122 -926 784 66 0 1 [134, 183, 131] 33 0 0x8200 0x5c00
131 122 -924 784 75 0 1 [97, 119, 97] 32 0 0x8b00 0x3100
132 122 -922 784 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 122 -920 784 75 0 1 [97, 119, 97] 18446744073709551615 1 0x8b00 0x3100
134 122 -918 784 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x3100
122 123 -942 786 70 0 1 [97, 119, 97] 38 0 0x8600 0x3100
123 123 -940 786 65 0 1 [97, 119, 97] 37 0 0x8100 0x3100
124 123 -938 786 65 0 1 [97, 119, 97] 37 0 0x8100 0x3100
125 123 -936 786 62 1 1 [63, 118, 228] 36 0 0x7e00 0xd800
```
- edit channel: 0 linkage errors, 0 failed event dispatches (both must be 0)
