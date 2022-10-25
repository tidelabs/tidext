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

use crate::{
  latest_block, make_rpc_call, with_runtime, Error, LagoonRuntime, NodeHealth, Permill,
  RewardDestination, Signer, TidechainCall, TidechainConfig, TidechainRuntime, TidefiKeyring,
  TidefiRuntime,
};
use pallet_transaction_payment_rpc_runtime_api::RuntimeDispatchInfo;
use parity_scale_codec::Encode;
use sp_runtime::MultiAddress;
use std::sync::Arc;
use subxt::{
  error::{DispatchError, TransactionError},
  events::Phase,
  ext::sp_core::{
    blake2_256,
    crypto::{Ss58AddressFormat, Ss58Codec},
  },
  rpc::{rpc_params, SubstrateTxStatus},
  utils::Encoded,
};
use tidefi_primitives::{
  AccountId, Balance, BalanceInfo, BlockNumber, CurrencyBalance, CurrencyId, CurrencyMetadata,
  EraIndex, Hash, Stake, SwapType,
};
use tidext_macro::tidext;

pub use client::*;

#[tidext]
mod client {
  use crate::{lagoon::tidefi_staking, tidechain::tidefi_staking};

  /// Tidechain client
  #[tidext::client]
  pub struct Client {
    pub client: Arc<subxt::OnlineClient<TidechainConfig>>,
    pub runtime_type: TidefiRuntime,
    pub signer: Option<TidefiKeyring>,
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

    /// Declare no desire to either validate or nominate
    #[tidext::pallet = "staking"]
    fn chill(&self);

