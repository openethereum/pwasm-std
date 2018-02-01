#!/bin/sh

set -e

# Requires wasm-build util, see https://github.com/paritytech/wasm-utils to install

cargo build --release --verbose --target wasm32-unknown-unknown
wasm-build --target wasm32-unknown-unknown ./target/wasm wasm_demo
