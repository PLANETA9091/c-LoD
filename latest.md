- phase: 0 sanity (19:55:42)
### c-LoD visual run (-930,796, marker=red_concrete, vantage -1005,150 yaw 270)
- GL:     Vendor: Mesa (0xffffffff);    Device: llvmpipe (LLVM 21.1.8, 256 bits) (0xffffffff);OpenGL renderer string: llvmpipe (LLVM 21.1.8, 256 bits);OpenGL core profile version string: 4.5 (Core Profile) Mesa 26.0.8-1ubuntu0.3;
- boot + gates OK (19:56:23)
- phase: 1 server booted (19:56:23)
- sky-grid armed
- phase: 2 leg-1 client joined (19:56:52)
- joined session 1 (19:56:52)
- stream live: cells=65536
- phase: 3 scan populated (19:57:19)
- terrain range: 2 terraced mountains at (CX-4,CZ-20)/(CX-2,CZ+16), tops 185/190, grass terraces + stone caps, 66..82 m from the eye (inside the 83.5 m band, chunk edge 64 m)
- scan complete=1 (cells=cells=65536/65536)
- paint settled=1 (wire 0p)
- **FAIL:** rejoin1: client left the game mid-test (joins=1 lefts=1) — the shot is the main-menu panorama, not the LOD proof

