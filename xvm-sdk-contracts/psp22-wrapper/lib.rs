#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp22 {
    use ethabi::{
        ethereum_types::{
            H160,
            U256,
        },
        Token,
    };
    use hex_literal::hex;
    use ink_lang::ToAccountId;
    use ink_prelude::vec::Vec;
    use ink_storage::traits::SpreadAllocate;
    use openbrush::{
        contracts::{
            psp22::extensions::metadata::*,
            traits::psp22::PSP22Ref,
        },
        traits::Storage,
    };
    use xvm_sdk_helper::*;
    use xvm_sdk_psp22_controller::Psp22Ref;

    const EVM_ID: u8 = 0x0F;
    const TRANSFER_SELECTOR: [u8; 4] = hex!["a9059cbb"];

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
        pub fn new(
            version: u32,
            psp22_controller_hash: Hash,
            evm_contract_address: [u8; 20],
        ) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                instance.metadata.name = Some("Wrapped PSP22".as_bytes().to_vec());
                instance.metadata.symbol = Some("WPSP22".as_bytes().to_vec());
                instance.metadata.decimals = 18;
                instance.evm_address = evm_contract_address;
                let salt = version.to_le_bytes();
                let psp22 = Psp22Ref::new(evm_contract_address.into())
                    .endowment(0)
                    .code_hash(psp22_controller_hash)
                    .salt_bytes(salt)
                    .instantiate()
                    .unwrap_or_else(|error| {
                        panic!(
                            "failed at instantiating the psp22 controller contract: {:?}",
                            error
                        )
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
            Self::_transfer(self.evm_address, contract, amount, Vec::new())
                .map_err(|_| PSP22Error::Custom("transfer failed".as_bytes().to_vec()))?;
            self._mint_to(caller, amount)
        }

        #[ink(message)]
        pub fn withdraw(&mut self, amount: Balance) -> Result<(), PSP22Error> {
            let caller = self.env().caller();
            self._burn_from(caller, amount)?;
            PSP22Ref::transfer(&mut self.psp22_controller, caller, amount, Vec::new())
        }

        fn _transfer(
            evm_contract_address: [u8; 20],
            to: AccountId,
            value: Balance,
            _data: Vec<u8>,
        ) -> Result<(), XvmError> {
            let encoded_input = Self::_transfer_encode(Self::_h160(&to), value.into());
            Xvm::xvm_call(
                EVM_ID,
                Vec::from(evm_contract_address.as_ref()),
                encoded_input,
            )
        }

        fn _transfer_encode(to: H160, value: U256) -> Vec<u8> {
            let mut encoded = TRANSFER_SELECTOR.to_vec();
            let input = [Token::Address(to), Token::Uint(value)];
            encoded.extend(&ethabi::encode(&input));
            encoded
        }

        fn _h160(from: &AccountId) -> H160 {
            let mut dest: H160 = [0; 20].into();
            dest.as_bytes_mut()
                .copy_from_slice(&<ink_env::AccountId as AsRef<[u8]>>::as_ref(from)[..20]);
            dest
        }
    }
}
