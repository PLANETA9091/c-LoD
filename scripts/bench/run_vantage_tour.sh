#!/usr/bin/env bash
# v5.2.2 VANTAGE TOUR: fill the capture buffers from MULTIPLE vantage points.
#
# Physics (established v5.2.1): capture strobes dust only against rendered
# geometry (reversed-Z sky hides particles), client RD8 renders ~128 blocks
# around the camera ground position -> ONE vantage paints only its own
# ~140-block disk. The stream window is 512x512 (origin min corner
# (-930,796) -> window x[-930,-418] z[796,1308], center (-674,1052)).
# This tour hops the camera across the window so the persistent capture
# buffers accumulate the WHOLE disk.
#
# Per leg: tp (spectator, Y=280, pitch 90 = dwell pose) -> DWELL seconds
# -> day lock -> horizon ON shot -> remove lod_terrain -> OFF shot -> re-add
# -> paint_diff metric. Shots land in $SHOTS (default ~/mc-test/shots/v522).
#
# Usage: run_vantage_tour.sh [legs] [dwell_seconds]
#   legs: comma list like "tower,A,B,C,D,E" (default all six)
set -uo pipefail
SHOTS="${SHOTS:-$HOME/mc-test/shots/v522}"
DWELL="${2:-240}"
LEGS="${1:-tower,A,B,C,D,E}"
mkdir -p "$SHOTS"

# leg: yaw_toward_window_center x z
declare -A LX LZ
LX[tower]=-929.5; LZ[tower]=796.5     # min corner (historical dwell point)
LX[A]=-802;      LZ[A]=924            # NW quarter
LX[B]=-546;      LZ[B]=924            # NE quarter
LX[C]=-802;      LZ[C]=1180           # SW quarter
LX[D]=-546;      LZ[D]=1180           # SE quarter
LX[E]=-674;      LZ[E]=1052           # window center
# yaw so the horizon shot looks INTO the window (toward -674,1052)
declare -A YAW
YAW[tower]=45; YAW[A]=135; YAW[B]=225; YAW[C]=315; YAW[D]=45; YAW[E]=0

srv() { echo "$1" > /tmp/server.in; }
shot() { ffmpeg -y -loglevel error -f x11grab -video_size 1280x720 \
          -framerate 2 -i :99 -frames:v 1 "$1" 2>/dev/null; echo "shot $(basename "$1")"; }
metric() { # on off tag
  echo "--- leg $3 paint ---"
  python3 /home/z/my-project/c-LoD/scripts/bench/paint_diff.py "$1" "$2" \
    | tee "$SHOTS/metric_$3.txt"
}

srv "gamemode spectator Stepan"
sleep 2

IFS=',' read -ra PLAN <<< "$LEGS"
for leg in "${PLAN[@]}"; do
  X="${LX[$leg]:-}"; Z="${LZ[$leg]:-}"
  [ -z "$X" ] && { echo "unknown leg $leg"; continue; }
  echo "=== LEG $leg @ ($X,$Z) dwell ${DWELL}s ==="
  srv "tp Stepan $X 280 $Z 0 90"
  sleep 10
  # DWELL in chunks so the day clock stays locked (doDaylightCycle drifts)
  ELAPSED=0
  while [ "$ELAPSED" -lt "$DWELL" ]; do
    sleep 30; ELAPSED=$((ELAPSED+30))
    srv "time set 6000"
  done
  srv "time set 6000"
  # horizon ON shot: yaw into the window, pitch 20
  srv "tp Stepan $X 280 $Z ${YAW[$leg]} 20"
  sleep 12
  shot "$SHOTS/${leg}_ON.png"
  srv "posteffect remove Stepan clod:lod_terrain"
  sleep 12
  shot "$SHOTS/${leg}_OFF.png"
  srv "posteffect add Stepan clod:lod_terrain"
  sleep 3
  srv "posteffect list Stepan"
  sleep 2
  metric "$SHOTS/${leg}_ON.png" "$SHOTS/${leg}_OFF.png" "$leg"
done
echo "VANTAGE TOUR DONE -> $SHOTS"
