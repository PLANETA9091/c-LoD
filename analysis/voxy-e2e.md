# Voxy 0.2.20-beta (MC 26.3) — полный E2E-анализ

Источник: `MCRcortex/voxy` ветка `263` (= `dev`, 5bf3335) + распакованный jar с
Modrinth (`voxy-0.2.20-beta.jar`, game_versions `[26.3]`). Цель: реплицировать
**сущность** LOD-ов c-LoD-ом (структуры, геометрия, свет, атмосфера) в
ограничении posteffect + серверный ресурспак, БЕЗ Display entities.

---

## 1. Мир: данные и структуры

### 1.1. Воксель — 64 бита (`Mapper.composeMappingId`)
```
биты  0–26 : (не используются; воздух = только свет)
биты 27–46 : blockId — 20 бит, индекс в глобальной палитре BlockState
биты 47–55 : biomeId —  9 бит, индекс палитры биомов
биты 56–63 : light —     8 бит: (blockLight << 4) | skyLight
```
`isAir = blockId == 0`. Воздух несёт свет; вода/листва — обычные воксели.

### 1.2. Секция и октree (`WorldSection`, `WorldEngine`)
- Секция = **32×32×32 вокселей** (`long[32768]`, 256 КБ), адрес `(lvl,x,y,z)`,
  координаты 24 бита, id = `(lvl<<60)|(y&0xFF)<<52|z<<28|x<<4`.
- `MAX_LOD_LAYER = 4` → уровни 0..4; уровень L — ячейка = 2^L блоков,
  секция L покрывает 32·2^L блоков.
- `nonEmptyChildren` (байт-маска 8 детей), `nonEmptyBlockCount` (для lvl 0) —
  это основа иерархической окклюзии.

### 1.3. Mipper — как рождаются дальние LOD (`other/Mipper.java`)
**Цвета НЕ усредняются.** Из 8 детей выбирается воксель с **максимальной
opacity**, приоритет по углам (111 > 110 > 011 > 010 > 101 > 100 > 001 > 000),
и берётся его **полный id** (blockstate + биом + свет). Если все 8 — воздух,
свет усредняется (blockLight — среднее, skyLight — ceil-среднее).

Opacity = `BlockState.getLightDampening()`; **листья принудительно 15**
(сплошные в LOD). Следствие: дальний LOD — это *настоящие блоки*, прореженные
с шагом 2^L, поэтому Voxy выглядит как ваниль на любой дистанции.

### 1.4. Greedy meshing (`client/core/util/ScanMesher2D.java`)
Плоскость 32×32, run-length по строке + merge соседних строк, максимум
**16×16** на квад. Выдаёт `(x, z, length, width, data)`. Оси: Y → Z → X
(`axis = 0..2`), лицо = `(axis<<1)|side`.

### 1.5. Квад — 64 бита (`shaders/lod/quad_format.glsl`)
```
биты  0– 2 : face (6 граней)
биты  3– 6 : size.x−1 (4 бита, 1..16)
биты  7–10 : size.y−1 (4 бита, 1..16)
биты 11–15 : pos.x (5 бит, внутри 32³ секции)
биты 16–20 : pos.y (5 бит)
биты 21–25 : pos.z (5 бит)
биты 26–45 : stateId (20 бит — палитра блоков)
биты 46–54 : biomeId (9 бит)
биты 55–62 : lightId (8 бит)
```
Квады раскладываются по отдельным буферам: translucent / double-sided /
directional (`getQuadTyping`). Мировая позиция: `quadStart*2^L + origin секции`.

---

## 2. Рендер: как это выглядит

### 2.1. blockModelAtlas (`shaders/lod/gl46/quads.frag`, `block_model.glsl`)
- Атлас моделей: сетка **256×256 моделей**; модель = **3×2 тайла граней**
  (`vec2(3.0,2.0)*256.0`); `texPos = (modelId&0xFF, modelId>>8)/256 + смещение
  грани`. Запечён на клиенте из ванильных моделей (SoftwareRasterizer).
- `BlockModel`: `faceData[6]` (UV-инсеты с точностью 1/64, alpha-cutout флаг,
  tintState), флаги: biome-LUT, translucent, shaded.
- UV внутри квада: `uv ∈ [0, quadSize]` в ячейках; **`frac(uv)` — тайлинг:
  одна ячейка = один тайл текстуры**; mip через `textureGrad` — дальние
  LOD-квады автоматически блюрятся до среднего цвета.
- Обрезка: `frac(uv)` за пределами `(size-1)` → discard (усечённые квады).

