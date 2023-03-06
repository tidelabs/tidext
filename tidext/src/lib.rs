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

//! Tidext is the official [Tidechain](https://github.com/tidelabs/tidechain)
//! client used by Tidefi, Quorums and Oracle.
//!
//! Currently this client is geared toward [`subxt`] implementation.
//!
//! - **Signer** — [`Stronghold`] is aggressively feature gated so your
//!   client can use regular signer for testing purpose.
//!
//! [`Stronghold`]: keyring::stronghold::Stronghold
//!
//! # Stronghold signer
//!
//! Stronghold is a secure software implementation with the sole purpose of
//! isolating digital secrets from exposure to hackers and accidental leaks.
//! It uses encrypted snapshots that can be easily backed up and securely
//! shared between devices. Written in stable rust, it has strong guarantees
//! of memory safety and process integrity.
//!
//! ## Initialize stronghold signer from seed
//! ```no_run
//! use tidext::TidefiKeyring;
//!
//! TidefiKeyring::try_from_seed("sr25519 raw seed or mnemonic", None)?;
//!
//! ```
//!
//! ## Initialize stronghold signer from existing snapshot
//! ```no_run
//! use tidext::TidefiKeyring;
//!
//! TidefiKeyring::try_from_stronghold_path("~/.stronghold", None, Some("stronghold password"))?;
//!   
//! ```
//!
//! ## Initialize stronghold signer from existing stronghold instance
//! ```no_run
//! use tidext::TidefiKeyring;
//! use iota_stronghold::Stronghold;
//!
//! // This should run into a separate thread, see examples for more details
//! let stronghold = Stronghold::init_stronghold_system(vec![], vec![]);
//!
//! TidefiKeyring::try_from_stronghold_instance(stronghold, None)?;
//!   
//! ```
//!
//! # Tidext client
//!
//! All functions are directly generated from [`tidechain`] metadata with [`subxt`]
//!
//! ## Basic usage
//! ```no_run
//! use tidext::ClientBuilder;
//!
//! // This needs to run on a different thread
//! let client = ClientBuilder::new().build().await?;
//! let swap_fee = client.swap_fee().await?;
//! ```
//!
//! ## Access the runtime API
//!
//! Convert the client to a runtime API wrapper for custom runtime access
//!
//! The [`subxt`] proc macro will provide methods to submit extrinsic(s) and read storage
//! specific to the Tidechain
//!
//! ```no_run
//! use tidext::ClientBuilder;
//!
//! let client = ClientBuilder::new().build().await?;
//! let tides = tidext::with_runtime! {
//!   client,
//!   current_runtime,
//!   {
//!    client
//!     .runtime()
//!     .storage()
//!     .at(None)
//!     .await?
//!     .fetch(&current_runtime.storage().balances().total_issuance())
//!     .await?
//!    }
//!   }
//! ```

pub use crate::{
  client::{Client, ClientBuilder},
  error::Error,
};
pub use keyring::*;
pub use parity_scale_codec::{Decode, Encode};

pub use sp_runtime::{MultiAddress, Permill};
pub use subxt::{tx::Signer, Error as SubstrateSubxtError};
pub use subxt_impl::{runtimes::*, TidechainConfig};
pub use tidefi_primitives as primitives;

pub use traits::*;
pub use utils::*;

#[macro_use]
extern crate log;

mod client;
mod error;
mod keyring;
mod subxt_impl;
mod traits;
mod utils;

/// Test utils
#[cfg(feature = "test")]
pub mod test_utils;
