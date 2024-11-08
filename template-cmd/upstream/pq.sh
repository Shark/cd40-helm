#!/usr/bin/env bash
set -euo pipefail
[[ -n "${TRACE:-}" ]] && set -x
DIR="$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

rm -rf "$DIR"/pq
git clone --depth 1 https://github.com/lib/pq.git "$DIR"/pq
cd "$DIR"/pq
git fetch --depth=1 origin 2a217b94f5ccd3de31aec4152a541b9ff64bed05
git checkout 2a217b94f5ccd3de31aec4152a541b9ff64bed05
git apply "$DIR"/pq.patch
