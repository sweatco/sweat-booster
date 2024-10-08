#!/bin/bash
set -eox pipefail

echo ">> Building contract"

rustup target add wasm32-unknown-unknown
cargo build -p sweat-booster --target wasm32-unknown-unknown --profile=release --features integration-test,

cp ./target/wasm32-unknown-unknown/release/sweat_booster.wasm res/sweat_booster.wasm
