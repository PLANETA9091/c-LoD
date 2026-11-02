#!/usr/bin/env bash
# v6.8.1 VOXY REFERENCE FEED — the voxy-reference shots land in the
# `voxy-ref` branch (raw CDN, NO API quota), the same lesson the visual
# verdicts learned in v6.7.2: artifacts were repeatedly rate-limited for
# the sandbox egress IPs, and evidence that cannot be read is not evidence.
#
# Best-effort by design: any failure prints a WARN and exits 0 — the feed
# can never flip the job's real verdict.
#
# Usage (from CI): publish_voxy_feed.sh <shots-dir> <run-id> <run-number>
#   shots-dir defaults to /home/runner/mc-test/voxy-shots
#
# v6.8.3 ZERO-SHOT IS ALWAYS A MARKER: run 41454056043 killed the voxy job
# 76 s in — BEFORE run_voxy_shots.sh's mkdir created the shots dir — and
# the old early-exit (`no shots dir — nothing to publish`) left no branch,
# indistinguishable from "hasn't run yet". The v6.8.2 failure-marker only
# covered the dir-exists-but-empty case. Now EVERY zero-shot outcome
# (missing dir, empty dir, failed conversion) publishes the marker, and
# the marker carries the per-step trace ($HOME/mc-test/voxy-steps.log,
# written by the job's steps themselves) so a pre-boot death names its
# step without any API access.
set -u

SRC="${1:-/home/runner/mc-test/voxy-shots}"
RUN_ID="${2:-unknown}"
RUN_NO="${3:-}"
STEPS_LOG="$HOME/mc-test/voxy-steps.log"
# v6.8.6: the leg's own stdout/stderr (run_voxy_shots.sh tees everything)
# — the marker's only window into a death the per-step trace does not name.
LEG_LOG="$HOME/mc-test/voxy-leg.log"

SHA7=unknown
if [ -n "${GITHUB_SHA:-}" ]; then SHA7="${GITHUB_SHA:0:7}"; fi

# Orphan-checkout the feed branch into a scratch worktree (identical to
# the ci-verdicts publisher). Globals the marker writes: run dir + logs.
feed_worktree() {
  git config user.name "voxy-ref-bot"
  git config user.email "actions@users.noreply.github.com"
  git fetch origin voxy-ref 2>/dev/null || true
  rm -rf /tmp/xfeed
  git worktree prune
  git worktree add --detach /tmp/xfeed HEAD || return 1
  (
    cd /tmp/xfeed
    git checkout --orphan voxy-ref
    git rm -rf --quiet . 2>/dev/null || true
  )
  if git show-ref --verify --quiet refs/remotes/origin/voxy-ref; then
    (
      cd /tmp/xfeed
      git rm -rf --quiet . 2>/dev/null || true
      git checkout -B voxy-ref origin/voxy-ref
    )
  fi
  return 0
}

# Prune first, keep the newest 5 run-* dirs; the current run is never
# pruned even if its id sorts oldest. latest.md/index.md are generated
# AFTER this (the v6.8.1 self-test found that ordering bug).
prune_runs() {
  ls -d /tmp/xfeed/run-* 2>/dev/null | sort | head -n -5 | while read -r old; do
    [ "$old" = "/tmp/xfeed/run-${RUN_ID}" ] && continue
    rm -rf "$old"
  done
}

push_feed() { # $1 = commit message
  cd /tmp/xfeed
  git add -A
  git commit -m "$1" || echo "no changes"
  git push origin voxy-ref \
    && echo "voxy feed updated" \
    || echo "WARN: voxy feed push failed (non-fatal)"
}

