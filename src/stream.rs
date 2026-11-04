//! The streaming core: SKY-GRID BIT-STREAM (v5.0).
//!
//! Every byte of LOD data travels to the vanilla client as DUST PARTICLES
//! spawned above the scan disk — no map carriers, no offhand item, no
//! textures in the pack at all.
//!
//! Two particle planes float over the disk:
//!   HI (Y=260) — the 8-bit height word of the cell (h + 64, clamped 0..255);
//!   LO (Y=272) — the 8-bit color index of the cell (6x6x6 RGB lattice
//!   0..215, water = 216) in the HIGH byte of the word.
//!
//! One 16-bit word encodes EIGHT data bits as FOUR particles:
//!   particle slot s (0..3, Z axis) carries bits (14 - 2*s) .. (15 - 2*s);
//!   its X sub-position inside the cell IS the 2-bit value, its Z
//!   sub-position IS the slot. The sub-cell quantum is cell/4, centered.
//!   (The low byte of the word stays zero — the decoder reads the high
//!   byte only, keeping the codec dead simple and symmetric.)
//!
//! Server-side pacing: `tick_batch()` returns the next chunk of spawns
//! (at most `particles_per_tick`), marching the disk row-major, valid
//! cells only. A full 256x256 disk at 1024 particles/tick (128 cells =
//! 4 + 4 planes) takes 512 engine ticks — one lap every ~26 s at the
//! default 50 ms cadence, continuously refreshing the client's picture.
//!
//! The client-side decoder (`assets/glsl/capture.fsh`) re-projects all 16
//! sub-cell positions of a cell onto the screen, strobes them against the
//! depth buffer (dust renders on the OPAQUE layer and writes depth; sky
//! reads 1.0), and OR-merges decoded words into persistent ping-pong
//! targets. A particle that never made it through the strobe simply keeps
//! the previous value — the stream self-heals on the next lap.
//!
//! v5.4 DELTA-ZONE scheduler: the client's buffers are PERSISTENT, so a
//! cell only needs re-streaming when (a) its words CHANGED on the server
//! (the dirty phase), or (b) it is decodable right now — inside the decode
//! band and some player's frustum — and has not been sent for
//! VIEW_REFRESH_TICKS (cone phase for view priority, then the zone sweep
//! row-major). Cells no player can decode were always culled at delivery;
//! now they cost ZERO budget. Steady state with a painted zone: the batch
//! collapses to a few dozen particles per tick — a ~25x idle-traffic cut
//! versus the v5.3.3 constant lap, with edits still propagating in
//! seconds. Passing NO views keeps the legacy full-disk lap (diagnostics,
//! no-player self-heal).
//!
//! v6.2 LIVE EDITS: the client's buffers OR-merge decoded words, so a
//! changed cell word can never LOSE bits on refresh — an edit must be
//! REPLACED atomically. Every dirty stream therefore carries one extra
//! RESET electron on a third sky plane (Y = RESET_Y, between the two
//! data planes): the capture's reset pass flags that cell for the frame,
//! and the data captures swap OR-merge for a direct write once the fresh
//! word decodes complete (all four slots strobed). Partial decodes keep
//! the legacy OR self-heal; a complete decode after a reset is idempotent
//! while the reset electron lives out its particle lifetime.
//!
//! v6.6 ALL-LOD FAIRNESS: the cone phase served `views[0]` only — with N
//! players online, N-1 of them got no frustum priority and their turn
//! refills waited on the 60 s zone sweep. The cone budget now rotates
//! ROUND-ROBIN across the audience: one view owns the cone budget per
//! tick (the same subjective fill speed as the single-player case), the
//! cursor hands it to the next view on the next tick, and every tracked
//! view keeps its own rebuilt cone cache (matched by eye/look drift, so
//! player-list order can never break reuse). ALL LODs stream, and the
//! wire serves every client fairly.

/// Sub-cell lattice: 4 positions per axis inside one cell.
pub const SUB_PER_CELL: f64 = 4.0;
/// Particle layers: the HEIGHT word rides the high layer, the COLOR word
/// the low one (v5.1 — one carrier hue, words told apart by plane height).
/// A layer is decodable when the particle renders in front of geometry
/// (26.3 reversed-Z hides particles against the sky), so the low layer
/// decodes from the ground, the high one from elevated vantage points.
pub const LAYER_Y: [f64; 2] = [80.0, 120.0];
/// Legacy plane constants (kept for packgen templating of the layer list).
pub const PLANE_HI_Y: f64 = LAYER_Y[1];
pub const PLANE_LO_Y: f64 = LAYER_Y[0];
/// Water color index in the 6x6x6 lattice.
pub const COLOR_WATER: u16 = 216;

/// v6.2 edit-reset plane: midway between the two data planes. Same 3D
/// eye distance as the average data dot (the spawn gate and the decode
/// band treat it identically), but its dots project away from every data
/// probe point — the data captures never see them and the dedicated
/// reset pass never sees data dots. Only DIRTY streams spawn here.
pub const RESET_Y: f64 = 100.0;

/// v5.3 proximity culling: particles never spawn within this radius (blocks,
/// 3D, eye-centered) of a player. MUST stay well below the capture's decode
/// zone (~28..96 blocks — beyond ~96 the fog washes the hue below the strobe
/// threshold and the cell stops decoding; the v5.3.0 64-block cull killed
/// the whole transport live on the bench). Near dots are the biggest, so the
/// eraser mask discs stay small; the dome's near gate paints nothing inside
/// the vanilla chunk bubble anyway.
pub const CULL_RADIUS: f64 = 28.0;

/// v5.3.1 STEALTH far gate: particles never spawn FARTHER than this (blocks,
/// 3D, eye-centered) from a player either. Beyond ~96 blocks the fog-washed
/// hue stops decoding (the capture's un-fog can only stretch so far), so a
/// far dot is PERMANENTLY visible garbage: rendered (force mode survives the
/// client's render-distance clip) but never decoded, hence never erased by
/// the dust-mask pass — exactly the sky specks seen from high vantages.
/// The persistent capture buffers make the disk cumulative: as players move,
/// the zone sweeps the grid and every cell still decodes within seconds.
pub const ZONE_FAR: f64 = 112.0;

/// v5.3.1 view-frustum spawn gate. The client's vertical FOV is a video
/// setting (30..110); the server cannot read it, so the cone assumes the
/// widest setting plus a margin. Dots outside the cone would be rendered
/// (force mode) but never looked at — invisible waste at best, and in
/// sky-blind views (looking down at water: no sky pixels -> the rot channel
/// decays -> decode AND eraser die) they would be plainly visible.
/// The margin keeps the gate safe: every dot that COULD land on someone's
/// screen is spawned; the extra cone is invisible waste only.
pub const ASSUMED_FOV_DEG: f64 = 110.0;
pub const FRUSTUM_MARGIN_DEG: f64 = 8.0;

/// v5.3.1 sky-blind gate: the camera channel is measured from SKY pixels
/// (rot_solve), and its confidence decays once no sky is in view. The eraser
/// shares the same camera, so dots spawned into a sky-blind frustum are
/// never erased. The server knows the player pitch exactly (Bukkit syncs it
/// every tick): treat the view as sky-blind when even the frustum's TOP edge
/// (pitch + ASSUMED_FOV/2 + margin) stays below the horizon. The 47° limit
/// bakes in an 8° terrain-silhouette margin for mountain bowls.
pub const SKY_PITCH_LIMIT_DEG: f32 = 47.0;

/// Cosine of the gate half-cone (ASSUMED_FOV/2 + FRUSTUM_MARGIN).
pub fn cos_half_cone() -> f64 {
    let half = (ASSUMED_FOV_DEG * 0.5 + FRUSTUM_MARGIN_DEG).to_radians();
    half.cos()
}

/// Bukkit yaw/pitch (degrees) -> unit look vector.
/// yaw 0 = +Z (south), 90 = -X (west); pitch -90 = up, +90 = down.
pub fn look_dir(yaw_deg: f32, pitch_deg: f32) -> [f64; 3] {
    let yaw = f64::from(yaw_deg).to_radians();
    let pitch = f64::from(pitch_deg).to_radians();
    let cp = pitch.cos();
    [-yaw.sin() * cp, -pitch.sin(), yaw.cos() * cp]
}

/// v5.3.1 spawn visibility gate for ONE dot against ONE player.
/// A dot is spawned to this player only when the player can actually SEE it
/// AND the client machinery can decode AND erase it:
///   1. sky in view (else the rot channel is dying -> no decode, no erase);
///   2. inside the decode band [CULL_RADIUS, ZONE_FAR] (3D, eye-centered);
///   3. inside the frustum cone (widest-FOV assumption + margin).
///
/// Every spawned dot is therefore on-screen, decodable and erased — the
/// transport is invisible from every vantage, sky-blind views included:
/// there is simply nothing there to see.
pub fn spawn_visible(
    s: &Spawn,
    eye: [f64; 3],
    look: [f64; 3],
    sky_visible: bool,
    cos_half: f64,
) -> bool {
    if !sky_visible {
        return false;
    }
    let dx = s.x - eye[0];
    let dy = s.y - eye[1];
    let dz = s.z - eye[2];
    let d2 = dx * dx + dy * dy + dz * dz;
    if d2 <= CULL_RADIUS * CULL_RADIUS || d2 > ZONE_FAR * ZONE_FAR {
        return false;
    }
    let inv = 1.0 / d2.sqrt();
    (dx * look[0] + dy * look[1] + dz * look[2]) * inv > cos_half
}

#[derive(Clone, Copy, Default, PartialEq)]
pub struct Cell {
    pub height: i16,
    pub water: bool,
    pub valid: bool,
    /// SEEDGEN v5.7: filled by native off-thread generation (noise+surface,
    /// no features/carvers). The loaded-chunk sampler upgrades provisional
    /// cells from the real chunk when the server generates it.
    pub provisional: bool,
    /// Surface albedo (palette.rs).
    pub rgb: [u8; 3],
}

impl Cell {
    /// The 8-bit color index (6x6x6 RGB lattice, water=216).
    pub fn color_index(&self) -> u16 {
        if self.water {
            return COLOR_WATER;
        }
        let r = (self.rgb[0] / 51).min(5) as u16;
        let g = (self.rgb[1] / 51).min(5) as u16;
        let b = (self.rgb[2] / 51).min(5) as u16;
        r + g * 6 + b * 36
    }
}

/// One villager-sparkle particle to spawn this tick (v5.1 carrier: the
/// only particle texture the 26.3 software rasterizers actually draw).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Spawn {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// v5.3.3 cone-priority view hint (the SPEED half of the stealth transport).
/// v5.3.1/v5.3.2 made every spawned dot decodable and erasable, but the
/// batch still walked the disk row-major: after a 180° turn the cells now
/// inside the frustum meet the cursor only half a lap later (~2-8 s of dead
/// sky). The scheduler fixes exactly that: cells inside the decode band AND
/// the frustum cone stream FIRST (stalest first, near-first), so the visible
/// zone refills within seconds of any turn — DH-class subjective fill speed
/// at the same particle budget. The per-player spawn gate keeps filtering
/// the legacy lap untouched (other players, view=None).
#[derive(Clone, Copy, Debug)]
pub struct ViewHint {
    /// Player eye position (world, blocks; by + 1.62).
    pub eye: [f64; 3],
    /// Unit look vector (stream::look_dir of the live yaw/pitch).
    pub look: [f64; 3],
    /// Engine tick of this call (age bookkeeping).
    pub now: u64,
    /// v6.5.3: ticks since THIS client joined (0 = brand-new join). The
    /// edit window must never arm while a fresh client is the audience —
    /// its particle pool is empty and cone/zone are its ONLY paint source
    /// (run #205: the window starved the initial paint and leg 2 regressed).
    pub age_ticks: u64,
}

/// A decodable cell (cone or zone sweep) is re-streamed at most once per
/// this many ticks — the client's persistent buffers still hold the dots'
/// words, so fresher refreshes are pure waste. This gates the CONE phase;
/// its value bounds the worst-case turn-refill latency (~3 s), the DH-class
/// speed the transport is tuned for.
pub const VIEW_REFRESH_TICKS: u64 = 60;

/// v5.4: the zone sweep's age gate — a much slower background self-heal
/// (packet loss, cleared client buffers). Edits do NOT wait for it (the
/// dirty phase), and turn refills do NOT wait for it (the cone phase).
pub const ZONE_REFRESH_TICKS: u64 = 600;

/// v6.5.3 EDIT WINDOW: once a decodable dirty cell streamed, the cone/zone
/// phases go SILENT for this many ticks. Why: run #204 proved the vanilla
/// client's particle pool SATURATES under the steady-state refresh stream
/// (thousands of dots/tick vs the engine's particle cap) and then REJECTS
/// new spawns — an edit's reset electron + fresh dots arrived into a full
/// pool, never rendered, and the marker became undeliverable on the
/// long-lived client (leg 1) while a FRESH client decoded it instantly
/// (leg 2 PASS). A ~10 s quiet window drains the pool (dots live 3-5 s),
/// so the edit batch renders into free slots. Re-armed only while decodable
/// dirty cells actually stream — an undecodable backlog never starves the
/// steady state.
pub const EDIT_WINDOW_TICKS: u64 = 200;

/// v6.5.3: beat 1 — the wire goes silent this many ticks BEFORE the edit
/// batch releases, so the client's saturated particle pool drains (dots
/// live 3-5 s) and the batch renders into free slots (run #206: the batch
/// went out into the still-full pool and never rendered).
pub const EDIT_HOLD_TICKS: u64 = 60;

/// v6.5.3: more decodable dirty cells waiting than this is a WAVE (the
/// boot-time provisional upgrades, a forceload) — waves feed the initial
/// paint and must never trigger the hold/release sequence.
pub const EDIT_HOLD_MAX_BATCH: u64 = 100;

/// v6.6.1 EDIT RECENCY: only dirty cells YOUNGER than this participate in
/// the hold/release decision. Why: the v6.6 CI visual run measured the
/// boot-era backlog (~12k seedgen-upgrade cells) still sitting in the
/// dirty queue at fill time — the queue-wide decision scan counted its
/// decodable members (> 100) and classified the GENUINE 25-cell marker
/// edit as a wave, so the hold/release drain never ran and the edit dots
/// went out into the still-saturated client pool (red=17 px on every
/// probe, while a fresh relog decode painted red=167 — the words were
/// right, the silence was missing). Backlog cells are wave material by
/// definition: they already waited minutes, 20 s more of the steady
/// state costs nothing. A real edit enqueued in the last 200 ticks
/// (20 s at 10 tps) re-arms the window even with a huge backlog.
pub const RECENT_EDIT_TICKS: u64 = 200;

/// v6.6.2 REBEAT, v6.7.1 MULTI-BEAT: each retry beat of the same edit
/// fires this many ticks after the previous release. Why: the CI visual
/// run 41328418141 exposed the transport's single-point fragility — a
/// SPARSE color word (RED_CONCRETE = index 2 = 0x0200) has exactly ONE
/// active slot-dot of four; if that one dot is lost (pool rejection,
/// strobe, fog), the client decodes color index 0 (black) and the column
/// renders fogged-black/navy while its HEIGHT word (0xEA00, three active
/// dots — loss-tolerant under OR-merge) paints a tall column in the WRONG
/// color on every client, fresh ones included.
/// Each beat re-dirties the just-streamed edit cells, cuts the post
/// silence, and reruns the whole hold→release sequence — every beat is a
/// complete-decode chance with a fresh reset electron. The beat COUNT is
/// `edit_beats` (v6.7.1, default 2 retries → three total chances): the
/// per-dot loss measured ~p=0.5 across runs, so the miss probability
/// falls from p to (1-p)^K — K=2 ≈ 0.75 decode, K=3 ≈ 0.875. Measured:
/// run 1 decoded red, run 2 missed the same dot — a coin flip per beat.
pub const REBEAT_DELAY_TICKS: u64 = 40;

/// v6.7.1 MULTI-BEAT: how many RETRY beats a genuine edit gets after its
/// release (the total decode chances are edit_beats + 1). 0 restores the
/// v6.6.1 single-shot edit; 1 is the v6.6.2 rebeat. Each retry re-runs
/// hold→release with a fresh reset electron, so a beat lost to a
/// phantom-complete partial write (the reset landed, the sparse color dot
/// did not) is repaired by the NEXT beat's reset + complete decode. The
/// cap on the knob (0..=8) keeps the edit episode bounded (~101 ticks per
/// beat at the default pacing).
pub const EDIT_BEATS_DEFAULT: u32 = 2;
pub const EDIT_BEATS_MAX: u32 = 8;
/// Cap on pending rebeat entries (the marker edits are tens of cells).
pub const REBEAT_MAX: usize = 512;

