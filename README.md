# Ink! XVM SDK 

This repository contains everything needed to use XVM from WASM contracts.
It contains an implementation of XVM chain-extension to use in your contracts.   
As well as ink! contracts SDK that implements XVM chain-extension to be used as is.

## Contracts SDK

XVMv2 can only process transactions that returns `()` hence query values is not supported yet. These contracts only implements functions that modify state.   
Transactions pass multiple layers of XVM abstractions in one line. All cross-VM communication looks like it all going inside the smart contract.

#### ERC20
This implementation is a controller of an underlying `ERC20` on EVM. Interact with `H160` addresses 

#### PSP22 Controller
This implementation is a controller of an underlying `ERC20` on EVM. Interact with `H256` native substrate addresses.
It implements `PSP22` standard, thus can be used in any DEX/wallet supporting it.

#### PSP22 Wrapper
This implementation is a wrapper of an underlying `ERC20` on EVM. Interact with `H256` native substrate addresses.   
As it implements wrapper pattern it has `deposit` & `withdraw` function and can be used as a bridgeless solution between WASM VM & EVM.   
It implements `PSP22` standard, thus can be used in any DEX/wallet supporting it.

#### ERC721
This implementation is a controller of an underlying `ERC721` on EVM. Interact with `H160` addresses

#### PSP34 Controller
This implementation is a controller of an underlying `ERC721` on EVM. Interact with `H256` native substrate addresses.
It implements `PSP34` standard, thus can be used in any DEX/wallet supporting it.

#### PSP34 Wrapper
This implementation is a wrapper of an underlying `ERC721` on EVM. Interact with `H256` native substrate addresses.   
As it implements wrapper pattern it has `deposit` & `withdraw` function and can be used as a bridgeless solution between WASM VM & EVM.   
It implements `PSP34` standard, thus can be used in any DEX/wallet supporting it.


## Library
#### XVM environment
Implementation of XVM chain extension added to a custom `XvmDefaultEnvironment`.   
1. Import the crate in your Cargo.Toml   
2. Add it to your contract in ink! macro `#[ink::contract(env = xvm_sdk::XvmDefaultEnvironment)]`.   
3. In your contract use it with `self.env().extension().xvm_call(..args)`.

#### XVM helper
This crate exposes `XvmErc20` struct that implements functions of ERC20 chain-extension.   
It makes it compatible with other custom environment like openbrush.   
HAve a look at PSP22 Wrapper for an example.
1. Import the crate in your Cargo.Toml   
2. Import struct in your contract use `use xvm_sdk_helper::XvmErc20;`   
3. Use it with `XvmErc20::transfer(..args)`

## Usage 

##### Try it local!
1. Build & run [Astar](https://github.com/AstarNetwork/Astar) in local `./target/release/astar-collator --dev --tmp`  
2. Add [Test account](https://github.com/AstarNetwork/Astar/blob/de5b8db29794917ffab8fb0a4a7b2a9a52491452/bin/collator/src/local/chain_spec.rs#L61-L66) that is funded with native token to metamask .
3. Using Remix deploys an ERC20 (or ERC721 mint) using injected provider to Astar local & the test account
4. Transfer native tokens from test account to alice H160 `0xd43593c715fdd31c61141abd04a99fd6822c8558`
5. Transfer ERC20 or mint ERC721 token from test account to Alice H160
6. Deploy ink! contract with ERC20/ERC721 address from EVM
7. Play with it!

Note:
Gas fees are paid by the caller() so in Wrapper contracts the contract H160 address should have native token as well (for withdraw function)   
To get H160 address of a H160 address use subkey ex: `./target/release/astar-collator key inspect Yn53Kb3EZcsgLvJNC64TRAqbAhk4NbqtaGznAhBycBHzy61`   
And use the 20 first bytes of the `Public key (hex)`
