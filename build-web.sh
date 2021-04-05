#!/bin/sh

cargo build --release --target wasm32-unknown-unknown
cp target/wasm32-unknown-unknown/release/simple_electronics.wasm docs

# https://github.com/WebAssembly/wabt
wasm-strip docs/simple_electronics.wasm
cp -r resources docs/

if [ "$1" = "serve" ]
then
    # cargo install basic-http-server
    basic-http-server docs
fi