/// v6.9.5 RELOG RE-ARM: how long after a NEW session joins the streamer
/// keeps the last genuine edit's cells cycling through the rebeat queue.
/// Why: run #235 settled the relog failure — LEG 1 painted the marker
/// (red=1604) while EVERY relog probe read red=0, with the camera channel
/// correct (az/el/cam buckets frame-exact) and dots demonstrably flowing
/// (strobe/mask hits in the sweep windows). The difference is the delivery
/// protocol, not the data: LEG 1's marker went through the edit window
/// (hold: the wire silences, the client's saturated particle pool drains,
/// release: the dots land into the EMPTIED pool), while the relog client
/// only ever saw plain cone/zone re-streams — its pool refills to
/// saturation within seconds of the join and after that every new spawn
/// loses the slot race, so the marker words never strobe again. The one
/// rebeat window that happened to be pending at the join fired while the
/// fresh client's camera channel was still blind (probe 0: cam "?") — a
/// coin toss, hence "stream state is not persistent/deterministic". The
/// re-arm makes it deterministic: each cycle is drain + release with reset
/// electrons, retried until the client's rot channel is ready or the span
/// expires.
///
/// v6.9.7 THE SPAN MUST OUTLIVE THE HARNESS (run #237's artifact timeline):
/// the re-arm fired at the join (17:14:10, `rearm 15c/828t` in the diag —
/// the 15 fill cells recorded ✓) but expired after 900 ticks (90 s) while
/// the client was still at spawn — the harness only TP'd it to the vantage
/// at 17:16:24, 134 s AFTER the join (the pack prompt accept + the two
/// 15 s asserts + the 30 s settle), and every drain cycle ran against a
/// view that could not decode the cells (they sat queued, dirty 1322
/// frozen). The span now covers 300 s — the join→tp sequence (~135 s)
/// plus ten full drain cycles and the whole probe window (~95 s) with
/// margin; a session change arms ~30 drain+release chances.
pub const REARM_SPAN_TICKS: u64 = 3000;

/// v6.9.5: cap on the recorded last-edit cells (the genuine-edit gate
/// never arms above EDIT_HOLD_MAX_BATCH recent decodable cells; stale
/// queue stragglers streamed by the same release are cut here so the
/// re-arm stays a bounded marker-sized batch).
pub const REARM_MAX_CELLS: usize = 100;

/// v6.9.8: how long a recorded edit's cells stay in the re-arm material.
/// Run #237 showed the re-arm cycling 15 marker cells; run #238 showed it
/// cycling ONE — a small single-cell background edit (the rescan wave and
/// the event channel keep firing cosmetically long after the marker)
/// OVERWROTE the recorded set, and the relog re-arm then re-delivered a
/// cell nobody cares about while the marker words never landed. The
/// material is now a STAMPED SET: every genuine release upserts its cells
/// with the recording tick, entries older than this window drop out, and
/// the marker cells stay in the set for the whole window regardless of
/// what tiny edits fire afterwards (cap still REARM_MAX_CELLS, oldest
/// evicted first).
pub const REARM_EDIT_WINDOW_TICKS: u64 = 9000;

/// v6.9.11: the window is 9000 ticks (15 min) because the CI fill->relog
/// gap is up to ~9 min when LEG 1 runs its full 24-probe loop (run #241:
/// the marker's cells aged out of the material after 60 s and the relog
/// re-arm had an EMPTY set — rearm 0c, every relog probe all-zero). The
/// re-arm cycle's own re-deliveries refresh the stamps once the span is
/// live, so 15 min only has to cover the LEG 1 duration.
///
/// v6.5.3: a client younger than this never gets the edit window — its
/// particle pool is still filling and cone/zone are its only paint source
/// (60 s at the engine's 10 tps).
pub const FRESH_CLIENT_TICKS: u64 = 600;

/// The whole streaming state (server side): one fixed disk of cells plus
/// the phase cursor of the particle stream.
pub struct Streamer {
    /// World-space corner (min x/z) of the cell grid.
    pub origin_x: f64,
    pub origin_z: f64,
    pub cell_size: i32,
    /// Grid side in cells (region_blocks / cell_size).
    pub side: u32,
    /// side x side cells, row-major z outer.
    cells: Vec<Cell>,
    /// Next cell offset (0..side^2) to stream.
    cursor: u64,
    /// Pre-allocated spawn batch for the current tick.
    batch: Vec<Spawn>,
    /// Cells streamed since boot (diagnostics).
    pub streamed_cells: u64,
    /// Laps completed over the whole disk.
    pub laps: u64,
    /// v5.3.3: engine tick each cell was last streamed at (0 = never).
    last_stream: Vec<u64>,
    /// Internal tick counter when no engine tick is supplied.
    tick_counter: u64,
    /// v6.6: one rebuilt cone list per tracked view (see ConeCache). Views
    /// are matched by eye/look drift, not by index — the player list order
    /// changes between ticks and must not invalidate caches.
    view_caches: Vec<ConeCache>,
    /// v6.6: round-robin cursor over the views slice for the cone phase.
    cone_cursor: usize,
    /// Cone-phase cells streamed since boot (diagnostics).
    pub cone_cells: u64,
    /// v5.4: cells flagged for re-stream (real edits, words differ).
    dirty: Vec<bool>,
    /// v6.6.1: engine tick each cell was marked dirty at (0 = not dirty).
    /// Ages the backlog out of the hold/release decision (RECENT_EDIT_TICKS).
    dirty_since: Vec<u64>,
    /// FIFO over dirty cells (phase 0 order = edit order).
    dirty_queue: std::collections::VecDeque<u32>,
    /// v6.6.2: (cell, tick when to re-dirty it) — a retry beat of the edit
    /// protocol. Scheduled by the hold/release path only; whether that
    /// release schedules further beats is decided by the episode counter
    /// (`rebeat_beats_left`), so the retries stay a bounded sequence.
    rebeat: std::collections::VecDeque<(u32, u64)>,
    /// v6.6.2: true while the CURRENT hold's release may schedule rebeats
    /// (set at the decision arm, consumed by the release).
    rebeat_armed: bool,
    /// v6.7.1 MULTI-BEAT: retry beats left in the CURRENT edit episode.
    /// Set at the genuine release (edit_beats - 1: its own entries are
    /// retry 1 of edit_beats), consumed one per continuation release. 0 =
    /// no further beats — the retries stay a bounded sequence, never a
    /// loop. A new genuine edit resets the episode.
    rebeat_beats_left: u32,
    /// v6.6.2: the upcoming arm comes from a rebeat re-dirty — its release
    /// must not schedule another rebeat (one retry, not a loop).
    pending_rebeat: bool,
    /// v6.9.5 RELOG RE-ARM: the cells of the recent genuine edit releases —
    /// the set a relogging client must re-receive through a drain+release
    /// window (its pool saturates; plain re-streams never land). v6.9.8:
    /// STAMPED entries (cell, recording tick) merged across releases — a
    /// small background edit must not overwrite the marker's cells —
    /// bounded by REARM_MAX_CELLS, entries older than
    /// REARM_EDIT_WINDOW_TICKS drop out.
    last_edit: Vec<(u32, u64)>,
    /// v6.9.5: engine tick until which the re-arm keeps cycling the last
    /// edit's cells (0 = the re-arm is off).
    rearm_until: u64,
    /// v6.7.1: retry beats per genuine edit (the `edit_beats` CLOD_OPTS
    /// knob, default EDIT_BEATS_DEFAULT). 0 disables the rebeat entirely.
    pub edit_beats: u32,
    /// Cells currently marked dirty (fast skip when 0).
    pub dirty_pending: usize,
    /// Dirty-phase cells streamed since boot (diagnostics).
    pub dirty_streams: u64,
    /// v6.5.3: ticks of cone/zone silence left in the current edit window
    /// (see EDIT_WINDOW_TICKS). 0 = the steady state runs.
    pub edit_window: u64,
    /// v6.5.3: ticks left in the drain HOLD before the edit batch releases
    /// (see EDIT_HOLD_TICKS). 0 = not holding.
    pub edit_hold: u64,
    /// Zone-sweep cells streamed since boot (diagnostics).
    pub zone_cells: u64,
    /// v6.2 live rescan: engine tick each cell was last SAMPLED at
    /// (0 = never). The rescan wave re-samples stale cells of loaded
    /// chunks so post-boot edits reach the LOD without a relog.
    last_scan: Vec<u64>,
    /// v6.2 rescan wave cursor (row-major, wraps like `cursor`).
    rescan_cursor: u64,
    /// v6.2 diagnostics: cells re-sampled by the rescan wave / edits it
    /// caught (encoded words changed).
    pub rescan_cells: u64,
    pub rescan_edits: u64,
}

/// v6.9.5: what a dirty-phase release should do beyond streaming.
struct ReleaseOpts<'a> {
    /// Push a rebeat entry per streamed cell (a genuine-edit release or a
    /// continuation beat while the episode counter allows).
    schedule_rebeat: bool,
    /// v6.9.5 RELOG RE-ARM: receive the cells actually streamed (stamped
    /// with the tick, v6.9.8) — the caller merges them into the re-arm
    /// material.
    record: Option<&'a mut Vec<(u32, u64)>>,
}

impl Streamer {
    pub fn new(origin_x: f64, origin_z: f64, region_blocks: i32, cell_size: i32) -> Self {
        let side = (region_blocks / cell_size).max(1) as u32;
        let n = side as usize * side as usize;
        Streamer {
            origin_x,
            origin_z,
            cell_size,
            side,
            cells: vec![Cell::default(); n],
            cursor: 0,
            batch: Vec::with_capacity(4096),
            streamed_cells: 0,
            laps: 0,
            last_stream: vec![0; n],
            tick_counter: 0,
            view_caches: Vec::new(),
            cone_cursor: 0,
            cone_cells: 0,
            dirty: vec![false; n],
            dirty_since: vec![0; n],
            dirty_queue: std::collections::VecDeque::new(),
            rebeat: std::collections::VecDeque::new(),
            rebeat_armed: false,
            rebeat_beats_left: 0,
            pending_rebeat: false,
            last_edit: Vec::new(),
            rearm_until: 0,
            edit_beats: EDIT_BEATS_DEFAULT,
            dirty_pending: 0,
            dirty_streams: 0,
            edit_window: 0,
            edit_hold: 0,
            zone_cells: 0,
            last_scan: vec![0; n],
            rescan_cursor: 0,
            rescan_cells: 0,
            rescan_edits: 0,
        }
    }

    /// v6.2: is this cell due for a live rescan? Never-sampled cells are
    /// always due (they include seedgen provisional fills, which the wave
    /// upgrades the moment their chunk loads); `period == 0` disables the
    /// wave entirely (the v6.1 paint-once behavior).
    pub fn scan_due(&self, cx: u32, cz: u32, now: u64, period: u64) -> bool {
        if period == 0 {
            return false;
        }
        let side = self.side as usize;
        match self.last_scan.get(cz as usize * side + cx as usize) {
            Some(&t) => t == 0 || now.saturating_sub(t) >= period,
            None => false,
        }
    }

    /// v6.2: record that this cell was sampled (paint or rescan) at `now`.
    pub fn mark_scanned(&mut self, cx: u32, cz: u32, now: u64) {
        let side = self.side as usize;
        if cx as usize >= side || cz as usize >= side {
            return;
        }
        if let Some(t) = self.last_scan.get_mut(cz as usize * side + cx as usize) {
            *t = now;
        }
    }

    /// v6.2 diagnostics for the rescan wave: cells sampled / edits caught.
    pub fn rescan_count(&mut self, cells: u64, edits: u64) {
        self.rescan_cells += cells;
        self.rescan_edits += edits;
    }

    /// v6.2 rescan wave cursor: the next row-major cell offset to consider.
    /// Advancing is unconditional (fresh cells skip for free), so the wave
    /// covers the whole disk evenly regardless of the due-gate.
    pub fn rescan_next(&mut self, total: u64) -> u64 {
        let t = self.rescan_cursor % total;
        self.rescan_cursor += 1;
        t
    }

    pub fn cells(&self) -> u64 {
        (self.side as u64) * (self.side as u64)
    }

    pub fn valid_cells(&self) -> u64 {
        self.cells.iter().filter(|c| c.valid).count() as u64
    }

    /// v6.7.1 diag: pending rebeat entries and the retry beats still
    /// scheduled in the current edit episode (the `rbeat` token of the
    /// stream diag line — ops see the bounded retry sequence live).
    pub fn rebeat_diag(&self) -> (usize, u32) {
        (self.rebeat.len(), self.rebeat_beats_left)
    }

    /// v6.9.5 diag: (re-arm cells, ticks left) — 0/0 while the span is off.
    /// The `rearm` token of the stream diag line. v6.9.8: the count is the
    /// entries still inside the freshness window.
    pub fn rearm_diag(&self, now: u64) -> (usize, u64) {
        if now < self.rearm_until {
            let fresh = self
                .last_edit
                .iter()
                .filter(|&&(_, t)| now.saturating_sub(t) < REARM_EDIT_WINDOW_TICKS)
                .count();
            (fresh, self.rearm_until - now)
        } else {
            (0, 0)
        }
    }

    /// v6.9.5 RELOG RE-ARM: a NEW session joined (any fresh per-player entry
    /// — the CI leg 2 relog, an operator's client, a second player). If the
    /// last genuine edit's cells are known, keep them cycling through the
    /// rebeat queue for REARM_SPAN_TICKS: each cycle is a drain+release
    /// with reset electrons — the only delivery that lands in a relogging
    /// client's saturating pool, and it must outlive the client's camera
    /// bootstrap (pack prompt, tp, sky-row rot decode) to be decoded. A
    /// join before any genuine edit is a no-op: the boot paint flows
    /// through the fresh-client path exactly as before.
    pub fn arm_relog_rearm(&mut self, now: u64) {
        if self.last_edit.is_empty() {
            return;
        }
        self.rearm_until = now + REARM_SPAN_TICKS;
    }

    /// v6.9.8: merge a genuine release's cells into the stamped re-arm
    /// material. Upsert: an existing entry's stamp refreshes, a new cell is
    /// appended; the cap evicts the OLDEST entries first. Entries older
    /// than REARM_EDIT_WINDOW_TICKS are pruned on every merge and on every
    /// re-arm pass — a background cosmetic edit can never erase the
    /// marker's cells while they are inside the window.
    fn record_edit_cells(&mut self, now: u64, cells: &[u32]) {
        // The prune runs FIRST — even on an empty merge the aging must
        // advance (the tick passed in is the merge's clock).
        self.last_edit
            .retain(|&(_, t)| now.saturating_sub(t) < REARM_EDIT_WINDOW_TICKS);
        for &idx in cells {
            if let Some(e) = self.last_edit.iter_mut().find(|e| e.0 == idx) {
                e.1 = now;
            } else {
                self.last_edit.push((idx, now));
            }
        }
        if self.last_edit.len() > REARM_MAX_CELLS {
            self.last_edit.sort_by_key(|&(_, t)| t);
            let excess = self.last_edit.len() - REARM_MAX_CELLS;
            self.last_edit.drain(0..excess);
        }
    }

    pub fn get(&self, cx: u32, cz: u32) -> Option<Cell> {
        let side = self.side as usize;
        self.cells.get(cz as usize * side + cx as usize).copied()
    }

    /// v6.6.1: `now` (the engine tick of the write) ages the dirty mark —
    /// the hold/release decision counts only RECENT_EDIT_TICKS-young cells,
    /// so the boot-era backlog can never mask a genuine edit as a wave.
    pub fn set_cell(&mut self, cx: u32, cz: u32, cell: Cell, now: u64) {
        let side = self.side as usize;
        if cx as usize >= side || cz as usize >= side {
            return;
        }
        let idx = cz as usize * side + cx as usize;
        if let Some(slot) = self.cells.get_mut(idx) {
            // v5.4 delta: a REAL edit (previous cell valid, encoded words
            // differ) must re-stream. The initial scan fill overwrites
            // invalid cells — that is painting, not editing, and the
            // cone/sweep phases own it (marking it dirty would flood phase
            // 0 on every cold boot).
            let prev = *slot;
            *slot = cell;
            if cell.valid && prev.valid {
                let words = Self::encode_words(&cell);
                let old = Self::encode_words(&prev);
                if old != words && !self.dirty[idx] {
                    self.dirty[idx] = true;
                    self.dirty_since[idx] = now;
                    self.dirty_queue.push_back(idx as u32);
                    self.dirty_pending += 1;
                }
            }
        }
    }

    /// World-space min corner of one cell.
    pub fn cell_origin(&self, cx: u32, cz: u32) -> (f64, f64) {
        (
            self.origin_x + cx as f64 * self.cell_size as f64,
            self.origin_z + cz as f64 * self.cell_size as f64,
        )
    }

