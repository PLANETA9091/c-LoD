#!/usr/bin/env bash
# Quick diagnostic client run: join -> accept pack -> two probes -> shots.
# Lighter than run_e2e_shots.sh: no lap wait, two shots (aerial + ground).
set -uo pipefail
SHOTS="$HOME/mc-test/shots"
mkdir -p "$SHOTS"

CX=-1186; CZ=542

srv() { echo "$1" > /tmp/server.in; }

wait_for() { # pattern, file, timeout_s
  for i in $(seq 1 "$3"); do
    grep -q "$1" "$2" 2>/dev/null && return 0
    sleep 1
  done
  return 1
}

wait_for "pack finalized (ONE-TIME, static)" /tmp/server.log 5 || { echo "server not finalized"; exit 2; }
ps aux | grep "[p]aper-26.3" > /dev/null || { echo "server process dead"; exit 2; }

bash /home/z/my-project/c-LoD/scripts/bench/run_client.sh
wait_for "Stepan joined the game" /tmp/server.log 420 || { echo "client never joined"; tail -20 /tmp/client.log; exit 3; }
echo "client joined"
# Spectator FIRST (before anything can kill the player): tp later needs it.
srv "gamemode spectator Stepan"
sleep 2
srv "tp Stepan -1186 140 542"
sleep 2
sleep 5

# Wait for the pack prompt to render, click Proceed.
PROMPT=0
for i in $(seq 1 60); do
  ffmpeg -y -loglevel error -f x11grab -video_size 1280x720 -framerate 2 -i :99 -frames:v 1 /tmp/prompt_probe.png 2>/dev/null
  if python3 -c "
from PIL import Image
im = Image.open('/tmp/prompt_probe.png').convert('RGB')
cnt = sum(1 for y in range(330, 365) for x in range(380, 900)
          if (lambda p: p[0] > 220 and p[1] > 220 and p[2] < 120)(im.getpixel((x, y))))
print(1 if cnt > 30 else 0)
" | grep -q "^1$"; then
    PROMPT=1; break
  fi
  sleep 2
done
echo "pack prompt visible=$PROMPT"
if [ "$PROMPT" = "1" ]; then
  python3 /home/z/my-project/c-LoD/scripts/bench/click.py 561 385   # Proceed
else
  # The client REMEMBERS the accepted pack from earlier sessions and applies
  # it automatically — no prompt, no click. Clicking blind lands in the world
  # / opens the pause menu. Just wait for the reload below.
  echo "no prompt — pack auto-applies, waiting for reload"
fi
# Wait until the client actually reloads WITH the pack (the race window).
wait_for "Reloading ResourceManager: vanilla, vanilla, server/" /tmp/client.log 60
sleep 10
echo "pack reload settled"

# Lock noon + clear weather (retry console until it echoes back).
for i in 1 2 3; do
  srv "gamerule doDaylightCycle false"
  srv "time set 6000"
  sleep 1
done
srv "weather clear"
srv "gamemode spectator Stepan"
sleep 2
srv "effect give Stepan minecraft:night_vision 5 0 true" # keep the ground shot readable
sleep 1

# Probe 1: aerial west — sky half (march) + terrain half (depth gate).
srv "tp Stepan $CX 100 $CZ 90 15"
sleep 4
ffmpeg -y -loglevel error -f x11grab -video_size 1280x720 -framerate 2 -i :99 -frames:v 1 "$SHOTS/d1_aerial.png"
echo "shot d1_aerial"

# Probe 2: high above, looking down 35 — maximum sky, planes at 80 below.
srv "tp Stepan $CX 100 $CZ 90 15"
sleep 4
ffmpeg -y -loglevel error -f x11grab -video_size 1280x720 -framerate 2 -i :99 -frames:v 1 "$SHOTS/d2_high.png"
echo "shot d2_high"

echo "=== server effect/tier lines ==="
rg -N "effect|tier|stream armed" /tmp/server.log | tail -12 || true
echo "=== client post lines ==="
rg -N -i "post effect|Reloading" /tmp/client.log | tail -8 || true
pkill -f "net.minecraft.client.main.Main" 2>/dev/null || true
echo "DIAG DONE"
