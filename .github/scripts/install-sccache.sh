#!/bin/bash
set -euo pipefail

mkdir -p "${HOME}/sccache"
curl -sSL "https://github.com/mozilla/sccache/releases/download/v${SCCACHE_VERSION}/sccache-v${SCCACHE_VERSION}-x86_64-unknown-linux-musl.tar.gz" -o sccache.tar.gz
echo "${SCCACHE_SHA256} sccache.tar.gz" | sha256sum -c
tar xzv -C "${HOME}/sccache" --strip-components=1 -f sccache.tar.gz
chmod +x "${HOME}/sccache/sccache"
rm sccache.tar.gz
