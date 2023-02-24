# Ink! XVM SDK

This repository contains everything needed to use XVM from WASM contracts.
It contains an implementation of XVM chain-extension to use in your contracts.
As well as ink! contracts SDK that implements XVM chain-extension to be used as is.

## Contracts SDK

XVMv2 can only process transactions that returns `()` hence query values is not supported yet. These contracts only implement functions that modify state.
Transactions pass multiple layers of XVM abstractions in one line. All cross-VM communication looks like it all going inside the smart contract.

#### ERC20

This implementation is a controller of an underlying `ERC20` on EVM. Interact with `H160` addresses

#### PSP22 Controller

This implementation is a controller of an underlying `ERC20` on EVM. Interact with `H256` native substrate addresses.
It implements `PSP22` standard, and thus can be used in any DEX/wallet supporting it.

#### PSP22 Wrapper

This implementation is a wrapper of an underlying `ERC20` on EVM. Interact with `H256` native substrate addresses.
As it implements wrapper pattern it has `deposit` & `withdraw` function and can be used as a bridgeless solution between WASM VM & EVM.
It implements `PSP22` standard, thus can be used in any DEX/wallet supporting it.

#### ERC721

This implementation is a controller of an underlying `ERC721` on EVM. Interact with `H160` addresses

#### PSP34 Controller

This implementation is a controller of an underlying `ERC721` on EVM. Interact with `H256` native substrate addresses.
It implements `PSP34` standard, and thus can be used in any DEX/wallet supporting it.

#### PSP34 Wrapper

This implementation is a wrapper of an underlying `ERC721` on EVM. Interact with `H256` native substrate addresses.
As it implements wrapper pattern it has `deposit` & `withdraw` function and can be used as a bridgeless solution between WASM VM & EVM.
It implements `PSP34` standard, and thus can be used in any DEX/wallet supporting it.

#### XVM Transfer

This implementation is a controller of an underlying `ERC20` on EVM. Interact with both `H256` native substrate and `H160` addresses. This is a universal contract where one of the parameters for the transfer function is the ERC20 contract address. This contract is used on [Astar Portal](https://portal.astar.network/).

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

##### Try it local!

1. Build & run [Astar](https://github.com/AstarNetwork/Astar) in local `./target/release/astar-collator --dev --tmp`
2. Add [Test account](https://github.com/AstarNetwork/Astar/blob/de5b8db29794917ffab8fb0a4a7b2a9a52491452/bin/collator/src/local/chain_spec.rs#L61-L66) that is funded with native token to metamask .
3. Using Remix IDE deploy an ERC20 (or ERC721 mint) using injected provider to Astar local & the test account.
4. Transfer ERC20 or mint ERC721 token from test account to Alice H160.
5. Deploy ink! contract with ERC20/ERC721 address from EVM.
6. Play with it!

Use [Hoon's address converter](https://hoonsubin.github.io/evm-substrate-address-converter/)
