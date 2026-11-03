#!/usr/bin/env bash
# v6.8.4 CONTROL FEED — publishes the pack-free differential verdict to the
# ci-verdicts branch (run-<id>-control.md + two JPEGs). The control job runs
# IN PARALLEL with the main visual job and both publish to the same branch,
# so the fetch-checkout-publish cycle RETRIES once on a rejected push (a
# non-fast-forward means the other job won the race; the retry re-reads the
# branch and rebuilds on top of it). Best-effort: any failure is a WARN,
# never a job verdict flip.
#
# Usage (from CI): publish_control_feed.sh <shots-dir> <run-id> <run-number>
set -u

SRC="${1:-/home/runner/mc-test/shots/ci-visual}"
RUN_ID="${2:-unknown}"
RUN_NO="${3:-}"
BENCH_DIR="$(cd "$(dirname "$0")" && pwd)"

SHA7=unknown
if [ -n "${GITHUB_SHA:-}" ]; then SHA7="${GITHUB_SHA:0:7}"; fi

# v6.8.5: a death BEFORE the control step (run 41468940410: the job ended
# with no summary at all) used to leave no verdict — same hole the voxy
# feed closed in v6.8.3. Publish a marker with the control steps trace.
if [ ! -f "$SRC/summary.md" ]; then
  echo "no control summary — publishing a failure marker"
  publish_marker() {
    git config user.name "ci-verdict-bot"
    git config user.email "actions@users.noreply.github.com"
    git fetch origin ci-verdicts 2>/dev/null || true
    rm -rf /tmp/cfeed
    git worktree prune
    git worktree add --detach /tmp/cfeed HEAD || return 1
    ( cd /tmp/cfeed && git checkout --orphan ci-verdicts && git rm -rf --quiet . 2>/dev/null || true )
    if git show-ref --verify --quiet refs/remotes/origin/ci-verdicts; then
      ( cd /tmp/cfeed && git rm -rf --quiet . 2>/dev/null || true && git checkout -B ci-verdicts origin/ci-verdicts )
      for f in latest.md index.md; do
        git show "origin/ci-verdicts:$f" > "/tmp/cfeed/$f" 2>/dev/null || true
      done
    fi
    mkdir -p "/tmp/cfeed/run-${RUN_ID}"
    cp "$HOME/mc-test/control-steps.log" "/tmp/cfeed/run-${RUN_ID}/control-steps.log" 2>/dev/null || true
    # v6.8.6: the pulse leg's own stdout/stderr (run_ci_visual.sh tees it
    # when CLOD_CONTROL=1) — names a death the per-step trace does not.
    cp "$HOME/mc-test/control-leg.log" "/tmp/cfeed/run-${RUN_ID}/control-leg.log" 2>/dev/null || true
    # v6.8.7: the SERVER's own words name login refusals — the #227 root
    # cause (white-list=true first-boot default) was in this log.
    cp /tmp/server.log "/tmp/cfeed/run-${RUN_ID}/server.log" 2>/dev/null || true
    cp /tmp/client.log "/tmp/cfeed/run-${RUN_ID}/client.log" 2>/dev/null || true
    {
      echo "# control run ${RUN_ID} (#${RUN_NO}) — FAILURE (no summary) $(date -u +%FT%TZ)"
      echo
      echo "The control job died BEFORE producing a summary (sha ${SHA7}) —"
      echo "see run-${RUN_ID}/control-steps.log (tail below) for the last step."
      echo
      echo "## control-steps.log tail"
      echo '```'
      tail -30 "$HOME/mc-test/control-steps.log" 2>/dev/null || echo "(no steps log)"
      echo '```'
      echo
      echo "## control-leg.log tail (the pulse leg's own stdout/stderr)"
      echo '```'
      tail -30 "$HOME/mc-test/control-leg.log" 2>/dev/null || echo "(no leg log — the death predates the v6.8.6 tee)"
      echo '```'
      echo
      echo "## server.log tail (join refusals name themselves here)"
      echo '```'
      grep -aE "Disconnecting|lost connection|whitelist|white-list|Failed to verify|Invalid session" /tmp/server.log 2>/dev/null | tail -6
      tail -10 /tmp/server.log 2>/dev/null || echo "(no server log)"
      echo '```'
    } > "/tmp/cfeed/run-${RUN_ID}-control.md"
    cd /tmp/cfeed
    git add -A
    git commit -q -m "control FAILED before summary: run ${RUN_ID} (${SHA7})" || echo "no changes"
    git push origin ci-verdicts
  }
  for attempt in 1 2; do
    if publish_marker; then echo "control failure marker published"; break; fi
    echo "WARN: marker push attempt $attempt failed" >&2
    sleep 10
  done
  exit 0
fi

publish() { # -> 0 when pushed
  git config user.name "ci-verdict-bot"
  git config user.email "actions@users.noreply.github.com"
  git fetch origin ci-verdicts 2>/dev/null || true
  rm -rf /tmp/cfeed
  git worktree prune
  git worktree add --detach /tmp/cfeed HEAD || return 1
  ( cd /tmp/cfeed && git checkout --orphan ci-verdicts && git rm -rf --quiet . 2>/dev/null || true )
  if git show-ref --verify --quiet refs/remotes/origin/ci-verdicts; then
    ( cd /tmp/cfeed && git rm -rf --quiet . 2>/dev/null || true && git checkout -B ci-verdicts origin/ci-verdicts )
  fi
  cp "$SRC/summary.md" "/tmp/cfeed/run-${RUN_ID}-control.md"
  # the main visual job owns latest.md — keep whatever it published
  if git show-ref --verify --quiet refs/remotes/origin/ci-verdicts; then
    for f in latest.md index.md; do
      git show "origin/ci-verdicts:$f" > "/tmp/cfeed/$f" 2>/dev/null || true
    done
  fi
  for pair in "09_burst_close:burst" "10_before_edit:baseline"; do
    IMG="${pair%%:*}"; TAG="${pair##*:}"
    [ -f "$SRC/$IMG.png" ] || continue
    python3 "$BENCH_DIR/jpg_to_feed.py" "$SRC/$IMG.png" "/tmp/cfeed/run-${RUN_ID}-control-${TAG}.jpg" 2>/dev/null || true
  done
  # keep the newest 3 control verdicts (files are prefix-tagged -control-)
  ls /tmp/cfeed/run-*-control.md 2>/dev/null | sort | head -n -3 | while read -r old; do
    R=$(basename "$old" -control.md)
    rm -f "/tmp/cfeed/$R-control.md" "/tmp/cfeed/$R-control-burst.jpg" "/tmp/cfeed/$R-control-baseline.jpg"
  done
  {
    echo "- run ${RUN_ID} (#${RUN_NO}) CONTROL ${SHA7}: $(grep -oE 'CONTROL (PASS|FAIL)' "$SRC/summary.md" | head -1), drift=$(grep -oE 'drift=[0-9]+' "$SRC/summary.md" | head -1)"
  } >> /tmp/cfeed/index.md
  cd /tmp/cfeed
  git add -A
  git commit -q -m "control verdict: run ${RUN_ID} (${SHA7})" || echo "no changes"
  git push origin ci-verdicts
}

ok=0
for attempt in 1 2; do
  if publish; then ok=1; echo "control feed updated"; break; fi
  echo "WARN: control feed push attempt $attempt failed — retrying (the parallel visual publisher may have won the race)" >&2
  sleep 10
done
[ "$ok" = "1" ] || echo "WARN: control feed push failed twice (non-fatal)"
exit 0
