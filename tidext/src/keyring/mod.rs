// Copyright 2021-2023 Semantic Network Ltd.
// This file is part of tidext.

// tidext is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// tidext is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with tidext.  If not, see <http://www.gnu.org/licenses/>.

//! Tidext official keyring.
use crate::Signer;
use subxt::ext::sp_core::sr25519;

#[cfg_attr(feature = "keyring-stronghold", path = "stronghold.rs")]
#[cfg_attr(not(feature = "keyring-stronghold"), path = "dev.rs")]
mod keyring_impl;
pub use keyring_impl::*;

/// Tidechain key-pair representation.
pub type KeyPair = sr25519::Pair;

/// Signing transactions requires a Signer.
/// This is responsible for providing the "from" account that the transaction is being signed by,
/// as well as actually signing a SCALE encoded payload. Optionally, a signer can also provide the
/// nonce for the transaction to use.
impl<T> Signer<T> for TidextKeyring<T>
where
  T: subxt::Config,
  T::AccountId: From<[u8; 32]>,
  T::Index: From<tidefi_primitives::Index>,
  T::AccountId: From<tidefi_primitives::AccountId>,
  T::Address: From<sp_runtime::MultiAddress<sp_runtime::AccountId32, u32>>,
  T::Signature: From<sp_runtime::MultiSignature>,
{
  /// Optionally returns a nonce.
  fn nonce(&self) -> Option<T::Index> {
    self.pair_signer.nonce().map(|nonce| nonce.into())
  }

  /// Return the account ID.
  fn account_id(&self) -> &T::AccountId {
    &self.account_id
  }

  /// Return the "from" address.
  fn address(&self) -> T::Address {
    self.pair_signer.address().into()
  }

  /// Takes a signer payload for an extrinsic, and returns a signature based on it.
  fn sign(&self, signer_payload: &[u8]) -> T::Signature {
    self.pair_signer.sign(signer_payload).into()
  }
}
