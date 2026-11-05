# c-LoD note: runs #249 (42074636739) + #251 (42082749360) — build-only, no visual signal

Both runs are pushes to fix/leg2-arm-race where the e2e/visual jobs are master-only on push and were skipped: build (3 OS), tests (94), fmt, clippy -D warnings, shader validation, seedgen bytecode, bundle, strictness gate — all green (#249 at 6b0b599 v6.10.1; #251 at 35eb53b, the Task 34 worklog commit).

No visual evidence exists for these heads; the last visual word is #250 (6b0b599, see run-42076419845.md, run-42076419845-forensics.md and run-248-250-independent-audit.md).

Housekeeping: #250's "voxy visual reference" job is stuck in_progress 18+ h after the run failed (runner-side hang, no artifact impact — visual-evidence/control/e2e-logs were uploaded before). The GitHub run page will need a manual cancel; it does not affect any verdict.
