//! ERC721 EVM contract interoperability using XVM interface.
#![cfg_attr(not(feature = "std"), no_std)]

pub use self::erc721::{
    Erc721,
    Erc721Ref,
};
use ink_lang as ink;

/// EVM ID (from astar runtime)
const EVM_ID: u8 = 0x1F;

/// The EVM ERC721 delegation contract.
#[ink::contract(env = xvm_sdk::XvmDefaultEnvironment)]
mod erc721 {
    const IS_APPROVED_FOR_ALL_SELECTOR: [u8; 4] = hex!["todo"];

    use ethabi::{ethereum_types::{H160, U256}, Token};
    use ink_prelude::vec::Vec;
    use hex_literal::hex;

    #[ink(storage)]
    pub struct Erc721 {
        evm_address: [u8; 20],
    }

    impl Erc721 {
        /// Create new ERC20 abstraction from given contract address.
        #[ink(constructor)]
        pub fn new(evm_address: [u8; 20]) -> Self {
            Self {
                evm_address,
            }
        }

        // For now we can't handle methods returning complex values
        // #[ink(message)]
        // pub fn balance_of(&self, owner: [u8; 20]) -> u128 {
        //     let encoded_input = Self::balance_of_encode(owner.into());
        // }

        // fn balance_of_encode(to: H160, value: U256) -> Vec<u8> {
        //     let input = [
        //         Token::FixedBytes(BALANCE_OF_SELECTOR.to_vec()),
        //         Token::Address(to),
        //     ];
        //     ethabi::encode(&input)
        // }

        #[ink(message)]
        pub fn approve(to: [u8; 20], tokenId: U256) {

        }

        #[ink(message)]
        pub fn set_approval_for_all(operator: [u8; 20], approved: bool) {

        }

        #[ink(message)]
        pub fn is_approved_for_all(owner: [u8; 20], operator: [u8; 20]) -> bool {
            let input = [
                Token::FixedBytes(IS_APPROVED_FOR_ALL_SELECTOR.to_vec()),
                Token::Address(owner),
                Token::Address(operator),
            ];
            let input = ethabi::encode(&input);

            self.env()
                .extension()
                .xvm_call(
                    super::EVM_ID,
                    Vec::from(self.evm_address.as_ref()),
                    input,
                )
                .is_ok()
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
            
        }
    }
}
