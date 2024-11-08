#!/usr/bin/env bash
set -euo pipefail
[[ -n "${TRACE:-}" ]] && set -x
DIR="$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

rm -rf "$DIR"/wasmtime
git clone --depth 1 https://github.com/bytecodealliance/wasmtime.git "$DIR"/wasmtime
cd "$DIR"/wasmtime
git fetch --depth=1 origin cbdf173f5b03b0803e37b13a106384301fcc5acb
git checkout cbdf173f5b03b0803e37b13a106384301fcc5acb
cd "$DIR"/wasmtime/crates/wasi-preview1-component-adapter
cargo build -p wasi-preview1-component-adapter --target wasm32-unknown-unknown --release --features command --no-default-features
