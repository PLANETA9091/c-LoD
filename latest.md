- phase: 0 sanity (16:35:04)
### c-LoD visual run (-930,796, marker=red_concrete, vantage -1005,150 yaw 270)
- GL:     Vendor: Mesa (0xffffffff);    Device: llvmpipe (LLVM 21.1.8, 256 bits) (0xffffffff);OpenGL renderer string: llvmpipe (LLVM 21.1.8, 256 bits);OpenGL core profile version string: 4.5 (Core Profile) Mesa 26.0.8-1ubuntu0.3;
- boot + gates OK (16:35:33)
- phase: 1 server booted (16:35:33)
- sky-grid armed
- phase: 2 leg-1 client joined (16:36:00)
- joined session 1 (16:36:00)
- stream live: cells=65536
- phase: 3 scan populated (16:36:23)
- terrain range: 2 terraced mountains at (CX-4,CZ-20)/(CX-2,CZ+16), tops 185/190, grass terraces + stone caps, 66..82 m from the eye (inside the 83.5 m band, chunk edge 64 m)
- scan complete=1 (cells=cells=65536/65536)
- paint settled=1 (wire 0p)
- **FAIL:** rejoin1: client left the game mid-test (joins=2 lefts=2) — the shot is the main-menu panorama, not the LOD proof

