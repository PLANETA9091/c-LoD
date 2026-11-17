- phase: 0 sanity (12:55:03)
### c-LoD visual run (-930,796, marker=red_concrete, vantage -1005,150 yaw 270)
- GL:     Vendor: Mesa (0xffffffff);    Device: llvmpipe (LLVM 21.1.8, 256 bits) (0xffffffff);OpenGL renderer string: llvmpipe (LLVM 21.1.8, 256 bits);OpenGL core profile version string: 4.5 (Core Profile) Mesa 26.0.8-1ubuntu0.3;
- boot + gates OK (12:55:25)
- phase: 1 server booted (12:55:25)
- sky-grid armed

### boot / stream
```
[c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 165296 bytes, sha1 2c78a7559b01ae16ea3c77b9f0a8fc3659bab250 — zero textures, zero world data, one push per join
[12:55:22 INFO]: [c-LoD] c-LoD pack finalized (ONE-TIME, static): shader-only, 165296 bytes, sha1 2c78a7559b01ae16ea3c77b9f0a8fc3659bab250 — zero textures, zero world data, one push per join
[12:55:22 INFO]: [c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[c-LoD] c-LoD sky-grid stream armed: 256x256 cells at (-1186,540), 2048 particles/tick, 256 ticks/lap
[c-LoD] stream: tick 500 0/65536 cells, lap 0, cone 0 zone 0 dirty 0 (q 0) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 0c/0e, edits +0p/0c/0e drop 0, spawned 0 / failed 0, cam0 none, wish 0 forced 0, native 0c/0ch fail 0 (0w q 0) helper off, wire bundle 0p/0b fail 0 ~0 kB/s cache 0/24576 h0 m0 e0, srbeat 0s/0q drop 0stale/0undec, ereb 0s/0q, sweep 0r/0d
[13:01:59 INFO]: [c-LoD] stream: tick 500 0/65536 cells, lap 0, cone 0 zone 0 dirty 0 (q 0) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 0c/0e, edits +0p/0c/0e drop 0, spawned 0 / failed 0, cam0 none, wish 0 forced 0, native 0c/0ch fail 0 (0w q 0) helper off, wire bundle 0p/0b fail 0 ~0 kB/s cache 0/24576 h0 m0 e0, srbeat 0s/0q drop 0stale/0undec, ereb 0s/0q, sweep 0r/0d
```
### sessions
```
```
### client log tail
```
Horizon 0.1.0
Horizon: Horizon's prepared game files are missing: no horizon-resources.json in /home/runner/horizon-test/.local/horizon-client/26.3
horizon-prepare --prepare, or pass --resources <folder>
Horizon renderer: Vulkan has no conformant adapter (Vulkan: only a software adapter (llvmpipe (LLVM 21.1.8, 256 bits))); asking again for the ones wgpu hides as non-conformant
Horizon renderer: trying OpenGL (Vulkan: only a software adapter (llvmpipe (LLVM 21.1.8, 256 bits)))
libEGL warning: DRI3 error: Could not get DRI3 device
libEGL warning: Ensure your X server supports DRI3 to get accelerated rendering
Horizon renderer: OpenGL has no conformant adapter (OpenGL: only a software adapter (llvmpipe (LLVM 21.1.8, 256 bits))); asking again for the ones wgpu hides as non-conformant
libEGL warning: DRI3 error: Could not get DRI3 device
libEGL warning: Ensure your X server supports DRI3 to get accelerated rendering
Horizon renderer: llvmpipe (LLVM 21.1.8, 256 bits) (Vulkan, Cpu)
Horizon renderer: requesting the device (driver llvmpipe Mesa 26.0.8-1ubuntu0.3 (LLVM 21.1.8)) with features [TEXTURE_FORMAT_16BIT_NORM | TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES | PIPELINE_STATISTICS_QUERY | TIMESTAMP_QUERY_INSIDE_ENCODERS | TIMESTAMP_QUERY_INSIDE_PASSES | MAPPABLE_PRIMARY_BUFFERS | MULTI_DRAW_INDIRECT_COUNT | VERTEX_WRITABLE_STORAGE | CLEAR_TEXTURE | TEXTURE_ATOMIC | SHADER_EARLY_DEPTH_TEST | SUBGROUP | TIMESTAMP_QUERY | INDIRECT_FIRST_INSTANCE | RG11B10UFLOAT_RENDERABLE | FLOAT32_FILTERABLE | FLOAT32_BLENDABLE] and the adapter's limits (buffers up to 2147483647 bytes, storage bindings up to 134217728 bytes, 2048 texture layers)
Horizon renderer: device ready; configuring the surface (1040x700 Bgra8Unorm, Mailbox)
Horizon renderer: the chosen graphics API gave no GPU (No Vulkan or OpenGL graphics driver found: Horizon needs the Vulkan loader (libvulkan.so.1) and a Vulkan driver for your GPU (Mesa's, or NVIDIA's), or OpenGL through EGL (Vulkan: only a software adapter (llvmpipe (LLVM 21.1.8, 256 bits)); OpenGL: only a software adapter (llvmpipe (LLVM 21.1.8, 256 bits)))); rendering on the CPU, which is slow
Horizon renderer: HDR output not offered by this display
Horizon renderer: creating the pipelines
Horizon renderer: pipelines ready (multi-draw on, occlusion culling off, GPU culling unavailable, GPU timer on)
Horizon renderer: acquiring the first frame
Horizon renderer: submitting the first frame
Horizon renderer: presenting the first frame
X connection to :99 broken (explicit kill or server shutdown).
--- exceptions (Realms/RTKit noise filtered) ---
(no client log)
```
### server log tail
```
[13:03:29 INFO]: [ChunkHolderManager] Waiting 60s for chunk system to halt for world 'minecraft:overworld'
[13:03:29 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:overworld'
[13:03:29 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:overworld'
[13:03:29 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:overworld' in 0.00s
[13:03:29 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:overworld'
[13:03:29 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:overworld'
[13:03:29 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[13:03:29 INFO]: [ChunkHolderManager] Waiting 60s for chunk system to halt for world 'minecraft:the_nether'
[13:03:29 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:the_nether'
[13:03:29 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_nether'
[13:03:29 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_nether' in 0.00s
[13:03:29 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_nether'
[13:03:29 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_nether'
[13:03:29 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[13:03:29 INFO]: [ChunkHolderManager] Waiting 60s for chunk system to halt for world 'minecraft:the_end'
[13:03:29 INFO]: [ChunkHolderManager] Halted chunk system for world 'minecraft:the_end'
[13:03:29 INFO]: [ChunkHolderManager] Saving all chunkholders for world 'minecraft:the_end'
[13:03:29 INFO]: [ChunkHolderManager] Saved 0 block chunks, 0 entity chunks, 0 poi chunks in world 'minecraft:the_end' in 0.00s
[13:03:29 INFO]: [ChunkHolderManager] Waiting 60s for chunk I/O to halt for world 'minecraft:the_end'
[13:03:29 INFO]: [ChunkHolderManager] Halted I/O scheduler for world 'minecraft:the_end'
[13:03:29 INFO]: Waiting for all RegionFile I/O tasks to complete...
[13:03:29 INFO]: All RegionFile I/O tasks to complete
[13:03:29 INFO]: [MoonriseCommon] Awaiting termination of worker pool for up to 60s...
[13:03:29 INFO]: [MoonriseCommon] Awaiting termination of I/O pool for up to 60s...
[launcher] server exited with code 143
```
### shots
```
total 56
drwxr-xr-x 2 runner runner  4096 Nov 17 13:03 .
drwxr-xr-x 3 runner runner  4096 Nov 17 12:38 ..
-rw-r--r-- 1 runner runner 39619 Nov 17 13:03 server_full.log
-rw-r--r-- 1 runner runner  6788 Nov 17 13:03 summary.md
```

**exit=1** failures=0 leg1=0 leg2=0
