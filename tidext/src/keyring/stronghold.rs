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

#![cfg(feature = "keyring-stronghold")]
use crate::{Error, TidechainConfig};
use std::{fmt, path::Path, sync::Arc};
use stronghold::{
  procedures::{BIP39Recover, GenerateKey, KeyType, PublicKey, Sr25519Sign},
  KeyProvider, Location, SnapshotPath, Stronghold,
};
use subxt::{
  ext::sp_core::{
    crypto::{CryptoType, DeriveJunction, Pair, SecretStringError},
    sr25519::{Public, Signature},
  },
  tx::PairSigner,
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
  client_path: Vec<u8>,
  is_ephemeral: bool,
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

    let client = if self.is_ephemeral {
      stronghold
        .get_client(self.client_path.clone())
        .expect("Failed to load client from stronghold")
    } else {
      stronghold
        .load_client(self.client_path.clone())
        .expect("Failed to load client from stronghold")
    };

    let proc = Sr25519Sign {
      msg: message,
      private_key: keypair_location,
    };

    let sig = match client.execute_procedure(proc) {
      Ok(sig) => sig,
      e => panic!("Failed to sign message: {:?}", e),
    };

    Signature::from_raw(sig)
  }

  /// Get public key pair
  fn public(&self) -> Self::Public {
    let stronghold = self.stronghold.clone();
    let keypair_location = self.keypair_location.clone();

    let client = if self.is_ephemeral {
      stronghold
        .get_client(self.client_path.clone())
        .expect("Failed to load client from stronghold")
    } else {
      stronghold
        .load_client(self.client_path.clone())
        .expect("Failed to load client from stronghold")
    };

    let proc = PublicKey {
      ty: KeyType::Sr25519,
      private_key: keypair_location,
    };

    let res = match client.execute_procedure(proc) {
      Ok(pk) => pk,
      e => panic!("Failed to get public key: {:?}", e),
    };

    let mut pk = [0u8; 32];
    pk.copy_from_slice(&res);

    Public::from_raw(pk)
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
    client_path: Vec<u8>,
    keypair_location: Location,
    is_ephemeral: bool,
  ) -> TidextKeyring<T> {
    TidextKeyring {
      account_id,
      pair_signer: Arc::new(TidefiPairSigner::new(StrongholdSigner {
        keypair_location,
        stronghold: Arc::new(stronghold),
        client_path,
        is_ephemeral,
      })),
    }
  }
  /// Try to get signer from existing stronghold instance
  pub fn try_from_stronghold_instance(
    client_path: Vec<u8>,
    stronghold: Stronghold,
    keypair_location: Option<Location>,
  ) -> Result<Self, Error> {
    let default_keypair_location = keypair_location.unwrap_or(Location::Generic {
      vault_path: b"tidext".to_vec(),
      record_path: b"default".to_vec(),
    });

    get_signer_from_stronghold(client_path, stronghold, &default_keypair_location)
  }

  /// Try to launch a new stronghold instance with the provided path and location
  pub fn try_from_stronghold_path<P: AsRef<Path>, V: AsRef<Vec<u8>>>(
    client_path: Vec<u8>,
    stronghold_path: P,
    keypair_location: Option<Location>,
    passphrase: Option<V>,
  ) -> Result<Self, Error> {
    let default_keypair_location = keypair_location.unwrap_or(Location::Generic {
      vault_path: b"tidext".to_vec(),
      record_path: b"default".to_vec(),
    });

    let stronghold = init_stronghold_from_path(client_path.clone(), stronghold_path, passphrase)?;
    get_signer_from_stronghold(client_path, stronghold, &default_keypair_location)
  }

  /// Try to launch a new stronghold instance with the provided seed and location
  pub fn try_from_seed(
    client_path: Vec<u8>,
    seed: String,
    keypair_location: Option<Location>,
  ) -> Result<Self, Error> {
    let default_keypair_location = keypair_location.unwrap_or(Location::Generic {
      vault_path: b"tidext".to_vec(),
      record_path: b"default".to_vec(),
    });

    let stronghold = init_stronghold_from_seed(
      client_path.clone(),
      &default_keypair_location,
      Some(seed),
      None,
    )?;
    get_signer_from_stronghold(client_path, stronghold, &default_keypair_location)
  }

  pub fn try_with_ephemeral_stronghold(
    stronghold: Stronghold,
    client_path: Vec<u8>,
    keypair_location: Option<Location>,
  ) -> Result<Self, Error> {
    let default_keypair_location = keypair_location.unwrap_or(Location::Generic {
      vault_path: b"tidext".to_vec(),
      record_path: b"default".to_vec(),
    });

    let (stronghold, buff) =
      init_ephemeral_stronghold(stronghold, &client_path, &default_keypair_location)?;

    let mut account_id = [0u8; 32];
    account_id.copy_from_slice(&buff);

    Ok(Self::new(
      account_id.into(),
      stronghold,
      client_path,
      default_keypair_location,
      true,
    ))
  }
}

