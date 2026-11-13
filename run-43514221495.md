- phase: 0 sanity (21:51:54)
### c-LoD visual run (-930,796, marker=red_concrete, vantage -1005,150 yaw 270)
- GL:     Vendor: Mesa (0xffffffff);    Device: llvmpipe (LLVM 21.1.8, 256 bits) (0xffffffff);OpenGL renderer string: llvmpipe (LLVM 21.1.8, 256 bits);OpenGL core profile version string: 4.5 (Core Profile) Mesa 26.0.8-1ubuntu0.3;
- boot + gates OK (21:52:26)
- phase: 1 server booted (21:52:26)
- sky-grid armed
- phase: 2 leg-1 client joined (21:52:53)
- joined session 1 (21:52:53)
- stream live: cells=65536
- phase: 3 scan populated (21:53:17)
- terrain range: 2 terraced mountains at (CX-4,CZ-20)/(CX-2,CZ+16), tops 185/190, grass terraces + stone caps, 66..82 m from the eye (inside the 83.5 m band, chunk edge 64 m)
- scan complete=1 (cells=cells=65536/65536)
- paint settled=1 (wire 0p)
- **FAIL:** baseline try 1: client left the game mid-test (joins=1 lefts=1) — the shot is the main-menu panorama, not the LOD proof

### boot / stream
```
[c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 133295 bytes, sha1 5b399dacbd39fa84c875677f820f1f2366d2bd6a — zero textures, zero world data, one push per join
[21:52:22 INFO]: [c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 133295 bytes, sha1 5b399dacbd39fa84c875677f820f1f2366d2bd6a — zero textures, zero world data, one push per join
[21:52:22 INFO]: [c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[c-LoD] stream: tick 500 65536/65536 cells, lap 338, cone 0 zone 0 dirty 0 (q 6263) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 9318c/1234e, edits +1125p/802c/118e drop 0, spawned 0 / failed 0, cam0 yaw -35 pitch 13, wish 418 forced 94, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 0p/0b fail 0 ~0 kB/s cache 0/24576 h0 m0 e0, srbeat 0s/0q drop 0stale/0undec, ereb 0s/0q
[21:53:26 INFO]: [c-LoD] stream: tick 500 65536/65536 cells, lap 338, cone 0 zone 0 dirty 0 (q 6263) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 9318c/1234e, edits +1125p/802c/118e drop 0, spawned 0 / failed 0, cam0 yaw -35 pitch 13, wish 418 forced 94, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 0p/0b fail 0 ~0 kB/s cache 0/24576 h0 m0 e0, srbeat 0s/0q drop 0stale/0undec, ereb 0s/0q
```
### sessions
```
[21:52:51 INFO]: System chat: Stepan joined the game
[21:53:29 INFO]: System chat: Stepan left the game
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
[21:53:31] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[21:53:31] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[21:53:31] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[21:53:31] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[21:53:31] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[21:53:31] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[21:53:31] [Render thread/INFO]: Created: 2048x2048x4 minecraft:textures/atlas/blocks.png-atlas
[21:53:31] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[21:53:31] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[21:53:31] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[21:53:31] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[21:53:31] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
```
### server log tail
```
[21:53:48 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:overworld'
[c-LoD] EB: enter, native_gen=true
[21:53:49 INFO]: [ChunkHolderManager] Saved 897 block chunks, 529 entity chunks, 13 poi chunks in world 'minecraft:overworld' in 1.12s
[21:53:49 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:overworld'
[21:53:49 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:overworld'
[21:53:49 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[21:53:49 INFO]: [ChunkHolderManager] Waiting 60s for chunk system to halt for world 'minecraft:the_nether'
[21:53:49 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:the_nether'
[21:53:49 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_nether'
[21:53:49 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_nether' in 0.00s
[21:53:49 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_nether'
[21:53:49 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_nether'
[21:53:49 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[21:53:49 INFO]: [ChunkHolderManager] Waiting 60s for chunk system to halt for world 'minecraft:the_end'
[21:53:49 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:the_end'
[21:53:49 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_end'
[21:53:49 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_end' in 0.00s
[21:53:49 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_end'
[21:53:49 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_end'
[21:53:49 INFO]: Waiting for all RegionFile I/O tasks to complete...
[21:53:49 INFO]: All RegionFile I/O tasks to complete
[21:53:49 INFO]: [MoonriseCommon] Awaiting termination of worker pool for up to 60s...
[21:53:49 INFO]: [MoonriseCommon] Awaiting termination of I/O pool for up to 60s...
[c-LoD] EB: enter, native_gen=true
[launcher] server exited with code 143
```
### shots
```
total 16
drwxr-xr-x 2 runner runner 4096 Nov 13 21:51 .
drwxr-xr-x 3 runner runner 4096 Nov 13 21:51 ..
-rw-r--r-- 1 runner runner 7444 Nov 13 21:53 summary.md
```

**exit=1** failures=1 leg1=0 leg2=0
