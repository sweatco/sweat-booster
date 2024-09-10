#!/bin/bash
set -eox pipefail

echo ">> Building contract"

rustup target add wasm32-unknown-unknown

cd contract
cargo near build --no-locked --no-docker --out-dir ../res
