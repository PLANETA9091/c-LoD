# Evidence note: the pack-flow asymmetry is MEASURED — and the pack CONTENT is exonerated (bytecode + both leg-2 logs)

Date: 2026-11-07. Artifacts re-downloaded for #248 (42063218460) and #250 (42076419845): `visual-evidence` (full leg-2 `client.log` + `server.log` — the artifact log is the LEG-2 session only: the second `run_client.sh` launch truncates `/tmp/client.log`; #248 covers 16:31:28→16:36:13, #250 covers 17:49:07→17:54:23) and #250's `visual-control-evidence` (the pack-free control arm). This pass mined the pack flow end to end and cross-checked the 26.3 client bytecode.

## 1. The cached-file path is no longer an inference — it is logged, in both runs

Both dead-leg client logs carry the explicit line, at the exact pack-push moment (`server.log`: "pack pushed … 94648 bytes, sha1 b9b7bf855d119ab74118bd82ed37b4cf0b5a9c6f — the ONE push this session", twice per run, same sha1):

- #248 leg-2, 16:31:45: `[Download-2/INFO]: Returning cached file since actual hash matches requested`
- #250 leg-2, 17:49:25: `[Download-2/INFO]: Returning cached file since actual hash matches requested`

leg-1 must download (fresh runner VM → the client dir `downloads/` cache is empty at the first push; the harness observes the yellow prompt + Proceed click every leg-1). The asymmetry leg-1 download / leg-2 cache-hit is now a measured fact.

## 2. But the cached file is byte-identical — pack CONTENT cannot differ between legs

26.3 `HttpUtil.downloadFile` (the class that logs "Returning cached file"): on a cache hit it runs `checkExistingFile` → `cachedFilePath(dir, hash)` → verifies the cached file's hash against the REQUESTED hash → returns that same path (updating its mtime). On a miss it downloads to a temp file, verifies the same hash, moves it into the SAME path. Both legs then feed `DownloadQueue` → `ServerPackManager` → reload the identical bytes at the identical path.

Corollary from the engine doc (engine.rs:1-13): the pack is a **static shader-only artifact** — "ZERO world data: no textures, no PNGs". The dust sprite comes from the VANILLA `particles.png` atlas, and the leg-2 log shows the atlas identical before and after the pack (`Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas` at join reload 17:49:18 AND pack reload 17:49:27 — the x0 is the harness `mipmapLevels:0` pin, all atlases x0). **The pack does not touch the particle path.** "Cached bytes break dust" is dead as a mechanism.

## 3. Reload anomalies closed

- The two `SimplePreparableReloadListener` stack traces in the leg-2 log (join reload AND pack reload, both runs) are the ALSA/OpenAL `Failed to open OpenAL device` → "Error starting SoundSystem. Turning off sounds & music" — an Xvfb environment artifact, harmless, both legs would have it.
- The leg-2 pack reload completed at 17:49:25→17:49:27 (2 s); every probe ran ≥17:50:20. Timing is already compensated by the v6.10.1 re-arm anyway.

## 4. The prompt/acceptance memory (bytecode)

`ServerData$ServerPackStatus` = ENABLED / DISABLED / PROMPT, persisted per server entry via the `acceptTextures` codec (servers.dat) — this is where prompt-vs-auto-accept memory lives. The bench joins via `--quickPlayMultiplayer`, which may not persist the entry, so whether leg-2 prompted-and-clicked or auto-accepted is unresolved from the artifacts — but BOTH paths end in the pack applied (reload logged with `server/00000000/<uuid>` in the source list).

## 5. The reshaped suspect space (replaces the 8b16c22 ranking)

The server side is exonerated by counters (8b16c22), the pack CONTENT is now exonerated by bytecode+logs (§1-2). What remains for "end_rod renders, dust = 0, twice":

1. **Dust type unrenderable on this build/environment — the leg-1 dust cell is STILL UNMEASURED** and is now the PRIMARY discriminator (bfbed5c instrument, rows 1 vs 3). If leg-1 green dust reads ~0 too, the whole bit-stream encoding must migrate its carrier type (P0).
2. **Pack-PRESENCE interaction or relog-session state**: dust renders on leg-1 (if row 1 wins) but not on leg-2 — the capture chains ship with the pack and render on both legs; a chain × dust-pass interaction or relog client state is the live mechanism class. The v6.10.2c cache-wipe control stays valid (it tests the download-vs-cached state machine), but a negative result is now the EXPECTED outcome under suspects 1-2 and exonerates less than originally framed.
3. Acceptance/prompt UI path (click through the shim, focus state) — exotic, third.

**The new cheap discriminator is the CONTROL ARM**: it has only ever fired end_rod — dust on the pack-free client is unmeasured. → v6.10.2d adds a green dust burst to the control arm (see instrument-addendum-control-type-matrix.md): dust renders there → the break needs the pack present (suspect 2) and the leg-1 probe splits chain-interaction vs relog-state; dust ~0 there → suspect 1 confirmed, migrate the carrier.
