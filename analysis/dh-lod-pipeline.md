# Distant Horizons 3.3.4 — пайплайн рождения LOD (разбор исходников)

Источники: `analysis/dh` (тег 3.3.4, GitLab) + `analysis/dh/coreSubProjects` (core-сабмодуль
`6441062`). Пути ниже — от `coreSubProjects/core/src/main/java/com/seibel/distanthorizons/core/`
(`common` — обвязка над MC). Всё ниже — из кода, не из догадок.

## 1. Полные данные: chunk → FullDataSourceV2
`dataObjects/transformers/LodDataBuilder.createFromChunk`:
- Секция = 4×4 чанка (128×128 блоков, SECTION_BLOCK_DETAIL_LEVEL).
- Колонка 1×1 блок сканируется СВЕРХУ ВНИЗ от `max(lightHeightmap, solidHeightmap)`
  (подняться пока не air), раны режутся по смене (biome, blockstate);
  точка = `(id из (biome,state)-мапы, height, bottomY, blockLight, skyLight)`.
  Свет берётся от блока НАД раной (y+1); на верхушке skyLight = 15.
- `forceSingleBlock`: если текущий блок не solid/liquid/opaque (снег-слой, огонь,
  цветы) — следующая точка обязана быть отдельной, чтобы оверлей красил только
  верх колонки.
- Точки кодируются в `FullDataPointUtil` long; слияние одинаковых (id+light)
  подряд — на этапе `RenderDataPointReducingList`/mip-апскейла.

## 2. Render-данные: FullData → RenderDataPoint
`dataObjects/transformers/FullDataToRenderDataTransformer.setRenderColumnView`
(идёт сверху вниз по колонке):
- **Cave culling**: блок не-solid/liquid/полу-прозрачный, skyLight==0, у
  следующей точки тоже skyLight==0, ниже `caveCullingHeight` (default 32 над
  minHeight) → пещера: предыдущая точка продлевается вниз (`setYMin(bottomY)`),
  блок пропускается. Первая/последняя точка колонки не кулятся.
- **Ignored blocks** (`blocksToAvoid`): пропускаются; цвет и свет передаются
  блоку ниже (`tintWithAvoidedBlocks`, по умолчанию true); свет передаётся
  всегда (иначе тёмная сетка на границах LOD).
- **Water under ice**: вода под льдом → лёд.
- **Snow layer 1-3** (и water-surface-replacement): высота точки −1; если слой
  был единственным блоком — красим блок ниже (снег → `snowColor`, иначе
  `waterColor`).
- **Цвет точки** = `levelWrapper.getBlockColor(pos, biome, dataSource, block)`:
  см. §4. Материал = `block.getMaterialId()` (EDhApiBlockMaterial).
- **Вертикальный мердж по цвету**: `color == lastColor && bottomY+height ==
  lastBottom` → предыдущая точка продлевается вниз (две точки в одну). Это
  и есть «слияние» — меш не растёт внутри однотонных колонн.
- Пустая колонка → один `EMPTY_DATA`; void-точка (над бездной) —
  `setVoidAir`: minY=0, alpha=0, material AIR (только для света).

