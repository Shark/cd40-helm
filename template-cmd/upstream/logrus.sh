#!/usr/bin/env bash
set -euo pipefail
[[ -n "${TRACE:-}" ]] && set -x
DIR="$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

rm -rf "$DIR"/logrus
git clone --depth 1 https://github.com/sirupsen/logrus.git "$DIR"/logrus
cd "$DIR"/logrus
git fetch --depth=1 origin dd1b4c2e81afc5c255f216a722b012ed26be57df
git checkout dd1b4c2e81afc5c255f216a722b012ed26be57df
git apply "$DIR"/logrus.patch
