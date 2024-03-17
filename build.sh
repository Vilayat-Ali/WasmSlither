#!/bin/sh
set -ex
RUST_LOG=info wasm-pack build
wasm-pack build --target web --out-name snake --out-dir ./web/static
node delete.js