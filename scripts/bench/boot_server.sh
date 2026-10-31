#!/usr/bin/env bash
# Boot the Paper 26.3 + CRUSSTY + c-LoD stack; stdin via /tmp/server.in fifo.
# Usage: boot_server.sh   (returns once "pack finalized" appears, gates run)
set -euo pipefail
# BENCH_DIR must be absolute: the script cds into the dist root later,
# and $0-relative paths would break after the cd.
BENCH_DIR="$(cd "$(dirname "$0")" && pwd)"
D="${MC_TEST_DIST:-$HOME/mc-test/dist}"
JAVA="${CLOD_JAVA:-$HOME/tools/jdk25/bin/java}"
# Disk center: override via CLOD_CENTER env ("x,z"), default region spawn
CENTER="${CLOD_CENTER:-0,64}"
# v5.2.2: particles_per_tick 2048 (was 1024) — halves the lap (256 ticks),
# doubling the per-bit decode probability per minute; server CPU headroom
# verified (spawn cost is one packet per particle, 0 failed at 33M+).
export CLOD_OPTS="tick_ms=100;region_blocks=512;cell_size=2;particles_per_tick=2048;scan_budget=4096;http_port=8199;center_x=${CENTER%,*};center_z=${CENTER#*,}"
# 4GB container: Paper+module must fit alongside the client later;
# host OOM-killer targets the fattest RSS — stay small and fast
export DIST_JAVA_OPTS="-Xms256M -Xmx700M -XX:MaxMetaspaceSize=256M -XX:MaxDirectMemorySize=128M -XX:+UseG1GC"

# v5.5 frontier: chunk-system thread tuning (measured A/B on the 2-CPU bench:
# auto=1 gen worker -> 35 cells/s, workers=4 -> 72 cells/s, workers=6 -> 80
# cells/s with a deeper 5m TPS dip; 4/2 is the balanced default). Override or
# disable (empty) via env. The patcher no-ops until Paper writes the config
# (first boot), so the very first boot runs auto — that is the baseline.
export PAPER_WORKERS="${PAPER_WORKERS:-4}"
export PAPER_IO="${PAPER_IO:-2}"

# v5.6 SHUTDOWN DISCIPLINE (crash forensics 2026-10-30, hs_err_pid16818):
# a TERM-then-KILL barrage ("pkill; sleep 1; pkill -9") hit the JVM mid-
# shutdown and SIGSEGV'd the Signal Dispatcher (pc=0, hs_err at 18m39s idle).
# A single clean TERM shuts down cleanly; the historical "idle crash" was the
# session-teardown signal storm, not idle itself. So: TERM once, WAIT for the
# JVM to exit on its own (chunk-save takes seconds), KILL only as last resort.
graceful_stop() {
  for sigpid in $(pgrep -f 'paper-26.3' 2>/dev/null); do kill -TERM "$sigpid" 2>/dev/null || true; done
  for i in $(seq 1 30); do
    pgrep -f 'paper-26.3' >/dev/null 2>&1 || return 0
    sleep 1
  done
  pkill -9 -f 'paper-26.3' 2>/dev/null || true
  sleep 1
}
graceful_stop
pkill -f launcher.jar 2>/dev/null || true
pkill -f 'dev.dist.launcher.Main' 2>/dev/null || true
pkill -f 'libcrussty_runtime' 2>/dev/null || true
# v5.5: the port-8199 holder is the Paper JVM (rewritten argv) — wait until
# the port is actually FREE or the pack HTTP bind fails (os error 98) and
# the session is pack-less (found live in the v5.5 verification boot).
for i in $(seq 1 10); do
  if ! (exec 3<>/dev/tcp/127.0.0.1/8199) 2>/dev/null; then break; fi
  exec 3>&- 2>/dev/null || true
  sleep 1
done
sleep 1

# v5.5 frontier: optional chunk-system thread tuning (A/B via env).
# PAPER_WORKERS=4 PAPER_IO=2 boot_server.sh  ->  patched boot.
python3 "$BENCH_DIR/patch_paper_cfg.py" "$D"

rm -f /tmp/server.in /tmp/server.log
mkfifo /tmp/server.in
cd "$D"
# reader first (tail blocks on open until the writer side opens), then
# hold the write end open for the lifetime of the server
tail -f /tmp/server.in 2>/tmp/tail.err | "$JAVA" -Xmx2G -jar launcher.jar > /tmp/server.log 2>&1 &
SRV=$!
exec 9>/tmp/server.in
echo "launcher pid $SRV"

for i in $(seq 1 150); do
  if grep -q "pack finalized (ONE-TIME, static)" /tmp/server.log 2>/dev/null; then break; fi
  if ! kill -0 $SRV 2>/dev/null; then echo "SERVER DIED"; tail -40 /tmp/server.log; exit 1; fi
  sleep 2
done
grep -q "pack finalized (ONE-TIME, static)" /tmp/server.log || { echo "TIMEOUT waiting for the boot pack"; tail -40 /tmp/server.log; exit 1; }
sleep 2
# v5.5 HTTP gate: the pack must actually be SERVED (a failed bind above
# otherwise surfaces only as a silent client download failure).
HTTP_OK=0
for i in $(seq 1 10); do
  if curl -sf -o /dev/null http://127.0.0.1:8199/clod-pack.zip; then HTTP_OK=1; break; fi
  sleep 1
done
[ "$HTTP_OK" = "1" ] || { echo "HTTP GATE FAILED: pack not served on 8199"; grep -i "http server" /tmp/server.log; exit 1; }
echo "http gate: pack served OK"
echo "=== gates ==="
python3 "$BENCH_DIR/../e2e/gates.py" /tmp/server.log
echo "=== server is up (stdin: /tmp/server.in, log: /tmp/server.log) ==="
