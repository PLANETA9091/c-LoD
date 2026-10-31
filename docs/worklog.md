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

---
Task ID: 8b (продолжение)
Agent: main
Task: FORCED-тикеты, чистый v5.0.4 прогон, верификация марша

Work Log:
- CHUNK_LOAD (flags=2) НЕ генерирует чанки за пределами view-distance — скан стоял на чанках возле игрока (7071/65536). Переключено на FORCED (flags=15, ванильная /forceload-семантика, генерация гарантирована), снятие тикета после сэмпла.
- Чистый прогон v5.0.4: день (time set 6000 работает, gamerule в 26.3 нет), 4 направления + oblique + ground + after_lap. Частицы в кадре, пак авто-принят, сервер стабилен всю сессию.
- ЛОД-полоса за RD в кадрах ещё НЕ видна: клеток 1380-7071 из 65536 (волна декодирует только клетки чьи частицы в кадре), марш-рендер требует отдельной верификации (след. шаг: рендер clod_h_b прямо на экран — одна итерация покажет и декод и марш).
- clod-pack.info отдаёт cells=0/0 всегда (косметический баг эндпойнта) — мониторинг через server.log.

Stage Summary:
- Транспорт ЖИВОЙ и доказан: частицы в кадре, декод клеток на клиенте подтверждён диагностической панелью.
- Бэклог спринта: (1) разогнать цикл скан→стрим→декод, (2) верифицировать марш (визуализация буфера), (3) финальная матрица кадров, (4) краш scan_cell pc=0 на idle (редуцирован троттлингом, не побеждён).

---
Task ID: 9
Agent: main
Task: восстановление стенда после сброса, независимое попадание в десинк эффект-листа, ЧИСТЫЙ A/B доказательство ЛОД (clouds-off), синхронизация с v5.2.2

Work Log:
- Песочница сброшена (~/mc-test, /home/z/tools, /tmp утеряны; выжили /home/z/my-project и liblod.so в target/). Стенд восстановлен: restore_env.sh (rust 1.99.1, JDK 25.0.5, glslang 16.6.0), setup_client.py (619MB, 5147/5147), python-xlib, симлинк jdk25.
- Обнаружено: локальное дерево (v5.1.0) отстало от origin/master — параллельная сессия успела запушить v5.2.1/v5.2.2 (fix десинка «Reconcile with ONE remove+add» + стоп ретраев, живые счётчики /clod-pack.info, particles_per_tick=2048 + 256 тиков/лап). Локально приведено к fa4999a; уникальные вкладки перенесены поверх; запасная ветка backup/v5.1.1-clean-ab.
- На устаревшем бинарнике независимо воспроизведён и починен десинк: «effect terrain add -> false» 3954 строки/прогон, st.terrain_on не восстанавливался. Мой sync_effect-хилер уступает v5.2.2-версии (та ещё убирает шторм ретраев, фризивший llvmpipe) — принята версия v5.2.2.
- ГЛАВНОЕ: чистый A/B протокол — renderClouds:false (облака травили диф: 5995 px шума на краях), toggle ТОЛЬКО clod:lod_terrain (/posteffect remove/add), ab_diff.py с тепловой сеткой. Результат: низкая камера (30,78,130) 6034 px (0.65%) — чистый силуэт над горизонтом; дальняя точка (30,78,530) 400 блоков: 43550 px (4.73%) — сплошной купол дальнего рельефа, видимый глазом (c13_14_far_composite.png).
- Геометрия: эффективный диск — квадрант x∈[0,512), z∈[64,576) от угла (0,64); аэро-облёты на запад/север смотрели вне диска. Силуэт ЛОД виден только с камеры НИЖЕ рельефа. pitch 26.3 инвертирован (плюс=вниз).
- Стрим на прогоне (v5.1.0-era, 1024/тик): tick 5400, 13570/65536 cells, lap 90, 5.11M частиц, 0 failed; пак один пуш на сессию, sha1 стабилен; после принятия пака ноль clod-warn в клиенте.
- v5.2.2 пересобрана локально: 36/36 тестов, clippy чисто, GLSL 11/11, lib задеплоен в dist/modules/lod.

Stage Summary:
- Ответ на «Лоды рендерится и стрим работает не криво?» — ДА: стрим ровный, ЛОД рендерится из персистентного хайтфилда (чистые A/B кадры в docs/e2e-v5.1.1/). Бэклог: дистанционный гейт в марше (краска заходит на ближний купол), краш scan_cell.

---
Task ID: 10
Agent: main
Task: Прозрачность стрима — ответ фактурой (TPS, кадры, надёжность) + восстановление стенда

