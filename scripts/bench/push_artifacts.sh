#!/usr/bin/env bash
# Push bench frames / stream proof / test artifacts to GitHub (PLANETA9091/c-LoD).
# User directive: "кадры + стримы и все тесты на гитхаб" — every shot batch,
# stream log and test output lands in docs/ and is pushed immediately.
# Usage: push_artifacts.sh "<commit message>"
set -euo pipefail
REPO=/home/z/my-project/c-LoD
SHOTS="$HOME/mc-test/shots"
DEST="$REPO/docs/e2e-v5.0.4"
STAMP=$(date -u +"%Y-%m-%d %H:%M")

mkdir -p "$DEST"
# copy every new frame from the bench shots dir (skip already-copied by checksum)
if [ -d "$SHOTS" ]; then
  for f in "$SHOTS"/*.png; do
    [ -e "$f" ] || continue
    b=$(basename "$f")
    if [ ! -f "$DEST/$b" ] || ! cmp -s "$f" "$DEST/$b"; then
      cp "$f" "$DEST/$b"
    fi
  done
fi
# stream logs / counters snapshot
if [ -f /tmp/server.log ]; then
  rg -N "stream|cells|lap|failed|armed|GATE|pack finalized" /tmp/server.log \
    > "$DEST/stream_log_tail.txt" || true
  tail -40 /tmp/server.log > "$DEST/server_log_tail.txt" || true
fi
[ -f /tmp/client.log ] && rg -N -i "pack|post|error" /tmp/client.log > "$DEST/client_log_tail.txt" || true
curl -s --max-time 5 http://127.0.0.1:8199/clod-pack.info > "$DEST/clod-pack.info" 2>/dev/null || true

cd "$REPO"
if git status --porcelain | grep -q .; then
  git add -A
  git -c user.name="Z User" -c user.email="z@container" commit -m "artifacts: $1 ($STAMP)" -q
  git push -q origin master
  echo "PUSHED: $1 ($STAMP)"
  git log --oneline -1
else
  echo "nothing new to push ($STAMP)"
fi
