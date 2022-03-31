// Copyright 2021-2022 Semantic Network Ltd.
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

#![cfg(not(feature = "keyring-stronghold"))]

use crate::{keyring::AccountId, TidechainConfig};
pub use sp_keyring::AccountKeyring;
use std::fmt;
use subxt::{sp_core::sr25519::Pair, DefaultExtra, PairSigner};
/// Pair of SR25519 keys for development.
pub type TidefiPairSigner = PairSigner<TidechainConfig, DefaultExtra<TidechainConfig>, Pair>;

#[derive(Clone)]
pub enum TidefiKeyring {
  AccountKeyring(AccountKeyring),
  Custom(usize),
}

impl fmt::Debug for TidefiKeyring {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("TidefiKeyring").finish()
  }
}

impl TidefiKeyring {
  pub fn account_id(&self) -> AccountId {
    match self {
      Self::AccountKeyring(keyrig) => keyrig.to_account_id(),
      Self::Custom(idx) => AccountKeyring::numeric_id(*idx),
    }
  }

  pub fn pair_signer(&self) -> TidefiPairSigner {
    PairSigner::new(match self {
      Self::AccountKeyring(keyrig) => keyrig.pair(),
      Self::Custom(idx) => AccountKeyring::numeric(*idx),
    })
  }
}