## 3. Квады и коллары: ColumnBox + ColumnRenderBufferBuilder
`dataObjects/render/bufferBuilding/ColumnBox.addBoxQuadsToBuilder`:
- Каждый datapoint колонки = плоско-окрашенный BOX (minX/minY/minZ, ширина
  datapoint'а, высота диапазона).
- **Top-грань** рисуется, если точка выше не начинается ровно на maxY (или она
  прозрачна); **bottom** — если точка ниже не кончается ровно на minY (или
  прозрачна; над бездной нижней грани нет).
- **Боковые грани (коллары)**: для каждого из 4 направлений берётся колонка
  соседа (`ColumnRenderView`), и диапазон [yMin,yMax] текущей точки режется на
  Y-сегменты светом соседа:
  - соседа нет (регион не сгенерирован) → полная стена на всю высоту,
    skyLight = MAX. (На фронтире DH эти стены скрыты туманом: farFogEnd ==
    радиусу данных.)
  - сосед непрозрачен и перекрывает диапазон → сегмент получает
    `SKYLIGHT_COVERED = -1` → **грань не рисуется**.
  - сосед прозрачен → свет берётся у точки НИЖЕ соседа.
  - OVERRDRAW-хак: на границе LOD-регионов (x==0 / z==0 / край) при lit
    соседе грань рисуется даже под перекрытием (дыры при смене детализации).
  - сегменты с skylight >= 0 и высотой > 0 → `addQuadAdj` (стена) со светом
    сегмента.
- **Shade в билд-тайме**: UP 1.0, DOWN 0.5, N/S 0.8, E/W 0.6
  (`ClientLevelWrapper.getShade`, ENABLED-ветка).
- **FADE_TO_DIRT** (`LodQuadBuilder`): нижний ряд вершин стены из травы
  перекрашивается в `getDirtBlockColor()`; GPU интерполирует по пролету стены.
- Полноценный culling данных (`FullDataOcclusionCuller`) выкидывает точки
  колонки, полностью окружённые непрозрачными соседями по 4 сторонам (в
  хайтфилде это «всё, что ниже топа» — у нас не актуально).

## 4. Цвета: ClientBlockStateColorCache (плоские цвета)
На один BlockState один раз:
- **Текстура**: первая непустая грань в порядке UP → unculled → N → E → W → S
  → DOWN; RotatedPillar (брёвна) пропускает UP (кора вместо торца); Slab →
  DOUBLE-вариант; HugeMushroom → default state. Первый quad → его sprite.
- **Средний цвет** (`calculateColorFromTexture`): ПОПИКСЕЛЬНО в ЛИНЕЙНОМ
  пространстве: `r_lin = srgbToLinear(r/255)`, вес = alpha (умножается на
  scale режима); суммы `red += r_lin*a*scale`, `alpha += a*scale`; результат
  `linearToSrgb(sum/alpha)` — обратно через таблицу. Таблицы — стандартный
  sRGB EOTF (таблица ДХ — квантованная версия формулы; байт-разница макс).
  Режимы (EColorMode):
  - `Leaves` (LeavesBlock): пиксели с a==0 пропускаются, остальные a=255.
  - `Flower` (FlowerBlock): пиксели, где `g+25 < b || g+25 < r`, получают вес ×5
    (цветок доминирует над зеленью стебля).
  - `Glass` (имя содержит "glass"): прозрачные пиксели учитываются (сумма 0 —
    no-op).
  - `Chisel`: пиксели не считаются вовсе → цвет (0,255,255,255) = невидимый.
  - Default: a==0 → пропуск.
- Пропавший texture (magenta) → alpha 0 (не рендерится).
- **Ликвиды**: цвет = particle sprite (вода → water_still, первый кадр).
- **Тинт**: если quad тинтованный (`needPostTinting`), цвет = MC color resolver
  (grass/foliage/water для биома в точке) × база: `multiplyARGBwithRGB` =
  целочисленное `(c*t)/255` поканально. Наш `mul_rgb` совпадает.
- Biome tint у DH — живой resolver MC (в т.ч. шум внутри биома); наша статичная
  таблица по биому — согласованное приближение (отличие видно только в
  болотах/саваннах с градиентом).

## 5. ТЕКСТУРНЫЕ LOD (новое в 3.3.x, включено по умолчанию!)
`Config.Client.Advanced.Graphics.Texture`:
- `enableTexturedLods = true` (по умолчанию!), `maxTexturedLodDetailLevel = 2`
  (датапоинты ≤ 4×4 блока получают текстуры; дальние — плоские).
- `BlockTextureRegistry`:
  - На blockstate — «set id» = 6 тайлов (по граням). Нет текстуры грани →
    подстановка UP-тайла (fences/walls/anvils). Все грани плоские → set 0.
  - Тайл 16×16 (спрайт ближайшим сэмплом до 16×16, первый кадр анимации),
    **ratio-кодирование**: `avg` = простое среднее ПО ВИДИМЫМ (a≠0) пикселям
    ПОКАНАЛЬНО (не линейно!); `ratio = round(c/avg * 127.5)`, clamp 255;
    alpha = исходная. Все пиксели == avg → тайл не создаётся (плоский).
    `128 → 1.0`, `255 → 2.0`.
  - Тайл 0 — зарезервированный плоский серый (128,128,128,255).
- **`useBottomTextureForSides`** (`blocksDontUseSideTextureCsv` = grass_block,
  mycelium, nylium, dirt_path, grass + тег grass_blocks/nylium): боковые грани
  текстурируются НИЖНЕЙ текстурой (dirt) — повторяющаяся боковая трава
  выглядит плохо.
- **pickFaceQuad**: при смеси тинтованных и нет (трава+земля на боку) берётся
  НЕтинтованный quad; спрайт копируется «как есть» (тинт остаётся в плоском
  цвете — ratio умножает на него, тинт сохраняется).
- Ликвиды: particle sprite (water_still).
- **Шейдер** (`shaders/terrain/gl/frag.frag`):
  - Плоский цвет идёт из вершины УЖЕ освещённым:
    `vertexColor = lightmap(sky/16, block/16) * shadedColor` (vert.vert).
  - Текстура: `tile = texture(uBlockAtlas, uv)` (с мипами!),
    `clamped = fragColor.rgb * (tile.rgb * 2.0)`,
    `fragColor.rgb = mix(fragColor.rgb, clamped, tile.a)`.
  - UV повторяется на каждый блок грани: `fract(vBlockPos)`; ориентация:
    up `(x,z)`, down `(x,1−z)`, north `(1−x,1−y)`, south `(x,1−y)`,
    west `(z,1−y)`, east `(1−z,1−y)`. У далёких LOD 1 юнит = cell_size блоков →
    горизонтальный повтор = на ячейку; вертикаль всегда по блокам (Y в
    блоковых единицах).
  - **Шум — только на плоских**: `uNoiseEnabled && vTextureTileId == 0u`.
  - Дизеринг/клип (`uDitherDhRendering`) — по конфигу, к геометрии отношения
    не имеет.

## 6. Туман, свет, прочее (проверено в v4.3.0, не менялось)
- Fog: exp-squared, `x = max((d−s)/l,0)*dens`, `1 − range/exp(x²)`,
  farFogStart 0.4, farFogEnd 1.0 × радиус данных, dens 2.5, цвет — мировой
  туман; применяется только к нарисованным LOD-пикселям (`drawnTo` = depth !=
  clear — это НЕ про дыры данных).
- Шум: `applyNoise` — verbatim портирован у нас (steps 4, intensity 0.05,
  dropoff 1024).
- void/несгенерированные данные: стены против отсутствующего соседа РИСУЮТСЯ,
  но сидят на фронтире за туманом. Наше v4.3.0-решение (дыры скана без стен)
  оставлено намеренно: у нас дыры бывают внутри радиуса, где тумана нет;
  отклонение задокументировано в шейдере.

## 7. Что портировано в c-LoD (v4.4.0)
- `bake_block_colors.py` → точная цветовая математика §4 (линейное усреднение,
  Leaves/Flower/Glass, брёвна→кора, слэбы→DOUBLE, ликвиды→particle).
- `bake_block_textures.py` → ratio-тайлы §5 (16×16, простое видимое среднее,
  useBottomTextureForSides, fallback на UP, дедуп по байтам).
- `atlas.rs`/`jni.rs` → tex_top/tex_side на ячейку (CLODAT03).
- `packgen.rs` + `lod_terrain.json` → текстура тайлов (256 тайлов/ряд) +
  per-cell tile-id текстура.
- `lod_terrain.fsh` → DH UV-повторы, `mix(albedo, albedo*tile*2, tile.a)`,
  шум только на плоских.
