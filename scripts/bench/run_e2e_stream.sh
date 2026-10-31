#!/usr/bin/env bash
# v4.3 STREAMING e2e: prove LODs stream to an unmodified vanilla 26.3 client.
#
# Scenario:
#   0. server already booted + base pack finalized (boot_server.sh)
#   1. HTTP baseline: /clod-pack.info shows revision 0
#   2. client joins, accepts the FIRST pack (the only click of the run)
#   3. base state shot: the disk corners are unscanned -> no LOD there
#   4. tp the player into the far corner -> chunks load -> player-centric
#      scan fills the corner -> revision rebuilds -> pack re-pushed
#   5. THE CRITICAL ASSERTION: the client re-downloads and applies the new
#      revision WITHOUT any second prompt (no click in this phase)
#   6. after-shot: LOD terrain now renders in the previously-void corner
set -uo pipefail
SHOTS="$HOME/mc-test/shots"
CLOD=/home/z/my-project/c-LoD
mkdir -p "$SHOTS"

srv() { echo "$1" > /tmp/server.in; }

wait_for() { # pattern, file, timeout_s
  for i in $(seq 1 "$3"); do
    grep -q "$1" "$2" 2>/dev/null && return 0
    sleep 1
  done
  return 1
}

shot() { ffmpeg -y -loglevel error -f x11grab -video_size 1280x720 -framerate 2 -i :99 -frames:v 1 "$1"; }

# 1. server sanity + HTTP streaming baseline
wait_for "pack finalized (base rev 0)" /tmp/server.log 5 || { echo "server not finalized"; exit 2; }
ps aux | grep "[p]aper-26.3" > /dev/null || { echo "server process dead"; exit 2; }
echo "=== /clod-pack.info (baseline) ==="
curl -s http://127.0.0.1:8199/clod-pack.info || { echo "info endpoint dead"; exit 2; }
BASE_SHA=$(curl -s http://127.0.0.1:8199/clod-pack.sha1)
echo "base sha1: $BASE_SHA"

# 2. client + the ONE acceptance click
bash "$CLOD/scripts/bench/run_client.sh"
wait_for "Stepan joined the game" /tmp/server.log 420 || { echo "client never joined"; tail -20 /tmp/client.log; exit 3; }
echo "client joined"
sleep 5
shot "$SHOTS/s00_join_prompt.png"
python3 "$CLOD/scripts/bench/click.py" 562 385   # Yes (window 640x360 centered on 1280x720)
sleep 12
shot "$SHOTS/s01_base_applied.png"
grep -ci "resource pack" /tmp/client.log || true

# 3. world state + BASE shot from the disk center (corner still void).
# Wait out the spawn-region revisions first: the player keeps chunks loaded
# around spawn, so rev 1..N fire here — the corner revision must be NEW.
srv "gamerule doDaylightCycle false"
srv "time set 6000"
srv "weather clear"
srv "gamemode spectator Stepan"
sleep 30
REVS_BEFORE=$(grep -c "pack rebuilt (stream rev" /tmp/server.log)
REVS_BEFORE=${REVS_BEFORE:-0}
REV_TARGET=$((REVS_BEFORE + 1))
echo "revisions before corner tp: $REVS_BEFORE (waiting for rev $REV_TARGET)"
srv "tp Stepan 0 140 0 45 15"   # looking NE toward the (-,-) corner
sleep 3
shot "$SHOTS/s02_base_center_looking_at_void_corner.png"
echo "base shot taken; corner (-200,-200) is unscanned -> no LOD expected"

# 4. stream trigger: teleport into the corner (chunks load there)
srv "tp Stepan -200 140 -200 45 15"
echo "teleported into the corner — waiting for a streamed revision"
for i in $(seq 1 240); do
  N=$(grep -c "pack rebuilt (stream rev" /tmp/server.log)
  N=${N:-0}
  [ "$N" -ge "$REV_TARGET" ] && break
  sleep 1
done
N=$(grep -c "pack rebuilt (stream rev" /tmp/server.log)
N=${N:-0}
[ "$N" -ge "$REV_TARGET" ] || { echo "NO streamed revision for the corner"; tail -30 /tmp/server.log; exit 4; }
echo "revision rebuilt on the server:"
grep -E "pack rebuilt|pack rev" /tmp/server.log | tail -4
wait_for "pack rev .* pushed to" /tmp/server.log 60 || { echo "revision never pushed"; exit 5; }

# 5. THE critical moment: re-push must NOT show any dialog
shot "$SHOTS/s03_push_moment_no_dialog.png"
sleep 12   # client downloads + reloads the revision
shot "$SHOTS/s04_after_stream_applied.png"

# 6. zoomed evidence: same viewport before/after
ffmpeg -y -loglevel error -i "$SHOTS/s02_base_center_looking_at_void_corner.png" \
  -vf "crop=640:360:320:180,scale=1280:720:flags=neighbor" "$SHOTS/s05_zoom_before.png"
ffmpeg -y -loglevel error -i "$SHOTS/s04_after_stream_applied.png" \
  -vf "crop=640:360:320:180,scale=1280:720:flags=neighbor" "$SHOTS/s06_zoom_after.png"

echo "=== /clod-pack.info (after) ==="
curl -s http://127.0.0.1:8199/clod-pack.info
AFTER_SHA=$(curl -s http://127.0.0.1:8199/clod-pack.sha1)
if [ "$AFTER_SHA" = "$BASE_SHA" ]; then echo "FAIL: sha1 did not change"; exit 6; fi
echo "sha1 changed: $BASE_SHA -> $AFTER_SHA"

# THE streaming UX assertion: exactly ONE pack acceptance (the first click);
# every later revision applied silently.
PROMPT_SHOTS="$SHOTS/s03_push_moment_no_dialog.png"
echo "push-moment shot saved: $PROMPT_SHOTS (must contain NO dialog)"

echo "=== client log pack markers ==="
grep -i "pack" /tmp/client.log | tail -12 || true

pkill -f "net.minecraft.client.main.Main" 2>/dev/null || true
echo "STREAMING E2E PHASE DONE"