/// Initialize a new stronghold instance from the `sr25519` mnemonic or raw seed
pub fn init_stronghold_from_seed(
  client_path: Vec<u8>,
  keypair_location: &Location,
  mnemonic_or_seed: Option<String>,
  seed_passphrase: Option<String>,
) -> Result<Stronghold, Error> {
  let stronghold = Stronghold::default();

  let client = stronghold.create_client(client_path.clone())?;

  let proc = BIP39Recover {
    passphrase: seed_passphrase,
    mnemonic: mnemonic_or_seed,
    ty: KeyType::Sr25519,
    output: keypair_location.clone(),
  };

  client.execute_procedure(proc)?;
  stronghold.write_client(client_path)?;

  Ok(stronghold)
}

// TODO: use `commit` and store keyprovider in snapshot state.

/// Initialize a new stronghold instance from the provided snapshot path and passphrase
pub fn init_stronghold_from_path<P: AsRef<Path>, T: AsRef<Vec<u8>>>(
  client_path: Vec<u8>,
  stronghold_path: P,
  passphrase: Option<T>,
) -> Result<Stronghold, Error> {
  let stronghold = Stronghold::default();

  let encryption_key = passphrase.map(|s| s.as_ref().to_vec()).unwrap_or_default();
  // let key = hash_blake2b(encryption_key);
  let keyprovider = KeyProvider::with_passphrase_truncated(encryption_key)
    .map_err(|e| Error::Stronghold(format!("Failed to derive key from passphrase {:?}", e)))?;

  if stronghold_path.as_ref().exists() {
    let snapshot_path = SnapshotPath::from_path(stronghold_path);

    stronghold.load_client_from_snapshot(client_path, &keyprovider, &snapshot_path)?;
  } else {
    return Err(Error::Stronghold("Invalid snapshot path".to_string()));
  }

  Ok(stronghold)
}

/// Try to get signer details for an existing stronghold instance at the specific location
pub fn get_signer_from_stronghold<T>(
  client_path: Vec<u8>,
  stronghold: Stronghold,
  keypair_location: &Location,
) -> Result<TidextKeyring<T>, Error>
where
  T: subxt::Config,
  T::AccountId: From<[u8; 32]>,
{
  let client = stronghold.load_client(client_path.clone())?;

  let proc = PublicKey {
    ty: KeyType::Sr25519,
    private_key: keypair_location.clone(),
  };

  match client.execute_procedure(proc) {
    Ok(pk) => {
      let mut key = [0u8; 32];
      key.copy_from_slice(&pk);

      Ok(TidextKeyring::new(
        T::AccountId::from(key),
        stronghold,
        client_path,
        keypair_location.clone(),
        false,
      ))
    }
    _ => Err(Error::Stronghold("Invalid public Key".into())),
  }
}

/// Initialize the ephemeral stronghold client with a seed.
pub fn init_ephemeral_stronghold(
  stronghold: Stronghold,
  client_path: &Vec<u8>,
  keypair_location: &Location,
) -> Result<(Stronghold, Vec<u8>), Error> {
  let client = stronghold.create_client(client_path)?;
  let proc0 = GenerateKey {
    ty: KeyType::Sr25519,
    output: keypair_location.clone(),
  };

  let proc1 = PublicKey {
    ty: KeyType::Sr25519,
    private_key: keypair_location.clone(),
  };

  client.execute_procedure(proc0)?;

  let res = client.execute_procedure(proc1)?;

  Ok((stronghold, res))
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::Signer;
  use std::str::FromStr;
  use tidefi_primitives::AccountId;

  #[tokio::test]
  async fn test_get_pair() {
    let client_path = b"client_path".to_vec();
    let mnemonic = "plug math bacon find roast scrap shrug exchange announce october exclude plate";
    let mnemonic_pair = TidefiKeyring::try_from_seed(client_path.clone(), mnemonic.into(), None)
      .expect("Unable to intialize pair signer");
    let seed = "0x9abdf3e8edda03c1708bcd5bc3353e91efd503fd9105ff0ee68a7cbc66b740d8";
    let seed_pair = TidefiKeyring::try_from_seed(client_path, seed.into(), None)
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
