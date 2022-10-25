//! ERC20 EVM contract interoperability using XVM interface.
#![cfg_attr(not(feature = "std"), no_std)]

pub use self::erc20::{
    Erc20,
    Erc20Ref,
};
use ink_lang as ink;

/// EVM ID (from astar runtime)
const EVM_ID: u8 = 0x0F;

/// The EVM ERC20 delegation contract.
#[ink::contract(env = xvm_sdk::XvmDefaultEnvironment)]
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

    use ethabi::{ethereum_types::{H160, U256}, Token};
    use ink_prelude::vec::Vec;
    use hex_literal::hex;

    #[ink(storage)]
    pub struct Erc20 {
        evm_address: [u8; 20],
    }

    impl Erc20 {
        /// Create new ERC20 abstraction from given contract address.
        #[ink(constructor)]
        pub fn new(evm_address: [u8; 20]) -> Self {
            Self {
                evm_address,
            }
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
        pub fn transfer(&mut self, to: [u8; 20], value: u128)  -> bool {
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
        pub fn transfer_from(&mut self, from: [u8; 20], to: [u8; 20], value: u128)  -> bool {
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
            let input = [
                Token::FixedBytes(APPROVE_SELECTOR.to_vec()),
                Token::Address(to),
                Token::Uint(value),
            ];
            ethabi::encode(&input)
        }

        fn transfer_encode(to: H160, value: U256) -> Vec<u8> {
            let input = [
                Token::FixedBytes(TRANSFER_SELECTOR.to_vec()),
                Token::Address(to),
                Token::Uint(value),
            ];
            ethabi::encode(&input)
        }

        fn transfer_from_encode(from: H160, to: H160, value: U256) -> Vec<u8> {
            let input = [
                Token::FixedBytes(TRANSFER_FROM_SELECTOR.to_vec()),
                Token::Address(from),
                Token::Address(to),
                Token::Uint(value),
            ];
            ethabi::encode(&input)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        const SAMPLE_CONTRACT: [u8; 20] = hex!["a10d3DBe7c28F46A90476B841B8509978e00B995"];
        const SAMPLE_TO: [u8; 20] = hex!["7C32982c3e7Fb8B2AF6ABD3323583C6A41f921C5"];
        const SAMPLE_FROM: [u8; 20] = hex!["d806D071365C9Cc3EE19873212A3E0D553Ab97Da"];
        const SAMPLE_VALUE: u128 = 12353241324u128;

        #[test]
        fn arguments_encoding() {
            let input = Erc20::approve_encode(SAMPLE_TO.into(), SAMPLE_VALUE.into());
            assert_eq!(input, hex![]);

            let input = Erc20::transfer_encode(SAMPLE_TO.into(), SAMPLE_VALUE.into());
            assert_eq!(input, hex![]);

            let input = Erc20::transfer_from_encode(SAMPLE_FROM.into(), SAMPLE_TO.into(), SAMPLE_VALUE.into());
            assert_eq!(input, hex![]);
        }
    }
}
