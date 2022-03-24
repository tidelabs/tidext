use subxt::sp_core::{crypto::AccountId32, sr25519};

#[cfg_attr(feature = "keyring-stronghold", path = "stronghold.rs")]
#[cfg_attr(feature = "keyring-stronghold_ex", path = "stronghold_ex.rs")]
#[cfg_attr(
  not(any(feature = "keyring-stronghold", feature = "keyring-stronghold_ex")),
  path = "dev.rs"
)]
mod keyring_impl;

pub use self::keyring_impl::*;

/// `TiDeFi` Account ID representation
pub type AccountId = AccountId32;

/// `TiDeFi` Key-pair representation
pub type KeyPair = sr25519::Pair;
