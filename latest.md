- phase: 0 sanity (22:12:49)
### c-LoD visual run (-930,796, marker=red_concrete, vantage -1005,150 yaw 270)
- GL:     Vendor: Mesa (0xffffffff);    Device: llvmpipe (LLVM 21.1.8, 256 bits) (0xffffffff);OpenGL renderer string: llvmpipe (LLVM 21.1.8, 256 bits);OpenGL core profile version string: 4.5 (Core Profile) Mesa 26.0.8-1ubuntu0.3;
- boot + gates OK (22:13:12)
- phase: 1 server booted (22:13:12)
- sky-grid armed
- phase: 2 leg-1 client joined (22:13:32)
- joined session 1 (22:13:32)
- stream live: cells=65536
- phase: 3 scan populated (22:13:54)
- terrain range: 2 terraced mountains at (CX-4,CZ-20)/(CX-2,CZ+16), tops 185/190, grass terraces + stone caps, 66..82 m from the eye (inside the 83.5 m band, chunk edge 64 m)
- scan complete=1 (cells=cells=65536/65536)
- paint settled=1 (wire 0p)
- **FAIL:** rejoin1: client left the game mid-test (joins=2 lefts=2) — the shot is the main-menu panorama, not the LOD proof

### boot / stream
```
[c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 152751 bytes, sha1 01f2404353e66e75a3964df5e650df0db1048f6e — zero textures, zero world data, one push per join
[22:13:09 INFO]: [c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 152751 bytes, sha1 01f2404353e66e75a3964df5e650df0db1048f6e — zero textures, zero world data, one push per join
[c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[22:13:09 INFO]: [c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[c-LoD] stream: tick 700 65536/65536 cells, lap 386, cone 803 zone 808 dirty 756 (q 11638) ewin 200 rbeat 33p/1 rearm 80c/2796t rescan 49312c/2646e, edits +5651p/4039c/447e drop 0, spawned 0 / failed 0, cas 0d, cam0 none, wish 317 forced 195, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 27082p/38b fail 0 ~0 kB/s cache 13316/24576 h13766 m13316 e0, srbeat 943s/132q drop 19stale/0undec, ereb 36s/809q, sweep 0r/0d, cdiag off
[22:16:57 INFO]: [c-LoD] stream: tick 700 65536/65536 cells, lap 386, cone 803 zone 808 dirty 756 (q 11638) ewin 200 rbeat 33p/1 rearm 80c/2796t rescan 49312c/2646e, edits +5651p/4039c/447e drop 0, spawned 0 / failed 0, cas 0d, cam0 none, wish 317 forced 195, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 27082p/38b fail 0 ~0 kB/s cache 13316/24576 h13766 m13316 e0, srbeat 943s/132q drop 19stale/0undec, ereb 36s/809q, sweep 0r/0d, cdiag off
```
### sessions
```
[22:13:31 INFO]: System chat: Stepan joined the game
[22:14:06 INFO]: System chat: Stepan left the game
[22:14:38 INFO]: System chat: Stepan joined the game
[22:14:45 INFO]: System chat: Stepan left the game
```
### client log tail
```
[22:14:45] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[22:14:45] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[22:14:45] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[22:14:45] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[22:14:45] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[22:14:45] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[22:14:45] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[22:14:45] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[22:14:45] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[22:14:45] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[22:14:45] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[22:14:45] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[22:14:45] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[22:14:45] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[22:14:45] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[22:14:45] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[22:14:45] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[22:14:45] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[22:14:45] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[22:14:45] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[22:14:45] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[22:14:45] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[22:14:45] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[22:14:45] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[22:14:45] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[22:14:45] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[22:14:45] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[22:14:45] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[22:14:45] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[22:14:45] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[22:14:45] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[22:14:45] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[22:14:45] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[22:14:45] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[22:14:45] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[22:14:45] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[22:14:45] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[22:14:45] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_particles] net.minecraft.network.protocol.game.ClientboundLevelParticlesPacket -> 54 bytes
[22:14:45] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:bundle_delimiter] net.minecraft.network.protocol.game.ClientboundBundleDelimiterPacket -> 1 bytes
[22:14:45] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:section_blocks_update] net.minecraft.network.protocol.game.ClientboundSectionBlocksUpdatePacket -> 16 bytes
[22:14:45] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos -> 11 bytes
[22:14:45] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:block_update] net.minecraft.network.protocol.game.ClientboundBlockUpdatePacket -> 10 bytes
[22:14:45] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:section_blocks_update] net.minecraft.network.protocol.game.ClientboundSectionBlocksUpdatePacket -> 22 bytes
[22:14:45] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:light_update] net.minecraft.network.protocol.game.ClientboundLightUpdatePacket -> 4115 bytes
[22:14:45] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:set_entity_motion] net.minecraft.network.protocol.game.ClientboundSetEntityMotionPacket -> 8 bytes
[22:14:45] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:move_entity_pos_rot] net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$PosRot -> 12 bytes
[22:14:45] [Render thread/INFO]: Reloading ResourceManager: vanilla, vanilla
[22:14:45] [Worker-Main-2/INFO]: Found unifont_all_no_pua-17.0.01.hex, loading
[22:14:45] [Worker-Main-3/INFO]: Found unifont_pua-17.0.01.hex, loading
[22:14:45] [Worker-Main-2/INFO]: Found unifont_jp_patch-17.0.01.hex, loading
[22:14:46] [Render thread/DEBUG]: Ignoring packet due to disconnection: net.minecraft.network.protocol.game.ClientboundBlockUpdatePacket@83159b1
[22:14:46] [Render thread/DEBUG]: Ignoring packet due to disconnection: net.minecraft.network.protocol.game.ClientboundSectionBlocksUpdatePacket@21da6532
[22:14:46] [Render thread/DEBUG]: Ignoring packet due to disconnection: ClientboundLightUpdatePacket[x=-59, z=53, lightData=ClientboundLightUpdatePacketData[skyYMask={4}, blockYMask={4}, emptySkyYMask={}, emptyBlockYMask={}, skyUpdates=[[B@18c01798], blockUpdates=[[B@788045e5]]]
[22:14:46] [Render thread/DEBUG]: Ignoring packet due to disconnection: ClientboundSetEntityMotionPacket[id=122, movement=(-0.1334309955441616, -0.0783739241897089, 0.05713239333455422)]
[22:14:46] [Render thread/DEBUG]: Ignoring packet due to disconnection: Entity_net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$PosRot@256689a8
[22:14:46] [Download-2/INFO]: Could not authorize you against Realms server: java.lang.RuntimeException: Failed to parse into SignedJWT: 0
[22:14:46] [Download-2/ERROR]: Couldn't connect to realms
com.mojang.realmsclient.exception.RealmsServiceException: Realms authentication error with message 'java.lang.RuntimeException: Failed to parse into SignedJWT: 0'
	at com.mojang.realmsclient.client.RealmsClient.execute(RealmsClient.java:625)
	at com.mojang.realmsclient.client.RealmsClient.clientCompatible(RealmsClient.java:354)
	at com.mojang.realmsclient.RealmsAvailability.lambda$check$0(RealmsAvailability.java:48)
	at java.base/java.util.concurrent.CompletableFuture$AsyncSupply.run(CompletableFuture.java:1789)
	at java.base/java.util.concurrent.ThreadPoolExecutor.runWorker(ThreadPoolExecutor.java:1090)
	at java.base/java.util.concurrent.ThreadPoolExecutor$Worker.run(ThreadPoolExecutor.java:614)
	at java.base/java.lang.Thread.run(Thread.java:1474)
[ALSOFT] (EE) Could not query RTKit: No such file or directory (2)
[22:14:46] [Render thread/INFO]: OpenAL initialized on device No Output
[22:14:46] [Render thread/INFO]: Sound engine started
[22:14:46] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[22:14:46] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[22:14:46] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[22:14:46] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[22:14:46] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[22:14:46] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[22:14:46] [Render thread/INFO]: Created: 2048x2048x4 minecraft:textures/atlas/blocks.png-atlas
[22:14:46] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[22:14:46] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[22:14:46] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[22:14:46] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[22:14:47] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
--- exceptions (Realms/RTKit noise filtered) ---
58:	at com.mojang.authlib.minecraft.client.MinecraftClient.buildHttpException(MinecraftClient.java:170)
13886:[22:14:45] [Worker-Main-3/ERROR]: Couldn't compile pipeline (clod:lod_terrain/10): 
13887:com.mojang.renderpearl.util.ShaderCompileException: Couldn't parse GLSL: clod:post/debug_lens:62: error: '' :  syntax error, unexpected IDENTIFIER
14591:[22:14:45] [Worker-Main-3/ERROR]: Couldn't preload shader clod:shaders/post/debug_lens.fsh
14592:java.io.FileNotFoundException: clod:shaders/post/debug_lens.fsh
14611:[22:14:45] [Worker-Main-3/ERROR]: Couldn't find source for FRAGMENT shader (clod:post/debug_lens)
14628:[22:14:45] [Worker-Main-3/ERROR]: Couldn't find source for FRAGMENT shader (clod:post/debug_lens)
14691:[22:14:45] [Render thread/ERROR]: Failed to load post chain: clod:lod_terrain
14692:net.minecraft.client.renderer.ShaderManager$CompilationException: Failed to compile post processing pipeline clod:lod_terrain/10
14805:[22:14:46] [Render thread/DEBUG]: Ignoring packet due to disconnection: net.minecraft.network.protocol.game.ClientboundBlockUpdatePacket@83159b1
14806:[22:14:46] [Render thread/DEBUG]: Ignoring packet due to disconnection: net.minecraft.network.protocol.game.ClientboundSectionBlocksUpdatePacket@21da6532
14807:[22:14:46] [Render thread/DEBUG]: Ignoring packet due to disconnection: ClientboundLightUpdatePacket[x=-59, z=53, lightData=ClientboundLightUpdatePacketData[skyYMask={4}, blockYMask={4}, emptySkyYMask={}, emptyBlockYMask={}, skyUpdates=[[B@18c01798], blockUpdates=[[B@788045e5]]]
14808:[22:14:46] [Render thread/DEBUG]: Ignoring packet due to disconnection: ClientboundSetEntityMotionPacket[id=122, movement=(-0.1334309955441616, -0.0783739241897089, 0.05713239333455422)]
14809:[22:14:46] [Render thread/DEBUG]: Ignoring packet due to disconnection: Entity_net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$PosRot@256689a8
14811:[22:14:46] [Download-2/ERROR]: Couldn't connect to realms
```
### server log tail
```
[22:18:06 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:overworld'
[22:18:06 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:overworld'
[c-LoD] EB: enter, native_gen=true
[22:18:07 INFO]: [ChunkHolderManager] Saved 846 block chunks, 531 entity chunks, 13 poi chunks in world 'minecraft:overworld' in 0.84s
[22:18:07 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:overworld'
[22:18:07 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:overworld'
[22:18:07 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[22:18:07 INFO]: [ChunkHolderManager] Waiting 60s for chunk system to halt for world 'minecraft:the_nether'
[22:18:07 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:the_nether'
[22:18:07 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_nether'
[22:18:07 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_nether' in 0.00s
[22:18:07 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_nether'
[22:18:07 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_nether'
[22:18:07 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[22:18:07 INFO]: [ChunkHolderManager] Waiting 60s for chunk system to halt for world 'minecraft:the_end'
[22:18:07 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:the_end'
[22:18:07 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_end'
[22:18:07 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_end' in 0.00s
[22:18:07 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_end'
[22:18:07 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_end'
[22:18:07 INFO]: Waiting for all RegionFile I/O tasks to complete...
[22:18:07 INFO]: All RegionFile I/O tasks to complete
[22:18:07 INFO]: [MoonriseCommon] Awaiting termination of worker pool for up to 60s...
[22:18:07 INFO]: [MoonriseCommon] Awaiting termination of I/O pool for up to 60s...
[launcher] server exited with code 143
```
### shots
```
total 84
drwxr-xr-x 2 runner runner  4096 Nov 18 22:18 .
drwxr-xr-x 3 runner runner  4096 Nov 18 22:12 ..
-rw-r--r-- 1 runner runner 55287 Nov 18 22:18 server_full.log
-rw-r--r-- 1 runner runner 18377 Nov 18 22:18 summary.md
```

**exit=1** failures=1 leg1=0 leg2=0