Work Log:
- Песочница сброшена снова (тулчейн+~/mc-test утеряны; репо и target/release/liblod.so v5.2.2 целы). Восстановлено restore_env.sh одной командой + setup_client.py (5147/5147); найден и починен баг восстановления: boot_server.sh резолвит /home/z/tools/jdk25 — теперь restore_env.sh создаёт и этот симлинк.
- Сервер: все гейты OK; стрим v5.2.2 на полной мощности (2048 пак/тик).
- ОТВЕТ НА ВОПРОС «стрим мешает игре? он прозрачный?»:
  (1) TPS в устойчивом режиме 20.0/20.0/19.9 под полным стримом (просадка 19.1-19.4 только в ранней фазе — шторм генерации чанков сканом, не транспорт);
  (2) 13.3M частиц, 0 failed, скан 16336/65536;
  (3) видимость: с земли (выживание, внутри активной зоны) кадр чистый — ноль видимых частиц; худший случай (Y=280, взгляд в диск) — редкая россыпь точек 1-2 px (s_final_level.png);
  (4) архитектурно: capture read-only, LOD рисует только небо, реальная геометрия всегда выигрывает по depth, ближе 256м краски нет.
- Бенч-заметки 26.3: 6-арг tp = Incorrect argument (работает facing); консольные относительные координаты резолвятся от контекста консоли — нужен `execute at`; llvmpipe-клиент рендерит с лагом секунды — двеллы 90-240с обязательны.
- Артефакты: docs/e2e-v5.2.2-transparency/ (WORKLOG + 4 кадра).

Stage Summary:
- Стрим геймплейно полностью прозрачен (TPS 20.0, 0 failed, мир не трогает), визуально почти прозрачен (с земли невидим, с высоты — лёгкая пыль), цена канала — сеть ~41k пак/с (бэклог: дельта-режим).

## Task 11 — v5.3.2 ZERO-RESIDUAL (полная невидимость носителя + движение)

Задача: «чтобы даже с определённых взглядов не было видно частиц и диска (и чтобы даже в движении стриминг полностью работал идеально)». Синхронизация с origin (v5.3.1 stealth transport, f396967 — параллельная сессия), затем три слоя поверх: (1) маска из ОБЕИХ capture-раундов; (2) цветовая сеть в dust_mask (два тира: в полосе 3px / вне полосы 2px — второй тир съедает протухшие до-телепортные точки); (3) адаптивный erase_dust (кольца r+1…16r+16 против плотных волн). Находки: скан — кольцевой от центра сетки (256,320), камера вне круга скана = пустой транспорт; HI-точка на фоне неба пересекает LO-плоскость в 4–14 м — band-гейт по минимуму пересечений запрещал сеть для самого видимого класса (фикс: любая плоскость). Результаты: статика 10/10 кадров 0 зелёных px (OFF 260–1337), спин 75с — 0, хопы — 0, top-down sky-blind — +0 спавнов; скан 28833/65536, spawned 2.8M+, failed 0. 39/39 тестов, GLSL 12/12, кадры и отчёт — docs/e2e-v5.3.2-zero-residual/.

## Task 12 — v5.5 GEN-FRONTIER (скорость скана: последний медленный полукруг цикла)

Задача (бэклог v5.4): скан-фронтир был "gen-ticket bound" (~16 клеток/с, полный диск ≈ час). Живой диагноз: фронттир встаёт при TPS 20 — цепочка FORCED-тикетов против авто-размера chunk-system Paper на 2 CPU (авто = 1 gen-воркер) + окно wish (96) меньше чанк-набора фронтир-кольца (~100 чанков).
- config.rs: два ручки — gen_budget (16/тик вместо фиксированных 4) и gen_wish (512 в полёте вместо 96), клампы 1..256/32..4096; тест frontier_knobs_parse_and_clamp.
- engine.rs: wish_admit принимает окно параметром (тест на 32 и 512), сэмплинг-кап poll 128→256, диагностика `wish N forced N` в stream:-строке.
- Бенч: patch_paper_cfg.py (патчер paper-global.yml chunk-system worker/io-threads, env-gated), run_frontier_leg.sh (честная A/B нога: свежий мир + фиксированный сид + замер), scan_rate.py (парсер скорости фронтира); boot_server.sh: HTTP-гейт (пак обязан быть доступен — найден живьём тихий фейл бинда из-за выжившего JVM на порту 8199) + дефолт PAPER_WORKERS=4/PAPER_IO=2.
- Замер (2 CPU, свежий мир, сид clodbench2026, 180 с): auto → 35.3 клеток/с; workers=4/io=2 → 72.5 (best 755/с); workers=6/io=4 → 80.2 (best 896/с). Дефолт шипа — 4/2 (баланс TPS). Тёплый мир: сохранённые чанки сэмплятся мгновенно (49235 клеток за секунды).
- No-regression: полный стелс-протокол на v5.5 — top-down 0/0, level 196→0 (resid 0.0000%), спин+хопы 5/5 кадров 0 зелёных px, финальная A/B 320→0; пак байт-в-байт adeb4d16 (фронтир — только серверная сторона); TPS 20.0; 48/48 тестов, clippy 0, fmt 0.
Stage Summary:
- v5.5: скан разогнан ×2.3 на 2-CPU стенде с конфигурируемыми ручками для реальных хостов; невидимость и скорость стрима не тронуты (доказано кадрами). Бэклог: CPU-стена 2-ядерного стенда (клиент llvmpipe ест ядро), краш scan_cell на idle (смягчён), дельта-фидбек от клиента (нет канала — vanilla).

