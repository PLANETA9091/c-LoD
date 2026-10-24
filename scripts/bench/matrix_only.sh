#!/usr/bin/env bash
set -uo pipefail
SHOTS="$HOME/mc-test/shots"
srv() { echo "$1" > /tmp/server.in; }
shot() { srv "tp Stepan $4 $5 $6 $2 $3"; sleep 3; ffmpeg -y -loglevel error -f x11grab -video_size 1280x720 -framerate 2 -i :99 -frames:v 1 "$SHOTS/$1.png"; echo "shot $1"; }
shot 01_user_day    -856.27 15.40  -929.67 173.38 796.37
shot 02_day_north   180 15  -930 173 796
shot 03_day_east    270 15  -930 173 796
shot 04_day_south     0 15  -930 173 796
shot 05_day_west     90 15  -930 173 796
shot 06_day_aerial  263 35  -930 220 796
shot 07_day_horizon 263  5  -930 200 796
shot 08_day_return -856.27 15.40  -929.67 173.38 796.37
for gx in -1370 -1150 -930 -710 -490; do
  for gz in 376 586 796 1006 1216; do
    shot "g_${gx}_${gz}" 263 22 "$gx" 210 "$gz"
  done
done
echo MATRIX DONE
