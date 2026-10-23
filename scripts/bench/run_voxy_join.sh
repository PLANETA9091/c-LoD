#!/usr/bin/env bash
# Voxy A/B phase 1: launch the Voxy client and get it INTO THE WORLD with the
# c-LoD pack declined. Under llvmpipe the world load + VoxyRenderSystem init
# (MDIC shader compile) takes minutes — the server-side "Stepan joined" line
# fires LONG before the client can render anything (live 2026-10-23: every
# shot came out as the Mojang screen because the old script started the grid
# flight on "joined" and pkilled the still-loading client at the end).
# Waits for the CLIENT-side "Voxy render system created" line instead, then
# declines the pack. Phase 2 (run_voxy_ab.sh) does flight + shots.
set -uo pipefail
SHOTS="$HOME/mc-test/shots"
S="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
mkdir -p "$SHOTS"

srv() { echo "$1" > /tmp/server.in; }
LOG_MARK=$(wc -c < /tmp/server.log 2>/dev/null || echo 0)
wait_for() { for i in $(seq 1 "$3"); do tail -c +$((LOG_MARK+1)) "$2" 2>/dev/null | grep -q "$1" && return 0; sleep 1; done; return 1; }

bash "$S/run_voxy_client.sh"
for i in $(seq 1 20); do [ -f /tmp/voxy_client.log ] && break; sleep 1; done
[ -f /tmp/voxy_client.log ] || { echo "voxy client never launched (java missing)"; exit 3; }
# Fresh-log mark: a dying predecessor client can keep writing to the same
# (truncated) inode — without the mark, its old "render system created" line
# matches instantly and the whole phase runs against a booting client
# (live 2026-10-23). java has truncated the log by the time this runs.
CLOG_MARK=$(wc -c < /tmp/voxy_client.log)
wait_for_v() { for i in $(seq 1 "$2"); do tail -c +$((CLOG_MARK+1)) /tmp/voxy_client.log 2>/dev/null | grep -q "$1" && return 0; sleep 1; done; return 1; }
wait_for "Stepan joined the game" /tmp/server.log 420 || { echo "voxy client never joined"; tail -30 /tmp/voxy_client.log; exit 3; }
echo "voxy client joined (server side)"
wait_for_v "Game crashed" 5 && { echo "VOXY CLIENT CRASHED on join:"; rg -A3 "Game crashed" /tmp/voxy_client.log | head -8; exit 4; }

# Client-side world load + VoxyRenderSystem init + MDIC shader compile: up to
# 6 min under llvmpipe (observed 2-4 min on the 2-core box; ~5s when the
# Mesa on-disk shader cache is warm).
wait_for_v "Voxy render system created" 240 || { echo "voxy render system never came up"; tail -20 /tmp/voxy_client.log; exit 5; }
echo "voxy render system up"
wait_for_v "Game crashed" 5 && { echo "VOXY CLIENT CRASHED during render-system init:"; rg -A3 "Game crashed" /tmp/voxy_client.log | head -8; exit 4; }

# World screen is up (or seconds from it): the server pushes the c-LoD
# resource pack — DECLINE it (No button) so the posteffect LOD never renders
# on top of Voxy's own LODs (clean baseline). servers.dat was reset by
# run_voxy_client.sh, so the prompt is guaranteed to appear.
sleep 20
python3 "$S/click.py" 718 385
sleep 5
# Verification: the pack, if applied, logs sodium's terrain-shader complaint.
if tail -c +$((CLOG_MARK+1)) /tmp/voxy_client.log | grep -q "replaces terrain shaders"; then
  echo "WARNING: c-LoD pack WAS applied — baseline contaminated (click missed?)"
else
  echo "pack not applied (decline ok)"
fi
ffmpeg -y -loglevel error -f x11grab -video_size 1280x720 -framerate 2 -i :99 -frames:v 1 "$SHOTS/20_voxy_join.png"
echo "pack declined, join state captured"
echo "VOXY JOIN PHASE DONE"