### 2.2. Цвет фрагмента (`quads.frag computeColour`)
```
colour  = textureGrad(blockModelAtlas, texPos, dx, dy)      // реальные тексели
discard если alpha (mip 0) <= 0.1                            // cutout (листва!)
outColour = (colour * uint2RGBA(interData.y))                // lightmap×shade
            + vec4(0,0,0, interData.w&0xFF/255)              // addin-альфа
затем × uint2RGBA(interData.z)                               // biome tint
```
где (`quad_util.glsl makeRemainingAttributes`):
- `interData.y = packVec4( lightmap(lightId) * faceShade(face) )` — **RGBA из
  светокарты, умноженной на ванильное затенение грани**;
- `interData.z = biome tint` (`colourData[tintBase + biomeId]` при biome-LUT,
  иначе `model.colourTint`);
- тест глубины против ванильного depthTex (`DEPTH_SCALAR_COMPARE`, reverse-Z).

### 2.3. Свет (`lighting.glsl` + ванильная светокарта)
- `getLightmapUv(index)`: 16×16 светокарта, `clamp(base*15/16 + 0.5/16)`.
- index = lightId вокселя = (block<<4)|sky.
- faceShade = `cl.cardinalLighting()`: **UP 1.0, DOWN 0.5, N/S 0.8, E/W 0.6**
  (unshaded-модели → NO_SHADE = up).

### 2.4. Туман и fade (`NormalRenderPipeline.finish` + `post/blit_texture_depth_cutout.frag`)
```
fog:  fogLerp = clamp(dist * (1/(end-start)) - start/(end-start), 0, CAP)
      CAP = clamp(max(vanRD, 320)*sqrt(3) * invDelta + startDelta, 0, 1)
      colour.rgb = mix(colour.rgb, fogColor.rgb, fogLerp * fogColor.a)
fade: MODE=1 (xz): start = max(vanRD, rd*0.9); end = max(vanRD, rd)
      colour.a *= 1 - clamp(xzDist * scale - start*scale, 0, 1)
```
- `dist` — **длина реконструированной 3D-точки** (`invProj*clip`), не gl_FragCoord.z.
- Туман линейный по ВАНИЛЬНЫМ environmentalStart/End, но **каппится на
  ~554 м** — дальше чистая геометрия до горизонта.
- Композит: alpha-блендинг SRC_ALPHA/1−SRC_ALPHA поверх ванили + перезапись
  gl_FragDepth из LOD-глубины (транслюценты сортируются корректно).
- `MixinFogRenderer`: ванильный fog **по render-distance** отключается
  (→ 999999999), environmental — по режиму; дефолт `FOG_AND_FADE`.

### 2.5. Окклюзия и пути рендера
- Иерархический compute-traversal по AABB-ям секций + HiZ (hierarchical/),
  GL4.6 MDIC (gl46/), reverse-Z [0..1], floating origin через секции,
  TAA-оффсет, опциональный SSAO. Для c-LoD нереплицируемо и не нужно:
  posteffect-проход сам отсекается по ванильной глубине (уже работает).

---

## 3. Что НЕ может быть реплицировано в posteffect (и почему это ок)
| Voxy | Почему не воспроизводимо | Что берём вместо |
|---|---|---|
| 3D-октree + MDIC-геометрия | posteffect рисует fullscreen-квад, не вершины | рейкаст по mip-пирамиде высот (уже есть) |
| Окклюзия HiZ-траверс | не нужна — видимая поверхность = рейкаст | отсечение по ванильному depth (есть) |
| Клиентская запечка моделей | сервер не имеет рантайма моделей | запечка атласа состояний на сервере из ванильных текстур |
| Живая светокарта-текстура | нельзя биндить | UBO `LightmapInfo` (проверяется) → fallback: реконструкция из timeline + `get_brightness` |
| Транслюцент-сортировка | один проход | вода как полуплоскость с alpha |

## 4. Решение v4.3.0 «VOXY CORE» — реплицируемая сущность
1. **Mipper 1:1 на Rust** (`voxy_core.rs`): пер-уровневая пирамида
   max-opacity-угловых вокселей с полным id — дальние LOD = настоящие блоки.
2. **Greedy ≤16 на Rust**: квады каждого уровня; из них — UV-спаны для
   шейдера (frac-тайлинг: 1 ячейка = 1 тайл).
3. **Атлас состояний** `atlas_states.png` (≤1024²): на состояние — 3×2 тайла
   граней 16 px + ручной mip-стрип (16/8/4/2/1) — mip-ы как у Voxy (MipGen),
   т.к. `PostChainConfig$TextureInput` грузит SimpleTexture БЕЗ mip-цепочки.
4. **Шейдер quads.frag-математика**: textureGrad-выбор тайла по (state, face),
   cutout-discard ≤0.1, ×(lightmap×faceShade), ×biome-tint, срез frac за
   пределами квада.
5. **Свет**: UBO LightmapInfo (`get_brightness(l)=l/(4−3l)`, SkyFactor/
   BlockFactor/SkyLightColor/AmbientColor/BlockLightTint — формула lightmap.fsh
   дословно) с fallback на timeline-реконструкцию v4.2.
