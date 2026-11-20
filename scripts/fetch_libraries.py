#!/usr/bin/env python3
"""Fetch the 26.3 client libraries (114) into ~/mc-test/dist/libraries."""
import json
import os
import sys
import urllib.request
from concurrent.futures import ThreadPoolExecutor

MANIFEST = "/home/z/my-project/analysis/mc263/26.3.json"
DEST = "/home/z/mc-test/dist/libraries"
WORKERS = 16


def fetch(item):
    art = item.get("downloads", {}).get("artifact")
    if not art:
        return None
    url, path = art["url"], art["path"]
    dest = os.path.join(DEST, path)
    if os.path.exists(dest) and os.path.getsize(dest) == art.get("size", -1):
        return None
    os.makedirs(os.path.dirname(dest), exist_ok=True)
    for attempt in (1, 2):
        try:
            urllib.request.urlretrieve(url, dest)
            return path
        except Exception as exc:
            if attempt == 2:
                return f"FAIL {path}: {exc}"
    return None


def main():
    libs = json.load(open(MANIFEST))["libraries"]
    print(f"{len(libs)} libraries")
    done = fails = 0
    with ThreadPoolExecutor(WORKERS) as pool:
        for res in pool.map(fetch, libs):
            if res is None:
                done += 1
            elif str(res).startswith("FAIL"):
                fails += 1
                print(res)
            else:
                done += 1
    print(f"fetched/kept {done}, failed {fails}")
    sys.exit(1 if fails else 0)


if __name__ == "__main__":
    main()
