#!/bin/bash
# v6.59.9 cascade lane (reconstruction from recovery notes §4; the v6.59.8y
# run_cascade_probe.sh lineage died with the sandbox recycle). Boot the
# stand, join, fill the cascade (r0 floor 90), sweep a window at the
# low-shore vantage, then shoot the wall from the FIXED 165/30 vantage.
# Heap budget (v6.59.8x): client 768M + paper 512M + launcher 384M.
set -u
CL_W=${CL_W:-640}
CL_H=${CL_H:-360}
SWEEP_WAIT=${SWEEP_WAIT:-1500}
R0_FLOOR=${R0_FLOOR:-90}
R1_FLOOR=${R1_FLOOR:-25}
CX=-930
CZ=796
SWEEP_X=${SWEEP_X:--336}
SWEEP_Y=${SWEEP_Y:-113}
SWEEP_Z=${SWEEP_Z:--128}
SHOTS=${SHOTS:-/home/z/my-project/c-LoD/tmp551/shots}
mkdir -p "$SHOTS"
cd /home/z/my-project/c-LoD

note() { echo "$(date +%H:%M:%S) $*" | tee -a "$SHOTS/probe.log"; }
srv() { echo "$1" > /tmp/server.in; }
f1() { python3 scripts/bench/press_key.py 67 2>/dev/null || true; }
accept() { # name tries — click the required-pack prompt away
  for _ in $(seq 1 "$2"); do
    DISPLAY=:99 xdotool mousemove 320 208 click 1 2>/dev/null || true
    sleep 4
  done
  note "$1: no pack prompt rendered in time (cached pack auto-accept?)"
}
shot() { # name — one frame, HUD off
  f1; sleep 2
  ffmpeg -nostdin -y -loglevel error -f x11grab -video_size ${CL_W}x${CL_H} \
    -framerate 2 -i :99 -frames:v 1 "$SHOTS/$1.png" 2>>"$SHOTS/ffmpeg.err" \
    && note "shot $1" || note "screenshot $1 failed"
}
cov() { # ring index -> coverage %
  curl -fsS http://127.0.0.1:8199/clod-pack.info 2>/dev/null \
    | grep -o "cascade r$1=[0-9.]*%" | head -1 \
    | grep -o "[0-9.]*%" | tr -d '%'
}

# ---- boot ---------------------------------------------------------------
note "cascade probe: center $CX,$CZ, options cascade=true"
bash scripts/bench/boot_server.sh > /tmp/boot_lane.log 2>&1
sleep 110
bash scripts/bench/run_client.sh > /tmp/client_lane.log 2>&1 &
sleep 140
accept cascade-loop 2

# ---- the fill: ring-0 coverage floor -------------------------------------
DEADLINE=$(( $(date +%s) + 12 * 60 ))
best_r0=0
best_r1=0
while [ "$(date +%s)" -lt "$DEADLINE" ]; do
  r0=$(cov 0)
  r1=$(cov 1)
  best_r0=$(python3 -c "print(max($best_r0, ${r0:-0}))")
  best_r1=$(python3 -c "print(max($best_r1, ${r1:-0}))")
  note "coverage r0=${r0:-?}% r1=${r1:-?}% (best $best_r0/$best_r1)"
  ok=$(python3 -c "print(1 if ${r0:-0} >= $R0_FLOOR and ${r1:-0} >= $R1_FLOOR else 0)")
  [ "$ok" = "1" ] && break
  accept cascade-loop 2
  sleep 30
done
accept cascade-final 5
curl -fsS http://127.0.0.1:8199/clod-pack.info > "$SHOTS/pack-info.txt" 2>/dev/null || true

# ---- the sweep: accumulate the atlas at the low-shore vantage ------------
srv "gamemode spectator Stepan"
srv "tp Stepan $SWEEP_X $SWEEP_Y $SWEEP_Z 0 8"
accept cascade-shot 5
note "sweep vantage $SWEEP_X,$SWEEP_Y,$SWEEP_Z; wait ${SWEEP_WAIT}s (the atlas accumulates)"
sleep "$SWEEP_WAIT"

# ---- the money shots from the FIXED wall vantage (165 m, pitch 30) -------
srv "tp Stepan $CX 165 $CZ 0 30"
sleep 10
shot cascade_aerial
f1
ffmpeg -nostdin -y -loglevel error -f x11grab -video_size ${CL_W}x${CL_H} \
  -framerate 2 -i :99 -frames:v 1 "$SHOTS/cascade_aerial_zoom.png" 2>>"$SHOTS/ffmpeg.err"
# 1.1 burst: three frames 2 s apart (strobe proof)
for i in 1 2 3; do
  ffmpeg -nostdin -y -loglevel error -f x11grab -video_size ${CL_W}x${CL_H} \
    -framerate 2 -i :99 -frames:v 1 "$SHOTS/cascade_burst_$i.png" 2>>"$SHOTS/ffmpeg.err"
  sleep 2
done
srv "tp Stepan $CX 190 $CZ 0 12"
sleep 10
shot cascade_horizon
f1
ffmpeg -nostdin -y -loglevel error -f x11grab -video_size ${CL_W}x${CL_H} \
  -framerate 2 -i :99 -frames:v 1 "$SHOTS/cascade_horizon_zoom.png" 2>>"$SHOTS/ffmpeg.err"

# ---- the gate ------------------------------------------------------------
note "GATE: best r0=$best_r0 (floor $R0_FLOOR), r1=$best_r1 (floor $R1_FLOOR)"
srv "stop"
sleep 8
pkill -f "paper-26.3.jar" 2>/dev/null || true
pkill -f "dev.dist.launcher.Main" 2>/dev/null || true
pkill -f "net.minecraft" 2>/dev/null || true
g0=$(python3 -c "print(1 if $best_r0 >= $R0_FLOOR else 0)")
g1=$(python3 -c "print(1 if $best_r1 >= $R1_FLOOR else 0)")
{ [ "$g0" = "1" ] && [ "$g1" = "1" ] \
  && { note "CASCADE GATE: GREEN"; exit 0; } \
  || { note "CASCADE GATE: RED"; exit 1; }; }
