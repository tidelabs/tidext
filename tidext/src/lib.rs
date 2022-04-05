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
//! TidefiKeyring::try_from_seed("sr25519 raw seed or mnemonic", None)
//!   .await?;
//! ```
//!
//! ## Initialize stronghold signer from existing snapshot
//! ```no_run
//! use tidext::TidefiKeyring;
//!
//! TidefiKeyring::try_from_stronghold_path("~/.stronghold", None, Some("stronghold password"))
//!   .await?;
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
//! TidefiKeyring::try_from_stronghold_instance(stronghold, None)
//!   .await?;
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
//! let client = ClientBuilder::new().set_signer(signer).build().await?;
//! let swap_fee = client.swap_fee().await?;
//! ```
//!
//! ## Access the runtime API
//!
//! Convert the client to a runtime api wrapper for custom runtime access
//!
//! The [`subxt`] proc macro will provide methods to submit extrinsics and read storage
//! specific to the Tidechain [`runtime`]
//!
//! ```no_run
//! use tidext::ClientBuilder;
//!
//! let client = ClientBuilder::new().set_signer(signer).build().await?;
//! let runtime = client.runtime();
//! let tides = runtime.storage().balances().total_issuance(None).await?;
//! ```
//! [`runtime`]: TidechainRuntimeApi

pub use crate::{
  client::{Client, ClientBuilder},
  error::Error,
};
pub use keyring::*;
use pallet_transaction_payment_rpc_runtime_api::RuntimeDispatchInfo;
pub use parity_scale_codec::{Decode, Encode};
use primitives::{
  AccountId, Balance, BalanceInfo, BlockNumber, CurrencyBalance, CurrencyId, CurrencyMetadata,
  Hash, Stake, SwapType,
};
use sp_runtime::MultiAddress;
pub use sp_runtime::Permill;
use std::sync::Arc;
pub use subxt::{extrinsic::Signer, Error as SubstrateSubxtError};
use subxt::{
  rpc::{rpc_params, ClientT},
  sp_core::Bytes,
  PolkadotExtrinsicParams,
};
pub use subxt_impl::{tidechain, TidechainConfig};
use tidechain::runtime_types::pallet_staking::RewardDestination;
pub use tidefi_primitives as primitives;
use tidext_macro::tidext;
pub use traits::*;
pub use utils::*;

#[macro_use]
extern crate log;

mod error;
mod keyring;
mod subxt_impl;
mod traits;
mod utils;

/// Test utils
#[cfg(feature = "test")]
pub mod test_utils;

/// Tidechain runtime API generated from scale encoding file
pub type TidechainRuntimeApi =
  tidechain::RuntimeApi<TidechainConfig, PolkadotExtrinsicParams<TidechainConfig>>;

pub use crate::tidechain::runtime_types::{
  lagoon_runtime::Call as TidechainCall, pallet_oracle::pallet::Call as OracleCall,
  pallet_quorum::pallet::Call as QuorumCall, pallet_tidefi::pallet::Call as TidefiCall,
  pallet_tidefi_stake::pallet::Call as TidefiStakingCall,
};

#[tidext]
mod client {
  /// Tidechain client
  #[tidext::client]
  pub struct Client {
    pub runtime_api: Arc<TidechainRuntimeApi>,
    pub signer: TidefiKeyring,
  }

  // Automatic implementation of subxt functions, because we are lazy.
  #[tidext::subxt]
  trait ClientSubxt {
    /// Transfer tokens
    #[tidext::pallet = "tidefi"]
    fn transfer(&self, destination_id: AccountId, currency_id: CurrencyId, amount: Balance);

    /// Submit a new swap request on-chain
    #[tidext::pallet = "tidefi"]
    fn swap(
      &self,
      currency_id_from: CurrencyId,
      amount_from: Balance,
      currency_id_to: CurrencyId,
      amount_to: Balance,
      swap_type: SwapType,
      slippage_tolerance: Option<Permill>,
    );

    /// Cancel swap for the current signer
    #[tidext::pallet = "tidefi"]
    fn cancel_swap(&self, request_id: Hash);

    /// Request withdrawal for the current signer
    #[tidext::pallet = "tidefi"]
    fn withdrawal(&self, currency_id: CurrencyId, amount: Balance, external_address: Vec<u8>);

    /// Stake token for the current signer
    #[tidext::pallet = "tidefi_staking"]
    fn stake(&self, currency_id: CurrencyId, amount: Balance, duration: u32);

    /// Declare no desire to either validate or nominate
    #[tidext::pallet = "staking"]
    fn chill(&self);

