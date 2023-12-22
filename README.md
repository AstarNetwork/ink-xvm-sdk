# Ink! XVM SDK

This repository contains examples contracts using XVM to call EVM from ink! needed to use XVM from WASM contracts.
It contains an implementation of XVM chain-extension to use in your contracts.
As well as ink! contracts SDK that implements XVM chain-extension to be used as is.

## Contracts SDK

#### PSP22 Wrapper

This implementation is a wrapper of an underlying `ERC20` on EVM. Interact with native substrate addresses.
As it implements wrapper pattern it has `deposit` & `withdraw` function and can be used as a bridgeless solution between WASM VM & EVM.
It implements `PSP22` standard, thus can be used in any DEX/wallet supporting it.
Please have a look at the tests that describe the flow to use `deposit` and `withdraw`.

#### PSP34 Wrapper

This implementation is a wrapper of an underlying `ERC721` on EVM. Interact with substrate native substrate addresses.
As it implements wrapper pattern it has `deposit` & `withdraw` function and can be used as a bridgeless solution between WASM VM & EVM.
It implements `PSP34` standard, and thus can be used in any DEX/wallet supporting it.

## Library

#### XVM environment

Implementation of XVM chain extension added to a custom `XvmDefaultEnvironment`.

1. Import the crate in your Cargo.toml
2. Add it to your contract in ink! macro `#[ink::contract(env = xvm_sdk::XvmDefaultEnvironment)]`.
3. In your contract use it with `self.env().extension().xvm_call(..args)`.

#### XVM Builder

This crate exposes `Xvm` struct that implements xvm_call with chain-extension builder from ink_env.
It makes it compatible with other custom environment like openbrush.
Have a look at PSP22 Wrapper for an example.

1. Import the crate in your Cargo.toml
2. Import struct in your contract use `use xvm_helper::*;`
3. Use it with `XvmErc20::transfer(..args)`

## Usage

##### Try it!

1. Clone the repo
2. Run `yarn`
3. Build ink! contracts `yarn build:ink`

**To run on local node:**
Ensure you have a local node running with `./target/release/astar-collator --dev -lruntime::contracts=debug -l=runtime=debug,xvm=trace --enable-evm-rpcp` (to have XVM and ink! logs).     
Then run `yarn test`.

**To run on Shibuya:**
Create a .env file from .env.example and fill it with your credentials:
Add your Shibuya EVM private key in `ACCOUNT_PRIVATE_KEY_EVM`
And your Shibuya Substrate passphrase in `SUBSTRATE_MNEMO`.
Then run `yarn test:shibuya`.
