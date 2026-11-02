# voxy reference run 41506226890 (#227) — FAILURE (no shots) 2026-11-02T19:03:20Z

The job produced zero screenshots (shots dir exists: yes, sha 0769873).
See run-41506226890/ for client.log, server.log, steps.log and leg.log (tails below).

## steps.log tail (per-step trace written by the job steps)
```
2026-11-02T19:02:45Z X11+GL+pip start
2026-11-02T19:03:06Z X11+GL+pip ok
2026-11-02T19:03:06Z sRGB shim start
2026-11-02T19:03:07Z sRGB shim ok
2026-11-02T19:03:10Z setup_client start
2026-11-02T19:03:11Z setup_client ok
2026-11-02T19:03:11Z fabric layer start
2026-11-02T19:03:11Z voxy-setup: version json ok
2026-11-02T19:03:11Z voxy-setup: downloading fabric installer
2026-11-02T19:03:12Z voxy-setup: running fabric installer
2026-11-02T19:03:15Z voxy-setup: fabric installer done
2026-11-02T19:03:15Z voxy-setup: loader profile ok
2026-11-02T19:03:15Z voxy-setup: mod fetch check: fabric-api-0.165.0+26.3.jar
2026-11-02T19:03:15Z voxy-setup: mod ok: fabric-api-0.165.0+26.3.jar
2026-11-02T19:03:15Z voxy-setup: mod fetch check: sodium-fabric-0.9.2+mc26.3.jar
2026-11-02T19:03:15Z voxy-setup: mod ok: sodium-fabric-0.9.2+mc26.3.jar
2026-11-02T19:03:15Z voxy-setup: mod fetch check: voxy-0.2.20-beta.jar
2026-11-02T19:03:16Z voxy-setup: mod ok: voxy-0.2.20-beta.jar
2026-11-02T19:03:16Z voxy-setup: writing voxy config (rd=8, threads=2)
2026-11-02T19:03:16Z voxy-setup: fabric layer ready
2026-11-02T19:03:16Z fabric layer ok
2026-11-02T19:03:18Z voxy leg start
2026-11-02T19:03:18Z voxy leg: start (full leg stdout/stderr teed to voxy-leg.log)
2026-11-02T19:03:19Z voxy leg: SERVER SCRIPT FAILED rc=1 — the server boot echoes + its voxy_server.log tail are in voxy-leg.log above
```

## client.log tail
```
(no client log)
```

## server.log tail
```
(no server log)
```

## voxy-leg.log tail (the leg's own stdout/stderr)
```
fetching Paper 26.3 (no local jar found)
api ok, downloading paper build ?
cp: '/home/runner/mc-test/voxy-server/paper-26.3.jar' and './paper-26.3.jar' are the same file
```