### boot / stream
```
[c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 153603 bytes, sha1 2867a7a3d6fcecee18045acb46d0b51c4e117fa4 — zero textures, zero world data, one push per join
[19:56:21 INFO]: [c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 153603 bytes, sha1 2867a7a3d6fcecee18045acb46d0b51c4e117fa4 — zero textures, zero world data, one push per join
[c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[19:56:21 INFO]: [c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[c-LoD] stream: tick 800 65536/65536 cells, lap 325, cone 0 zone 768 dirty 684 (q 6352) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 49862c/716e, edits +1146p/834c/141e drop 0, spawned 0 / failed 0, cas 0d, cam0 none, wish 395 forced 117, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 12220p/12b fail 0 ~0 kB/s cache 12220/24576 h0 m12220 e0, srbeat 0s/1060q drop 0stale/0undec, ereb 0s/700q, sweep 0r/0d, cdiag off
[20:01:00 INFO]: [c-LoD] stream: tick 800 65536/65536 cells, lap 325, cone 0 zone 768 dirty 684 (q 6352) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 49862c/716e, edits +1146p/834c/141e drop 0, spawned 0 / failed 0, cas 0d, cam0 none, wish 395 forced 117, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 12220p/12b fail 0 ~0 kB/s cache 12220/24576 h0 m12220 e0, srbeat 0s/1060q drop 0stale/0undec, ereb 0s/700q, sweep 0r/0d, cdiag off
```
### sessions
```
[19:56:52 INFO]: System chat: Stepan joined the game
[19:57:33 INFO]: System chat: Stepan left the game
```
### client log tail
```
com.mojang.authlib.exceptions.InvalidCredentialsException: Status: 401
	at com.mojang.authlib.exceptions.MinecraftClientHttpException.toAuthenticationException(MinecraftClientHttpException.java:70)
	at com.mojang.authlib.services.MinecraftServicesUserApiService.fetchProperties(MinecraftServicesUserApiService.java:168)
	at net.minecraft.client.Minecraft.lambda$new$1(Minecraft.java:451)
	at java.base/java.util.concurrent.CompletableFuture$AsyncSupply.run(CompletableFuture.java:1789)
	at java.base/java.util.concurrent.ThreadPoolExecutor.runWorker(ThreadPoolExecutor.java:1090)
	at java.base/java.util.concurrent.ThreadPoolExecutor$Worker.run(ThreadPoolExecutor.java:614)
	at java.base/java.lang.Thread.run(Thread.java:1474)
Caused by: MinecraftClientHttpException[type=HTTP_ERROR, status=401, response=ErrorResponse[path=/player/attributes, error=null, errorMessage=null, details=null]]
	at com.mojang.authlib.minecraft.client.MinecraftClient.buildHttpException(MinecraftClient.java:170)
	at com.mojang.authlib.minecraft.client.MinecraftClient.readServiceResponse(MinecraftClient.java:147)
	at com.mojang.authlib.minecraft.client.MinecraftClient.getWithEtag(MinecraftClient.java:81)
	at com.mojang.authlib.minecraft.client.MinecraftClient.get(MinecraftClient.java:65)
	at com.mojang.authlib.services.MinecraftServicesUserApiService.fetchProperties(MinecraftServicesUserApiService.java:118)
	... 5 more
[libclodgl] glXChooseFBConfig called (9 attribs)
[libclodgl] glXChooseFBConfig -> configs
[libclodgl] missing real symbol: glNamedFramebufferTextureMultiviewOVR
[19:58:08] [Render thread/INFO]: Using graphics backend OpenGL, using drivers: 4.5 (Core Profile) Mesa 26.0.8-1ubuntu0.3
[19:58:08] [Render thread/INFO]: Using graphics device: llvmpipe (LLVM 21.1.8, 256 bits) (Mesa)
[19:58:08] [Render thread/INFO]: Using graphics device extensions: GL_ARB_multi_draw_indirect, GL_ARB_buffer_storage, GL_ARB_shader_draw_parameters, GL_ARB_base_instance, GL_KHR_debug, GL_ARB_draw_indirect, GL_ARB_clip_control, GL_ARB_vertex_attrib_binding, GL_ARB_direct_state_access, GL_EXT_texture_filter_anisotropic
[libclodgl] glXChooseFBConfig called (9 attribs)
[libclodgl] glXChooseFBConfig -> configs
[19:58:08] [Render thread/INFO]: Created window using SDL video driver: x11
[19:58:09] [Render thread/DEBUG]: Using polling for tracking audio device changes
[19:58:09] [Download-1/INFO]: Could not authorize you against Realms server: java.lang.RuntimeException: Failed to parse into SignedJWT: 0
[19:58:09] [Download-1/ERROR]: Failed to fetch Realms feature flags
com.mojang.realmsclient.exception.RealmsServiceException: Realms authentication error with message 'java.lang.RuntimeException: Failed to parse into SignedJWT: 0'
	at com.mojang.realmsclient.client.RealmsClient.execute(RealmsClient.java:625)
	at com.mojang.realmsclient.client.RealmsClient.fetchFeatureFlags(RealmsClient.java:204)
	at java.base/java.util.concurrent.CompletableFuture$AsyncSupply.run(CompletableFuture.java:1789)
	at java.base/java.util.concurrent.ThreadPoolExecutor.runWorker(ThreadPoolExecutor.java:1090)
	at java.base/java.util.concurrent.ThreadPoolExecutor$Worker.run(ThreadPoolExecutor.java:614)
	at java.base/java.lang.Thread.run(Thread.java:1474)
[19:58:10] [Render thread/INFO]: Reloading ResourceManager: vanilla, vanilla
[19:58:10] [Render thread/DEBUG]: Growing IndexBuffer: Old limit 0, new limit 60.
[19:58:11] [Worker-Main-3/INFO]: Found unifont_all_no_pua-17.0.01.hex, loading
[19:58:11] [Worker-Main-2/INFO]: Found unifont_pua-17.0.01.hex, loading
[19:58:11] [Worker-Main-3/INFO]: Found unifont_jp_patch-17.0.01.hex, loading
[ALSOFT] (EE) Could not query RTKit: No such file or directory (2)
[19:58:16] [Render thread/INFO]: OpenAL initialized on device No Output
[19:58:16] [Render thread/INFO]: Sound engine started
[19:58:16] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[19:58:16] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[19:58:16] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[19:58:16] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[19:58:16] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[19:58:16] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[19:58:16] [Render thread/INFO]: Created: 2048x2048x4 minecraft:textures/atlas/blocks.png-atlas
[19:58:16] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[19:58:16] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[19:58:16] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[19:58:16] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[19:58:16] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
[19:58:17] [Render thread/INFO]: Connecting to 127.0.0.1, 25565
[19:58:18] [Server Connector #1/ERROR]: Couldn't connect to server
io.netty.channel.AbstractChannel$AnnotatedConnectException: finishConnect(..) failed with error(-111): Connection refused: localhost/127.0.0.1:25565
	Suppressed: java.util.concurrent.CompletionException: Rethrowing promise failure cause
		at io.netty.util.concurrent.DefaultPromise.rethrowIfFailed(DefaultPromise.java:686)
		at io.netty.util.concurrent.DefaultPromise.syncUninterruptibly(DefaultPromise.java:427)
		at io.netty.channel.DefaultChannelPromise.syncUninterruptibly(DefaultChannelPromise.java:125)
		at io.netty.channel.DefaultChannelPromise.syncUninterruptibly(DefaultChannelPromise.java:30)
		at net.minecraft.client.gui.screens.ConnectScreen$1.run(ConnectScreen.java:115)
Caused by: java.net.ConnectException: finishConnect(..) failed with error(-111): Connection refused
	at io.netty.channel.unix.Errors.newConnectException0(Errors.java:166)
	at io.netty.channel.unix.Errors.handleConnectErrno(Errors.java:131)
	at io.netty.channel.unix.Socket.finishConnect(Socket.java:361)
	at io.netty.channel.epoll.AbstractEpollChannel$AbstractEpollUnsafe.doFinishConnect(AbstractEpollChannel.java:772)
	at io.netty.channel.epoll.AbstractEpollChannel$AbstractEpollUnsafe.finishConnect(AbstractEpollChannel.java:749)
	at io.netty.channel.epoll.AbstractEpollChannel$AbstractEpollUnsafe.epollOutReady(AbstractEpollChannel.java:616)
	at io.netty.channel.epoll.AbstractEpollChannel$AbstractEpollUnsafe.handle(AbstractEpollChannel.java:477)
	at io.netty.channel.epoll.EpollIoHandler$DefaultEpollIoRegistration.handle(EpollIoHandler.java:349)
	at io.netty.channel.epoll.EpollIoHandler.processReady(EpollIoHandler.java:548)
	at io.netty.channel.epoll.EpollIoHandler.run(EpollIoHandler.java:491)
	at io.netty.channel.SingleThreadIoEventLoop.runIo(SingleThreadIoEventLoop.java:225)
	at io.netty.channel.SingleThreadIoEventLoop.run(SingleThreadIoEventLoop.java:196)
	at io.netty.util.concurrent.SingleThreadEventExecutor$5.run(SingleThreadEventExecutor.java:1204)
	at io.netty.util.internal.ThreadExecutorMap$2.run(ThreadExecutorMap.java:74)
	at java.base/java.lang.Thread.run(Thread.java:1474)
[19:58:18] [Render thread/DEBUG]: Growing IndexBuffer: Old limit 60, new limit 2136.
--- exceptions (Realms/RTKit noise filtered) ---
25:[19:58:08] [Render thread/ERROR]: Error parsing option value 4 for option Simulation Distance: Value 4 outside of range [5:17]
39:[19:58:08] [Download-2/ERROR]: Failed to fetch user properties
40:com.mojang.authlib.exceptions.InvalidCredentialsException: Status: 401
41:	at com.mojang.authlib.exceptions.MinecraftClientHttpException.toAuthenticationException(MinecraftClientHttpException.java:70)
48:Caused by: MinecraftClientHttpException[type=HTTP_ERROR, status=401, response=ErrorResponse[path=/player/attributes, error=null, errorMessage=null, details=null]]
49:	at com.mojang.authlib.minecraft.client.MinecraftClient.buildHttpException(MinecraftClient.java:170)
95:[19:58:18] [Server Connector #1/ERROR]: Couldn't connect to server
96:io.netty.channel.AbstractChannel$AnnotatedConnectException: finishConnect(..) failed with error(-111): Connection refused: localhost/127.0.0.1:25565
97:	Suppressed: java.util.concurrent.CompletionException: Rethrowing promise failure cause
103:Caused by: java.net.ConnectException: finishConnect(..) failed with error(-111): Connection refused
104:	at io.netty.channel.unix.Errors.newConnectException0(Errors.java:166)
```
### server log tail
```
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[c-LoD] engine tick alive: tick 850
[c-LoD] EB: enter, native_gen=true
[20:01:50 INFO]: [c-LoD] engine tick alive: tick 850
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
```
### shots
```
total 104
drwxr-xr-x 2 runner runner  4096 Nov 19 20:02 .
drwxr-xr-x 3 runner runner  4096 Nov 19 19:54 ..
-rw-r--r-- 1 runner runner 84373 Nov 19 20:02 server_full.log
-rw-r--r-- 1 runner runner 12041 Nov 19 20:02 summary.md
```

**exit=1** failures=1 leg1=0 leg2=0