---
Task ID: 13 (реконструкция по коммитам — запись утеряна при сбросе песочницы)
Agent: main
Task: v5.4/v5.5/v5.6 — delta-zone scheduler, gen-frontier, pre-join arm; краш-форензика ЗАКРЫТА; CI полностью зелёный

Work Log:
- v5.4 DELTA-ZONE (59fbdc7): стрим платит только за декодируемые клетки и реальные правки (не платит за пустые/уже закрашенные/вне окна клетки).
- v5.5 GEN-FRONTIER (862144b): скан догоняет стрим (замер ×2.3) — см. Task 12.
- b675043/b7c9ec4: чин CI-YAML (heredoc внутри run-блока, кавычки имён с ':' и '#') + v5.6 PRE-JOIN ARM: диск армится (и тёпло сканируется) ДО первого захода игрока — headless CI теперь гоняет полный boot-путь без игрока.
- 5d58220: TESTS.md — полная карта тестов (7 слоёв: 48 Rust-тестов, статик-гейты, pack integrity, boot-гейты G1–G10, CI e2e-live ×2 бута, реальный клиент, архив доказательств).
- 841d5a0: фикс e2e-live ассерта — 13 записей в паке с v5.3.2 (dust_mask/dust_eater).
- 5bf2451 = e2e v5.6 живьём: арм за 32 с до первого join, тёплый диск 54874/65536 на тике 200, 1M+ частиц / 0 failed, TPS 20.0, 0 carrier px на горизонтом кадре pitch -10.
- КРАШ-ФОРЕНЗИКА ЗАКРЫТА: hs_err_pid16818 поймал «idle crash» в моменте — SIGSEGV pc=0 в Signal.dispatch на JVM, находящейся в середине shutdown, прибитой TERM+KILL-шквалом старого boot-скрипта (18m39s после ухода игрока = teardown сессии, НЕ idle). Одиночный TERM / двойной TERM / 18.5-мин idle soak — чисто на v5.6. Дисциплина shutdown (TERM → wait → KILL последним) в boot_server.sh и обоих CI-стоп-блоках.
- Доказательства: docs/e2e-v5.6-prejoin/.

Stage Summary:
- v5.6: первый полностью зелёный CI с 10-27; краш из бэклога Task 8 закрыт атрибуцией (не баг модуля). Бэклог: CPU-стена 2-ядерного стенда (не модуль), дельта-фидбек от клиента (нет канала — vanilla).

---
Task ID: 14
Agent: main
Task: Диспетчеризация GitHub: закрытие устаревшего PR #1, синхронизация версий/README, все тесты — через GitHub CI

Work Log:
- Состояние после сброса: репо цел (master f356730 → fetch до 5bf2451), тулчейн/стенд отсутствуют. origin ушёл вперёд: v5.3.0…v5.6 (см. Task 13), плюс эпоха-ветки looksbetter/good-look/fix/looks-better/restore-looksbetter (v4.4.x) и conceal-wip (v5.2.3 dust_conceal, явно помечен «kept as backup, not merged»).
- PR #1 «Fix LOD spikes/curtains and empty bands (atlas sampling + hole fill)» ЗАКРЫТ как not_planned с подробным комментарием: ветка правит src/atlas.rs и src/png.rs, удалённые с master в v5.0 (SKY-GRID: ноль текстур, пак 13 записей без PNG, ассерт в pack_builds). Его два корня приземлились иначе: texelFetch/nearest + void→-1e8 + ScanBox — v4.2; «пустые клетки» устранены архитектурно (незасканированная клетка просто не стримится). Комментарий: PR #1#issuecomment-6532284205.
- Версии синхронизированы с фактическим v5.6: Cargo.toml 5.5.0→5.6.0, Cargo.lock 5.2.2→5.6.0, module.json 5.3.2→5.6.0 (влияет на имя CRUSSTY-бандла в CI packaging).
- README переписан под v5.6: новая диаграмма (стрим частиц вместо атласа, 9 проходов цепи, stealth/delta-zone/frontier/pre-join arm), таблица фактов дополнена reversed-Z + NMS-мостом частиц, раздел «The bit-stream in one paragraph», конфиг-таблица из живого config.rs (atlas_size/scan_deadline_secs/atlas_cache/atlas_path удалены — их нет с v5.0), раздел «Measured, not promised» с реальными числами e2e, честные ограничения v5.6.
- docs/worklog.md дополнен Task 13 (реконструкция v5.6) и этим Task 14.
- Все тесты — на GitHub CI (требование протокола бэкапа): fmt/clippy/48 тестов/glslang/pack-integrity/3 ОС-сборки/бандл/e2e-live; локальный прогон не требуется.

