#!/usr/bin/env bash
# Restore the bench environment after a sandbox reset.
# Only /home/z/my-project survives resets; toolchains + ~/mc-test are rebuilt.
#
# Toolchains (synchronous, with timeout - background downloads get killed):
#   - rustup stable -> ~/.cargo (rustfmt + clippy)
#   - Temurin JDK 25 -> /home/z/tools/jdk-25.0.5+8
#   - glslang 16.6.0 -> /home/z/tools/glslang
# Bench root:
#   - ~/mc-test/dl/paper-26.3.jar   (copy from my-project if absent)
#   - ~/mc-test/jdk25               (symlink)
#   - ~/mc-test/shim/libGL.so.1     (sRGB shim, gcc)
#   - dist root via setup_server.sh
# Client runtime (~474 MB) is NOT downloaded here; run setup_client.py separately.
set -euo pipefail

export PATH="$HOME/.cargo/bin:$PATH:/home/z/tools/glslang/bin:$PATH"
T=/home/z/tools
mkdir -p "$T" ~/mc-test/dl ~/mc-test/shim

# ---- rustup ---------------------------------------------------------------
if [ ! -x "$HOME/.cargo/bin/cargo" ]; then
  timeout 120 curl -sSf --max-time 60 https://sh.rustup.rs -o /tmp/rustup-init.sh
  sh /tmp/rustup-init.sh -y --profile minimal --default-toolchain stable >/tmp/rustup.log 2>&1
  rustup component add rustfmt clippy
fi
echo "rust: $(cargo --version)"

# ---- Temurin JDK 25 -------------------------------------------------------
if [ ! -x "$T/jdk-25.0.5+8/bin/java" ]; then
  timeout 400 curl -sSL --max-time 380 \
    "https://api.adoptium.net/v3/binary/latest/25/ga/linux/x64/jdk/hotspot/normal/eclipse" \
    -o "$T/jdk25.tar.gz"
  (cd "$T" && tar xzf jdk25.tar.gz)
fi
ln -sfn "$T/jdk-25.0.5+8" ~/mc-test/jdk25
# boot_server.sh resolves the JVM at /home/z/tools/jdk25 — keep BOTH
# symlinks alive (sandbox resets drop the tools dir wholesale).
ln -sfn "$T/jdk-25.0.5+8" "$T/jdk25"
"$T/jdk25/bin/java" -version 2>&1 | head -1

# ---- glslang 16.6.0 -------------------------------------------------------
if [ ! -x "$T/glslang/bin/glslangValidator" ]; then
  timeout 300 curl -sSL --max-time 280 \
    "https://github.com/KhronosGroup/glslang/releases/download/16.6.0/glslang-16.6.0-linux-x86_64-release.tar.gz" \
    -o "$T/glslang.tar.gz"
  mkdir -p "$T/glslang" && tar xzf "$T/glslang.tar.gz" -C "$T/glslang"
fi
glslangValidator --version | head -1

# ---- Paper jar ------------------------------------------------------------
if [ ! -f ~/mc-test/dl/paper-26.3.jar ]; then
  cp /home/z/my-project/paper-26.3.jar ~/mc-test/dl/paper-26.3.jar
fi
ls -la ~/mc-test/dl/paper-26.3.jar

# ---- GL shim --------------------------------------------------------------
if [ ! -f ~/mc-test/shim/libGL.so.1 ]; then
  gcc -shared -fPIC -O2 -o ~/mc-test/shim/libGL.so.1 \
    /home/z/my-project/c-LoD/scripts/bench/libclodgl.c -ldl
fi
echo "shim: $(ls -la ~/mc-test/shim/libGL.so.1)"

# ---- dist root ------------------------------------------------------------
bash /home/z/my-project/c-LoD/scripts/bench/setup_server.sh
echo "bench environment restored"
