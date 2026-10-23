#!/usr/bin/env bash
# DH LOD-data soak: fabric DH server + DH client parked at the bench camera.
# The server real-time gen + client requests + client-side gen all write into
# the client's DH sqlite; this script measures the growth over ~8 minutes.
set -u
S="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
D="$HOME/mc-test/dh-dist"
JAVA="$HOME/tools/jdk25/bin/java"

cleanup() {
  pkill -f "net.fabricmc.loader" 2>/dev/null
  pkill -f "net.minecraft.client.main.Main" 2>/dev/null
  pkill -f fabric-server 2>/dev/null
  pkill Xvfb 2>/dev/null
  pkill -f "tail -f /tmp/fabcin" 2>/dev/null
  return 0
}
trap cleanup EXIT
cleanup; sleep 2

count_rows() {
  python3 - "$1" <<'EOF'
import sqlite3, sys, glob
total = 0
for db in glob.glob(sys.argv[1] + '/**/*.sqlite', recursive=True):
    try:
        c = sqlite3.connect(f'file:{db}?mode=ro', uri=True)
        for t in ('FullData', 'ChunkHash'):
            try:
                total += c.execute(f"SELECT count(*) FROM '{t}'").fetchone()[0]
            except Exception:
                pass
        c.close()
    except Exception:
        pass
print(total)
EOF
}

CLIENT_DB="$HOME/mc-test/client/Distant_Horizons_server_data"
SERVER_DB="$D/world/dimensions/minecraft/overworld/data"
echo "client rows before: $(count_rows "$CLIENT_DB")"
echo "server rows before: $(count_rows "$SERVER_DB")"

rm -f /tmp/dhserver.log /tmp/fabcin
mkfifo /tmp/fabcin
cd "$D"
tail -f /tmp/fabcin 2>/dev/null | "$JAVA" -Xmx1300M -jar fabric-server.jar nogui > /tmp/dhserver.log 2>&1 &
for i in $(seq 1 120); do grep -qE 'Done \(' /tmp/dhserver.log 2>/dev/null && break; sleep 2; done
grep -qE 'Done \(' /tmp/dhserver.log || { echo "FABRIC BOOT TIMEOUT"; exit 1; }
echo "fabric DH server up"

CLOD_DH_PORT=25566 bash "$S/run_dh_client.sh" >/dev/null 2>&1
for i in $(seq 1 240); do grep -q "Stepan joined the game" /tmp/dhserver.log 2>/dev/null && break; sleep 1; done
echo "client joined — park at the camera"
echo "gamemode spectator Stepan" > /tmp/fabcin
sleep 1
echo "tp Stepan -929 173 796 223 15" > /tmp/fabcin
sleep 1
echo "gamerule doDaylightCycle false" > /tmp/fabcin
echo "time set 6000" > /tmp/fabcin

echo "soaking 480s (server gen + client sync at the camera)"
sleep 480

echo "client rows after:  $(count_rows "$CLIENT_DB")"
echo "server rows after:  $(count_rows "$SERVER_DB")"
grep -iE "generat|queue" /tmp/dh_client.log | grep -viE "fabric|resource" | tail -4
