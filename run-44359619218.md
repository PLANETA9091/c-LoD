- phase: 0 sanity (17:32:48)
### c-LoD visual run (-930,796, marker=red_concrete, vantage -1005,150 yaw 270)
- GL:     Vendor: Mesa (0xffffffff);    Device: llvmpipe (LLVM 21.1.8, 256 bits) (0xffffffff);OpenGL renderer string: llvmpipe (LLVM 21.1.8, 256 bits);OpenGL core profile version string: 4.5 (Core Profile) Mesa 26.0.8-1ubuntu0.3;
- boot + gates OK (17:33:13)
- phase: 1 server booted (17:33:13)
- sky-grid armed
- phase: 2 leg-1 client joined (17:33:40)
- joined session 1 (17:33:40)
- stream live: cells=65536
- phase: 3 scan populated (17:34:07)
- terrain range: 2 terraced mountains at (CX-4,CZ-20)/(CX-2,CZ+16), tops 185/190, grass terraces + stone caps, 66..82 m from the eye (inside the 83.5 m band, chunk edge 64 m)
- scan complete=1 (cells=cells=65536/65536)
- paint settled=1 (wire 0p)
- **FAIL:** rejoin1: client left the game mid-test (joins=2 lefts=2) — the shot is the main-menu panorama, not the LOD proof

