#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod psp34_wrapper {
    use ethabi::ethereum_types::U256;
    use ink::{
        prelude::{
            format,
            vec::Vec,
        },
        ToAccountId,
    };
    use openbrush::{
        contracts::psp34::{
            extensions::metadata::*,
            Id,
            PSP34Error,
            PSP34Ref,
        },
        traits::{
            Storage,
            String,
        },
    };
    use xvm_helper::XvmErc721;
    use xvm_sdk_psp34_controller::PSP34ControllerRef;

    #[ink(storage)]
    #[derive(Storage)]
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
        pub fn new(
            version: u32,
            psp34_controller_hash: Hash,
            evm_contract_address: [u8; 20],
            id: Id,
            name: String,
            symbol: String,
        ) -> Result<Self, PSP34Error> {
            let salt = version.to_le_bytes();
            let psp34 = PSP34ControllerRef::new(evm_contract_address.into())
                .endowment(0)
                .code_hash(psp34_controller_hash)
                .salt_bytes(salt)
                .try_instantiate()
                .map_err(|error| {
                    PSP34Error::Custom(
                        format!("Failed to Instantiate: {:?}", error)
                            .as_bytes()
                            .to_vec(),
                    )
                })?
                .map_err(|_| PSP34Error::Custom(Vec::<u8>::from("Failed to Instantiate")))?;

            let mut instance = Self {
                psp34: Default::default(),
                metadata: Default::default(),
                evm_address: evm_contract_address,
                psp34_controller: psp34.to_account_id(),
            };
            instance._set_attribute(id.clone(), String::from("name"), name);
            instance._set_attribute(id, String::from("symbol"), symbol);
            Ok(instance)
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