Stage Summary:
- GitHub приведён в порядок: PR #1 закрыт с разбором, версии 5.6.0 везде, README честно описывает стриминговую архитектуру, worklog полон. CI на новом пуше — единственный источник истины по тестам.

---
Task ID: 15
Agent: main
Task: v5.7 SEEDGEN — нативная генерация never-generated чанков вне чанк-системы (рецепт Voxy seedgen на 26.3)

Work Log:
- Задача пользователя: «генерировать чанки, которые сервер даже не генерировал, супер-быстро, как seedgen у Voxy».
- Форензика paper-26.3 (mojang-mapped): полный рецепт iterateNoiseColumn снят с байткода — DensityVolume(1,h,1,x,minY,z) → NoiseChunk(rs, NULL-Beardifier, settings, fluidPicker, Blender.empty(), vol) → cachingSamplers().get(finalDensity).sampleVolume(volume) → y-loop: aquifer.computeSubstance(x,y,z,d) ?? defaultBlock, predicate Heightmap.Types.MOTION_BLOCKING.isOpaque() → hit y. ВАЖНО: 26.3 getBaseHeight/getBaseColumn НЕ применяет surface rules — материал верхней поверхности берём сами: MaterialSystem.topMaterial(rule, rs, ctx, biomeFn, chunk, samplers, pos, steep=false) с одноразовым ProtoChunk (пустая heightmap → градиент 0 → flat-семантика; крутые склоны теряются, апгрейд с реального чанка возвращает).
- Краш-расследование №1: SIGSEGV в jni_GetMethodID — bind_method звался на ЭКЗЕМПЛЯРЕ (level/cc/gen/settings/...) вместо класса. Все байндинги переведены на классы (get_object_class / load_cls).
- Краш-расследование №2 (ложный след 40 минут): «маркеры не печатаются, бинарь старый» — cp писал в dist/ РЕПО, а стенд читает ~/mc-test/dist (setup_server.sh копирует из target). Деплой на стенд всегда: cp target/release/liblod.so ~/mc-test/dist/modules/lod/.
- NPE-фикс: WorldGenerationContext.of(accessor) хранит level=null, а Paper-правило OptionallyFlatBedrockCondition зовёт .level() при компиляции правила → NPE на каждой колонке. Фикс: 3-арговый конструктор (ChunkGenerator, LevelHeightAccessor, Level) с живым ServerLevel.
- biomeFn для topMaterial БЕЗ хелпер-класса: MethodHandles.lookup().unreflect(BiomeManager.getBiome).bindTo(mgr) → MethodHandleProxies.asInterfaceInstance(Function.class, mh) — чистый JDK, ноль сборочных зависимостей.
- Реализация src/seedgen.rs (~1100 строк): SeedQueue (dedup/condvar), SeedIds (глобальные refs + method ids), bootstrap на main-потоке (одна попытка), воркеры-демоны (AttachCurrentThreadAsDaemon), чанк-батчинг: ОДИН NoiseChunk на 16x16x384 объём, 64 колонны-клетки читают общий ScopedDensityBuffer; per-column local frame; buf.close()/nc.close() на всех путях (пулы буферов!).
- Интеграция: scan_cell при незагруженном чанке → seedgen::enqueue (fallback — тикет-фронтир); клетки пишутся provisional=true; sample_chunk апгрейдит provisional с загруженных чанков (деревья/карверы/структуры доезжают с реальной генерацией; счётчики prov_upgraded/prov_diffs — фиделити-метрика). stream_diag вынесен из-под player-гейта (playerless-заполнение видно в логе и /clod-pack.info). G9 в gates.py стал playerless-aware. Конфиг: native_gen=true, native_threads=0 (auto, 1..4).
- ЖИВАЯ ВЕРИФИКАЦИЯ (2-CPU стенд, свежий мир, НОЛЬ игроков): tick 100 → 188/65536; tick 200 → 65536/65536 (100%!), native 1024 чанка, fail 0, queue 0. Между тиками 100→200 = 23 с стеночных → ~2750 клеток/с ОДНИМ воркером против ~80 клеток/с тикет-фронтира v5.5 = ×35. Все гейты зелёные (G1-G10 playerless).
- Осторожно с числами: 1 воркер на 2-ядерной коробке при idle-тиках; на реальном хосте native_threads масштабирует по ядрам; шум-математика ~1 чанк/25мс на ядро.
- Инструментарий: scripts/bench/seedgen_rate.py (парсер заполнения), run_seedgen_leg.sh (playerless-нога A/B).

