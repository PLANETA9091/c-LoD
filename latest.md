### c-LoD visual run (-930,796, marker=red_concrete, vantage -1005,150 yaw 270)
- GL:     Vendor: Mesa (0xffffffff);    Device: llvmpipe (LLVM 20.1.2, 256 bits) (0xffffffff);OpenGL renderer string: llvmpipe (LLVM 20.1.2, 256 bits);OpenGL core profile version string: 4.5 (Core Profile) Mesa 25.2.8-0ubuntu0.24.04.4;
- boot + gates OK (19:37:09)
- sky-grid armed
- joined session 1 (19:37:30)
- stream live: cells=65536
- scan complete=1 (cells=cells=65536/65536)
- paint settled=0 (wire 710496p)
- baseline try 1: drift=544 px
- burst_close white px: 536 (renderer alive if > 50)
- burst_far white px: 0 (renderer alive in the marker zone if > 20)
- fill issued: 5x111x5 red_concrete at -930,796 (zone forceloaded ±48)

### /clod-cells AFTER FILL (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 46 0 0x8400 0x3100
123 122 -940 784 71 0 1 [97, 119, 97] 46 0 0x8700 0x3100
124 122 -938 784 68 0 1 [97, 119, 97] 46 0 0x8400 0x3100
125 122 -936 784 62 1 1 [63, 118, 228] 46 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 46 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 46 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 46 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 53 0 0x8000 0x5c00
130 122 -926 784 66 0 1 [134, 183, 131] 53 0 0x8200 0x5c00
131 122 -924 784 75 0 1 [97, 119, 97] 53 0 0x8b00 0x3100
132 122 -922 784 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 122 -920 784 75 0 1 [97, 119, 97] 18446744073709551615 1 0x8b00 0x3100
134 122 -918 784 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x3100
122 123 -942 786 70 0 1 [97, 119, 97] 46 0 0x8600 0x3100
123 123 -940 786 65 0 1 [97, 119, 97] 46 0 0x8100 0x3100
124 123 -938 786 65 0 1 [97, 119, 97] 46 0 0x8100 0x3100
125 123 -936 786 62 1 1 [63, 118, 228] 46 0 0x7e00 0xd800
```
### stream diag after fill:
[c-LoD] stream: tick 4800 65536/65536 cells, lap 4617, cone 97875 zone 640 dirty 1009 (q 10959) ewin 228 rbeat 0p/0 rescan 1194263c/1851e, edits +4119p/3024c/537e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 345 forced 167, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1088ch/fail 0, wire bundle 793730p/2874b fail 0 ~63 kB/s cache 15807/24576 h777923 m15807 e0
[19:45:17 INFO]: [c-LoD] stream: tick 4800 65536/65536 cells, lap 4617, cone 97875 zone 640 dirty 1009 (q 10959) ewin 228 rbeat 0p/0 rescan 1194263c/1851e, edits +4119p/3024c/537e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 345 forced 167, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1088ch/fail 0, wire bundle 793730p/2874b fail 0 ~63 kB/s cache 15807/24576 h777923 m15807 e0
- LENS ALL-ZERO: both capture buffers read empty — the capture chain never committed this session; these probes measure a dead pipeline, not the transport (dot-loss verdict N/A)
- probe 0: red=27 paint=14890 lens_red=0 lens_black=192 lens_water=0 lens_dark=200400 lens_zero=1 rot=ok scan=15 carriers=0 (need red>=150 paint>=300)
- probe 1: red=27 paint=14890 lens_red=0 lens_black=192 lens_water=0 lens_dark=200400 lens_zero=1 rot=ok scan=15 carriers=0 (need red>=150 paint>=300)
- probe 2: red=27 paint=14890 lens_red=0 lens_black=192 lens_water=0 lens_dark=200400 lens_zero=1 rot=ok scan=15 carriers=0 (need red>=150 paint>=300)
- probe 3: red=27 paint=14890 lens_red=0 lens_black=192 lens_water=0 lens_dark=200400 lens_zero=1 rot=ok scan=15 carriers=0 (need red>=150 paint>=300)
- probe 4: red=27 paint=14890 lens_red=0 lens_black=192 lens_water=0 lens_dark=200400 lens_zero=1 rot=ok scan=15 carriers=0 (need red>=150 paint>=300)
- probe 5: red=27 paint=14890 lens_red=0 lens_black=192 lens_water=0 lens_dark=200400 lens_zero=1 rot=ok scan=15 carriers=0 (need red>=150 paint>=300)
- probe 6: red=27 paint=14890 lens_red=0 lens_black=192 lens_water=0 lens_dark=200400 lens_zero=1 rot=ok scan=15 carriers=0 (need red>=150 paint>=300)
- probe 7: red=27 paint=14890 lens_red=0 lens_black=192 lens_water=0 lens_dark=200400 lens_zero=1 rot=ok scan=15 carriers=0 (need red>=150 paint>=300)
- probe 8: red=27 paint=14890 lens_red=0 lens_black=192 lens_water=0 lens_dark=200400 lens_zero=1 rot=ok scan=15 carriers=0 (need red>=150 paint>=300)
- probe 9: red=27 paint=14890 lens_red=0 lens_black=192 lens_water=0 lens_dark=200400 lens_zero=1 rot=ok scan=15 carriers=0 (need red>=150 paint>=300)
- probe 10: red=27 paint=14890 lens_red=0 lens_black=192 lens_water=0 lens_dark=200400 lens_zero=1 rot=ok scan=15 carriers=0 (need red>=150 paint>=300)
- probe 11: red=27 paint=14890 lens_red=0 lens_black=192 lens_water=0 lens_dark=200400 lens_zero=1 rot=ok scan=15 carriers=0 (need red>=150 paint>=300)
- probe 12: red=27 paint=14890 lens_red=0 lens_black=192 lens_water=0 lens_dark=200400 lens_zero=1 rot=ok scan=15 carriers=0 (need red>=150 paint>=300)
- probe 13: red=27 paint=14890 lens_red=0 lens_black=192 lens_water=0 lens_dark=200400 lens_zero=1 rot=ok scan=15 carriers=0 (need red>=150 paint>=300)
- probe 14: red=27 paint=14890 lens_red=0 lens_black=192 lens_water=0 lens_dark=200400 lens_zero=1 rot=ok scan=15 carriers=0 (need red>=150 paint>=300)
- probe 15: red=27 paint=14890 lens_red=0 lens_black=192 lens_water=0 lens_dark=200400 lens_zero=1 rot=ok scan=15 carriers=0 (need red>=150 paint>=300)
- probe 16: red=27 paint=14890 lens_red=0 lens_black=192 lens_water=0 lens_dark=200400 lens_zero=1 rot=ok scan=15 carriers=0 (need red>=150 paint>=300)
- probe 17: red=27 paint=14712 lens_red=0 lens_black=0 lens_water=0 lens_dark=200400 lens_zero=1 rot=ok scan=15 carriers=15 (need red>=150 paint>=300)
- probe 18: red=27 paint=14890 lens_red=0 lens_black=192 lens_water=0 lens_dark=200400 lens_zero=1 rot=ok scan=15 carriers=0 (need red>=150 paint>=300)
- probe 19: red=27 paint=15044 lens_red=0 lens_black=0 lens_water=0 lens_dark=200400 lens_zero=1 rot=ok scan=15 carriers=83 (need red>=150 paint>=300)
- probe 20: red=27 paint=14924 lens_red=0 lens_black=0 lens_water=0 lens_dark=200400 lens_zero=1 rot=ok scan=15 carriers=45 (need red>=150 paint>=300)
- probe 21: red=27 paint=14896 lens_red=0 lens_black=192 lens_water=0 lens_dark=200400 lens_zero=1 rot=ok scan=15 carriers=4 (need red>=150 paint>=300)
- probe 22: red=27 paint=14724 lens_red=0 lens_black=0 lens_water=0 lens_dark=200400 lens_zero=1 rot=ok scan=15 carriers=20 (need red>=150 paint>=300)
- probe 23: red=27 paint=15011 lens_red=0 lens_black=0 lens_water=0 lens_dark=200400 lens_zero=1 rot=ok scan=15 carriers=82 (need red>=150 paint>=300)
### stream diag after probes:
[19:48:18 INFO]: [c-LoD] stream: tick 6600 65536/65536 cells, lap 5066, cone 107915 zone 1280 dirty 1088 (q 10990) ewin 0 rbeat 0p/0 rescan 1775699c/1940e, edits +5997p/4374c/978e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 345 forced 167, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1088ch/fail 0, wire bundle 879505p/2971b fail 0 ~112 kB/s cache 15890/24576 h863615 m15890 e0
[c-LoD] stream: tick 6700 65536/65536 cells, lap 5166, cone 110585 zone 1280 dirty 1088 (q 10990) ewin 0 rbeat 0p/0 rescan 1807773c/1941e, edits +5999p/4376c/978e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 345 forced 167, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1088ch/fail 0, wire bundle 900771p/2993b fail 0 ~112 kB/s cache 15890/24576 h884881 m15890 e0
[19:48:28 INFO]: [c-LoD] stream: tick 6700 65536/65536 cells, lap 5166, cone 110585 zone 1280 dirty 1088 (q 10990) ewin 0 rbeat 0p/0 rescan 1807773c/1941e, edits +5999p/4376c/978e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 345 forced 167, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1088ch/fail 0, wire bundle 900771p/2993b fail 0 ~112 kB/s cache 15890/24576 h884881 m15890 e0
- joined session 2 (19:48:50)
- relog probe 0: red=77 lens_red=0 lens_black=192 lens_water=0 lens_dark=200400 lens_zero=1 rot=ok scan=15 carriers=7 (need >= 150)
- relog probe 1: red=76 lens_red=0 lens_black=0 lens_water=0 lens_dark=200718 lens_zero=0 rot=mixed(91g/91r) scan=15 carriers=176 (need >= 150)
- relog probe 2: red=76 lens_red=0 lens_black=0 lens_water=0 lens_dark=200400 lens_zero=1 rot=ok scan=15 carriers=70 (need >= 150)
- relog probe 3: red=76 lens_red=0 lens_black=0 lens_water=0 lens_dark=200400 lens_zero=1 rot=ok scan=15 carriers=35 (need >= 150)
- relog probe 4: red=77 lens_red=0 lens_black=192 lens_water=0 lens_dark=200400 lens_zero=1 rot=ok scan=15 carriers=6 (need >= 150)
- relog probe 5: red=75 lens_red=0 lens_black=0 lens_water=0 lens_dark=200400 lens_zero=0 rot=mixed(100g/100r) scan=15 carriers=176 (need >= 150)
- relog probe 6: red=77 lens_red=0 lens_black=0 lens_water=0 lens_dark=200400 lens_zero=1 rot=ok scan=15 carriers=68 (need >= 150)
- relog probe 7: red=75 lens_red=0 lens_black=0 lens_water=0 lens_dark=200400 lens_zero=1 rot=ok scan=15 carriers=20 (need >= 150)
- relog probe 8: red=77 lens_red=0 lens_black=192 lens_water=0 lens_dark=200400 lens_zero=1 rot=ok scan=15 carriers=1 (need >= 150)
- relog probe 9: red=74 lens_red=0 lens_black=0 lens_water=0 lens_dark=200718 lens_zero=0 rot=mixed(100g/100r) scan=15 carriers=176 (need >= 150)
- relog probe 10: red=77 lens_red=0 lens_black=0 lens_water=0 lens_dark=200400 lens_zero=1 rot=ok scan=15 carriers=48 (need >= 150)
- relog probe 11: red=77 lens_red=0 lens_black=0 lens_water=0 lens_dark=200400 lens_zero=1 rot=ok scan=15 carriers=19 (need >= 150)
- **FAIL:** marker lost after relog — stream state is not persistent/deterministic

### client log forensics (pipeline/particle/atlas — v6.8.2)
```
ALSA lib confmisc.c:1342:(snd_func_refer) error evaluating name
ALSA lib conf.c:5208:(_snd_config_evaluate) function snd_func_refer returned error: No such file or directory
ALSA lib conf.c:5731:(snd_config_expand) Evaluate error: No such file or directory
[19:48:47] [Render thread/ERROR]: Error starting SoundSystem. Turning off sounds & music
java.lang.IllegalStateException: Failed to open OpenAL device
	at net.minecraft.client.sounds.SoundEngine.reload(SoundEngine.java:110)
	at net.minecraft.server.packs.resources.SimplePreparableReloadListener.lambda$reload$1(SimplePreparableReloadListener.java:15)
	at net.minecraft.server.packs.resources.SimpleReloadInstance.lambda$prepareTasks$3(SimpleReloadInstance.java:59)
