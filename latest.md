### c-LoD visual run (-930,796, marker=red_concrete, vantage -1005,150 yaw 270)
- GL:     Vendor: Mesa (0xffffffff);    Device: llvmpipe (LLVM 20.1.2, 256 bits) (0xffffffff);OpenGL renderer string: llvmpipe (LLVM 20.1.2, 256 bits);OpenGL core profile version string: 4.5 (Core Profile) Mesa 25.2.8-0ubuntu0.24.04.4;
- boot + gates OK (19:13:28)
- sky-grid armed
- joined session 1 (19:13:49)
- stream live: cells=65536
- scan complete=1 (cells=cells=65536/65536)
- paint settled=0 (wire 710102p)
- baseline try 1: drift=0 px
- burst_close white px: 5405 (renderer alive if > 50)
- burst_far white px: 5405 (renderer alive in the marker zone if > 20)
- fill issued: 5x111x5 red_concrete at -930,796 (zone forceloaded ±48)

### /clod-cells AFTER FILL (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 61 0 0x8400 0x3100
123 122 -940 784 71 0 1 [97, 119, 97] 61 0 0x8700 0x3100
124 122 -938 784 68 0 1 [97, 119, 97] 61 0 0x8400 0x3100
125 122 -936 784 62 1 1 [63, 118, 228] 61 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 61 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 61 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 61 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 66 0 0x8000 0x5c00
130 122 -926 784 66 0 1 [134, 183, 131] 66 0 0x8200 0x5c00
131 122 -924 784 75 0 1 [97, 119, 97] 66 0 0x8b00 0x3100
132 122 -922 784 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 122 -920 784 75 0 1 [97, 119, 97] 18446744073709551615 1 0x8b00 0x3100
134 122 -918 784 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x3100
122 123 -942 786 70 0 1 [97, 119, 97] 61 0 0x8600 0x3100
123 123 -940 786 65 0 1 [97, 119, 97] 61 0 0x8100 0x3100
124 123 -938 786 65 0 1 [97, 119, 97] 61 0 0x8100 0x3100
125 123 -936 786 62 1 1 [63, 118, 228] 61 0 0x7e00 0xd800
```
### stream diag after fill:
[c-LoD] stream: tick 4800 65536/65536 cells, lap 4630, cone 97969 zone 640 dirty 1033 (q 11385) ewin 216 rbeat 0p/0 rescan 1201235c/3613e, edits +4385p/3242c/585e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 345 forced 167, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 794606p/2719b fail 0 ~53 kB/s cache 15558/24576 h779048 m15558 e0
[19:21:37 INFO]: [c-LoD] stream: tick 4800 65536/65536 cells, lap 4630, cone 97969 zone 640 dirty 1033 (q 11385) ewin 216 rbeat 0p/0 rescan 1201235c/3613e, edits +4385p/3242c/585e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 345 forced 167, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 794606p/2719b fail 0 ~53 kB/s cache 15558/24576 h779048 m15558 e0
- probe 0: red=0 paint=0 lens_red=? lens_black=? lens_water=? lens_dark=? lens_zero=? rot=? scan=? carriers=? (need red>=150 paint>=300)
- probe 1: red=0 paint=0 lens_red=? lens_black=? lens_water=? lens_dark=? lens_zero=? rot=? scan=? carriers=? (need red>=150 paint>=300)
- probe 2: red=0 paint=0 lens_red=? lens_black=? lens_water=? lens_dark=? lens_zero=? rot=? scan=? carriers=? (need red>=150 paint>=300)
- probe 3: red=0 paint=0 lens_red=? lens_black=? lens_water=? lens_dark=? lens_zero=? rot=? scan=? carriers=? (need red>=150 paint>=300)
- probe 4: red=0 paint=0 lens_red=? lens_black=? lens_water=? lens_dark=? lens_zero=? rot=? scan=? carriers=? (need red>=150 paint>=300)
- probe 5: red=0 paint=0 lens_red=? lens_black=? lens_water=? lens_dark=? lens_zero=? rot=? scan=? carriers=? (need red>=150 paint>=300)
- probe 6: red=0 paint=0 lens_red=? lens_black=? lens_water=? lens_dark=? lens_zero=? rot=? scan=? carriers=? (need red>=150 paint>=300)
- probe 7: red=0 paint=0 lens_red=? lens_black=? lens_water=? lens_dark=? lens_zero=? rot=? scan=? carriers=? (need red>=150 paint>=300)
- probe 8: red=0 paint=0 lens_red=? lens_black=? lens_water=? lens_dark=? lens_zero=? rot=? scan=? carriers=? (need red>=150 paint>=300)
- probe 9: red=0 paint=0 lens_red=? lens_black=? lens_water=? lens_dark=? lens_zero=? rot=? scan=? carriers=? (need red>=150 paint>=300)
- probe 10: red=0 paint=0 lens_red=? lens_black=? lens_water=? lens_dark=? lens_zero=? rot=? scan=? carriers=? (need red>=150 paint>=300)
- probe 11: red=0 paint=0 lens_red=? lens_black=? lens_water=? lens_dark=? lens_zero=? rot=? scan=? carriers=? (need red>=150 paint>=300)
- probe 12: red=0 paint=0 lens_red=? lens_black=? lens_water=? lens_dark=? lens_zero=? rot=? scan=? carriers=? (need red>=150 paint>=300)
- probe 13: red=0 paint=0 lens_red=? lens_black=? lens_water=? lens_dark=? lens_zero=? rot=? scan=? carriers=? (need red>=150 paint>=300)
- probe 14: red=0 paint=0 lens_red=? lens_black=? lens_water=? lens_dark=? lens_zero=? rot=? scan=? carriers=? (need red>=150 paint>=300)
- probe 15: red=0 paint=0 lens_red=? lens_black=? lens_water=? lens_dark=? lens_zero=? rot=? scan=? carriers=? (need red>=150 paint>=300)
- probe 16: red=0 paint=0 lens_red=? lens_black=? lens_water=? lens_dark=? lens_zero=? rot=? scan=? carriers=? (need red>=150 paint>=300)
- probe 17: red=0 paint=0 lens_red=? lens_black=? lens_water=? lens_dark=? lens_zero=? rot=? scan=? carriers=? (need red>=150 paint>=300)
- probe 18: red=0 paint=0 lens_red=? lens_black=? lens_water=? lens_dark=? lens_zero=? rot=? scan=? carriers=? (need red>=150 paint>=300)
- probe 19: red=0 paint=0 lens_red=? lens_black=? lens_water=? lens_dark=? lens_zero=? rot=? scan=? carriers=? (need red>=150 paint>=300)
- probe 20: red=0 paint=0 lens_red=? lens_black=? lens_water=? lens_dark=? lens_zero=? rot=? scan=? carriers=? (need red>=150 paint>=300)
- probe 21: red=0 paint=0 lens_red=? lens_black=? lens_water=? lens_dark=? lens_zero=? rot=? scan=? carriers=? (need red>=150 paint>=300)
- probe 22: red=0 paint=0 lens_red=? lens_black=? lens_water=? lens_dark=? lens_zero=? rot=? scan=? carriers=? (need red>=150 paint>=300)
- probe 23: red=0 paint=0 lens_red=? lens_black=? lens_water=? lens_dark=? lens_zero=? rot=? scan=? carriers=? (need red>=150 paint>=300)
### stream diag after probes:
[19:24:18 INFO]: [c-LoD] stream: tick 6400 65536/65536 cells, lap 4891, cone 104004 zone 1280 dirty 1108 (q 11416) ewin 0 rbeat 0p/0 rescan 1718126c/3692e, edits +6157p/4507c/992e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 345 forced 167, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 848441p/2783b fail 0 ~112 kB/s cache 15639/24576 h832802 m15639 e0
[c-LoD] stream: tick 6500 65536/65536 cells, lap 4991, cone 105979 zone 1280 dirty 1108 (q 11416) ewin 0 rbeat 0p/0 rescan 1754064c/3692e, edits +6158p/4508c/992e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 345 forced 167, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 863136p/2798b fail 0 ~77 kB/s cache 15639/24576 h847497 m15639 e0
[19:24:28 INFO]: [c-LoD] stream: tick 6500 65536/65536 cells, lap 4991, cone 105979 zone 1280 dirty 1108 (q 11416) ewin 0 rbeat 0p/0 rescan 1754064c/3692e, edits +6158p/4508c/992e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 345 forced 167, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 863136p/2798b fail 0 ~77 kB/s cache 15639/24576 h847497 m15639 e0
- joined session 2 (19:24:55)
- relog probe 0: red=0 lens_red=? lens_black=? lens_water=? lens_dark=? lens_zero=? rot=? scan=? carriers=? (need >= 150)
- relog probe 1: red=0 lens_red=? lens_black=? lens_water=? lens_dark=? lens_zero=? rot=? scan=? carriers=? (need >= 150)
- relog probe 2: red=0 lens_red=? lens_black=? lens_water=? lens_dark=? lens_zero=? rot=? scan=? carriers=? (need >= 150)
- relog probe 3: red=0 lens_red=? lens_black=? lens_water=? lens_dark=? lens_zero=? rot=? scan=? carriers=? (need >= 150)
- relog probe 4: red=0 lens_red=? lens_black=? lens_water=? lens_dark=? lens_zero=? rot=? scan=? carriers=? (need >= 150)
- relog probe 5: red=0 lens_red=? lens_black=? lens_water=? lens_dark=? lens_zero=? rot=? scan=? carriers=? (need >= 150)
- relog probe 6: red=0 lens_red=? lens_black=? lens_water=? lens_dark=? lens_zero=? rot=? scan=? carriers=? (need >= 150)
- relog probe 7: red=0 lens_red=? lens_black=? lens_water=? lens_dark=? lens_zero=? rot=? scan=? carriers=? (need >= 150)
- relog probe 8: red=0 lens_red=? lens_black=? lens_water=? lens_dark=? lens_zero=? rot=? scan=? carriers=? (need >= 150)
- relog probe 9: red=0 lens_red=? lens_black=? lens_water=? lens_dark=? lens_zero=? rot=? scan=? carriers=? (need >= 150)
- relog probe 10: red=0 lens_red=? lens_black=? lens_water=? lens_dark=? lens_zero=? rot=? scan=? carriers=? (need >= 150)
- relog probe 11: red=0 lens_red=? lens_black=? lens_water=? lens_dark=? lens_zero=? rot=? scan=? carriers=? (need >= 150)
- **FAIL:** marker lost after relog — stream state is not persistent/deterministic

### client log forensics (pipeline/particle/atlas — v6.8.2)
```
[19:24:46] [main/DEBUG]: Loading LWJGL system
WARNING: java.lang.System::load has been called by org.lwjgl.system.Library$$Lambda/0x00000000531e3d60 in an unnamed module (file:/home/runner/mc-test/client/libraries/org/lwjgl/lwjgl/3.4.3/lwjgl-3.4.3.jar)
WARNING: sun.misc.Unsafe::objectFieldOffset has been called by org.lwjgl.system.MemoryBackendUnsafeLegacy (file:/home/runner/mc-test/client/libraries/org/lwjgl/lwjgl/3.4.3/lwjgl-3.4.3.jar)
WARNING: Please consider reporting this to the maintainers of class org.lwjgl.system.MemoryBackendUnsafeLegacy
[19:24:46] [main/DEBUG]: Loading OpenGL
[19:24:46] [main/DEBUG]: Loading shaderc
[19:24:51] [Render thread/ERROR]: Error parsing option value 4 for option Simulation Distance: Value 4 outside of range [5:33]
[19:24:51] [Render thread/INFO]: Backend library: LWJGL version 3.4.3+4
[19:24:51] [Render thread/INFO]: Using graphics backend OpenGL, using drivers: 4.5 (Core Profile) Mesa 25.2.8-0ubuntu0.24.04.4
[19:24:51] [Render thread/INFO]: Using graphics device: llvmpipe (LLVM 20.1.2, 256 bits) (Mesa)
[19:24:51] [Render thread/INFO]: Using graphics device extensions: GL_ARB_multi_draw_indirect, GL_ARB_buffer_storage, GL_ARB_shader_draw_parameters, GL_ARB_base_instance, GL_KHR_debug, GL_ARB_draw_indirect, GL_ARB_clip_control, GL_ARB_vertex_attrib_binding, GL_ARB_direct_state_access, GL_EXT_texture_filter_anisotropic
[19:24:51] [Download-2/ERROR]: Failed to fetch user properties
com.mojang.authlib.exceptions.InvalidCredentialsException: Status: 401
	at com.mojang.authlib.exceptions.MinecraftClientHttpException.toAuthenticationException(MinecraftClientHttpException.java:70)
