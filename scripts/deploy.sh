#!/bin/sh

source dev.env

make build

if [ $? -ne 0 ]; then
  echo ">> Error building contract"
  exit 1
fi

echo ">> Deploying contract"

near dev-deploy --wasmFile "res/sweat_booster.wasm" --initFunction "init" --initArgs "{\"ft_account_id\": \"$TOKEN_ACCOUNT_ID\"}"
