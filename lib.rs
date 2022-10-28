//! Sample contract that uses XVM SDK for EVM interoperability.
#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod erc20_sample {
    use xvm_sdk_erc20::Erc20Ref;

    #[ink(storage)]
    pub struct Erc20Sample {
        erc20: Erc20Ref,
    }

    impl Erc20Sample {
        #[ink(constructor)]
        pub fn new(version: u32, erc20_code_hash: Hash, evm_contract_address: [u8; 20]) -> Self {
            let salt = version.to_le_bytes();
            let erc20 = Erc20Ref::new(evm_contract_address.into())
                .endowment(0)
                .code_hash(erc20_code_hash)
                .salt_bytes(salt)
                .instantiate()
                .unwrap_or_else(|error| {
                    panic!("failed at instantiating the erc20 contract: {:?}", error)
                });
            Self { erc20 }
        }

        #[ink(message)]
        pub fn claim(&mut self) -> bool {
            let to = [0xffu8; 20];
            let value = 424242u128;
            self.erc20.transfer(to, value)
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;
    }
}