6. **Туман/fade/композит** — уже точные в v4.2 (тот же исходник), остаются.
7. **Скан**: на колонку — top state-id + biome-id (расширение jni.rs), свет
   поверхности (sky 15 над рельефом; block-light проба — фаза 2).

## 5. Проверено на реальных исходниках (не по памяти)
- `PostChainConfig$TextureInput` = `{sampler_name, location, width, height,
  bilinear}` → `textures/effect/<name>.png`, SimpleTexture, без mip-ов →
  ручной mip-стрип обязателен.
- `lightmap.fsh` (26.3): формула выше дословно; UBO объявлен только в
  lightmap.fsh — привязка в post-проходе требует проверки на живом клиенте.
- Колормапы: `assets/minecraft/textures/colormap/{grass,foliage,dry_foliage}.png`.

## 6. Скан №2 (v4.3.2): текстурная модель quads.frag — построчно (файлы 26.3-ветки)

### 6.1. quads.frag — mip через textureGrad от ПОЛНОГО uv
- `vec2 uv2 = modf(uv, tile)*(1/(768,512))` — uv приходит из вершинки в
  ЯЧЕЙКАХ (getCornerUV = uvCorner + quadSizeAddin*corner), `modf` делит на
  целую часть (индекс ячейки в кваде) и дробную (позиция в тайле).
  => **один тайл текстуры на ОДНУ ЯЧЕЙКУ уровня L (2^L блоков)**, не на блок.
- `uvSmol = uv*(1/(768,512)); dx=dFdx(uvSmol); dy=dFdy(uvSmol);
  colour = textureGrad(blockModelAtlas, texPos, dx, dy)` — производные берутся
  от ПОЛНОГО uv (до fract-эквивалента), поэтому GPU-мип = `log2(16*pxFoot/2^L)`
  (16 px тайл на 2^L-ячейке): дальние квад-ы блюрятся к среднему цвету тайла.
  v4.3.1 выбирал стрип-уровень `ceil(log2(pxFoot))` с тем же периодом — давал
  8..16 текселей/пиксель без редукции = шум «наложения текстур».
- Дискард: `if (useDiscard() && textureLod(blockModelAtlas, texPos, 0).a <= 0.1)`
  — тест по **mip-0** альфе в точной позиции тайла (комментарий в коде: «stops
  mipmapping fucking it over»), цвет при этом с дистанционного мипа.

### 6.2. makeQuadFlags (quad_util.glsl) + ModelFactory — семантика cutout
- `flags |= faceHasAlphaCuttout(faceData);
   flags |= any(quadSize > 1) & faceHasAlphaCuttoutOverride(faceData);`
  => **у merged-квадов (quadSize>1) дискад отключён override-флагом** — дальние
  леса рендерятся СПЛОШНЫМ усреднённым цветом.
- ModelFactory: `needsAlphaDiscard = coverage<0.9 | layer!=SOLID;
  &= layer!=TRANSLUCENT`; override (bit23) — когда грань не покрывает блок.
  `LEAVES => layer=SOLID` (листва — SOLID, но coverage<0.9 => cutout на
  одиночных ячейках).
- Порт в c-LoD: каждая ячейка heightfield уровня L>=1 — это и есть
  Mipper-merged квад => дискад только при L==0 (одиночная ячейка).

### 6.3. MipGen.java + TextureUtils.mipColours — рецепт мип-цепочки
- `solidify`: BFS от каждого непрозрачного текселя — прозрачные получают RGB
  ближайшего непрозрачного (альфа не трогается). Без этого мипы тянут чёрный
  (0,0,0) прозрачных текселей — тёмная сетка вдали.
- `mipColours(darkened=false)`: RGB — среднее 2x2 в ЛИНЕЙНОМ пространстве
  (srgbToLinear -> avg/4 -> linearToSrgb), альфа — простое байтовое среднее.
- Ванильные мип-ы клиента (generateMipmap) так НЕ делают — это кастом Voxy;
  наш ручной стрип обязан повторять именно его (v4.3.2: solidify_tile +
  линейный mip_tile в voxy_core.rs — байт-в-байт семантика).

### 6.4. Верификация на реальном клиенте (CI, vanilla 26.3 + Xvfb)
- До (ca97bfb): пёстрый шум по всей LOD-поверхности (небо-точки листвы на
  mip0-альфе всех уровней + алиасинг 16 т/п + тёмная сетка straight-мипов).
- После (6067b63, три прогона CI): вдали сплошные усреднённые тайлы (леса
  цельные, вода гладкая), вблизи — пер-блочная деталь и дырки листвы,
  пер-ячеечный тайлинг «один тайл на ячейку» = Voxy-look. Зум-кропы в
  docs/e2e-ci/ (коммитятся CI сpermissions: contents:write).
