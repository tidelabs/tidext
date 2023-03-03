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

use crate::{Error, TidechainConfig};
#[cfg(feature = "keyring-dev")]
pub use sp_keyring::AccountKeyring;
use std::{fmt, sync::Arc};
#[cfg(feature = "keyring-dev")]
use subxt::ext::sp_core::crypto::Pair as TraitPair;
use subxt::{ext::sp_core::sr25519::Pair, tx::PairSigner};

/// Pair of SR25519 keys for development.
pub type TidefiPairSigner = PairSigner<TidechainConfig, Pair>;
/// Tidefi keyring
pub type TidefiKeyring = TidextKeyring<TidechainConfig>;

#[derive(Clone)]
pub enum TidextAccountKeyring {
  #[cfg(feature = "keyring-dev")]
  AccountKeyring(AccountKeyring),
  #[cfg(feature = "keyring-dev")]
  Custom(usize),
}

/// Tidefi keyring backed with a stronghold pair signer.
#[derive(Clone)]
pub struct TidextKeyring<T>
where
  T: subxt::Config,
{
  pub(super) account_id: T::AccountId,
  pub pair_signer: Arc<TidefiPairSigner>,
}

impl<T> fmt::Debug for TidextKeyring<T>
where
  T: subxt::Config,
{
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("TidextKeyring").finish()
  }
}

#[cfg(feature = "keyring-dev")]
impl<T> From<usize> for TidextKeyring<T>
where
  T: subxt::Config,
  T::AccountId: From<tidefi_primitives::AccountId>,
{
  fn from(custom_signer: usize) -> TidextKeyring<T> {
    TidextKeyring::new(TidextAccountKeyring::Custom(custom_signer))
  }
}

#[cfg(feature = "keyring-dev")]
impl<T> From<AccountKeyring> for TidextKeyring<T>
where
  T: subxt::Config,
  T::AccountId: From<tidefi_primitives::AccountId>,
{
  fn from(custom_signer: AccountKeyring) -> TidextKeyring<T> {
    TidextKeyring::new(TidextAccountKeyring::AccountKeyring(custom_signer))
  }
}

impl<T> TidextKeyring<T>
where
  T: subxt::Config,
  T::AccountId: From<tidefi_primitives::AccountId>,
{
  #[cfg(feature = "keyring-dev")]
  pub fn new(signer: TidextAccountKeyring) -> Self {
    let account_id: T::AccountId = match signer {
      TidextAccountKeyring::AccountKeyring(keyrig) => keyrig.to_account_id().into(),
      TidextAccountKeyring::Custom(idx) => AccountKeyring::numeric_id(idx).into(),
    };

    Self {
      account_id: account_id.into(),
      pair_signer: Arc::new(TidefiPairSigner::new(match signer {
        TidextAccountKeyring::AccountKeyring(keyrig) => keyrig.pair(),
        TidextAccountKeyring::Custom(idx) => AccountKeyring::numeric(idx),
      })),
    }
  }

  #[cfg(not(feature = "keyring-dev"))]
  pub fn new(_signer: TidextAccountKeyring) -> Self {
    unimplemented!("Add `keyring-dev` feature to use development keyring");
  }

  #[cfg(feature = "keyring-dev")]
  pub async fn try_from_seed(seed: String) -> Result<Self, Error> {
    let (pair_signer, _) = Pair::from_string_with_seed(seed.as_str(), None)
      .map_err(|_| Error::Other("Invalid seed".into()))?;

    Ok(Self {
      account_id: T::AccountId::from(pair_signer.public().into()),
      pair_signer: Arc::new(TidefiPairSigner::new(pair_signer)),
    })
  }

  #[cfg(not(feature = "keyring-dev"))]
  pub async fn try_from_seed(_seed: String) -> Result<Self, Error> {
    unimplemented!("Add `keyring-dev` feature to use development keyring");
  }
}
