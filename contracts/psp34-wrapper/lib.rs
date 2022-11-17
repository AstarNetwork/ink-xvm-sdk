#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod psp34_wrapper {
    use ink_prelude::vec::Vec;
    use ink_storage::traits::SpreadAllocate;
    use openbrush::{
        contracts::psp34::extensions::metadata::*,
        traits::{
            Storage,
            String,
        },
    };
    use openbrush::contracts::psp34::{PSP34Error, PSP34Ref};
    use xvm_sdk_psp34_controller::PSP34ControllerRef;
    use ethabi::ethereum_types::U256;
    use openbrush::contracts::psp34::Id;
    use ink_lang::ToAccountId;
    use xvm_helper::XvmErc721;

    #[derive(Default, SpreadAllocate, Storage)]
    #[ink(storage)]
    pub struct PSP34Wrapper {
        #[storage_field]
        psp34: psp34::Data,
        #[storage_field]
        metadata: metadata::Data,
        evm_address: [u8; 20],
        psp34_controller: AccountId,
    }

    impl PSP34 for PSP34Wrapper {}

    impl PSP34Metadata for PSP34Wrapper {}

    impl PSP34Wrapper {
        #[ink(constructor)]
        pub fn new(version: u32, psp34_controller_hash: Hash, evm_contract_address: [u8; 20], id: Id, name: String, symbol: String) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                let name_key: Vec<u8> = String::from("name");
                let symbol_key: Vec<u8> = String::from("symbol");
                instance._set_attribute(id.clone(), name_key, name);
                instance._set_attribute(id, symbol_key, symbol);
                instance.evm_address = evm_contract_address;
                let salt = version.to_le_bytes();
                let psp34 = PSP34ControllerRef::new(evm_contract_address.into())
                    .endowment(0)
                    .code_hash(psp34_controller_hash)
                    .salt_bytes(salt)
                    .instantiate()
                    .unwrap_or_else(|error| {
                        panic!("failed at instantiating the psp34 controller contract: {:?}", error)
                    });
                instance.psp34_controller = psp34.to_account_id();
            })
        }

        #[ink(message)]
        pub fn deposit(&mut self, id: Id) -> Result<(), PSP34Error> {
            let caller = self.env().caller();
            let contract = self.env().account_id();
            XvmErc721::transfer_from(self.evm_address, caller, contract, cast(id.clone()))
                .map_err(|_| PSP34Error::Custom(String::from("transfer failed")))?;
            self._mint_to(caller, id)
        }

        #[ink(message)]
        pub fn withdraw(&mut self, id: Id) -> Result<(), PSP34Error> {
            let caller = self.env().caller();
            self._burn_from(caller, id.clone())?;
            PSP34Ref::transfer(&mut self.psp34_controller, caller, id, Vec::new())
        }
    }

    fn cast(id: Id) -> U256 {
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