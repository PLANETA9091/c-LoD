#!/bin/bash
# v6.59.8x WINDOW-11 WORLD CHECK: после кадров #550 телепортируем игрока
# в мировой квадрат окна 11 (ячейки кольца 192..255 x 160..223 -> x 94..1102,
# z 1308..2332 при origin (-2978,-1252)). Если там суша 122 м — mx=mn=186
# это реальные высоты плоской равнины; если рельеф — декод константно врёт.
set -u
srv() { echo "$1" > /tmp/server.in; }
srv "gamemode spectator Stepan"
srv "tp Stepan 500 200 1800 0 8"
sleep 12
ffmpeg -nostdin -y -loglevel error -f x11grab -video_size 640x360 -framerate 2 -i :99 -frames:v 1 /home/z/my-project/c-LoD/tmp550/w11_check.png 2>/dev/null
echo "shot w11_check"
srv "tp Stepan 800 200 2100 0 8"
sleep 12
ffmpeg -nostdin -y -loglevel error -f x11grab -video_size 640x360 -framerate 2 -i :99 -frames:v 1 /home/z/my-project/c-LoD/tmp550/w11_check2.png 2>/dev/null
echo "shot w11_check2"
grep -i "teleported" /tmp/server.log | tail -2
