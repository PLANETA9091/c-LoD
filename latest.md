> 🎭 **NEW — v6.18.2 THE ENGAGEMENT MASK (run #287 verdict + пуш 91321fd):** вторая полоса РАБОТАЕТ — 07_far_north восстановился до **99.47% exact** (mean 0.81), наши кадры между ранами изменились в 2.4× сильнее voxy-х (канал армится на позах снимков). Но полоса тумана на 01-10 стоит, а 08 подрос (stall-кадр llvmpipe — ванильный лес исчез из нашего кадра, march дорисовал купол). Вместо гадания — ИНСТРУМЕНТ: `unfog_debug=true` шипит МАСКУ СРАБАТЫВАНИЯ (GREEN = замена тумана применилась / RED = ванильный туман сатурация / BLUE = канал слеп / DIM = туманы согласны) на §5b-лане (линза + маска в одном клиенте, цепочка debug_unfog). Плюс ЧЕСТНЫЙ FALLBACK: безканальная пара марша = реальные дефолты overworld из client.jar (0 / 1024) вместо догадки pre-v4.2 (67 / 768) — до-канальный туман марша теперь байт-в-байт формула voxy. Ожидание #288: маски покажут, ГДЕ замена стреляет.

> 🔁 **NEW — v6.18.1 THE SECOND STRIP (run #286 verdict: `run-42466249389-verdict-unfog.md` + фид `gallery/parity-run-286/`):** «емае, продолжи!» — v6.18 UN-FOG не стрелял: парити-вантажи фланкируют кадр горами, левые 16 колонн камерного канала — террейн на каждой строке, канал слеп на всех измеряемых парах (01-10 не сдвинулись, белый туман на месте). Доказательство от противного: 07_far_north — единственный вантаж с небом слева — единственный живой; его 100.000 → 85.386 = ПРОГРЕСС (settle-выравнивание дало voxy домерешить дальний лес — пара теперь несёт честный гэп march-vs-mesh, а не вакуум). 11_topdown: контроль пыли ЧИСТ (колонн нет; остаток = класс «вода» → G8c). Фикс v6.18.1: те же 16 байт пейлоада второй полосой на фиксированных колоннах 632..647 (верх-центр = небо на каждой позе снимка), col_scan валидирует обе полосы, rot_solve читает через strip_base(), без новых UBO (sky-программа не биндит Globals — класс #232). Локально: 96/96 тестов, клLPy, fmt, гейты. Ожидание #287: 01-10 прыгают, когда замена тумана наконец стреляет.

### c-LoD visual run (-930,796, marker=red_concrete, vantage -1005,150 yaw 270)
- GL:     Vendor: Mesa (0xffffffff);    Device: llvmpipe (LLVM 20.1.2, 256 bits) (0xffffffff);OpenGL renderer string: llvmpipe (LLVM 20.1.2, 256 bits);OpenGL core profile version string: 4.5 (Core Profile) Mesa 25.2.8-0ubuntu0.24.04.4;
- boot + gates OK (09:41:26)
- sky-grid armed
- joined session 1 (09:41:53)
- stream live: cells=65536
- terrain range: 2 terraced mountains at (CX-4,CZ-20)/(CX-2,CZ+16), tops 185/190, grass terraces + stone caps, 66..82 m from the eye (inside the 83.5 m band, chunk edge 64 m)
- scan complete=1 (cells=cells=65536/65536)
- paint settled=1 (wire 54193p)
- baseline try 1: drift=0 px
- lod_silhouette_cols=0 (LOD paint ON SCREEN at 75 m if > 20 — the march-drawn marker column; vanilla cannot render there, chunk edge 64 m; #255 re-measures 49, the ZONE_FAR=48 regression #256 and the control read 0)
- 0 terrain-green px in the sky region (LOD MOUNTAINS on screen if > 5000; the range paints ColLow-green on low terraces via the fallback palette; flat-world baselines and the pack-free control read ~0)
- burst_close white px: 530 (renderer alive if > 50)
- burst_far white px: 343 (renderer alive in the marker zone if > 20)
- cut walk 40 m: 70 white px (> 20 = rendered)
- cut walk 60 m: 3 white px (> 20 = rendered)
- cut walk 100 m: 0 white px (> 20 = rendered)
- measured far-render cut: 40 m (G4 flap instrument; the decode proof needs >= 83 m from the y=151.6 eye)
- fill issued: 5x111x5 red_concrete at -930,796 (zone forceloaded ±48)

### /clod-cells AFTER FILL (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 1149 0 0x8400 0x3100
123 122 -940 784 71 0 1 [97, 119, 97] 1149 0 0x8700 0x3100
124 122 -938 784 68 0 1 [97, 119, 97] 1149 0 0x8400 0x3100
125 122 -936 784 62 1 1 [63, 118, 228] 1149 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 1149 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 1142 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 1141 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 1141 0 0x8000 0x5c00
130 122 -926 784 66 0 1 [134, 183, 131] 1140 0 0x8200 0x5c00
131 122 -924 784 75 0 1 [97, 119, 97] 1139 0 0x8b00 0x3100
132 122 -922 784 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 122 -920 784 75 0 1 [97, 119, 97] 18446744073709551615 1 0x8b00 0x3100
134 122 -918 784 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x3100
122 123 -942 786 70 0 1 [97, 119, 97] 1149 0 0x8600 0x3100
123 123 -940 786 65 0 1 [97, 119, 97] 1149 0 0x8100 0x3100
124 123 -938 786 65 0 1 [97, 119, 97] 1149 0 0x8100 0x3100
125 123 -936 786 62 1 1 [63, 118, 228] 1149 0 0x7e00 0xd800
```
### stream diag after fill:
[09:44:47 INFO]: [c-LoD] stream: tick 1900 65536/65536 cells, lap 604, cone 5323 zone 650 dirty 787 (q 11666) ewin 253 rbeat 0p/0 rearm 0c/0t rescan 402367c/1824e, edits +4376p/3287c/568e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 317 forced 195, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 54607p/135b fail 0 ~0 kB/s cache 11448/24576 h43159 m11448 e0
[c-LoD] stream: tick 1900 65536/65536 cells, lap 604, cone 5323 zone 650 dirty 787 (q 11666) ewin 253 rbeat 0p/0 rearm 0c/0t rescan 402367c/1824e, edits +4376p/3287c/568e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 317 forced 195, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 54607p/135b fail 0 ~0 kB/s cache 11448/24576 h43159 m11448 e0
- probe 0: red=0 paint=0 lod_silhouette_cols=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 1: red=0 paint=0 lod_silhouette_cols=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 2: red=0 paint=0 lod_silhouette_cols=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 3: red=0 paint=384 lod_silhouette_cols=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=4 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 4: red=0 paint=422 lod_silhouette_cols=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=198114 lens_zero=0 rot=decoded cam_x_b=? cam_z_b=? az_b=? el_b=? strobe_hits=22 mask_hits=22 (need red>=60 paint>=300 lod>20)
- probe 5: red=0 paint=384 lod_silhouette_cols=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=7 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 6: red=0 paint=384 lod_silhouette_cols=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=3 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 7: red=0 paint=192 lod_silhouette_cols=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 8: red=0 paint=0 lod_silhouette_cols=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 9: red=0 paint=0 lod_silhouette_cols=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 10: red=0 paint=0 lod_silhouette_cols=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 11: red=0 paint=0 lod_silhouette_cols=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 12: red=0 paint=0 lod_silhouette_cols=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 13: red=0 paint=0 lod_silhouette_cols=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 14: red=0 paint=0 lod_silhouette_cols=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 15: red=0 paint=0 lod_silhouette_cols=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 16: red=0 paint=0 lod_silhouette_cols=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 17: red=0 paint=0 lod_silhouette_cols=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 18: red=0 paint=384 lod_silhouette_cols=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=15 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 19: red=0 paint=192 lod_silhouette_cols=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=2 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 20: red=0 paint=418 lod_silhouette_cols=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=198114 lens_zero=0 rot=decoded cam_x_b=? cam_z_b=? az_b=? el_b=? strobe_hits=22 mask_hits=22 (need red>=60 paint>=300 lod>20)
- probe 21: red=0 paint=384 lod_silhouette_cols=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=11 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 22: red=0 paint=192 lod_silhouette_cols=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=1 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 23: red=0 paint=425 lod_silhouette_cols=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=? cam_z_b=? az_b=? el_b=? strobe_hits=22 mask_hits=22 (need red>=60 paint>=300 lod>20)
### stream diag after probes:
[09:47:57 INFO]: [c-LoD] stream: tick 3800 65536/65536 cells, lap 1324, cone 22038 zone 1930 dirty 806 (q 11678) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1015433c/1880e, edits +5885p/4355c/875e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 317 forced 195, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 198023p/290b fail 0 ~101 kB/s cache 11508/24576 h186515 m11508 e0
[c-LoD] stream: tick 3900 65536/65536 cells, lap 1424, cone 24708 zone 1930 dirty 806 (q 11678) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1048518c/1880e, edits +5885p/4355c/875e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 317 forced 195, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 219279p/312b fail 0 ~112 kB/s cache 11508/24576 h207771 m11508 e0
[09:48:07 INFO]: [c-LoD] stream: tick 3900 65536/65536 cells, lap 1424, cone 24708 zone 1930 dirty 806 (q 11678) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1048518c/1880e, edits +5885p/4355c/875e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 317 forced 195, native 65536c/1089ch fail 0 (3w q 0) helper on, trees 1089ch/fail 0, wire bundle 219279p/312b fail 0 ~112 kB/s cache 11508/24576 h207771 m11508 e0
- parity debug tour: 11 vantages (session-1 client, DEBUG pack — evidence only, the gate pairs the §7b world session)
- parity debug tour done: 11 shots in parity_dbg/ (the §7b world session feeds the gate)
- joined session 2 (09:50:40)
- leg-2 baseline scene px (x<940): 3 (0 = the relog client paints no world at all)
- leg-2 renderer probe NEAR (20 m) white px: 478 raw / 478 changed-vs-baseline (> 20 = the client renders particles)
- leg-2 renderer probe FAR (80 m, marker zone) white px: 329 raw / 329 changed-vs-baseline (> 20 = no distance cut; ~0 with NEAR alive = particles beyond the client chunk radius do not render)
- leg-2 carrier probe green dust (80 m): changed-green px 0 / total changed 13 vs baseline (> 100 green = a stable green carrier exists; total > 0 with green ~0 = the dust renders but fog washes it — a ruler problem, not a carrier problem)
- relog probe 0: red=0 scene=26 lens_red=0 lens_black=0 lens_water=0 lens_dark=198114 lens_zero=0 rot=decoded cam_x_b=? cam_z_b=? az_b=? el_b=? strobe_hits=22 mask_hits=22 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 1: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=7 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 2: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 3: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=19 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 4: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=3 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 5: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=2 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 6: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=19 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 7: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=2 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 8: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=3 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 9: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=7 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 10: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=1 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 11: red=0 scene=31 lens_red=0 lens_black=0 lens_water=0 lens_dark=198114 lens_zero=0 rot=decoded cam_x_b=? cam_z_b=? az_b=? el_b=? strobe_hits=22 mask_hits=22 (need red >= 60; scene > 0 = the client paints a world at all)
- **FAIL:** marker lost after relog — stream state is not persistent/deterministic

### client log forensics (pipeline/particle/atlas — v6.8.2)
```
ALSA lib confmisc.c:1342:(snd_func_refer) error evaluating name
ALSA lib conf.c:5208:(_snd_config_evaluate) function snd_func_refer returned error: No such file or directory
ALSA lib conf.c:5731:(snd_config_expand) Evaluate error: No such file or directory
[09:50:37] [Render thread/ERROR]: Error starting SoundSystem. Turning off sounds & music
java.lang.IllegalStateException: Failed to open OpenAL device
	at net.minecraft.client.sounds.SoundEngine.reload(SoundEngine.java:110)
	at net.minecraft.server.packs.resources.SimplePreparableReloadListener.lambda$reload$1(SimplePreparableReloadListener.java:15)
	at net.minecraft.server.packs.resources.SimpleReloadInstance.lambda$prepareTasks$3(SimpleReloadInstance.java:59)
[09:50:37] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[09:50:37] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[09:50:37] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[09:50:37] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[09:50:37] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[09:50:37] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[09:50:37] [Render thread/INFO]: Created: 2048x2048x4 minecraft:textures/atlas/blocks.png-atlas
[09:50:37] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[09:50:37] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[09:50:37] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[09:50:37] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[09:50:37] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
[09:50:44] [Render thread/INFO]: Reloading ResourceManager: vanilla, vanilla, server/00000000/6b1f2f7e-9c0d-4a4b-8b0a-b10d1b4ab100
ALSA lib conf.c:5208:(_snd_config_evaluate) function snd_func_card_inum returned error: No such file or directory
ALSA lib confmisc.c:422:(snd_func_concat) error evaluating strings
ALSA lib conf.c:5208:(_snd_config_evaluate) function snd_func_concat returned error: No such file or directory
ALSA lib confmisc.c:1342:(snd_func_refer) error evaluating name
ALSA lib conf.c:5208:(_snd_config_evaluate) function snd_func_refer returned error: No such file or directory
ALSA lib conf.c:5731:(snd_config_expand) Evaluate error: No such file or directory
ALSA lib conf.c:5208:(_snd_config_evaluate) function snd_func_card_inum returned error: No such file or directory
ALSA lib confmisc.c:422:(snd_func_concat) error evaluating strings
ALSA lib conf.c:5208:(_snd_config_evaluate) function snd_func_concat returned error: No such file or directory
ALSA lib confmisc.c:1342:(snd_func_refer) error evaluating name
ALSA lib conf.c:5208:(_snd_config_evaluate) function snd_func_refer returned error: No such file or directory
ALSA lib conf.c:5731:(snd_config_expand) Evaluate error: No such file or directory
[09:50:47] [Render thread/ERROR]: Error starting SoundSystem. Turning off sounds & music
java.lang.IllegalStateException: Failed to open OpenAL device
	at net.minecraft.client.sounds.SoundEngine.reload(SoundEngine.java:110)
	at net.minecraft.server.packs.resources.SimplePreparableReloadListener.lambda$reload$1(SimplePreparableReloadListener.java:15)
	at net.minecraft.server.packs.resources.SimpleReloadInstance.lambda$prepareTasks$3(SimpleReloadInstance.java:59)
[09:50:47] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[09:50:47] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[09:50:47] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[09:50:47] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[09:50:47] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[09:50:47] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[09:50:47] [Render thread/INFO]: Created: 2048x2048x4 minecraft:textures/atlas/blocks.png-atlas
[09:50:47] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[09:50:47] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[09:50:47] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[09:50:47] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[09:50:47] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
```

### /clod-cells AFTER PROBES (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 16 0 0x8400 0x3100
123 122 -940 784 71 0 1 [97, 119, 97] 16 0 0x8700 0x3100
124 122 -938 784 68 0 1 [97, 119, 97] 15 0 0x8400 0x3100
125 122 -936 784 62 1 1 [63, 118, 228] 15 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 14 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 14 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 13 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 13 0 0x8000 0x5c00
130 122 -926 784 66 0 1 [134, 183, 131] 12 0 0x8200 0x5c00
131 122 -924 784 75 0 1 [97, 119, 97] 11 0 0x8b00 0x3100
132 122 -922 784 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 122 -920 784 75 0 1 [97, 119, 97] 18446744073709551615 1 0x8b00 0x3100
134 122 -918 784 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x3100
122 123 -942 786 70 0 1 [97, 119, 97] 17 0 0x8600 0x3100
123 123 -940 786 65 0 1 [97, 119, 97] 16 0 0x8100 0x3100
124 123 -938 786 65 0 1 [97, 119, 97] 16 0 0x8100 0x3100
125 123 -936 786 62 1 1 [63, 118, 228] 15 0 0x7e00 0xd800
```
- edit channel: 0 linkage errors, 0 failed event dispatches (both must be 0)
- parity world: view-distance=8 (the voxy leg's server value)
- parity world: joined session 3 (09:55:57)
- parity world: scan populated (cells=65536; the march paints from these buffers)
- parity world tour: 11 vantages (production pack, vd 8, shared structures)
- parity world tour done: 11 shots in parity/ (the gate pairs voxy/<name>.png vs parity/<name>.png)
