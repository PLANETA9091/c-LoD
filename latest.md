- phase: 0 sanity (11:04:22)
### c-LoD visual run (-930,796, marker=red_concrete, vantage -1005,150 yaw 270)
- GL:     Vendor: Mesa (0xffffffff);    Device: llvmpipe (LLVM 21.1.8, 256 bits) (0xffffffff);OpenGL renderer string: llvmpipe (LLVM 21.1.8, 256 bits);OpenGL core profile version string: 4.5 (Core Profile) Mesa 26.0.8-1ubuntu0.3;
- boot + gates OK (11:04:57)
- phase: 1 server booted (11:04:57)
- sky-grid armed
- phase: 2 leg-1 client joined (11:05:24)
- joined session 1 (11:05:24)
- stream live: cells=65536
- phase: 3 scan populated (11:05:48)
- terrain range: 2 terraced mountains at (CX-4,CZ-20)/(CX-2,CZ+16), tops 185/190, grass terraces + stone caps, 66..82 m from the eye (inside the 83.5 m band, chunk edge 64 m)
- scan complete=1 (cells=cells=65536/65536)
- paint settled=1 (wire 0p)
- **FAIL:** rejoin1: client left the game mid-test (joins=2 lefts=2) — the shot is the main-menu panorama, not the LOD proof

