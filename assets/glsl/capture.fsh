#version 330
#extension GL_ARB_separate_shader_objects : require
#include <minecraft:globals.glsl>

// c-LoD v6.2 SKY-GRID BIT-STREAM capture pass.
//
// The server streams LOD cells as villager-sparkle particles (the only
// texture the 26.3 software rasterizers draw — dust quads die with
// degenerate UVs, verified live on GL llvmpipe AND Vulkan lavapipe).
// v6.22 TIERED TRANSPORT: each cell's words ride its RANK planes — the
// HEIGHT word on TiersHi[k], the COLOR word on TiersLo[k] (k = the lowest
// tier strictly above the cell surface; rank 0 = the v5.1 120/80 pair):
// the carrier has one hue, so the capture pass tells words apart by plane
// height and walks the ladder per cell. One 16-bit word = 4 particles per
// plane: particle slot s (Z axis, msb-first) carries bits (14-2s)..(15-2s);
// its X sub-position inside the cell IS the 2-bit value, its Z sub-position
// IS the slot. Sub-cell quantum = cell/4, centered.
//
// This pass decodes ONE word back into its persistent 256x256 target (1 px
// = 1 cell). For the pixel's cell it re-projects all 16 sub-particle
// positions onto the screen (probe camera) and strobes every projection
// against the bright villager-green hue, then merges the decoded word into
// the previous value. Anything not strobed keeps its old value; the stream
// self-heals on the next lap.
//
// v6.2 EDIT PROTOCOL: the OR-merge can never LOSE bits, so a changed word
// would corrupt forever. Every edit therefore ships one RESET electron on
// a third plane (Y=100, between the data planes — mode 2 of this shader
// writes the clod_reset flag). A flagged cell REPLACES its word the first
// time the fresh decode completes (all four slots strobed); partial
// decodes keep the legacy OR self-heal, and once replaced, further ORs of
// the same word are idempotent for the rest of the electron's lifetime.
//
// v5.3: the old sky-depth rejection (dep < 1e-6) is GONE. 26.3 reversed-Z
// reads the bare sky as depth 0 and the carrier (translucent sheet) never
// writes depth, so the old gate silently blinded the decoder to every dot
// against the sky — the most visible ones, and the whole HI plane from
// ground views. The hue test alone is precise (sky is blue: g - b < 0),
// sky-background dots now decode AND get erased by the dust-mask pass.
// v5.3: the fresh THIS-frame word also rides the .b channel (never
// OR-merged) — dust_mask.fsh projects exactly those live sub-positions
// to build the eraser mask.

uniform sampler2D InSampler;      // minecraft:main color (dust hue strobe)
uniform sampler2D InDepthSampler; // minecraft:main depth (unused since v5.3, kept for the chain schema)
uniform sampler2D PrevSampler;    // previous plane target (persistent)
uniform sampler2D RotSampler;     // clod_rot_a (camera + fog channel)
uniform sampler2D ResetSampler;   // clod_reset (v6.2 edit-reset flags, this frame)
uniform sampler2D HeightRefSampler; // clod_h pair (v6.23 color-side rank cross-check)
uniform sampler2D HeightPersistSampler; // the OTHER clod_h half — the persistent marked word (v6.36.0)

// v5.0.3: SamplerInfo never existed as a client UBO — it stayed zeroed
// (the whole capture wrote cell (0,0), the march divided by zero).
// The screen size rides the STANDARD Globals block (ScreenSize), the
// target size is compile-time known.

// x = origin_x, y = origin_z, z = HIGH layer y, w = LOW layer y
layout(std140) uniform ClodGrid {
    vec4 GridInfo;
};

// x = cell size, y = sub quantum (cell/4), z = device depth near (data
// modes) or the RESET plane Y (mode 2), w = word (0 = HI, 1 = LO, 2 = reset)
layout(std140) uniform ClodStream {
    vec4 StreamInfo;
};

