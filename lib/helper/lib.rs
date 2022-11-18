#![cfg_attr(not(feature = "std"), no_std)]

use xvm_builder::*;
use ink_prelude::vec::Vec;
use ink_env::AccountId;
use ethabi::{
    ethereum_types::{
        H160,
        U256,
    },
    Token,
};
use hex_literal::hex;
type Balance = <ink_env::DefaultEnvironment as ink_env::Environment>::Balance;

const EVM_ID: u8 = 0x0F;
const APPROVE_SELECTOR: [u8; 4] = hex!["095ea7b3"];
const TRANSFER_SELECTOR: [u8; 4] = hex!["a9059cbb"];
const TRANSFER_FROM_SELECTOR: [u8; 4] = hex!["23b872dd"];
const MINT_SELECTOR: [u8; 4] = hex!["40c10f19"];

pub struct XvmErc20;

impl XvmErc20 {
    pub fn approve(evm_contract_address: [u8; 20], spender: AccountId, value: Balance) -> Result<(), XvmError> {
        let encoded_input = Self::approve_encode(h160(&spender), value.into());
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
            Self::transfer_encode(h160(&to), value.into());
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
            Self::transfer_from_encode(h160(&from), h160(&to), value.into());
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
}

pub struct XvmErc721;

impl XvmErc721 {
    pub fn transfer_from(evm_contract_address: [u8; 20], from: AccountId, to: AccountId, id: U256) -> Result<(), XvmError> {
        let encoded_input =
            Self::transfer_from_encode(h160(&from), h160(&to), id);
        Xvm::xvm_call(
            EVM_ID,
            Vec::from(evm_contract_address.as_ref()),
            encoded_input,
        )
    }

    pub fn approve(evm_contract_address: [u8; 20], spender: AccountId, id: U256) -> Result<(), XvmError> {
        let encoded_input = Self::approve_encode(h160(&spender), id);
        Xvm::xvm_call(
            EVM_ID,
            Vec::from(evm_contract_address.as_ref()),
            encoded_input,
        )
    }

    pub fn mint(evm_contract_address: [u8; 20], to: AccountId, id: U256) -> Result<(), XvmError> {
        let encoded_input = Self::mint_encode(h160(&to), id.into());
        Xvm::xvm_call(
            EVM_ID,
            Vec::from(evm_contract_address.as_ref()),
            encoded_input,
        )
    }

    fn transfer_from_encode(from: H160, to: H160, id: U256) -> Vec<u8> {
        let mut encoded = TRANSFER_FROM_SELECTOR.to_vec();
        let input = [
            Token::Address(from),
            Token::Address(to),
            Token::Uint(id),
        ];
        encoded.extend(&ethabi::encode(&input));
        encoded
    }

    fn approve_encode(to: H160, id: U256) -> Vec<u8> {
        let mut encoded = APPROVE_SELECTOR.to_vec();
        let input = [Token::Address(to), Token::Uint(id)];
        encoded.extend(&ethabi::encode(&input));
        encoded
    }

    fn mint_encode(to: H160, id: U256) -> Vec<u8> {
        let mut encoded = MINT_SELECTOR.to_vec();
        let input = [Token::Address(to), Token::Uint(id)];
        encoded.extend(&ethabi::encode(&input));
        encoded
    }
}

fn h160(from: &AccountId) -> H160 {
    let mut dest: H160 = [0; 20].into();
    dest.as_bytes_mut()
        .copy_from_slice(&<ink_env::AccountId as AsRef<[u8]>>::as_ref(from)[..20]);
    dest
}