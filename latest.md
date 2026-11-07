> ⚡ **NEW — THE PARITY WORLD (`run-42411779934-verdict-parity-world.md` + `gallery/sheet-I-parity-jump.png` + CI-фид `gallery/parity-run-285/`):** «я предпологаю что щяс паритет вообще 0% хах:) но короче продолжай» — угадал (baseline #281: exact 0.002–0.27%), и этот раунд прыгнул: **exact 62.5–94.8% на всех 10 парах, 07_far_north = 100.000% exact — до зелёного гейта 3 пикселя** (±1 в градиенте неба). v6.17 снял три класса дифа из триптихов #281: (1) наши панели (правые 27% кадра) — parity-сессия §7b работает на ПРОИЗВОДСТВЕННОМ паке (debug_lens=false, без линз-пасса и панелей); (2) горизонт данных (наш vd 4 против voxy vd 8) — §7b седит view-distance 8; (3) РАЗНЫЕ МИРЫ — voxy-лег теперь строит те же горы+маркер до meshing-а. Плюс mipmap 0→4 и particles-pin симметрия. Остаток ОДИН и он опознан: **ванильный туман на дальнем терре** (voxy-меш рисует чётко) — v6.18 UN-FOG: инверсия mix(color, fogColor, f) по depth-буферу в пост-пассе пака. Гейт честно красный (так заказано), фид опубликован, регрессия v6.16.6 (стирание ветки) НЕ повторилась — дерево цело.

> 🚦 **NEW — THE PARITY GATE (`run-42404167317-verdict-parity.md` + `gallery/sheet-H-паритет-гейт.png` + CI-фид `gallery/parity-run-281/`):** «сделай gallery в github ci — voxy и наш клиент снимают одни и те же координаты, потом pixel perfect всех кадров, если не 100% такое же — CI красный» — ГОТОВО и РАБОТАЕТ: общий список точек (parity_vantages.txt), оба лега снимают его, parity_diff.py парит кадры по именам и меряет exact%/near%/среднюю дельту, CI-джоб parity-gate КРАСНЫЙ пока < 100% (так и заказано), фид публикуется на эту ветку каждый ран (первый коммит e33758b). 4 рана (#275/#276/#278/#281) поймали и вылечили 4 бага самого инструмента (ffmpeg ел stdin матрицы и портил имена; окно voxy 640×360 против кадра 1280×720; 403 на пуш фида; time set 6000 = НОЧЬ в 26.3 + часы утекают сквозь doDaylightCycle false — теперь noon пинится перед КАЖДЫМ снимком в обоих легах). Честная базовая линия: 07_far_north 70.9% near (пустое небо = контроль честности), **10_marker_ray — целевая пара: voxy рисует настоящий полуденный ландшафт (лес + озеро), наш клиент — небо + пыль + панели**; очередь сходимости в G8 roadmap (paint → невидимость пыли → линза → лук).

> 🔧 **NEW — THE TERRAIN RANGE (`run-42393812250-verdict-terrain.md` + `gallery/sheet-G-горы-и-флап.png`):** «это не лоды, а я прошу о лоды» — ответ: ветка feat/visible-terrain строит НАСТОЯЩИЙ дальний рельеф — две террасные горы на 66..82 м (за краем чанков 64 м, внутри декод-полосы 83.5 м), сканер стримит их высоты (grass rgb [134,183,131] в clod-cells), fallback-палитра красит ландшафтно. Блокер — флап capture-коммита (класс автора v6.10.0, +3 рана подряд): точка зрения y=150 оправдана, глаза не виноваты. CUT WALK v6.15.2 замерил рендер внутри рана: 20м 577px / 40м 77px / 60м 2px / 75м 356px / 100м 0px — «кат» НЕ монотонный, рендер и capture — разные флапающие домены. Следующий шаг — G7c capture-retry (rejoin при пустом capture до проб).

### c-LoD visual run (-930,796, marker=red_concrete, vantage -1005,150 yaw 270)
- GL:     Vendor: Mesa (0xffffffff);    Device: llvmpipe (LLVM 20.1.2, 256 bits) (0xffffffff);OpenGL renderer string: llvmpipe (LLVM 20.1.2, 256 bits);OpenGL core profile version string: 4.5 (Core Profile) Mesa 25.2.8-0ubuntu0.24.04.4;
- boot + gates OK (20:37:10)
- sky-grid armed
- joined session 1 (20:37:36)
- stream live: cells=65536
- terrain range: 2 terraced mountains at (CX-4,CZ-20)/(CX-2,CZ+16), tops 185/190, grass terraces + stone caps, 66..82 m from the eye (inside the 83.5 m band, chunk edge 64 m)
- scan complete=1 (cells=cells=65536/65536)
- paint settled=1 (wire 43671p)
- baseline try 1: drift=0 px
- lod_silhouette_cols=0 (LOD paint ON SCREEN at 75 m if > 20 — the march-drawn marker column; vanilla cannot render there, chunk edge 64 m; #255 re-measures 49, the ZONE_FAR=48 regression #256 and the control read 0)
- 0 terrain-green px in the sky region (LOD MOUNTAINS on screen if > 5000; the range paints ColLow-green on low terraces via the fallback palette; flat-world baselines and the pack-free control read ~0)
- burst_close white px: 498 (renderer alive if > 50)
- burst_far white px: 311 (renderer alive in the marker zone if > 20)
- cut walk 40 m: 71 white px (> 20 = rendered)
- cut walk 60 m: 0 white px (> 20 = rendered)
- cut walk 100 m: 0 white px (> 20 = rendered)
- measured far-render cut: 40 m (G4 flap instrument; the decode proof needs >= 83 m from the y=151.6 eye)
- fill issued: 5x111x5 red_concrete at -930,796 (zone forceloaded ±48)

### /clod-cells AFTER FILL (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 1188 0 0x8400 0x3100
123 122 -940 784 71 0 1 [97, 119, 97] 1188 0 0x8700 0x3100
124 122 -938 784 68 0 1 [97, 119, 97] 1188 0 0x8400 0x3100
125 122 -936 784 62 1 1 [63, 118, 228] 1184 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 1183 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 1183 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 1182 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 1182 0 0x8000 0x5c00
130 122 -926 784 66 0 1 [134, 183, 131] 1181 0 0x8200 0x5c00
131 122 -924 784 75 0 1 [97, 119, 97] 1180 0 0x8b00 0x3100
132 122 -922 784 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 122 -920 784 75 0 1 [97, 119, 97] 18446744073709551615 1 0x8b00 0x3100
134 122 -918 784 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x3100
122 123 -942 786 70 0 1 [97, 119, 97] 1185 0 0x8600 0x3100
123 123 -940 786 65 0 1 [97, 119, 97] 1185 0 0x8100 0x3100
124 123 -938 786 65 0 1 [97, 119, 97] 1184 0 0x8100 0x3100
125 123 -936 786 62 1 1 [63, 118, 228] 1184 0 0x7e00 0xd800
```
### stream diag after fill:
[c-LoD] stream: tick 1900 65536/65536 cells, lap 595, cone 4042 zone 646 dirty 749 (q 12692) ewin 245 rbeat 0p/0 rearm 0c/0t rescan 397807c/2364e, edits +5568p/4200c/559e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 304 forced 208, native 65536c/1091ch fail 0 (3w q 0) helper on, trees 1091ch/fail 0, wire bundle 44049p/117b fail 0 ~0 kB/s cache 11418/24576 h32631 m11418 e0
[20:40:30 INFO]: [c-LoD] stream: tick 1900 65536/65536 cells, lap 595, cone 4042 zone 646 dirty 749 (q 12692) ewin 245 rbeat 0p/0 rearm 0c/0t rescan 397807c/2364e, edits +5568p/4200c/559e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 304 forced 208, native 65536c/1091ch fail 0 (3w q 0) helper on, trees 1091ch/fail 0, wire bundle 44049p/117b fail 0 ~0 kB/s cache 11418/24576 h32631 m11418 e0
- probe 0: red=0 paint=0 lod_silhouette_cols=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 1: red=0 paint=0 lod_silhouette_cols=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 2: red=0 paint=0 lod_silhouette_cols=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 3: red=0 paint=0 lod_silhouette_cols=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 4: red=0 paint=0 lod_silhouette_cols=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 5: red=0 paint=0 lod_silhouette_cols=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 6: red=0 paint=0 lod_silhouette_cols=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 7: red=0 paint=0 lod_silhouette_cols=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=0 (need red>=60 paint>=300 lod>20)
- probe 8: red=0 paint=384 lod_silhouette_cols=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=16 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 9: red=0 paint=384 lod_silhouette_cols=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=3 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 10: red=0 paint=192 lod_silhouette_cols=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=1 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 11: red=0 paint=384 lod_silhouette_cols=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=21 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 12: red=0 paint=384 lod_silhouette_cols=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=3 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 13: red=0 paint=384 lod_silhouette_cols=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=5 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 14: red=0 paint=384 lod_silhouette_cols=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=9 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 15: red=0 paint=384 lod_silhouette_cols=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=4 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 16: red=0 paint=395 lod_silhouette_cols=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=198114 lens_zero=0 rot=decoded cam_x_b=? cam_z_b=? az_b=? el_b=? strobe_hits=22 mask_hits=22 (need red>=60 paint>=300 lod>20)
- probe 17: red=0 paint=384 lod_silhouette_cols=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=7 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 18: red=0 paint=192 lod_silhouette_cols=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=1 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 19: red=0 paint=384 lod_silhouette_cols=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=22 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 20: red=0 paint=384 lod_silhouette_cols=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=4 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 21: red=0 paint=192 lod_silhouette_cols=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=0 mask_hits=30 (need red>=60 paint>=300 lod>20)
- probe 22: red=0 paint=385 lod_silhouette_cols=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=198114 lens_zero=0 rot=decoded cam_x_b=? cam_z_b=? az_b=? el_b=? strobe_hits=22 mask_hits=22 (need red>=60 paint>=300 lod>20)
- probe 23: red=0 paint=384 lod_silhouette_cols=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=5 mask_hits=30 (need red>=60 paint>=300 lod>20)
### stream diag after probes:
[20:43:41 INFO]: [c-LoD] stream: tick 3800 65536/65536 cells, lap 1787, cone 30102 zone 1286 dirty 773 (q 12717) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1010612c/2437e, edits +7195p/5361c/935e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 304 forced 208, native 65536c/1091ch fail 0 (3w q 0) helper on, trees 1091ch/fail 0, wire bundle 256885p/341b fail 0 ~56 kB/s cache 11479/24576 h245406 m11479 e0
[c-LoD] stream: tick 3900 65536/65536 cells, lap 1887, cone 32772 zone 1286 dirty 773 (q 12717) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1043936c/2437e, edits +7195p/5361c/935e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 304 forced 208, native 65536c/1091ch fail 0 (3w q 0) helper on, trees 1091ch/fail 0, wire bundle 278147p/363b fail 0 ~112 kB/s cache 11479/24576 h266668 m11479 e0
[20:43:51 INFO]: [c-LoD] stream: tick 3900 65536/65536 cells, lap 1887, cone 32772 zone 1286 dirty 773 (q 12717) ewin 0 rbeat 0p/0 rearm 0c/0t rescan 1043936c/2437e, edits +7195p/5361c/935e drop 0, spawned 0 / failed 0, cam0 yaw -90 pitch 5, wish 304 forced 208, native 65536c/1091ch fail 0 (3w q 0) helper on, trees 1091ch/fail 0, wire bundle 278147p/363b fail 0 ~112 kB/s cache 11479/24576 h266668 m11479 e0
- parity debug tour: 10 vantages (session-1 client, DEBUG pack — evidence only, the gate pairs the §7b world session)
- parity debug tour done: 10 shots in parity_dbg/ (the §7b world session feeds the gate)
- joined session 2 (20:46:12)
- leg-2 baseline scene px (x<940): 0 (0 = the relog client paints no world at all)
- leg-2 renderer probe NEAR (20 m) white px: 518 raw / 518 changed-vs-baseline (> 20 = the client renders particles)
- leg-2 renderer probe FAR (80 m, marker zone) white px: 319 raw / 318 changed-vs-baseline (> 20 = no distance cut; ~0 with NEAR alive = particles beyond the client chunk radius do not render)
- leg-2 carrier probe green dust (80 m): changed-green px 0 / total changed 7 vs baseline (> 100 green = a stable green carrier exists; total > 0 with green ~0 = the dust renders but fog washes it — a ruler problem, not a carrier problem)
- LEG-2 SCENE DEAD: the relog client paints ZERO scene pixels (x<940) — pure sky + lens panels; the fresh-session capture chain never commits (renderer-dead class, #227/#228); these probes measure a worldless client, not the transport
- relog probe 0: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=4 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 1: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=6 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 2: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=9 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 3: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=2 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 4: red=0 scene=20 lens_red=0 lens_black=0 lens_water=0 lens_dark=198114 lens_zero=0 rot=decoded cam_x_b=? cam_z_b=? az_b=? el_b=? strobe_hits=22 mask_hits=22 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 5: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=2 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 6: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=5 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 7: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=18 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 8: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=11 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 9: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=6 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 10: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=13 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- relog probe 11: red=0 scene=0 lens_red=0 lens_black=0 lens_water=0 lens_dark=197796 lens_zero=0 rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3 strobe_hits=2 mask_hits=30 (need red >= 60; scene > 0 = the client paints a world at all)
- **FAIL:** marker lost after relog — stream state is not persistent/deterministic

### client log forensics (pipeline/particle/atlas — v6.8.2)
```
ALSA lib confmisc.c:1342:(snd_func_refer) error evaluating name
ALSA lib conf.c:5208:(_snd_config_evaluate) function snd_func_refer returned error: No such file or directory
ALSA lib conf.c:5731:(snd_config_expand) Evaluate error: No such file or directory
[20:46:10] [Render thread/ERROR]: Error starting SoundSystem. Turning off sounds & music
java.lang.IllegalStateException: Failed to open OpenAL device
	at net.minecraft.client.sounds.SoundEngine.reload(SoundEngine.java:110)
	at net.minecraft.server.packs.resources.SimplePreparableReloadListener.lambda$reload$1(SimplePreparableReloadListener.java:15)
	at net.minecraft.server.packs.resources.SimpleReloadInstance.lambda$prepareTasks$3(SimpleReloadInstance.java:59)
[20:46:10] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[20:46:10] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[20:46:10] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[20:46:10] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[20:46:10] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[20:46:10] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[20:46:10] [Render thread/INFO]: Created: 2048x2048x4 minecraft:textures/atlas/blocks.png-atlas
[20:46:10] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[20:46:10] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[20:46:10] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[20:46:10] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[20:46:10] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
[20:46:17] [Render thread/INFO]: Reloading ResourceManager: vanilla, vanilla, server/00000000/6b1f2f7e-9c0d-4a4b-8b0a-b10d1b4ab100
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
[20:46:19] [Render thread/ERROR]: Error starting SoundSystem. Turning off sounds & music
java.lang.IllegalStateException: Failed to open OpenAL device
	at net.minecraft.client.sounds.SoundEngine.reload(SoundEngine.java:110)
	at net.minecraft.server.packs.resources.SimplePreparableReloadListener.lambda$reload$1(SimplePreparableReloadListener.java:15)
	at net.minecraft.server.packs.resources.SimpleReloadInstance.lambda$prepareTasks$3(SimpleReloadInstance.java:59)
[20:46:19] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas
[20:46:19] [Render thread/INFO]: Created: 256x128x0 minecraft:textures/atlas/celestials.png-atlas
[20:46:19] [Render thread/INFO]: Created: 128x128x0 minecraft:textures/atlas/decorated_pot.png-atlas
[20:46:19] [Render thread/INFO]: Created: 512x256x0 minecraft:textures/atlas/paintings.png-atlas
[20:46:19] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/shield_patterns.png-atlas
[20:46:19] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/banner_patterns.png-atlas
[20:46:19] [Render thread/INFO]: Created: 2048x2048x4 minecraft:textures/atlas/blocks.png-atlas
[20:46:19] [Render thread/INFO]: Created: 1024x512x0 minecraft:textures/atlas/items.png-atlas
[20:46:19] [Render thread/INFO]: Created: 1024x1024x0 minecraft:textures/atlas/gui.png-atlas
[20:46:19] [Render thread/INFO]: Created: 128x64x0 minecraft:textures/atlas/map_decorations.png-atlas
[20:46:19] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/chest.png-atlas
[20:46:19] [Render thread/INFO]: Created: 512x512x0 minecraft:textures/atlas/shulker_boxes.png-atlas
```

### /clod-cells AFTER PROBES (server belief)
```
# cx cz wx wz height water valid rgb last_age dirty hword cword
122 122 -942 784 68 0 1 [97, 119, 97] 38 0 0x8400 0x3100
123 122 -940 784 71 0 1 [97, 119, 97] 37 0 0x8700 0x3100
124 122 -938 784 68 0 1 [97, 119, 97] 37 0 0x8400 0x3100
125 122 -936 784 62 1 1 [63, 118, 228] 37 0 0x7e00 0xd800
126 122 -934 784 62 1 1 [63, 118, 228] 36 0 0x7e00 0xd800
127 122 -932 784 62 1 1 [63, 118, 228] 36 0 0x7e00 0xd800
128 122 -930 784 62 1 1 [63, 118, 228] 35 0 0x7e00 0xd800
129 122 -928 784 64 0 1 [134, 183, 131] 34 0 0x8000 0x5c00
130 122 -926 784 66 0 1 [134, 183, 131] 34 0 0x8200 0x5c00
131 122 -924 784 75 0 1 [97, 119, 97] 33 0 0x8b00 0x3100
132 122 -922 784 77 0 1 [97, 119, 97] 18446744073709551615 1 0x8d00 0x3100
133 122 -920 784 75 0 1 [97, 119, 97] 18446744073709551615 1 0x8b00 0x3100
134 122 -918 784 73 0 1 [97, 119, 97] 18446744073709551615 1 0x8900 0x3100
122 123 -942 786 70 0 1 [97, 119, 97] 38 0 0x8600 0x3100
123 123 -940 786 65 0 1 [97, 119, 97] 37 0 0x8100 0x3100
124 123 -938 786 65 0 1 [97, 119, 97] 37 0 0x8100 0x3100
125 123 -936 786 62 1 1 [63, 118, 228] 37 0 0x7e00 0xd800
```
- edit channel: 0 linkage errors, 0 failed event dispatches (both must be 0)
- parity world: view-distance=8 (the voxy leg's server value)
- parity world: joined session 3 (20:51:31)
- parity world: scan populated (cells=65536; the march paints from these buffers)
- parity world tour: 10 vantages (production pack, vd 8, shared structures)
- parity world tour done: 10 shots in parity/ (the gate pairs voxy/<name>.png vs parity/<name>.png)
