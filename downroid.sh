#!/usr/bin/env sh
set -e
[ "$(uname -m)" = aarch64 ] || { echo "downflac: only ARM64 builds are published"; exit 1; }
curl -fsSL https://github.com/rohambob212/downflac/releases/latest/download/downflac-aarch64-linux-android.tar.gz \
  | tar -xz -C "$PREFIX/bin" downflac
echo "installed: $(command -v downflac)"