// v6.22 TIERED TRANSPORT: the rank ladders (must match src/stream.rs, which
// fills them through the ClodTiers UBO in every capture/dust_mask pass of
// the chain JSON). Rank k serves cells whose surface sits below TiersLo[k]:
// the COLOR word rides TiersLo[k], the HEIGHT word TiersHi[k] (Lo + 40),
// the edit-reset electron TiersRs[k] (Lo + 20). A dot at or below the
// terrain never renders, so the server picks the lowest rank strictly above
// the cell surface and this pass — which cannot know the height before the
// word decodes — WALKS the tiers ascending per cell (the walk IS the height
// discovery). Rank 0 = the v5.1 planes (80/120/100).
layout(std140) uniform ClodTiers {
    vec4 TiersLo; // x..w = rank 0..3 color-word planes
    vec4 TiersHi; // x..w = rank 0..3 height-word planes
    vec4 TiersRs; // x..w = rank 0..3 reset-electron planes
};

// The rank ladder member for rank r (explicit chain — desktop GLSL 330
// allows dynamic vector indexing, but the game's post-chain compiles on
// drivers where the scalar path is the safe one).
float tierY(vec4 ladders, int r) {
    return r == 0 ? ladders.x : r == 1 ? ladders.y : r == 2 ? ladders.z : ladders.w;
}

// v6.23 TIER-CONSISTENCY: the rank the SERVER would have picked for a
// surface (stream.rs::tier_rank mirrored EXACTLY — same ladder through the
// ClodTiers UBO the walk probes, same TIER_MARGIN 2.0, same saturation;
// the boundary windows are pinned stream-side by tier_rank_climbs_with_terrain).
// No new constants: the gate is the protocol's own rank rule applied to the
// stash itself.
int surface_rank(float surface) {
    float h = surface + 2.0;
    for (int r = 0; r < 4; ++r) {
        if (tierY(TiersLo, r) > h) {
            return r;
        }
    }
    return 3; // stream.rs unwrap_or(LO_TIERS.len() - 1) saturation
}

layout(location = 0) in vec2 texCoord;
layout(location = 0) out vec4 fragColor;

const float PI = 3.14159265358979;

