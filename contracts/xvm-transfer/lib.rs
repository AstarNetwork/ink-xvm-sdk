//! Contract for transferring ERC20 tokens from SS58 accounts to SS58 or H160 accounts.
#![cfg_attr(not(feature = "std"), no_std)]

/// EVM ID (from astar runtime)
const EVM_ID: u8 = 0x0f;

#[ink::contract(env = xvm_environment::XvmDefaultEnvironment)]
mod xvm_transfer {
    use ethabi::{
        ethereum_types::{
            H160,
            U256,
        },
        Token,
    };
    use hex_literal::hex;
    use ink::prelude::vec::Vec;

    const TRANSFER_SELECTOR: [u8; 4] = hex!["a9059cbb"];

    #[derive(Debug, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum To {
        EVM([u8; 20]),
        WASM(AccountId),
    }

    impl From<To> for H160 {
        fn from(f: To) -> Self {
            return match f {
                To::EVM(a) => a.into(),
                To::WASM(a) => {
                    let mut dest: H160 = [0; 20].into();
                    dest.as_bytes_mut()
                        .copy_from_slice(&<AccountId as AsRef<[u8]>>::as_ref(&a)[..20]);
                    dest
                }
            }
        }
    }

    #[ink(storage)]
    pub struct XvmTransfer {}

    impl XvmTransfer {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

        #[ink(message)]
        pub fn transfer(&mut self, to: To, value: u128, erc20_address: [u8; 20]) -> bool {
            let encoded_input = Self::transfer_encode(to.into(), value.into());
            self.env()
                .extension()
                .xvm_call(
                    super::EVM_ID,
                    Vec::from(erc20_address.as_ref()),
                    encoded_input,
                )
                .is_ok()
        }

        fn transfer_encode(to: H160, value: U256) -> Vec<u8> {
            let mut encoded = TRANSFER_SELECTOR.to_vec();
            let input = [Token::Address(to), Token::Uint(value)];
            encoded.extend(&ethabi::encode(&input));
            encoded
        }
    }
}
