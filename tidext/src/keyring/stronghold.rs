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

#![cfg(feature = "keyring-stronghold")]
use crate::{Error, TidechainConfig};
use std::{fmt, path::Path, sync::Arc};
use stronghold::{Location, ProcResult, Procedure, ResultMessage, Stronghold};
use subxt::{
  sp_core::{
    crypto::{CryptoType, DeriveJunction, Pair, SecretStringError},
    sr25519::{Public, Signature},
  },
  PairSigner,
};

pub use iota_stronghold as stronghold;

/// Stronghold pair signer.
pub type TidefiPairSigner = PairSigner<TidechainConfig, StrongholdSigner>;
/// Tidefi keyring
pub type TidefiKeyring = TidextKeyring<TidechainConfig>;

/// Stronghold signer instance.
#[derive(Clone)]
pub struct StrongholdSigner {
  keypair_location: Location,
  stronghold: Arc<Stronghold>,
}

impl CryptoType for StrongholdSigner {
  type Pair = Self;
}

/// Stronghold typical cryptographic PKI key pair type
impl Pair for StrongholdSigner {
  type Public = Public;
  type Seed = Vec<u8>;
  type Signature = Signature;
  type DeriveError = String;

  /// Sign message
  fn sign(&self, message: &[u8]) -> Self::Signature {
    let stronghold = self.stronghold.clone();
    let message = message.to_vec();
    let keypair_location = self.keypair_location.clone();
    let sig = match futures::executor::block_on(stronghold.runtime_exec(Procedure::Sr25519Sign {
      keypair: keypair_location,
      msg: message,
    })) {
      ProcResult::Sr25519Sign(ResultMessage::Ok(sig)) => sig,
      r => panic!("unexpected result: {:?}", r),
    };
    sig.inner().clone()
  }

  /// Get public key pair
  fn public(&self) -> Self::Public {
    let stronghold = self.stronghold.clone();
    let keypair_location = self.keypair_location.clone();
    let pk =
      match futures::executor::block_on(stronghold.runtime_exec(Procedure::Sr25519PublicKey {
        keypair: keypair_location,
      })) {
        ProcResult::Sr25519PublicKey(ResultMessage::Ok(pk)) => pk,
        r => panic!("unexpected result: {:?}", r),
      };
    *pk.inner()
  }

  // These functions are not used by the the signer, this is why they are `unimplemented`.
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
  fn to_raw_vec(&self) -> Vec<u8> {
    unimplemented!()
  }
  fn verify<M: AsRef<[u8]>>(_sig: &Self::Signature, _message: M, _pubkey: &Self::Public) -> bool {
    unimplemented!()
  }
  fn verify_weak<P: AsRef<[u8]>, M: AsRef<[u8]>>(_sig: &[u8], _message: M, _pubkey: P) -> bool {
    unimplemented!()
  }
}

/// Tidefi keyring backed with a stronghold pair signer.
#[derive(Clone)]
pub struct TidextKeyring<T>
where
  T: subxt::Config,
  T::AccountId: From<[u8; 32]>,
{
  pub(super) account_id: T::AccountId,
  pub(super) pair_signer: Arc<TidefiPairSigner>,
}

impl<T> fmt::Debug for TidextKeyring<T>
where
  T: subxt::Config,
  T::AccountId: From<[u8; 32]>,
{
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("TidextKeyring").finish()
  }
}

impl<T> TidextKeyring<T>
where
  T: subxt::Config,
  T::AccountId: From<[u8; 32]>,
{
  /// Create new TidextKeyring from `AccountId` and `Stronghold`, with keypair stored on the given `Location`.
  pub fn new(
    account_id: T::AccountId,
    stronghold: Stronghold,
    keypair_location: Location,
  ) -> TidextKeyring<T> {
    TidextKeyring {
      account_id,
      pair_signer: Arc::new(TidefiPairSigner::new(StrongholdSigner {
        keypair_location,
        stronghold: Arc::new(stronghold),
      })),
    }
  }
  /// Try to get signer from existing stronghold instance
  pub async fn try_from_stronghold_instance(
    stronghold: Stronghold,
    keypair_location: Option<Location>,
  ) -> Result<Self, Error> {
    let default_keypair_location = keypair_location.unwrap_or(Location::Generic {
      vault_path: b"tidext".to_vec(),
      record_path: b"default".to_vec(),
    });

    get_signer_from_stronghold(stronghold, &default_keypair_location).await
  }

  /// Try to launch a new stronghold instance with the provided path and location
  pub async fn try_from_stronghold_path<P: AsRef<Path>, V: AsRef<Vec<u8>>>(
    stronghold_path: P,
    keypair_location: Option<Location>,
    passphrase: Option<V>,
  ) -> Result<Self, Error> {
    let default_keypair_location = keypair_location.unwrap_or(Location::Generic {
      vault_path: b"tidext".to_vec(),
      record_path: b"default".to_vec(),
    });

    get_signer_from_stronghold(
      init_stronghold_from_path(stronghold_path, passphrase).await?,
      &default_keypair_location,
    )
    .await
  }

  /// Try to launch a new stronghold instance with the provided seed and location
  pub async fn try_from_seed(
    seed: String,
    keypair_location: Option<Location>,
  ) -> Result<Self, Error> {
    let default_keypair_location = keypair_location.unwrap_or(Location::Generic {
      vault_path: b"tidext".to_vec(),
      record_path: b"default".to_vec(),
    });

    get_signer_from_stronghold(
      init_stronghold_from_seed(&default_keypair_location, Some(seed), None).await?,
      &default_keypair_location,
    )
    .await
  }
}

