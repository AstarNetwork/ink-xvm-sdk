//! PSP22 Controller of an ERC20 EVM contract interoperability using XVM interface.
#![cfg_attr(not(feature = "std"), no_std)]

pub use self::psp22::{
    Psp22,
    Psp22Ref,
};

/// EVM ID (from astar runtime)
const EVM_ID: u8 = 0x0F;

#[ink::contract(env = xvm_environment::XvmDefaultEnvironment)]
mod psp22 {
    use ethabi::{
        ethereum_types::{
            H160,
            U256,
        },
        Token,
    };
    use hex_literal::hex;
    use ink::prelude::{
        string::String,
        vec::Vec,
    };

    const APPROVE_SELECTOR: [u8; 4] = hex!["095ea7b3"];
    const TRANSFER_SELECTOR: [u8; 4] = hex!["a9059cbb"];
    const TRANSFER_FROM_SELECTOR: [u8; 4] = hex!["23b872dd"];

    /// Only one Error is supported
    #[derive(Debug, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum PSP22Error {
        Custom(String),
    }

    #[ink(storage)]
    pub struct Psp22 {
        evm_address: [u8; 20],
    }

    impl Psp22 {
        #[ink(constructor)]
        pub fn new(evm_address: [u8; 20]) -> Self {
            Self { evm_address }
        }

        #[ink(message, selector = 0xb20f1bbd)]
        pub fn approve(&mut self, spender: AccountId, value: Balance) -> Result<(), PSP22Error> {
            let encoded_input = Self::approve_encode(Self::h160(&spender), value.into());
            self.env()
                .extension()
                .xvm_call(
                    super::EVM_ID,
                    Vec::from(self.evm_address.as_ref()),
                    encoded_input,
                )
                .map_err(|_| PSP22Error::Custom(String::from("approve failed")))
        }

        #[ink(message, selector = 0xdb20f9f5)]
        pub fn transfer(
            &mut self,
            to: AccountId,
            value: Balance,
            _data: Vec<u8>,
        ) -> Result<(), PSP22Error> {
            let encoded_input = Self::transfer_encode(Self::h160(&to), value.into());
            self.env()
                .extension()
                .xvm_call(
                    super::EVM_ID,
                    Vec::from(self.evm_address.as_ref()),
                    encoded_input,
                )
                .map_err(|_| PSP22Error::Custom(String::from("transfer failed")))
        }

        #[ink(message, selector = 0x54b3c76e)]
        pub fn transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            value: Balance,
            _data: Vec<u8>,
        ) -> Result<(), PSP22Error> {
            let encoded_input =
                Self::transfer_from_encode(Self::h160(&from), Self::h160(&to), value.into());
            self.env()
                .extension()
                .xvm_call(
                    super::EVM_ID,
                    Vec::from(self.evm_address.as_ref()),
                    encoded_input,
                )
                .map_err(|_| PSP22Error::Custom(String::from("transfer_from failed")))
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

        fn h160(from: &AccountId) -> H160 {
            let mut dest: H160 = [0; 20].into();
            dest.as_bytes_mut()
                .copy_from_slice(&<AccountId as AsRef<[u8]>>::as_ref(from)[..20]);
            dest
        }
    }
}
