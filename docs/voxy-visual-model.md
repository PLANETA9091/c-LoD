# The Voxy visual model — extracted from the binaries (0.2.20-beta, MC 26.3)

The user call: c-LoD must LOOK like Voxy, and that judgment must be based on
the actual Voxy code, not on invention. Sources of truth (all local):

- Release binary: `analysis/voxy-0.2.20-beta.jar` (Modrinth, `fxxUqruK/iIrM8ZKQ`,
  game version 26.3).
- Full source: `analysis/voxy/` (MCRcortex/voxy, branch 26.3).
- Dependency found the hard way on a real client boot: **voxy 0.2.20-beta
  requires sodium 0.9.2/0.9.3** (fabric-loader refuses the mod set otherwise).

## 1. What a Voxy "LOD" actually is

Voxy does NOT render a heightfield. It renders real block geometry:

- `rendering/building/RenderDataFactory.java` — scans the world voxels and
  emits greedy quads per section (worklog v4.1: octree 32³→512³, 5 levels).
- Every quad carries:
  - the real block MODEL id (`quads.frag:getModelId` — a 768x512 tile atlas
    `blockModelAtlas` of actual block textures, sampled with mipmaps via
    `textureGrad`, alpha-discard for cutouts like leaves);
  - per-quad light = REAL world voxel light (block light 4 bit + sky light),
    `applyQuadLight` (RenderDataFactory.java:380) + `lighting.glsl`
    `getLightmapUv` — sampled from the LIVE vanilla lightmap texture
    (`LightMapHelper.getLightmapTextureId` =
    `Minecraft.getInstance().gameRenderer.levelLightmap().texture()`).
  - biome tint (`tintingState`, partial tint decided by the texture's own
    grayscale: `computeColour`).
  - the face id — normals are AXIS-ALIGNED per quad (`ssao.comp:face2norm`
    converts face→normal; there are NO smooth vertex normals).
- Data sources (`commonImpl/importers/`): `WorldImporter` (scans chunks the
  client has loaded) and `DHImporter` (imports Distant Horizons data). In
  0.2.20 there is NO seedgen — far LODs on a vanilla multiplayer server only
  exist after chunks were actually seen (or imported from DH).

## 2. The post pipeline (`client/core/NormalRenderPipeline.java`)

1. Terrain pass renders into an offscreen framebuffer.
2. **SSAO** runs over the whole LOD frame (`postOpaquePreTranslucent` →
   `SSAO.computeSSAO`, `post/ssao.comp`):
   - mode AUTO picks BASIC / BETTER(12 spp) / BEST(24 spp) by VRAM;
   - BETTER/BEST = photon-vision SSAO (sixthsurge/photon, "used with
     permission"), screen-space hemisphere sampling, radius 1.5 blocks,
     screen-clamped at 0.1, `pow(1 - ao/steps, 2.5)`;
   - BASIC = one reprojection probe: offset the pixel along the quad normal
     by 0.5, reproject through the depth buffer, angle → `pow(ao,0.15)` →
     `(1-ao)/3 + 2/3`;
   - applied as `colour.rgb *= ao`, gated per-pixel by the metadata bit 6
     (which quads even participate).
3. Blit with FOG + FADE (`finish`):
   - fog = vanilla environmental pair: `invEndFogDelta`, startDelta,
     `endDistance = max(vanillaRD, 320) * sqrt(3)` — c-LoD's marcher already
     replicates this byte-exactly (see lod_terrain.fsh comments);
   - fade = `FogMode.FADE MODE=1` (xz only), last 10% of the render distance
     (`sectionRenderDistance*16*32 - sqrt(...)`, start at 90%) — also already
     replicated (FADE_START = 0.9);
   - alpha blending onto the vanilla frame only where fog does not cover
     everything.

## 3. Why c-LoD reads flatter than Voxy today

| component | Voxy | c-LoD v6.7.3 |
|---|---|---|
| geometry | real block quads (1:1 near, greedy far) | raymarched heightfield, 2 m cells |
| color | real textures + biome tint, mipmapped | 6x6x6 lattice (216 colors) per cell |
| light | per-quad real voxel light → live lightmap | reconstructed lightmap(0,15) only |
| normals | axis-aligned per quad | flat per-cell gradient |
| AO | full-frame SSAO (BASIC/BETTER/BEST) | none |
| fog/fade | vanilla pair + 10% xz fade | identical (already ported) |

The last row is the ONLY fully closed gap. Everything above it needs either
client-side voxel data (textures, per-quad light — impossible without a mod)
or a faithful port of the SSAO/fog/AO recipe INTO the heightfield marcher.
Any such port must cite this file and the Voxy line numbers — no invented
constants (the rejected "v7.0 VOXY-GRADE" attempt died for exactly that).

## 4. The reference stand (scripts/bench/*voxy*)

- `setup_client_voxy.sh` — vanilla runtime (setup_client.py) + fabric-installer
  client-mode (loader 0.20.1) + pinned mods (fabric-api 0.165.0, sodium 0.9.2,
  voxy 0.2.20-beta) + deterministic voxy-config.json.
- `run_server_voxy.sh` — PLAIN Paper 26.3 (no c-LoD module: no pack push, no
  dust particles), seed `clodbench2026` — the same seed the c-LoD visual CI
  leg uses.
- `run_client_voxy.sh` — Xvfb + llvmpipe + sRGB shim; KnotClient launch;
  `MESA_GL_VERSION_OVERRIDE=4.6` + `MESA_GLSL_VERSION_OVERRIDE=460`
  (llvmpipe caps its GLSL compiler at 4.50 while Voxy ships `#version 460`).
- `run_voxy_shots.sh` — join, noon lock, spectator, the same vantage matrix
  as the c-LoD leg (disk center -930,796), reference screenshots to
  `~/mc-test/voxy-shots/`.
- CI: `.github/workflows/ci.yml` job `voxy-reference` (master + dispatch,
  `continue-on-error` until the comparison harness settles).

## 5. Verdicts so far (live client runs, 4 GB sandbox)

- Voxy boots and initializes on llvmpipe (GL 4.6 Core Profile, Mesa 25.0.7)
  ONLY with both Mesa overrides; without them the first compute shader dies
  (`GLSL 4.60 is not supported`).
- The 4 GB sandbox OOM-killed the client at ~2.3-2.5 GB RSS (Voxy native
  buffers + llvmpipe host memory) while meshing — hence the CI job carries
  generous memory knobs (VOXY_CLIENT_XMX=2500M, VOXY_RD=8) and the sandbox
  defaults stay tiny (700M/rd3).
- The reference matrix itself is the CI job's product: evidence lands in the
  `voxy-reference-shots` artifact; the marcher comparison reads it from
  there.
