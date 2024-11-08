#!/usr/bin/env bash
set -euo pipefail
[[ -n "${TRACE:-}" ]] && set -x
DIR="$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

rm -rf "$DIR"/term
git clone --depth 1 https://github.com/moby/term.git "$DIR"/term
cd "$DIR"/term
git fetch --depth=1 origin 9c3c875fad924eb6c9dd32a361b5fc0a49a4feb9
git checkout 9c3c875fad924eb6c9dd32a361b5fc0a49a4feb9
git apply "$DIR"/term.patch
