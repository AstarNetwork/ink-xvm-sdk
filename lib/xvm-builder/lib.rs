#![cfg_attr(not(feature = "std"), no_std)]

use ink::{
    env::chain_extension::FromStatusCode,
    prelude::vec::Vec,
};

pub struct Xvm;
impl Xvm {
    pub fn xvm_call(vm_id: u8, target: Vec<u8>, input: Vec<u8>) -> Result<(), XvmError> {
        ::ink::env::chain_extension::ChainExtensionMethod::build(0x00010001)
            .input::<(u8, Vec<u8>, Vec<u8>)>()
            .output::<(), false>()
            .handle_error_code::<XvmError>()
            .call(&(vm_id, target, input))
    }
}

#[derive(scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum XvmError {
    FailXvmCall,
    UnknownStatusCode,
}

impl FromStatusCode for XvmError {
    fn from_status_code(status_code: u32) -> core::result::Result<(), Self> {
        match status_code {
            0 => Ok(()),
            1 => Err(Self::FailXvmCall),
            _ => Err(Self::UnknownStatusCode),
        }
    }
}