publish_failure_marker() {
  echo "no shots — publishing a failure marker"
  feed_worktree || exit 0
  mkdir -p "/tmp/xfeed/run-${RUN_ID}"
  cp /tmp/voxy_client.log "/tmp/xfeed/run-${RUN_ID}/client.log" 2>/dev/null || true
  cp /tmp/voxy_server.log "/tmp/xfeed/run-${RUN_ID}/server.log" 2>/dev/null || true
  [ -f "$STEPS_LOG" ] && cp "$STEPS_LOG" "/tmp/xfeed/run-${RUN_ID}/steps.log" 2>/dev/null || true
  [ -f "$LEG_LOG" ] && cp "$LEG_LOG" "/tmp/xfeed/run-${RUN_ID}/leg.log" 2>/dev/null || true
  {
    echo "# voxy reference run ${RUN_ID} (#${RUN_NO}) — FAILURE (no shots) $(date -u +%FT%TZ)"
    echo
    echo "The job produced zero screenshots (shots dir exists: $([ -d "$SRC" ] && echo yes || echo NO), sha ${SHA7})."
    echo "See run-${RUN_ID}/ for client.log, server.log, steps.log and leg.log (tails below)."
    echo
    echo "## steps.log tail (per-step trace written by the job steps)"
    echo '```'
    tail -40 "$STEPS_LOG" 2>/dev/null || echo "(no steps log — death before the first traced step)"
    echo '```'
    echo
    echo "## client.log tail"
    echo '```'
    tail -60 /tmp/voxy_client.log 2>/dev/null || echo "(no client log)"
    echo '```'
    echo
    echo "## server.log tail"
    echo '```'
    tail -40 /tmp/voxy_server.log 2>/dev/null || echo "(no server log)"
    echo '```'
    echo
    echo "## voxy-leg.log tail (the leg's own stdout/stderr)"
    echo '```'
    tail -30 "$LEG_LOG" 2>/dev/null || echo "(no leg log — the death predates the v6.8.6 tee)"
    echo '```'
  } > /tmp/xfeed/latest.md
  prune_runs
  {
    echo "# voxy reference feed"
    echo
    for d in $(ls -d /tmp/xfeed/run-* 2>/dev/null | sort); do
      if [ -f "$d/latest.md" ] || ls "$d"/*.jpg >/dev/null 2>&1; then
        n=$(ls "$d"/*.jpg 2>/dev/null | wc -l)
        echo "- $(basename "$d"): ${n} shots"
      fi
    done
  } > /tmp/xfeed/index.md
  push_feed "voxy reference FAILED (no shots): run ${RUN_ID} (${SHA7})"
  exit 0
}

SHOTS=$(ls "$SRC"/*.png "$SRC"/*.jpg "$SRC"/*.jpeg 2>/dev/null || true)
if [ -z "$SHOTS" ]; then
  publish_failure_marker
fi

# Downscale to max 1280 px wide JPEG (q85) — the branch must stay light
# (pillow is installed by the job's setup step; if it is missing the raw
# files are copied as-is).
OUT=/tmp/voxy_feed_shots
rm -rf "$OUT"
mkdir -p "$OUT"
if SRC="$SRC" python3 - <<'PYEOF' 2>/dev/null
import glob, os
from PIL import Image
out = "/tmp/voxy_feed_shots"
src = os.environ.get("SRC", "")
for f in sorted(glob.glob(os.path.join(src, "*.png"))
                + glob.glob(os.path.join(src, "*.jpg"))
                + glob.glob(os.path.join(src, "*.jpeg"))):
    im = Image.open(f).convert("RGB")
    if im.width > 1280:
        im = im.resize((1280, max(1, round(im.height * 1280 / im.width))), Image.LANCZOS)
    name = os.path.splitext(os.path.basename(f))[0] + ".jpg"
    im.save(os.path.join(out, name), "JPEG", quality=85)
PYEOF
then
  echo "shots downscaled with pillow"
else
  echo "WARN: pillow missing/failed — copying raw shots" >&2
  cp "$SRC"/*.png "$SRC"/*.jpg "$SRC"/*.jpeg "$OUT"/ 2>/dev/null || true
fi

N_SHOTS=$(ls "$OUT" 2>/dev/null | wc -l)
if [ "$N_SHOTS" -eq 0 ]; then
  echo "WARN: shot conversion produced nothing"
  publish_failure_marker
fi

feed_worktree || exit 0
mkdir -p "/tmp/xfeed/run-${RUN_ID}"
cp "$OUT"/*.jpg /tmp/xfeed/run-${RUN_ID}/ 2>/dev/null || true
cp "$OUT"/* /tmp/xfeed/run-${RUN_ID}/ 2>/dev/null || true
cp /tmp/voxy_client.log "/tmp/xfeed/run-${RUN_ID}/client.log" 2>/dev/null || true
cp /tmp/voxy_server.log "/tmp/xfeed/run-${RUN_ID}/server.log" 2>/dev/null || true
cp "$STEPS_LOG" "/tmp/xfeed/run-${RUN_ID}/steps.log" 2>/dev/null || true

prune_runs

{
  echo "# voxy reference shots — run ${RUN_ID} (#${RUN_NO}) $(date -u +%FT%TZ)"
  echo
  echo "${N_SHOTS} shots in run-${RUN_ID}/ (1280 px JPEG)."
  echo
  for f in /tmp/xfeed/run-${RUN_ID}/*.jpg; do
    [ -f "$f" ] || continue
    echo "- ![$(basename "$f")](run-${RUN_ID}/$(basename "$f"))"
  done
} > /tmp/xfeed/latest.md

{
  echo "# voxy reference feed"
  echo
  for d in $(ls -d /tmp/xfeed/run-* 2>/dev/null | sort); do
    n=$(ls "$d"/*.jpg 2>/dev/null | wc -l)
    echo "- $(basename "$d"): ${n} shots"
  done
} > /tmp/xfeed/index.md

push_feed "voxy reference: run ${RUN_ID} (${SHA7})"
exit 0
