#![cfg(feature = "keyring-stronghold")]
use crate::{keyring::AccountId, TidechainConfig};
use std::{fmt, sync::Arc};
use subxt::{
  sp_core::{
    crypto::{CryptoType, DeriveJunction, Pair, SecretStringError},
    sr25519::{Public, Signature},
  },
  DefaultExtra, PairSigner,
};

pub use iota_stronghold as stronghold;
use stronghold::{Location, ProcResult, Procedure, ResultMessage, Stronghold};

/// Stronghold pair signer.
pub type TidefiPairSigner =
  PairSigner<TidechainConfig, DefaultExtra<TidechainConfig>, StrongholdSigner>;

#[derive(Clone)]
pub struct StrongholdSigner {
  keypair_location: Location,
  stronghold: Arc<Stronghold>,
}

impl CryptoType for StrongholdSigner {
  type Pair = Self;
}

// we implement `Pair` because `PairSigner` requires it.
// Only `public` and `sign` methods are needed, so the others are `unimplemented!()`.
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

  fn verify<M: AsRef<[u8]>>(_sig: &Self::Signature, _message: M, _pubkey: &Self::Public) -> bool {
    unimplemented!()
  }

  fn verify_weak<P: AsRef<[u8]>, M: AsRef<[u8]>>(_sig: &[u8], _message: M, _pubkey: P) -> bool {
    unimplemented!()
  }

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

  fn to_raw_vec(&self) -> Vec<u8> {
    unimplemented!()
  }
}

#[derive(Clone)]
pub struct TidefiKeyring {
  account_id: AccountId,
  keypair_location: Location,
  stronghold: Arc<Stronghold>,
}

impl fmt::Debug for TidefiKeyring {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("TidefiKeyring").finish()
  }
}

impl TidefiKeyring {
  /// Create new TidefiKeyring from `AccountId` and `Stronghold`, with keypair stored on the given `Location`.
  pub fn new(
    account_id: AccountId,
    stronghold: Stronghold,
    keypair_location: Location,
  ) -> TidefiKeyring {
    TidefiKeyring {
      account_id,
      stronghold: Arc::new(stronghold),
      keypair_location,
    }
  }

  /// Retrieve the `TidefiPairSigner`.
  pub fn pair_signer(&self) -> TidefiPairSigner {
    TidefiPairSigner::new(StrongholdSigner {
      keypair_location: self.keypair_location.clone(),
      stronghold: self.stronghold.clone(),
    })
  }

  /// Retrieve the `AccountId`.
  pub fn account_id(&self) -> AccountId {
    self.account_id.clone()
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use std::str::FromStr;

  async fn init_stronghold(
    mnemonic_or_seed: String,
    passphrase: Option<String>,
  ) -> (AccountId, Location, Stronghold) {
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
      let system = actix::System::new();
      let stronghold = system
        .block_on(Stronghold::init_stronghold_system(b"path".to_vec(), vec![]))
        .unwrap();
      tx.send(stronghold).unwrap();
      system.run().expect("actix system run failed");
    });
    let stronghold = rx.recv().unwrap();

    let keypair_location = Location::generic("SR25519", "keypair");

    match stronghold
      .runtime_exec(Procedure::Sr25519Generate {
        mnemonic_or_seed: Some(mnemonic_or_seed),
        passphrase,
        output: keypair_location.clone(),
        hint: [0u8; 24].into(),
      })
      .await
    {
      ProcResult::Sr25519Generate(ResultMessage::OK) => (),
      r => panic!("unexpected result: {:?}", r),
    }

    let pk = match stronghold
      .runtime_exec(Procedure::Sr25519PublicKey {
        keypair: keypair_location.clone(),
      })
      .await
    {
      ProcResult::Sr25519PublicKey(ResultMessage::Ok(pk)) => pk,
      r => panic!("unexpected result: {:?}", r),
    };

    (AccountId::from(pk.inner().0), keypair_location, stronghold)
  }

  #[tokio::test]
  async fn test_get_pair() {
    let mnemonic = "plug math bacon find roast scrap shrug exchange announce october exclude plate";
    let (account_id, location, stronghold) = init_stronghold(mnemonic.into(), None).await;
    let mnemonic_pair = TidefiKeyring::new(account_id, stronghold, location);
    let seed = "0x9abdf3e8edda03c1708bcd5bc3353e91efd503fd9105ff0ee68a7cbc66b740d8";
    let (account_id, location, stronghold2) = init_stronghold(seed.into(), None).await;
    let seed_pair = TidefiKeyring::new(account_id, stronghold2, location);
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
