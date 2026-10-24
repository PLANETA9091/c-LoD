#!/usr/bin/env bash
# Post-finalize settle wait: the LOD scan unforces 16384 chunks; Paper's
# save+unload storm must drain before the client login (the login sits in the
# main-thread queue BEHIND it -> "client never joined").
# Waits until no new [cplug-sdk] flush-timeout lines appear for 90s.
set -u
LOG=/tmp/server.log
BASE=$(grep -c "main-thread flush timed out" "$LOG" 2>/dev/null || echo 0)
echo "baseline flush-timeouts: $BASE"
for i in $(seq 1 60); do
  sleep 10
  CUR=$(grep -c "main-thread flush timed out" "$LOG" 2>/dev/null || echo 0)
  if [ "$CUR" -le "$BASE" ]; then
    echo "quiet after $((i*10))s (timeouts: $BASE -> $CUR)"
    tail -2 "$LOG"
    exit 0
  fi
  BASE=$CUR
  echo "...draining: $CUR queued-flush timeouts so far"
done
echo "still busy after 600s"; exit 1
