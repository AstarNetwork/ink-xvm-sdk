//! PSP34 Controller of an ERC721 EVM contract interoperability using XVM interface.
#![cfg_attr(not(feature = "std"), no_std)]

pub use self::psp34::{
    PSP34Controller,
    PSP34ControllerRef,
};

/// EVM ID (from astar runtime)
const EVM_ID: u8 = 0x0F;

#[ink::contract(env = xvm_environment::XvmDefaultEnvironment)]
mod psp34 {
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
    const TRANSFER_FROM_SELECTOR: [u8; 4] = hex!["23b872dd"];
    const MINT_SELECTOR: [u8; 4] = hex!["40c10f19"];

    #[derive(scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Id {
        U8(u8),
        U16(u16),
        U32(u32),
        U64(u64),
        U128(u128),
        Bytes(Vec<u8>),
    }

    /// Only one Error is supported
    #[derive(Debug, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum PSP34Error {
        Custom(String),
    }

    #[ink(storage)]
    pub struct PSP34Controller {
        evm_address: [u8; 20],
    }

    impl PSP34Controller {
        #[ink(constructor)]
        pub fn new(evm_address: [u8; 20]) -> Self {
            Self { evm_address }
        }

        #[ink(message, selector = 0x1932a8b0)]
        pub fn approve(
            &mut self,
            operator: AccountId,
            id: Option<Id>,
            _approved: bool,
        ) -> Result<(), PSP34Error> {
            if id.is_none() {
                return Err(PSP34Error::Custom(String::from("Id should not be None")))
            }
            let encoded_input = Self::approve_encode(Self::h160(&operator), id.unwrap().into());
            self.env()
                .extension()
                .xvm_call(
                    super::EVM_ID,
                    Vec::from(self.evm_address.as_ref()),
                    encoded_input,
                )
                .map_err(|_| PSP34Error::Custom(String::from("approve failed")))
        }

        #[ink(message, selector = 0x3128d61b)]
        pub fn transfer(
            &mut self,
            to: AccountId,
            id: Id,
            _data: Vec<u8>,
        ) -> Result<(), PSP34Error> {
            let caller = self.env().caller();
            let encoded_input =
                Self::transfer_from_encode(Self::h160(&caller), Self::h160(&to), id.into());
            self.env()
                .extension()
                .xvm_call(
                    super::EVM_ID,
                    Vec::from(self.evm_address.as_ref()),
                    encoded_input,
                )
                .map_err(|_| PSP34Error::Custom(String::from("transfer failed")))
        }

        // This is not part of PSP34 so there is no standard selector
        #[ink(message)]
        pub fn mint(&mut self, to: AccountId, id: Id) -> Result<(), PSP34Error> {
            let encoded_input = Self::mint_encode(Self::h160(&to), id.into());
            self.env()
                .extension()
                .xvm_call(
                    super::EVM_ID,
                    Vec::from(self.evm_address.as_ref()),
                    encoded_input,
                )
                .map_err(|_| PSP34Error::Custom(String::from("mint failed")))
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

        fn h160(from: &AccountId) -> H160 {
            let mut dest: H160 = [0; 20].into();
            dest.as_bytes_mut()
                .copy_from_slice(&<AccountId as AsRef<[u8]>>::as_ref(from)[..20]);
            dest
        }
    }

    impl From<Id> for U256 {
        fn from(id: Id) -> Self {
            return match id {
                Id::U8(v) => U256::from(v),
                Id::U16(v) => U256::from(v),
                Id::U32(v) => U256::from(v),
                Id::U64(v) => U256::from(v),
                Id::U128(v) => U256::from(v),
                Id::Bytes(v) => U256::from(v.as_slice()),
            }
        }
    }
}
