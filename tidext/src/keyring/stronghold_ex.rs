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

use crate::{keyring::AccountId, TidechainConfig};
use std::fmt;
use subxt::{
  sp_core::{
    crypto::{CryptoType, DeriveJunction, Pair, SecretStringError},
    sr25519::{Public, Signature},
  },
  DefaultExtra, PairSigner,
};

use ex_client::{public_key_inner, sr25519_sign_inner, KeyStore, Location, SecureBucket};

pub type TidefiPairSigner =
  PairSigner<TidechainConfig, DefaultExtra<TidechainConfig>, StrongholdSigner>;

#[derive(Clone)]
pub struct StrongholdSigner {
  pair_location: Location,
  keystore: KeyStore,
  secure_bucket: SecureBucket,
}

impl CryptoType for StrongholdSigner {
  type Pair = Self;
}

impl Pair for StrongholdSigner {
  type Public = Public;
  type Seed = Vec<u8>;
  type Signature = Signature;
  type DeriveError = String;

  fn generate_with_phrase(_password: Option<&str>) -> (Self, String, Self::Seed) {
    unimplemented!()
  }

  fn from_phrase(
    _phrase: &str,
    _password: Option<&str>,
  ) -> Result<(Self, Self::Seed), SecretStringError> {
    unimplemented!()
  }

  fn derive<Iter: Iterator<Item = DeriveJunction>>(
    &self,
    _path: Iter,
    _seed: Option<Self::Seed>,
  ) -> Result<(Self, Option<Self::Seed>), Self::DeriveError> {
    unimplemented!()
  }

  fn from_seed(_seed: &Self::Seed) -> Self {
    unimplemented!()
  }

  fn from_seed_slice(_seed: &[u8]) -> Result<Self, SecretStringError> {
    unimplemented!()
  }

  fn sign(&self, message: &[u8]) -> Self::Signature {
    let msg = message.to_vec();

    let loc = self.pair_location.clone();

    sr25519_sign_inner(self.secure_bucket.clone(), self.keystore.clone(), msg, loc)
      .expect("failed to sign")
  }

  fn verify<M: AsRef<[u8]>>(_sig: &Self::Signature, _message: M, _pubkey: &Self::Public) -> bool {
    unimplemented!()
  }

  fn verify_weak<P: AsRef<[u8]>, M: AsRef<[u8]>>(_sig: &[u8], _message: M, _pubkey: P) -> bool {
    unimplemented!()
  }

  fn public(&self) -> Self::Public {
    let loc = self.pair_location.clone();
    let bytes = public_key_inner(self.secure_bucket.clone(), self.keystore.clone(), loc)
      .expect("failed to get public key");

    Public::from_raw(bytes)
  }

  fn to_raw_vec(&self) -> Vec<u8> {
    unimplemented!()
  }
}

#[derive(Clone)]
pub struct TidefiKeyring {
  account_id: AccountId,
  keypair_location: Location,
  keystore: KeyStore,
  secure_bucket: SecureBucket,
}

impl fmt::Debug for TidefiKeyring {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("TidefiKeyring").finish()
  }
}

impl TidefiKeyring {
  pub fn new(
    account_id: AccountId,
    keystore: KeyStore,
    secure_bucket: SecureBucket,
    keypair_location: Location,
  ) -> Self {
    Self {
      account_id,
      keystore,
      secure_bucket,
      keypair_location,
    }
  }

  pub fn pair_signer(&self) -> TidefiPairSigner {
    TidefiPairSigner::new(StrongholdSigner {
      pair_location: self.keypair_location.clone(),
      keystore: self.keystore.clone(),
      secure_bucket: self.secure_bucket.clone(),
    })
  }

  pub fn account_id(&self) -> AccountId {
    self.account_id.clone()
  }
}