    /// Bond TIDE tokens.
    /// Take the signer account as a stash and lock up `value` of its balance. `controller` will
    /// be the account that controls it
    #[tidext::pallet = "staking"]
    #[tidext::substitute_params = (
      MultiAddress::Id(controller),
      value,
      RewardDestination::Controller
    )]
    fn bond(&self, controller: AccountId, value: Balance);

    /// Unstake token for the current signer
    #[tidext::pallet = "tidefi_staking"]
    fn unstake(&self, stake_id: Hash, force_unstake: bool);

    /// Return a list of all assets registered on-chain
    #[tidext::pallet = "tidefi"]
    #[tidext::rpc = "getAssets"]
    fn all_assets(&self) -> Result<Vec<(CurrencyId, CurrencyMetadata<Vec<u8>>)>, Error>;

    /// Submit signed extrinsic via RPC
    #[tidext::pallet = "author"]
    #[tidext::rpc = "submitExtrinsic"]
    fn submit_signed_extrinsic(&self, extrinsic: String) -> Result<Hash, Error>;

    /// Validate node connection.
    /// You should call this function every 20 seconds to keep the connection alive.
    #[tidext::pallet = "system"]
    #[tidext::rpc = "health"]
    fn system_health(&self) -> Result<NodeHealth, Error>;

    /// Get the swap fee
    /// If you are a market maker, you should use [`swap_fee_market_maker`] as a reference.
    #[tidext::pallet = "fees"]
    #[tidext::consts = "fee_amount"]
    fn swap_fee(&self) -> Result<Permill, Error>;

    /// Get the market maker swap fee
    #[tidext::pallet = "fees"]
    #[tidext::consts = "market_maker_fee_amount"]
    fn swap_fee_market_maker(&self) -> Result<Permill, Error>;

    /// Submit batch call for the current signer
    #[tidext::pallet = "utility"]
    #[tidext::substitute_fn = "batch"]
    fn submit_batch(&self, calls: Vec<TidechainCall>);
  }

  // Custom implementation of our client
  #[tidext::custom]
  impl Client {
    /// Set new signer for the client
    pub fn set_signer(&mut self, signer: TidefiKeyring) {
      self.signer = signer;
    }

    /// Return tidechain runtime
    pub fn runtime(&self) -> &Arc<TidechainRuntimeApi> {
      &self.runtime_api
    }

    /// Return account id for current signer
    pub fn account_id(&self) -> &AccountId {
      self.signer.account_id()
    }

    /// Return a list of all stakes for the `AccountId` with optional `CurrencyId`
    pub async fn stakes(
      &self,
      account_id: &AccountId,
      currency_id: Option<CurrencyId>,
    ) -> Result<Vec<(CurrencyId, Stake<Balance, BlockNumber>)>, Error> {
      Ok(match currency_id {
        None => query_storage!(self, tidefi_staking, account_stakes, account_id)?
          .into_iter()
          .map(|stake| (stake.currency_id, stake))
          .collect(),
        Some(currency_id) => query_storage!(self, tidefi_staking, account_stakes, account_id)?
          .into_iter()
          .filter(|stake| stake.currency_id == currency_id)
          .map(|stake| (currency_id, stake))
          .collect(),
      })
    }

    /// Return available balance for the current signer
    pub async fn balance(
      &self,
      account_id: &AccountId,
      currency_id: CurrencyId,
    ) -> Result<CurrencyBalance<Balance>, Error> {
      make_rpc_call!(
        self,
        "tidefi_getAccountBalance",
        CurrencyBalance<BalanceInfo>,
        account_id,
        currency_id
      )
      .map(|wrapped_balance| CurrencyBalance {
        available: wrapped_balance.available.amount,
        reserved: wrapped_balance.reserved.amount,
      })
      .map_err(Into::into)
    }

    /// Return available balances with all owned assets for the current signer
    pub async fn balances(
      &self,
      account_id: &AccountId,
    ) -> Result<Vec<(CurrencyId, CurrencyBalance<Balance>)>, Error> {
      make_rpc_call!(
        self,
        "tidefi_getAccountBalances",
        Vec<(CurrencyId, CurrencyBalance<BalanceInfo>)>,
        account_id
      )
      .map(|wrapped_balances| {
        wrapped_balances
          .iter()
          .map(|(currency_id, balance_info)| {
            (
              *currency_id,
              CurrencyBalance {
                available: balance_info.available.amount,
                reserved: balance_info.reserved.amount,
              },
            )
          })
          .collect()
      })
      .map_err(Into::into)
    }

    /// Return the cost (gas fee) of an extrinsic on-chain (always in TIDE)
    pub async fn extrinsic_cost(&self, extrinsic: String) -> Result<Balance, Error> {
      let best_block = latest_block!(self);
      Ok(
        make_rpc_call!(
          self,
          "payment_queryInfo",
          Option<RuntimeDispatchInfo<Balance>>,
          extrinsic,
          best_block
        )?
        .unwrap_or_default()
        .partial_fee,
      )
    }

    /// Return the total staked for the currency
    pub async fn total_stake_for(&self, currency_id: CurrencyId) -> Result<Balance, Error> {
      Ok(query_storage!(self, tidefi_staking, staking_pool, &currency_id)?.unwrap_or(0))
    }

    /// Return the total supply for the currency across all accounts
    pub async fn total_supply_for(&self, currency_id: CurrencyId) -> Result<Balance, Error> {
      match currency_id {
        CurrencyId::Tide => query_storage!(self, balances, total_issuance).map_err(Into::into),
        CurrencyId::Wrapped(wrapped_token) => {
          Ok(query_storage!(self, assets, asset, &wrapped_token)?.map_or(0, |asset| asset.supply))
        }
      }
    }
  }
}
