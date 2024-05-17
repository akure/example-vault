#!/bin/bash

osmosisd tx wasm instantiate 1 '{"base_denom":"uosmo"}' --amount=300uosmo --label "test-01" --node tcp://localhost:26679 --from alice --keyring-backend test --home ~/.osmosis --chain-id osmosis  --fees 300000uosmo --gas 7000000 --admin="osmo1t8eh66t2w5k67kwurmn5gqhtq6d2ja0vp7jmmq"
