// This file is part of Astar.

// Copyright (C) 2019-2023 Stake Technologies Pte.Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later

// Astar is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Astar is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Astar. If not, see <http://www.gnu.org/licenses/>.

#![cfg_attr(not(feature = "std"), no_std, no_main)]

use sp_core::H160;
pub use unified_accounts_chain_extension_types::{Command, UnifiedAddress};
use ink::primitives::AccountId;

/// UA Extension Interface
pub struct UAExtension;

impl UAExtension {
    pub const ID: u16 = 03;

    const fn get_func_id(idx: u16) -> u32 {
        ((Self::ID as u32) << 16) + (idx as u32)
    }

    pub fn to_h160(account_id: AccountId) -> Option<H160> {
        let func_id: u32 = Self::get_func_id(Command::GetEvmAddress.into());

        // fn(AccountId) -> Option<H160>
        ::ink::env::chain_extension::ChainExtensionMethod::build(func_id)
            .input::<AccountId>()
            .output::<Option<H160>, false>()
            .ignore_error_code()
            .call(&(account_id))
    }

    pub fn to_h160_or_default(account_id: AccountId) -> UnifiedAddress<H160> {
        let func_id: u32 = Self::get_func_id(Command::GetEvmAddressOrDefault.into());

        // fn(AccountId) -> H160
        ::ink::env::chain_extension::ChainExtensionMethod::build(func_id)
            .input::<AccountId>()
            .output::<UnifiedAddress<H160>, false>()
            .ignore_error_code()
            .call(&(account_id))
    }

    pub fn to_account_id(evm_address: H160) -> Option<AccountId> {
        let func_id: u32 = Self::get_func_id(Command::GetNativeAddress.into());

        // fn(H160) -> Option<AccountId>
        ::ink::env::chain_extension::ChainExtensionMethod::build(func_id)
            .input::<H160>()
            .output::<Option<AccountId>, false>()
            .ignore_error_code()
            .call(&(evm_address))
    }

    pub fn to_account_id_or_default(evm_address: H160) -> UnifiedAddress<AccountId> {
        let func_id: u32 = Self::get_func_id(Command::GetNativeAddressOrDefault.into());

        // fn(H160) -> AccountId
        ::ink::env::chain_extension::ChainExtensionMethod::build(func_id)
            .input::<H160>()
            .output::<UnifiedAddress<AccountId>, false>()
            .ignore_error_code()
            .call(&(evm_address))
    }
}