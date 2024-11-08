#!/usr/bin/env bash
set -euo pipefail
[[ -n "${TRACE:-}" ]] && set -x
DIR="$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

cd "$DIR"/../template-cmd/upstream
./containerd.sh
./kubectl.sh
./logrus.sh
./pq.sh
./term.sh
./wasmtime.sh