### boot / stream
```
[c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 147077 bytes, sha1 853cf451c7edc996df4bd1db31e7333382182477 — zero textures, zero world data, one push per join
[16:35:29 INFO]: [c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 147077 bytes, sha1 853cf451c7edc996df4bd1db31e7333382182477 — zero textures, zero world data, one push per join
[c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[16:35:29 INFO]: [c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[c-LoD] stream: tick 800 65536/65536 cells, lap 416, cone 446 zone 809 dirty 758 (q 10419) ewin 256 rbeat 23p/1 rearm 80c/2787t rescan 56101c/2381e, edits +4816p/3092c/438e drop 0, spawned 0 / failed 0, cam0 none, wish 321 forced 191, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 27954p/34b fail 0 ~0 kB/s cache 13385/24576 h14569 m13385 e0, srbeat 1416s/105q drop 21stale/0undec, ereb 257s/833q, sweep 0r/0d
[16:39:33 INFO]: [c-LoD] stream: tick 800 65536/65536 cells, lap 416, cone 446 zone 809 dirty 758 (q 10419) ewin 256 rbeat 23p/1 rearm 80c/2787t rescan 56101c/2381e, edits +4816p/3092c/438e drop 0, spawned 0 / failed 0, cam0 none, wish 321 forced 191, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 27954p/34b fail 0 ~0 kB/s cache 13385/24576 h14569 m13385 e0, srbeat 1416s/105q drop 21stale/0undec, ereb 257s/833q, sweep 0r/0d
```
### sessions
```
[16:35:59 INFO]: System chat: Stepan joined the game
[16:36:36 INFO]: System chat: Stepan left the game
[16:37:12 INFO]: System chat: Stepan joined the game
[16:37:21 INFO]: System chat: Stepan left the game
```
### client log tail
```
[16:37:21] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:client_tick_end] net.minecraft.network.protocol.game.ServerboundClientTickEndPacket -> 1 bytes
[16:37:21] [Render thread/WARN]: Requested post effect does not exist: minecraft:end_of_frame
[16:37:21] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:section_blocks_update] net.minecraft.network.protocol.game.ClientboundSectionBlocksUpdatePacket -> 22 bytes
[16:37:21] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:block_update] net.minecraft.network.protocol.game.ClientboundBlockUpdatePacket -> 10 bytes
[16:37:21] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 10 bytes
[16:37:21] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:keep_alive] net.minecraft.network.protocol.common.ClientboundKeepAlivePacket -> 9 bytes
[16:37:21] [Netty Epoll IO #0/DEBUG]: OUT: [play:serverbound/minecraft:keep_alive] net.minecraft.network.protocol.common.ServerboundKeepAlivePacket -> 9 bytes
[16:37:21] [Worker-Main-2/ERROR]: Couldn't compile pipeline (clod:lod_terrain/10): 
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
[16:37:21] [Worker-Main-2/ERROR]: Couldn't preload shader clod:shaders/post/debug_lens.fsh
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
[16:37:21] [Worker-Main-2/ERROR]: Couldn't find source for FRAGMENT shader (clod:post/debug_lens)
[16:37:21] [Worker-Main-2/ERROR]: Couldn't find source for FRAGMENT shader (clod:post/debug_lens)
[16:37:21] [Render thread/ERROR]: Failed to load post chain: clod:lod_terrain
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
[16:37:21] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:light_update] net.minecraft.network.protocol.game.ClientboundLightUpdatePacket -> 4115 bytes
[16:37:21] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:block_update] net.minecraft.network.protocol.game.ClientboundBlockUpdatePacket -> 10 bytes
[16:37:21] [Render thread/INFO]: Reloading ResourceManager: vanilla, vanilla
[16:37:21] [Worker-Main-2/INFO]: Found unifont_all_no_pua-17.0.01.hex, loading
[16:37:21] [Worker-Main-3/INFO]: Found unifont_pua-17.0.01.hex, loading
[16:37:22] [Worker-Main-2/INFO]: Found unifont_jp_patch-17.0.01.hex, loading
[16:37:22] [Download-1/INFO]: Could not authorize you against Realms server: java.lang.RuntimeException: Failed to parse into SignedJWT: 0
[16:37:22] [Download-1/ERROR]: Couldn't connect to realms
com.mojang.realmsclient.exception.RealmsServiceException: Realms authentication error with message 'java.lang.RuntimeException: Failed to parse into SignedJWT: 0'
	at com.mojang.realmsclient.client.RealmsClient.execute(RealmsClient.java:625)
	at com.mojang.realmsclient.client.RealmsClient.clientCompatible(RealmsClient.java:354)
	at com.mojang.realmsclient.RealmsAvailability.lambda$check$0(RealmsAvailability.java:48)
	at java.base/java.util.concurrent.CompletableFuture$AsyncSupply.run(CompletableFuture.java:1789)
	at java.base/java.util.concurrent.ThreadPoolExecutor.runWorker(ThreadPoolExecutor.java:1090)
	at java.base/java.util.concurrent.ThreadPoolExecutor$Worker.run(ThreadPoolExecutor.java:614)
	at java.base/java.lang.Thread.run(Thread.java:1474)
[16:37:23] [Render thread/INFO]: OpenAL initialized on device No Output
[16:37:23] [Render thread/INFO]: Sound engine started
[ALSOFT] (EE) Could not query RTKit: No such file or directory (2)
[16:37:23] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[16:37:23] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[16:37:23] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[16:37:23] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[16:37:23] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[16:37:23] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[16:37:23] [Render thread/INFO]: Created: 2048x2048x4 minecraft:textures/atlas/blocks.png-atlas
[16:37:23] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[16:37:23] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[16:37:23] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[16:37:23] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[16:37:23] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
--- exceptions (Realms/RTKit noise filtered) ---
25:[16:37:06] [Render thread/ERROR]: Error parsing option value 4 for option Simulation Distance: Value 4 outside of range [5:33]
48:[16:37:06] [Download-2/ERROR]: Failed to fetch user properties
49:com.mojang.authlib.exceptions.InvalidCredentialsException: Status: 401
50:	at com.mojang.authlib.exceptions.MinecraftClientHttpException.toAuthenticationException(MinecraftClientHttpException.java:70)
57:Caused by: MinecraftClientHttpException[type=HTTP_ERROR, status=401, response=ErrorResponse[path=/player/attributes, error=null, errorMessage=null, details=null]]
58:	at com.mojang.authlib.minecraft.client.MinecraftClient.buildHttpException(MinecraftClient.java:170)
15877:[16:37:21] [Worker-Main-2/ERROR]: Couldn't compile pipeline (clod:lod_terrain/10): 
15878:com.mojang.renderpearl.util.ShaderCompileException: Couldn't parse GLSL: clod:post/debug_lens:62: error: '' :  syntax error, unexpected IDENTIFIER
15889:[16:37:21] [Worker-Main-2/ERROR]: Couldn't preload shader clod:shaders/post/debug_lens.fsh
15890:java.io.FileNotFoundException: clod:shaders/post/debug_lens.fsh
15904:[16:37:21] [Worker-Main-2/ERROR]: Couldn't find source for FRAGMENT shader (clod:post/debug_lens)
15905:[16:37:21] [Worker-Main-2/ERROR]: Couldn't find source for FRAGMENT shader (clod:post/debug_lens)
15906:[16:37:21] [Render thread/ERROR]: Failed to load post chain: clod:lod_terrain
15907:net.minecraft.client.renderer.ShaderManager$CompilationException: Failed to compile post processing pipeline clod:lod_terrain/10
15926:[16:37:22] [Download-1/ERROR]: Couldn't connect to realms
```
### server log tail
```
[16:40:39 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:overworld'
[c-LoD] EB: enter, native_gen=true
[16:40:40 INFO]: [ChunkHolderManager] Saved 897 block chunks, 514 entity chunks, 13 poi chunks in world 'minecraft:overworld' in 1.21s
[16:40:40 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:overworld'
[16:40:40 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:overworld'
[16:40:40 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[16:40:40 INFO]: [ChunkHolderManager] Waiting 60s for chunk system to halt for world 'minecraft:the_nether'
[16:40:40 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:the_nether'
[16:40:40 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_nether'
[16:40:40 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_nether' in 0.00s
[16:40:40 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_nether'
[16:40:40 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_nether'
[16:40:40 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[16:40:40 INFO]: [ChunkHolderManager] Waiting 60s for chunk system to halt for world 'minecraft:the_end'
[16:40:40 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:the_end'
[16:40:40 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_end'
[16:40:40 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_end' in 0.00s
[16:40:40 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_end'
[16:40:40 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_end'
[16:40:40 INFO]: Waiting for all RegionFile I/O tasks to complete...
[16:40:40 INFO]: All RegionFile I/O tasks to complete
[16:40:40 INFO]: [MoonriseCommon] Awaiting termination of worker pool for up to 60s...
[16:40:40 INFO]: [MoonriseCommon] Awaiting termination of I/O pool for up to 60s...
[c-LoD] EB: enter, native_gen=true
[launcher] server exited with code 143
```
### shots
```
total 84
drwxr-xr-x 2 runner runner  4096 Nov 18 16:40 .
drwxr-xr-x 3 runner runner  4096 Nov 18 16:34 ..
-rw-r--r-- 1 runner runner 59114 Nov 18 16:40 server_full.log
-rw-r--r-- 1 runner runner 14361 Nov 18 16:40 summary.md
```

**exit=1** failures=1 leg1=0 leg2=0
