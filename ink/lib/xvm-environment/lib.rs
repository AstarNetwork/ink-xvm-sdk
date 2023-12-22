//! The XVM public interface for Ink! smart contracts.
#![cfg_attr(not(feature = "std"), no_std, no_main)]
use ink::{
    env::{
        chain_extension::FromStatusCode,
        DefaultEnvironment,
        Environment,
    },
    prelude::vec::Vec,
};


/// The XVM chain extension adapter.
#[ink::chain_extension]
pub trait XvmExtension {
    type ErrorCode = XvmError;

    #[ink(extension = 0x00010001, handle_status = false)]
    fn xvm_call(vm_id: u8, target: Vec<u8>, input: Vec<u8>, value: u128) -> Result<Vec<u8>, XvmError>;
}

/// XVM chain extension errors.
#[derive(scale::Encode, scale::Decode, Debug)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum XvmError {
    InvalidVmId,
    SameVmCallNotAllowed,
    InvalidTarget,
    InputTooLarge,
    BadOrigin,
    ExecutionFailed,
    UnknownStatusCode,
}

impl FromStatusCode for XvmError {
    fn from_status_code(status_code: u32) -> core::result::Result<(), Self> {
        match status_code {
            0 => Ok(()),
            1 => Err(Self::InvalidVmId),
            2 => Err(Self::SameVmCallNotAllowed),
            3 => Err(Self::InvalidTarget),
            4 => Err(Self::InputTooLarge),
            5 => Err(Self::BadOrigin),
            6 => Err(Self::ExecutionFailed),
            _ => Err(Self::UnknownStatusCode)
        }
    }
}

impl From<scale::Error> for XvmError {
    fn from(_: scale::Error) -> Self {
        Self::UnknownStatusCode
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