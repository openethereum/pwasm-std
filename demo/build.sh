#!/bin/sh

set -e

# Requires wasm-build util, see https://github.com/paritytech/wasm-utils to install

cargo build --release --verbose --target wasm32-unknown-emscripten
wasm-build ./target/wasm wasm_demo
