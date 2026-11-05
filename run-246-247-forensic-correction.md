# c-LoD forensic correction: runs #246 (41913791144, 903cf98 v6.9.14) + #247 (41913921369, 6984b59)

**The 70-frame pixel forensics of the visual-evidence artifacts overturns three readings of the original verdicts and relocates the real failure. This is the pivot that redirected the hunt (and produced the v6.10.0 scene gate).**

## 1. The green-dust carrier probe was a FALSE POSITIVE

The verdict "a stable green carrier exists (366 green px)" measured a static instrument artifact: the green debug-panel frame strip x=952..959, y=65..382, color (0,153,0). Proof: the 366 px stand in EVERY one of the 70 frames of BOTH runs (including no-burst frames), the mask is pixel-identical (overlap 366/366) across runs and across burst states, and the dust burst itself changed exactly 0 pixels vs a no-burst leg-2 frame in both runs. (Later understood fully in #250: the dust command ran with delta 0 0 0 speed 0, stacking all 600 particles at ONE subpixel point at 80 m — the probe never emitted a measurable dust field. Both halves of the probe were dead.)

## 2. The carrier-type hypothesis is killed

#247's console end_rod burst at 80 m flapped exactly like happy_villager (407→354→0 across probes) while NEAR at 20 m stayed stable (543/530/527/579) and the same burst zone explains both. The flap is particle rendering at 80 m on llvmpipe, not the carrier type. happy_villager is exonerated; the carrier migration of v6.9.14 is unjustified.

## 3. A standing censor: the particles:0 pin

run_client.sh:52 still pinned `particles:0` (MINIMAL) from the v6.9.12 instrument. Under MINIMAL vanilla never spawns non-forced particles and forced bursts survive ~19% of attempts (double nextInt(10) in calculateParticleLevel). This explains the eye/strobe/mask zero-deaths and the #243 mystery ("189 bursts arrive, zero rendered"). Removed in v6.10.0 (particles:2 both legs).

## 4. The real instance: leg-2 paints no scene at all

Per-pixel audit of all 70 frames: every leg-2 (relog) frame of #246/#247 is pure sky + the lens-panel column (204800 px) with ZERO non-sky scene pixels (x<940) and is_red=0 over the whole frame, while leg-1 frames from the same vantage carry the LOD terrain strip (is_red 2039/449/1946 across probes). The camera is correct and GL is alive (bursts render on the same client). The fresh-session capture→paint chain never commits on a relog client — the RENDERER DEAD class of #227/#228 survives in leg 2. The `red` metric in the relog probes has been measuring "is there a scene at all", not "did the dots arrive".

## 5. Side effect

The "red stripe = carrier column" reading of v6.9.14 is LOD terrain paint: the stripe stands in the pre-fill baselines, where no carrier existed yet.

**Consequence:** the hunt moved from carriers/transport to the fresh-session capture chain → v6.10.0 scene gate, #248 root cause, and the #250 capture-path narrowing (see the later markers).
