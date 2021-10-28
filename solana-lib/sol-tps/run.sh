#!/bin/bash

pushd dex/crank
cargo run -- create_keypair --number 10
popd

#../target/debug/sol-tps sol create_keypair --number 10
# ../target/debug/sol-tps sol mint_account --number 10
# ../target/debug/sol-tps sol sol_transfer --number 10