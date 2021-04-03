#!/bin/sh

cargo build --release --target wasm32-unknown-unknown
cp target/wasm32-unknown-unknown/release/simple_electronics.wasm docs
cp -r resources docs/

if [ "$1" = "serve" ]
then
    # cargo install basic-http-server
    basic-http-server docs
fi
