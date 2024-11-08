#!/usr/bin/env bash
set -euo pipefail
[[ -n "${TRACE:-}" ]] && set -x
DIR="$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

brew install asdf
echo '. $HOMEBREW_PREFIX/opt/asdf/libexec/asdf.sh' >> ~/.zshrc
. $HOMEBREW_PREFIX/opt/asdf/libexec/asdf.sh

asdf plugin add golang
asdf install golang 1.23.1
asdf global golang 1.23.1

asdf plugin add rust
asdf install rust 1.81.0
asdf global rust 1.81.0
rustup target add wasm32-unknown-unknown
cargo install cargo-component@0.17.0

curl -sSL https://github.com/WebAssembly/binaryen/releases/download/version_119/binaryen-version_119-x86_64-linux.tar.gz | \
 sudo tar -xz --strip-components=2 -C /usr/local/bin binaryen-version_119/bin

curl -sSL https://github.com/bytecodealliance/wasm-tools/releases/download/v1.219.1/wasm-tools-1.219.1-x86_64-linux.tar.gz | \
 sudo tar -xz --strip-components=1 -C /usr/local/bin wasm-tools-1.219.1-x86_64-linux/wasm-tools

curl -sSL https://github.com/WebAssembly/wabt/releases/download/1.0.36/wabt-1.0.36-ubuntu-20.04.tar.gz | \
 sudo tar -xz --strip-components=2 -C /usr/local/bin wabt-1.0.36/bin
