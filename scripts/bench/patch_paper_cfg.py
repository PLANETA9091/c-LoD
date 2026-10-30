#!/usr/bin/env python3
"""Patch Paper's chunk-system worker pool in <dist>/config/paper-global.yml.

v5.5 frontier sprint: on small-CPU hosts Paper's auto chunk-system sizing
(availableProcessors - 2, min 1) leaves generation I/O-bound and the c-LoD
scan frontier starves (~16 cells/s observed on a 2-CPU sandbox). Explicit
worker/io threads keep the FORCED-ticket pipeline saturated.

Env knobs (both optional; the script is a no-op without either):
  PAPER_WORKERS  chunk-system worker-threads (gen pipeline)
  PAPER_IO       chunk-system io-threads (region-file I/O)

Usage: PAPER_WORKERS=4 PAPER_IO=2 patch_paper_cfg.py <dist-root>
Idempotent: rewrites only the two keys, keeps the rest of the YAML intact
(line-level edit, no dependency needed).
"""
import os
import re
import sys


def patch(path: str, workers: int, io_t: int) -> int:
    with open(path, encoding="utf-8") as fh:
        lines = fh.readlines()
    out, in_chunk, seen_w, seen_i = [], False, False, False
    for line in lines:
        if re.match(r"^chunk-system:\s*(#.*)?$", line):
            in_chunk = True
        elif re.match(r"^\S", line):  # any other top-level key
            in_chunk = False
        if in_chunk and re.match(r"^\s+worker-threads:\s*\S+", line):
            indent = line[: len(line) - len(line.lstrip())]
            out.append(f"{indent}worker-threads: {workers}\n")
            seen_w = True
        elif in_chunk and re.match(r"^\s+io-threads:\s*\S+", line):
            indent = line[: len(line) - len(line.lstrip())]
            out.append(f"{indent}io-threads: {io_t}\n")
            seen_i = True
        else:
            out.append(line)
    # Keys missing from the default file (still at -1 shorthand section?):
    # append explicit ones under chunk-system.
    if not (seen_w and seen_i):
        idx = next((i for i, l in enumerate(out) if l.startswith("chunk-system:")), None)
        if idx is None:
            out.append("chunk-system:\n")
            idx = len(out) - 1
        add = []
        if not seen_w:
            add.append(f"  worker-threads: {workers}\n")
        if not seen_i:
            add.append(f"  io-threads: {io_t}\n")
        # insert right after the chunk-system: header
        end = idx + 1
        while end < len(out) and (out[end].startswith(" ") or out[end].strip() == ""):
            end += 1
        out[end:end] = add
    with open(path, "w", encoding="utf-8") as fh:
        fh.writelines(out)
    return 0


def main() -> int:
    root = sys.argv[1] if len(sys.argv) > 1 else os.path.expanduser("~/mc-test/dist")
    workers = os.environ.get("PAPER_WORKERS", "").strip()
    io_t = os.environ.get("PAPER_IO", "").strip()
    if not workers and not io_t:
        print("patch_paper_cfg: PAPER_WORKERS/PAPER_IO unset - no-op")
        return 0
    path = os.path.join(root, "config", "paper-global.yml")
    if not os.path.exists(path):
        print(f"patch_paper_cfg: {path} missing (first boot not done yet?) - no-op")
        return 0
    with open(path, encoding="utf-8") as fh:
        before = [
            l.strip() for l in fh if "worker-threads" in l or "io-threads" in l
        ]
    rc = patch(path, int(workers or 4), int(io_t or 2))
    with open(path, encoding="utf-8") as fh:
        after = [l.strip() for l in fh if "worker-threads" in l or "io-threads" in l]
    print(f"patch_paper_cfg: {before} -> {after}")
    return rc


if __name__ == "__main__":
    sys.exit(main())
