//! ERC721 EVM contract interoperability using XVM interface.
#![cfg_attr(not(feature = "std"), no_std)]

pub use self::erc721::{
    Erc721,
    Erc721Ref,
};

/// EVM ID (from astar runtime)
const EVM_ID: u8 = 0x0F;

/// The EVM ERC721 delegation contract.
#[ink::contract(env = xvm_environment::XvmDefaultEnvironment)]
mod erc721 {
    const APPROVE_SELECTOR: [u8; 4] = hex!["095ea7b3"];
    const TRANSFER_FROM_SELECTOR: [u8; 4] = hex!["23b872dd"];
    const MINT_SELECTOR: [u8; 4] = hex!["40c10f19"];

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
    pub struct Erc721 {
        evm_address: [u8; 20],
    }

    impl Erc721 {
        /// Create new ERC721 abstraction from given contract address.
        #[ink(constructor)]
        pub fn new(evm_address: [u8; 20]) -> Self {
            Self { evm_address }
        }

        #[ink(message)]
        pub fn transfer_from(&mut self, from: [u8; 20], to: [u8; 20], token_id: u128) -> bool {
            let encoded_input = Self::transfer_from_encode(from.into(), to.into(), token_id.into());
            self.env()
                .extension()
                .xvm_call(
                    super::EVM_ID,
                    Vec::from(self.evm_address.as_ref()),
                    encoded_input,
                )
                .is_ok()
        }

        #[ink(message)]
        pub fn approve(&mut self, to: [u8; 20], token_id: u128) -> bool {
            let encoded_input = Self::approve_encode(to.into(), token_id.into());
            self.env()
                .extension()
                .xvm_call(
                    super::EVM_ID,
                    Vec::from(self.evm_address.as_ref()),
                    encoded_input,
                )
                .is_ok()
        }

        #[ink(message)]
        pub fn mint(&mut self, to: [u8; 20], token_id: u128) -> bool {
            let encoded_input = Self::mint_encode(to.into(), token_id.into());
            self.env()
                .extension()
                .xvm_call(
                    super::EVM_ID,
                    Vec::from(self.evm_address.as_ref()),
                    encoded_input,
                )
                .is_ok()
        }

        fn transfer_from_encode(from: H160, to: H160, token_id: U256) -> Vec<u8> {
            let mut encoded = TRANSFER_FROM_SELECTOR.to_vec();
            let input = [
                Token::Address(from),
                Token::Address(to),
                Token::Uint(token_id),
            ];
            encoded.extend(&ethabi::encode(&input));
            encoded
        }

        fn approve_encode(to: H160, token_id: U256) -> Vec<u8> {
            let mut encoded = APPROVE_SELECTOR.to_vec();
            let input = [Token::Address(to), Token::Uint(token_id)];
            encoded.extend(&ethabi::encode(&input));
            encoded
        }

        fn mint_encode(to: H160, token_id: U256) -> Vec<u8> {
            let mut encoded = MINT_SELECTOR.to_vec();
            let input = [Token::Address(to), Token::Uint(token_id)];
            encoded.extend(&ethabi::encode(&input));
            encoded
        }
    }
}
