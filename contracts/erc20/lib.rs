//! ERC20 EVM contract interoperability using XVM interface.
#![cfg_attr(not(feature = "std"), no_std)]

pub use self::erc20::{
    Erc20,
    Erc20Ref,
};

/// EVM ID (from astar runtime)
const EVM_ID: u8 = 0x0F;

/// The EVM ERC20 delegation contract.
#[ink::contract(env = xvm_environment::XvmDefaultEnvironment)]
mod erc20 {
    // ======= IERC20.sol:IERC20 =======
    // Function signatures:
    // dd62ed3e: allowance(address,address)
    // 095ea7b3: approve(address,uint256)
    // 70a08231: balanceOf(address)
    // 18160ddd: totalSupply()
    // a9059cbb: transfer(address,uint256)
    // 23b872dd: transferFrom(address,address,uint256)
    //
    const APPROVE_SELECTOR: [u8; 4] = hex!["095ea7b3"];
    const TRANSFER_SELECTOR: [u8; 4] = hex!["a9059cbb"];
    const TRANSFER_FROM_SELECTOR: [u8; 4] = hex!["23b872dd"];

    use ethabi::{
        ethereum_types::{
            H160,
            U256,
        },
        Token,
    };
    use hex_literal::hex;
    use ink::prelude::vec::Vec;

    #[ink(storage)]
    pub struct Erc20 {
        evm_address: [u8; 20],
    }

    impl Erc20 {
        /// Create new ERC20 abstraction from given contract address.
        #[ink(constructor)]
        pub fn new(evm_address: [u8; 20]) -> Self {
            Self { evm_address }
        }

        /// Send `approve` call to ERC20 contract.
        #[ink(message)]
        pub fn approve(&mut self, to: [u8; 20], value: u128) -> bool {
            let encoded_input = Self::approve_encode(to.into(), value.into());
            self.env()
                .extension()
                .xvm_call(
                    super::EVM_ID,
                    Vec::from(self.evm_address.as_ref()),
                    encoded_input,
                )
                .is_ok()
        }

        /// Send `transfer` call to ERC20 contract.
        #[ink(message)]
        pub fn transfer(&mut self, to: [u8; 20], value: u128) -> bool {
            let encoded_input = Self::transfer_encode(to.into(), value.into());
            self.env()
                .extension()
                .xvm_call(
                    super::EVM_ID,
                    Vec::from(self.evm_address.as_ref()),
                    encoded_input,
                )
                .is_ok()
        }

        /// Send `transfer_from` call to ERC20 contract.
        #[ink(message)]
        pub fn transfer_from(&mut self, from: [u8; 20], to: [u8; 20], value: u128) -> bool {
            let encoded_input = Self::transfer_from_encode(from.into(), to.into(), value.into());
            self.env()
                .extension()
                .xvm_call(
                    super::EVM_ID,
                    Vec::from(self.evm_address.as_ref()),
                    encoded_input,
                )
                .is_ok()
        }

        fn approve_encode(to: H160, value: U256) -> Vec<u8> {
            let mut encoded = APPROVE_SELECTOR.to_vec();
            let input = [Token::Address(to), Token::Uint(value)];
            encoded.extend(&ethabi::encode(&input));
            encoded
        }

        fn transfer_encode(to: H160, value: U256) -> Vec<u8> {
            let mut encoded = TRANSFER_SELECTOR.to_vec();
            let input = [Token::Address(to), Token::Uint(value)];
            encoded.extend(&ethabi::encode(&input));
            encoded
        }

        fn transfer_from_encode(from: H160, to: H160, value: U256) -> Vec<u8> {
            let mut encoded = TRANSFER_FROM_SELECTOR.to_vec();
            let input = [Token::Address(from), Token::Address(to), Token::Uint(value)];
            encoded.extend(&ethabi::encode(&input));
            encoded
        }
    }
}