Stage Summary:
- v5.7 SEEDGEN: холодный мир заполняется на 100% за десятки секунд без единого тикета и без TPS-цены; клетки provisional и само-апгрейдятся до полной фиделити при реальной генерации. Бэклог: хелпер-класс для y-loop (срезать ~1500 JNI-операций/колонку → ещё ×5-10), features (деревья) в нативном генераторе, A/B-нога тикетов для отчёта.

---
Task ID: 16
Agent: main
Task: v5.8 SEEDGEN TURBO — Java-side y-loop helper: один JNI-вызов на чанк вместо ~1500 на колонку

Work Log:
- Бэклог Task 15 пункт 1 (главный под требование «ультра-быстро»): форензика показала, что в gen_column каждый Y-шаг колонны — это 5 JNI-переходов (indexUnchecked, buf.get, blockY, computeSubstance, pred.test) + exception_check на каждой итерации и push/pop_local_frame на колонку. На колонну до поверхности ~200 шагов ≈ 1000-1500 JNI-операций, на чанк ≈ 256 колонн ≈ 250-400k переходов. Замер v5.7: ~2750 клеток/с при теоретическом потолке шумовой математики ~10к/с на ядро — разрыв и есть JNI-налог.
- РЕШЕНИЕ: java/ClodSeedgen.java — хелпер, компилированный ТОЛЬКО против JDK (ни одного Minecraft-импорта; все Minecraft-типы через MethodHandles, резолвятся в init() по mojang-mapped именам, снятым с живого v5.7-resolve). Весь чанк (DensityVolume+NoiseChunk+sampleVolume, ProtoChunk, 64-256 колонн: y-loop, topMaterial, материал/биом) выполняется ВНУТРИ JVM; наружу — packed long на клетку (bit63 valid, 0-15 height+32768, 16 liquid, 17-31/32-46 ординалы материалов/биомов) + String[] таблицы интернирования (volatile-снапшоты). Байткод встраивается в .so (include_bytes!), класс дефайнится при бутстрапе через ClassLoader.defineClass (JNI игнорирует protected).
- ПРЕФЛАЙТ БЕЗ PAPER: мок-иерархия (28 классов с точными пакетами net.minecraft.*) + harness на 1029 проверок — packing, interning, void-семантика, порядок dx-major, детерминизм, вода/биомы. Поймал ДВА реальных бага ДО стенда: (1) забытый `package clod;` — defineClass падал бы live; (2) инвертированный liquid (`liquid = isEmpty` вместо `!isEmpty` — Rust-путь отрицает). + 3 собственных бага теста (океан не в том чанке, паритет биомов, indexUnchecked мока).
- Rust: define_helper() в resolve() (defineClass + init с 16 глобальными рефами + dims; любая ошибка → null-поля → v5.7-путь, seedgen не выключается); gen_chunk диспетчеризует: helper → gen_chunk_native (один call_static_object_method, GetLongArrayRegion, две таблицы, unpack/palette/set_cell provisional) → при throw: legacy-путь для этого чанка, после 15 подряд — helper выключается навсегда (helper_fails в diag). unpack_cell + java_pack-зеркало покрыты тестами (контракт бит-в-бит).
- CI: новый job javahelper — перекомпилировать java/ClodSeedgen.class и сравнить sha1 с закоммиченным (защита от расползания исходник/байткод; scripts/java/build.sh). README: диаграмма (v5.7+5.8), measured-строка, 55 тестов, honest limitations (graceful degradation). Версии 5.8.0 (Cargo.toml/module.json/Cargo.lock локально).
- Локальная верификация восстановленным тулчейном (rustup stable 1.99): cargo fmt OK, clippy -D warnings 0, cargo test 55/55. Префлайт Java: PREFLIGHT PASS 1029 ok. Числа заполняемости ждёт CI e2e-live (playerless G9).

Stage Summary:
- v5.8: JNI-граница seedgen срезана с ~250-400k переходов на чанк до ~15; шум-математика (~25 мс/чанк на ядро) теперь почти весь бюджет. Ожидаемый разгон против v5.7 ×2.5-3.5 на том же железе (потолок ~10k клеток/с на воркера), финальная цифра — из CI-лога e2e-live. Оутпут бит-в-бит тот же (тот же рецепт, те же вызовы в том же порядке; палитра и provisional-апгрейд не тронуты). Бэклог: деревья в нативном генераторе, A/B-нога тикетов для отчёта.

