#!/bin/bash

osmosisd tx wasm store ../artifacts/example_vault.wasm --node tcp://localhost:26679 --from alice --keyring-backend test --home ~/.osmosis --chain-id osmosis  --fees 300000uosmo --gas 7000000
