- phase: 0 sanity (12:20:33)
### c-LoD visual run (-930,796, marker=red_concrete, vantage -1005,150 yaw 270)
- GL:     Vendor: Mesa (0xffffffff);    Device: llvmpipe (LLVM 21.1.8, 256 bits) (0xffffffff);OpenGL renderer string: llvmpipe (LLVM 21.1.8, 256 bits);OpenGL core profile version string: 4.5 (Core Profile) Mesa 26.0.8-1ubuntu0.3;
- boot + gates OK (12:21:01)
- phase: 1 server booted (12:21:01)
- sky-grid armed
- phase: 2 leg-1 client joined (12:21:28)
- joined session 1 (12:21:28)
- stream live: cells=65536
- phase: 3 scan populated (12:21:51)
- terrain range: 2 terraced mountains at (CX-4,CZ-20)/(CX-2,CZ+16), tops 185/190, grass terraces + stone caps, 66..82 m from the eye (inside the 83.5 m band, chunk edge 64 m)
- scan complete=1 (cells=cells=65536/65536)
- paint settled=1 (wire 0p)
- **FAIL:** baseline try 1: client left the game mid-test (joins=1 lefts=1) — the shot is the main-menu panorama, not the LOD proof

### boot / stream
```
[c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 137297 bytes, sha1 d1b6702bfc2bd1be94558cdf1b697a852d544636 — zero textures, zero world data, one push per join
[12:20:57 INFO]: [c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 137297 bytes, sha1 d1b6702bfc2bd1be94558cdf1b697a852d544636 — zero textures, zero world data, one push per join
[c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[12:20:57 INFO]: [c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[c-LoD] stream: tick 500 65536/65536 cells, lap 327, cone 0 zone 0 dirty 0 (q 6603) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 7679c/355e, edits +1722p/1279c/167e drop 0, spawned 0 / failed 0, cam0 yaw -35 pitch 13, wish 411 forced 101, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 0p/0b fail 0 ~0 kB/s cache 0/24576 h0 m0 e0, srbeat 0s/0q drop 0stale/0undec, ereb 0s/55q, sweep 0r/0d
[12:22:01 INFO]: [c-LoD] stream: tick 500 65536/65536 cells, lap 327, cone 0 zone 0 dirty 0 (q 6603) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 7679c/355e, edits +1722p/1279c/167e drop 0, spawned 0 / failed 0, cam0 yaw -35 pitch 13, wish 411 forced 101, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 0p/0b fail 0 ~0 kB/s cache 0/24576 h0 m0 e0, srbeat 0s/0q drop 0stale/0undec, ereb 0s/55q, sweep 0r/0d
```
### sessions
```
[12:21:28 INFO]: System chat: Stepan joined the game
[12:22:04 INFO]: System chat: Stepan left the game
```
### client log tail
```
[12:22:04] [Netty Epoll IO #0/DEBUG]:  IN: [play:clientbound/minecraft:level_chunk_with_light] net.minecraft.network.protocol.game.ClientboundLevelChunkWithLightPacket -> 34771 bytes
[12:22:04] [Render thread/INFO]: Reloading ResourceManager: vanilla, vanilla
[12:22:04] [Worker-Main-3/INFO]: Found unifont_all_no_pua-17.0.01.hex, loading
[12:22:04] [Worker-Main-1/INFO]: Found unifont_pua-17.0.01.hex, loading
[12:22:04] [Worker-Main-3/INFO]: Found unifont_jp_patch-17.0.01.hex, loading
[12:22:05] [Download-2/INFO]: Could not authorize you against Realms server: java.lang.RuntimeException: Failed to parse into SignedJWT: 0
[12:22:05] [Download-2/ERROR]: Couldn't connect to realms
com.mojang.realmsclient.exception.RealmsServiceException: Realms authentication error with message 'java.lang.RuntimeException: Failed to parse into SignedJWT: 0'
	at com.mojang.realmsclient.client.RealmsClient.execute(RealmsClient.java:625)
	at com.mojang.realmsclient.client.RealmsClient.clientCompatible(RealmsClient.java:354)
	at com.mojang.realmsclient.RealmsAvailability.lambda$check$0(RealmsAvailability.java:48)
	at java.base/java.util.concurrent.CompletableFuture$AsyncSupply.run(CompletableFuture.java:1789)
	at java.base/java.util.concurrent.ThreadPoolExecutor.runWorker(ThreadPoolExecutor.java:1090)
	at java.base/java.util.concurrent.ThreadPoolExecutor$Worker.run(ThreadPoolExecutor.java:614)
	at java.base/java.lang.Thread.run(Thread.java:1474)
[ALSOFT] (EE) Could not query RTKit: No such file or directory (2)
[12:22:06] [Render thread/INFO]: OpenAL initialized on device No Output
[12:22:06] [Render thread/INFO]: Sound engine started
[12:22:06] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[12:22:06] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[12:22:06] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[12:22:06] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[12:22:06] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[12:22:06] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[12:22:06] [Render thread/INFO]: Created: 2048x2048x4 minecraft:textures/atlas/blocks.png-atlas
[12:22:06] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[12:22:06] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[12:22:06] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[12:22:06] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[12:22:06] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
```
### server log tail
```
[12:22:23 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:overworld'
[c-LoD] EB: enter, native_gen=true
[12:22:24 INFO]: [ChunkHolderManager] Saved 897 block chunks, 521 entity chunks, 13 poi chunks in world 'minecraft:overworld' in 1.15s
[12:22:24 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:overworld'
[12:22:24 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:overworld'
[c-LoD] EB: enter, native_gen=true
[12:22:24 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[12:22:24 INFO]: [ChunkHolderManager] Waiting 60s for chunk system to halt for world 'minecraft:the_nether'
[12:22:24 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:the_nether'
[12:22:24 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_nether'
[12:22:24 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_nether' in 0.00s
[12:22:24 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_nether'
[12:22:24 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_nether'
[12:22:24 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[12:22:24 INFO]: [ChunkHolderManager] Waiting 60s for chunk system to halt for world 'minecraft:the_end'
[12:22:24 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:the_end'
[12:22:24 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_end'
[12:22:24 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_end' in 0.00s
[12:22:24 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_end'
[12:22:24 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_end'
[12:22:24 INFO]: Waiting for all RegionFile I/O tasks to complete...
[12:22:24 INFO]: All RegionFile I/O tasks to complete
[12:22:24 INFO]: [MoonriseCommon] Awaiting termination of worker pool for up to 60s...
[12:22:24 INFO]: [MoonriseCommon] Awaiting termination of I/O pool for up to 60s...
[launcher] server exited with code 143
```
### shots
```
total 60
drwxr-xr-x 2 runner runner  4096 Nov 15 12:22 .
drwxr-xr-x 3 runner runner  4096 Nov 15 12:19 ..
-rw-r--r-- 1 runner runner 43016 Nov 15 12:22 server_full.log
-rw-r--r-- 1 runner runner  7630 Nov 15 12:22 summary.md
```

**exit=1** failures=1 leg1=0 leg2=0
