- phase: 0 sanity (11:06:48)
### c-LoD visual run (-930,796, marker=red_concrete, vantage -1005,150 yaw 270)
- GL:     Vendor: Mesa (0xffffffff);    Device: llvmpipe (LLVM 21.1.8, 256 bits) (0xffffffff);OpenGL renderer string: llvmpipe (LLVM 21.1.8, 256 bits);OpenGL core profile version string: 4.5 (Core Profile) Mesa 26.0.8-1ubuntu0.3;
- boot + gates OK (11:07:05)
- phase: 1 server booted (11:07:06)
- sky-grid armed
- phase: 2 leg-1 client joined (11:07:23)
- joined session 1 (11:07:23)
- stream live: cells=65536
- phase: 3 scan populated (11:07:45)
- terrain range: 2 terraced mountains at (CX-4,CZ-20)/(CX-2,CZ+16), tops 185/190, grass terraces + stone caps, 66..82 m from the eye (inside the 83.5 m band, chunk edge 64 m)
- scan complete=1 (cells=cells=65536/65536)
- paint settled=1 (wire 0p)
- **FAIL:** baseline try 1: client left the game mid-test (joins=1 lefts=1) — the shot is the main-menu panorama, not the LOD proof

### boot / stream
```
[c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 137297 bytes, sha1 d1b6702bfc2bd1be94558cdf1b697a852d544636 — zero textures, zero world data, one push per join
[11:07:02 INFO]: [c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 137297 bytes, sha1 d1b6702bfc2bd1be94558cdf1b697a852d544636 — zero textures, zero world data, one push per join
[c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[11:07:02 INFO]: [c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[c-LoD] stream: tick 400 65536/65536 cells, lap 23, cone 0 zone 0 dirty 0 (q 5131) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 7365c/1270e, edits +3662p/2734c/169e drop 0, spawned 0 / failed 0, cam0 yaw 0 pitch 0, wish 448 forced 64, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 0p/0b fail 0 ~0 kB/s cache 0/24576 h0 m0 e0, srbeat 0s/0q drop 0stale/0undec, ereb 0s/63q, sweep 0r/24d
[11:07:54 INFO]: [c-LoD] stream: tick 400 65536/65536 cells, lap 23, cone 0 zone 0 dirty 0 (q 5131) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 7365c/1270e, edits +3662p/2734c/169e drop 0, spawned 0 / failed 0, cam0 yaw 0 pitch 0, wish 448 forced 64, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 0p/0b fail 0 ~0 kB/s cache 0/24576 h0 m0 e0, srbeat 0s/0q drop 0stale/0undec, ereb 0s/63q, sweep 0r/24d
```
### sessions
```
[11:07:22 INFO]: System chat: Stepan joined the game
[11:07:56 INFO]: System chat: Stepan left the game
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
[11:07:58] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[11:07:58] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[11:07:58] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[11:07:58] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[11:07:58] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[11:07:58] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[11:07:58] [Render thread/INFO]: Created: 2048x2048x4 minecraft:textures/atlas/blocks.png-atlas
[11:07:58] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[11:07:58] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[11:07:58] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[11:07:58] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[11:07:58] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
```
### server log tail
```
[11:08:16 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:overworld'
[11:08:16 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:overworld'
[c-LoD] EB: enter, native_gen=true
[11:08:17 INFO]: [ChunkHolderManager] Saved 872 block chunks, 585 entity chunks, 13 poi chunks in world 'minecraft:overworld' in 0.95s
[11:08:17 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:overworld'
[11:08:17 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:overworld'
[11:08:17 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[11:08:17 INFO]: [ChunkHolderManager] Waiting 60s for chunk system to halt for world 'minecraft:the_nether'
[11:08:17 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:the_nether'
[11:08:17 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_nether'
[11:08:17 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_nether' in 0.00s
[11:08:17 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_nether'
[11:08:17 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_nether'
[11:08:17 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[11:08:17 INFO]: [ChunkHolderManager] Waiting 60s for chunk system to halt for world 'minecraft:the_end'
[11:08:17 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:the_end'
[11:08:17 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_end'
[11:08:17 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_end' in 0.00s
[11:08:17 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_end'
[11:08:17 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_end'
[11:08:17 INFO]: Waiting for all RegionFile I/O tasks to complete...
[11:08:17 INFO]: All RegionFile I/O tasks to complete
[11:08:17 INFO]: [MoonriseCommon] Awaiting termination of worker pool for up to 60s...
[11:08:17 INFO]: [MoonriseCommon] Awaiting termination of I/O pool for up to 60s...
[launcher] server exited with code 143
```
### shots
```
total 56
drwxr-xr-x 2 runner runner  4096 Nov 15 11:08 .
drwxr-xr-x 3 runner runner  4096 Nov 15 11:06 ..
-rw-r--r-- 1 runner runner 38666 Nov 15 11:08 server_full.log
-rw-r--r-- 1 runner runner  7523 Nov 15 11:08 summary.md
```

**exit=1** failures=1 leg1=0 leg2=0