Task 16 (продолжение — CI wave 1, run 41188128251):
- ПЕРВЫЙ CI-ПРОГОН v5.8: rustfmt/clippy/tests/javahelper/3×release/bundle — ВСЕ ЗЕЛЁНЫЕ (включая новый javahelper-гейт: перекомпилированный на Temurin 21 байткод побайтно совпал с закоммиченным). Упал ТОЛЬКО e2e-live, шаг «Boot #2», быстро (~1m50s весь job): Boot #1 прошёл целиком (пак + гейты + fetch) — значит хелпер живьём РАБОТАЕТ (defineClass+init+fill на настоящем Paper 26.3), а падение — тайминг.
- ФОРЕНЗИКА БЕЗ ЛОГА (api rate-limit): в CI-рецепте бута цикл ждёт «pack finalized» и СРАЗУ одним grep'ом проверяет «sky-grid stream armed». Порядок маркеров в логе: pack finalized → streaming world → [seedgen bootstrap] → … → stream armed (печатается асинхронно позже). v5.7-бутстрап занимал ~50 мс, v5.8 добавил defineClass + ~40 Class.forName + ~20 findVirtual ≈ 100-400 мс между маркерами — гонка, в которую Boot #1 попал по везению, Boot #2 закономерно.
- ФИКС ОБЕИХ СТОРОН: (1) ЛЕНИВЫЙ АРМ — bootstrap вынесен из EB-блока в scan_cell: первый незагруженный чанк армит seedgen прямо на движковом треде (одна попытка, tried() — атомарный флаг), маркер «stream armed» больше не ждёт резолв хелпера; (2) CI честно ЖДЁТ «sky-grid stream armed» до 120 с в обоих бутах (одиночный grep был гонкой с v5.6); (3) gates.py печатает INFO-строку нативного заполнения (chunks/cells/fails/workers/helper) — телеметрия без ассертов.
- Локально: fmt/clippy -D warnings/55 тестов, actionlint + YAML + py_compile чисто.
- CI wave 2 (runs 177/178 — ea1e232 + 10ee27e): ПОЛНОСТЬЮ ЗЕЛЁНЫЕ — rustfmt, clippy (-D warnings), 55 тестов, glslang, javahelper (байткод свеж), 3×release, CRUSSTY-бандл, e2e-live ОБА бута + SHA-стабильность пака, strict gate. Гонка арма закрыта; v5.8 — в master.

---
Task ID: 17
Agent: main
Task: v5.9 SEEDGEN HYPER — «может ещё быстрее можно»: батч-JNI, версионированные таблицы, identity-кэши, шаг-математика Y-прохода

