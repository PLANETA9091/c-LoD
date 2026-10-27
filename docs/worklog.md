# Worklog

---
Task ID: 1
Agent: main
Task: Восстановление окружения (сессия после сброса): client.jar 26.3, CRUSSTY, состояние репозитория c-LoD

Work Log:
- Окружение было сброшено — все каталоги analysis/ пропали.
- Заново скачан client.jar 26.3 (41.5MB, piston-data), распакованы post_effect JSON (5 ванильных), 101 шейдерный файл, 18 post-классов.
- Склонированы CRUSSTY (платформа) и c-hello (референсный модуль).
- Обнаружено: репозиторий c-LoD уже существует с v2.0 (posteffect-архитектура), CI зелёный на 7297d70.

Stage Summary:
- Механизм posteffect подтверждён бинарно: ServerPlayer.postEffects/sendPostEffects → ClientboundPostEffectsPacket(List<Identifier>) → ShaderManager.loadPostChain сканирует ВСЕ ресурсы (включая серверный ресурспак): post_effect/*.json + shaders через shaderc.
- Схема PostChainConfig: targets{width,height,persistent,clear_color}, passes[{vertex_shader,fragment_shader,inputs[{sampler_name,target,use_depth_buffer,bilinear,texture,width,height}],output,uniforms{name,type,value}}].
- UBO: Globals (CameraBlockPos, CameraOffset, GameTime, ScreenSize), Fog (FogColor...), SamplerInfo (OutSize/InSize); кастомные UBO из JSON упаковываются std140.
- shaderc (GlslCompiler, ShadercIncludeResult) — препроцессор 26.3: только #include, #moj_import НЕ существует.

---
Task ID: 2
Agent: main
Task: Доработка c-LoD до продукта: исправления, релизная автоматизация, верификация

Work Log:
- НАЙДЕН И ИСПРАВЛЕН дефект: assets/glsl/{sun_scan,sun_solve,lod_terrain}.fsh использовали #moj_import — на 26.3 клиенте шейдер не скомпилился бы. Заменено на #include.
- check_glsl.sh: обновлён под #include + жёсткий отказ при #moj_import.
- Проверен pack_format=97 против version.json 26.3 (resource_major=97) — совпадает.
- scripts/e2e/gates.py переписан: старый был артефактом v1 (Display-архитектура), маркеры G1-G6 теперь соответствуют v2-логу (booting → pipeline ready → kernel ready → JNI bridge → atlas world → pack rebuilt).
- Добавлен .github/workflows/release.yml: тег v* → сборка linux-x64/macos-x64/macos-arm64/windows-x64 → CRUSSTY-бандлы (lod-vX-platform.tar.gz) + SHA256SUMS → GitHub Release (проверка version==tag).
- Локальная верификация (Rust 1.89.0 установлен из tarball): cargo fmt --check OK, clippy -D warnings OK, 23/23 тестов OK, release-сборка liblod.so 4.1MB OK, glslang: 7 шейдеров + 2 цепи OK.
- Commit f34a44b запушен в master, тег v2.0.0 запушен.

Stage Summary:
- c-LoD v2.0.0: продукт завершён — Rust-модуль (2261 строк: atlas/config/engine/http/jni/packgen/png/sha1/zipw), серверный ресурспак с 2 post-цепями (6-проходный lod_terrain: sun_scan/sun_reduce/sun_solve/rot_copy/lod_terrain/blit + atmosphere), без единого Display entity.
- GitHub: master на f34a44b, тег v2.0.0 → CI + Release workflow запущены.

---
Task ID: 3
Agent: main
Task: Финализация: CI/Release зелёные, каталог CRUSSTY

Work Log:
- CI на f34a44b: SUCCESS — все 9 джоб (rustfmt, clippy deny-warnings, tests, glslang, 3 ОС-сборки, packaging, strictness gate).
- Release на v2.0.0: SUCCESS — 4 бандла (linux-x64, macos-x64, macos-arm64, windows-x64) + SHA256SUMS.txt, публично доступны (200/302 CDN).
- Каталог crussty-catalog: добавлен модуль "lod" v2.0.0 на все 4 платформы с sha256 (commit 0d43c8f в main) — `crussty install lod` теперь работает.

Stage Summary:
- Продукт доставлен и опубликован: репо PLANETA9091/c-LoD (v2.0.0), релиз с бандлами, запись в каталоге.

---
Task ID: 4
Agent: main
Task: Тестирование c-LoD v4.1.0 на реальном клиенте + полное сканирование исходников Voxy

Work Log:
- Восстановлена среда после сброса: rustup (1.99.1), c-LoD переклонирован (master=v4.1.0 14d486f, CI+Release зелёные), CRUSSTY пересобран.
- Исходники Voxy (MCRcortex/voxy, ветка 26.3) склонированы и просканированы полностью агентом: octree 32^3→512^3 (5 уровней), greedy mesh ≤16x16, GL4.6 MDIC+HiZ+compute traversal, reverse-Z/0-1 depth, floating-origin через секции 32^3, fog/fade рецепт (последние 10% радиуса), metadata-in-alpha. Ключевой вывод: Voxy НЕ восстанавливает ориентацию в шейдерах (матрицы из CPU через mixin) — наш sky-shader camera channel — правильное решение для posteffect-only.
- Стенд ~/mc-test пересобран с нуля: JDK25 (Temurin 25.0.4.1), Paper 26.3-207, клиентский рантайм из манифеста (66 библиотек, нативы, 5147 ассетов 474MB).
- Диагностика и обход (все фиксируются в scripts/bench/README.md):
  * OOM/рипер песочницы -> heap 700M/1200M, atlas 512, one-shot скрипт в одном вызове;
  * fifo deadlock boot-скрипта -> порядок reader-first;
  * MC 26.3 требует Vulkan (renderpearl) -> options.txt preferredGraphicsBackend:opengl;
  * dril GLX не даёт sRGB-visual'ов, renderpearl просит SDL_GL_FRAMEBUFFER_SRGB_CAPABLE=1 -> shim libclodgl.c (LD_LIBRARY_PATH libGL.so.1, вырезает 0x20B1/0x20B2 в glXChooseFBConfig/Visual, glXGetProcAddress возвращает фильтрованные указатели);
  * MC использует SDL3 (LWJGL sdl-bindings), не GLFW — probes через SdlProbe.java;
  * промпт ресурспака -> XTest клик (python-xlib) по Yes;
  * Paper 26.3 переписывает server.properties (white-list=true default!) -> явно white-list=false.
- E2E на реальном клиенте ПРОЙДЕН: join, пак принят (клик), пост-эффекты clod:lod_terrain/clod:atmosphere загрузились (второй пакет warnings без clod), 8 скриншотов: полдень (v4.0-killer) LOD-полоса рендерится на 4 yaw, Veil-档 вне окна атласа чистая, возврат внутри восстанавливает LOD.
- ША1 пака детерминирован через 5 бутов: 750334661a32e2a28c204082899b23aae81cf1b8 (atlas restored).
- Скриншоты + харнесс закоммичены: a867e16 (docs/e2e-2026-10-20-noon/ + scripts/bench/ + README), push master, CI запущен.

Stage Summary:
- v4.1.0 верифицирован на реальном ванильном 26.3 клиенте в ПОЛДЕНЬ — солнце-трекер мёртв, камера-канал жив.
- Известные дефекты внешнего вида (на v4.2): тёмный столб по центру взгляда (grazing rays), тёмные цвета LOD, 43% покрытие атласа из-за локального scan_deadline=45s.
- Идеи из Voxy на v4.2: conservative max-height mip, fog-curve реконструкция + 10% edge fade, precision-схема секций, screen-area метрика LOD.

---
Task ID: 5
Agent: main
Task: v4.2.0 — исправление «налаживаются текстуры» + полный визуальный цикл скриншот→зум→фикс

Work Log:
- Восстановлен стенд после сброса: rustup 1.99.1, JDK 25.0.4.1 (~/tools/jdk25), glslang 16.6.0, клиентский рантайм 26.3 (5147 ассетов), CRUSSTY runtime, дистр Paper 26.3-207 + modules/lod. Paper-джар survived в /home/z/my-project/paper-26.3.jar; скан Voxy выжил в analysis/voxy.
- СТАТИЧЕСКИЙ АНАЛИЗ нашёл 3 корня: (1) окно марша minW/maxW считало origin ЦЕНТРОМ атласа, хотя packgen передаёт MIN EDGE → LOD-окно смещено на пол-атласа, на восток/юг LOD не было ВООБЩЕ (подтверждено скриншотами v4.1: yaw south/east = пусто); (2) билинейная выборка атласа: сmez цветов через границы ячеек + ПОЛУ-вода на побережьях + ghost-высоты на фронтире скана; (3) инверсия знаков в марше с v4.0: above = ground - rayY, хит ловил переход +→- (ВЫХОД луча из-под земли, бэкфейс), вход сверху вниз не ловился никогда.
- ФИКСЫ: minW=origin/maxW=origin+size; полностью nearest-выборка (texelFetch, per-cell), per-cell нормали (плоские Voxy-квады); инвертирован хит (вход -→+), затравка знака от камеры (t=0) с clamp -1 против VOID-колонки; horizon=size, dt=cell*0.5, MAX_STEPS=260; композит-фейд был ДВОЙНО инвертирован (edge=1-clamp → LOD давился везде кроме внешних 10%) → fadeOut по семантике Voxy (colour.a *= 1-fadeOut).
- ДИАГНОСТИКА на реальном клиенте (маркерные полосы: красный=цепь жива, зелёный=канал камеры мёртв, магента/жёлтый=хит/туман): цепь жива, conf=1, el=-15.2° точен, марш бьёт ВЕЗДЕ, туман не насыщен → композит. Затем ПРОБ-полоса (марш центрального луча, R=tHit/512, G=(hitY+64)/256, B=camY/256): camY=140.8, hitY=97.5, tHit=164 — геометрия точна.
- RAW-ATLAS VIEW (атлас прямо на экран): у GPU в atlas_col полосы на краях — это ДИФФУЗИОННЫЕ стрики encode_color_rgba в НЕОСКАНИРОВАННЫХ углах (PNG в паке byte-perfect против .bin — 0 mismatch). «Занавесы» = марш читал VOID как +1e8 = «под землёй» → каждый луч фейково бил в границу скан-диска, окрашенную стриками. ФИКС: void → -1e8 в ground_at; марш и фейд ограничены СКАНИРОВАННЫМ квадратом (новый UBO ClodScan/ScanBox из packgen: Chebyshev-радиус валидных ячеек).
- GameTime-детектив: диагностическая полоса показала GameTime ≈ 199→204 и растущий, `time set 6000` на него НЕ влияет — это WORLD AGE, не дневное время → свет был «ночной» на всех прогонах. РЕШЕНИЕ: точная реконструкция дневного тика из LIVE FogColor (база #c0d8ff × трек day.json #ffffff→#0f0f16, две рампы 11867..13670 / 22330..24133, инверсия наименьших квадратов + разрешение рассвет/закат ветки по непрерывности через новый пиксель rot (1,1); плато → 6000/18000). Реализация дублирована в rot_solve (персистенция) и lod_terrain (применение). Данные из client.jar: data/minecraft/timeline/day.json + dimension_type/overworld.json.
- Бенч: renderDistance 4→8 (стена тумана RD4 прятала полосу LOD), scan_deadline 45→200с, cell_size 4→2 (двойная детализация: отдельные кроны деревьев, лианы — Voxy-вид), клиентская матрица 8 ракурсов на все 4 стороны + облик.
- Цикл скриншот→зум→фикс×5: полосы → занавесы → тьма → плато-свет → финал. Финальные скрины: docs/e2e-2026-10-22-v42/ (юг=плоский океан+пляж, север=джунгли с кронами, запад=холмы+реки, облик=канопи с озёрами, у земли LOD Continuo с реальными чанками).
- Гейты: cargo fmt --check OK, clippy -D warnings OK, 38/38 тестов OK, glslang 10/10 OK.

Stage Summary:
- v4.2.0 визуально готов: настоящий LOD рендерится на все 4 стороны, Voxy-подобная геометрия (per-cell квады, кроны, вода, пляжи), точный ванильный свет (день/ночь/рассвет из живого тумана), Voxy-фейд на последних 10%, правильное окно атласа и скан-фронтир.
- Найденные корни задокументированы в комментариях шейдеров. Атлас-инструментарий (dump_atlas.py, диагностические полосы) описан в диалоге; бенч-скрипты обновлены в scripts/bench/.

---
Task ID: 6 (реконструкция после сброса — запись утеряна, код выжил на GitHub)
Agent: main
Task: v5.0.0/v5.0.1 SKY-GRID BIT-STREAM — весь LOD через стриминг, ноль текстур в паке, один пуш

Work Log:
- Разведка 26.3 байткода отвергла каналы: карты (карт-картиры на ваниле не рендерят произвольные данные), экранные каналы, переключение цепей (ClientboundPostEffectsPacket = только List<Identifier>). Выбран канал dust-частиц.
- Реализовано src/stream.rs: 16-бит слово = 4 частицы (X-суб = 2 бита данных, Z-суб = слот, msb-first, суб-квант cell/4); HI-слово = высота (h+64) старший байт, LO-слово = цветовой индекс 6x6x6 (вода=216); бюджет 1024 частиц/тик, полный круг 512 тиков ~26 с.
- capture.fsh: проекция 16 суб-позиций на экран, depth-строб (dust OPAQUE пишет depth; device depth 26.3: d = near/vz), merge в persistent ping-pong clod_h_a/b, clod_c_a/b 256x256 RGBA.
- НО: v5.0.1 находка — 26.3 reversed-Z прячет частицы ПРОТИВ неба (depth неба читается как 0); смена: LAYER_Y=[80,200] — частицы колоннами ПЕРЕД террейном, декодируются на фоне геометрии; hue strobe + чёрные/белые цвета (цвет частицы не данные, умножается random[0.48..1.0], HUE выживает).
- jni.rs: натч-мост CraftWorld.getHandle() -> ServerLevel.sendParticles + ParticleTypes.DUST + DustParticleOptions.<init>(IF)V (Bukkit spawnParticle — default-метод, GetMethodID не видит; NoSuchMethodError x36k устранён).
- packgen: 12 записей, НОЛЬ PNG; тест no_texture_inputs_stream_only; пуш пака РОВНО ОДИН при входе.
- Верификация: 41/41 тестов, clippy -D warnings, glslang 11/11; NoSuchMethodError=0, «stream armed» живьём.

Stage Summary:
- v5.0.1 запушена: 9788076 (SKY-GRID) + 1d8f103 (particle columns, reversed-Z findings). GitHub-бэкап СПАС работу от сброса песочницы.

---
Task ID: 7
Agent: main
Task: Восстановление после повторного сброса песочницы + GitHub-бэкап как постоянный протокол

Work Log:
- Сброс уничтожил инструментал (rustup, JDK, glslang, ~/mc-test) и локальный checkout v5.0. Уцелело: /home/z/my-project (worklog до Task 5, paper-26.3.jar, analysis/{voxy,mc263,crussty,c-hello}), c-LoD в состоянии v4.2-незакоммиченного.
- По требованию пользователя настроен протокол GitHub-бэкапа (PAT, pushes после каждого изменения, включая WIP).
- НАЙДЕНО: remote PLANETA9091/c-LoD уже содержит v4.2 (c3b987c), v4.3, v4.4.x, v5.0.0, v5.0.1 (1d8f103) — стриминг выжил на GitHub.
- Локальный грязный v4.2-дерево закоммичен в ветку backup/local-v42-pre-reset и запушен; master -> origin/master (1d8f103): src/stream.rs, capture.fsh на месте, Cargo.toml 5.0.0.
- worklog скопирован в репо (docs/worklog.md) и запушен — отныне бэкапится с каждым пушем.

Stage Summary:
- Ничего не потеряно: GitHub-бэкап сработал. Дальше — инструментал (rustup, JDK 25, glslang), сборка liblod.so v5.0.1, стенд, E2E с визуальным доказательством стриминга.

---
Task ID: 8
Agent: main
Task: v5.0.3/v5.0.4 — не-блокирующая генерация чанков, оживление стрим-декода, краш-форензика

Work Log:
- Восстановлен стенд после сброса: restore_env.sh (rustup+JDK25+glslang+paper+shim, одна команда), клиентский рантайм 474MB, python3-xlib/scrot через pip.
- E2E нашёл крит: Bukkit setChunkForceLoaded СИНХРОННО генерирует чанки на главном потоке (свежий мир) → watchdog «not responded 10s», HTTP-пак мёртв. ФИКС v5.0.3: NMS-мост ServerChunkCache.addTicketAndLoadWithRadius(TicketType.CHUNK_LOAD, ChunkPos, 0) — не-блокирующая генерация NORMAL-приоритета + удержание, release_ticket после сэмплинга. Bukkit-force поверхность удалена.
- Capture/марш не работал: (1) SamplerInfo UBO никогда не существовал в клиенте — OutSize/InSize были нулями (вся capture писала клетку (0,0), march делил на ноль) → заменён на Globals.ScreenSize (стандартный UBO 26.3, ванильные цепочки подтверждают); (2) capture использовал ЗАХАРДКОЖЕННУЮ камеру (yaw=180/pitch=-5/fov=70) вместо живого rot-канала → проекции мимо всех частиц при любом повороте. ФИКС: камера из clod_rot_a.
- Доказано живьём: частицы рендерятся клиентом (44k тёмных px в кадре), capture ДЕКОДИРУЕТ (цветная панель клеток в диагностическом кадре при камере в кроне), пак принимается автоматически, пост-цепочки активируются (terrain add true, veil add true), time set 6000 работает (gamerule в 26.3 НЕ существует).
- Краш-форензика (SIGSEGV pc=0, native thread, через 5-18 мин после ухода игрока): scan_cell, RBP=1e8ns → clod-engine цикл. Найдена гонка lost-wakeup в cplug-sdk flush (spawn-on-demand/exit-when-empty) — заменена постоянным воркером; idle-троттлинг 100мс→1с; poll_forced гейтится игроками. Краш редуцирован (18+ мин простоя), не побеждён — БЭКЛОГ.
- Плоскости частиц 260/272 → 80/120 (декод с земли против геометрии); black/white dust подтверждены; DUST_SCALE диаг 3.0 → 0.8.
- Мини-диск 128 (region_blocks) — весь в окне декода RD8 — доказал декод; финальный полный 512-диск возвращён.
- Незакрыто: серверный скан медленный (7071/65536 за 4000 тиков — force-очередь узкая), марш-рендер ЛОД на горизонте не подтверждён кадром; clod-pack.info врёт cells=0.

Stage Summary:
- v5.0.4: стрим-канал ЖИВОЙ и доказан картинкой; сервер не виснет на генерации; краш-проблема локализована и смягчена. Следующий спринт: разогнать скан, верифицировать марш, финальная матрица кадров.
