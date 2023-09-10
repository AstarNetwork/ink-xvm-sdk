#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(PSP22, PSP22Metadata)]
#[openbrush::contract]
pub mod my_psp22 {
    use ink::{
        prelude::{
            format,
        },
        ToAccountId,
    };
    use openbrush::traits::Storage;
    use xvm_helper::*;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct PSP22Wrapper {
        #[storage_field]
        psp22: psp22::Data,
        #[storage_field]
        metadata: metadata::Data,
        evm_address: [u8; 20],
    }

    impl PSP22Wrapper {
        #[ink(constructor)]
        pub fn new(
            evm_contract_address: [u8; 20],
        ) -> Self {
            let mut instance = Self::default();

            instance.metadata.name.set(&Some(String::from("Wrapped PSP22")));
            instance.metadata.symbol.set(&Some(String::from("WPSP22")));
            instance.metadata.decimals.set(&18);

            instance.evm_address = evm_contract_address;

            instance
        }

        #[ink(message)]
        pub fn deposit(&mut self, amount: Balance) -> Result<(), PSP22Error> {
            let caller = self.env().caller();
            let contract = self.env().account_id();
            XvmErc20::transfer_from(self.evm_address, contract, caller, amount, 0u128, Vec::new())
                .map_err(|_| PSP22Error::Custom(Stri"transfer failed".to_string()))?;
            self._mint_to(caller, amount)
        }

        #[ink(message)]
        pub fn withdraw(&mut self, amount: Balance) -> Result<(), PSP22Error> {
            let caller = self.env().caller();
            self._burn_from(caller, amount)?;
            XvmErc20::transfer(self.evm_address, caller, amount, 0u128, Vec::new())
                .map_err(|_| PSP22Error::Custom("transfer failed".as_bytes().to_vec()))
        }
    }
}