### boot / stream
```
[c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 155702 bytes, sha1 9d720ddec9160eff388c38a83dbe582bb781289a — zero textures, zero world data, one push per join
[c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[11:04:54 INFO]: [c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 155702 bytes, sha1 9d720ddec9160eff388c38a83dbe582bb781289a — zero textures, zero world data, one push per join
[11:04:54 INFO]: [c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[c-LoD] stream: tick 800 65536/65536 cells, lap 417, cone 779 zone 769 dirty 774 (q 10695) ewin 257 rbeat 22p/1 rearm 80c/2768t rescan 58620c/2834e, edits +4988p/3218c/402e drop 0, spawned 0 / failed 0, cas 0d, cam0 none, wish 335 forced 177, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 27231p/37b fail 0 ~0 kB/s cache 13342/24576 h13889 m13342 e0, srbeat 1005s/89q drop 17stale/0undec, ereb 80s/846q, sweep 0r/0d, cdiag off
[11:09:16 INFO]: [c-LoD] stream: tick 800 65536/65536 cells, lap 417, cone 779 zone 769 dirty 774 (q 10695) ewin 257 rbeat 22p/1 rearm 80c/2768t rescan 58620c/2834e, edits +4988p/3218c/402e drop 0, spawned 0 / failed 0, cas 0d, cam0 none, wish 335 forced 177, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 27231p/37b fail 0 ~0 kB/s cache 13342/24576 h13889 m13342 e0, srbeat 1005s/89q drop 17stale/0undec, ereb 80s/846q, sweep 0r/0d, cdiag off
```
### sessions
```
[11:05:22 INFO]: System chat: Stepan joined the game
[11:06:00 INFO]: System chat: Stepan left the game
[11:06:36 INFO]: System chat: Stepan joined the game
[11:06:44 INFO]: System chat: Stepan left the game
```
### client log tail
```
[11:06:44] [Render thread/WARN]: Requested post effect does not exist: minecraft:end_of_frame
[11:06:44] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:block_update] net.minecraft.network.protocol.game.ClientboundBlockUpdatePacket -> 10 bytes
[11:06:44] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:set_entity_motion] net.minecraft.network.protocol.game.ClientboundSetEntityMotionPacket -> 9 bytes
[11:06:44] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos_rot] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$PosRot -> 13 bytes
[11:06:44] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 4 bytes
[11:06:44] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:light_update] net.minecraft.network.protocol.game.ClientboundLightUpdatePacket -> 4115 bytes
[11:06:44] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:set_entity_motion] net.minecraft.network.protocol.game.ClientboundSetEntityMotionPacket -> 9 bytes
[11:06:44] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos_rot] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$PosRot -> 13 bytes
[11:06:44] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 4 bytes
[11:06:44] [Worker-Main-2/ERROR]: Couldn't compile pipeline (clod:lod_terrain/18): 
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
[11:06:44] [Worker-Main-2/ERROR]: Couldn't preload shader clod:shaders/post/debug_lens.fsh
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
[11:06:44] [Worker-Main-2/ERROR]: Couldn't find source for FRAGMENT shader (clod:post/debug_lens)
[11:06:44] [Worker-Main-2/ERROR]: Couldn't find source for FRAGMENT shader (clod:post/debug_lens)
[11:06:44] [Render thread/ERROR]: Failed to load post chain: clod:lod_terrain
net.minecraft.client.renderer.ShaderManager$CompilationException: Failed to compile post processing pipeline clod:lod_terrain/18
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
[11:06:44] [Render thread/INFO]: Reloading ResourceManager: vanilla, vanilla
[11:06:44] [Worker-Main-2/INFO]: Found unifont_all_no_pua-17.0.01.hex, loading
[11:06:44] [Worker-Main-1/INFO]: Found unifont_pua-17.0.01.hex, loading
[11:06:45] [Worker-Main-2/INFO]: Found unifont_jp_patch-17.0.01.hex, loading
[11:06:45] [Download-1/INFO]: Could not authorize you against Realms server: java.lang.RuntimeException: Failed to parse into SignedJWT: 0
[11:06:45] [Download-1/ERROR]: Couldn't connect to realms
com.mojang.realmsclient.exception.RealmsServiceException: Realms authentication error with message 'java.lang.RuntimeException: Failed to parse into SignedJWT: 0'
	at com.mojang.realmsclient.client.RealmsClient.execute(RealmsClient.java:625)
	at com.mojang.realmsclient.client.RealmsClient.clientCompatible(RealmsClient.java:354)
	at com.mojang.realmsclient.RealmsAvailability.lambda$check$0(RealmsAvailability.java:48)
	at java.base/java.util.concurrent.CompletableFuture$AsyncSupply.run(CompletableFuture.java:1789)
	at java.base/java.util.concurrent.ThreadPoolExecutor.runWorker(ThreadPoolExecutor.java:1090)
	at java.base/java.util.concurrent.ThreadPoolExecutor$Worker.run(ThreadPoolExecutor.java:614)
	at java.base/java.lang.Thread.run(Thread.java:1474)
[ALSOFT] (EE) Could not query RTKit: No such file or directory (2)
[11:06:46] [Render thread/INFO]: OpenAL initialized on device No Output
[11:06:46] [Render thread/INFO]: Sound engine started
[11:06:46] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[11:06:46] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[11:06:46] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[11:06:46] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[11:06:46] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[11:06:46] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[11:06:46] [Render thread/INFO]: Created: 2048x2048x4 minecraft:textures/atlas/blocks.png-atlas
[11:06:46] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[11:06:46] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[11:06:46] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[11:06:46] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[11:06:46] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
--- exceptions (Realms/RTKit noise filtered) ---
25:[11:06:30] [Render thread/ERROR]: Error parsing option value 4 for option Simulation Distance: Value 4 outside of range [5:17]
49:[11:06:30] [Download-2/ERROR]: Failed to fetch user properties
50:com.mojang.authlib.exceptions.InvalidCredentialsException: Status: 401
51:	at com.mojang.authlib.exceptions.MinecraftClientHttpException.toAuthenticationException(MinecraftClientHttpException.java:70)
58:Caused by: MinecraftClientHttpException[type=HTTP_ERROR, status=401, response=ErrorResponse[path=/player/attributes, error=null, errorMessage=null, details=null]]
59:	at com.mojang.authlib.minecraft.client.MinecraftClient.buildHttpException(MinecraftClient.java:170)
14923:[11:06:44] [Worker-Main-2/ERROR]: Couldn't compile pipeline (clod:lod_terrain/18): 
14924:com.mojang.renderpearl.util.ShaderCompileException: Couldn't parse GLSL: clod:post/debug_lens:62: error: '' :  syntax error, unexpected IDENTIFIER
14935:[11:06:44] [Worker-Main-2/ERROR]: Couldn't preload shader clod:shaders/post/debug_lens.fsh
14936:java.io.FileNotFoundException: clod:shaders/post/debug_lens.fsh
14950:[11:06:44] [Worker-Main-2/ERROR]: Couldn't find source for FRAGMENT shader (clod:post/debug_lens)
14951:[11:06:44] [Worker-Main-2/ERROR]: Couldn't find source for FRAGMENT shader (clod:post/debug_lens)
14952:[11:06:44] [Render thread/ERROR]: Failed to load post chain: clod:lod_terrain
14953:net.minecraft.client.renderer.ShaderManager$CompilationException: Failed to compile post processing pipeline clod:lod_terrain/18
14970:[11:06:45] [Download-1/ERROR]: Couldn't connect to realms
```
### server log tail
```
[11:10:02 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:overworld'
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[11:10:04 INFO]: [ChunkHolderManager] Saved 897 block chunks, 546 entity chunks, 13 poi chunks in world 'minecraft:overworld' in 1.32s
[11:10:04 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:overworld'
[11:10:04 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:overworld'
[11:10:04 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[11:10:04 INFO]: [ChunkHolderManager] Waiting 60s for chunk system to halt for world 'minecraft:the_nether'
[11:10:04 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:the_nether'
[11:10:04 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_nether'
[11:10:04 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_nether' in 0.00s
[11:10:04 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_nether'
[11:10:04 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_nether'
[11:10:04 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[11:10:04 INFO]: [ChunkHolderManager] Waiting 60s for chunk system to halt for world 'minecraft:the_end'
[11:10:04 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:the_end'
[11:10:04 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_end'
[11:10:04 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_end' in 0.00s
[11:10:04 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_end'
[11:10:04 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_end'
[11:10:04 INFO]: Waiting for all RegionFile I/O tasks to complete...
[11:10:04 INFO]: All RegionFile I/O tasks to complete
[11:10:04 INFO]: [MoonriseCommon] Awaiting termination of worker pool for up to 60s...
[11:10:04 INFO]: [MoonriseCommon] Awaiting termination of I/O pool for up to 60s...
[launcher] server exited with code 143
```
### shots
```
total 84
drwxr-xr-x 2 runner runner  4096 Nov 21 11:10 .
drwxr-xr-x 3 runner runner  4096 Nov 21 11:03 ..
-rw-r--r-- 1 runner runner 58609 Nov 21 11:10 server_full.log
-rw-r--r-- 1 runner runner 14410 Nov 21 11:10 summary.md
```

**exit=1** failures=1 leg1=0 leg2=0
