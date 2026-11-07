# voxy reference run 42373181009 (#263) — FAILURE (no shots) 2026-11-07T12:41:23Z

The job produced zero screenshots (shots dir exists: yes, sha f360061).
See run-42373181009/ for client.log, server.log, steps.log and leg.log (tails below).

## steps.log tail (per-step trace written by the job steps)
```
2026-11-07T12:35:45Z X11+GL+pip start
2026-11-07T12:36:06Z X11+GL+pip ok
2026-11-07T12:36:06Z sRGB shim start
2026-11-07T12:36:06Z sRGB shim ok
2026-11-07T12:36:11Z setup_client start
2026-11-07T12:36:12Z setup_client ok
2026-11-07T12:36:12Z fabric layer start
2026-11-07T12:36:12Z voxy-setup: version json ok
2026-11-07T12:36:12Z voxy-setup: downloading fabric installer
2026-11-07T12:36:12Z voxy-setup: running fabric installer
2026-11-07T12:36:15Z voxy-setup: fabric installer done
2026-11-07T12:36:15Z voxy-setup: loader profile ok
2026-11-07T12:36:15Z voxy-setup: mod fetch check: fabric-api-0.165.0+26.3.jar
2026-11-07T12:36:15Z voxy-setup: mod ok: fabric-api-0.165.0+26.3.jar
2026-11-07T12:36:15Z voxy-setup: mod fetch check: sodium-fabric-0.9.2+mc26.3.jar
2026-11-07T12:36:15Z voxy-setup: mod ok: sodium-fabric-0.9.2+mc26.3.jar
2026-11-07T12:36:15Z voxy-setup: mod fetch check: voxy-0.2.20-beta.jar
2026-11-07T12:36:17Z voxy-setup: mod ok: voxy-0.2.20-beta.jar
2026-11-07T12:36:17Z voxy-setup: writing voxy config (rd=8, threads=2)
2026-11-07T12:36:17Z voxy-setup: fabric layer ready
2026-11-07T12:36:17Z fabric layer ok
2026-11-07T12:36:21Z voxy leg start
2026-11-07T12:36:21Z voxy leg: start (full leg stdout/stderr teed to voxy-leg.log)
2026-11-07T12:36:44Z voxy leg: server up (Done ( in voxy_server.log)
2026-11-07T12:36:47Z voxy leg: client launch issued
2026-11-07T12:37:16Z voxy leg: client joined
2026-11-07T12:37:23Z voxy leg: meshing wait 240s
2026-11-07T12:41:23Z voxy leg: meshing wait done
```

## client.log tail
```
	at knot//net.minecraft.network.PacketProcessor.processQueuedPackets(PacketProcessor.java:38)
	at knot//net.minecraft.client.Minecraft.runTick(Minecraft.java:1238)
	at knot//net.minecraft.client.Minecraft.run(Minecraft.java:967)
	at knot//net.minecraft.client.main.Main.main(Main.java:292)
	at net.fabricmc.loader.impl.game.minecraft.MinecraftGameProvider.launch(MinecraftGameProvider.java:517)
	at net.fabricmc.loader.impl.launch.knot.Knot.launch(Knot.java:72)
	at net.fabricmc.loader.impl.launch.knot.KnotClient.main(KnotClient.java:23)

[12:37:17] [Render thread/WARN]: [me.cx.vy.ct.ce.gl.sr.Shader$Builder]: 0:455(40): warning: some implementations may not support implicit int -> uint conversions for `&' operators; consider casting explicitly for portability

