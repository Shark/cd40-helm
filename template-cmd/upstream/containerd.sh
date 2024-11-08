#!/usr/bin/env bash
set -euo pipefail
[[ -n "${TRACE:-}" ]] && set -x
DIR="$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

rm -rf "$DIR"/containerd
git clone --depth 1 https://github.com/containerd/containerd.git "$DIR"/containerd
cd "$DIR"/containerd
git fetch --depth=1 origin 71909c1814c544ac47ab91d2e8b84718e517bb99
git checkout 71909c1814c544ac47ab91d2e8b84718e517bb99
git apply "$DIR"/containerd.patch
