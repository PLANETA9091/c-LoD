#!/usr/bin/env bash
# v6.16 PARITY FEED — the parity-gate job's outputs (parity.md, parity.json,
# the voxy|ours|diff triptychs and the diff heatmaps) land in the
# `ci-verdicts` branch under gallery/parity-run-<RUN_NO>/ — the same
# no-API-quota raw-CDN lesson the verdict feed (v6.7.2) and the voxy feed
# (v6.8.1) each taught once. The user's parity gate stays RED in CI while
# the render differs; the feed is how the difference is SEEN (and driven to
# zero) without ever burning the artifact API.
#
# Best-effort by design: any failure prints a WARN and exits 0 — the feed
# can never flip the gate's real verdict.
#
# Usage (from CI): publish_parity_feed.sh <parity-out-dir> <run-id> <run-number>
#
# Branch hygiene: the parity feed NEVER touches latest.md / run-*.md /
# run-*.jpg (the verdict feed owns them) and never touches the hand-built
# gallery sheets — it appends gallery/parity-run-*/ and rewrites only
# gallery/parity-index.md.
set -u

SRC="${1:-parity-out}"
RUN_ID="${2:-unknown}"
RUN_NO="${3:-}"
SHA7=unknown
if [ -n "${GITHUB_SHA:-}" ]; then SHA7="${GITHUB_SHA:0:7}"; fi

[ -d "$SRC" ] || { echo "WARN: parity dir $SRC missing — nothing to publish"; exit 0; }
ls "$SRC"/* >/dev/null 2>&1 || { echo "WARN: parity dir empty — nothing to publish"; exit 0; }

git config user.name "ci-verdict-bot"
git config user.email "actions@users.noreply.github.com"
git fetch origin ci-verdicts 2>/dev/null || true
rm -rf /tmp/pfeed
git worktree prune
git worktree add --detach /tmp/pfeed HEAD || exit 0
(
  cd /tmp/pfeed
  git checkout --orphan ci-verdicts-tmp 2>/dev/null && git rm -rf --quiet . 2>/dev/null || true
)
if git show-ref --verify --quiet refs/remotes/origin/ci-verdicts; then
  ( cd /tmp/pfeed \
    && git checkout -B ci-verdicts origin/ci-verdicts ) || { echo "WARN: ci-verdicts checkout failed"; exit 0; }
fi
# v6.16.6 THE BRANCH WIPE, caught by my own repair: the previous revision ran
# `git rm -rf --quiet .` AFTER the checkout -B — every parity feed commit
# deleted the WHOLE ci-verdicts content (verdicts, instruments, sheets A-G,
# latest.md, roadmap.md) before adding its parity files, and the branch
# collapsed to gallery/parity-run-* alone (runs #276/#278/#281's feed
# commits did it three times; the CI verdict feed's own publisher puts the
# rm BEFORE the checkout — the orphan dance only — which is the correct
# order). The checkout -B already resets the index+worktree to the remote
# branch; NOTHING else is removed — the feed APPENDS.

RD="/tmp/pfeed/gallery/parity-run-${RUN_NO}"
mkdir -p "$RD"

# v6.25.5 TIME AUDIT: the parity session's own console (server.log saved by
# the visual leg next to its parity/ shots) rides the feed — #332 published
# a NIGHT matrix (ours sky (42,46,58) vs voxy day) while every pin "should
# have" worked; the feed now carries the console testimony so the next
# time-of-day incident is self-diagnosing without any API access.
EVDIR="${4:-}"
if [ -n "$EVDIR" ] && [ -f "$EVDIR/server.log" ]; then
  {
    echo "# parity session time audit (the §7b console channel)"
    echo
    echo "- v6.25.5: boot markers, pins, gamerules and joins from the parity session server.log (tail 40). The #332 night-matrix class is self-diagnosing from here on."
    echo
    echo '```'
    grep -nE "Done \(|pack finalized|doDaylightCycle|[Ss]et the time|time set|gamerule|Stepan joined" "$EVDIR/server.log" | tail -40
    echo '```'
  } > "$RD/time-audit.md" 2>/dev/null || true
fi

# Downscale the triptychs (1952x368 -> 1280 wide JPEG q85) and the heatmaps
# (1280x720 grayscale -> JPEG q80); text reports ride as-is.
SRC="$SRC" RD="$RD" python3 - <<'PYEOF' 2>/dev/null || cp "$SRC"/*.md "$SRC"/*.json "$RD"/ 2>/dev/null || true
import glob, os
from PIL import Image
src = os.environ["SRC"]; rd = os.environ["RD"]
for f in sorted(glob.glob(os.path.join(src, "*.png"))):
    im = Image.open(f).convert("RGB")
    if im.width > 1280:
        im = im.resize((1280, max(1, round(im.height * 1280 / im.width))), Image.LANCZOS)
    name = os.path.splitext(os.path.basename(f))[0] + ".jpg"
    im.save(os.path.join(rd, name), "JPEG", quality=85)
for f in sorted(glob.glob(os.path.join(src, "*.md")) + glob.glob(os.path.join(src, "*.json"))):
    import shutil; shutil.copy(f, rd)
PYEOF

cp "$SRC/parity.md" "$RD/" 2>/dev/null || true
cp "$SRC/parity.json" "$RD/" 2>/dev/null || true

# prune: keep the newest 5 parity runs
ls -d /tmp/pfeed/gallery/parity-run-* 2>/dev/null | sort | head -n -5 | while read -r old; do
  rm -rf "$old"
done

# per-run page: the gate verdict + the triptych strip inline
PASS=$(python3 -c "import json;print(json.load(open('$RD/parity.json'))['pass'])" 2>/dev/null || echo "?")
{
  echo "# PARITY RUN ${RUN_NO} (run ${RUN_ID}, sha ${SHA7}) — gate: ${PASS}"
  echo
  [ -f "$RD/parity.md" ] && cat "$RD/parity.md"
  echo
  echo "## voxy | ours | diff (triptychs)"
  echo
  for f in $(ls "$RD"/tri_*.jpg 2>/dev/null | sort); do
    b=$(basename "$f")
    echo "- ![${b}](parity-run-${RUN_NO}/${b})"
  done
  [ -f "$RD/time-audit.md" ] && echo "- time audit: [time-audit.md](parity-run-${RUN_NO}/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)"
} > "$RD/latest.md" 2>/dev/null || true

# the index: every parity run, newest first, with its gate state
{
  echo "# parity feed index (c-LoD vs Voxy, pixel-perfect gate)"
  echo
  for d in $(ls -d /tmp/pfeed/gallery/parity-run-* 2>/dev/null | sort -r); do
    n=$(basename "$d")
    p=$(python3 -c "import json;print(json.load(open('$d/parity.json'))['pass'])" 2>/dev/null || echo "?")
    echo "- ${n}: gate ${p}"
  done
} > /tmp/pfeed/gallery/parity-index.md

cd /tmp/pfeed
git add -A
git commit -m "parity feed: run ${RUN_ID} (#${RUN_NO}, sha ${SHA7}) — gate ${PASS}" || echo "no changes"
git push origin ci-verdicts \
  && echo "parity feed updated" \
  || echo "WARN: parity feed push failed (non-fatal)"
exit 0
