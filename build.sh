#!/bin/bash

set -e

cargo build --target wasm32-unknown-unknown

wasm-bindgen --out-dir=dist --target=web --omit-default-module-path target/wasm32-unknown-unknown/debug/museum.wasm

npx webpack

cd dist

npx serve .

cd ..