/// Initialize a new stronghold instance from the `sr25519` mnemonic or raw seed
pub async fn init_stronghold_from_seed(
  keypair_location: &Location,
  mnemonic_or_seed: Option<String>,
  seed_passphrase: Option<String>,
) -> Result<Stronghold, Error> {
  let (tx, rx) = std::sync::mpsc::channel();
  std::thread::spawn(move || {
    let system = actix::System::new();
    let stronghold = system
      .block_on(Stronghold::init_stronghold_system(vec![], vec![]))
      .unwrap();
    tx.send(stronghold).unwrap();
    system.run().expect("actix system run failed");
  });
  let stronghold = rx.recv().unwrap();

  if let ProcResult::Sr25519Generate(ResultMessage::Error(stronghold_error)) = stronghold
    .runtime_exec(Procedure::Sr25519Generate {
      mnemonic_or_seed,
      passphrase: seed_passphrase,
      output: keypair_location.clone(),
      hint: [0u8; 24].into(),
    })
    .await
  {
    Err(Error::Stronghold(stronghold_error))
  } else {
    Ok(stronghold)
  }
}

/// Initialize a new stronghold instance from the provided snapshot path and passphrase
pub async fn init_stronghold_from_path<P: AsRef<Path>, T: AsRef<Vec<u8>>>(
  stronghold_path: P,
  passphrase: Option<T>,
) -> Result<Stronghold, Error> {
  let (tx, rx) = std::sync::mpsc::channel();
  std::thread::spawn(move || {
    let system = actix::System::new();
    let stronghold = system
      .block_on(Stronghold::init_stronghold_system(vec![], vec![]))
      .unwrap();
    tx.send((stronghold, actix::System::current())).unwrap();
    system.run().expect("actix system run failed");
  });
  let (mut stronghold, system) = rx.recv().unwrap();

  let stronghold_path = stronghold_path.as_ref();
  let encryption_key = passphrase.map(|s| s.as_ref().to_vec()).unwrap_or_default();

  if stronghold_path.exists() {
    if let ResultMessage::Error(stronghold_error) = stronghold
      .read_snapshot(
        Vec::new(),
        None,
        &encryption_key,
        None,
        Some(stronghold_path.to_path_buf()),
      )
      .await
    {
      system.stop();
      return Err(Error::Stronghold(stronghold_error));
    };
  } else {
    return Err(Error::Stronghold("Invalid snapshot path".to_string()));
  }

  Ok(stronghold)
}

/// Try to get signer details for an existing stronghold instance at the specific location
pub async fn get_signer_from_stronghold<T>(
  stronghold: Stronghold,
  keypair_location: &Location,
) -> Result<TidextKeyring<T>, Error>
where
  T: subxt::Config,
  T::AccountId: From<[u8; 32]>,
{
  match stronghold
    .runtime_exec(Procedure::Sr25519PublicKey {
      keypair: keypair_location.clone(),
    })
    .await
  {
    ProcResult::Sr25519PublicKey(ResultMessage::Ok(pk)) => Ok(TidextKeyring::new(
      T::AccountId::from(pk.inner().0),
      stronghold,
      keypair_location.clone(),
    )),
    _ => Err(Error::Stronghold("Invalid public key".into())),
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::Signer;
  use std::str::FromStr;
  use tidefi_primitives::AccountId;

  #[tokio::test]
  async fn test_get_pair() {
    let mnemonic = "plug math bacon find roast scrap shrug exchange announce october exclude plate";
    let mnemonic_pair = TidefiKeyring::try_from_seed(mnemonic.into(), None)
      .await
      .expect("Unable to intialize pair signer");
    let seed = "0x9abdf3e8edda03c1708bcd5bc3353e91efd503fd9105ff0ee68a7cbc66b740d8";
    let seed_pair = TidefiKeyring::try_from_seed(seed.into(), None)
      .await
      .expect("Unable to intialize pair signer");
    assert_eq!(mnemonic_pair.account_id(), seed_pair.account_id())
  }

  #[test]
  fn test_get_accountid_from_string() {
    let alice = "BauKu2iL4fncgfy22YSLGc1aDLpyuUUe5z8yNF2pDtLNr4E";
    let bob = "A1k3praCLftTgBTb6aVavh3UNKwXN599Fqov17MkEy6bwCU";
    let alice_id = AccountId::from_str(alice);
    let bob_id = AccountId::from_str(bob);
    assert_ne!(alice_id.unwrap(), bob_id.unwrap());
  }
}
