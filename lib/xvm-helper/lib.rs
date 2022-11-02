#![cfg_attr(not(feature = "std"), no_std)]

use ethabi::{
    ethereum_types::{
        H160,
        U256,
    },
    Token,
};
use hex_literal::hex;
use ink_env::AccountId;
use ink_prelude::vec::Vec;
use openbrush::traits::Balance;

const EVM_ID: u8 = 0x0F;
const APPROVE_SELECTOR: [u8; 4] = hex!["095ea7b3"];
const TRANSFER_SELECTOR: [u8; 4] = hex!["a9059cbb"];
const TRANSFER_FROM_SELECTOR: [u8; 4] = hex!["23b872dd"];

pub struct Xvm;
impl Xvm {
    fn xvm_call(vm_id: u8, target: Vec<u8>, input: Vec<u8>) -> Result<(), XvmError> {
        ::ink_env::chain_extension::ChainExtensionMethod::build(0x00010001)
            .input::<(u8, Vec<u8>, Vec<u8>)>()
            .output::<()>()
            .handle_error_code::<XvmError>()
            .call(&(vm_id, target, input))
    }
}
pub struct XvmErc20;
impl XvmErc20 {
    pub fn approve(evm_contract_address: [u8; 20], spender: AccountId, value: Balance) -> Result<(), XvmError> {
        let encoded_input = Self::approve_encode(Self::h160(&spender), value.into());
        Xvm::xvm_call(
            EVM_ID,
            Vec::from(evm_contract_address.as_ref()),
            encoded_input,
        )
    }

    pub fn transfer(
        evm_contract_address: [u8; 20],
        to: AccountId,
        value: Balance,
        _data: Vec<u8>,
    ) -> Result<(), XvmError> {
        let encoded_input =
            Self::transfer_encode(Self::h160(&to), value.into());
        Xvm::xvm_call(
            EVM_ID,
            Vec::from(evm_contract_address.as_ref()),
            encoded_input,
        )
    }

    pub fn transfer_from(
        evm_contract_address: [u8; 20],
        from: AccountId,
        to: AccountId,
        value: Balance,
        _data: Vec<u8>,
    ) -> Result<(), XvmError> {
        let encoded_input =
            Self::transfer_from_encode(Self::h160(&from), Self::h160(&to), value.into());
        Xvm::xvm_call(
            EVM_ID,
            Vec::from(evm_contract_address.as_ref()),
            encoded_input,
        )
    }

    fn approve_encode(to: H160, value: U256) -> Vec<u8> {
        let mut encoded = APPROVE_SELECTOR.to_vec();
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

    fn transfer_encode(to: H160, value: U256) -> Vec<u8> {
        let mut encoded = TRANSFER_SELECTOR.to_vec();
        let input = [Token::Address(to), Token::Uint(value)];
        encoded.extend(&ethabi::encode(&input));
        encoded
    }

    fn h160(from: &AccountId) -> H160 {
        let mut dest: H160 = [0; 20].into();
        dest.as_bytes_mut()
            .copy_from_slice(&<ink_env::AccountId as AsRef<[u8]>>::as_ref(from)[..20]);
        dest
    }
}

#[derive(scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum XvmError {
    FailXvmCall,
}

impl ink_env::chain_extension::FromStatusCode for XvmError {
    fn from_status_code(status_code: u32) -> core::result::Result<(), Self> {
        match status_code {
            0 => Ok(()),
            1 => Err(Self::FailXvmCall),
            _ => panic!("encountered unknown status code"),
        }
    }
}

impl From<scale::Error> for XvmError {
    fn from(_: scale::Error) -> Self {
        panic!("encountered unexpected invalid SCALE encoding")
    }
}
