#!/bin/bash
# v6.59.9 WINDOW-11 WORLD CHECK (reconstruction of the v6.59.8y script):
# teleport the player into the world square window 11 maps to and shoot
# two frames — the verdict on whether the atlas constant 186 (height 122 m
# under the ±64 code) is a real flat plain or a constant-decode bug.
# Window 11 = ring cells 192..255 x 160..223; with region_blocks=512 the
# ring origin is (center-256, center-256) = (-1186, 540), so the square is
# x 1886..2994, z 3100..4108 — the two probe points sit inside it.
set -u
srv() { echo "$1" > /tmp/server.in; }
srv "gamemode spectator Stepan"
srv "tp Stepan 2200 200 3500 0 8"
sleep 12
ffmpeg -nostdin -y -loglevel error -f x11grab -video_size 640x360 -framerate 2 -i :99 -frames:v 1 /home/z/my-project/c-LoD/tmp551/w11_check.png 2>/dev/null
echo "shot w11_check"
srv "tp Stepan 2600 200 3900 0 8"
sleep 12
ffmpeg -nostdin -y -loglevel error -f x11grab -video_size 640x360 -framerate 2 -i :99 -frames:v 1 /home/z/my-project/c-LoD/tmp551/w11_check2.png 2>/dev/null
echo "shot w11_check2"
grep -i "teleported" /tmp/server.log | tail -2