[19:48:47] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[19:48:47] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[19:48:47] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[19:48:47] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[19:48:47] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[19:48:47] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[19:48:47] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/blocks.png-atlas
[19:48:47] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[19:48:47] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[19:48:47] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[19:48:47] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[19:48:47] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
[19:48:54] [Render thread/INFO]: Reloading ResourceManager: vanilla, vanilla, server/00000000/6b1f2f7e-9c0d-4a4b-8b0a-b10d1b4ab100
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
[19:48:56] [Render thread/ERROR]: Error starting SoundSystem. Turning off sounds & music
java.lang.IllegalStateException: Failed to open OpenAL device
	at net.minecraft.client.sounds.SoundEngine.reload(SoundEngine.java:110)
	at net.minecraft.server.packs.resources.SimplePreparableReloadListener.lambda$reload$1(SimplePreparableReloadListener.java:15)
	at net.minecraft.server.packs.resources.SimpleReloadInstance.lambda$prepareTasks$3(SimpleReloadInstance.java:59)
[19:48:56] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[19:48:56] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[19:48:56] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[19:48:56] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[19:48:56] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[19:48:56] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[19:48:56] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/blocks.png-atlas
[19:48:56] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[19:48:56] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[19:48:56] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[19:48:56] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[19:48:56] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
```

### /clod-cells AFTER PROBES (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 45 0 0x8400 0x3100
123 122 -940 784 71 0 1 [97, 119, 97] 45 0 0x8700 0x3100
124 122 -938 784 68 0 1 [97, 119, 97] 44 0 0x8400 0x3100
125 122 -936 784 62 1 1 [63, 118, 228] 44 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 43 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 43 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 42 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 42 0 0x8000 0x5c00
130 122 -926 784 66 0 1 [134, 183, 131] 41 0 0x8200 0x5c00
131 122 -924 784 75 0 1 [97, 119, 97] 40 0 0x8b00 0x3100
132 122 -922 784 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 122 -920 784 75 0 1 [97, 119, 97] 18446744073709551615 1 0x8b00 0x3100
134 122 -918 784 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x3100
122 123 -942 786 70 0 1 [97, 119, 97] 46 0 0x8600 0x3100
123 123 -940 786 65 0 1 [97, 119, 97] 45 0 0x8100 0x3100
124 123 -938 786 65 0 1 [97, 119, 97] 45 0 0x8100 0x3100
125 123 -936 786 62 1 1 [63, 118, 228] 44 0 0x7e00 0xd800
```
- edit channel: 0 linkage errors, 0 failed event dispatches (both must be 0)
