// This file is part of Substrate.

// Copyright (C) 2017-2020 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! MultiAddress type is a wrapper for multiple downstream account formats.

use crate::traits::Member;
use codec::{Encode, Decode};
use sp_std::vec::Vec;

/// A multi-format address wrapper for on-chain accounts.
#[derive(Encode, Decode, PartialEq, Eq, Clone, crate::RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Hash))]
pub enum MultiAddress<AccountId, AccountIndex> where
	AccountId: Member,
	AccountIndex: Member,
{
	/// It's an account ID (pubkey).
	Id(AccountId),
	/// It's an account index.
	Index(#[codec(compact)] AccountIndex),
	/// It's some arbitrary raw bytes.
	Raw(Vec<u8>),
	/// It's a 32 byte representation.
	Account32([u8; 32]),
	/// Its a 20 byte representation.
	Account20([u8; 20]),
}

#[cfg(feature = "std")]
impl<AccountId, AccountIndex> std::fmt::Display for MultiAddress<AccountId, AccountIndex> where
	AccountId: Member,
	AccountIndex: Member,
{
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{:?}", self)
	}
}

impl<AccountId, AccountIndex> From<AccountId> for MultiAddress<AccountId, AccountIndex> where
	AccountId: Member,
	AccountIndex: Member,
{
	fn from(a: AccountId) -> Self {
		MultiAddress::Id(a)
	}
}

impl<AccountId, AccountIndex> Default for MultiAddress<AccountId, AccountIndex> where
	AccountId: Member + Default,
	AccountIndex: Member,
{
	fn default() -> Self {
		MultiAddress::Id(Default::default())
	}
}