Caused by: MinecraftClientHttpException[type=HTTP_ERROR, status=401, response=ErrorResponse[path=/player/attributes, error=null, errorMessage=null, details=null]]
	at com.mojang.authlib.minecraft.client.MinecraftClient.buildHttpException(MinecraftClient.java:170)
[19:24:51] [Render thread/INFO]: Reloading ResourceManager: vanilla, vanilla
[19:24:51] [Download-2/INFO]: Could not authorize you against Realms server: java.lang.RuntimeException: Failed to parse into SignedJWT: 0
[19:24:51] [Download-2/ERROR]: Failed to fetch Realms feature flags
com.mojang.realmsclient.exception.RealmsServiceException: Realms authentication error with message 'java.lang.RuntimeException: Failed to parse into SignedJWT: 0'
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
[19:24:54] [Render thread/ERROR]: Error starting SoundSystem. Turning off sounds & music
java.lang.IllegalStateException: Failed to open OpenAL device
	at net.minecraft.client.sounds.SoundEngine.reload(SoundEngine.java:110)
	at net.minecraft.server.packs.resources.SimplePreparableReloadListener.lambda$reload$1(SimplePreparableReloadListener.java:15)
	at net.minecraft.server.packs.resources.SimpleReloadInstance.lambda$prepareTasks$3(SimpleReloadInstance.java:59)