[12:37:17] [Render thread/INFO]: OpenGL debug message: java.lang.Throwable: id=5, source=SHADER COMPILER, type=OTHER, severity=HIGH, message='0:203(52): warning: some implementations may not support implicit int -> uint conversions for `&' operators; consider casting explicitly for portability'
java.lang.Throwable: id=5, source=SHADER COMPILER, type=OTHER, severity=HIGH, message='0:203(52): warning: some implementations may not support implicit int -> uint conversions for `&' operators; consider casting explicitly for portability'
	at knot//com.mojang.renderpearl.backend.opengl.GlDebug.wrapOperation$bil000$voxy$wrapDebug(GlDebug.java:520)
	at knot//com.mojang.renderpearl.backend.opengl.GlDebug.printDebugLog(GlDebug.java:83)
	at knot//org.lwjgl.opengl.GL20C.glCompileShader(Native Method)
	at knot//me.cortex.voxy.client.core.gl.shader.Shader$Builder.createShader(Shader.java:212)
	at knot//me.cortex.voxy.client.core.gl.shader.Shader$Builder.compileToProgram(Shader.java:164)
	at knot//me.cortex.voxy.client.core.gl.shader.Shader$Builder.compile(Shader.java:184)
	at knot//me.cortex.voxy.client.core.rendering.section.backend.mdic.MDICSectionRenderer.<init>(MDICSectionRenderer.java:87)
	at java.base/jdk.internal.reflect.DirectConstructorHandleAccessor.newInstance(DirectConstructorHandleAccessor.java:62)
	at java.base/java.lang.reflect.Constructor.newInstanceWithCaller(Constructor.java:499)
	at java.base/java.lang.reflect.Constructor.newInstance(Constructor.java:483)
	at knot//me.cortex.voxy.client.core.rendering.section.backend.AbstractSectionRenderer$Factory.lambda$create$0(AbstractSectionRenderer.java:42)
	at knot//me.cortex.voxy.client.core.rendering.section.backend.AbstractSectionRenderer$Factory.create(AbstractSectionRenderer.java:25)
	at knot//me.cortex.voxy.client.core.VoxyRenderSystem.<init>(VoxyRenderSystem.java:147)
	at knot//net.minecraft.client.renderer.LevelRenderer.voxy$createEngineDirect(LevelRenderer.java:2905)
	at knot//net.minecraft.client.renderer.LevelRenderer.voxy$createRenderer(LevelRenderer.java:2897)
	at knot//net.minecraft.client.renderer.extract.LevelExtractor.handler$bie000$voxy$reload(LevelExtractor.java:3253)
	at knot//net.minecraft.client.renderer.extract.LevelExtractor.allChanged(LevelExtractor.java)
	at knot//net.minecraft.client.renderer.extract.LevelExtractor.setLevel(LevelExtractor.java:537)
	at knot//net.minecraft.client.Minecraft.updateLevelInEngines(Minecraft.java:2336)
	at knot//net.minecraft.client.Minecraft.updateLevelInEngines(Minecraft.java:2326)
	at knot//net.minecraft.client.Minecraft.setLevel(Minecraft.java:2183)
	at knot//net.minecraft.client.multiplayer.ClientPacketListener.handleLogin(ClientPacketListener.java:550)
	at knot//net.minecraft.network.protocol.game.ClientboundLoginPacket.handle(ClientboundLoginPacket.java:54)
	at knot//net.minecraft.network.protocol.game.ClientboundLoginPacket.handle(ClientboundLoginPacket.java:17)
	at knot//net.minecraft.network.PacketProcessor$ListenerAndPacket.handle(PacketProcessor.java:55)
	at knot//net.minecraft.network.PacketProcessor.processQueuedPackets(PacketProcessor.java:38)
	at knot//net.minecraft.client.Minecraft.runTick(Minecraft.java:1238)
	at knot//net.minecraft.client.Minecraft.run(Minecraft.java:967)
	at knot//net.minecraft.client.main.Main.main(Main.java:292)
	at net.fabricmc.loader.impl.game.minecraft.MinecraftGameProvider.launch(MinecraftGameProvider.java:517)
	at net.fabricmc.loader.impl.launch.knot.Knot.launch(Knot.java:72)
	at net.fabricmc.loader.impl.launch.knot.KnotClient.main(KnotClient.java:23)

[12:37:17] [Render thread/WARN]: [me.cx.vy.ct.ce.gl.sr.Shader$Builder]: 0:203(52): warning: some implementations may not support implicit int -> uint conversions for `&' operators; consider casting explicitly for portability

[12:37:17] [Render thread/INFO]: [me.cx.vy.ct.ce.VoxyRenderSystem]: Voxy render system created with 536870912 geometry capacity, using pipeline 'NormalRenderPipeline' with renderer 'MDICSectionRenderer'
[12:37:18] [Render thread/INFO]: Started 1 worker threads
[12:37:18] [Render thread/INFO]: [me.cx.vy.cl.VoxyInstance]: Dedicated voxy thread pool size: 1
[12:37:18] [Render thread/WARN]: [me.cx.vy.cl.is.DHImporter]: Unable to load sqlite JDBC or lzma decompressor, DHImporting wont be available
[12:37:18] [Render thread/INFO]: [System] [CHAT] Stepan joined the game
[12:37:18] [Render thread/INFO]: Loaded 2 advancements
[12:37:18] [Render thread/INFO]: Stopping worker threads
[12:37:18] [Render thread/INFO]: Started 1 worker threads
[12:37:18] [Render thread/WARN]: Requested post effect does not exist: minecraft:end_of_frame
[12:37:19] [Render thread/INFO]: Resizing Dynamic Transforms UBO, capacity limit of 2 reached during a single frame. New capacity will be 4.
[12:37:20] [Render thread/INFO]: Resizing Dynamic Transforms UBO, capacity limit of 4 reached during a single frame. New capacity will be 8.
[12:37:21] [Render thread/INFO]: Resizing Dynamic Transforms UBO, capacity limit of 8 reached during a single frame. New capacity will be 16.
[12:37:22] [Render thread/INFO]: [System] [CHAT] Your game mode has been updated to Spectator Mode
```