    /// v6.5.2 CI FORENSICS: dump a window of cells around a world coordinate
    /// — height/water/valid/rgb, the encoded words the streamer would send,
    /// the last-stream age and the dirty flag. Served by /clod-cells so a CI
    /// run can compare the SERVER's belief about a cell (what it streams)
    /// with what the client actually painted. Run #203: the client showed a
    /// permanent SKY HOLE at the marker footprint (stable across 24 probes
    /// AND a fresh relog decode) while the dome around it painted — this
    /// dump separates "the server streams garbage/never re-streams" from
    /// "the server streams the right words and the client fails to decode".
    pub fn debug_window(&self, wx: i32, wz: i32, radius: u32, now: u64) -> String {
        let cell = self.cell_size as f64;
        let ccx = (((wx as f64 - self.origin_x) / cell).floor().max(0.0)) as u32;
        let ccz = (((wz as f64 - self.origin_z) / cell).floor().max(0.0)) as u32;
        let side = self.side;
        let mut out =
            String::from("# cx cz wx wz height water valid rgb last_age dirty hword cword\n");
        let x0 = ccx.saturating_sub(radius);
        let z0 = ccz.saturating_sub(radius);
        let x1 = (ccx + radius).min(side - 1);
        let z1 = (ccz + radius).min(side - 1);
        for z in z0..=z1 {
            for x in x0..=x1 {
                let idx = (z * side + x) as usize;
                let Some(c) = self.cells.get(idx) else {
                    continue;
                };
                let (ox, oz) = self.cell_origin(x, z);
                let age = if self.last_stream[idx] == 0 {
                    u64::MAX
                } else {
                    now.wrapping_sub(self.last_stream[idx])
                };
                let (h, w) = Self::encode_words(c);
                out.push_str(&format!(
                    "{} {} {} {} {} {} {} {:?} {} {} {:#06x} {:#06x}\n",
                    x,
                    z,
                    ox as i32,
                    oz as i32,
                    c.height,
                    c.water as u8,
                    c.valid as u8,
                    c.rgb,
                    age,
                    self.dirty[idx] as u8,
                    h,
                    w
                ));
            }
        }
        out
    }

    /// Encode one cell into its two 16-bit words (HI height, LO color).
    pub fn encode_words(cell: &Cell) -> (u16, u16) {
        let h = (i32::from(cell.height) + 64).clamp(0, 255) as u16;
        (h << 8, cell.color_index() << 8)
    }

    /// Spawn positions of one word: 4 particles, slot = Z sub, bits = X sub.
    /// msb-first: slot 0 carries bits 14..15 (the HIGH pair).
    pub fn word_spawns(word: u16, cx: f64, cz: f64, sub: f64, plane_y: f64, out: &mut Vec<Spawn>) {
        for slot in 0..4usize {
            let bits = ((word >> (14 - 2 * slot)) & 3) as f64;
            let dx = (bits - 1.5) * sub;
            let dz = (slot as f64 - 1.5) * sub;
            out.push(Spawn {
                x: cx + dx,
                y: plane_y,
                z: cz + dz,
            });
        }
    }

    /// Rebuild the cone cache (v5.3.3): every valid cell whose HI or LO
    /// probe point sits inside the decode band [CULL_RADIUS, ZONE_FAR] (3D
    /// from the eye) AND inside the frustum cone. Sorted near-first. The
    /// test is the per-player spawn gate geometry (cos_half_cone), so every
    /// cone-phase spawn passes the gate — no budget is wasted on culled
    /// dots. v6.6: builds the list for ONE tracked view (see ConeCache);
    /// the caller stores it next to the eye/look it was built for.
    fn build_cone_list(&self, eye: [f64; 3], look: [f64; 3]) -> Vec<u32> {
        let side = self.side;
        let cell = self.cell_size as f64;
        let cos_half = cos_half_cone();
        let mut list: Vec<(f32, u32)> = Vec::new();
        for cz in 0..side {
            for cx in 0..side {
                let idx = cz * side + cx;
                if !self.cells[idx as usize].valid {
                    continue;
                }
                let (wx, wz) = self.cell_origin(cx, cz);
                let cxw = wx + cell * 0.5;
                let czw = wz + cell * 0.5;
                // The cell always streams HI+LO together, so BOTH plane probe
                // points must sit inside the band and the frustum — otherwise
                // the gate would cull the other plane's spawns as waste.
                let mut all_ok = true;
                let mut best = f64::INFINITY;
                for &py in &LAYER_Y {
                    let dx = cxw - eye[0];
                    let dy = py - eye[1];
                    let dz = czw - eye[2];
                    let d2 = dx * dx + dy * dy + dz * dz;
                    if d2 < best {
                        best = d2;
                    }
                    // Decode band with a 2-block safety margin on both ends
                    // so the sub-cell offsets stay inside it, plus the exact
                    // spawn-gate frustum test.
                    if d2 <= (CULL_RADIUS + 2.0) * (CULL_RADIUS + 2.0)
                        || d2 > (ZONE_FAR - 2.0) * (ZONE_FAR - 2.0)
                    {
                        all_ok = false;
                        break;
                    }
                    let inv = 1.0 / d2.sqrt();
                    if (dx * look[0] + dy * look[1] + dz * look[2]) * inv <= cos_half {
                        all_ok = false;
                        break;
                    }
                }
                if all_ok {
                    list.push((best as f32, idx));
                }
            }
        }
        list.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
        list.into_iter().map(|(_, i)| i).collect()
    }

    /// v6.6: the cache slot whose eye/look still matches this view (eye
    /// within 8 blocks and the look within ~5 deg) — the same drift
    /// tolerances the v5.3.3 single-cache rebuild used. Matching by
    /// GEOMETRY, not by index: the player list order changes between
    /// ticks (joins, leaves, Bukkit ordering) and must never invalidate
    /// a still-valid cache.
    fn find_cache_slot(&self, eye: [f64; 3], look: [f64; 3]) -> Option<usize> {
        self.view_caches.iter().position(|c| {
            let dx = c.eye[0] - eye[0];
            let dz = c.eye[2] - eye[2];
            dx * dx + dz * dz <= 64.0 && dot_v(c.look, look) >= 0.9962
        })
    }

    /// Spawn the particle batch of one cell (4 HI-plane + 4 LO-plane
    /// spawns; `edit` adds the v6.2 RESET electron for dirty streams).
    fn push_cell_spawns(&mut self, idx: u32, sub: f64, now: u64, edit: bool) {
        let side = self.side as usize;
        let cx = (idx as usize % side) as u32;
        let cz = (idx as usize / side) as u32;
        let Some(cell) = self.get(cx, cz) else {
            return;
        };
        if !cell.valid {
            return;
        }
        let (wx, wz) = self.cell_origin(cx, cz);
        let cxw = wx + self.cell_size as f64 * 0.5;
        let czw = wz + self.cell_size as f64 * 0.5;
        let (hword, cword) = Self::encode_words(&cell);
        // v5.1: the HI word rides ONLY the high layer and the LO word
        // ONLY the low layer — the villager-sparkle carrier has one hue,
        // so the capture pass tells words apart by plane height.
        Self::word_spawns(hword, cxw, czw, sub, LAYER_Y[1], &mut self.batch);
        Self::word_spawns(cword, cxw, czw, sub, LAYER_Y[0], &mut self.batch);
        if edit {
            // v6.2 RESET electron: one dot at the (qx=0, qz=0) probe
            // position on the reset plane. The client's reset pass flags
            // the cell for the frame and the data captures replace the
            // OR-merged word once the fresh decode completes.
            self.batch.push(Spawn {
                x: cxw - 1.5 * sub,
                y: RESET_Y,
                z: czw - 1.5 * sub,
            });
        }
        let idx = idx as usize;
        self.last_stream[idx] = now;
        if self.dirty[idx] {
            self.dirty[idx] = false;
            self.dirty_since[idx] = 0;
            self.dirty_pending = self.dirty_pending.saturating_sub(1);
        }
        self.streamed_cells += 1;
    }

    /// v6.5.3: the dirty-phase release loop, factored out so the hold and
    /// the idle paths share it. Pops the dirty queue once, streams every
    /// decodable cell (edit=true — the reset electron rides along), keeps
    /// undecodable cells queued. Reports how many cells streamed and whether
    /// the phase cap was hit (the wave signature). v6.6.2/v6.7.1: when
    /// `schedule_rebeat` is set (a genuine-edit release or a continuation
    /// beat while the episode counter allows), every streamed cell gets a
    /// retry entry — the rebeat re-dirties it REBEAT_DELAY_TICKS later for
    /// another complete-decode chance. v6.9.5: `release.record` receives the
    /// cells actually streamed (the caller keeps the last genuine edit's
    /// set for the relog re-arm).
    fn stream_dirty(
        &mut self,
        cap: usize,
        views: &[ViewHint],
        now: u64,
        edit_streamed: &mut u64,
        dirty_cap_hit: &mut bool,
        mut release: ReleaseOpts<'_>,
    ) {
        let sub = self.cell_size as f64 / SUB_PER_CELL;
        // v6.9.10 RECENT-EDIT PRIORITY: the fresh (RECENT_EDIT_TICKS-young)
        // decodable dirty cells stream FIRST — before the shared dirty
        // queue — capped at EDIT_HOLD_MAX_BATCH. Runs #239/#240: a genuine
        // edit's cells land at the TAIL of the 12k-entry backlog queue, and
        // each release's pop fills the 1024-dot batch cap on decode-band
        // backlog cells whose duplicates re-queue forever — the tail is
        // unreachable by construction. #239's span-gated re-arm priority
        // fixed the relog leg but left LEG 1 on the "lucky walk" (#240's
        // fill never reached the client at all: red=0 on all 24 probes).
        // The recent-edit gate is the SAME predicate the genuine-edit
        // decision uses, so every fresh edit — the marker fill, the
        // background cosmetic edits, the re-arm's re-dirties — now rides
        // the release front; the queue below then skips them as clean.
        if self.dirty_pending > 0 {
            let mut pri: Vec<u32> = Vec::new();
            let mut seen: std::collections::HashSet<u32> = std::collections::HashSet::new();
            for &idx_u32 in self.dirty_queue.iter() {
                if pri.len() >= EDIT_HOLD_MAX_BATCH as usize {
                    break;
                }
                if !seen.insert(idx_u32) {
                    continue;
                }
                let i = idx_u32 as usize;
                if self.dirty[i]
                    && now.saturating_sub(self.dirty_since[i]) < RECENT_EDIT_TICKS
                    && self.decodable_by_any(i, views)
                {
                    pri.push(idx_u32);
                }
            }
            for idx in pri {
                if self.batch.len() >= cap {
                    break;
                }
                let i = idx as usize;
                if self.dirty[i] {
                    self.push_cell_spawns(idx, sub, now, true);
                    self.dirty_streams += 1;
                    *edit_streamed += 1;
                    if release.schedule_rebeat && self.rebeat.len() < REBEAT_MAX {
                        self.rebeat.push_back((idx, now + REBEAT_DELAY_TICKS));
                    }
                    if let Some(rec) = release.record.as_deref_mut() {
                        if rec.len() < REARM_MAX_CELLS {
                            rec.push((idx, now));
                        }
                    }
                }
            }
        }
        let q_len = self.dirty_queue.len();
        for _ in 0..q_len {
            if self.batch.len() >= cap {
                *dirty_cap_hit = self.dirty_pending > 0;
                break;
            }
            let Some(idx_u32) = self.dirty_queue.pop_front() else {
                break;
            };
            let idx = idx_u32 as usize;
            if !self.dirty[idx] {
                continue; // already streamed via the cone/sweep phases
            }
            if self.decodable_by_any(idx, views) {
                self.push_cell_spawns(idx_u32, sub, now, true);
                self.dirty_streams += 1;
                *edit_streamed += 1;
                if release.schedule_rebeat && self.rebeat.len() < REBEAT_MAX {
                    self.rebeat.push_back((idx_u32, now + REBEAT_DELAY_TICKS));
                }
                if let Some(rec) = release.record.as_deref_mut() {
                    if rec.len() < REARM_MAX_CELLS {
                        rec.push((idx_u32, now));
                    }
                }
            } else {
                // Nobody can decode it now — keep it queued for later.
                self.dirty_queue.push_back(idx_u32);
            }
        }
    }

    /// v5.4: is this cell decodable by ANY of the views right now? The same
    /// geometry as the per-player spawn gate / the cone cache membership:
    /// BOTH plane probe points inside the decode band [CULL_RADIUS+2,
    /// ZONE_FAR-2] and inside the frustum cone. Cells failing this for every
    /// view were always culled at delivery — the scheduler no longer spends
    /// budget on them.
    fn decodable_by_any(&self, idx: usize, views: &[ViewHint]) -> bool {
        let side = self.side as usize;
        let cx = (idx % side) as u32;
        let cz = (idx / side) as u32;
        let (wx, wz) = self.cell_origin(cx, cz);
        let cxw = wx + self.cell_size as f64 * 0.5;
        let czw = wz + self.cell_size as f64 * 0.5;
        let cos_half = cos_half_cone();
        for v in views {
            let mut all_ok = true;
            for &py in &LAYER_Y {
                let dx = cxw - v.eye[0];
                let dy = py - v.eye[1];
                let dz = czw - v.eye[2];
                let d2 = dx * dx + dy * dy + dz * dz;
                if d2 <= (CULL_RADIUS + 2.0) * (CULL_RADIUS + 2.0)
                    || d2 > (ZONE_FAR - 2.0) * (ZONE_FAR - 2.0)
                {
                    all_ok = false;
                    break;
                }
                let inv = 1.0 / d2.sqrt();
                if (dx * v.look[0] + dy * v.look[1] + dz * v.look[2]) * inv <= cos_half {
                    all_ok = false;
                    break;
                }
            }
            if all_ok {
                return true;
            }
        }
        false
    }