Work Log:
- Постановка пользователя: seedgen уже ×35 против тикетов (v5.7) и внутри-JVM (v5.8) — «может ещё быстрее можно». Профилировка горячего пути v5.8 по коду: (1) intern-таблицы (matTable/bioTable) перечитывались КАЖДЫЙ чанк по одной JNI-операции НА СТРОКУ — 60-230 переходов на чанк, на порядок больше самого вызова chunk(); (2) на клетку: getFluidState+isEmpty+getBlock+getKey+toString+хэш-lookup карты + вся биомная цепочка unwrapKey→ResourceKey→Identifier→toString → ~2000 аллокаций+lookup'ов на чанк; (3) на Y-шаг колонны 5 MethodHandle-вызовов, из них indexUnchecked и blockY — чистая аффинная математика; (4) стрим-лок брался 2 раза на клетку (get + set_cell).
- РЕШЕНИЯ: (1) chunkBatch(int[],cell) — ОДИН JNI-вызов на БАТЧ чанков (конфиг seedgen_batch, default 8, clamp 1..64); payload: заголовок [n, n_ok, tabVer, spare] + сегменты [status, 256 клеток]; упавший чанк зануляет только СВОЙ сегмент (status 0) — соседние доезжают, фоллбек в v5.7-проход точечный; (2) tabVer (volatile int, ++ в rebuild) в заголовке: Rust-воркер кэширует таблицы и перечитывает только при сдвиге версии (JMM: volatile-запись таблицы до tabVer даёт happens-before читателю версии); (3) ThreadLocal IdentityHashMap: BlockState→(liquid|matOrd<<1), Holder→bioOrd — ванильные BlockState/Holder канонические синглтоны, identity-эквивалентность точна, вызовы и порядок сохранены → бит-в-бит; (4) шаг-математика: на КОЛОННЕ верифицируется аффинность indexUnchecked/blockY по ТРЁМ точкам (idx step, y step == -1), при подтверждении в цикле остаются bufGet+computeSubstance+pred (3 из 5), при отказе — точный v5.8-путь; верификация числовая, не структурная — работает с ЛЮБОЙ раскладкой volume; (5) один стрим-лок на БАТЧ покраски; авто-воркеры 4→8 (clamp, native_threads по-прежнему главнее); (6) полностью валидные/вне-диска чанки отсеиваются Rust-стороной ДО JNI-вызова (покраска пропускает уже валидные клетки).
- КОНТРАКТ СОХРАНЁН: рецепт iterateNoiseColumn внутри хелпера не тронут (те же вызовы в том же порядке); Rust-side parse_batch/unpack_cell/palette/provisional — те же. SeedQueue.pop → pop_batch (pop удалён как dead code, тесты переведены).
- ПРЕФЛАЙТ БЕЗ PAPER (scripts/java/preflight/: мок-иерархия net.minecraft ~30 классов + Preflight.java + run.sh): 52 чека × 2 JVM. Поймал ЧЕТЫРЕ проблемы ДО стенда: (1) мок ProtoChunk не наследовал ChunkAccess → ClassCastException в asType-адаптации MethodHandle (проверил, что в ванили наследует); (2) знак плотности мока был инвертирован (получался каменный столб до Y=319 вместо воздуха); (3) void-чанк с водой аквифера не void (вне мировой границы флюидов нет — AIR); (4) соразмерность ассертов таблиц реальному тестовому участку. Финал: PASS 52 checks × {affine, weird}, CELLS_SHA d213f48ea2526caf ИДЕНТИЧЕН на обеих раскладках, COLUMNS fast=1604 slow=0 (affine) vs fast=0 slow=1604 (weird — Fisher-Yates permutation томит фоллбек) — обе ветви шаг-математики доказанно эквивалентны; статусы сегментов/таб-ver/закрытие буферов на throw — проверены.
- Сборка байткода в JRE-only песочнице: ct.sym отсутствует → --release 17 падает; скачан Temurin 21.0.13+9 (ТА ЖЕ версия, что setup-java в CI) → javac --release 17 -Xlint:-rawtypes совпадает флагами со scripts/java/build.sh; scripts/java/clocal/Compile.java (compiler API через jdk.compiler модуль) оставлен как запасной путь. CI javahelper получил восстановительный артефакт (upload java/ClodSeedgen.class always()) — если point-release javac сдвинет байткод, каноническая пересборка приезжает оттуда.
- Rust: gen_chunk → gen_chunk_legacy (v5.7-проход, фоллбек) + gen_batch (payload-парсинг чистой функцией parse_batch с тестами; Tables-кэш по tabVer; один лок стрима); worker_loop: дренаж очереди pop_batch → один gen_batch → точечный legacy-фоллбек на status-0 сегментах; helper_fails считаются только с живым хелпером, disable при f==15 ровно один раз. Тесты 55→61: batch_payload_round_trip, batch_failed_segment_flagged, batch_payload_rejects_corruption, queue_pop_batch_drains_up_to_max, queue_pop_batch_blocks_then_wakes, seedgen_knobs_parse_and_clamp. Локально: fmt чисто, clippy -D warnings чисто, 61/61.
- Доки: README (диаграмма 5.7–5.9, конфиг-строка seedgen_batch, measured-абзац), TESTS.md (61 тест, слой 2b — preflight), версии 5.9.0 (Cargo.toml/module.json/Cargo.lock).

Stage Summary:
- v5.9: JNI-граница — 1 вызов на 8 чанков (~120/чанк → ~0.13/чанк с учётом таблиц v5.8), интернинг клеток кэширован по identity, Y-проход на 2/5 вызовов короче, стрим-лок на батч. Оутпут бит-в-бит тот же — доказано префлайтом на двух раскладках volume с совпавшим хэшем клеток и таблиц. Число заполнения/с ждёт CI e2e-live (playerless G9); ожидание — снять оставшийся JNI-налог и поднять масштабирование по ядрам (авто-воркеры до 8). Бэклог: деревья в нативном генераторе, A/B-нога тикетов для отчёта, skip-маска валидных клеток в payload (если re-sweep станет горячим).

---
Task ID: 18
Agent: main
Task: v6.0 NATIVE DUST WIRE — «партиклы нативными, чтобы вообще не жрали на сервере»: кэш пакетов по позиции + ClientboundBundlePacket, один send на игрока/tick

