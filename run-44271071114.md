- phase: 0 sanity (08:56:18)
### c-LoD visual run (-930,796, marker=red_concrete, vantage -1005,150 yaw 270)
- GL:     Vendor: Mesa (0xffffffff);    Device: llvmpipe (LLVM 21.1.8, 256 bits) (0xffffffff);OpenGL renderer string: llvmpipe (LLVM 21.1.8, 256 bits);OpenGL core profile version string: 4.5 (Core Profile) Mesa 26.0.8-1ubuntu0.3;
- boot + gates OK (08:56:40)
- phase: 1 server booted (08:56:40)
- sky-grid armed
- phase: 2 leg-1 client joined (08:57:03)
- joined session 1 (08:57:03)
- stream live: cells=65536
- phase: 3 scan populated (08:57:25)
- terrain range: 2 terraced mountains at (CX-4,CZ-20)/(CX-2,CZ+16), tops 185/190, grass terraces + stone caps, 66..82 m from the eye (inside the 83.5 m band, chunk edge 64 m)
- scan complete=1 (cells=cells=65536/65536)
- paint settled=1 (wire 12797p)
- **FAIL:** rejoin1: client left the game mid-test (joins=2 lefts=2) — the shot is the main-menu panorama, not the LOD proof

### boot / stream
```
[c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 147077 bytes, sha1 853cf451c7edc996df4bd1db31e7333382182477 — zero textures, zero world data, one push per join
[c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[08:56:38 INFO]: [c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 147077 bytes, sha1 853cf451c7edc996df4bd1db31e7333382182477 — zero textures, zero world data, one push per join
[08:56:38 INFO]: [c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[c-LoD] stream: tick 800 65536/65536 cells, lap 396, cone 655 zone 809 dirty 749 (q 11493) ewin 199 rbeat 29p/1 rearm 80c/2744t rescan 55588c/2194e, edits +5356p/3673c/445e drop 0, spawned 0 / failed 0, cam0 none, wish 314 forced 198, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 26848p/41b fail 0 ~0 kB/s cache 13270/24576 h13578 m13270 e0, srbeat 1071s/125q drop 23stale/0undec, ereb 37s/829q, sweep 0r/0d
[09:01:42 INFO]: [c-LoD] stream: tick 800 65536/65536 cells, lap 396, cone 655 zone 809 dirty 749 (q 11493) ewin 199 rbeat 29p/1 rearm 80c/2744t rescan 55588c/2194e, edits +5356p/3673c/445e drop 0, spawned 0 / failed 0, cam0 none, wish 314 forced 198, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 26848p/41b fail 0 ~0 kB/s cache 13270/24576 h13578 m13270 e0, srbeat 1071s/125q drop 23stale/0undec, ereb 37s/829q, sweep 0r/0d
```
### sessions
```
[08:57:01 INFO]: System chat: Stepan joined the game
[08:57:37 INFO]: System chat: Stepan left the game
[08:58:31 INFO]: System chat: Stepan joined the game
[08:58:39 INFO]: System chat: Stepan left the game
```
### client log tail
```
[08:58:39] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:light_update] net.minecraft.network.protocol.game.ClientboundLightUpdatePacket -> 2064 bytes
[08:58:39] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:light_update] net.minecraft.network.protocol.game.ClientboundLightUpdatePacket -> 4115 bytes
[08:58:39] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:light_update] net.minecraft.network.protocol.game.ClientboundLightUpdatePacket -> 2064 bytes
[08:58:39] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:block_update] net.minecraft.network.protocol.game.ClientboundBlockUpdatePacket -> 10 bytes
[08:58:39] [Worker-Main-2/ERROR]: Couldn't compile pipeline (clod:lod_terrain/10): 
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
[08:58:39] [Worker-Main-2/ERROR]: Couldn't preload shader clod:shaders/post/debug_lens.fsh
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
[08:58:39] [Worker-Main-2/ERROR]: Couldn't find source for FRAGMENT shader (clod:post/debug_lens)
[08:58:39] [Worker-Main-2/ERROR]: Couldn't find source for FRAGMENT shader (clod:post/debug_lens)
[08:58:39] [Render thread/ERROR]: Failed to load post chain: clod:lod_terrain
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
[08:58:39] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:set_time] net.minecraft.network.protocol.game.ClientboundSetTimePacket -> 10 bytes
[08:58:39] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:light_update] net.minecraft.network.protocol.game.ClientboundLightUpdatePacket -> 2064 bytes
[08:58:39] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:set_entity_motion] net.minecraft.network.protocol.game.ClientboundSetEntityMotionPacket -> 8 bytes
[08:58:39] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos_rot] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$PosRot -> 12 bytes
[08:58:39] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[08:58:39] [Render thread/INFO]: Reloading ResourceManager: vanilla, vanilla
[08:58:39] [Worker-Main-3/INFO]: Found unifont_all_no_pua-17.0.01.hex, loading
[08:58:39] [Worker-Main-1/INFO]: Found unifont_pua-17.0.01.hex, loading
[08:58:39] [Worker-Main-3/INFO]: Found unifont_jp_patch-17.0.01.hex, loading
[08:58:40] [Download-2/INFO]: Could not authorize you against Realms server: java.lang.RuntimeException: Failed to parse into SignedJWT: 0
[08:58:40] [Download-2/ERROR]: Couldn't connect to realms
com.mojang.realmsclient.exception.RealmsServiceException: Realms authentication error with message 'java.lang.RuntimeException: Failed to parse into SignedJWT: 0'
	at com.mojang.realmsclient.client.RealmsClient.execute(RealmsClient.java:625)
	at com.mojang.realmsclient.client.RealmsClient.clientCompatible(RealmsClient.java:354)
	at com.mojang.realmsclient.RealmsAvailability.lambda$check$0(RealmsAvailability.java:48)
	at java.base/java.util.concurrent.CompletableFuture$AsyncSupply.run(CompletableFuture.java:1789)
	at java.base/java.util.concurrent.ThreadPoolExecutor.runWorker(ThreadPoolExecutor.java:1090)
	at java.base/java.util.concurrent.ThreadPoolExecutor$Worker.run(ThreadPoolExecutor.java:614)
	at java.base/java.lang.Thread.run(Thread.java:1474)
[ALSOFT] (EE) Could not query RTKit: No such file or directory (2)
[08:58:40] [Render thread/INFO]: OpenAL initialized on device No Output
[08:58:40] [Render thread/INFO]: Sound engine started
[08:58:40] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[08:58:40] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[08:58:40] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[08:58:40] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[08:58:40] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[08:58:40] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[08:58:40] [Render thread/INFO]: Created: 2048x2048x4 minecraft:textures/atlas/blocks.png-atlas
[08:58:40] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[08:58:40] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[08:58:40] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[08:58:40] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[08:58:40] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
--- exceptions (Realms/RTKit noise filtered) ---
25:[08:58:26] [Render thread/ERROR]: Error parsing option value 4 for option Simulation Distance: Value 4 outside of range [5:33]
47:[08:58:26] [Download-2/ERROR]: Failed to fetch user properties
48:com.mojang.authlib.exceptions.InvalidCredentialsException: Status: 401
49:	at com.mojang.authlib.exceptions.MinecraftClientHttpException.toAuthenticationException(MinecraftClientHttpException.java:70)
56:Caused by: MinecraftClientHttpException[type=HTTP_ERROR, status=401, response=ErrorResponse[path=/player/attributes, error=null, errorMessage=null, details=null]]
57:	at com.mojang.authlib.minecraft.client.MinecraftClient.buildHttpException(MinecraftClient.java:170)
14882:[08:58:39] [Worker-Main-2/ERROR]: Couldn't compile pipeline (clod:lod_terrain/10): 
14883:com.mojang.renderpearl.util.ShaderCompileException: Couldn't parse GLSL: clod:post/debug_lens:62: error: '' :  syntax error, unexpected IDENTIFIER
14894:[08:58:39] [Worker-Main-2/ERROR]: Couldn't preload shader clod:shaders/post/debug_lens.fsh
14895:java.io.FileNotFoundException: clod:shaders/post/debug_lens.fsh
14909:[08:58:39] [Worker-Main-2/ERROR]: Couldn't find source for FRAGMENT shader (clod:post/debug_lens)
14910:[08:58:39] [Worker-Main-2/ERROR]: Couldn't find source for FRAGMENT shader (clod:post/debug_lens)
14911:[08:58:39] [Render thread/ERROR]: Failed to load post chain: clod:lod_terrain
14912:net.minecraft.client.renderer.ShaderManager$CompilationException: Failed to compile post processing pipeline clod:lod_terrain/10
14934:[08:58:40] [Download-2/ERROR]: Couldn't connect to realms
```
### server log tail
```
[09:01:59 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:overworld'
[09:01:59 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:overworld'
[c-LoD] EB: enter, native_gen=true
[09:02:00 INFO]: [ChunkHolderManager] Saved 841 block chunks, 528 entity chunks, 12 poi chunks in world 'minecraft:overworld' in 0.90s
[09:02:00 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:overworld'
[09:02:00 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:overworld'
[09:02:00 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[09:02:00 INFO]: [ChunkHolderManager] Waiting 60s for chunk system to halt for world 'minecraft:the_nether'
[09:02:00 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:the_nether'
[09:02:00 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_nether'
[09:02:00 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_nether' in 0.00s
[09:02:00 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_nether'
[09:02:00 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_nether'
[09:02:00 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[09:02:00 INFO]: [ChunkHolderManager] Waiting 60s for chunk system to halt for world 'minecraft:the_end'
[09:02:00 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:the_end'
[09:02:00 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_end'
[09:02:00 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_end' in 0.00s
[09:02:00 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_end'
[09:02:00 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_end'
[09:02:00 INFO]: Waiting for all RegionFile I/O tasks to complete...
[09:02:00 INFO]: All RegionFile I/O tasks to complete
[09:02:00 INFO]: [MoonriseCommon] Awaiting termination of worker pool for up to 60s...
[09:02:00 INFO]: [MoonriseCommon] Awaiting termination of I/O pool for up to 60s...
[launcher] server exited with code 143
```
### shots
```
total 84
drwxr-xr-x 2 runner runner  4096 Nov 18 09:02 .
drwxr-xr-x 3 runner runner  4096 Nov 18 08:55 ..
-rw-r--r-- 1 runner runner 57458 Nov 18 09:02 server_full.log
-rw-r--r-- 1 runner runner 14481 Nov 18 09:02 summary.md
```

**exit=1** failures=1 leg1=0 leg2=0
