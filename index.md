# c-LoD visual verdicts

- 2026-11-06: forensic addendum to #250 — the spawn-gate suspicion rests on STALE doc constants (Y=260/272 vs live LAYER_Y=[80,120]: the whole bit-stream is in-band and in-cone from the bench vantage, both legs); the gate diag is statistically blind (zero firings in #250, early-return + %100 window); the real discriminator is already measured — end_rod vs dust at identical coords on leg-2: 318-324 changed px vs 0 (twice) — and the whole bit-stream is dust: candidate root cause "dust does not render on relog clients"; the missing cell is a leg-1 dust probe (never measured) + optional leg-2 type matrix (see run-250-gate-stale-constants-dust-vs-endrod.md)
- run 42082749360 (#251) 35eb53b 2026-11-06: build-only, visual skipped (see run-249-251-build-only.md)
- run 42076419845 (#250) 6b0b599 2026-11-05T17:54:28Z: FAIL — leg-2 scene dead; forensics + independent audit + CORRECTION (see run-248-250-independent-audit.md)
- run 42074636739 (#249) 6b0b599 2026-11-05T17:28:07Z: build-only, visual skipped (see run-249-251-build-only.md)
- run 42063218460 (#248) d4ff522 2026-11-05T16:36:18Z: FAIL — leg-2 scene dead; root cause: join-time posteffect arm loses pack push/reload race (see run-248-250-independent-audit.md)
- runs 41913791144 (#246) + 41913921369 (#247): verdicts superseded — forensic correction (see run-246-247-forensic-correction.md)