// One cell word probe: re-project the cell's 16 sub-particle positions at
// `planeY`, strobe each against the carrier hue and OR the decoded 2-bit
// values into the word. anyHit = at least one sub-position strobed this
// frame; allSlots = every Z slot strobed (a COMPLETE word — the v6.2
// replace gate). The strobe is the v5.3.1 5-tap cross on the UN-FOGGED
// color; geometry must match stream.rs::word_spawns exactly.
// depthNear = the device depth near of THIS pass (StreamInfo.z on the data
// passes); <= 0 disables the v6.25 mirror arm's floating-carrier depth gate
// (the mode-2 reset pass does not carry the depth near and keeps the hue
// arms only).
void probe_word(vec2 cellWorld, float planeY, float sub, float depthNear, out uint word, out bool anyHit, out bool allSlots) {
    word = 0u;
    anyHit = false;
    allSlots = true;

    vec4 rotP = texelFetch(RotSampler, ivec2(0, 0), 0);
    float az = rotP.r * 2.0 * PI;
    float el = rotP.g * PI - PI * 0.5;
    float fov = 5.0 + rotP.a * 115.0;

    vec3 F = vec3(cos(el) * cos(az), sin(el), cos(el) * sin(az));
    vec3 R = vec3(-sin(az), 0.0, cos(az));
    vec3 U = vec3(-cos(az) * sin(el), cos(el), -sin(az) * sin(el));

    float tanH = tan(radians(fov) * 0.5);
    float aspect = ScreenSize.x / max(ScreenSize.y, 1.0);
    vec3 cam = vec3(CameraBlockPos) + CameraOffset;

    // v5.3 UN-FOG: the vanilla particle renderer fogs the carrier exactly
    // like terrain, so dots beyond the fog start fail the hue strobe (the
    // green channel sinks toward the blue fog color — measured live:
    // (112,149,139) instead of villager green). The fog state rides the
    // same camera channel (group B, identical to lod_terrain.fsh); the
    // loop un-fogs every strobe sample with the dot's own distance.
    vec4 fogCol = texelFetch(RotSampler, ivec2(1, 0), 0);
    vec4 fogRng = texelFetch(RotSampler, ivec2(0, 1), 0);
    bool fogLive = fogRng.a > 0.5 && fogCol.a > 0.004;
    float fStart = fogLive ? fogRng.r * 255.0 * 8.0 : 96.0;
    float fEnd = fogLive ? fogRng.g * 255.0 * 8.0 : 320.0;
    float vanRD = fogRng.b * 255.0 * 16.0;
    vec3 fogTint = fogLive ? fogCol.rgb : vec3(0.75, 0.85, 1.0);
    float fogOpacity = fogLive ? fogCol.a : 1.0;
    float invEndFogDelta = 1.0 / max(fEnd - fStart, 1.0);
    float startDelta = -fStart * invEndFogDelta;
    float endDistance = max(fogLive ? vanRD : 128.0, 320.0) * 1.7320508;
    float maxFog = clamp(endDistance * invEndFogDelta + startDelta, 0.0, 1.0);

    bool slot0 = false;
    bool slot1 = false;
    bool slot2 = false;
    bool slot3 = false;

    for (int qz = 0; qz < 4; ++qz) {
        for (int qx = 0; qx < 4; ++qx) {
            vec3 P = vec3(
                cellWorld.x + (float(qx) - 1.5) * sub,
                planeY,
                cellWorld.y + (float(qz) - 1.5) * sub
            );
            vec3 d = P - cam;
            float z = dot(d, F);
            if (z < 2.0) {
                allSlots = false;
                continue; // behind / too close
            }
            vec2 ndc = vec2(
                dot(d, R) / (z * tanH * aspect),
                dot(d, U) / (z * tanH)
            );
            if (abs(ndc.x) > 1.0 || abs(ndc.y) > 1.0) {
                allSlots = false;
                continue; // off screen
            }
            vec2 sc = ndc * 0.5 + 0.5;
            // STROBE v5.3.1: 5-tap cross (center + 4 axis taps at 0.75
            // texel) on the UN-FOGGED color. Tolerates +-0.75 px of camera
            // jitter / angular motion between the client's particle
            // projection and the rot-channel reconstruction (bilinear smear
            // widens effective coverage to ~1.5 px); first passing tap wins.
            // The dot's own distance z drives the same linear fog the
            // renderer applied; (1-f) is clamped so deep-fog noise never
            // explodes.
            vec2 texel = 1.0 / ScreenSize;
            // v6.5 FOG-INVARIANT RING: the mean (g-b) of the 4 diagonal
            // texels ~3 px out. Fog is a per-depth lerp toward one color, so
            // it scales the dot and its backdrop IDENTICALLY — the (g-b)
            // contrast between them survives as contrast * (1-f) while every
            // absolute hue threshold dies (measured on CI run 41260665463:
            // day-lit core (0.46, 0.54, 0.70) — b-dominant, the sky-blue fog
            // wins the absolute gate even after the un-fog when group B is
            // missing). Sky ring (g-b) ~ -0.35, fogged carrier ~ +0.4*(1-f):
            // the margin 0.06 holds to f ~ 0.85, far past the decode band.
            // The mean damps a neighboring dot landing in one diagonal
            // (sub-quantum spacing ~3.5 px at 75 m: one polluted tap of
            // four costs a quarter of its contrast).
            vec2 scrPx = vec2(ScreenSize);
            ivec2 basePx = ivec2(clamp(sc * scrPx, vec2(0.0), scrPx - 1.0));
            float ringGB = 0.0;
            float ringLum = 0.0;
            for (int d = 0; d < 4; ++d) {
                ivec2 o = d == 0 ? ivec2(3, 3)
                        : d == 1 ? ivec2(-3, 3)
                        : d == 2 ? ivec2(3, -3)
                        : ivec2(-3, -3);
                ivec2 q = clamp(basePx + o, ivec2(0), ivec2(scrPx) - 1);
                vec3 s = texelFetch(InSampler, q, 0).rgb;
                ringGB += s.g - s.b;
                ringLum += s.r + s.g + s.b;
            }
            ringGB *= 0.25;
            ringLum *= 0.25 / 3.0;
            // v6.25 FLOATING-CARRIER DEPTH GATE: the carrier is translucent
            // and never writes depth, so the depth behind a LIVE dot is the
            // terrain BEHIND the plane (or the sky, raw ~ 0 -> zScene huge).
            // A dark px the ray reaches BEFORE the plane is terrain shading
            // (a terrace shadow writes depth closer) and must never fire the
            // mirror arm — the same one-line proof the G8d march uses
            // (tVan = DEPTH_NEAR/depth). Sampled once at the projected
            // center; the 5-tap cross stays a color test.
            float zScene = depthNear / max(texture(InDepthSampler, sc).r, 1.0e-6);
            float fog = clamp(z * invEndFogDelta + startDelta, 0.0, maxFog) * fogOpacity;
            bool dust = false;
            for (int t = 0; t < 5; ++t) {
                vec2 off = t == 0 ? vec2(0.0)
                    : t == 1 ? vec2(0.75 * texel.x, 0.0)
                    : t == 2 ? vec2(-0.75 * texel.x, 0.0)
                    : t == 3 ? vec2(0.0, 0.75 * texel.y)
                    : vec2(0.0, -0.75 * texel.y);
                vec3 raw = texture(InSampler, sc + off).rgb;
                vec3 pc = (raw - fogTint * fog) / max(1.0 - fog, 0.15);
                // v6.5 two arms: the v5.3.1 absolute hue test on the
                // un-fogged color (precise when the group B fog payload is
                // alive) OR the fog-invariant local contrast (immune to a
                // dead payload and to the cylindrical-vs-forward fog model
                // mismatch). Neutrals fail the second arm on g > r; sky
                // fails on contrast ~ 0 (the ring IS the backdrop).
                // v6.20.0 UNIVERSAL LOCAL-CONTRAST GATE (#305 forensics):
                // BOTH arms now require the fog-invariant ring delta. The
                // absolute arm alone fired on every grass/foliage pixel a
                // probe landed on — vanilla grass reads g ~ 0.49 and passes
                // g > 0.40 && g > r + 0.08 && g > b + 0.08 — a standing
                // false-dot field the OR-merge locked into the persistent
                // words; the march then painted the garbage bytes as the
                // #305 far-band towers (black lattice-0 cells, wrong
                // heights, the purple = the 2x2 color average of red fill
                // next to false water). The §5b lanes stayed clean only
                // because the synthetic palette's green (g = 0.40) misses
                // the absolute gate by 0.005. The ring delta is the dot's
                // real signature, fog-invariant (fog scales the dot and
                // the ring identically): +0.7 over sky, +0.13 over grass,
                // +0.4 over stone, +0.83 over water; a backdrop false
                // positive reads 0.
                bool delta = (raw.g - raw.b) - ringGB > 0.06;
                bool absolute = delta && pc.g > 0.40 && pc.g > pc.r + 0.08 && pc.g > pc.b + 0.08;
                bool relative = delta && raw.g > raw.r + 0.02;
                // v6.25 MIRROR LUMINANCE-DELTA ARM (#318/#319 forensics):
                // the §7b canopy shades the rank-0/1 carriers — the particle
                // samples the blocked skylight and renders NEUTRAL DARK
                // (measured on the #319 mask8 10m frame: (35,38,42), 100% of
                // the dot class inside |g-b| <= 0.08 and |g-r| <= 0.08, lum
                // 0.15 vs ring 0.41) — INVISIBLE to both hue arms, so those
                // cells never decode and their dots are never erased: the
                // eastern fake wall. The arm is the v6.5 ring gate with the
                // sign flipped on LUMINANCE (the same 0.06, fog scales dot
                // and ring identically) + the measured neutrality window
                // (0.08 = the absolute arm's own margin) + the depth gate
                // above. The §5b lanes are untouched: their dots pass the
                // hue arms first, their terrain has no dark-on-bright
                // neutral edges at the probe positions, and the tier
                // windows + merge honesty still bound every stash.
                float lum = (raw.r + raw.g + raw.b) * (1.0 / 3.0);
                bool mirror = depthNear > 0.0
                    && (ringLum - lum) > 0.06
                    && abs(raw.g - raw.b) <= 0.08
                    && abs(raw.g - raw.r) <= 0.08
                    && zScene >= z;
                if (absolute || relative || mirror) {
                    dust = true;
                    break;
                }
            }
            if (!dust) {
                allSlots = false;
                continue;
            }
            word |= uint(qx) << (14 - 2 * qz);
            anyHit = true;
            if (qz == 0) slot0 = true;
            else if (qz == 1) slot1 = true;
            else if (qz == 2) slot2 = true;
            else slot3 = true;
        }
    }
    // v6.20.0 ROW-COMPLETE = the word is complete: every slot's dot was
    // found in THIS frame (each found dot contributes its 2-bit column, so
    // a row without its dot decodes as column 0 = zero bits — garbage the
    // old allSlots could never catch: it ANDed the 16-probe grid where 12
    // probes are structurally empty and was therefore always false, which
    // also silently killed the v6.2 fresh-replace edit path). The old mark
    // = anyHit stamped a PARTIAL first decode as decoded forever — the OR
    // cannot heal rows whose dots are swallowed by terrain above the
    // stream planes (LAYER_Y = 80/120 vs the §7b world's 190 m summits),
    // so those cells painted garbage bytes for the whole session. The mark
    // now flips only on a row-complete frame.
    allSlots = slot0 && slot1 && slot2 && slot3;
}

