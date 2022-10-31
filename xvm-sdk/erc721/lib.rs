//! ERC721 EVM contract interoperability using XVM interface.
#![cfg_attr(not(feature = "std"), no_std)]

pub use self::erc721::{
    Erc721,
    Erc721Ref,
};
use ink_lang as ink;

/// EVM ID (from astar runtime)
const EVM_ID: u8 = 0x0F;

/// The EVM ERC721 delegation contract.
#[ink::contract(env = xvm_sdk::XvmDefaultEnvironment)]
mod erc721 {
    const APPROVE_SELECTOR: [u8; 4] = hex!["095ea7b3"];
    const TRANSFER_FROM_SELECTOR: [u8; 4] = hex!["095ea7b3"];
    const MINT_SELECTOR: [u8; 4] = hex!["40c10f19"];

    use ethabi::{ethereum_types::{H160, U256}, Token};
    use ink_prelude::vec::Vec;
    use hex_literal::hex;

    #[ink(storage)]
    pub struct Erc721 {
        evm_address: [u8; 20],
    }

    impl Erc721 {
        /// Create new ERC721 abstraction from given contract address.
        #[ink(constructor)]
        pub fn new(evm_address: [u8; 20]) -> Self {
            Self {
                evm_address,
            }
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
    }
}
