#!/usr/bin/env bash
# Voxy A/B, one call. Clients don't survive between tool calls on this box,
# the world load under llvmpipe+voxy takes ~4-6 min, and the single-call cap
# is 590 s — so the world-readiness detector is pixel-based (the Mojang load
# screen is saturated red at the frame center; the world is not) and the
# grid flight is 3x3 (spacing 192 => coverage ±288 >= the ±256 atlas band).
# Vanilla RD lowered to 4 to cut the login load time (the A/B compares the
# LOD band, not the near seam; c-LoD band stays ±256).
set -uo pipefail
SHOTS="$HOME/mc-test/shots"
S="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
mkdir -p "$SHOTS"

srv() { echo "$1" > /tmp/server.in; }
wait_for() { for i in $(seq 1 "$3"); do tail -c +$((LOG_MARK+1)) "$2" 2>/dev/null | grep -q "$1" && return 0; sleep 1; done; return 1; }
grab() { ffmpeg -y -loglevel error -f x11grab -video_size 1280x720 -framerate 2 -i :99 -frames:v 1 "$1" 2>/dev/null; }

pkill -9 -f "net.fabricmc.loader.impl.launch.knot.KnotClient" 2>/dev/null || true
sleep 1
rm -f "$HOME/mc-test/dist/world/playerdata/"d1652ad9*.dat 2>/dev/null || true
bash "$S/boot_paper_plain.sh"
LOG_MARK=$(wc -c < /tmp/server.log 2>/dev/null || echo 0)
bash "$S/run_voxy_client.sh"
for i in $(seq 1 20); do [ -f /tmp/voxy_client.log ] && break; sleep 1; done
[ -f /tmp/voxy_client.log ] || { echo "voxy client never launched"; exit 3; }
CLOG_MARK=$(wc -c < /tmp/voxy_client.log)
wait_for_v() { for i in $(seq 1 "$2"); do tail -c +$((CLOG_MARK+1)) /tmp/voxy_client.log 2>/dev/null | grep -q "$1" && return 0; sleep 1; done; return 1; }

( CLI_PID=""; for i in $(seq 1 90); do CLI_PID=$(pgrep -f "net.fabricmc.loader.impl.launch.knot.KnotClient" | head -1); [ -n "$CLI_PID" ] && break; sleep 2; done; [ -n "$CLI_PID" ] && for i in $(seq 1 30); do echo "=== t+$((i*20))s ===" >> /tmp/heaplog; "$HOME/mc-test/jdk25/bin/jcmd" "$CLI_PID" GC.heap_info >> /tmp/heaplog 2>&1; if [ "$i" = "6" ] || [ "$i" = "9" ]; then "$HOME/mc-test/jdk25/bin/jcmd" "$CLI_PID" Thread.print > /tmp/tdump_$i.txt 2>&1; fi; sleep 20; done ) &
HEAP_PROBE=$!
wait_for "Stepan joined the game" /tmp/server.log 420 || { echo "voxy client never joined"; tail -20 /tmp/voxy_client.log; exit 3; }
echo "server-side join ok"
wait_for_v "Stepan joined the game" 300 || { echo "client never finished login"; tail -20 /tmp/voxy_client.log; exit 3; }
echo "client-side join ok"
wait_for_v "Game crashed" 5 && { echo "VOXY CLIENT CRASHED:"; rg -A3 "Game crashed" /tmp/voxy_client.log | head -8; exit 4; }

if grep -q "replaces terrain shaders" /tmp/voxy_client.log; then
  echo "NOTE: c-LoD pack present in the resource load (auto-applied — no prompt)"
fi

# World-readiness: the Mojang loading screen is saturated red at the frame
# center. The load ALSO has a DARK phase ("Downloading terrain" dirt bg) that
# a naive non-red check misreads as the world (live 21:34) — so: minimum 100 s
# after the client-side join AND two consecutive non-red samples.
sleep 100
world_up=0
hits=0
for i in $(seq 1 24); do
  grab /tmp/voxy_w.png
  read R G B < <(python3 - <<'PY'
from PIL import Image
im = Image.open('/tmp/voxy_w.png').convert('RGB')
box = im.crop((560, 300, 720, 420))  # center of the 640x360 client window
px = list(box.getdata())
n = len(px)
print(sum(p[0] for p in px)//n, sum(p[1] for p in px)//n, sum(p[2] for p in px)//n)
PY
)
  if [ "${R:-255}" -lt 180 ] || [ "${G:-0}" -gt 120 ]; then
    hits=$((hits+1))
    if [ "$hits" -ge 2 ]; then
      grab /tmp/voxy_w2.png
      if ! cmp -s /tmp/voxy_w.png /tmp/voxy_w2.png; then world_up=1; break; fi
    fi
  else
    hits=0
  fi
  sleep 20
done
[ "$world_up" = 1 ] || { echo "world never finished loading (center still red)"; grab "$SHOTS/20_voxy_join.png"; exit 5; }
echo "world is up"
sleep 10

# Best-effort pack decline (no-op if the pack auto-applied / no dialog).
python3 "$S/click.py" 718 385
sleep 4

srv "gamemode spectator Stepan"
srv "weather clear"
srv "gamerule doDaylightCycle false"
srv "time set 6000"
sleep 2

UX=-929.67; UY=173.38; UZ=796.37; UYAW=-856.27; UPITCH=15.40
i=0
# Strip ALONG the view axis (yaw -856.27deg => facing west/-x). With vanilla
# RD 4 the client receives only a +/-64-block ring per stop; stops must be
# <= 128 apart or the LOD band has HOLES exactly where the shots look
# (live 22:30: stops at 0/-256/+256 left a 128-block empty gap). 4 stops at
# 128 spacing cover +/-256 continuously = the full c-LoD atlas band.
for dx in -192 -64 64 192; do
  i=$((i+1))
  srv "tp Stepan $(( ${UX%.*} + dx )) 200 $(( ${UZ%.*} )) $UYAW $UPITCH"
  sleep 20
done
echo "grid flight done ($i stops) — settling for Voxy meshing"
sleep 20

vshot() { # name time
  srv "time set $2"
  sleep 2
  srv "tp Stepan $UX $UY $UZ $UYAW $UPITCH"
  sleep 7
  grab "$SHOTS/$1.png"
  echo "shot $1 (time=$2, user camera)"
}

vshot 20_voxy_day 6000
vshot 21_voxy_sunset 12610
vshot 22_voxy_night 18000
vshot 23_voxy_day_return 6000

kill "$HEAP_PROBE" 2>/dev/null
# F3 diagnostic overlay: voxy registers a debug-screen entry (sections
# rendered / mesh queues) — one screenshot with F3 held open.
sleep 25
python3 "$S/press_key.py" 68 >/dev/null 2>&1 || true
sleep 2
grab "$SHOTS/29_voxy_f3.png"
python3 "$S/press_key.py" 68 >/dev/null 2>&1 || true
echo "client left ALIVE for the mesh phase (run_voxy_shots.sh next)"
echo "VOXY FLIGHT PHASE DONE"