## server.log tail
```
[12:36:37 INFO]: Paper: Using libdeflate (Linux x86_64) compression from Velocity.
[12:36:37 INFO]: Paper: Using OpenSSL 3.x.x (Linux x86_64) cipher from Velocity.
[12:36:37 WARN]: **** SERVER IS RUNNING IN OFFLINE/INSECURE MODE!
[12:36:37 WARN]: The server will make no attempt to authenticate usernames. Beware.
[12:36:37 WARN]: While this makes the game possible to play without internet access, it also opens up the ability for hackers to connect with any username they choose.
[12:36:37 WARN]: To change this, set "online-mode" to "true" in the server.properties file.
[12:36:37 INFO]: Preparing level "world"
[12:36:37 INFO]: Selecting spawn point for level 'minecraft:overworld'...
[12:36:41 INFO]: Selecting spawn point for level 'minecraft:the_nether'...
[12:36:42 INFO]: Selecting spawn point for level 'minecraft:the_end'...
[12:36:42 INFO]: Loading 0 persistent chunks for level 'minecraft:overworld'...
[12:36:42 INFO]: Preparing spawn area: 100%
[12:36:42 INFO]: Prepared spawn area in 5139 ms
[12:36:42 INFO]: Loading 0 persistent chunks for level 'minecraft:the_nether'...
[12:36:42 INFO]: Preparing spawn area: 100%
[12:36:42 INFO]: Prepared spawn area in 999 ms
[12:36:42 INFO]: Loading 0 persistent chunks for level 'minecraft:the_end'...
[12:36:42 INFO]: Preparing spawn area: 100%
[12:36:42 INFO]: Prepared spawn area in 273 ms
[12:36:42 INFO]: Done preparing level "world" (5.414s)
[12:36:43 INFO]: [spark] Starting background profiler...
[12:36:43 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:overworld
[12:36:43 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_nether
[12:36:43 INFO]: Saving chunks for level 'ServerLevel[world]'/minecraft:the_end
[12:36:43 INFO]: Running delayed init tasks
[12:36:43 INFO]: Done (17.096s)! For help, type "help"
[12:36:43 INFO]: *************************************************************************************
[12:36:43 INFO]: This is the first time you're starting this server.
[12:36:43 INFO]: It's recommended you read our 'Getting Started' documentation for guidance.
[12:36:43 INFO]: View this and more helpful information here: https://docs.papermc.io/paper/next-steps
[12:36:43 INFO]: *************************************************************************************
[12:37:09 INFO]: UUID of player Stepan is d1652ad9-77ce-3200-9642-f973c97adf36
[12:37:16 INFO]: System chat: Stepan joined the game
[12:37:16 INFO]: Stepan[/127.0.0.1:52624] logged in with entity id 7 at ([minecraft:overworld]-335.5, 79.0, -118.5)
[12:37:21 INFO]: System chat: Incorrect argument for command
gamerule doDaylightCycle false<--[HERE]
[12:37:21 INFO]: System chat: Set minecraft:overworld to 6000 tick(s)
[12:37:21 INFO]: System chat: Set the weather to clear
[12:37:21 INFO]: System chat: Set Stepan's game mode to Spectator Mode
[12:41:23 INFO]: System chat: Teleported Stepan to -929.500000, 140.000000, 796.500000
```

## voxy-leg.log tail (the leg's own stdout/stderr)
```
fetching Paper 26.3 (no local jar found)
api ok, downloading paper-26.3-244.jar
voxy reference server up (pid 3879, seed clodbench2026)
voxy client pid 4053 (xvfb 4033)
voxy client joined
waiting for voxy meshing...
	at knot//me.cortex.voxy.client.core.VoxyRenderSystem.<init>(VoxyRenderSystem.java:147)
	at knot//net.minecraft.client.renderer.LevelRenderer.voxy$createEngineDirect(LevelRenderer.java:2905)
	at knot//net.minecraft.client.renderer.LevelRenderer.voxy$createRenderer(LevelRenderer.java:2897)
	at knot//net.minecraft.client.renderer.extract.LevelExtractor.handler$bie000$voxy$reload(LevelExtractor.java:3253)
[12:37:17] [Render thread/INFO]: [me.cx.vy.ct.ce.VoxyRenderSystem]: Voxy render system created with 536870912 geometry capacity, using pipeline 'NormalRenderPipeline' with renderer 'MDICSectionRenderer'
[12:37:18] [Render thread/INFO]: [me.cx.vy.cl.VoxyInstance]: Dedicated voxy thread pool size: 1
```
