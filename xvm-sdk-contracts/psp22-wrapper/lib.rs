#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp22 {
    use ink_prelude::{
        string::String,
        vec::Vec,
    };
    use ink_storage::traits::SpreadAllocate;
    use openbrush::{
        contracts::psp22::extensions::metadata::*,
        traits::Storage,
    };
    use xvm_sdk_helper::*;

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct PSP22Wrapper {
        #[storage_field]
        psp22: psp22::Data,
        #[storage_field]
        metadata: metadata::Data,
        evm_address: [u8; 20],
    }

    impl PSP22 for PSP22Wrapper {}
    impl PSP22Metadata for PSP22Wrapper {}

    impl PSP22Wrapper {
        #[ink(constructor)]
        pub fn new(evm_address: [u8; 20]) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                instance.metadata.name = Some(String::from("Wrapped PSP22"));
                instance.metadata.symbol = Some(String::from("WPSP22"));
                instance.metadata.decimals = 18;
                instance.evm_address = evm_address;
            })
        }

        #[ink(message)]
        pub fn deposit(&mut self, amount: Balance) -> Result<(), PSP22Error> {
            let caller = Self::env().caller();
            let contract = Self::env().account_id();
            XvmErc20::transfer_from(self.evm_address, caller, contract, amount, Vec::new())
                .map_err(|_| PSP22Error::Custom(String::from("transfer_from failed")))?;
            self._mint(caller, amount)
        }

        #[ink(message)]
        pub fn withdraw(&mut self, amount: Balance) -> Result<(), PSP22Error> {
            let caller = Self::env().caller();
            self._burn_from(caller, amount)?;
            XvmErc20::transfer(self.evm_address, caller, amount, Vec::new())
                .map_err(|_| PSP22Error::Custom(String::from("transfer_from failed")))
        }
    }
}
