### c-LoD visual run (-930,796, marker=red_concrete, vantage -1005,150 yaw 270)
- GL:     Vendor: Mesa (0xffffffff);    Device: llvmpipe (LLVM 20.1.2, 256 bits) (0xffffffff);OpenGL renderer string: llvmpipe (LLVM 20.1.2, 256 bits);OpenGL core profile version string: 4.5 (Core Profile) Mesa 25.2.8-0ubuntu0.24.04.4;
- boot + gates OK (19:15:38)
- sky-grid armed
- joined session 1 (19:16:01)
- stream live: cells=65536
- scan complete=1 (cells=cells=65536/65536)
- paint settled=0 (wire 704332p)
- baseline try 1: drift=0 px
- burst_close white px: 5405 (renderer alive if > 50)
- burst_far white px: 5405 (renderer alive in the marker zone if > 20)
- fill issued: 5x111x5 red_concrete at -930,796 (zone forceloaded ±48)

### /clod-cells AFTER FILL (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 74 0 0x8400 0x3100
123 122 -940 784 71 0 1 [97, 119, 97] 74 0 0x8700 0x3100
124 122 -938 784 68 0 1 [97, 119, 97] 74 0 0x8400 0x3100
125 122 -936 784 62 1 1 [63, 118, 228] 74 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 74 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 74 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 74 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 55 0 0x8000 0x5c00
130 122 -926 784 66 0 1 [134, 183, 131] 55 0 0x8200 0x5c00
131 122 -924 784 75 0 1 [97, 119, 97] 55 0 0x8b00 0x3100
132 122 -922 784 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 122 -920 784 75 0 1 [97, 119, 97] 18446744073709551615 1 0x8b00 0x3100
134 122 -918 784 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x3100
122 123 -942 786 65 0 1 [97, 119, 97] 74 0 0x8100 0x3100
123 123 -940 786 65 0 1 [97, 119, 97] 74 0 0x8100 0x3100
124 123 -938 786 65 0 1 [97, 119, 97] 74 0 0x8100 0x3100
125 123 -936 786 62 1 1 [63, 118, 228] 74 0 0x7e00 0xd800
```
### stream diag after fill:
[c-LoD] stream: tick 4900 65536/65536 cells, lap 4635, cone 97145 zone 640 dirty 1220 (q 12610) ewin 239 rbeat 0p/1 rescan 1217945c/1375e, edits +4289p/3220c/684e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 299 forced 213, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 789433p/2786b fail 0 ~0 kB/s cache 15832/24576 h773601 m15832 e0
[19:23:57 INFO]: [c-LoD] stream: tick 4900 65536/65536 cells, lap 4635, cone 97145 zone 640 dirty 1220 (q 12610) ewin 239 rbeat 0p/1 rescan 1217945c/1375e, edits +4289p/3220c/684e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 299 forced 213, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 789433p/2786b fail 0 ~0 kB/s cache 15832/24576 h773601 m15832 e0
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
[19:26:38 INFO]: [c-LoD] stream: tick 6500 65536/65536 cells, lap 5433, cone 115195 zone 1280 dirty 1273 (q 12591) ewin 0 rbeat 0p/0 rescan 1734787c/1449e, edits +5677p/4175c/954e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 299 forced 213, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 938702p/2945b fail 0 ~112 kB/s cache 15834/24576 h922868 m15834 e0
[c-LoD] stream: tick 6600 65536/65536 cells, lap 5533, cone 116786 zone 1280 dirty 1273 (q 12591) ewin 0 rbeat 0p/0 rescan 1765896c/1449e, edits +5679p/4177c/954e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 299 forced 213, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 950329p/2957b fail 0 ~61 kB/s cache 15834/24576 h934495 m15834 e0
[19:26:48 INFO]: [c-LoD] stream: tick 6600 65536/65536 cells, lap 5533, cone 116786 zone 1280 dirty 1273 (q 12591) ewin 0 rbeat 0p/0 rescan 1765896c/1449e, edits +5679p/4177c/954e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 299 forced 213, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 950329p/2957b fail 0 ~61 kB/s cache 15834/24576 h934495 m15834 e0
- joined session 2 (19:27:14)
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
WARNING: java.lang.System::load has been called by org.lwjgl.system.Library$$Lambda/0x000000004c1e3d60 in an unnamed module (file:/home/runner/mc-test/client/libraries/org/lwjgl/lwjgl/3.4.3/lwjgl-3.4.3.jar)
WARNING: sun.misc.Unsafe::objectFieldOffset has been called by org.lwjgl.system.MemoryBackendUnsafeLegacy (file:/home/runner/mc-test/client/libraries/org/lwjgl/lwjgl/3.4.3/lwjgl-3.4.3.jar)
WARNING: Please consider reporting this to the maintainers of class org.lwjgl.system.MemoryBackendUnsafeLegacy
[19:27:01] [main/DEBUG]: Loading OpenGL
[19:27:01] [main/DEBUG]: Loading shaderc
[19:27:06] [Render thread/ERROR]: Error parsing option value 4 for option Simulation Distance: Value 4 outside of range [5:33]
[19:27:06] [Render thread/INFO]: Backend library: LWJGL version 3.4.3+4
[19:27:06] [Render thread/INFO]: Using graphics backend OpenGL, using drivers: 4.5 (Core Profile) Mesa 25.2.8-0ubuntu0.24.04.4
[19:27:06] [Render thread/INFO]: Using graphics device: llvmpipe (LLVM 20.1.2, 256 bits) (Mesa)
[19:27:06] [Render thread/INFO]: Using graphics device extensions: GL_ARB_multi_draw_indirect, GL_ARB_buffer_storage, GL_ARB_shader_draw_parameters, GL_ARB_base_instance, GL_KHR_debug, GL_ARB_draw_indirect, GL_ARB_clip_control, GL_ARB_vertex_attrib_binding, GL_ARB_direct_state_access, GL_EXT_texture_filter_anisotropic
[19:27:06] [Download-2/ERROR]: Failed to fetch user properties
com.mojang.authlib.exceptions.InvalidCredentialsException: Status: 401
	at com.mojang.authlib.exceptions.MinecraftClientHttpException.toAuthenticationException(MinecraftClientHttpException.java:70)
Caused by: MinecraftClientHttpException[type=HTTP_ERROR, status=401, response=ErrorResponse[path=/player/attributes, error=null, errorMessage=null, details=null]]
	at com.mojang.authlib.minecraft.client.MinecraftClient.buildHttpException(MinecraftClient.java:170)
[19:27:07] [Download-2/INFO]: Could not authorize you against Realms server: java.lang.RuntimeException: Failed to parse into SignedJWT: 0
[19:27:07] [Download-2/ERROR]: Failed to fetch Realms feature flags
com.mojang.realmsclient.exception.RealmsServiceException: Realms authentication error with message 'java.lang.RuntimeException: Failed to parse into SignedJWT: 0'
[19:27:07] [Render thread/INFO]: Reloading ResourceManager: vanilla, vanilla
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
[19:27:09] [Render thread/ERROR]: Error starting SoundSystem. Turning off sounds & music
java.lang.IllegalStateException: Failed to open OpenAL device
	at net.minecraft.client.sounds.SoundEngine.reload(SoundEngine.java:110)
	at net.minecraft.server.packs.resources.SimplePreparableReloadListener.lambda$reload$1(SimplePreparableReloadListener.java:15)
	at net.minecraft.server.packs.resources.SimpleReloadInstance.lambda$prepareTasks$3(SimpleReloadInstance.java:59)
[19:27:09] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[19:27:09] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[19:27:09] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[19:27:09] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[19:27:09] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[19:27:09] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[19:27:09] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/blocks.png-atlas
[19:27:09] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[19:27:09] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[19:27:09] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[19:27:09] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[19:27:09] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
com.mojang.authlib.exceptions.MinecraftClientException: Failed to read from https://sessionserver.mojang.com/session/minecraft/profile/d1652ad977ce32009642f973c97adf36?unsigned=false due to Read timed out
Caused by: java.net.SocketTimeoutException: Read timed out
```

### /clod-cells AFTER PROBES (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 1 0 0x8400 0x3100
123 122 -940 784 71 0 1 [97, 119, 97] 1 0 0x8700 0x3100
124 122 -938 784 68 0 1 [97, 119, 97] 0 0 0x8400 0x3100
125 122 -936 784 62 1 1 [63, 118, 228] 0 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 59 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 59 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 58 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 58 0 0x8000 0x5c00
130 122 -926 784 66 0 1 [134, 183, 131] 57 0 0x8200 0x5c00
131 122 -924 784 75 0 1 [97, 119, 97] 56 0 0x8b00 0x3100
132 122 -922 784 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 122 -920 784 75 0 1 [97, 119, 97] 18446744073709551615 1 0x8b00 0x3100
134 122 -918 784 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x3100
122 123 -942 786 65 0 1 [97, 119, 97] 2 0 0x8100 0x3100
123 123 -940 786 65 0 1 [97, 119, 97] 1 0 0x8100 0x3100
124 123 -938 786 65 0 1 [97, 119, 97] 1 0 0x8100 0x3100
125 123 -936 786 62 1 1 [63, 118, 228] 0 0 0x7e00 0xd800
```
- edit channel: 0 linkage errors, 0 failed event dispatches (both must be 0)
