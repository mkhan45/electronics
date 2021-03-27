#!/bin/sh

cargo build --release --target wasm32-unknown-unknown
cp target/wasm32-unknown-unknown/release/electrosim.wasm web

if [ "$1" = "serve" ]
then
    # cargo install basic-http-server
    basic-http-server web
fi
