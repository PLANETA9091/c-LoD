- phase: 0 sanity (12:44:57)
### c-LoD visual run (-930,796, marker=red_concrete, vantage -1005,150 yaw 270)
- GL:     Vendor: Mesa (0xffffffff);    Device: llvmpipe (LLVM 21.1.8, 256 bits) (0xffffffff);OpenGL renderer string: llvmpipe (LLVM 21.1.8, 256 bits);OpenGL core profile version string: 4.5 (Core Profile) Mesa 26.0.8-1ubuntu0.3;
- boot + gates OK (12:45:22)
- phase: 1 server booted (12:45:22)
- sky-grid armed
- phase: 2 leg-1 client joined (12:45:47)
- joined session 1 (12:45:47)
- stream live: cells=65536
- phase: 3 scan populated (12:46:10)
- terrain range: 2 terraced mountains at (CX-4,CZ-20)/(CX-2,CZ+16), tops 185/190, grass terraces + stone caps, 66..82 m from the eye (inside the 83.5 m band, chunk edge 64 m)
- scan complete=1 (cells=cells=65536/65536)
- paint settled=1 (wire 12384p)
- **FAIL:** rejoin1: client left the game mid-test (joins=2 lefts=2) — the shot is the main-menu panorama, not the LOD proof

### boot / stream
```
[c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 137297 bytes, sha1 d1b6702bfc2bd1be94558cdf1b697a852d544636 — zero textures, zero world data, one push per join
[12:45:19 INFO]: [c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 137297 bytes, sha1 d1b6702bfc2bd1be94558cdf1b697a852d544636 — zero textures, zero world data, one push per join
[c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[12:45:19 INFO]: [c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[c-LoD] stream: tick 800 65536/65536 cells, lap 404, cone 1089 zone 768 dirty 773 (q 10627) ewin 259 rbeat 8p/1 rearm 80c/2765t rescan 56685c/1839e, edits +5260p/3528c/448e drop 0, spawned 0 / failed 0, cam0 none, wish 327 forced 185, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 29214p/50b fail 0 ~0 kB/s cache 13028/24576 h16186 m13028 e0, srbeat 953s/13q drop 16stale/0undec, ereb 40s/856q, sweep 0r/0d
[12:50:02 INFO]: [c-LoD] stream: tick 800 65536/65536 cells, lap 404, cone 1089 zone 768 dirty 773 (q 10627) ewin 259 rbeat 8p/1 rearm 80c/2765t rescan 56685c/1839e, edits +5260p/3528c/448e drop 0, spawned 0 / failed 0, cam0 none, wish 327 forced 185, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 29214p/50b fail 0 ~0 kB/s cache 13028/24576 h16186 m13028 e0, srbeat 953s/13q drop 16stale/0undec, ereb 40s/856q, sweep 0r/0d
```
### sessions
```
[12:45:46 INFO]: System chat: Stepan joined the game
[12:46:22 INFO]: System chat: Stepan left the game
[12:47:17 INFO]: System chat: Stepan joined the game
[12:47:25 INFO]: System chat: Stepan left the game
```
### client log tail
```
[12:47:24] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:light_update] net.minecraft.network.protocol.game.ClientboundLightUpdatePacket -> 2064 bytes
[12:47:24] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:light_update] net.minecraft.network.protocol.game.ClientboundLightUpdatePacket -> 4115 bytes
[12:47:24] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:block_update] net.minecraft.network.protocol.game.ClientboundBlockUpdatePacket -> 10 bytes
[12:47:25] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:light_update] net.minecraft.network.protocol.game.ClientboundLightUpdatePacket -> 4115 bytes
[12:47:25] [Worker-Main-1/ERROR]: Couldn't compile pipeline (clod:lod_terrain/9): 
com.mojang.renderpearl.util.ShaderCompileException: Couldn't parse GLSL: clod:post/lod_terrain:22: error: 'ColBufSampler' : undeclared identifier
clod:post/lod_terrain:22: error: 'texelFetch' : no matching overloaded function found
clod:post/lod_terrain:22: error: '=' :  cannot convert from ' const float' to ' temp highp 4-component vector of float'

	at com.mojang.renderpearl.frontend.shaders.GlslCompiler.compileToSpv(GlslCompiler.java:140)
	at com.mojang.renderpearl.frontend.shaders.PipelineBuilder.generateBackendCreateInfo(PipelineBuilder.java:76)
	at com.mojang.renderpearl.frontend.shaders.PipelineBuilder.lambda$compilePipeline$0(PipelineBuilder.java:329)
	at java.base/java.util.concurrent.CompletableFuture$AsyncSupply.run(CompletableFuture.java:1789)
	at java.base/java.util.concurrent.CompletableFuture$AsyncSupply.exec(CompletableFuture.java:1781)
	at java.base/java.util.concurrent.ForkJoinTask.doExec(ForkJoinTask.java:511)
	at java.base/java.util.concurrent.ForkJoinPool$WorkQueue.topLevelExec(ForkJoinPool.java:1450)
	at java.base/java.util.concurrent.ForkJoinPool.runWorker(ForkJoinPool.java:2019)
	at java.base/java.util.concurrent.ForkJoinWorkerThread.run(ForkJoinWorkerThread.java:187)
[12:47:25] [Worker-Main-1/ERROR]: Couldn't preload shader clod:shaders/post/lod_terrain.fsh
java.io.FileNotFoundException: clod:shaders/post/lod_terrain.fsh
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
[12:47:25] [Worker-Main-1/ERROR]: Couldn't find source for FRAGMENT shader (clod:post/lod_terrain)
[12:47:25] [Worker-Main-1/ERROR]: Couldn't find source for FRAGMENT shader (clod:post/lod_terrain)
[12:47:25] [Render thread/ERROR]: Failed to load post chain: clod:lod_terrain
net.minecraft.client.renderer.ShaderManager$CompilationException: Failed to compile post processing pipeline clod:lod_terrain/9
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
[12:47:25] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:set_time] net.minecraft.network.protocol.game.ClientboundSetTimePacket -> 10 bytes
[12:47:25] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:light_update] net.minecraft.network.protocol.game.ClientboundLightUpdatePacket -> 2064 bytes
[12:47:25] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:section_blocks_update] net.minecraft.network.protocol.game.ClientboundSectionBlocksUpdatePacket -> 22 bytes
[12:47:25] [Render thread/INFO]: Reloading ResourceManager: vanilla, vanilla
[12:47:25] [Worker-Main-1/INFO]: Found unifont_all_no_pua-17.0.01.hex, loading
[12:47:25] [Worker-Main-2/INFO]: Found unifont_pua-17.0.01.hex, loading
[12:47:25] [Worker-Main-1/INFO]: Found unifont_jp_patch-17.0.01.hex, loading
[12:47:26] [Download-1/INFO]: Could not authorize you against Realms server: java.lang.RuntimeException: Failed to parse into SignedJWT: 0
[12:47:26] [Download-1/ERROR]: Couldn't connect to realms
com.mojang.realmsclient.exception.RealmsServiceException: Realms authentication error with message 'java.lang.RuntimeException: Failed to parse into SignedJWT: 0'
	at com.mojang.realmsclient.client.RealmsClient.execute(RealmsClient.java:625)
	at com.mojang.realmsclient.client.RealmsClient.clientCompatible(RealmsClient.java:354)
	at com.mojang.realmsclient.RealmsAvailability.lambda$check$0(RealmsAvailability.java:48)
	at java.base/java.util.concurrent.CompletableFuture$AsyncSupply.run(CompletableFuture.java:1789)
	at java.base/java.util.concurrent.ThreadPoolExecutor.runWorker(ThreadPoolExecutor.java:1090)
	at java.base/java.util.concurrent.ThreadPoolExecutor$Worker.run(ThreadPoolExecutor.java:614)
	at java.base/java.lang.Thread.run(Thread.java:1474)
[ALSOFT] (EE) Could not query RTKit: No such file or directory (2)
[12:47:27] [Render thread/INFO]: OpenAL initialized on device No Output
[12:47:27] [Render thread/INFO]: Sound engine started
[12:47:27] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[12:47:27] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[12:47:27] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[12:47:27] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[12:47:27] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[12:47:27] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[12:47:27] [Render thread/INFO]: Created: 2048x2048x4 minecraft:textures/atlas/blocks.png-atlas
[12:47:27] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[12:47:27] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[12:47:27] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[12:47:27] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[12:47:27] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
--- exceptions (Realms/RTKit noise filtered) ---
106:	at com.mojang.authlib.minecraft.client.MinecraftClient.buildHttpException(MinecraftClient.java:167)
122:[12:47:12] [Download-2/ERROR]: Failed to fetch user properties
123:com.mojang.authlib.exceptions.InvalidCredentialsException: Status: 401
124:	at com.mojang.authlib.exceptions.MinecraftClientHttpException.toAuthenticationException(MinecraftClientHttpException.java:70)
131:Caused by: MinecraftClientHttpException[type=HTTP_ERROR, status=401, response=ErrorResponse[path=/player/attributes, error=null, errorMessage=null, details=null]]
132:	at com.mojang.authlib.minecraft.client.MinecraftClient.buildHttpException(MinecraftClient.java:170)
17774:[12:47:25] [Worker-Main-1/ERROR]: Couldn't compile pipeline (clod:lod_terrain/9): 
17775:com.mojang.renderpearl.util.ShaderCompileException: Couldn't parse GLSL: clod:post/lod_terrain:22: error: 'ColBufSampler' : undeclared identifier
17788:[12:47:25] [Worker-Main-1/ERROR]: Couldn't preload shader clod:shaders/post/lod_terrain.fsh
17789:java.io.FileNotFoundException: clod:shaders/post/lod_terrain.fsh
17803:[12:47:25] [Worker-Main-1/ERROR]: Couldn't find source for FRAGMENT shader (clod:post/lod_terrain)
17804:[12:47:25] [Worker-Main-1/ERROR]: Couldn't find source for FRAGMENT shader (clod:post/lod_terrain)
17805:[12:47:25] [Render thread/ERROR]: Failed to load post chain: clod:lod_terrain
17806:net.minecraft.client.renderer.ShaderManager$CompilationException: Failed to compile post processing pipeline clod:lod_terrain/9
17826:[12:47:26] [Download-1/ERROR]: Couldn't connect to realms
```
### server log tail
```
[12:50:44 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:overworld'
[c-LoD] EB: enter, native_gen=true
[12:50:45 INFO]: [ChunkHolderManager] Saved 897 block chunks, 529 entity chunks, 12 poi chunks in world 'minecraft:overworld' in 1.16s
[12:50:45 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:overworld'
[12:50:45 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:overworld'
[12:50:45 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[12:50:45 INFO]: [ChunkHolderManager] Waiting 60s for chunk system to halt for world 'minecraft:the_nether'
[12:50:45 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:the_nether'
[12:50:45 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_nether'
[12:50:45 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_nether' in 0.00s
[12:50:45 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_nether'
[12:50:45 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_nether'
[12:50:45 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[12:50:45 INFO]: [ChunkHolderManager] Waiting 60s for chunk system to halt for world 'minecraft:the_end'
[12:50:45 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:the_end'
[12:50:45 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_end'
[12:50:45 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_end' in 0.00s
[12:50:45 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_end'
[12:50:45 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_end'
[12:50:45 INFO]: Waiting for all RegionFile I/O tasks to complete...
[12:50:45 INFO]: All RegionFile I/O tasks to complete
[12:50:45 INFO]: [MoonriseCommon] Awaiting termination of worker pool for up to 60s...
[12:50:45 INFO]: [MoonriseCommon] Awaiting termination of I/O pool for up to 60s...
[c-LoD] EB: enter, native_gen=true
[launcher] server exited with code 143
```
### shots
```
total 84
drwxr-xr-x 2 runner runner  4096 Nov 15 12:50 .
drwxr-xr-x 3 runner runner  4096 Nov 15 12:44 ..
-rw-r--r-- 1 runner runner 58243 Nov 15 12:50 server_full.log
-rw-r--r-- 1 runner runner 14285 Nov 15 12:50 summary.md
```

**exit=1** failures=1 leg1=0 leg2=0
