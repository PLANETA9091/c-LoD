#!/usr/bin/env bash
# Graceful CRUSSTY/Paper stop: write `stop` to the fifo, then WAIT for the
# process to exit by itself. SIGKILL mid-save corrupts world_gen_settings.dat
# (seen 15:26 -> "No existing world data" crash on next boot).
set -u
FIFO=/tmp/server.in
[ -p "$FIFO" ] || { echo "no fifo"; exit 1; }

# writer must stay attached long enough for tail to deliver the line
( echo stop > "$FIFO" ) & W=$!
sleep 2
kill $W 2>/dev/null

for i in $(seq 1 150); do
  pgrep -f paper-26.3 >/dev/null || { echo "server exited cleanly after ${i}x2s"; exit 0; }
  sleep 2
done
echo "server still alive after 300s — dumping last lines"
tail -5 /tmp/server.log
exit 1
