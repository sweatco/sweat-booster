#!/bin/sh

make build

if [ $? -ne 0 ]; then
  echo ">> Error building contract"
  exit 1
fi

cd contract
cargo near deploy