[19:24:54] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[19:24:54] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[19:24:54] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[19:24:54] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[19:24:54] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[19:24:54] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[19:24:54] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/blocks.png-atlas
[19:24:54] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[19:24:54] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[19:24:54] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[19:24:54] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[19:24:54] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
```

### /clod-cells AFTER PROBES (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 14 0 0x8400 0x3100
123 122 -940 784 71 0 1 [97, 119, 97] 14 0 0x8700 0x3100
124 122 -938 784 68 0 1 [97, 119, 97] 13 0 0x8400 0x3100
125 122 -936 784 62 1 1 [63, 118, 228] 13 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 12 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 12 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 11 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 11 0 0x8000 0x5c00
130 122 -926 784 66 0 1 [134, 183, 131] 10 0 0x8200 0x5c00
131 122 -924 784 75 0 1 [97, 119, 97] 9 0 0x8b00 0x3100
132 122 -922 784 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 122 -920 784 75 0 1 [97, 119, 97] 18446744073709551615 1 0x8b00 0x3100
134 122 -918 784 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x3100
122 123 -942 786 70 0 1 [97, 119, 97] 15 0 0x8600 0x3100
123 123 -940 786 65 0 1 [97, 119, 97] 14 0 0x8100 0x3100
124 123 -938 786 65 0 1 [97, 119, 97] 14 0 0x8100 0x3100
125 123 -936 786 62 1 1 [63, 118, 228] 13 0 0x7e00 0xd800
```
- edit channel: 0 linkage errors, 0 failed event dispatches (both must be 0)
