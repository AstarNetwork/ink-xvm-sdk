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
        contracts::traits::psp22::PSP22Ref,
    };
    use xvm_sdk_helper::*;
    use xvm_sdk_psp22_controller::Psp22Ref;
    use ink_lang::ToAccountId;

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct PSP22Wrapper {
        #[storage_field]
        psp22: psp22::Data,
        #[storage_field]
        metadata: metadata::Data,
        evm_address: [u8; 20],
        psp22_controller: AccountId,
    }

    impl PSP22 for PSP22Wrapper {}
    impl PSP22Metadata for PSP22Wrapper {}

    impl PSP22Wrapper {
        #[ink(constructor)]
        pub fn new(version: u32, psp22_controller_hash: Hash, evm_contract_address: [u8; 20]) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                instance.metadata.name = Some(String::from("Wrapped PSP22"));
                instance.metadata.symbol = Some(String::from("WPSP22"));
                instance.metadata.decimals = 18;
                instance.evm_address = evm_contract_address;
                let salt = version.to_le_bytes();
                let psp22 = Psp22Ref::new(evm_contract_address.into())
                    .endowment(0)
                    .code_hash(psp22_controller_hash)
                    .salt_bytes(salt)
                    .instantiate()
                    .unwrap_or_else(|error| {
                        panic!("failed at instantiating the psp22 controller contract: {:?}", error)
                    });
              instance.psp22_controller = psp22.to_account_id();
            })
        }

        #[ink(message)]
        pub fn get_psp22_controller(&self) -> AccountId {
            self.psp22_controller
        }

        #[ink(message)]
        pub fn deposit(&mut self, amount: Balance) -> Result<(), PSP22Error> {
            let caller = self.env().caller();
            let contract = self.env().account_id();
            XvmErc20::transfer(self.evm_address, contract, amount, Vec::new())
                .map_err(|_| PSP22Error::Custom(String::from("transfer failed")))?;
            self._mint(caller, amount)
        }

        #[ink(message)]
        pub fn withdraw(&mut self, amount: Balance) -> Result<(), PSP22Error> {
            let caller = self.env().caller();
            self._burn_from(caller, amount)?;
            PSP22Ref::transfer(&mut self.psp22_controller, caller, amount, Vec::new())?;
            Ok(())
        }
    }
}
