#!/bin/sh

cargo +nightly contract build --manifest-path ./xvm-sdk/erc20/Cargo.toml
cargo +nightly contract build