Work Log:
- Постановка пользователя: даже если сервер «не платит», за игроков тоже надо стараться, а партиклы можно сделать нативными, чтобы они ВООБЩЕ не жрали на сервере. Форензика legacy-доставки (jni.rs spawn_particle): на КАЖДУЮ частицу на КАЖДОГО игрока — 4 JNI-перехода (getHandle + get_field connection + NewObjectV 11-арг + call_void_method send), внутри send — ванильный dispatch + encode + ОДИН Netty-write на частицу. При батче 2048/tick и 4 игроках — до 32k JNI-переходов и 8k Java-объектов пакетов в секунду только на транспорт.
- РЕШЕНИЕ (v6.0 NATIVE DUST WIRE), три слоя амортизации:
  (1) КЭШ ПАКЕТОВ (src/wire.rs): каждый уникальный ClientboundLevelParticlesPacket строится ОДИН РАЗ и живёт глобальной ссылкой. Позиции спавна лежат на решётке (центры клеток — целые/полублоки, суб-офсеты — четверти, Y плоскостей 80/120 ровные), поэтому ключ — точный i64 (x*16, y*16, z*16) без epsilon и коллизий (тест на 128x128 клеток x 16 суб-позиций + полуплоскости). Кэш: HashMap ключ→(global ref, clock), cap 24576 записей, age-based eviction (холодные первыми, сортировка по clock; при полном прогреве — насильственная четверть самых старых); hits/misses/evictions в диаг.
  (2) БАНДЕРОЛЬ (jni.rs bundle_send): весь гейт-прошедший батч игрока уезжает ОДНИМ ClientboundBundlePacket(Iterable) через ТОТ ЖЕ доказанный m_send — один ванильный фрейм, один encode, один Netty-write на игрока на tick. Поверхность резолвится лестницей ctor-сигнатур (Iterable/Collection/List); класс опциональный — не резолвится → wire_ready()=false → legacy-путь, ноль регрессии. Пакеты внутри бандла — ТЕ ЖЕ записи, байт-в-байт legacy-пакеты (общий build_particle_packet: happy_villager, force=true, count=0) — клиент декодирует идентичный трафик, меняется только конверт.
  (3) ОДНОРАЗОВЫЙ СПИСОК на игрока на tick (ArrayList(256) локальный реф) — handed to the bundle record и НИКОГДА не мутируется после send (никакого clear/reuse: бандл может энкодиться на event-loop позже — гонка с переиспользованием исключена by design). >1024 пакетов режутся на куски через List.subList (клиентские бандл-гарды delimiter-эпохи).
- ГЕЙТИНГ НЕ ТРОНУТ: sky-in-view + decode band + frustum-конус (v5.3.1) применяются РОВНО как в legacy — wire_deliver() зеркалит legacy-цикл (включая diag-бакеты near/far/cone). При падении wire (bundle ctor/build/send/list) игрок мгновенно падает в legacy-цикл ЭТОГО ЖЕ tick — транспорт не умирает никогда; после 3 подряд отказов игрок прилипает к legacy (WIRE_FAIL_LIMIT, лог однократный).
- Счётчики: WIRE_SPAWNED/WIRE_BUNDLES/WIRE_FAILED (атомарики, диаг-строка «wire bundle|legacy Np/Nb fail N cache len/cap h/m/e»). Конфиг native_dust (default true, «1|true|yes|on»), false = честная v5-доставка. Счётчик streamed_cells ведёт ту же бухгалтерию (packets/8/players).
- Локальная верификация (тулчейн восстановлен: /home/z/.cargo/bin rustc/cargo 1.99.1): cargo build чисто; cargo fmt; cargo clippy --all-targets 0 warnings (почищены let-else→?, удалён мёртвый m_al_clear/list_clear — список одноразовый, clear не нужен); тесты 61→65 (+quant_is_exact_and_collision_free, quant_handles_half_blocks, evict_prefers_cold_and_bounds_size — ловил реальный порядок выселения, evict теперь строго oldest-first; +native_dust_knob).
- ВНИМАНИЕ (для будущего e2e): ClientboundBundlePacket — самый версионно-хрупкий кусок поверхности (имя пакета protocol.game, сигнатура ctor, клиентские бандл-гарды). Все точки отказов покрыты фолбэком; в honest limitations добавлено.
- Доки: README (диаграмма v6.0, fact-строка ClientboundBundlePacket, конфиг native_dust, measured-абзац wire, honest limitations «bundle wire leans on vanilla bundle decoding», 65 тестов), версии 6.0.0 (Cargo.toml + module.json).

Stage Summary:
- v6.0: серверная цена транспорта срезана с 4 JNI + 1 NMS-объект + 1 send + 1 Netty-write НА ЧАСТИЦУ НА ИГРОКА до: 1 build+send на игрока на tick (2 маленьких Java-объекта) + ОДИН build пакета на уникальную позицию (после первого прохода — ~100% кэш-хитов, на частицу НОЛЬ Java-работы). Оутпут на проводе для клиента — те же байт-в-байт пакеты (бандл — только конверт), рендер и стелс-гейтинг не изменены. Числа TPS/памяти ждёт CI e2e-live; ожидание — стрим-степ перестаёт быть заметной строкой в тик-профиле. Бэклог: повторное использование списков через netty-безопасный snapshot, замер gate/wire в ab_diff, деревья в seedgen.
