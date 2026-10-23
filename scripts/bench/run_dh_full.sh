#!/usr/bin/env bash
# One-shot: fresh server + DH reference A/B (corridor flight + shot matrix).
# All-in-one so the sandbox reaper can't split the phases.
set -u
S="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

cleanup() {
  pkill -f "net.fabricmc.loader" 2>/dev/null
  pkill -f "net.minecraft.client.main.Main" 2>/dev/null
  pkill -f launcher.jar 2>/dev/null
  pkill -f paper-26.3 2>/dev/null
  pkill Xvfb 2>/dev/null
  pkill -f "tail -f /tmp/server.in" 2>/dev/null
  return 0
}
trap cleanup EXIT

cleanup
sleep 2

CLOD_SERVER_XMX="${CLOD_SERVER_XMX:-900M}" bash "$S/boot_server.sh" >/dev/null 2>&1
if grep -q "pack finalized (ONE-TIME, static)" /tmp/server.log; then
  echo "server up, starting DH A/B"
else
  echo "SERVER FAILED"; tail -8 /tmp/server.log; exit 1
fi

bash "$S/run_dh_ab.sh" 2>&1 | tail -14
echo "DH AB COMPLETE $(date +%H:%M:%S)"
