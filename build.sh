#!/bin/bash
set -e 
cd "`dirname $0`"
cargo build --all --target wasm32-fungible-token --release
# cp target/wasm32-fungible-token/release/*.wasm ./out/
