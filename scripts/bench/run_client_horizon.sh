#!/usr/bin/env bash
# v6.57 THE HORIZON BRIDGE: launch the Rust Horizon client (the 26.3 rewrite,
# horizon-rust/horizon) instead of the vanilla Java client when MC_CLIENT=horizon.
# Same window/shot contract as the vanilla path: Xvfb :99, /tmp/client.log —
# the bench's ffmpeg x11grab shots capture the Horizon window unchanged, and
# the join is detected from the SERVER log ("Stepan joined the game"), which
# a genuine Horizon login produces exactly like a vanilla one.
# Post-effect parity rides the server-pushed c-LoD resource pack: Horizon's
# naga pipeline runs post_effect/*.json + shaders/post/*.fsh (depth inputs,
# persistent targets) — our capture/dust_mask land there.
set -euo pipefail
HC_BIN="${HORIZON_CLIENT:-$HOME/horizon-src/target/fast/horizon-client}"
H="$HOME/horizon-test"

[ -x "$HC_BIN" ] || { echo "FAIL: horizon-client binary not at $HC_BIN (run setup_horizon.sh)"; exit 1; }

pkill -f horizon-client 2>/dev/null || true
pkill Xvfb 2>/dev/null || true
sleep 1

Xvfb :99 -screen 0 1280x720x24 +extension GLX +render -noreset > /tmp/xvfb.log 2>&1 &
sleep 2
export DISPLAY=:99

# v6.57 SOFTWARE VULKAN: the runner has no GPU; Horizon renders through wgpu,
# so point the Vulkan loader at lavapipe (mesa-vulkan-drivers). Both env
# spellings are set — the loader honours VK_DRIVER_FILES (new) or
# VK_ICD_FILENAMES (old) depending on its version.
LVP=""
for icd in /usr/share/vulkan/icd.d/lvp_icd*.json; do
  [ -e "$icd" ] && LVP="$icd" && break
done
if [ -n "$LVP" ]; then
  export VK_ICD_FILENAMES="$LVP"
  export VK_DRIVER_FILES="$LVP"
else
  echo "WARN: no lavapipe ICD found under /usr/share/vulkan/icd.d — wgpu may fall back to GL"
fi

# The bench contract: the offline name must be Stepan (the server log line
# "Stepan joined the game" is the bench's join gate). Horizon picks the name
# from --name first, then HORIZON_PLAYER_NAME, then a generated PlayerNNNNN —
# both explicit channels are set so a generated name can never win.
export HORIZON_PLAYER_NAME=Stepan
export HORIZON_NO_AUDIO=1
export HORIZON_HOME="$H/home"
mkdir -p "$H/home"

cd "$H"
# --render-distance 8 mirrors the vanilla options.txt pin (renderDistance:8);
# --name Stepan pins the offline identity; --server joins directly (title
# screen is skipped — no ctl dance needed).
nohup "$HC_BIN" --server 127.0.0.1:25565 --render-distance 8 --name Stepan \
  > /tmp/client.log 2>&1 &
CLI=$!
echo "horizon-client pid $CLI (xvfb ok, lavapipe: ${LVP:-none})"