    /// Nominate validators
    #[tidext::pallet = "staking"]
    #[tidext::substitute_params = (
      value.iter().map(|v| MultiAddress::Id(v.clone())).collect(),
    )]
    fn nominate(&self, value: Vec<AccountId>);

    /// Bond TDFY tokens
    /// Take the signer account as a stash and lock up `value` of its balance. `controller` will
    /// be the account that controls it
    #[tidext::pallet = "staking"]
    #[tidext::substitute_params = (
      MultiAddress::Id(controller),
      value,
      RewardDestination::Controller.into()
    )]
    fn bond(&self, controller: AccountId, value: Balance);

    /// Bond some extra amount
    #[tidext::pallet = "staking"]
    fn bond_extra(&self, value: Balance);

    /// Schedule a portion of the stash to be unlocked ready for transfer out after the bond
    #[tidext::pallet = "staking"]
    fn unbond(&self, amount: Balance);

    /// Remove any unlocked chunks from the unlocking queue from our management
    #[tidext::pallet = "staking"]
    fn withdraw_unbonded(&self, num_slashing_spans: u32);

    /// Stake token for the current signer
    #[tidext::pallet = "tidefi_staking"]
    fn stake(&self, currency_id: CurrencyId, amount: Balance, duration: u32);

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

    /// Send a batch of dispatch calls for the current signer
    #[tidext::pallet = "utility"]
    #[tidext::substitute_fn = "batch"]
    #[tidext::substitute_params = (
      calls.into_iter().map(Into::into).collect(),
    )]
    fn submit_batch(&self, calls: Vec<TidechainCall>);

    /// Send a batch of dispatch calls for the current signer. Unlike `submit_batch` it allows error and won't interrupt.
    #[tidext::pallet = "utility"]
    #[tidext::substitute_params = (
      calls.into_iter().map(Into::into).collect(),
    )]
    fn force_batch(&self, calls: Vec<TidechainCall>);

    /// Claim available sunrise rewards for the `era_index`.
    #[tidext::pallet = "tidefi"]
    fn claim_sunrise_rewards(&self, era_index: EraIndex);
  }

  // Custom implementation of our client
  #[tidext::custom]
  impl Client {
    /// Get runtime type
    pub fn runtime_type(&self) -> TidefiRuntime {
      self.runtime_type.clone()
    }

    /// Set new signer for the client
    pub fn set_signer(&mut self, signer: Option<TidefiKeyring>) {
      self.signer = signer;
    }

    /// Return tidechain online client
    pub fn runtime(&self) -> &Arc<subxt::OnlineClient<TidechainConfig>> {
      &self.client
    }

    /// Return account id for current signer
    pub fn account_id(&self) -> Option<&AccountId> {
      self.signer().map(|signer| signer.account_id()).ok()
    }

    /// Return SS58 address of the current signer on the active chain
    pub fn account_id_ss58(&self) -> Option<String> {
      let network_override = Ss58AddressFormat::from(self.ss58_prefix());
      self
        .account_id()
        .map(|signer| signer.to_ss58check_with_version(network_override))
    }

    /// Return SS58 prefix of the current active network
    pub fn ss58_prefix(&self) -> u16 {
      with_runtime! {
        self,
        current_runtime,
        {
          self
          .runtime()
          .constants()
          .at(&current_runtime.constants().system().ss58_prefix())
          // if no prefix are set, we fallback to substrate
          .unwrap_or(42)
        }
      }
    }

    /// Return signer if set
    pub fn signer(&self) -> Result<&TidefiKeyring, Error> {
      self.signer.as_ref().ok_or(Error::NoSignerAvailable)
    }

    /// Submit signed extrinsic via RPC and wait for inclusing in a block and may take up to 6 seconds to complete
    pub async fn submit_signed_extrinsic_and_wait_for_in_block_success(
      &self,
      extrinsic: String,
    ) -> Result<Hash, Error> {
      // rebuild the extrinsic bytes from the hex string
      let ext_bytes = hex::decode(
        extrinsic
          .strip_prefix("0x")
          .ok_or(Error::InvalidExtrinsic)?,
      )
      .map_err(|_| Error::InvalidExtrinsic)?;

      // extrinsic hash
      let ext_hash = blake2_256(&ext_bytes[..]);

      // submit and watch for transaction progress.
      let mut sub = self
        .runtime()
        .rpc()
        .watch_extrinsic(Encoded(ext_bytes))
        .await?;

      while let Some(status) = sub.next().await {
        match status? {
          SubstrateTxStatus::InBlock(block_hash) | SubstrateTxStatus::Finalized(block_hash) => {
            let block = self
              .runtime()
              .rpc()
              .block(Some(block_hash))
              .await?
              .ok_or(Error::Substrate(TransactionError::BlockHashNotFound.into()))?;

            let extrinsic_idx = block
              .block
              .extrinsics
              .iter()
              .position(|ext| {
                let hash = blake2_256(&ext.encode());
                hash == ext_hash
              })
              .ok_or(Error::Substrate(TransactionError::BlockHashNotFound.into()))?;

            let events = self.runtime().events().at(Some(block_hash)).await?;
            for ev in events.iter().filter(|ev| {
              ev.as_ref()
                .map(|ev| ev.phase() == Phase::ApplyExtrinsic(extrinsic_idx as u32))
                .unwrap_or(true)
            }) {
              let ev = ev?;
              // if we find an `ExtrinsicFailed` connected with the same extrinsic idx, we should throw the error
              if ev.pallet_name() == "System" && ev.variant_name() == "ExtrinsicFailed" {
                let dispatch_error =
                  DispatchError::decode_from(ev.field_bytes(), &self.runtime().metadata());
                return Err(Error::Substrate(dispatch_error.into()));
              }
            }

            // close the loop
            break;
          }
          SubstrateTxStatus::FinalityTimeout(_) => {
            return Err(Error::Substrate(
              TransactionError::FinalitySubscriptionTimeout.into(),
            ))
          }
          _ => continue,
        }
      }

      Ok(Hash::from(ext_hash))
    }

    /// Return a list of all stakes for the `AccountId` with optional `CurrencyId`
    pub async fn stakes(
      &self,
      account_id: &AccountId,
      currency_id: Option<CurrencyId>,
    ) -> Result<Vec<(CurrencyId, Stake<Balance, BlockNumber>)>, Error> {
      let stakes = match currency_id {
        None => {
          with_runtime! {
            self,
            current_runtime,
            {
              self
              .runtime()
              .storage()
              .fetch(&current_runtime.storage().tidefi_staking().account_stakes(account_id), None)
              .await?
              .unwrap_or_default()
              .into_iter()
              .map(|stake| (stake.currency_id, stake))
              .collect()
            }
          }
        }
        Some(currency_id) => {
          with_runtime! {
            self,
            current_runtime,
            {
              self
              .runtime()
              .storage()
              .fetch(&current_runtime.storage().tidefi_staking().account_stakes(account_id), None)
              .await?
              .unwrap_or_default()
              .into_iter()
              .filter(|stake| stake.currency_id == currency_id)
              .map(|stake| (currency_id, stake))
              .collect()
            }
          }
        }
      };

      Ok(stakes)
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

    /// Return the cost (gas fee) of an extrinsic on-chain (always in TDFY)
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
      Ok(with_runtime! {
        self,
        current_runtime,
        {
            self
            .runtime()
            .storage()
            .fetch(&current_runtime.storage().tidefi_staking().staking_pool(currency_id), None)
            .await?
            .unwrap_or_default()
        }
      })
    }

    /// Return the total supply for the currency across all accounts
    pub async fn total_supply_for(&self, currency_id: CurrencyId) -> Result<Balance, Error> {
      Ok(match currency_id {
        CurrencyId::Tdfy => with_runtime! {
          self,
          current_runtime,
          {
            self
            .runtime()
            .storage()
            .fetch(&current_runtime.storage().balances().total_issuance(), None)
            .await?
            .unwrap_or_default()
          }
        },
        CurrencyId::Wrapped(wrapped_token) => with_runtime! {
          self,
          current_runtime,
          {
            self
            .runtime()
            .storage()
            .fetch(&current_runtime.storage().assets().asset(wrapped_token), None)
            .await?
            .map(|asset| asset.supply)
            .unwrap_or_default()
          }
        },
      })
    }

    #[cfg(feature = "decoder")]
    /// Extrinsic decoder
    /// This function is unstable and may be updated anytime with breaking changes.
    pub async fn decode_extrinsic(&self, data: &mut &[u8]) -> Result<subxt::Extrinsic, Error> {
      self
        .runtime()
        .metadata()
        .decode_extrinsic(data)
        .map_err(Into::into)
    }
  }
}
