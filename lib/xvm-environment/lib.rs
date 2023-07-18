//! The XVM public interface for Ink! smart contracts.
#![cfg_attr(not(feature = "std"), no_std)]
use ink::{
    env::{
        chain_extension::FromStatusCode,
        DefaultEnvironment,
        Environment,
    },
    prelude::vec::Vec,
};

/// General result type.
pub type Result<T> = core::result::Result<T, XvmError>;

/// The XVM chain extension adapter.
#[ink::chain_extension]
pub trait XvmExtension {
    type ErrorCode = XvmError;

    #[ink(extension = 0x00010001)]
    fn xvm_call(vm_id: u8, target: Vec<u8>, input: Vec<u8>) -> Result<Vec<u8>>;
}

/// XVM chain extension errors.
#[derive(scale::Encode, scale::Decode, Debug)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum XvmError {
    FailXvmCall,
}

impl FromStatusCode for XvmError {
    fn from_status_code(status_code: u32) -> core::result::Result<(), Self> {
        match status_code {
            0 => Ok(()),
            1 => Err(Self::FailXvmCall),
            _ => panic!("encountered unknown status code"),
        }
    }
}

/// XVM default contract environment.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum XvmDefaultEnvironment {}

impl Environment for XvmDefaultEnvironment {
    const MAX_EVENT_TOPICS: usize = <DefaultEnvironment as Environment>::MAX_EVENT_TOPICS;

    type AccountId = <DefaultEnvironment as Environment>::AccountId;
    type Balance = <DefaultEnvironment as Environment>::Balance;
    type Hash = <DefaultEnvironment as Environment>::Hash;
    type BlockNumber = <DefaultEnvironment as Environment>::BlockNumber;
    type Timestamp = <DefaultEnvironment as Environment>::Timestamp;

    type ChainExtension = XvmExtension;
}

impl From<scale::Error> for XvmError {
    fn from(_: scale::Error) -> Self {
        panic!("encountered unexpected invalid SCALE encoding")
    }
}
