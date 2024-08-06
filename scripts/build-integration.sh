#!/bin/bash
set -eox pipefail

echo ">> Building contract"

rustup target add wasm32-unknown-unknown
cargo build -p sweat_booster --target wasm32-unknown-unknown --profile=contract --features integration-test,

cp ./target/wasm32-unknown-unknown/contract/sweat_booster.wasm res/sweat_booster.wasm
