//! Tidext official keyring.
use subxt::sp_core::sr25519;

#[cfg_attr(feature = "keyring-stronghold", path = "stronghold.rs")]
#[cfg_attr(feature = "keyring-stronghold_ex", path = "stronghold_ex.rs")]
#[cfg_attr(
  not(any(feature = "keyring-stronghold", feature = "keyring-stronghold_ex")),
  path = "dev.rs"
)]
mod keyring_impl;
pub use keyring_impl::*;

/// Tidechain key-pair representation.
pub type KeyPair = sr25519::Pair;
