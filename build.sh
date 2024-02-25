#!/bin/sh

set -ex
wasm-pack build --target web --out-name snake --out-dir ./web/static