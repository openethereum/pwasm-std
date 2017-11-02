#!/bin/sh

set -e

# Requires wasm-build util, see https://github.com/paritytech/wasm-utils to install

if ! [ -x "$(command -v wasm-build)" ]; then
  cargo +nightly install --git https://github.com/paritytech/wasm-utils wasm-build
fi

cargo build --release --verbose --target wasm32-unknown-emscripten
wasm-build ./target/wasm wasm-demo