### boot / stream
```
[c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 147077 bytes, sha1 853cf451c7edc996df4bd1db31e7333382182477 — zero textures, zero world data, one push per join
[17:33:10 INFO]: [c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 147077 bytes, sha1 853cf451c7edc996df4bd1db31e7333382182477 — zero textures, zero world data, one push per join
[c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[17:33:10 INFO]: [c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[c-LoD] stream: tick 800 65536/65536 cells, lap 442, cone 593 zone 778 dirty 801 (q 11113) ewin 257 rbeat 29p/1 rearm 80c/2801t rescan 51549c/1441e, edits +5279p/3532c/452e drop 0, spawned 0 / failed 0, cam0 none, wish 310 forced 202, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 27158p/36b fail 0 ~0 kB/s cache 13341/24576 h13817 m13341 e0, srbeat 1154s/138q drop 23stale/0undec, ereb 33s/870q, sweep 0r/0d
[17:37:04 INFO]: [c-LoD] stream: tick 800 65536/65536 cells, lap 442, cone 593 zone 778 dirty 801 (q 11113) ewin 257 rbeat 29p/1 rearm 80c/2801t rescan 51549c/1441e, edits +5279p/3532c/452e drop 0, spawned 0 / failed 0, cam0 none, wish 310 forced 202, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 27158p/36b fail 0 ~0 kB/s cache 13341/24576 h13817 m13341 e0, srbeat 1154s/138q drop 23stale/0undec, ereb 33s/870q, sweep 0r/0d
```
### sessions
```
[17:33:40 INFO]: System chat: Stepan joined the game
[17:34:20 INFO]: System chat: Stepan left the game
[17:34:56 INFO]: System chat: Stepan joined the game
[17:35:05 INFO]: System chat: Stepan left the game
```
### client log tail
```
[17:35:04] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[17:35:04] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[17:35:04] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[17:35:04] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[17:35:04] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[17:35:04] [Render thread/WARN]: Requested post effect does not exist: minecraft:end_of_frame
[17:35:05] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:section_blocks_update] net.minecraft.network.protocol.game.ClientboundSectionBlocksUpdatePacket -> 16 bytes
[17:35:05] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:block_update] net.minecraft.network.protocol.game.ClientboundBlockUpdatePacket -> 10 bytes
[17:35:05] [Worker-Main-2/ERROR]: Couldn't compile pipeline (clod:lod_terrain/10): 
com.mojang.renderpearl.util.ShaderCompileException: Couldn't parse GLSL: clod:post/debug_lens:62: error: '' :  syntax error, unexpected IDENTIFIER

	at com.mojang.renderpearl.frontend.shaders.GlslCompiler.compileToSpv(GlslCompiler.java:140)
	at com.mojang.renderpearl.frontend.shaders.PipelineBuilder.generateBackendCreateInfo(PipelineBuilder.java:76)
	at com.mojang.renderpearl.frontend.shaders.PipelineBuilder.lambda$compilePipeline$0(PipelineBuilder.java:329)
	at java.base/java.util.concurrent.CompletableFuture$AsyncSupply.run(CompletableFuture.java:1789)
	at java.base/java.util.concurrent.CompletableFuture$AsyncSupply.exec(CompletableFuture.java:1781)
	at java.base/java.util.concurrent.ForkJoinTask.doExec(ForkJoinTask.java:511)
	at java.base/java.util.concurrent.ForkJoinPool$WorkQueue.topLevelExec(ForkJoinPool.java:1450)
	at java.base/java.util.concurrent.ForkJoinPool.runWorker(ForkJoinPool.java:2019)
	at java.base/java.util.concurrent.ForkJoinWorkerThread.run(ForkJoinWorkerThread.java:187)
[17:35:05] [Worker-Main-2/ERROR]: Couldn't preload shader clod:shaders/post/debug_lens.fsh
java.io.FileNotFoundException: clod:shaders/post/debug_lens.fsh
	at net.minecraft.server.packs.resources.ResourceProvider.lambda$getResourceOrThrow$0(ResourceProvider.java:21)
	at java.base/java.util.Optional.orElseThrow(Optional.java:403)
	at net.minecraft.server.packs.resources.ResourceProvider.getResourceOrThrow(ResourceProvider.java:21)
	at net.minecraft.client.renderer.GameRenderer$1.getShader(GameRenderer.java:261)
	at com.mojang.renderpearl.frontend.shaders.PipelineBuilder.loadShaderSource(PipelineBuilder.java:365)
	at com.mojang.renderpearl.frontend.shaders.PipelineBuilder.generateBackendCreateInfo(PipelineBuilder.java:70)
	at com.mojang.renderpearl.frontend.shaders.PipelineBuilder.lambda$compilePipeline$0(PipelineBuilder.java:329)
	at java.base/java.util.concurrent.CompletableFuture$AsyncSupply.run(CompletableFuture.java:1789)
	at java.base/java.util.concurrent.CompletableFuture$AsyncSupply.exec(CompletableFuture.java:1781)
	at java.base/java.util.concurrent.ForkJoinTask.doExec(ForkJoinTask.java:511)
	at java.base/java.util.concurrent.ForkJoinPool$WorkQueue.topLevelExec(ForkJoinPool.java:1450)
	at java.base/java.util.concurrent.ForkJoinPool.runWorker(ForkJoinPool.java:2019)
	at java.base/java.util.concurrent.ForkJoinWorkerThread.run(ForkJoinWorkerThread.java:187)
[17:35:05] [Worker-Main-2/ERROR]: Couldn't find source for FRAGMENT shader (clod:post/debug_lens)
[17:35:05] [Worker-Main-2/ERROR]: Couldn't find source for FRAGMENT shader (clod:post/debug_lens)
[17:35:05] [Render thread/ERROR]: Failed to load post chain: clod:lod_terrain
net.minecraft.client.renderer.ShaderManager$CompilationException: Failed to compile post processing pipeline clod:lod_terrain/10
	at net.minecraft.client.renderer.PostChain.createPass(PostChain.java:101)
	at net.minecraft.client.renderer.PostChain.load(PostChain.java:66)
	at net.minecraft.client.renderer.ShaderManager$PostChainCache.loadPostChain(ShaderManager.java:340)
	at net.minecraft.client.renderer.ShaderManager$PostChainCache.getOrLoadPostChain(ShaderManager.java:327)
	at net.minecraft.client.renderer.ShaderManager.getPostChain(ShaderManager.java:259)
	at net.minecraft.client.renderer.GameRenderer.preparePostEffects(GameRenderer.java:541)
	at net.minecraft.client.renderer.GameRenderer.render(GameRenderer.java:491)
	at net.minecraft.client.Minecraft.renderFrame(Minecraft.java:1376)
	at net.minecraft.client.Minecraft.runTick(Minecraft.java:1274)
	at net.minecraft.client.Minecraft.run(Minecraft.java:967)
	at net.minecraft.client.main.Main.main(Main.java:292)
[17:35:05] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:light_update] net.minecraft.network.protocol.game.ClientboundLightUpdatePacket -> 4115 bytes
[17:35:05] [Render thread/INFO]: Reloading ResourceManager: vanilla, vanilla
[17:35:05] [Worker-Main-3/INFO]: Found unifont_pua-17.0.01.hex, loading
[17:35:05] [Worker-Main-1/INFO]: Found unifont_all_no_pua-17.0.01.hex, loading
[17:35:05] [Worker-Main-1/INFO]: Found unifont_jp_patch-17.0.01.hex, loading
[17:35:06] [Download-2/INFO]: Could not authorize you against Realms server: java.lang.RuntimeException: Failed to parse into SignedJWT: 0
[17:35:06] [Download-2/ERROR]: Couldn't connect to realms
com.mojang.realmsclient.exception.RealmsServiceException: Realms authentication error with message 'java.lang.RuntimeException: Failed to parse into SignedJWT: 0'
	at com.mojang.realmsclient.client.RealmsClient.execute(RealmsClient.java:625)
	at com.mojang.realmsclient.client.RealmsClient.clientCompatible(RealmsClient.java:354)
	at com.mojang.realmsclient.RealmsAvailability.lambda$check$0(RealmsAvailability.java:48)
	at java.base/java.util.concurrent.CompletableFuture$AsyncSupply.run(CompletableFuture.java:1789)
	at java.base/java.util.concurrent.ThreadPoolExecutor.runWorker(ThreadPoolExecutor.java:1090)
	at java.base/java.util.concurrent.ThreadPoolExecutor$Worker.run(ThreadPoolExecutor.java:614)
	at java.base/java.lang.Thread.run(Thread.java:1474)
[ALSOFT] (EE) Could not query RTKit: No such file or directory (2)
[17:35:07] [Render thread/INFO]: OpenAL initialized on device No Output
[17:35:07] [Render thread/INFO]: Sound engine started
[17:35:07] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[17:35:07] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[17:35:07] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[17:35:07] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[17:35:07] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[17:35:07] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[17:35:07] [Render thread/INFO]: Created: 2048x2048x4 minecraft:textures/atlas/blocks.png-atlas
[17:35:07] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[17:35:07] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[17:35:07] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[17:35:07] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[17:35:07] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
--- exceptions (Realms/RTKit noise filtered) ---
25:[17:34:50] [Render thread/ERROR]: Error parsing option value 4 for option Simulation Distance: Value 4 outside of range [5:33]
39:[17:34:50] [Download-2/ERROR]: Failed to fetch user properties
40:com.mojang.authlib.exceptions.InvalidCredentialsException: Status: 401
41:	at com.mojang.authlib.exceptions.MinecraftClientHttpException.toAuthenticationException(MinecraftClientHttpException.java:70)
48:Caused by: MinecraftClientHttpException[type=HTTP_ERROR, status=401, response=ErrorResponse[path=/player/attributes, error=null, errorMessage=null, details=null]]
49:	at com.mojang.authlib.minecraft.client.MinecraftClient.buildHttpException(MinecraftClient.java:170)
14850:[17:35:05] [Worker-Main-2/ERROR]: Couldn't compile pipeline (clod:lod_terrain/10): 
14851:com.mojang.renderpearl.util.ShaderCompileException: Couldn't parse GLSL: clod:post/debug_lens:62: error: '' :  syntax error, unexpected IDENTIFIER
14862:[17:35:05] [Worker-Main-2/ERROR]: Couldn't preload shader clod:shaders/post/debug_lens.fsh
14863:java.io.FileNotFoundException: clod:shaders/post/debug_lens.fsh
14877:[17:35:05] [Worker-Main-2/ERROR]: Couldn't find source for FRAGMENT shader (clod:post/debug_lens)
14878:[17:35:05] [Worker-Main-2/ERROR]: Couldn't find source for FRAGMENT shader (clod:post/debug_lens)
14879:[17:35:05] [Render thread/ERROR]: Failed to load post chain: clod:lod_terrain
14880:net.minecraft.client.renderer.ShaderManager$CompilationException: Failed to compile post processing pipeline clod:lod_terrain/10
14898:[17:35:06] [Download-2/ERROR]: Couldn't connect to realms
```
### server log tail
```
[17:38:23 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:overworld'
[c-LoD] EB: enter, native_gen=true
[17:38:24 INFO]: [ChunkHolderManager] Saved 897 block chunks, 514 entity chunks, 13 poi chunks in world 'minecraft:overworld' in 1.26s
[17:38:24 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:overworld'
[17:38:24 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:overworld'
[17:38:24 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[17:38:24 INFO]: [ChunkHolderManager] Waiting 60s for chunk system to halt for world 'minecraft:the_nether'
[17:38:24 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:the_nether'
[17:38:24 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_nether'
[17:38:24 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_nether' in 0.00s
[17:38:24 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_nether'
[17:38:24 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_nether'
[17:38:24 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[17:38:24 INFO]: [ChunkHolderManager] Waiting 60s for chunk system to halt for world 'minecraft:the_end'
[17:38:24 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:the_end'
[17:38:24 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_end'
[17:38:24 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_end' in 0.00s
[17:38:24 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_end'
[17:38:24 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_end'
[17:38:24 INFO]: Waiting for all RegionFile I/O tasks to complete...
[17:38:24 INFO]: All RegionFile I/O tasks to complete
[17:38:24 INFO]: [MoonriseCommon] Awaiting termination of worker pool for up to 60s...
[17:38:24 INFO]: [MoonriseCommon] Awaiting termination of I/O pool for up to 60s...
[c-LoD] EB: enter, native_gen=true
[launcher] server exited with code 143
```
### shots
```
total 84
drwxr-xr-x 2 runner runner  4096 Nov 18 17:38 .
drwxr-xr-x 3 runner runner  4096 Nov 18 17:32 ..
-rw-r--r-- 1 runner runner 59535 Nov 18 17:38 server_full.log
-rw-r--r-- 1 runner runner 14375 Nov 18 17:38 summary.md
```

**exit=1** failures=1 leg1=0 leg2=0
