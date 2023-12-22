#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(PSP34, PSP34Metadata)]
#[openbrush::contract]
pub mod psp34_wrapper {
    use ethabi::ethereum_types::U256;
    use ink::prelude::borrow::ToOwned;
    use openbrush::traits::Storage;
    use xvm_helper::XvmErc721;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct PSP34Wrapper {
        #[storage_field]
        psp34: psp34::Data,
        #[storage_field]
        metadata: metadata::Data,
        evm_address: [u8; 20],
    }

    impl PSP34Wrapper {
        #[ink(constructor)]
        pub fn new(
            evm_contract_address: [u8; 20],
            id: Id,
            name: String,
            symbol: String,
        ) -> Self {
            let mut instance = Self::default();
            metadata::Internal::_set_attribute(&mut instance, id.clone(), "name".to_owned(), name);
            metadata::Internal::_set_attribute(&mut instance, id, "symbol".to_owned(), symbol);
            instance.evm_address = evm_contract_address;
            instance
        }

        #[ink(message)]
        pub fn deposit(&mut self, id: Id) -> Result<(), PSP34Error> {
            let caller = self.env().caller();
            let contract = self.env().account_id();
            XvmErc721::transfer_from(self.evm_address, caller, contract, cast(id.clone()))
                .map_err(|_| PSP34Error::Custom("transfer failed".to_owned()))?;
            psp34::Internal::_mint_to(self, caller, id)
        }

        #[ink(message)]
        pub fn withdraw(&mut self, id: Id) -> Result<(), PSP34Error> {
            let caller = self.env().caller();
            let contract = self.env().account_id();
            psp34::Internal::_burn_from(self, caller, id.clone())?;
            XvmErc721::transfer_from(self.evm_address, contract, caller, cast(id.clone()))
                .map_err(|_| PSP34Error::Custom("transfer failed".to_owned()))
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
