- phase: 0 sanity (08:01:20)
### c-LoD visual run (-930,796, marker=red_concrete, vantage -1005,150 yaw 270)
- GL:     Vendor: Mesa (0xffffffff);    Device: llvmpipe (LLVM 21.1.8, 256 bits) (0xffffffff);OpenGL renderer string: llvmpipe (LLVM 21.1.8, 256 bits);OpenGL core profile version string: 4.5 (Core Profile) Mesa 26.0.8-1ubuntu0.3;
- boot + gates OK (08:01:42)
- phase: 1 server booted (08:01:42)
- sky-grid armed
- phase: 2 leg-1 client joined (08:02:03)
- joined session 1 (08:02:03)
- stream live: cells=65536
- phase: 3 scan populated (08:02:26)
- terrain range: 2 terraced mountains at (CX-4,CZ-20)/(CX-2,CZ+16), tops 185/190, grass terraces + stone caps, 66..82 m from the eye (inside the 83.5 m band, chunk edge 64 m)
- scan complete=1 (cells=cells=65536/65536)
- paint settled=1 (wire 0p)
- **FAIL:** rejoin1: client left the game mid-test (joins=2 lefts=2) — the shot is the main-menu panorama, not the LOD proof

### boot / stream
```
[c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 155403 bytes, sha1 d72eae3f543eb13ed99fa51a683ce93a66e6338f — zero textures, zero world data, one push per join
[08:01:39 INFO]: [c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 155403 bytes, sha1 d72eae3f543eb13ed99fa51a683ce93a66e6338f — zero textures, zero world data, one push per join
[c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[08:01:39 INFO]: [c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[c-LoD] stream: tick 700 65536/65536 cells, lap 392, cone 849 zone 1019 dirty 544 (q 11382) ewin 0 rbeat 1p/0 rearm 80c/2804t rescan 45523c/1720e, edits +5309p/3757c/397e drop 0, spawned 0 / failed 0, cas 0d, cam0 none, wish 314 forced 198, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 28109p/39b fail 0 ~0 kB/s cache 13072/24576 h15037 m13072 e0, srbeat 1058s/352q drop 21stale/0undec, ereb 32s/632q, sweep 0r/0d, cdiag off
[08:05:22 INFO]: [c-LoD] stream: tick 700 65536/65536 cells, lap 392, cone 849 zone 1019 dirty 544 (q 11382) ewin 0 rbeat 1p/0 rearm 80c/2804t rescan 45523c/1720e, edits +5309p/3757c/397e drop 0, spawned 0 / failed 0, cas 0d, cam0 none, wish 314 forced 198, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 28109p/39b fail 0 ~0 kB/s cache 13072/24576 h15037 m13072 e0, srbeat 1058s/352q drop 21stale/0undec, ereb 32s/632q, sweep 0r/0d, cdiag off
```
### sessions
```
[08:02:02 INFO]: System chat: Stepan joined the game
[08:02:37 INFO]: System chat: Stepan left the game
[08:03:12 INFO]: System chat: Stepan joined the game
[08:03:20 INFO]: System chat: Stepan left the game
```
### client log tail
```
[08:03:20] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[08:03:20] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[08:03:20] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[08:03:20] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[08:03:20] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[08:03:20] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[08:03:20] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[08:03:20] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[08:03:20] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[08:03:20] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[08:03:20] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[08:03:20] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[08:03:20] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[08:03:20] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[08:03:20] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[08:03:20] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[08:03:20] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[08:03:20] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[08:03:20] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[08:03:20] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[08:03:20] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[08:03:20] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[08:03:20] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[08:03:20] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[08:03:20] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[08:03:20] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[08:03:20] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[08:03:20] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[08:03:20] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[08:03:20] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[08:03:20] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[08:03:20] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[08:03:20] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[08:03:20] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[08:03:20] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[08:03:20] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[08:03:20] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[08:03:20] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[08:03:20] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[08:03:20] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[08:03:20] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:bundle_delimiter] net.minecraft.network.protocol.game.ClientboundBundleDelimiterPacket -> 1 bytes
[08:03:20] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:light_update] net.minecraft.network.protocol.game.ClientboundLightUpdatePacket -> 2064 bytes
[08:03:20] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:section_blocks_update] net.minecraft.network.protocol.game.ClientboundSectionBlocksUpdatePacket -> 16 bytes
[08:03:20] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:block_update] net.minecraft.network.protocol.game.ClientboundBlockUpdatePacket -> 10 bytes
[08:03:20] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:set_entity_motion] net.minecraft.network.protocol.game.ClientboundSetEntityMotionPacket -> 8 bytes
[08:03:20] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:entity_position_sync] net.minecraft.network.protocol.game.ClientboundEntityPositionSyncPacket -> 38 bytes
[08:03:20] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:rotate_head] net.minecraft.network.protocol.game.ClientboundRotateHeadPacket -> 3 bytes
[08:03:20] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:light_update] net.minecraft.network.protocol.game.ClientboundLightUpdatePacket -> 2064 bytes
[08:03:20] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:block_update] net.minecraft.network.protocol.game.ClientboundBlockUpdatePacket -> 10 bytes
[08:03:20] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:section_blocks_update] net.minecraft.network.protocol.game.ClientboundSectionBlocksUpdatePacket -> 16 bytes
[08:03:20] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:light_update] net.minecraft.network.protocol.game.ClientboundLightUpdatePacket -> 4115 bytes
[08:03:20] [Render thread/INFO]: Reloading ResourceManager: vanilla, vanilla
[08:03:20] [Worker-Main-3/INFO]: Found unifont_all_no_pua-17.0.01.hex, loading
[08:03:20] [Worker-Main-1/INFO]: Found unifont_pua-17.0.01.hex, loading
[08:03:20] [Worker-Main-3/INFO]: Found unifont_jp_patch-17.0.01.hex, loading
[08:03:20] [Download-2/INFO]: Could not authorize you against Realms server: java.lang.RuntimeException: Failed to parse into SignedJWT: 0
[08:03:20] [Download-2/ERROR]: Couldn't connect to realms
com.mojang.realmsclient.exception.RealmsServiceException: Realms authentication error with message 'java.lang.RuntimeException: Failed to parse into SignedJWT: 0'
	at com.mojang.realmsclient.client.RealmsClient.execute(RealmsClient.java:625)
	at com.mojang.realmsclient.client.RealmsClient.clientCompatible(RealmsClient.java:354)
	at com.mojang.realmsclient.RealmsAvailability.lambda$check$0(RealmsAvailability.java:48)
	at java.base/java.util.concurrent.CompletableFuture$AsyncSupply.run(CompletableFuture.java:1789)
	at java.base/java.util.concurrent.ThreadPoolExecutor.runWorker(ThreadPoolExecutor.java:1090)
	at java.base/java.util.concurrent.ThreadPoolExecutor$Worker.run(ThreadPoolExecutor.java:614)
	at java.base/java.lang.Thread.run(Thread.java:1474)
[ALSOFT] (EE) Could not query RTKit: No such file or directory (2)
[08:03:21] [Render thread/INFO]: OpenAL initialized on device No Output
[08:03:21] [Render thread/INFO]: Sound engine started
[08:03:21] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[08:03:21] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[08:03:21] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[08:03:21] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[08:03:21] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[08:03:21] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[08:03:21] [Render thread/INFO]: Created: 2048x2048x4 minecraft:textures/atlas/blocks.png-atlas
[08:03:21] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[08:03:21] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[08:03:21] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[08:03:21] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[08:03:21] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
--- exceptions (Realms/RTKit noise filtered) ---
25:[08:03:06] [Render thread/ERROR]: Error parsing option value 4 for option Simulation Distance: Value 4 outside of range [5:17]
49:[08:03:07] [Download-2/ERROR]: Failed to fetch user properties
50:com.mojang.authlib.exceptions.InvalidCredentialsException: Status: 401
51:	at com.mojang.authlib.exceptions.MinecraftClientHttpException.toAuthenticationException(MinecraftClientHttpException.java:70)
58:Caused by: MinecraftClientHttpException[type=HTTP_ERROR, status=401, response=ErrorResponse[path=/player/attributes, error=null, errorMessage=null, details=null]]
59:	at com.mojang.authlib.minecraft.client.MinecraftClient.buildHttpException(MinecraftClient.java:170)
21792:[08:03:19] [Worker-Main-1/ERROR]: Couldn't compile pipeline (clod:lod_terrain/3): 
21793:com.mojang.renderpearl.util.ShaderCompileException: Couldn't parse GLSL: clod:post/capture:565: error: 'eraseFlag' : undeclared identifier
22100:[08:03:19] [Worker-Main-1/ERROR]: Couldn't preload shader clod:shaders/post/capture.fsh
22101:java.io.FileNotFoundException: clod:shaders/post/capture.fsh
22116:[08:03:19] [Worker-Main-1/ERROR]: Couldn't find source for FRAGMENT shader (clod:post/capture)
22117:[08:03:19] [Worker-Main-1/ERROR]: Couldn't find source for FRAGMENT shader (clod:post/capture)
22118:[08:03:19] [Render thread/ERROR]: Failed to load post chain: clod:lod_terrain
22119:net.minecraft.client.renderer.ShaderManager$CompilationException: Failed to compile post processing pipeline clod:lod_terrain/3
22868:[08:03:20] [Download-2/ERROR]: Couldn't connect to realms
```
### server log tail
```
[08:06:38 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:overworld'
[08:06:38 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:overworld'
[c-LoD] EB: enter, native_gen=true
[08:06:39 INFO]: [ChunkHolderManager] Saved 841 block chunks, 523 entity chunks, 12 poi chunks in world 'minecraft:overworld' in 0.96s
[08:06:39 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:overworld'
[08:06:39 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:overworld'
[08:06:39 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[08:06:39 INFO]: [ChunkHolderManager] Waiting 60s for chunk system to halt for world 'minecraft:the_nether'
[08:06:39 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:the_nether'
[08:06:39 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_nether'
[08:06:39 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_nether' in 0.00s
[08:06:39 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_nether'
[08:06:39 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_nether'
[08:06:39 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[08:06:39 INFO]: [ChunkHolderManager] Waiting 60s for chunk system to halt for world 'minecraft:the_end'
[08:06:39 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:the_end'
[08:06:39 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_end'
[08:06:39 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_end' in 0.00s
[08:06:39 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_end'
[08:06:39 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_end'
[08:06:39 INFO]: Waiting for all RegionFile I/O tasks to complete...
[08:06:39 INFO]: All RegionFile I/O tasks to complete
[08:06:39 INFO]: [MoonriseCommon] Awaiting termination of worker pool for up to 60s...
[08:06:39 INFO]: [MoonriseCommon] Awaiting termination of I/O pool for up to 60s...
[launcher] server exited with code 143
```
### shots
```
total 84
drwxr-xr-x 2 runner runner  4096 Nov 21 08:06 .
drwxr-xr-x 3 runner runner  4096 Nov 21 08:00 ..
-rw-r--r-- 1 runner runner 55304 Nov 21 08:06 server_full.log
-rw-r--r-- 1 runner runner 17858 Nov 21 08:06 summary.md
```

**exit=1** failures=1 leg1=0 leg2=0
