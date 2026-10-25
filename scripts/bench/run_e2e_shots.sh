#!/usr/bin/env bash
# Client e2e phase: launch vanilla client, auto-join, screenshot matrix.
# Requires: server already booted and pack finalized (boot_server.sh).
set -uo pipefail
S="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SHOTS="$HOME/mc-test/shots"
mkdir -p "$SHOTS"

srv() { echo "$1" > /tmp/server.in; }

wait_for() { # pattern, file, timeout_s
  for i in $(seq 1 "$3"); do
    grep -q "$1" "$2" 2>/dev/null && return 0
    sleep 1
  done
  return 1
}

# 1. server sanity
wait_for "pack finalized (ONE-TIME, static)" /tmp/server.log 5 || { echo "server not finalized"; exit 2; }
ps aux | grep "[p]aper-26.3" > /dev/null || { echo "server process dead"; exit 2; }
curl -s -o /tmp/pack_check.zip -w "pack HTTP %{http_code} %{size_download}B\n" http://127.0.0.1:8199/clod-pack.zip

# 2. client
bash "$S/run_client.sh"
wait_for "Stepan joined the game" /tmp/server.log 420 || {
  echo "client never joined"; tail -25 /tmp/client.log
  echo "--- server tail (join-side view) ---"; tail -25 /tmp/server.log
  exit 3
}
echo "client joined"
sleep 5
ffmpeg -y -loglevel error -f x11grab -video_size 1280x720 -framerate 2 -i :99 -frames:v 1 "$SHOTS/00_join_state.png"
echo "join-state shot taken (pack prompt expected)"

# the server pack prompt is on screen — click Yes (window 640x360 centered on
# 1280x720 Xvfb: Yes button spans x 489..635, y 376..394 -> center ~562,385).
# Click twice: the second click is a no-op once the dialog is gone, and it
# covers flaky first clicks (Xlib warm-up, dropped XTest events).
# Fresh-sandbox quirk (2026-10-26): python-xlib dies on a missing
# ~/.Xauthority (XauthError) BEFORE clicking, the e2e script has no -e, so
# every click silently no-ops and the pack prompt survives the whole matrix.
touch "$HOME/.Xauthority"
python3 "$S/click.py" 562 385
sleep 12
python3 "$S/click.py" 562 385
sleep 6

# Fresh-sandbox quirk (2026-10-25): the client dies client-side within the
# first second of the world join (spawn-chunk race, no server death message;
# the server keeps it a spectator with Health 0). A click on the death
# screen's Respawn button (window-relative 320,172 -> screen 640,352)
# revives it; the pack is already auto-accepted via servers.dat, so this
# click cannot hit the pack prompt.
python3 "$S/click.py" 640 352
sleep 8
python3 "$S/click.py" 640 352
sleep 4
sleep 12
ffmpeg -y -loglevel error -f x11grab -video_size 1280x720 -framerate 2 -i :99 -frames:v 1 "$SHOTS/00_pack_applied.png"
echo "pack-accept clicked"

# 3. world state: lock NOON + clear weather, then the user's reference matrix.
#    Reference viewpoint (user): seed 4819696040575474753, camera
#    /execute in minecraft:overworld run tp @s -929.67 173.38 796.37 -856.27 15.40
#    (yaw -856.27 normalizes to 223.73 = facing NE, pitch 15.40 slightly down)
srv "gamerule doDaylightCycle false"
srv "time set 6000"
srv "weather clear"
srv "gamemode spectator @a"
sleep 2

UX=-929.67; UY=173.38; UZ=796.37; UYAW=-856.27; UPITCH=15.40

shot() { # name yaw pitch x y z
  srv "tp @a $4 $5 $6 $2 $3"
  sleep 3
  ffmpeg -y -loglevel error -f x11grab -video_size 1280x720 -framerate 2 -i :99 -frames:v 1 "$SHOTS/$1.png"
  echo "shot $1 (yaw=$2 pitch=$3 at $4,$5,$6)"
}

# The reference view is the FIRST shot after the long cross-world tp. The
# post-effect chain re-applies on the client after a teleport and its first
# activation at the new view compiles the 10-pass pipeline (5-15s under
# llvmpipe) — live-observed on CI: shot 01 at tp+3s showed NO LOD band while
# the same view 2 minutes later (08) showed the full band. Settle before the
# reference frame.
sleep 15
shot 01_user_day    "$UYAW" "$UPITCH" "$UX" "$UY" "$UZ"   # the reference view
shot 02_yaw_south    0      "$UPITCH" "$UX" "$UY" "$UZ"
shot 03_yaw_east    90      "$UPITCH" "$UX" "$UY" "$UZ"
shot 04_yaw_north   180     "$UPITCH" "$UX" "$UY" "$UZ"
shot 05_aerial      "$UYAW" "$UPITCH" "$UX" 240  "$UZ"    # higher vantage, same heading

srv "time set 12610"   # dusk — vanilla sunset gradient (red-band probe)
sleep 2
shot 06_sunset     "$UYAW" "$UPITCH" "$UX" "$UY" "$UZ"

srv "time set 18000"   # midnight — water-glow + veil-darkness probe
sleep 2
shot 07_night      "$UYAW" "$UPITCH" "$UX" "$UY" "$UZ"

srv "time set 6000"    # back to noon — sanity that lighting tracks time
sleep 2
shot 08_day_return "$UYAW" "$UPITCH" "$UX" "$UY" "$UZ"

echo "=== client log pack markers ==="
grep -iE "pack|post" /tmp/client.log | tail -12 || true
pkill -f "net.minecraft.client.main.Main" 2>/dev/null || true
echo "E2E CLIENT PHASE DONE"
