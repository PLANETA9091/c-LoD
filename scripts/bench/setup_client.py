#!/usr/bin/env python3
"""Rebuild the local vanilla 26.3 client runtime for the c-LoD test bench.

Downloads (per the client version manifest 26.3.json):
  - all linux-compatible library jars          -> client/libraries/<path>
  - natives extracted from *-natives-linux.jar -> client/natives/
  - asset index 34 + all asset objects         -> client/assets/
  - logging config                             -> client/logconfig.xml

The client.jar is reused from analysis/mc263/client.jar when present (the
local bench); otherwise (CI) it is downloaded from the official Mojang URL in
the manifest and verified against the manifest SHA-1.

The manifest itself resolves in this order:
  1. $CLOD_MANIFEST
  2. scripts/bench/26.3.json (the committed copy — the CI path, no analysis
     tree exists on a runner)
  3. /home/z/my-project/analysis/mc263/26.3.json (the local bench copy)
"""
import concurrent.futures as futures
import hashlib
import json
import os
import shutil
import sys
import urllib.request
import zipfile

_BENCH = os.path.dirname(os.path.abspath(__file__))
MANIFEST = os.environ.get(
    "CLOD_MANIFEST",
    next(p for p in (os.path.join(_BENCH, "26.3.json"),
                     "/home/z/my-project/analysis/mc263/26.3.json")
         if os.path.exists(p)),
)
CLIENT_JAR = os.environ.get("CLOD_CLIENT_JAR",
                            "/home/z/my-project/analysis/mc263/client.jar")
ROOT = os.path.expanduser("~/mc-test/client")
UA = {"User-Agent": "c-LoD-bench/4.1 (test harness)"}


def fetch_client_jar(m, dest):
    """CI path: pull the real client.jar from the manifest URL, sha1-verified."""
    url = m["downloads"]["client"]["url"]
    sha1 = m["downloads"]["client"]["sha1"]
    print(f"client.jar not local — downloading from Mojang: {url}", flush=True)
    fetch(url, dest)
    h = hashlib.sha1(open(dest, "rb").read()).hexdigest()
    if h != sha1:
        raise SystemExit(f"client.jar sha1 mismatch: got {h}, want {sha1}")
    print(f"client.jar verified ({sha1[:12]}...)", flush=True)


def rules_allow(entry):
    for rule in entry.get("rules", []):
        os_ok = rule.get("os", {}).get("name") in (None, "linux")
        if rule.get("action") == "allow":
            if not os_ok:
                return False
        elif rule.get("action") == "disallow" and os_ok:
            return False
    return True


def fetch(url, dest, tries=3):
    os.makedirs(os.path.dirname(dest), exist_ok=True)
    for i in range(tries):
        try:
            import urllib.request
            req = urllib.request.Request(url, headers=UA)
            with urllib.request.urlopen(req, timeout=120) as r, open(dest + ".part", "wb") as f:
                shutil.copyfileobj(r, f)
            os.replace(dest + ".part", dest)
            return True
        except Exception as e:
            if i == tries - 1:
                print(f"FAIL {url}: {e}", flush=True)
                return False
    return False


def main():
    m = json.load(open(MANIFEST))
    libs_dir = os.path.join(ROOT, "libraries")
    natives_dir = os.path.join(ROOT, "natives")
    assets_dir = os.path.join(ROOT, "assets")
    for d in (libs_dir, natives_dir, os.path.join(assets_dir, "indexes"),
              os.path.join(assets_dir, "objects"), os.path.join(ROOT, "versions", "26.3")):
        os.makedirs(d, exist_ok=True)

    client_dst = os.path.join(ROOT, "versions", "26.3", "26.3.jar")
    if os.path.exists(CLIENT_JAR):
        shutil.copyfile(CLIENT_JAR, client_dst)
    else:
        fetch_client_jar(m, client_dst)

    # ---- libraries + natives ----
    artifacts, native_jars = [], []
    for lib in m["libraries"]:
        if not rules_allow(lib):
            continue
        art = lib.get("downloads", {}).get("artifact")
        if art and art.get("url"):
            artifacts.append((art["url"], os.path.join(libs_dir, art["path"])))
            if "-natives-linux" in art["path"] or art["path"].endswith("natives-linux.jar"):
                native_jars.append(os.path.join(libs_dir, art["path"]))

    ok = 0
    with futures.ThreadPoolExecutor(max_workers=12) as ex:
        for good in ex.map(lambda a: fetch(*a), artifacts):
            ok += good
    print(f"libraries: {ok}/{len(artifacts)}", flush=True)

    for jar in native_jars:
        with zipfile.ZipFile(jar) as z:
            z.extractall(natives_dir)
    print(f"natives extracted: {len(native_jars)} jars -> {natives_dir}", flush=True)

    # classpath file for the launch script
    with open(os.path.join(ROOT, "cp.txt"), "w") as f:
        f.write(":".join([os.path.join(ROOT, "versions", "26.3", "26.3.jar")]
                         + [a[1] for a in artifacts]))

    # ---- assets ----
    ai = m["assetIndex"]
    idx_path = os.path.join(assets_dir, "indexes", ai["id"] + ".json")
    fetch(ai["url"], idx_path)
    idx = json.load(open(idx_path))
    objs = []
    for name, obj in idx.get("objects", {}).items():
        h = obj["hash"]
        dest = os.path.join(assets_dir, "objects", h[:2], h)
        if not os.path.exists(dest) or os.path.getsize(dest) != obj["size"]:
            objs.append((f"https://resources.download.minecraft.net/{h[:2]}/{h}", dest, obj["size"]))
    print(f"assets to fetch: {len(objs)}", flush=True)
    ok = 0
    with futures.ThreadPoolExecutor(max_workers=16) as ex:
        for good in ex.map(lambda a: fetch(a[0], a[1]), objs):
            ok += good
    print(f"assets: {ok}/{len(objs)}", flush=True)

    os.makedirs(os.path.join(ROOT, "resources"), exist_ok=True)

    log_cfg = m.get("logging", {}).get("client", {})
    if log_cfg:
        f = log_cfg.get("file", {})
        if f.get("url"):
            fetch(f["url"], os.path.join(ROOT, "logconfig.xml"))
            print("log config fetched", flush=True)

    print("CLIENT RUNTIME READY", flush=True)


if __name__ == "__main__":
    sys.exit(main())
