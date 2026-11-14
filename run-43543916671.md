- phase: 0 sanity (02:02:38)
### c-LoD visual run (-930,796, marker=red_concrete, vantage -1005,150 yaw 270)
- GL:     Vendor: Mesa (0xffffffff);    Device: llvmpipe (LLVM 21.1.8, 256 bits) (0xffffffff);OpenGL renderer string: llvmpipe (LLVM 21.1.8, 256 bits);OpenGL core profile version string: 4.5 (Core Profile) Mesa 26.0.8-1ubuntu0.3;
- boot + gates OK (02:03:07)
- phase: 1 server booted (02:03:07)
- sky-grid armed
- phase: 2 leg-1 client joined (02:03:34)
- joined session 1 (02:03:34)
- stream live: cells=65536
- phase: 3 scan populated (02:04:01)
- terrain range: 2 terraced mountains at (CX-4,CZ-20)/(CX-2,CZ+16), tops 185/190, grass terraces + stone caps, 66..82 m from the eye (inside the 83.5 m band, chunk edge 64 m)
- scan complete=1 (cells=cells=65536/65536)
- paint settled=1 (wire 0p)
- **FAIL:** baseline try 1: client left the game mid-test (joins=1 lefts=1) — the shot is the main-menu panorama, not the LOD proof

### boot / stream
```
[c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 133468 bytes, sha1 2b551a509429ae820d8de7653b4a4367f03d15e0 — zero textures, zero world data, one push per join
[02:03:03 INFO]: [c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 133468 bytes, sha1 2b551a509429ae820d8de7653b4a4367f03d15e0 — zero textures, zero world data, one push per join
[02:03:03 INFO]: [c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[c-LoD] stream: tick 500 65536/65536 cells, lap 334, cone 0 zone 0 dirty 0 (q 5929) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 4388c/359e, edits +1498p/1069c/138e drop 0, spawned 0 / failed 0, cam0 yaw 0 pitch 0, wish 424 forced 88, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 0p/0b fail 0 ~0 kB/s cache 0/24576 h0 m0 e0, srbeat 0s/0q drop 0stale/0undec, ereb 0s/0q
[02:04:08 INFO]: [c-LoD] stream: tick 500 65536/65536 cells, lap 334, cone 0 zone 0 dirty 0 (q 5929) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 4388c/359e, edits +1498p/1069c/138e drop 0, spawned 0 / failed 0, cam0 yaw 0 pitch 0, wish 424 forced 88, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 0p/0b fail 0 ~0 kB/s cache 0/24576 h0 m0 e0, srbeat 0s/0q drop 0stale/0undec, ereb 0s/0q
```
### sessions
```
[02:03:34 INFO]: System chat: Stepan joined the game
[02:04:13 INFO]: System chat: Stepan left the game
```
### client log tail
```
	at net.minecraft.server.packs.resources.SimpleReloadInstance.lambda$prepareTasks$3(SimpleReloadInstance.java:59)
	at net.minecraft.util.thread.BlockableEventLoop.doRunTask(BlockableEventLoop.java:173)
	at net.minecraft.util.thread.ReentrantBlockableEventLoop.doRunTask(ReentrantBlockableEventLoop.java:23)
	at net.minecraft.util.thread.BlockableEventLoop.pollTask(BlockableEventLoop.java:147)
	at net.minecraft.util.thread.BlockableEventLoop.runAllTasks(BlockableEventLoop.java:127)
	at net.minecraft.client.Minecraft.runTick(Minecraft.java:1240)
	at net.minecraft.client.Minecraft.run(Minecraft.java:967)
	at net.minecraft.client.main.Main.main(Main.java:292)
[02:04:15] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[02:04:15] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[02:04:15] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[02:04:15] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[02:04:15] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[02:04:15] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[02:04:15] [Render thread/INFO]: Created: 2048x2048x4 minecraft:textures/atlas/blocks.png-atlas
[02:04:15] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[02:04:15] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[02:04:15] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[02:04:15] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[02:04:15] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
[02:04:16] [Download-2/INFO]: Could not authorize you against Realms server: java.lang.RuntimeException: Failed to parse into SignedJWT: 0
[02:04:16] [Download-2/ERROR]: Couldn't connect to realms
com.mojang.realmsclient.exception.RealmsServiceException: Realms authentication error with message 'java.lang.RuntimeException: Failed to parse into SignedJWT: 0'
	at com.mojang.realmsclient.client.RealmsClient.execute(RealmsClient.java:625)
	at com.mojang.realmsclient.client.RealmsClient.clientCompatible(RealmsClient.java:354)
	at com.mojang.realmsclient.RealmsAvailability.lambda$check$0(RealmsAvailability.java:48)
	at java.base/java.util.concurrent.CompletableFuture$AsyncSupply.run(CompletableFuture.java:1789)
	at java.base/java.util.concurrent.ThreadPoolExecutor.runWorker(ThreadPoolExecutor.java:1090)
	at java.base/java.util.concurrent.ThreadPoolExecutor$Worker.run(ThreadPoolExecutor.java:614)
	at java.base/java.lang.Thread.run(Thread.java:1474)
```
### server log tail
```
[02:04:32 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:overworld'
[c-LoD] EB: enter, native_gen=true
[02:04:34 INFO]: [ChunkHolderManager] Saved 897 block chunks, 525 entity chunks, 13 poi chunks in world 'minecraft:overworld' in 1.35s
[02:04:34 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:overworld'
[02:04:34 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:overworld'
[02:04:34 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[02:04:34 INFO]: [ChunkHolderManager] Waiting 60s for chunk system to halt for world 'minecraft:the_nether'
[02:04:34 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:the_nether'
[02:04:34 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_nether'
[02:04:34 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_nether' in 0.00s
[02:04:34 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_nether'
[02:04:34 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_nether'
[02:04:34 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[02:04:34 INFO]: [ChunkHolderManager] Waiting 60s for chunk system to halt for world 'minecraft:the_end'
[02:04:34 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:the_end'
[02:04:34 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_end'
[02:04:34 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_end' in 0.00s
[02:04:34 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_end'
[02:04:34 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_end'
[02:04:34 INFO]: Waiting for all RegionFile I/O tasks to complete...
[02:04:34 INFO]: All RegionFile I/O tasks to complete
[02:04:34 INFO]: [MoonriseCommon] Awaiting termination of worker pool for up to 60s...
[02:04:34 INFO]: [MoonriseCommon] Awaiting termination of I/O pool for up to 60s...
[c-LoD] EB: enter, native_gen=true
[launcher] server exited with code 143
```
### shots
```
total 60
drwxr-xr-x 2 runner runner  4096 Nov 14 02:04 .
drwxr-xr-x 3 runner runner  4096 Nov 14 02:01 ..
-rw-r--r-- 1 runner runner 43564 Nov 14 02:04 server_full.log
-rw-r--r-- 1 runner runner  7567 Nov 14 02:04 summary.md
```

**exit=1** failures=1 leg1=0 leg2=0
