- phase: 0 sanity (01:33:02)
### c-LoD visual run (-930,796, marker=red_concrete, vantage -1005,150 yaw 270)
- GL:     Vendor: Mesa (0xffffffff);    Device: llvmpipe (LLVM 21.1.8, 256 bits) (0xffffffff);OpenGL renderer string: llvmpipe (LLVM 21.1.8, 256 bits);OpenGL core profile version string: 4.5 (Core Profile) Mesa 26.0.8-1ubuntu0.3;
- boot + gates OK (01:33:31)
- phase: 1 server booted (01:33:31)
- sky-grid armed
- phase: 2 leg-1 client joined (01:34:00)
- joined session 1 (01:34:00)
- stream live: cells=65536
- phase: 3 scan populated (01:34:24)
- terrain range: 2 terraced mountains at (CX-4,CZ-20)/(CX-2,CZ+16), tops 185/190, grass terraces + stone caps, 66..82 m from the eye (inside the 83.5 m band, chunk edge 64 m)
- scan complete=1 (cells=cells=65536/65536)
- paint settled=1 (wire 0p)
- **FAIL:** baseline try 1: client left the game mid-test (joins=1 lefts=1) — the shot is the main-menu panorama, not the LOD proof

### boot / stream
```
[c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 133468 bytes, sha1 2b551a509429ae820d8de7653b4a4367f03d15e0 — zero textures, zero world data, one push per join
[c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[01:33:28 INFO]: [c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 133468 bytes, sha1 2b551a509429ae820d8de7653b4a4367f03d15e0 — zero textures, zero world data, one push per join
[01:33:28 INFO]: [c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[c-LoD] stream: tick 500 65536/65536 cells, lap 343, cone 0 zone 0 dirty 0 (q 5425) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 8080c/468e, edits +2252p/1640c/162e drop 0, spawned 0 / failed 0, cam0 yaw -35 pitch 13, wish 425 forced 87, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 0p/0b fail 0 ~0 kB/s cache 0/24576 h0 m0 e0, srbeat 0s/0q drop 0stale/0undec, ereb 0s/0q
[01:34:33 INFO]: [c-LoD] stream: tick 500 65536/65536 cells, lap 343, cone 0 zone 0 dirty 0 (q 5425) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 8080c/468e, edits +2252p/1640c/162e drop 0, spawned 0 / failed 0, cam0 yaw -35 pitch 13, wish 425 forced 87, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 0p/0b fail 0 ~0 kB/s cache 0/24576 h0 m0 e0, srbeat 0s/0q drop 0stale/0undec, ereb 0s/0q
```
### sessions
```
[01:33:59 INFO]: System chat: Stepan joined the game
[01:34:35 INFO]: System chat: Stepan left the game
```
### client log tail
```
java.lang.IllegalStateException: Failed to open OpenAL device
	at com.mojang.blaze3d.audio.Library.openDeviceOrFallback(Library.java:274)
	at com.mojang.blaze3d.audio.Library.init(Library.java:149)
	at net.minecraft.client.sounds.SoundEngine.loadLibrary(SoundEngine.java:121)
	at net.minecraft.client.sounds.SoundEngine.reload(SoundEngine.java:110)
	at net.minecraft.client.sounds.SoundManager.apply(SoundManager.java:122)
	at net.minecraft.client.sounds.SoundManager.apply(SoundManager.java:41)
	at net.minecraft.server.packs.resources.SimplePreparableReloadListener.lambda$reload$1(SimplePreparableReloadListener.java:15)
	at java.base/java.util.concurrent.CompletableFuture$UniAccept.tryFire(CompletableFuture.java:739)
	at java.base/java.util.concurrent.CompletableFuture$Completion.run(CompletableFuture.java:503)
	at net.minecraft.server.packs.resources.SimpleReloadInstance.lambda$prepareTasks$3(SimpleReloadInstance.java:59)
	at net.minecraft.util.thread.BlockableEventLoop.doRunTask(BlockableEventLoop.java:173)
	at net.minecraft.util.thread.ReentrantBlockableEventLoop.doRunTask(ReentrantBlockableEventLoop.java:23)
	at net.minecraft.util.thread.BlockableEventLoop.pollTask(BlockableEventLoop.java:147)
	at net.minecraft.util.thread.BlockableEventLoop.runAllTasks(BlockableEventLoop.java:127)
	at net.minecraft.client.Minecraft.runTick(Minecraft.java:1240)
	at net.minecraft.client.Minecraft.run(Minecraft.java:967)
	at net.minecraft.client.main.Main.main(Main.java:292)
[01:34:38] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[01:34:38] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[01:34:38] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[01:34:38] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[01:34:38] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[01:34:38] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[01:34:38] [Render thread/INFO]: Created: 2048x2048x4 minecraft:textures/atlas/blocks.png-atlas
[01:34:38] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[01:34:38] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[01:34:38] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[01:34:38] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[01:34:38] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
```
### server log tail
```
[c-LoD] EB: enter, native_gen=true
[c-LoD] EB: enter, native_gen=true
[01:34:56 INFO]: [ChunkHolderManager] Saved 897 block chunks, 512 entity chunks, 13 poi chunks in world 'minecraft:overworld' in 1.24s
[01:34:56 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:overworld'
[01:34:56 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:overworld'
[01:34:56 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[01:34:56 INFO]: [ChunkHolderManager] Waiting 60s for chunk system to halt for world 'minecraft:the_nether'
[01:34:56 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:the_nether'
[01:34:56 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_nether'
[01:34:56 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_nether' in 0.00s
[01:34:56 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_nether'
[01:34:56 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_nether'
[01:34:56 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[01:34:56 INFO]: [ChunkHolderManager] Waiting 60s for chunk system to halt for world 'minecraft:the_end'
[01:34:56 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:the_end'
[01:34:56 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_end'
[01:34:56 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_end' in 0.00s
[01:34:56 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_end'
[01:34:56 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_end'
[01:34:56 INFO]: Waiting for all RegionFile I/O tasks to complete...
[01:34:56 INFO]: All RegionFile I/O tasks to complete
[01:34:56 INFO]: [MoonriseCommon] Awaiting termination of worker pool for up to 60s...
[01:34:56 INFO]: [MoonriseCommon] Awaiting termination of I/O pool for up to 60s...
[c-LoD] EB: enter, native_gen=true
[launcher] server exited with code 143
```
### shots
```
total 4472
drwxr-xr-x 2 runner runner    4096 Nov 14 01:34 .
drwxr-xr-x 3 runner runner    4096 Nov 14 01:32 ..
-rw-r--r-- 1 runner runner 4559355 Nov 14 01:34 server_full.log
-rw-r--r-- 1 runner runner    7385 Nov 14 01:34 summary.md
```

**exit=1** failures=1 leg1=0 leg2=0
