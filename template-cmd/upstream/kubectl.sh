#!/usr/bin/env bash
set -euo pipefail
[[ -n "${TRACE:-}" ]] && set -x
DIR="$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

rm -rf "$DIR"/kubectl
git clone --depth 1 https://github.com/kubernetes/kubectl.git "$DIR"/kubectl
cd "$DIR"/kubectl
git fetch --depth=1 origin 3aac470db0f8dd11e588469ee0b14ffbb592dc8d
git checkout 3aac470db0f8dd11e588469ee0b14ffbb592dc8d
git apply "$DIR"/kubectl.patch