    /// Advance the stream by one engine tick. v5.4 DELTA-ZONE phases over
    /// the views slice (empty slice = legacy full-disk lap, diagnostics):
    ///   0. dirty delta — cells whose words changed on the server stream
    ///      first (edit propagation, capped at half the budget);
    ///   1. cone priority (v5.3.3) — decode-band cells inside the FIRST
    ///      view's frustum stream stalest/near-first (turn refill in
    ///      seconds);
    ///   2. zone sweep — the row-major cursor, but only decodable cells
    ///      (band + ANY view's frustum, age-gated) cost budget. Cells no
    ///      player can decode were always culled at delivery; now they cost
    ///      zero. A painted, unchanged zone collapses the batch to the
    ///      VIEW_REFRESH_TICKS trickle.
    pub fn tick_batch(&mut self, budget: usize, views: &[ViewHint]) -> &[Spawn] {
        self.batch.clear();
        let total = self.side as u64 * self.side as u64;
        if total == 0 {
            return &self.batch;
        }
        self.tick_counter = self.tick_counter.wrapping_add(1);
        let now = views.first().map(|v| v.now).unwrap_or(self.tick_counter);
        let sub = self.cell_size as f64 / SUB_PER_CELL;
        let budget = budget.max(8);
        let zone_mode = !views.is_empty();

        // ---- v6.6.2 REBEAT, v6.7.1 MULTI-BEAT: the edit retries -----------
        // Cells the previous release streamed come due REBEAT_DELAY_TICKS
        // later: re-dirty them (fresh stamp), cut the post silence, and
        // let the decision below re-run the hold→release sequence. The
        // release schedules further beats only while the episode counter
        // (`rebeat_beats_left`) has one — bounded by `edit_beats`.
        let mut rebeat_rearmed = false;
        while let Some(&(idx, at)) = self.rebeat.front() {
            if now < at {
                break;
            }
            self.rebeat.pop_front();
            let i = idx as usize;
            if !self.dirty[i] {
                self.dirty[i] = true;
                self.dirty_since[i] = now;
                self.dirty_queue.push_back(idx);
                self.dirty_pending += 1;
                self.pending_rebeat = true;
                rebeat_rearmed = true;
            }
        }
        if rebeat_rearmed {
            self.edit_window = 0;
        }

        // ---- v6.9.5/v6.9.6 RELOG RE-ARM -------------------------------------
        // While the span is live, keep the last edit's cells queued for a
        // re-dirty REBEAT_DELAY_TICKS out: the pop re-dirties them (cutting
        // any post silence), the decision arms the hold, and the release
        // lands them into the drained pool with reset electrons. Plain
        // cone/zone re-streams cannot do this for a relogged client — its
        // pool saturates within seconds and never frees a slot for the
        // marker words again (run #235). The dedup keeps one pending entry
        // per cell; the entries the pop consumed re-queue on the next tick.
        //
        // v6.9.6 (run #236's lesson): a cell that went dirty while the
        // client was still at SPAWN (undecodable — the marker is hundreds
        // of blocks away) sits in the queue through the whole prompt+tp
        // window, and its dirty_since ages past RECENT_EDIT_TICKS — the
        // genuine-edit decision then stops counting it, the hold NEVER
        // arms, and the cells stream wave-style (no drain) into the
        // saturated pool forever: red=0 on every probe although the re-arm
        // bursts visibly flowed (strobe/mask hits in every #236 window).
        // While the span is live, re-stamp the still-queued cells as
        // RECENT — the edit window stays ARMABLE until the client's view
        // can finally decode them (the harness tp's to the vantage ~35 s
        // into the session; the first post-tp decision then arms the drain
        // and the release lands).
        if now < self.rearm_until {
            self.last_edit
                .retain(|&(_, t)| now.saturating_sub(t) < REARM_EDIT_WINDOW_TICKS);
            for &(idx, _) in self.last_edit.iter() {
                let i = idx as usize;
                if !self.dirty[i] {
                    if self.rebeat.len() < REBEAT_MAX && !self.rebeat.iter().any(|&(e, _)| e == idx)
                    {
                        self.rebeat.push_back((idx, now + REBEAT_DELAY_TICKS));
                    }
                } else if now.saturating_sub(self.dirty_since[i]) >= RECENT_EDIT_TICKS {
                    self.dirty_since[i] = now;
                }
            }
        }

        // ---- phase 0: dirty delta (v5.4) + the v6.5.3 edit window ----------
        // Runs #202-#206 measured the real killer: the steady-state refresh
        // stream saturates the vanilla client's particle pool, so an edit's
        // reset electron + fresh dots get REJECTED on a long-lived client.
        // A genuine edit (a SMALL batch of decodable dirty cells for a
        // SETTLED audience) runs a three-beat sequence:
        //   1. HOLD (EDIT_HOLD_TICKS): cone/zone go silent FIRST — the pool
        //      drains (dots live 3-5 s) — the edit batch itself WAITS;
        //   2. RELEASE: the batch streams into the drained pool;
        //   3. POST window (EDIT_WINDOW_TICKS): silence covers the released
        //      dots' lifetime so nothing competes with their render.
        // A WAVE (the boot-time provisional upgrades, a forceload — many
        // decodable dirty cells) and a FRESH client (empty pool, cone/zone
        // are its only paint) never hold: they stream straight through.
        self.edit_window = self.edit_window.saturating_sub(1);
        let fresh_audience = views.iter().any(|v| v.age_ticks < FRESH_CLIENT_TICKS);
        let mut edit_streamed = 0u64;
        let mut dirty_cap_hit = false;
        if zone_mode && self.dirty_pending > 0 {
            let cap = if self.edit_window > 0 {
                budget // edit mode: the whole wire serves the edit batch
            } else {
                budget / 2
            };
            if self.edit_hold > 0 {
                // Beat 1: the drain — the wire stays silent so the client's
                // saturated particle pool empties (dots live 3-5 s); the
                // edit batch WAITS. The release happens on the tick the
                // hold reaches zero (same tick, below).
                self.edit_hold -= 1;
                if self.edit_hold == 0 {
                    // v6.7.1 MULTI-BEAT: a GENUINE edit's release schedules
                    // retry beat 1 of `edit_beats`; a CONTINUATION arm (from
                    // a rebeat re-dirty) schedules the next beat while the
                    // episode counter still has one. Each retry release
                    // consumes one — bounded, never a loop.
                    let schedule_rebeat;
                    if self.rebeat_armed {
                        schedule_rebeat = self.edit_beats > 0;
                        self.rebeat_beats_left = self.edit_beats.saturating_sub(1);
                    } else if self.rebeat_beats_left > 0 {
                        schedule_rebeat = true;
                        self.rebeat_beats_left -= 1;
                    } else {
                        schedule_rebeat = false;
                    }
                    // v6.9.5: a genuine episode's delivered cells become the
                    // relog re-arm's material. v6.9.8: EVERY hold release
                    // records (stamped upserts) — continuation beats refresh
                    // their cells' stamps so the material lives as long as
                    // the cycle runs, and a background single-cell edit that
                    // joins the episode ADDS itself instead of overwriting
                    // the marker's cells (#238's `rearm 1c`).
                    let mut recorded: Vec<(u32, u64)> = Vec::new();
                    self.stream_dirty(
                        cap,
                        views,
                        now,
                        &mut edit_streamed,
                        &mut dirty_cap_hit,
                        ReleaseOpts {
                            schedule_rebeat,
                            record: Some(&mut recorded),
                        },
                    );
                    if !recorded.is_empty() {
                        let cells: Vec<u32> = recorded.iter().map(|&(i, _)| i).collect();
                        self.record_edit_cells(now, &cells);
                    }
                    if edit_streamed > 0
                        && !dirty_cap_hit
                        && (!fresh_audience || now < self.rearm_until)
                    {
                        // beat 3. v6.9.5: during the re-arm span the fresh
                        // release gets the post-silence too — the released
                        // dots' lifetime is the decode window, and the next
                        // re-dirty (REBEAT_DELAY out) cuts the rest anyway.
                        self.edit_window = EDIT_WINDOW_TICKS;
                    }
                }
            } else if self.edit_window > 0 {
                // Beat 3: post-release silence — the released dots' lifetime
                // must pass with no competition. New edits wait here.
            } else if fresh_audience && now >= self.rearm_until {
                // A fresh client's pool is empty and cone/zone are its only
                // paint — stream dirty cells straight through, never hold.
                // v6.9.5: UNLESS the relog re-arm is live — then the
                // re-armed cells fall through to the idle decision below
                // and go through the hold (the drain is the whole point:
                // a relogged client's pool saturates in seconds).
                self.stream_dirty(
                    cap,
                    views,
                    now,
                    &mut edit_streamed,
                    &mut dirty_cap_hit,
                    ReleaseOpts {
                        schedule_rebeat: false,
                        record: None,
                    },
                );
            } else {
                // Idle: decide between a genuine edit (HOLD first) and
                // everything else (waves, undecodable backlog — stream now).
                // v6.6.1: the decision counts only RECENT dirty cells — the
                // boot-era backlog (minutes old, still queued) is wave
                // material and must not outvote the genuine edit (the v6.6
                // CI run: 12k backlog cells made every edit look like a
                // wave and the drain never ran).
                let mut decodable_waiting = 0u64;
                for &idx in self.dirty_queue.iter() {
                    let i = idx as usize;
                    if self.dirty[i]
                        && now.saturating_sub(self.dirty_since[i]) < RECENT_EDIT_TICKS
                        && self.decodable_by_any(i, views)
                    {
                        decodable_waiting += 1;
                        if decodable_waiting > EDIT_HOLD_MAX_BATCH {
                            break; // a wave — the count no longer matters
                        }
                    }
                }
                if decodable_waiting > 0 && decodable_waiting <= EDIT_HOLD_MAX_BATCH {
                    // A genuine edit: start the drain, release in
                    // EDIT_HOLD_TICKS ticks. v6.6.2/v6.7.1: a GENUINE arm
                    // starts a new episode (the release schedules retry beat
                    // 1 of edit_beats); a CONTINUATION arm (a rebeat
                    // re-dirty, pending_rebeat set) schedules further beats
                    // only while rebeat_beats_left allows.
                    self.rebeat_armed = !self.pending_rebeat;
                    self.pending_rebeat = false;
                    self.edit_hold = EDIT_HOLD_TICKS;
                    self.edit_window = EDIT_HOLD_TICKS + EDIT_WINDOW_TICKS;
                } else {
                    self.stream_dirty(
                        cap,
                        views,
                        now,
                        &mut edit_streamed,
                        &mut dirty_cap_hit,
                        ReleaseOpts {
                            schedule_rebeat: false,
                            record: None,
                        },
                    );
                }
            }
        }

        // ---- phase 1: cone priority (v5.3.3, v6.6 ROUND-ROBIN) -------------
        // v6.5.3: SKIPPED inside an edit window — every spawn the steady
        // state makes competes with the edit's dots for the client's
        // saturated particle pool (run #204: the edit became undeliverable).
        // v6.6: the cone budget rotates across ALL views — the old code
        // served views[0] only, so N-1 players waited on the 60 s zone
        // sweep for their turn refills. One view owns the half budget per
        // tick (single-player fill speed is unchanged), the cursor hands
        // it to the next view every tick, and each tracked view keeps its
        // own cone cache matched by eye/look drift.
        if self.edit_window == 0 && !views.is_empty() {
            let vi = self.cone_cursor % views.len();
            self.cone_cursor = self.cone_cursor.wrapping_add(1);
            let v = views[vi];
            let slot = match self.find_cache_slot(v.eye, v.look) {
                Some(s) => s,
                None => {
                    let list = self.build_cone_list(v.eye, v.look);
                    if self.view_caches.len() >= MAX_CONE_CACHES {
                        self.view_caches.remove(0); // drop the oldest track
                    }
                    self.view_caches.push(ConeCache {
                        eye: v.eye,
                        look: v.look,
                        list,
                    });
                    self.view_caches.len() - 1
                }
            };
            let half_budget = budget / 2;
            // Clone the index list: spawning mutates self, the cached
            // slice cannot be borrowed across push_cell_spawns.
            let list: Vec<u32> = self.view_caches[slot].list.clone();
            for idx_u32 in list {
                if self.batch.len() >= half_budget {
                    break;
                }
                let idx = idx_u32 as usize;
                if now.wrapping_sub(self.last_stream[idx]) < VIEW_REFRESH_TICKS {
                    continue; // fresh enough — the client still holds it
                }
                self.push_cell_spawns(idx_u32, sub, now, false);
                self.cone_cells += 1;
            }
        }

        // ---- phase 2: zone sweep (v5.4) / legacy row-major lap -------------
        // zone_mode: only decodable cells cost budget (age-gated like the
        // cone — the client's persistent buffers still hold fresh words).
        // Legacy mode (no views): stream every valid cell (diagnostics,
        // self-heal with nobody online). v6.5.3: also silent inside the
        // edit window — the released dots' lifetime must pass undisturbed.
        if self.edit_window == 0 {
            let mut steps = 0u64;
            while self.batch.len() < budget && steps < total {
                let t = self.cursor % total;
                self.cursor += 1;
                steps += 1;
                let idx = t as usize;
                if !self.cells[idx].valid {
                    continue;
                }
                if zone_mode {
                    if self.last_stream[idx] != 0
                        && now.wrapping_sub(self.last_stream[idx]) < ZONE_REFRESH_TICKS
                    {
                        continue; // fresh enough — the client still holds it
                    }
                    // last_stream == 0: never streamed — always sweep-eligible.
                    if !self.decodable_by_any(idx, views) {
                        continue; // no player can decode it — zero budget
                    }
                    self.zone_cells += 1;
                } else if self.last_stream[idx] == now {
                    continue; // already streamed this tick by the cone phase
                }
                self.push_cell_spawns(t as u32, sub, now, false);
            }
        }
        if self.cursor >= total {
            self.cursor %= total;
            self.laps += 1;
        }
        &self.batch
    }
}