void main() {
    ivec2 px = ivec2(clamp(texCoord, 0.0, 1.0) * vec2(256.0));
    px = clamp(px, ivec2(0), ivec2(vec2(256.0)) - 1);

    float cell = StreamInfo.x;
    float sub = StreamInfo.y;
    vec2 cellWorld = GridInfo.xy + (vec2(px) + 0.5) * cell;

    // Probe camera gate: the LIVE camera from the sky camera-channel
    // (clod_rot_a, written by the sky override every frame): R = az/2pi,
    // G = (el+pi/2)/pi, B = conf, A = (fov-5)/115*254. The v5.0.0
    // hard-coded bench pose (yaw=180, pitch=-5, fov=70) decoded NOTHING
    // the moment the player turned — the projections missed every particle.
    vec4 rotP = texelFetch(RotSampler, ivec2(0, 0), 0);
    // ---- v6.2 mode 2: the edit-reset plane --------------------------------
    // Probe the cell's 16 sub-positions at the reset Y (StreamInfo.z for
    // this pass) and flag the cell when ANY of them strobes. A reset
    // electron is a COMMAND, not data: one dot suffices and partial
    // strobes mean the same thing — unlike data words, nothing here may
    // OR-accumulate. A blind camera flags nothing (the data captures
    // decode nothing either).
    if (StreamInfo.w >= 1.5) {
        if (rotP.b < 0.25) {
            fragColor = vec4(0.0, 0.0, 0.0, 1.0);
            return;
        }
        // v6.22: the electron rides the cell's RANK reset plane; the walk
        // probes the ladder ascending. A reset electron is a COMMAND, not
        // data: one dot suffices and partial strobes mean the same thing —
        // unlike data words, nothing here may OR-accumulate. A blind camera
        // flags nothing (the data captures decode nothing either).
        uint w;
        bool hit = false;
        bool all;
        for (int r = 0; r < 4 && !hit; ++r) {
            // depthNear = 0: the reset pass's UBO carries RESET_Y, not the
            // depth near — the electrons keep the hue arms (v6.25).
            probe_word(cellWorld, tierY(TiersRs, r), sub, 0.0, w, hit, all);
        }
        fragColor = vec4(hit ? 1.0 : 0.0, 0.0, 0.0, 1.0);
        return;
    }
    if (rotP.b < 0.25) {
        // v5.3.1 BLIND-CAMERA GATE: the camera channel is measured from sky
        // pixels and its confidence decays once no sky is in view. Decoding
        // with a stale pose would OR random hue noise into the persistent
        // words and stamp garbage into the eraser mask. Pass the previous
        // state through instead; the stream self-heals on the next sky view
        // (confidence jumps back to 1 on the first valid row, frame-exact).
        // The server-side spawn gate (stream.rs spawn_visible) stops feeding
        // dots into sky-blind frustums, so there is nothing to decode here
        // anyway — this gate keeps the buffers honest while conf decays.
        fragColor = texelFetch(PrevSampler, px, 0);
        return;
    }

    // v5.1 -> v6.22: one plane per pass becomes a TIER WALK. The server
    // spawns the cell's words on the lowest rank whose planes sit strictly
    // above the terrain (a dot inside a block never renders), so the client
    // probes the ladder ascending. The first strobe of the walk becomes the
    // OR candidate; the first ROW-COMPLETE tier overrides it and stops the
    // walk. For every rank below the true one the LO plane sits at/below
    // the surface — zero same-cell strobes — so a lower-tier partial can
    // only be cross-cell noise, which the complete word must replace. Rank
    // 0 exits after one probe: the §5b worlds keep the exact v5.1 cost.
    //
    // v6.23 TIER-CONSISTENCY GATE (#316 forensics): the walk's "zero
    // same-cell strobes" reasoning holds for REAL dots but NOT for strobe
    // noise — the relative arm fires on every terrain hue edge the probes
    // land near (center on grass, the +-3 px ring on stone: delta > 0.06),
    // and the §7b real world is built of grass-terrace-on-stone edges.
    // For a mountain cell the lower tiers' planes sit INSIDE the terrain:
    // their probe grids land on the terraced slopes and fire on every
    // grass/stone boundary. The v6.22 stash turned each such fire into an
    // immediate OR write into the previously-clean word, and the OR
    // ratchets (it can only set bits): honest summit bytes 249..254
    // climbed to the 255 ceiling = the 191 m columns the #316 aerials
    // show as the garbage stripe field (the color words ratcheted onto
    // partial lattice words = the purple wedge). THE GATE: a stash is
    // accepted only if the rank it decoded from equals the rank the
    // protocol itself derives from the word (HI: the byte's own surface;
    // LO: the height pair the earlier captures of this frame wrote — both
    // words ride ONE rank by construction, stream.rs push_cell_spawns).
    // A real word always passes (the server used this very rule to pick
    // the plane); noise passes only inside its rank's byte window.
    uint word = 0u;
    bool anyHit = false;
    bool allSlots = false;
    uint stash = 0u;
    bool stashed = false;
    bool stashAll = false;
    vec4 ladders = StreamInfo.w < 0.5 ? TiersHi : TiersLo;
    bool hiPass = StreamInfo.w < 0.5;
    for (int r = 0; r < 4; ++r) {
        uint w;
        bool hit;
        bool all;
        probe_word(cellWorld, tierY(ladders, r), sub, StreamInfo.z, w, hit, all);
        if (!hit) {
            continue;
        }
        bool honest;
        if (hiPass) {
            // encode_words: height byte = (surface + 64).clamp(0, 255) —
            // cells above the byte ceiling were never encodable; their
            // decoded rank reads 2 and their rank-3 spawn is rejected
            // (they stay void — the honest outcome for unknowable data).
            honest = surface_rank(float(w >> 8) - 64.0) == r;
        } else {
            // The color word carries no surface: cross-check against the
            // height pair. v6.36.0: the FRESH HI word of this frame dies to
            // the eraser the frame its completion lands (v6.35.0's eraseFlag
            // fires on allSlots || marked) — every frame AFTER the height
            // completion read an empty HeightRef and the rank cross-check
            // rejected the color word for exactly the cells whose heights
            // were already decoded: the color window was ONE frame wide,
            // and the #381-#383 tower frames' RED class (a ZERO color byte
            // under a marked height — 84-150k px, replicated 4 runs) is its
            // portrait. The PERSISTENT height half (the other ping-pong
            // buffer — every frame's HI round merges into it and its mark
            // survives the eraser) vouches instead: the rank gate keeps its
            // full noise-rejection power, the vouch no longer expires with
            // the dots.
            vec4 hpx = texelFetch(HeightRefSampler, px, 0);
            if (hpx.a < 0.5) {
                hpx = texelFetch(HeightPersistSampler, px, 0);
            }
            honest = hpx.a >= 0.5 && surface_rank(hpx.r * 255.0 - 64.0) == r;
        }
        if (!honest) {
            continue;
        }
        if (all) {
            stash = w;
            stashed = true;
            stashAll = true;
            break;
        }
        if (!stashed) {
            stash = w;
            stashed = true;
        }
    }
    word = stash;
    anyHit = stashed;
    allSlots = stashAll;

    vec4 prev = texelFetch(PrevSampler, px, 0);
    uint prevByte = uint(prev.r * 255.0 + 0.5);
    bool reset = texelFetch(ResetSampler, px, 0).r > 0.5;
    uint fresh = word >> 8;
    // v6.38.0 THE SLOT-COVERAGE LEDGER (the color-word completion fix):
    // allSlots demands all four slots' dots strobing in ONE frame — but
    // the two words of one cell lose dots INDEPENDENTLY (pool churn, fog
    // margins, particle lifetimes), and the tower state frames (#381-#386)
    // show the result: the color word's bits OR-accumulate across frames
    // (the green minority grows with dwell) while the mark never lands,
    // because the mark rides allSlots-only. The ledger rides the state
    // green channel (the v6.35.0 one-bit erase flag widened to a nibble):
    // the low 4 bits OR-accumulate every frame's strobed slots, bit 4
    // stays the eraser's kill flag (dust_mask reads the bit — the nibble
    // is invisible to it). A slot counts as covered when any of its dots
    // strobed THIS frame. The ledger complete = the word IS decoded: the
    // same guarantee the one-frame allSlots gave, spread over however
    // many frames the dots needed. Byte-stability: every §5b cell
    // completes allSlots in its first frame (ledger 0x0F + kill bit the
    // same frame the mark lands — byte-identical steady state), and the
    // ledger resets with the reset electron so a re-edited cell can never
    // inherit the old word's coverage.
    uint covered = uint(prev.g * 255.0 + 0.5) & 0x0Fu;
    uint slotBits = (slot0 ? 0x1u : 0u) | (slot1 ? 0x2u : 0u)
                  | (slot2 ? 0x4u : 0u) | (slot3 ? 0x8u : 0u);
    bool marked = prev.a > 0.5;
    uint outByte = prevByte;
    bool ledgerDone = false;
    if (reset) {
        // v6.2 EDIT PROTOCOL: a reset electron wipes the OR-merged word's
        // ledger with it — the fresh decode starts from a clean coverage
        // history (a complete decode REPLACES the byte, a partial one
        // seeds the new ledger; the old byte stays visible until the new
        // word completes, exactly as v6.2 intended).
        covered = slotBits;
        if (allSlots) {
            outByte = fresh;
            ledgerDone = true;
        }
    } else if (allSlots && !marked) {
        // v6.23 MERGE HONESTY (a): the first honest completion REPLACES
        // the accumulated partial bits. The fresh word is complete by
        // construction (every slot fired this frame) and every honest
        // partial is a subset of it — replacing loses nothing real and
        // throws away the noise partials the OR had already ratcheted in.
        outByte = fresh;
        covered = 0x0Fu;
        ledgerDone = true;
    } else if (anyHit && !marked) {
        // v6.2 (legacy, scoped): partial decodes keep the OR self-heal —
        // on UNMARKED cells only — and v6.38.0 lets the coverage history
        // finish the row the one-frame allSlots could not.
        outByte = prevByte | fresh;
        covered |= slotBits;
        if (covered == 0x0Fu) {
            ledgerDone = true;
        }
    }
    // v6.23 MERGE HONESTY (b): a MARKED cell's word is immutable without
    // the reset electron. Honest same-word traffic is unaffected (OR of a
    // word with itself is the identity — the §5b lanes stay byte-identical);
    // what dies is the ratchet: the walk's lower-tier noise partials could
    // OR bits into a proven word every frame (249 -> 255 = the #316
    // stripes); with the gate above they are rejected, and this block
    // makes the refusal permanent for the whole marked lifetime.
    // v6.38.0: the ledger-complete completion raises the completion flag —
    // the mark and the erase behavior follow the existing paths.
    if (ledgerDone) {
        allSlots = true;
    }
    float mark = allSlots ? 1.0 : prev.a;
    // v6.35.0 THE ERASE FLAG = COMPLETION, not anyHit — and v6.38.0 widens
    // the one-bit flag into the slot-coverage ledger: bit 4 stays the
    // eraser's kill flag (dust_mask now tests the bit, not the byte), the
    // low nibble OR-accumulates the strobed slots across frames until the
    // row completes. Complete words kill their dots as before (the §5b
    // steady state: allSlots every frame — ledger 0x0F + kill bit the
    // same frame, byte-identical rhythm), already-marked words too, and a
    // PARTIAL UNMARKED word's dots survive (kill bit 0) while the nibble
    // keeps the coverage alive through the ping-pong.
    float eraseFlag = float(((allSlots || prev.a > 0.5) ? 0x10u : 0u) | covered) / 255.0;
    // .b = the FRESH word (this frame, not merged) for the eraser mask.
    fragColor = vec4(
        float(outByte) / 255.0,
        eraseFlag,
        float(fresh) / 255.0,
        mark
    );
}