/// v6.6: dot product of two look vectors (both unit — no divide).
fn dot_v(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

/// v6.6 per-view cone cache: the rebuilt near-first cell list plus the
/// eye/look it was built for. Cached lists are reused while the eye moved
/// <= 8 blocks and the look turned <= ~5 deg (see find_cache_slot); any
/// larger drift rebuilds the list for that view only — the other views'
/// caches stay valid.
struct ConeCache {
    eye: [f64; 3],
    look: [f64; 3],
    list: Vec<u32>,
}

/// v6.6 cap on tracked cone caches. A busy server has a handful of
/// players; the cap bounds the rebuild cost even if views churn (a
/// spectator teleporting every tick builds at most one new cache per
/// tick, the oldest track is dropped first).
const MAX_CONE_CACHES: usize = 16;

#[cfg(test)]
mod tests {
    use super::*;

    fn cell(h: i16, water: bool, rgb: [u8; 3]) -> Cell {
        Cell {
            height: h,
            water,
            valid: true,
            provisional: false,
            rgb,
        }
    }

    #[test]
    fn color_index_lattice_and_water() {
        let c = cell(70, false, [0, 0, 0]);
        assert_eq!(c.color_index(), 0);
        let c = cell(70, false, [255, 255, 255]);
        assert_eq!(c.color_index(), 5 + 5 * 6 + 5 * 36);
        let c = cell(70, false, [127, 178, 56]);
        assert_eq!(c.color_index(), 2 + 3 * 6 + 36);
        let c = cell(70, true, [10, 20, 30]);
        assert_eq!(c.color_index(), COLOR_WATER);
    }

    #[test]
    fn word_round_trip_msb_first() {
        // 8 data bits in the high byte, 4 slots msb-first.
        let byte: u16 = 0b10_01_11_00;
        let word = byte << 8;
        let mut out = Vec::new();
        Streamer::word_spawns(word, 0.0, 0.0, 1.0, 0.0, &mut out);
        assert_eq!(out.len(), 4);
        // slot 0 -> bits 14..15 = 0b10 = 2; slot 1 -> 0b01 = 1;
        // slot 2 -> 0b11 = 3; slot 3 -> 0b00 = 0.
        let got: Vec<f64> = out.iter().map(|s| s.x).collect();
        assert_eq!(got, vec![0.5, -0.5, 1.5, -1.5]);
        let zs: Vec<f64> = out.iter().map(|s| s.z).collect();
        assert_eq!(zs, vec![-1.5, -0.5, 0.5, 1.5]);
        // Re-encode from positions: bits = round(x/sub + 1.5)
        for (s, slot) in out.iter().zip(0..4usize) {
            let bits = (s.x + 1.5).round() as i32;
            let word_bits = (word >> (14 - 2 * slot)) & 3;
            assert_eq!(bits as u16, word_bits);
        }
    }

    #[test]
    fn height_word_clamps_to_byte() {
        let c = cell(-64, false, [0, 0, 0]);
        let (h, _) = Streamer::encode_words(&c);
        assert_eq!(h >> 8, 0);
        let c = cell(320, false, [0, 0, 0]);
        let (h, _) = Streamer::encode_words(&c);
        assert_eq!(h >> 8, 255);
        let c = cell(64, false, [0, 0, 0]);
        let (h, _) = Streamer::encode_words(&c);
        assert_eq!(h >> 8, 128);
    }

    #[test]
    fn rescan_due_and_mark_semantics() {
        let mut s = Streamer::new(0.0, 0.0, 64, 4);
        s.set_cell(1, 1, cell(70, false, [1, 2, 3]), 0);
        // period 0 = the wave is off entirely (v6.1 paint-once behavior).
        assert!(!s.scan_due(1, 1, 100, 0));
        // Never-sampled cells are due immediately (seedgen provisional
        // fills included — the wave upgrades them once their chunk loads).
        assert!(s.scan_due(1, 1, 100, 40));
        s.mark_scanned(1, 1, 100);
        assert!(!s.scan_due(1, 1, 100, 40));
        assert!(!s.scan_due(1, 1, 139, 40), "fresh cell is not due");
        assert!(s.scan_due(1, 1, 140, 40), "age >= period is due");
        // Out-of-disk coords never panic and never report due.
        assert!(!s.scan_due(999, 1, 100, 40));
    }

    #[test]
    fn dirty_edit_stream_carries_reset_electron() {
        let mut s = Streamer::new(0.0, 0.0, 64, 4);
        s.set_cell(2, 2, cell(70, false, [1, 2, 3]), 0);
        // The view decodes cell (2,2): both plane probes sit in the band
        // and inside the frustum cone (see decodable_by_any geometry).
        let views = [ViewHint {
            eye: [10.0, 100.0, 60.0],
            look: [0.0, 0.0, -1.0],
            now: 10,
            age_ticks: u64::MAX,
        }];
        // Refresh streams (paint/cone/zone) carry NO reset electron.
        let b: Vec<Spawn> = s.tick_batch(64, &views).to_vec();
        assert!(!b.is_empty());
        assert!(
            b.iter().all(|p| p.y != RESET_Y),
            "refresh streams must not spawn reset electrons"
        );
        // A REAL edit (words changed): the dirty phase must stream the
        // cell with exactly ONE reset electron on the reset plane — after
        // the v6.5.3 hold/release sequence (a settled client drains first).
        s.set_cell(2, 2, cell(80, false, [9, 9, 9]), 11);
        assert_eq!(s.dirty_pending, 1);
        let mut b: Vec<Spawn> = Vec::new();
        for t in 11..(11 + EDIT_HOLD_TICKS + 2) {
            let views = [ViewHint {
                eye: [10.0, 100.0, 60.0],
                look: [0.0, 0.0, -1.0],
                now: t,
                age_ticks: u64::MAX,
            }];
            let batch = s.tick_batch(64, &views).to_vec();
            if !batch.is_empty() {
                b = batch;
            }
        }
        let resets: Vec<&Spawn> = b.iter().filter(|p| p.y == RESET_Y).collect();
        assert_eq!(resets.len(), 1, "an edit streams exactly one reset dot");
        assert_eq!(s.dirty_pending, 0, "the edit was streamed");
        // The reset dot sits at the (qx=0, qz=0) probe position: one sub
        // quantum up-left from the cell center.
        let r = resets[0];
        let (wx, wz) = s.cell_origin(2, 2);
        assert!((r.x - (wx + 2.0 - 1.5)).abs() < 1e-9);
        assert!((r.z - (wz + 2.0 - 1.5)).abs() < 1e-9);
    }

    #[test]
    fn batch_budget_and_phase_cycle() {
        let mut s = Streamer::new(0.0, 0.0, 512, 4);
        assert_eq!(s.side, 128);
        // Fill the whole disk with valid cells.
        for cz in 0..128u32 {
            for cx in 0..128u32 {
                s.set_cell(
                    cx,
                    cz,
                    cell(64 + (cx % 8) as i16, false, [100, 100, 100]),
                    0,
                );
            }
        }
        // 2048 particles / tick = 256 cells (4 height + 4 color = 8/cell).
        let b: Vec<Spawn> = s.tick_batch(2048, &[]).to_vec();
        assert_eq!(b.len(), 2048);
        assert_eq!(s.streamed_cells, 256);
        // Full lap: 16384 cells / 256 per tick = 64 ticks.
        for _ in 0..63 {
            s.tick_batch(2048, &[]);
        }
        assert_eq!(s.laps, 1);
        assert_eq!(s.streamed_cells, 16384);
        // Cursor wrapped: the next batch starts from cell 0 again.
        let b: Vec<Spawn> = s.tick_batch(16, &[]).to_vec();
        assert_eq!(b.len(), 16);
        let first = &b[0];
        let (wx, _wz) = s.cell_origin(0, 0);
        assert!((first.x - (wx + 2.0)).abs() < 1.0);
    }

    #[test]
    fn void_cells_cost_nothing() {
        let mut s = Streamer::new(0.0, 0.0, 64, 4);
        // Only 2 valid cells on the disk.
        s.set_cell(0, 0, cell(70, false, [90, 90, 90]), 0);
        s.set_cell(15, 15, cell(80, true, [0, 0, 200]), 0);
        let b: Vec<Spawn> = s.tick_batch(2048, &[]).to_vec();
        // 2 cells x (4 height + 4 color) particles each.
        assert_eq!(b.len(), 16);
        assert_eq!(s.streamed_cells, 2);
        // Water cell streams its color word on the LOW layer (last 4 spawns).
        let lo: Vec<&Spawn> = b.iter().skip(12).collect();
        assert_eq!(lo.len(), 4);
        assert!(lo.iter().all(|p| (p.y - LAYER_Y[0]).abs() < 1e-9));
        // All spawns of the second cell sit at cell (15,15) center.
        let (wx, wz) = s.cell_origin(15, 15);
        for p in lo {
            assert!((p.x - (wx + 2.0)).abs() <= 1.6);
            assert!((p.z - (wz + 2.0)).abs() <= 1.6);
        }
    }

    #[test]
    fn look_dir_bukkit_conventions() {
        // yaw 0, level -> +Z (south).
        let d = look_dir(0.0, 0.0);
        assert!((d[0]).abs() < 1e-9 && (d[1]).abs() < 1e-9 && (d[2] - 1.0).abs() < 1e-9);
        // yaw 90 -> -X (west).
        let d = look_dir(90.0, 0.0);
        assert!((d[0] + 1.0).abs() < 1e-9 && (d[2]).abs() < 1e-9);
        // pitch -90 -> straight up; +90 -> straight down.
        let up = look_dir(0.0, -90.0);
        assert!((up[1] - 1.0).abs() < 1e-9);
        let down = look_dir(0.0, 90.0);
        assert!((down[1] + 1.0).abs() < 1e-9);
        // Unit length at an oblique pose.
        let d = look_dir(37.0, -21.0);
        let len = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
        assert!((len - 1.0).abs() < 1e-9);
    }

    #[test]
    fn spawn_visible_gate_band_and_cone() {
        let cos_half = cos_half_cone();
        // Eye at the origin looking south (+Z); dot 50 blocks ahead: visible.
        let s = Spawn {
            x: 0.0,
            y: 80.0,
            z: 50.0,
        };
        let eye = [0.0, 80.0, 0.0];
        let look = look_dir(0.0, 0.0);
        assert!(spawn_visible(&s, eye, look, true, cos_half));
        // Same dot BEHIND the camera: outside the cone.
        let s = Spawn {
            x: 0.0,
            y: 80.0,
            z: -50.0,
        };
        assert!(!spawn_visible(&s, eye, look, true, cos_half));
        // Inside the cone but too close (inside the 28-block cull bubble).
        let s = Spawn {
            x: 0.0,
            y: 80.0,
            z: 20.0,
        };
        assert!(!spawn_visible(&s, eye, look, true, cos_half));
        // Inside the cone but beyond the 112-block decode zone: permanent
        // undecodable garbage — never spawned.
        let s = Spawn {
            x: 0.0,
            y: 80.0,
            z: 200.0,
        };
        assert!(!spawn_visible(&s, eye, look, true, cos_half));
        // Sky-blind view: nothing spawns at all, cone or not.
        let s = Spawn {
            x: 0.0,
            y: 80.0,
            z: 50.0,
        };
        assert!(!spawn_visible(&s, eye, look, false, cos_half));
        // Sideways dot at the cone edge: 90 degrees off the look axis.
        let s = Spawn {
            x: 50.0,
            y: 80.0,
            z: 0.0,
        };
        assert!(!spawn_visible(&s, eye, look, true, cos_half));
    }

    #[test]
    fn spawn_gate_zone_is_symmetric_3d() {
        // A dot 50 blocks straight BELOW the eye decodes fine when the
        // player looks down at it with sky in view (high vantage, horizon
        // in frame): the band test is 3D, not horizontal.
        let cos_half = cos_half_cone();
        let eye = [0.0, 130.0, 0.0];
        let s = Spawn {
            x: 0.0,
            y: 80.0,
            z: 0.0,
        };
        let look = look_dir(0.0, 0.0); // level — dots below are OUTSIDE the cone
        assert!(!spawn_visible(&s, eye, look, true, cos_half));
        let look = look_dir(0.0, 40.0); // pitched down 40, sky still in view
        assert!(spawn_visible(&s, eye, look, true, cos_half));
    }

    #[test]
    fn batch_positions_on_sub_lattice() {
        let mut s = Streamer::new(100.0, -40.0, 64, 4);
        s.set_cell(3, 5, cell(70, false, [200, 100, 50]), 0);
        let b: Vec<Spawn> = s.tick_batch(2048, &[]).to_vec();
        assert_eq!(b.len(), 8);
        let sub = 4.0 / SUB_PER_CELL; // 1.0
        let (wx, wz) = s.cell_origin(3, 5);
        let cxw = wx + 2.0;
        let czw = wz + 2.0;
        for p in b.iter() {
            // dx in {-1.5,-0.5,0.5,1.5} * sub from the cell center.
            let dx = p.x - cxw;
            let dz = p.z - czw;
            assert!((dx / sub + 1.5).fract().abs() < 1e-6);
            assert!((dz / sub + 1.5).fract().abs() < 1e-6);
            assert!(dx.abs() <= 2.0 && dz.abs() <= 2.0);
        }
        // v5.1: first the height word on the HIGH layer, then the color word
        // on the LOW layer (one hue — words are told apart by plane height).
        assert_eq!(b[0].y, LAYER_Y[1]);
        assert_eq!(b[3].y, LAYER_Y[1]);
        assert_eq!(b[4].y, LAYER_Y[0]);
        assert_eq!(b[7].y, LAYER_Y[0]);
    }

    fn vhint(x: f64, z: f64, yaw_deg: f64, pitch_deg: f64, now: u64) -> ViewHint {
        ViewHint {
            eye: [x, 64.0 + 1.62, z],
            look: look_dir(yaw_deg as f32, pitch_deg as f32),
            now,
            age_ticks: u64::MAX, // a settled client — the edit window may arm
        }
    }

    /// v6.6 ROUND-ROBIN CONE: the frustum priority must rotate across ALL
    /// views. The old code served views[0] only — with two players looking
    /// in opposite directions the second one waited on the 60 s zone sweep
    /// for every turn refill. The prefix of each tick's batch belongs to
    /// the cone phase (phase order), so the served view is observable: on
    /// its tick the cone prefix must lie in THAT view's band + frustum.
    #[test]
    fn cone_budget_rotates_round_robin_across_views() {
        let mut s = Streamer::new(0.0, 0.0, 256, 2);
        for cz in 0..256u32 {
            for cx in 0..256u32 {
                s.set_cell(cx, cz, cell(70, false, [100, 100, 100]), 0);
            }
        }
        // A: west half looking south (+Z, yaw 0); B: east half looking
        // north (-Z, yaw 180). Opposite frustums — no spawn can pass both
        // cone tests, so the prefix classification is unambiguous.
        let a = vhint(60.0, 60.0, 0.0, 0.0, 100);
        let b = vhint(196.0, 196.0, 180.0, 0.0, 100);
        let in_cone_of = |sp: &Spawn, v: &ViewHint| {
            let dx = sp.x - v.eye[0];
            let dy = sp.y - v.eye[1];
            let dz = sp.z - v.eye[2];
            let d2 = dx * dx + dy * dy + dz * dz;
            let inv = 1.0 / d2.sqrt();
            let cone =
                (dx * v.look[0] + dy * v.look[1] + dz * v.look[2]) * inv > cos_half_cone() - 0.02;
            let band = d2 > (CULL_RADIUS - 4.0) * (CULL_RADIUS - 4.0)
                && d2 <= (ZONE_FAR + 4.0) * (ZONE_FAR + 4.0);
            cone && band
        };
        // Tick 1 serves view 0 (A): the cone prefix must be A's.
        let b1: Vec<Spawn> = s.tick_batch(256, &[a, b]).to_vec();
        assert!(s.cone_cells > 0, "view A's cone must stream on its tick");
        let prefix1 = &b1[..(s.cone_cells as usize * 8).min(b1.len())];
        assert!(!prefix1.is_empty());
        assert!(
            prefix1.iter().all(|sp| in_cone_of(sp, &a)),
            "tick 1 cone prefix must serve view A"
        );
        // Tick 2 serves view 1 (B): the cone prefix must be B's — this is
        // the round-robin contract the views[0]-only code broke.
        let before = s.cone_cells;
        let b2: Vec<Spawn> = s.tick_batch(256, &[a, b]).to_vec();
        let served = s.cone_cells - before;
        assert!(served > 0, "view B's cone must stream on its own tick");
        let prefix2 = &b2[..(served as usize * 8).min(b2.len())];
        assert!(
            prefix2.iter().all(|sp| in_cone_of(sp, &b)),
            "tick 2 cone prefix must serve view B (round-robin)"
        );
    }

    /// v6.6: per-view cone caches are matched by GEOMETRY (eye/look drift),
    /// not by list index — flipping the player-list order between ticks
    /// must reuse the caches, and a burst of one-off spectator views must
    /// not break the tracked players (the cache cap evicts, the next
    /// served tick rebuilds transparently).
    #[test]
    fn cone_caches_survive_order_flips_and_view_churn() {
        let mut s = Streamer::new(0.0, 0.0, 256, 2);
        for cz in 0..256u32 {
            for cx in 0..256u32 {
                s.set_cell(cx, cz, cell(70, false, [100, 100, 100]), 0);
            }
        }
        let a = vhint(60.0, 60.0, 0.0, 0.0, 100);
        let b = vhint(196.0, 196.0, 180.0, 0.0, 100);
        // Build both caches in [a, b] order.
        let _ = s.tick_batch(2048, &[a, b]);
        let _ = s.tick_batch(2048, &[a, b]);
        // Order flip: B first. Both views must keep receiving cone service.
        let c1 = s.cone_cells;
        let _ = s.tick_batch(2048, &[b, a]);
        let _ = s.tick_batch(2048, &[b, a]);
        assert!(s.cone_cells > c1, "order flip must not starve the cones");
        // Churn: 20 distinct spectator eyes (new cache each tick, the cap
        // evicts the oldest tracks). Past VIEW_REFRESH_TICKS the players'
        // cone cells come due again — the (evicted) caches must rebuild
        // and serve, not vanish with the churn.
        for i in 0..20u64 {
            let sp = vhint(120.0 + f64::from(i as i16), 120.0, 90.0, 0.0, 100 + i);
            let _ = s.tick_batch(256, &[sp]);
        }
        let late = 100 + VIEW_REFRESH_TICKS + 30;
        let a2 = vhint(60.0, 60.0, 0.0, 0.0, late);
        let b2 = vhint(196.0, 196.0, 180.0, 0.0, late);
        let c2 = s.cone_cells;
        let _ = s.tick_batch(2048, &[a2, b2]);
        let _ = s.tick_batch(2048, &[a2, b2]);
        assert!(
            s.cone_cells > c2,
            "tracked views must keep cone service after cache churn"
        );
    }

    /// v5.3.3: cone-phase spawns precede the legacy lap and stay inside the
    /// decode band + frustum (the per-player gate will pass them).
    #[test]
    fn cone_phase_streams_first_and_in_band() {
        let mut s = Streamer::new(0.0, 0.0, 256, 2);
        for cz in 0..256u32 {
            for cx in 0..256u32 {
                s.set_cell(cx, cz, cell(70, false, [100, 100, 100]), 0);
            }
        }
        // Player at the disk center, looking +Z, level pitch.
        let v = vhint(128.0, 128.0, 0.0, 0.0, 100);
        let b: Vec<Spawn> = s.tick_batch(2048, &[v]).to_vec();
        assert!(!b.is_empty());
        // The cone phase fills at least a quarter of the budget immediately
        // (row-major cells are far away and mostly out of the frustum).
        let half = b.len() / 2;
        assert!(s.cone_cells > 0, "cone phase must stream cells");
        for sp in &b[..half.min(b.len())] {
            // Every cone-phase spawn must pass the EXACT per-player spawn
            // gate geometry (decode band + frustum): the scheduler tested the
            // cell probe points, the 2-block band margin covers the sub-cell
            // offsets, so nothing may fall outside.
            let dx = sp.x - v.eye[0];
            let dy = sp.y - v.eye[1];
            let dz = sp.z - v.eye[2];
            let d2 = dx * dx + dy * dy + dz * dz;
            let inv = 1.0 / d2.sqrt();
            let in_cone =
                (dx * v.look[0] + dy * v.look[1] + dz * v.look[2]) * inv > cos_half_cone() - 0.02;
            let in_band = d2 > (CULL_RADIUS - 4.0) * (CULL_RADIUS - 4.0)
                && d2 <= (ZONE_FAR + 4.0) * (ZONE_FAR + 4.0);
            assert!(
                in_band && in_cone,
                "cone spawn ({},{},{}) outside frustum/band",
                sp.x,
                sp.y,
                sp.z
            );
        }
    }

    /// The age gate: cone cells stream once, then hold for
    /// VIEW_REFRESH_TICKS with an unchanged view.
    #[test]
    fn cone_age_gate_holds_fresh_cells() {
        let mut s = Streamer::new(0.0, 0.0, 256, 2);
        for cz in 0..256u32 {
            for cx in 0..256u32 {
                s.set_cell(cx, cz, cell(70, false, [100, 100, 100]), 0);
            }
        }
        let v = vhint(128.0, 128.0, 0.0, 0.0, 100);
        let first: Vec<Spawn> = s.tick_batch(2048, &[v]).to_vec();
        // Cell identity via floor: sub-cell offsets never cross the border.
        let first_cells: Vec<(i64, i64)> = first
            .chunks(8)
            .map(|c| (c[0].x.floor() as i64, c[0].z.floor() as i64))
            .collect();
        let first_cone: Vec<(i64, i64)> =
            first_cells[..first_cells.len().min(s.cone_cells as usize)].to_vec();
        // Immediate next tick with the SAME view: no cone cell repeats.
        let v2 = vhint(128.0, 128.0, 0.0, 0.0, 101);
        let second: Vec<Spawn> = s.tick_batch(2048, &[v2]).to_vec();
        for sp in second.chunks(8).take(s.cone_cells as usize) {
            let key = (sp[0].x.floor() as i64, sp[0].z.floor() as i64);
            assert!(
                !first_cone.contains(&key),
                "cone cell {:?} re-streamed before VIEW_REFRESH_TICKS",
                key
            );
        }
        // After the age gate the stale cone cells refresh again.
        let v3 = vhint(128.0, 128.0, 0.0, 0.0, 100 + VIEW_REFRESH_TICKS);
        let before = s.cone_cells;
        let third: Vec<Spawn> = s.tick_batch(2048, &[v3]).to_vec();
        let third_cone = &third[..third.len().min((s.cone_cells - before) as usize * 8)];
        assert!(
            third_cone
                .iter()
                .any(|sp| first_cone.contains(&(sp.x.floor() as i64, sp.z.floor() as i64))),
            "stale cone cells must refresh after the age gate"
        );
    }

    /// Turning the camera rebuilds the cache: the new cone streams first.
    #[test]
    fn cone_follows_the_turn() {
        let mut s = Streamer::new(0.0, 0.0, 256, 2);
        for cz in 0..256u32 {
            for cx in 0..256u32 {
                s.set_cell(cx, cz, cell(70, false, [100, 100, 100]), 0);
            }
        }
        // Face +Z first, then turn to -X (vanilla yaw 90).
        let v = vhint(128.0, 128.0, 0.0, 0.0, 10);
        let _ = s.tick_batch(2048, &[v]);
        let v2 = vhint(128.0, 128.0, 90.0, 0.0, 200);
        let b: Vec<Spawn> = s.tick_batch(2048, &[v2]).to_vec();
        assert!(s.cone_cells > 0);
        // Cone-phase spawns of THIS tick (the front of the batch) must lie
        // in the west frustum band (or the near bubble).
        let cone_spawns = (s.cone_cells.min(128) * 8) as usize;
        for sp in &b[..cone_spawns.min(b.len())] {
            let dx = sp.x - v2.eye[0];
            let dy = sp.y - v2.eye[1];
            let dz = sp.z - v2.eye[2];
            let d2 = dx * dx + dy * dy + dz * dz;
            let inv = 1.0 / d2.sqrt();
            let in_cone = (dx * v2.look[0] + dy * v2.look[1] + dz * v2.look[2]) * inv
                > cos_half_cone() - 0.02;
            let in_band = d2 > (CULL_RADIUS - 4.0) * (CULL_RADIUS - 4.0)
                && d2 <= (ZONE_FAR + 4.0) * (ZONE_FAR + 4.0);
            assert!(
                in_band && in_cone,
                "spawn ({},{},{}) not in the west cone band",
                sp.x,
                sp.y,
                sp.z
            );
        }
    }

    /// v5.4 delta: an edited cell (words changed) streams in phase 0 —
    /// ahead of the cone and the sweep — as long as SOME view can decode it.
    #[test]
    fn changed_cell_streams_first() {
        let mut s = Streamer::new(0.0, 0.0, 256, 2);
        for cz in 0..256u32 {
            for cx in 0..256u32 {
                s.set_cell(cx, cz, cell(70, false, [100, 100, 100]), 0);
            }
        }
        let v = vhint(128.0, 128.0, 0.0, 0.0, 100);
        let _ = s.tick_batch(2048, &[v]);
        assert_eq!(s.dirty_pending, 0, "filling the disk never marks dirty");
        // Edit a cell far ahead of the row-major cursor: deep in the
        // frustum band (z ~ 200), inside the decode band. Grid is 128x128
        // cells (region 256 / cell 2), so (66,100) is in, (128,*) would be
        // out of the grid entirely.
        let (ex, ez) = (66u32, 100u32);
        s.set_cell(ex, ez, cell(90, false, [200, 100, 50]), 101);
        assert_eq!(s.dirty_pending, 1);
        // Tiny budget: ONLY the dirty cell may stream (phase 0 cap = 4
        // cells at budget 32, and the dirty cell is first in the queue).
        let _v2 = vhint(128.0, 128.0, 0.0, 0.0, 101);
        // v6.5.3: a genuine edit for a settled client runs hold -> release —
        // the wire is silent for EDIT_HOLD_TICKS (the pool drains), then the
        // batch releases into the clean pool.
        let mut b: Vec<Spawn> = Vec::new();
        for t in 101..(101 + EDIT_HOLD_TICKS + 2) {
            let v = vhint(128.0, 128.0, 0.0, 0.0, t);
            let batch = s.tick_batch(32, &[v]).to_vec();
            if !batch.is_empty() {
                b = batch;
            }
        }
        assert_eq!(s.dirty_pending, 0, "edited cell must stream after the hold");
        assert_eq!(s.dirty_streams, 1);
        let (wx, wz) = s.cell_origin(ex, ez);
        assert!(b.len() >= 8);
        // The edit rides the FRONT of the batch (phase 0 first); the rest of
        // the budget still flows to the cone/sweep as usual.
        for sp in &b[..8] {
            assert!(
                (sp.x - (wx + 1.0)).abs() < 2.1 && (sp.z - (wz + 1.0)).abs() < 2.1,
                "dirty cell must be the first batch: spawn at {},{} vs cell {} {}",
                sp.x,
                sp.z,
                wx,
                wz
            );
        }
    }

    /// v5.4 delta: rewriting a cell with the SAME encoded words must NOT
    /// mark it dirty (idempotent re-scans are free); the INITIAL fill of an
    /// invalid cell is painting, not editing — the cone/sweep own it.
    #[test]
    fn unchanged_rewrites_stay_clean() {
        let mut s = Streamer::new(0.0, 0.0, 64, 4);
        s.set_cell(0, 0, cell(70, false, [100, 100, 100]), 0);
        assert_eq!(s.dirty_pending, 0, "initial fill is painting, not editing");
        // Same words again: still clean.
        s.set_cell(0, 0, cell(70, false, [100, 100, 100]), 0);
        assert_eq!(s.dirty_pending, 0);
        // Real edit (height changed on a live cell) -> dirty.
        s.set_cell(0, 0, cell(90, false, [100, 100, 100]), 0);
        assert_eq!(s.dirty_pending, 1);
        // Stream it (legacy lap): dirty cleared.
        let _ = s.tick_batch(2048, &[]);
        assert_eq!(s.dirty_pending, 0);
        // Rewrite with the same words AFTER the send: stays clean.
        s.set_cell(0, 0, cell(90, false, [100, 100, 100]), 0);
        assert_eq!(s.dirty_pending, 0, "identical words must not re-queue");
        // And a real change re-queues it.
        s.set_cell(0, 0, cell(90, true, [100, 100, 100]), 0);
        assert_eq!(s.dirty_pending, 1);
    }

    /// v5.4 zone sweep: with a live view the batch NEVER leaves the
    /// decodable set (band + frustum) — out-of-zone cells cost zero budget,
    /// so a painted zone collapses to the refresh trickle.
    #[test]
    fn zone_sweep_spends_budget_only_on_decodable_cells() {
        let mut s = Streamer::new(0.0, 0.0, 256, 2);
        for cz in 0..256u32 {
            for cx in 0..256u32 {
                s.set_cell(cx, cz, cell(70, false, [100, 100, 100]), 0);
            }
        }
        // Warm up: 50 ticks fully paint the decodable set (the cone list is
        // ~4k cells at 128 cells/tick, done by ~tick 135 of this window).
        for tick in 0..50u64 {
            let vt = vhint(128.0, 128.0, 0.0, 0.0, 100 + tick);
            let b: Vec<Spawn> = s.tick_batch(2048, &[vt]).to_vec();
            for sp in &b {
                let dx = sp.x - vt.eye[0];
                let dy = sp.y - vt.eye[1];
                let dz = sp.z - vt.eye[2];
                let d2 = dx * dx + dy * dy + dz * dz;
                let inv = 1.0 / d2.sqrt();
                let in_cone = (dx * vt.look[0] + dy * vt.look[1] + dz * vt.look[2]) * inv
                    > cos_half_cone() - 0.02;
                let in_band = d2 > (CULL_RADIUS - 4.0) * (CULL_RADIUS - 4.0)
                    && d2 <= (ZONE_FAR + 4.0) * (ZONE_FAR + 4.0);
                assert!(
                    in_band && in_cone,
                    "tick {tick}: spawn ({},{},{}) outside every decodable zone",
                    sp.x,
                    sp.y,
                    sp.z
                );
            }
        }
        // Steady state (v5.4 DELTA-ZONE): the painted, unchanged zone
        // collapses the batch to ZERO — cone cells age < 60, sweep cells
        // age < 600, no dirty cells. Zero waste.
        let v_idle = vhint(128.0, 128.0, 0.0, 0.0, 150);
        let b: Vec<Spawn> = s.tick_batch(2048, &[v_idle]).to_vec();
        assert!(b.is_empty(), "idle zone must collapse to zero spawns");
        // The refresh wave: cone cells painted at tick 100 come due at 160.
        let v_due = vhint(128.0, 128.0, 0.0, 0.0, 161);
        let b: Vec<Spawn> = s.tick_batch(2048, &[v_due]).to_vec();
        assert!(!b.is_empty(), "due cone cells must refresh");
        for sp in &b {
            let dx = sp.x - v_due.eye[0];
            let dy = sp.y - v_due.eye[1];
            let dz = sp.z - v_due.eye[2];
            let d2 = dx * dx + dy * dy + dz * dz;
            let inv = 1.0 / d2.sqrt();
            let in_cone = (dx * v_due.look[0] + dy * v_due.look[1] + dz * v_due.look[2]) * inv
                > cos_half_cone() - 0.02;
            let in_band = d2 > (CULL_RADIUS - 4.0) * (CULL_RADIUS - 4.0)
                && d2 <= (ZONE_FAR + 4.0) * (ZONE_FAR + 4.0);
            assert!(in_band && in_cone, "refresh spawn outside the zone");
        }
    }

    /// v5.4 delta: a dirty cell NOBODY can decode waits in the queue and
    /// streams the moment a view that covers it appears.
    #[test]
    fn dirty_cell_out_of_zone_waits_for_a_view() {
        let mut s = Streamer::new(0.0, 0.0, 256, 2);
        for cz in 0..256u32 {
            for cx in 0..256u32 {
                s.set_cell(cx, cz, cell(70, false, [100, 100, 100]), 0);
            }
        }
        let v = vhint(128.0, 128.0, 0.0, 0.0, 100);
        let _ = s.tick_batch(2048, &[v]);
        // Edit BEHIND the player's back (north, -Z): not decodable now.
        // (66,20) sits ~88 blocks behind the eye — valid grid cell.
        s.set_cell(66, 20, cell(95, false, [200, 50, 50]), 101);
        assert_eq!(s.dirty_pending, 1);
        let v2 = vhint(128.0, 128.0, 0.0, 0.0, 101);
        let before = s.dirty_streams;
        let _ = s.tick_batch(2048, &[v2]);
        assert_eq!(
            s.dirty_streams, before,
            "out-of-zone dirty cell must wait (budget never wasted)"
        );
        assert_eq!(s.dirty_pending, 1, "still queued");
        // Turn 180 degrees (vanilla yaw 180 = -Z): now decodable -> the
        // v6.5.3 decision scan finds it, the hold runs, then it releases.
        let mut b: Vec<Spawn> = Vec::new();
        for t in 102..(102 + EDIT_HOLD_TICKS + 2) {
            let v = vhint(128.0, 128.0, 180.0, 0.0, t);
            let batch = s.tick_batch(2048, &[v]).to_vec();
            if !batch.is_empty() {
                b = batch;
            }
        }
        assert_eq!(s.dirty_streams, before + 1);
        assert_eq!(s.dirty_pending, 0);
        let (wx, wz) = s.cell_origin(66, 20);
        assert!(b
            .iter()
            .any(|sp| (sp.x - (wx + 1.0)).abs() < 2.1 && (sp.z - (wz + 1.0)).abs() < 2.1));
    }

    /// v6.4.2 regression (CI visual run 2026-10-31): the config names the
    /// disk CENTER, the Streamer takes the ORIGIN (center - region/2 — the
    /// same conversion engine::arm_from_config and the pack's GridInfo now
    /// apply). With the center passed verbatim the disk shifted (+256,+256),
    /// the CI vantage 100 blocks west of the named center fell outside every
    /// decode band (both plane probes need [CULL_RADIUS+2, ZONE_FAR-2]) and
    /// the scheduler streamed NOTHING (cone 0 zone 0, spawned 0, wire 0p).
    #[test]
    fn center_anchored_disk_streams_from_an_off_center_vantage() {
        let (cx, cz, region) = (-930.0, 796.0, 512);
        let half = f64::from(region) * 0.5;
        let mut s = Streamer::new(cx - half, cz - half, region, 2);
        for cz in 0..256u32 {
            for cx in 0..256u32 {
                s.set_cell(cx, cz, cell(70, false, [100, 100, 100]), 0);
            }
        }
        // the CI vantage: 75 blocks west of the named center (v6.4.2 — the
        // LO plane caps the horizontal decode distance at ~83 blocks, and
        // vanilla purity needs > 64), eye y=150+1.62, looking east (Bukkit
        // yaw -90 == 270), pitch 5 down.
        let v = ViewHint {
            eye: [cx - 75.0, 150.0 + 1.62, cz],
            look: look_dir(-90.0, 5.0),
            now: 400,
            age_ticks: u64::MAX,
        };
        let b: Vec<Spawn> = s.tick_batch(2048, &[v]).to_vec();
        assert!(
            s.cone_cells + s.zone_cells > 0,
            "no cells decoded from the off-center vantage — center->origin is broken"
        );
        assert!(!b.is_empty());
        // the spawned cells must sit inside the disk that the config center
        // implies: [-1186..-674] x [540..1052]
        for sp in b.iter() {
            assert!(sp.x >= cx - half && sp.x <= cx + half);
            assert!(sp.z >= cz - half && sp.z <= cz + half);
        }
    }

    /// v6.5.3: the EDIT WINDOW — a decodable dirty cell streams with its
    /// reset electron and the cone/zone phases go silent for
    /// EDIT_WINDOW_TICKS so the client's particle pool drains (run #204:
    /// the edit's dots were rejected by the saturated pool and the marker
    /// became undeliverable on a long-lived client while a fresh client
    /// decoded it instantly). After the window the steady state resumes.
    #[test]
    fn edit_window_silences_steady_state_and_recovers() {
        let (cx, cz, region) = (-930.0, 796.0, 512);
        let half = f64::from(region) * 0.5;
        let mut s = Streamer::new(cx - half, cz - half, region, 2);
        for cz in 0..256u32 {
            for cx in 0..256u32 {
                s.set_cell(cx, cz, cell(70, false, [100, 100, 100]), 0);
            }
        }
        let view_at = |now: u64| ViewHint {
            eye: [cx - 75.0, 150.0 + 1.62, cz],
            look: look_dir(-90.0, 5.0),
            now,
            age_ticks: u64::MAX, // settled client
        };
        // the steady state runs (nothing dirty yet)
        let v = view_at(400);
        let _: Vec<Spawn> = s.tick_batch(2048, &[v]).to_vec();
        assert!(s.cone_cells + s.zone_cells > 0, "steady state must stream");
        assert_eq!(s.edit_window, 0, "no edits -> no window");

        // THE EDIT: the disk-center cell (decodable, dead ahead) changes.
        s.set_cell(128, 128, cell(170, false, [142, 33, 33]), 401);
        assert_eq!(s.dirty_pending, 1);
        // Beat 1 — HOLD: the wire goes silent FIRST (the pool drains), the
        // edit batch waits; the window covers hold + post.
        let v = view_at(401);
        let batch: Vec<Spawn> = s.tick_batch(2048, &[v]).to_vec();
        assert!(batch.is_empty(), "the hold must silence the wire first");
        assert_eq!(s.edit_hold, EDIT_HOLD_TICKS, "the hold arms");
        assert_eq!(
            s.edit_window,
            EDIT_HOLD_TICKS + EDIT_WINDOW_TICKS,
            "the window covers hold + post"
        );
        for t in 402..(401 + EDIT_HOLD_TICKS) {
            let v = view_at(t);
            let batch: Vec<Spawn> = s.tick_batch(2048, &[v]).to_vec();
            assert!(batch.is_empty(), "the hold stays silent (t={t})");
        }
        assert_eq!(s.edit_hold, 1, "one hold tick left (it releases in-tick)");
        // Beat 2 — RELEASE: the next tick's decrement hits zero and the
        // batch streams into the drained pool IN THE SAME TICK.
        let v = view_at(401 + EDIT_HOLD_TICKS);
        let batch: Vec<Spawn> = s.tick_batch(2048, &[v]).to_vec();
        assert!(
            batch.iter().any(|sp| sp.y == RESET_Y),
            "the reset electron must ride the released edit batch"
        );
        let cone_after_edit = s.cone_cells;
        // Beat 3 — POST: silence covers the released dots' lifetime.
        let v = view_at(402 + EDIT_HOLD_TICKS);
        let batch: Vec<Spawn> = s.tick_batch(2048, &[v]).to_vec();
        assert!(batch.is_empty(), "the post window must stay silent");
        assert_eq!(
            s.cone_cells, cone_after_edit,
            "no cone streams in the post window"
        );
        // v6.6.2 REBEAT, v6.7.1 MULTI-BEAT: REBEAT_DELAY_TICKS after the
        // release the streamed cells re-dirty, the post silence is cut and
        // the whole hold→release sequence reruns — with the default
        // edit_beats=2 the edit gets TWO retries (three complete-decode
        // chances) for a fragile sparse color word (RED index 2 = one
        // active dot). Each retry carries its own reset electron.
        let rb = 401 + EDIT_HOLD_TICKS + REBEAT_DELAY_TICKS;
        let v = view_at(rb);
        let _: Vec<Spawn> = s.tick_batch(2048, &[v]).to_vec();
        assert_eq!(
            s.edit_hold, EDIT_HOLD_TICKS,
            "the rebeat must re-arm the hold"
        );
        let mut resets = Vec::new();
        for t in (rb + 1)..(rb + EDIT_HOLD_TICKS + 2) {
            let v = view_at(t);
            let batch: Vec<Spawn> = s.tick_batch(2048, &[v]).to_vec();
            if batch.iter().any(|sp| sp.y == RESET_Y) {
                resets.push(t);
            }
        }
        assert_eq!(
            resets.len(),
            1,
            "the second beat must release with its reset electron"
        );
        assert_eq!(s.dirty_pending, 0, "the rebeat released the edit again");
        assert_eq!(
            s.rebeat_diag(),
            (1, 0),
            "retry 2 of 2 is scheduled, the episode counter is spent"
        );
        // The third beat (retry 2 of edit_beats=2) fires one delay later and
        // releases with its own reset electron — the LAST one.
        let rb3 = resets[0] + REBEAT_DELAY_TICKS;
        let v = view_at(rb3);
        let _: Vec<Spawn> = s.tick_batch(2048, &[v]).to_vec();
        assert_eq!(
            s.edit_hold, EDIT_HOLD_TICKS,
            "the third beat must re-arm the hold"
        );
        let mut third_reset = 0u64;
        for t in (rb3 + 1)..(rb3 + EDIT_HOLD_TICKS + 2) {
            let v = view_at(t);
            let batch: Vec<Spawn> = s.tick_batch(2048, &[v]).to_vec();
            if batch.iter().any(|sp| sp.y == RESET_Y) {
                third_reset += 1;
            }
        }
        assert_eq!(
            third_reset, 1,
            "the third beat must release with its reset electron"
        );
        assert_eq!(s.dirty_pending, 0, "the third beat released the edit");
        // ...and it must NOT loop: the episode counter is spent, so tick
        // past the third post window — the window expires and nothing
        // re-dirties (a fourth beat would re-arm the hold).
        for t in (rb3 + EDIT_HOLD_TICKS + 2)..(rb3 + EDIT_HOLD_TICKS + EDIT_WINDOW_TICKS + 10) {
            let v = view_at(t);
            let _: Vec<Spawn> = s.tick_batch(2048, &[v]).to_vec();
        }
        assert_eq!(s.edit_window, 0, "the window must expire (no rebeat loop)");
        assert_eq!(
            s.rebeat_diag(),
            (0, 0),
            "no pending entries and no beats left after the last retry"
        );
        assert!(
            s.cone_cells > cone_after_edit,
            "the steady state resumes after the window ({} -> {})",
            cone_after_edit,
            s.cone_cells
        );

        // v6.5.3 gates: a FRESH audience (empty pool, needs the steady state
        // to paint) never arms the window, even on a real edit...
        s.set_cell(128, 128, cell(90, false, [142, 33, 33]), 5001);
        let fresh = ViewHint {
            eye: [cx - 75.0, 150.0 + 1.62, cz],
            look: look_dir(-90.0, 5.0),
            now: 5000,
            age_ticks: 10, // joined 1 s ago
        };
        let _: Vec<Spawn> = s.tick_batch(2048, &[fresh]).to_vec();
        assert_eq!(
            s.edit_window, 0,
            "a fresh client must keep cone/zone running (they are its only paint)"
        );
        // ...and a WAVE (hundreds of decodable dirty cells in one tick) never
        // arms it either — the phase cap saturates, which is the wave
        // signature, and waves feed the initial paint.
        for cz in 120..140u32 {
            for cx in 120..140u32 {
                s.set_cell(cx, cz, cell(80, false, [100, 100, 100]), 5001);
            }
        }
        assert!(s.dirty_pending > 256, "test must build a wave");
        let v = view_at(5001);
        let _: Vec<Spawn> = s.tick_batch(2048, &[v]).to_vec();
        assert_eq!(
            s.edit_window, 0,
            "a dirty wave must not silence the steady state"
        );
    }

    /// v6.9.5 RELOG RE-ARM: after the marker edit delivered once (the
    /// genuine episode), a NEW session (arm_relog_rearm) with a FRESH view
    /// must get the same cells re-delivered through the hold/release
    /// protocol — repeatedly, not once. Run #235's failure was exactly
    /// here: the fresh client's pool saturates and plain re-streams never
    /// land, while the ONE pending rebeat window fired into a still-blind
    /// camera. The re-arm cycles drain+release until the span expires, so
    /// the fresh client sees MULTIPLE reset-electron releases even though
    /// the v6.5.3 fresh-client exemption would normally skip the hold.
    #[test]
    fn relog_rearm_cycles_the_last_edit_through_the_drain() {
        let (cx, cz, region) = (-930.0, 796.0, 512);
        let half = f64::from(region) * 0.5;
        let mut s = Streamer::new(cx - half, cz - half, region, 2);
        for cz in 0..256u32 {
            for cx in 0..256u32 {
                s.set_cell(cx, cz, cell(70, false, [100, 100, 100]), 0);
            }
        }
        let settled = |now: u64| ViewHint {
            eye: [cx - 75.0, 150.0 + 1.62, cz],
            look: look_dir(-90.0, 5.0),
            now,
            age_ticks: u64::MAX,
        };
        let fresh_at = |now: u64, joined: u64| ViewHint {
            eye: [cx - 75.0, 150.0 + 1.62, cz],
            look: look_dir(-90.0, 5.0),
            now,
            age_ticks: now.saturating_sub(joined),
        };
        // The genuine edit: hold -> release -> the cells are recorded.
        let _: Vec<Spawn> = s.tick_batch(2048, &[settled(400)]).to_vec();
        s.set_cell(128, 128, cell(170, false, [142, 33, 33]), 401);
        let _: Vec<Spawn> = s.tick_batch(2048, &[settled(401)]).to_vec();
        for t in 402..(401 + EDIT_HOLD_TICKS) {
            let _: Vec<Spawn> = s.tick_batch(2048, &[settled(t)]).to_vec();
        }
        let v = settled(401 + EDIT_HOLD_TICKS);
        let batch: Vec<Spawn> = s.tick_batch(2048, &[v]).to_vec();
        assert!(
            batch.iter().any(|sp| sp.y == RESET_Y),
            "the genuine release must carry the reset electron"
        );
        assert_eq!(
            s.rearm_diag(461),
            (0, 0),
            "no re-arm before a session change"
        );

        // THE RELOG: a fresh client joins (the engine's per-player entry is
        // brand-new) and the streamer is armed.
        let joined = 460;
        s.arm_relog_rearm(470);
        assert_eq!(
            s.rearm_diag(470).0,
            1,
            "the last edit's single cell is the re-arm material"
        );
        // The pending rebeat entry (scheduled by the genuine release) fires
        // REBEAT_DELAY_TICKS out; the FRESH audience must take the HOLD path
        // now (the re-arm span suspends the v6.5.3 straight-through).
        let redirty = 470 + 40;
        let v = fresh_at(redirty, joined);
        let _: Vec<Spawn> = s.tick_batch(2048, &[v]).to_vec();
        assert_eq!(
            s.edit_hold, EDIT_HOLD_TICKS,
            "the re-armed cells must go through the drain even for a fresh audience"
        );
        // Hold ticks: the drain (the wire stays silent).
        for t in (redirty + 1)..(redirty + EDIT_HOLD_TICKS) {
            let v = fresh_at(t, joined);
            let _: Vec<Spawn> = s.tick_batch(2048, &[v]).to_vec();
        }
        // Release: the reset electron rides again — the fresh client gets a
        // complete-decode chance into its (drained) pool.
        let v = fresh_at(redirty + EDIT_HOLD_TICKS, joined);
        let batch: Vec<Spawn> = s.tick_batch(2048, &[v]).to_vec();
        assert!(
            batch.iter().any(|sp| sp.y == RESET_Y),
            "the relog release must carry the reset electron"
        );
        // ...and the cycle must REPEAT (that is the determinism fix): keep
        // ticking the span and count further reset-electron releases.
        let mut releases = 1u32;
        for t in (redirty + EDIT_HOLD_TICKS + 1)..(joined + 400) {
            let v = fresh_at(t, joined);
            let batch: Vec<Spawn> = s.tick_batch(2048, &[v]).to_vec();
            if batch.iter().any(|sp| sp.y == RESET_Y) {
                releases += 1;
                assert_eq!(
                    s.dirty_pending, 0,
                    "each relog release must drain the edit queue (t={t})"
                );
            }
        }
        assert!(
            releases >= 3,
            "the re-arm must cycle the drain+release repeatedly (got {releases})"
        );
        assert!(
            s.rearm_diag(joined + 400).1 > 0,
            "the span must still be live mid-window"
        );
    }

    /// v6.9.5: the re-arm is inert without edit material (a join before any
    /// genuine edit changes nothing — the boot paint flows as before) and
    /// stops cycling once the span expires.
    #[test]
    fn rearm_needs_edit_material_and_expires_with_the_span() {
        let (cx, cz, region) = (-930.0, 796.0, 512);
        let half = f64::from(region) * 0.5;
        let mut s = Streamer::new(cx - half, cz - half, region, 2);
        for cz in 0..256u32 {
            for cx in 0..256u32 {
                s.set_cell(cx, cz, cell(70, false, [100, 100, 100]), 0);
            }
        }
        let fresh_at = |now: u64, joined: u64| ViewHint {
            eye: [cx - 75.0, 150.0 + 1.62, cz],
            look: look_dir(-90.0, 5.0),
            now,
            age_ticks: now.saturating_sub(joined),
        };
        // A join with NOTHING edited yet: the arm is a no-op and no hold
        // ever arms (the fresh straight-through path serves the boot paint).
        s.arm_relog_rearm(100);
        assert_eq!(
            s.rearm_diag(100),
            (0, 0),
            "no edit material -> the re-arm stays off"
        );
        for t in 100..140 {
            let v = fresh_at(t, 100);
            let _: Vec<Spawn> = s.tick_batch(2048, &[v]).to_vec();
            assert_eq!(s.edit_hold, 0, "no material -> no drain (t={t})");
        }

        // NOW a genuine edit, a release, and a relog: the span cycles —
        // until it expires. Past the span no new entries are queued and the
        // steady state resumes with no holds and no reset electrons.
        let settled = ViewHint {
            eye: [cx - 75.0, 150.0 + 1.62, cz],
            look: look_dir(-90.0, 5.0),
            now: 200,
            age_ticks: u64::MAX,
        };
        let _: Vec<Spawn> = s.tick_batch(2048, &[settled]).to_vec();
        s.set_cell(128, 128, cell(170, false, [142, 33, 33]), 201);
        let _: Vec<Spawn> = s.tick_batch(2048, &[settled]).to_vec();
        for t in 202..(201 + EDIT_HOLD_TICKS) {
            let v = ViewHint {
                eye: [cx - 75.0, 150.0 + 1.62, cz],
                look: look_dir(-90.0, 5.0),
                now: t,
                age_ticks: u64::MAX,
            };
            let _: Vec<Spawn> = s.tick_batch(2048, &[v]).to_vec();
        }
        let v = ViewHint {
            eye: [cx - 75.0, 150.0 + 1.62, cz],
            look: look_dir(-90.0, 5.0),
            now: 201 + EDIT_HOLD_TICKS,
            age_ticks: u64::MAX,
        };
        let _: Vec<Spawn> = s.tick_batch(2048, &[v]).to_vec();
        let joined = 400;
        s.arm_relog_rearm(joined);
        let span_end = joined + REARM_SPAN_TICKS;
        // The span is live: holds DO arm for the fresh audience.
        let mut held = false;
        let mut late_resets = 0u32;
        for t in joined..(span_end + REBEAT_DELAY_TICKS + EDIT_HOLD_TICKS + 60) {
            let v = fresh_at(t, joined);
            let batch: Vec<Spawn> = s.tick_batch(2048, &[v]).to_vec();
            if t < span_end && s.edit_hold > 0 {
                held = true;
            }
            if t >= span_end + REBEAT_DELAY_TICKS + EDIT_HOLD_TICKS
                && batch.iter().any(|sp| sp.y == RESET_Y)
            {
                late_resets += 1;
            }
        }
        assert!(held, "the span must arm holds while live");
        assert_eq!(
            late_resets, 0,
            "past the span + one drain cycle, the re-arm must be silent"
        );
        assert_eq!(
            s.last_edit.len(),
            1,
            "the material stays recorded after the span"
        );
        assert_eq!(
            s.rearm_diag(span_end + REBEAT_DELAY_TICKS + EDIT_HOLD_TICKS + 59),
            (0, 0),
            "the diag reports the span off once it expires"
        );
    }

    /// v6.9.5: the re-arm's material is the GENUINE edit's cells only — a
    /// wave streamed straight through must not overwrite it (the boot
    /// backlog is wave material by definition; the relog re-arm exists for
    /// marker-sized edits).
    #[test]
    fn wave_stream_does_not_overwrite_rearm_material() {
        let (cx, cz, region) = (-930.0, 796.0, 512);
        let half = f64::from(region) * 0.5;
        let mut s = Streamer::new(cx - half, cz - half, region, 2);
        for cz in 0..256u32 {
            for cx in 0..256u32 {
                s.set_cell(cx, cz, cell(70, false, [100, 100, 100]), 0);
            }
        }
        let settled = |now: u64| ViewHint {
            eye: [cx - 75.0, 150.0 + 1.62, cz],
            look: look_dir(-90.0, 5.0),
            now,
            age_ticks: u64::MAX,
        };
        // A small genuine edit: hold -> release -> recorded.
        let _: Vec<Spawn> = s.tick_batch(2048, &[settled(400)]).to_vec();
        s.set_cell(128, 128, cell(170, false, [142, 33, 33]), 401);
        let _: Vec<Spawn> = s.tick_batch(2048, &[settled(401)]).to_vec();
        for t in 402..(401 + EDIT_HOLD_TICKS) {
            let _: Vec<Spawn> = s.tick_batch(2048, &[settled(t)]).to_vec();
        }
        let _: Vec<Spawn> = s
            .tick_batch(2048, &[settled(401 + EDIT_HOLD_TICKS)])
            .to_vec();
        assert_eq!(
            s.last_edit.len(),
            1,
            "the genuine edit's single cell is recorded"
        );

        // A WAVE (400 fresh decodable dirty cells) streams straight through
        // and must NOT become the re-arm material.
        for cz in 120..140u32 {
            for cx in 120..140u32 {
                s.set_cell(cx, cz, cell(80, false, [100, 100, 100]), 5001);
            }
        }
        assert!(s.dirty_pending > 256, "test must build a wave");
        let _: Vec<Spawn> = s.tick_batch(2048, &[settled(5001)]).to_vec();
        assert_eq!(
            s.last_edit.len(),
            1,
            "the wave must not overwrite the re-arm material"
        );
    }

    /// v6.9.8 (#238's `rearm 1c` lesson): the re-arm material is a STAMPED
    /// SET merged across releases — a small single-cell background edit
    /// (the rescan/event channel keeps firing cosmetically after the
    /// marker) ADDS itself instead of overwriting the marker's cells, and
    /// entries age out of the set after REARM_EDIT_WINDOW_TICKS.
    #[test]
    fn background_edit_merges_and_ages_out_of_rearm_material() {
        let (cx, cz, region) = (-930.0, 796.0, 512);
        let half = f64::from(region) * 0.5;
        let mut s = Streamer::new(cx - half, cz - half, region, 2);
        for cz in 0..256u32 {
            for cx in 0..256u32 {
                s.set_cell(cx, cz, cell(70, false, [100, 100, 100]), 0);
            }
        }
        let settled = |now: u64| ViewHint {
            eye: [cx - 75.0, 150.0 + 1.62, cz],
            look: look_dir(-90.0, 5.0),
            now,
            age_ticks: u64::MAX,
        };
        // The marker edit (the disk-center cell) delivers and records.
        let _: Vec<Spawn> = s.tick_batch(2048, &[settled(400)]).to_vec();
        s.set_cell(128, 128, cell(170, false, [142, 33, 33]), 401);
        let _: Vec<Spawn> = s.tick_batch(2048, &[settled(401)]).to_vec();
        for t in 402..(401 + EDIT_HOLD_TICKS) {
            let _: Vec<Spawn> = s.tick_batch(2048, &[settled(t)]).to_vec();
        }
        let _: Vec<Spawn> = s
            .tick_batch(2048, &[settled(401 + EDIT_HOLD_TICKS)])
            .to_vec();
        assert_eq!(s.last_edit.len(), 1, "the marker cell is recorded");

        // A small background edit east of the vantage (decodable, in cone —
        // cell (129,128), whose LO probe is still inside the band) delivers
        // ~60 ticks later and must MERGE. The marker's own rebeat entry
        // re-dirties the marker cell mid-window (the post-silence cuts),
        // so both cells ride ONE release a few ticks past the naive
        // hold+1 schedule — the loop ticks until the merge registers.
        s.set_cell(129, 128, cell(90, false, [142, 33, 33]), 500);
        let mut merged = false;
        for t in 501..700 {
            let _: Vec<Spawn> = s.tick_batch(2048, &[settled(t)]).to_vec();
            if s.last_edit.len() == 2 {
                merged = true;
                break;
            }
        }
        assert!(
            merged,
            "the background edit must merge, not overwrite ({:?})",
            s.last_edit
        );
        let last_stamp = s.last_edit.iter().map(|&(_, t)| t).max().unwrap();

        // The re-arm reports the FRESH entries; every hold release refreshes
        // its cells' stamps, so both entries carry the merge tick and stay
        // fresh across the whole span. The aging-out is verified directly
        // on the merge (the span is only 3000 ticks — shorter than the
        // 9000-tick window).
        s.arm_relog_rearm(600);
        assert_eq!(s.rearm_diag(600).0, 2, "both entries are fresh at arm time");
        assert_eq!(
            s.rearm_diag(3599).0,
            2,
            "both entries stay fresh across the whole span"
        );
        s.record_edit_cells(last_stamp + REARM_EDIT_WINDOW_TICKS, &[]);
        assert_eq!(
            s.last_edit.len(),
            0,
            "both entries age out one window after the last delivery"
        );
    }

    /// v6.9.10: a fresh edit's cells stream FIRST, ahead of a backlog that
    /// would otherwise fill the batch cap (#240's LEG 1: the fill cells sat
    /// at the tail of the 12k backlog queue; the pop filled the cap on
    /// decode-band backlog duplicates and the marker never reached the
    /// client). With the priority pass the fresh cell rides the release
    /// front even when the cap is tiny and the backlog is huge.
    #[test]
    fn fresh_edits_stream_before_the_backlog_tail() {
        let (cx, cz, region) = (-930.0, 796.0, 512);
        let half = f64::from(region) * 0.5;
        let mut s = Streamer::new(cx - half, cz - half, region, 2);
        for cz in 0..256u32 {
            for cx in 0..256u32 {
                s.set_cell(cx, cz, cell(70, false, [100, 100, 100]), 0);
            }
        }
        let settled = |now: u64| ViewHint {
            eye: [cx - 75.0, 150.0 + 1.62, cz],
            look: look_dir(-90.0, 5.0),
            now,
            age_ticks: u64::MAX,
        };
        // A deep backlog of OLD decodable dirty cells (east band, aged past
        // RECENT_EDIT_TICKS) — far more dots than any release cap.
        for cz in 110..118u32 {
            for cx in 105..145u32 {
                s.set_cell(cx, cz, cell(90, false, [142, 33, 33]), 100);
            }
        }
        assert!(s.dirty_pending > 250, "test must build a deep backlog");
        // THE FRESH EDIT lands ~800 ticks later (the backlog is stale by
        // then): the genuine-edit decision arms on it alone.
        s.set_cell(128, 128, cell(170, false, [142, 33, 33]), 900);
        let _: Vec<Spawn> = s.tick_batch(64, &[settled(900)]).to_vec();
        for t in 901..960 {
            let _: Vec<Spawn> = s.tick_batch(64, &[settled(t)]).to_vec();
        }
        // THE RELEASE (cap 64 dots — the backlog alone would fill it): the
        // fresh cell must ride the priority front.
        let batch: Vec<Spawn> = s.tick_batch(64, &[settled(960)]).to_vec();
        let fresh_idx = 128usize * 256 + 128;
        assert!(
            !s.dirty[fresh_idx],
            "the fresh edit must stream despite the cap-filling backlog"
        );
        assert!(
            s.dirty_pending > 250,
            "the stale backlog must still be queued (the cap cut it)"
        );
        let in_fresh_zone = batch
            .iter()
            .any(|sp| (-930.5..=-927.5).contains(&sp.x) && (795.5..=798.5).contains(&sp.z));
        assert!(in_fresh_zone, "the fresh cell's dots must be in the batch");
        // ...and the fresh cell's reset electron rode along.
        assert!(
            batch.iter().any(|sp| sp.y == RESET_Y),
            "the fresh cell streams as an edit (reset electron)"
        );
    }

    /// v6.9.6 (#236's lesson): the relog client joins at SPAWN — hundreds of
    /// blocks from the marker — so the re-armed cells go dirty while
    /// UNDECODABLE and sit in the queue through the whole prompt+tp window;
    /// their dirty_since ages past RECENT_EDIT_TICKS and the genuine-edit
    /// decision stops counting them: the hold never arms, the words stream
    /// wave-style into the saturated pool, red=0 on every probe. The fix
    /// re-stamps the still-queued cells as RECENT while the span is live,
    /// so the FIRST post-tp decision arms the drain and the release lands.
    #[test]
    fn rearm_stays_armable_through_a_stale_queued_episode() {
        let (cx, cz, region) = (-930.0, 796.0, 512);
        let half = f64::from(region) * 0.5;
        let mut s = Streamer::new(cx - half, cz - half, region, 2);
        for cz in 0..256u32 {
            for cx in 0..256u32 {
                s.set_cell(cx, cz, cell(70, false, [100, 100, 100]), 0);
            }
        }
        let settled = |now: u64| ViewHint {
            eye: [cx - 75.0, 150.0 + 1.62, cz],
            look: look_dir(-90.0, 5.0),
            now,
            age_ticks: u64::MAX,
        };
        // The genuine edit delivers once and records its cell.
        let _: Vec<Spawn> = s.tick_batch(2048, &[settled(400)]).to_vec();
        s.set_cell(128, 128, cell(170, false, [142, 33, 33]), 401);
        let _: Vec<Spawn> = s.tick_batch(2048, &[settled(401)]).to_vec();
        for t in 402..(401 + EDIT_HOLD_TICKS) {
            let _: Vec<Spawn> = s.tick_batch(2048, &[settled(t)]).to_vec();
        }
        let _: Vec<Spawn> = s
            .tick_batch(2048, &[settled(401 + EDIT_HOLD_TICKS)])
            .to_vec();
        assert_eq!(s.last_edit.len(), 1, "the edit cell is recorded");

        // The relog client joins at SPAWN (the far corner of the disk — the
        // marker cells are NOT decodable from there).
        let joined = 460;
        let spawn_view = |now: u64| ViewHint {
            eye: [0.5, 70.0, 0.5],
            look: look_dir(0.0, 0.0),
            now,
            age_ticks: now.saturating_sub(joined),
        };
        s.arm_relog_rearm(470);
        // The re-arm entry pops ~40 ticks out; the cell goes dirty and the
        // spawn view cannot decode it — it must sit queued, no hold, and
        // (without the re-stamp) age out of the RECENT window.
        let mut queued_seen = false;
        let mut held_while_blind = false;
        for t in 470..900 {
            let v = spawn_view(t);
            let _: Vec<Spawn> = s.tick_batch(2048, &[v]).to_vec();
            if s.dirty_pending > 0 {
                queued_seen = true;
            }
            if s.edit_hold > 0 {
                held_while_blind = true;
            }
        }
        assert!(queued_seen, "the cell must sit queued while undecodable");
        assert!(
            !held_while_blind,
            "no drain may arm for a view that cannot decode the cells"
        );

        // THE TP: the view arrives at the vantage. The re-stamp must have
        // kept the cell RECENT, so the very first decisions arm the drain.
        let mut armed = false;
        let mut released = false;
        for t in 900..1100 {
            let v = settled(t);
            let batch: Vec<Spawn> = s.tick_batch(2048, &[v]).to_vec();
            if s.edit_hold == EDIT_HOLD_TICKS {
                armed = true;
            }
            if batch.iter().any(|sp| sp.y == RESET_Y) {
                released = true;
                break;
            }
        }
        assert!(
            armed,
            "the post-tp decision must arm the drain (the re-stamp)"
        );
        assert!(
            released,
            "the post-tp release must carry the reset electron into the drained pool"
        );
    }

    /// v6.7.1 MULTI-BEAT, knob=0: the v6.6.1 single-shot edit — the release
    /// streams the batch once and NOTHING re-dirties afterwards (no rebeat
    /// entries, no re-arms). edit_beats=0 must be the escape hatch that
    /// restores the pre-rebeat behavior exactly.
    #[test]
    fn edit_beats_zero_disables_rebeats() {
        let (cx, cz, region) = (-930.0, 796.0, 512);
        let half = f64::from(region) * 0.5;
        let mut s = Streamer::new(cx - half, cz - half, region, 2);
        s.edit_beats = 0;
        for cz in 0..256u32 {
            for cx in 0..256u32 {
                s.set_cell(cx, cz, cell(70, false, [100, 100, 100]), 0);
            }
        }
        let view_at = |now: u64| ViewHint {
            eye: [cx - 75.0, 150.0 + 1.62, cz],
            look: look_dir(-90.0, 5.0),
            now,
            age_ticks: u64::MAX,
        };
        let _: Vec<Spawn> = s.tick_batch(2048, &[view_at(400)]).to_vec();
        s.set_cell(128, 128, cell(170, false, [142, 33, 33]), 401);
        let t_rel = 401 + EDIT_HOLD_TICKS;
        let mut released = false;
        for t in 401..=t_rel + 1 {
            let v = view_at(t);
            let batch: Vec<Spawn> = s.tick_batch(2048, &[v]).to_vec();
            if batch.iter().any(|sp| sp.y == RESET_Y) {
                released = true;
            }
        }
        assert!(released, "the single release must carry the reset electron");
        assert_eq!(s.rebeat_diag(), (0, 0), "no retry entries at edit_beats=0");
        // Tick well past where the v6.6.2 rebeat would have fired: nothing
        // re-dirties, the steady state just resumes.
        for t in
            (t_rel + 2)..(t_rel + REBEAT_DELAY_TICKS + EDIT_HOLD_TICKS + EDIT_WINDOW_TICKS + 10)
        {
            let _: Vec<Spawn> = s.tick_batch(2048, &[view_at(t)]).to_vec();
        }
        assert_eq!(s.edit_hold, 0, "no retry beat ever re-arms the hold");
        assert_eq!(s.dirty_pending, 0);
        assert_eq!(s.edit_window, 0);
    }

    /// v6.7.1 MULTI-BEAT, knob=3: the episode must produce EXACTLY
    /// edit_beats + 1 releases (the genuine one plus three retries), each
    /// with its own reset electron, and then stop — bounded by the knob,
    /// never a loop. This pins the beat-accounting arithmetic: a genuine
    /// release schedules retry 1 of K and leaves K-1 beats; every
    /// continuation release consumes one and schedules while any remain.
    #[test]
    fn edit_beats_scale_the_episode_exactly() {
        let (cx, cz, region) = (-930.0, 796.0, 512);
        let half = f64::from(region) * 0.5;
        let mut s = Streamer::new(cx - half, cz - half, region, 2);
        s.edit_beats = 3;
        for cz in 0..256u32 {
            for cx in 0..256u32 {
                s.set_cell(cx, cz, cell(70, false, [100, 100, 100]), 0);
            }
        }
        let view_at = |now: u64| ViewHint {
            eye: [cx - 75.0, 150.0 + 1.62, cz],
            look: look_dir(-90.0, 5.0),
            now,
            age_ticks: u64::MAX,
        };
        let _: Vec<Spawn> = s.tick_batch(2048, &[view_at(400)]).to_vec();
        s.set_cell(128, 128, cell(170, false, [142, 33, 33]), 401);
        // March the engine until the episode must be over: 4 beats ×
        // (REBEAT_DELAY + hold + post margin) covers the whole timeline.
        let horizon = 401 + 4 * (REBEAT_DELAY_TICKS + EDIT_HOLD_TICKS + 2) + EDIT_WINDOW_TICKS;
        let mut releases = 0u64;
        for t in 401..=horizon {
            let v = view_at(t);
            let batch: Vec<Spawn> = s.tick_batch(2048, &[v]).to_vec();
            if batch.iter().any(|sp| sp.y == RESET_Y) {
                releases += 1;
            }
        }
        assert_eq!(
            releases, 4,
            "edit_beats=3 must give exactly 1 genuine release + 3 retries"
        );
        assert_eq!(s.rebeat_diag(), (0, 0), "the episode is spent");
        assert_eq!(s.dirty_pending, 0, "every beat released cleanly");
    }

    /// v6.6.1 EDIT RECENCY: the boot-era backlog (minutes-old dirty cells
    /// still queued) must not outvote a genuine fresh edit in the
    /// hold/release decision. The v6.6 CI visual run measured exactly that
    /// pollution: ~12k backlog cells made the decision scan classify the
    /// 25-cell marker edit as a wave, the drain never ran, and the
    /// long-lived client never rendered the edit (red=17 px on every probe)
    /// while a fresh relog decode painted it (red=167) — the words were
    /// right, the silence was missing. 150 STALE decodable dirty cells + 1
    /// fresh edit must ARM the hold; the old queue-wide scan would have
    /// counted 151 and streamed straight through.
    #[test]
    fn stale_backlog_yields_the_edit_window() {
        let mut s = Streamer::new(0.0, 0.0, 256, 2);
        for cz in 0..256u32 {
            for cx in 0..256u32 {
                s.set_cell(cx, cz, cell(70, false, [100, 100, 100]), 0);
            }
        }
        // View A (west-looking — the strip is behind it, never decodable
        // by A) is the only audience during the backlog's aging — the 150
        // stale cells stay queued for the whole warm-up.
        // The stale backlog: 150 cells ALL inside B's band+cone (|dx| <=
        // 60 at dz 32..38, d in (30..110)), marked dirty at tick 1000.
        // They must be decodable-by-B so the OLD code would have counted
        // them; that is the regression being pinned.
        for i in 0..150u32 {
            let cx = 45 + i % 50; // world x 90..188
            let cz = 80 + i / 50; // world z 160..165
            s.set_cell(cx, cz, cell(90 + (i % 8) as i16, false, [90, 90, 90]), 1000);
        }
        assert_eq!(s.dirty_pending, 150);
        // Age the backlog past RECENT_EDIT_TICKS with A alone watching.
        for t in 1000..1400u64 {
            let v = vhint(40.0, 64.0, 90.0, 0.0, t);
            let _: Vec<Spawn> = s.tick_batch(256, &[v]).to_vec();
        }
        assert_eq!(s.dirty_pending, 150, "the backlog must survive A's watch");
        // View B arrives: its band covers the backlog AND the fresh edit.
        // The fresh edit (1 recent decodable cell) must win the window.
        let b = vhint(128.0, 128.0, 0.0, 0.0, 1500);
        s.set_cell(66, 84, cell(170, false, [142, 33, 33]), 1500);
        assert_eq!(s.dirty_pending, 151);
        let a2 = vhint(40.0, 64.0, 90.0, 0.0, 1500);
        let batch: Vec<Spawn> = s.tick_batch(256, &[a2, b]).to_vec();
        assert!(
            batch.is_empty(),
            "a genuine edit must silence the wire first (the hold)"
        );
        assert_eq!(
            s.edit_hold, EDIT_HOLD_TICKS,
            "the hold must arm despite the stale backlog"
        );
        assert_eq!(
            s.edit_window,
            EDIT_HOLD_TICKS + EDIT_WINDOW_TICKS,
            "the window covers hold + post"
        );
        // Release: the fresh edit streams (with its reset electron), the
        // stale backlog stays queued for the wave path.
        let mut released = false;
        for t in 1501..(1501 + EDIT_HOLD_TICKS + 2) {
            let b = vhint(128.0, 128.0, 0.0, 0.0, t);
            let a2 = vhint(40.0, 64.0, 90.0, 0.0, t);
            let batch: Vec<Spawn> = s.tick_batch(256, &[a2, b]).to_vec();
            if batch.iter().any(|sp| sp.y == RESET_Y) {
                released = true;
            }
        }
        assert!(released, "the fresh edit must release with its reset dot");
        assert!(
            s.dirty_pending >= 100,
            "the stale backlog stays queued (wave material, not silently dropped)"
        );
    }

    /// v6.6.1: a FRESH wave (> 100 young decodable dirty cells in one tick)
    /// still streams straight through — recency gating must not turn the
    /// boot wave / a forceload / a flood into 60-tick silences.
    #[test]
    fn fresh_wave_still_streams_through() {
        let mut s = Streamer::new(0.0, 0.0, 256, 2);
        for cz in 0..256u32 {
            for cx in 0..256u32 {
                s.set_cell(cx, cz, cell(70, false, [100, 100, 100]), 0);
            }
        }
        // 150 fresh decodable dirty cells in the view's band+cone (|dx|
        // <= 60 at dz 32..38 — every cell decodable), all marked at the
        // decision tick: a wave by the EDIT_HOLD_MAX_BATCH signature.
        for i in 0..150u32 {
            let cx = 40 + i % 50; // world x 80..178
            let cz = 80 + i / 50; // world z 160..165
            s.set_cell(cx, cz, cell(90 + (i % 8) as i16, false, [90, 90, 90]), 1000);
        }
        assert_eq!(s.dirty_pending, 150);
        let v = vhint(128.0, 128.0, 0.0, 0.0, 1000);
        let _: Vec<Spawn> = s.tick_batch(2048, &[v]).to_vec();
        assert_eq!(s.edit_hold, 0, "a fresh wave never holds");
        assert_eq!(s.edit_window, 0, "a fresh wave never silences the wire");
        assert!(
            s.dirty_streams > 0,
            "the wave must stream immediately (the initial paint depends on it)"
        );
    }

    /// v6.5.2: the /clod-cells forensics window — world->disk coord mapping,
    /// the encoded words, the last-stream age and the dirty flag must all
    /// reflect what the scheduler would actually do for a cell.
    #[test]
    fn debug_window_reports_server_belief() {
        let (cx, cz, region) = (-930.0, 796.0, 512);
        let half = f64::from(region) * 0.5;
        let mut s = Streamer::new(cx - half, cz - half, region, 2);
        // the disk origin lands at (-1186, 540); the named center (-930,796)
        // is cell (128, 128) for cell_size 2.
        s.set_cell(128, 128, cell(170, false, [142, 33, 33]), 0);
        s.set_cell(127, 128, cell(62, true, [10, 20, 30]), 0);
        let dump = s.debug_window(-930, 796, 1, 1000);
        assert!(dump.contains("# cx cz wx wz height water valid rgb last_age dirty hword cword"));
        let center = dump
            .lines()
            .find(|l| l.starts_with("128 128 "))
            .expect("center row");
        assert!(center.contains(" 170 "), "{center}");
        assert!(center.contains(" 0 1 "), "water=0 valid=1: {center}");
        assert!(center.starts_with("128 128 -930 796 "), "{center}");
        // height 170 -> byte 234 = 0xea; the HI word is 0xea00.
        assert!(
            center.contains("0xea00"),
            "HI word must be 0xea00: {center}"
        );
        // the dirty flag is visible and initially clear after set_cell
        // (set_cell marks dirty only on a word CHANGE vs the previous cell).
        let water_row = dump
            .lines()
            .find(|l| l.starts_with("127 128 "))
            .expect("water row");
        assert!(water_row.contains(" 1 1 "), "water=1 valid=1: {water_row}");
        // never-streamed cells report age u64::MAX (no bogus 0).
        assert!(
            center.contains(" 18446744073709551615 "),
            "age must be u64::MAX: {center}"
        );
        // out-of-disk worlds clamp inside the grid instead of panicking.
        let edge = s.debug_window(-99999, 99999, 1, 0);
        assert!(
            edge.lines().count() >= 1,
            "clamped dump must still emit rows"
        );
    }
}
