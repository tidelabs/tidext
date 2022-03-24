use crate::{
  error::Error,
  keyring::{AccountId, TidefiPairSigner},
};
use serde::{Deserialize, Serialize};
use subxt::{
  rpc::{rpc_params, ClientT},
  sp_core::Bytes,
  ClientBuilder as SubstrateClientBuilder, Config, DefaultConfig, DefaultExtra,
};

use jsonrpsee::ws_client::WsClientBuilder;
use pallet_transaction_payment_rpc_runtime_api::RuntimeDispatchInfo;
pub use sp_runtime::Permill;
use std::time::Duration;
pub use tidefi_primitives::assets;
use tidefi_primitives::{BalanceInfo, BlockNumber, CurrencyBalance, CurrencyMetadata, Stake};

// logging
#[macro_use]
extern crate log;

pub use parity_scale_codec::{Decode, Encode};

pub use subxt::{Error as SubstrateSubxtError, Signer};

pub mod error;
pub mod keyring;
mod traits;
pub use traits::*;
#[cfg(feature = "test")]
pub mod test_utils;

pub use tidefi_primitives::{Balance, CurrencyId, Hash, SwapType};
#[subxt::subxt(runtime_metadata_path = "res/tidechain_metadata.scale")]
pub mod tidechain {
  #[subxt(substitute_type = "sp_arithmetic::per_things::Percent")]
  use sp_runtime::Percent;
  #[subxt(substitute_type = "sp_arithmetic::per_things::Permill")]
  use sp_runtime::Permill;
  #[subxt(substitute_type = "tidefi_primitives::ComplianceLevel")]
  use tidefi_primitives::ComplianceLevel;
  #[subxt(substitute_type = "tidefi_primitives::CurrencyId")]
  use tidefi_primitives::CurrencyId;
  #[subxt(substitute_type = "tidefi_primitives::Mint")]
  use tidefi_primitives::Mint;
  #[subxt(substitute_type = "tidefi_primitives::OracleImAlive")]
  use tidefi_primitives::OracleImAlive;
  #[subxt(substitute_type = "tidefi_primitives::ProposalType")]
  use tidefi_primitives::ProposalType;
  #[subxt(substitute_type = "tidefi_primitives::ProposalVotes")]
  use tidefi_primitives::ProposalVotes;
  #[subxt(substitute_type = "tidefi_primitives::Stake")]
  use tidefi_primitives::Stake;
  #[subxt(substitute_type = "tidefi_primitives::SwapConfirmation")]
  use tidefi_primitives::SwapConfirmation;
  #[subxt(substitute_type = "tidefi_primitives::SwapStatus")]
  use tidefi_primitives::SwapStatus;
  #[subxt(substitute_type = "tidefi_primitives::SwapType")]
  use tidefi_primitives::SwapType;
  #[subxt(substitute_type = "tidefi_primitives::Withdrawal")]
  use tidefi_primitives::Withdrawal;
  #[subxt(substitute_type = "frame_support::storage::bounded_vec::BoundedVec")]
  use Vec;
}

pub type TidechainRuntimeApi =
  tidechain::RuntimeApi<TidechainConfig, DefaultExtra<TidechainConfig>>;

// Expose for easier batch calls
pub use crate::tidechain::runtime_types::{
  lagoon_runtime::Call as TidechainCall, pallet_oracle::pallet::Call as OracleCall,
  pallet_quorum::pallet::Call as QuorumCall, pallet_tidefi::pallet::Call as TidefiCall,
  pallet_tidefi_stake::pallet::Call as TidefiStakingCall,
};

/// Custom [`Config`] impl with Clone
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TidechainConfig;
impl Config for TidechainConfig {
  type Index = <DefaultConfig as Config>::Index;
  type BlockNumber = <DefaultConfig as Config>::BlockNumber;
  type Hash = <DefaultConfig as Config>::Hash;
  type Hashing = <DefaultConfig as Config>::Hashing;
  type AccountId = <DefaultConfig as Config>::AccountId;
  type Address = <DefaultConfig as Config>::Address;
  type Header = <DefaultConfig as Config>::Header;
  type Signature = <DefaultConfig as Config>::Signature;
  type Extrinsic = <DefaultConfig as Config>::Extrinsic;
}

/// Health struct returned by the RPC
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeHealth {
  /// Number of connected peers
  pub peers: usize,
  /// Is the node syncing
  pub is_syncing: bool,
  /// Should this node have any peers
  ///
  /// Might be false for local chains or when running without discovery.
  pub should_have_peers: bool,
}

// client builder
#[derive(Clone)]
pub struct ClientBuilder {
  pub rpc_url: String,
  pub gaz_limit: u64,
  pub signer: Option<TidefiPairSigner>,
  pub other_signatories: Vec<AccountId>,
}

impl Default for ClientBuilder {
  fn default() -> Self {
    Self {
      rpc_url: "ws://127.0.0.1:9944".into(),
      gaz_limit: 100_000_000_000,
      signer: None,
      other_signatories: Vec::new(),
    }
  }
}

impl ClientBuilder {
  /// Creates a new `ClientBuilder`.
  pub fn new() -> Self {
    trace!("Creating new ClientBuilder");
    Default::default()
  }

  /// Set the substrate RPC address.
  pub fn set_url<P: Into<String>>(mut self, url: P) -> Self {
    self.rpc_url = url.into();
    self
  }

  /// Set the gas limit for each call.
  pub fn set_gaz_limit(mut self, gaz_limit: u64) -> Self {
    self.gaz_limit = gaz_limit;
    self
  }

  /// Set the substrate main signer.
  pub fn set_signer(mut self, signer: TidefiPairSigner) -> Self {
    self.signer = Some(signer);
    self
  }

  /// Creates a new Client.
  pub async fn build(self) -> Result<Client, Error> {
    let ws_client = WsClientBuilder::default()
      .max_notifs_per_subscription(4096)
      .connection_timeout(Duration::from_secs(5))
      .build(&self.rpc_url)
      .await
      .map_err(|err| Error::Other(err.to_string()))?;

    // build substrate runtime api
    let runtime = SubstrateClientBuilder::new()
      .set_client(ws_client)
      .build()
      .await?;

    if let Some(signer) = self.signer {
      return Ok(Client {
        signer,
        runtime,
        rpc_url: self.rpc_url,
        gaz_limit: self.gaz_limit,
        other_signatories: self.other_signatories,
      });
    }

    Err(Error::Other(
      "Unable to build the client. Make sure to set a signer.".to_string(),
    ))
  }
}

/// Tidechain client
#[derive(Clone)]
pub struct Client {
  runtime: subxt::Client<TidechainConfig>,
  pub gaz_limit: u64,
  pub signer: TidefiPairSigner,
  pub rpc_url: String,
  pub other_signatories: Vec<AccountId>,
}

impl Client {
  /// Set new signer for the client.
  pub fn set_signer(&mut self, signer: TidefiPairSigner) {
    self.signer = signer;
  }

  /// `Subxt` Runtime (Isolated, but not `clonable`)
  pub fn runtime(&self) -> TidechainRuntimeApi {
    self.runtime.clone().to_runtime_api()
  }

  /// Get account id for current signer.
  pub fn get_account_id(&self) -> AccountId {
    self.signer.account_id().clone()
  }

  /// Get the swap percentage fee registered on-chain for a regular user
  // FIXME: MAke sure substrate handle well the f32 (0.2 percent by example)
  pub async fn get_regular_swap_fee(&self) -> Result<f32, Error> {
    self
      .runtime()
      .constants()
      .fees()
      .fee_amount()
      .map(|percent| percent.deconstruct() as f32 / 1_000_000_f32)
      .map_err(|err| Error::Other(err.to_string()))
  }

  /// Get the swap percentage fee registered on-chain for a market maker
  // FIXME: Add custom storage on-chain for the MM fee
  pub async fn get_market_maker_swap_fee(&self) -> Result<f32, Error> {
    self
      .runtime()
      .constants()
      .fees()
      .market_maker_fee_amount()
      .map(|percent| percent.deconstruct() as f32 / 1_000_000_f32)
      .map_err(|err| Error::Other(err.to_string()))
  }

  /// Validate node connection. You should call this function every ~ 10 seconds to keep the connection alive
  pub async fn system_health(&self) -> Result<NodeHealth, Error> {
    self
      .runtime()
      .client
      .rpc()
      .client
      .request::<NodeHealth>("system_health", None)
      .await
      .map_err(|err| Error::NetworkError(err.to_string()))
  }

  /// Submit extrinsic
  pub async fn submit_signed_extrinsic(&self, extrinsic: String) -> Result<Hash, Error> {
    let final_hash = self
      .runtime()
      .client
      .rpc()
      .client
      .request::<Hash>("author_submitExtrinsic", rpc_params![extrinsic])
      .await
      .map_err(|err| Error::Other(err.to_string()))?;
    Ok(final_hash)
  }

  /// Get the cost (gas fee) of an extrinsic on-chain (always in TIDE).
  pub async fn extrinsic_cost(&self, extrinsic: String) -> Result<Balance, Error> {
    let best_block = self
      .runtime()
      .client
      .rpc()
      .block_hash(None)
      .await?
      .unwrap_or_default();
    let extrinsic_cost = self
      .runtime()
      .client
      .rpc()
      .client
      .request::<Option<RuntimeDispatchInfo<Balance>>>(
        "payment_queryInfo",
        rpc_params![extrinsic, best_block],
      )
      .await
      .map_err(|err| Error::Other(err.to_string()))?
      .unwrap_or_default();

    Ok(extrinsic_cost.partial_fee)
  }

  /// Current quorum status.
  pub async fn quorum_status(&self) -> Result<bool, Error> {
    self
      .runtime()
      .storage()
      .quorum()
      .quorum_status(None)
      .await
      .map_err(|err| Error::Other(err.to_string()))
  }

  /// Return the total staked for the currency.
  pub async fn total_stake_for(&self, currency_id: CurrencyId) -> Result<Balance, Error> {
    Ok(
      self
        .runtime()
        .storage()
        .tidefi_staking()
        .staking_pool(&currency_id, None)
        .await?
        .unwrap_or(0),
    )
  }

  /// Return the total supply for the currency across all accounts.
  ///
  /// **Wrapped token** are minted when deposited and burned when withdrawed, the total supply represent
  /// the total tokens deposited in chain and not withdrawed.
  ///
  /// Total issuance for **TiDE token**, represent the number of tokens issued by the runtime.
  pub async fn total_supply_for(&self, currency_id: CurrencyId) -> Result<Balance, Error> {
    match currency_id {
      CurrencyId::Tide => self
        .runtime()
        .storage()
        .balances()
        .total_issuance(None)
        .await
        .map_err(|err| Error::Other(err.to_string())),
      CurrencyId::Wrapped(wrapped_token) => Ok(
        self
          .runtime()
          .storage()
          .assets()
          .asset(&wrapped_token, None)
          .await?
          .map_or(0, |asset| asset.supply),
      ),
    }
  }

  /// Return available balance.
  pub async fn balance(
    &self,
    account_id: &AccountId,
    currency_id: CurrencyId,
  ) -> Result<CurrencyBalance<Balance>, Error> {
    self
      .runtime()
      .client
      .rpc()
      .client
      .request::<CurrencyBalance<BalanceInfo>>(
        "tidefi_getAccountBalance",
        rpc_params![account_id, currency_id],
      )
      .await
      .map_err(|err| Error::Other(err.to_string()))
      .map(|wrapped_balance| CurrencyBalance {
        available: wrapped_balance.available.amount,
        reserved: wrapped_balance.reserved.amount,
      })
  }

  /// Return available balances with all owned assets.
  pub async fn balances(
    &self,
    account_id: &AccountId,
  ) -> Result<Vec<(CurrencyId, CurrencyBalance<Balance>)>, Error> {
    self
      .runtime()
      .client
      .rpc()
      .client
      .request::<Vec<(CurrencyId, CurrencyBalance<BalanceInfo>)>>(
        "tidefi_getAccountBalances",
        rpc_params![account_id],
      )
      .await
      .map_err(|err| Error::Other(err.to_string()))
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
  }

  /// Return a list of all assets registered on-chain.
  pub async fn all_assets(&self) -> Result<Vec<(CurrencyId, CurrencyMetadata<Vec<u8>>)>, Error> {
    self
      .runtime()
      .client
      .rpc()
      .client
      .request("tidefi_getAssets", None)
      .await
      .map_err(|err| Error::Other(err.to_string()))
  }

  /// Return a list of all stakes for the `AccountId` with optional `CurrencyId`.
  pub async fn stakes(
    &self,
    account_id: &AccountId,
    currency_id: Option<CurrencyId>,
  ) -> Result<Vec<(CurrencyId, Stake<Balance, BlockNumber>)>, Error> {
    Ok(match currency_id {
      None => self
        .runtime()
        .storage()
        .tidefi_staking()
        .account_stakes(&account_id, None)
        .await?
        .into_iter()
        .map(|stake| (stake.currency_id, stake.clone()))
        .collect(),
      Some(currency_id) => self
        .runtime()
        .storage()
        .tidefi_staking()
        .account_stakes(&account_id, None)
        .await?
        .into_iter()
        .filter(|stake| stake.currency_id == currency_id)
        .map(|stake| (currency_id, stake.clone()))
        .collect(),
    })
  }

  /// Request withdrawal for the current signer.
  pub async fn withdrawal(
    &self,
    currency_id: CurrencyId,
    amount: Balance,
    external_address: Vec<u8>,
  ) -> Result<(), Error> {
    self
      .runtime()
      .tx()
      .tidefi()
      .withdrawal(currency_id, amount, external_address)
      .sign_and_submit(&self.signer)
      .await?;
    Ok(())
  }

  /// Request withdrawal for the current signer and wait the processing in block.
  pub async fn withdrawal_and_watch(
    &self,
    currency_id: CurrencyId,
    amount: Balance,
    external_address: Vec<u8>,
  ) -> Result<(), Error> {
    self
      .runtime()
      .tx()
      .tidefi()
      .withdrawal(currency_id, amount, external_address)
      .sign_and_submit_then_watch(&self.signer)
      .await?
      .wait_for_finalized_success()
      .await
      .map_err(|err| Error::Other(err.to_string()))?;

    Ok(())
  }

  /// Initialize a withdrawal for the current signer.
  pub async fn withdrawal_extrinsic(
    &self,
    asset_id: CurrencyId,
    amount: Balance,
    external_address: Vec<u8>,
  ) -> Result<String, Error> {
    let extrinsic = self
      .runtime()
      .tx()
      .tidefi()
      .withdrawal(asset_id, amount, external_address)
      .create_signed(&self.signer, Default::default())
      .await?;

    let bytes: Bytes = extrinsic.encode().into();
    Ok(format!("0x{}", hex::encode(bytes.to_vec()).as_str()))
  }

  /// Initialize a swap for the current signer.
  /// The `amount_from` of the `currency_id_from` + the network fee eg. 2%, will be held untill the swap is processed or cancelled.
  pub async fn swap(
    &self,
    currency_id_from: CurrencyId,
    amount_from: Balance,
    currency_id_to: CurrencyId,
    amount_to: Balance,
    swap_type: SwapType,
    slippage_tolerance: Option<Permill>,
  ) -> Result<(), Error> {
    self
      .runtime()
      .tx()
      .tidefi()
      .swap(
        currency_id_from,
        amount_from,
        currency_id_to,
        amount_to,
        swap_type,
        slippage_tolerance,
      )
      .sign_and_submit(&self.signer)
      .await?;
    Ok(())
  }

  /// Cancel a swap and release the funds.
  pub async fn cancel_swap(&self, request_id: Hash) -> Result<(), Error> {
    self
      .runtime()
      .tx()
      .tidefi()
      .cancel_swap(request_id)
      .sign_and_submit(&self.signer)
      .await?;
    Ok(())
  }

  /// Initialize a swap for the current signer and wait the processing in block.
  /// The `amount_from` of the `currency_id_from` will be held untill the swap is processed or cancelled.
  pub async fn swap_and_watch(
    &self,
    currency_id_from: CurrencyId,
    amount_from: Balance,
    currency_id_to: CurrencyId,
    amount_to: Balance,
    swap_type: SwapType,
    slippage_tolerance: Option<Permill>,
  ) -> Result<(), Error> {
    self
      .runtime()
      .tx()
      .tidefi()
      .swap(
        currency_id_from,
        amount_from,
        currency_id_to,
        amount_to,
        swap_type,
        slippage_tolerance,
      )
      .sign_and_submit_then_watch(&self.signer)
      .await?
      .wait_for_finalized_success()
      .await
      .map_err(|err| Error::Other(err.to_string()))?;
    Ok(())
  }

  /// Generate signed swap extrinsic for the current signer.
  /// The `amount_from` of the `currency_id_from` will be held untill the swap is processed or cancelled.
  pub async fn swap_extrinsic(
    &self,
    currency_id_from: CurrencyId,
    amount_from: Balance,
    currency_id_to: CurrencyId,
    amount_to: Balance,
    swap_type: SwapType,
    slippage_tolerance: Option<Permill>,
  ) -> Result<String, Error> {
    let extrinsic = self
      .runtime()
      .tx()
      .tidefi()
      .swap(
        currency_id_from,
        amount_from,
        currency_id_to,
        amount_to,
        swap_type,
        slippage_tolerance,
      )
      .create_signed(&self.signer, Default::default())
      .await?;

    let bytes: Bytes = extrinsic.encode().into();
    Ok(format!("0x{}", hex::encode(bytes.to_vec()).as_str()))
  }

  /// Request stake for the current signer.
  ///
  pub async fn stake(
    &self,
    currency_id: CurrencyId,
    amount: Balance,
    duration: u32,
  ) -> Result<(), Error> {
    self
      .runtime()
      .tx()
      .tidefi_staking()
      .stake(currency_id, amount, duration)
      .sign_and_submit(&self.signer)
      .await?;
    Ok(())
  }

  /// Request stake for the current signer and wait the processing in block.
  pub async fn stake_and_watch(
    &self,
    currency_id: CurrencyId,
    amount: Balance,
    duration: u32,
  ) -> Result<(), Error> {
    self
      .runtime()
      .tx()
      .tidefi_staking()
      .stake(currency_id, amount, duration)
      .sign_and_submit_then_watch(&self.signer)
      .await?
      .wait_for_finalized_success()
      .await
      .map_err(|err| Error::Other(err.to_string()))?;

    Ok(())
  }

  /// Request stake extrinsic for the current signer.
  pub async fn stake_extrinsic(
    &self,
    currency_id: CurrencyId,
    amount: Balance,
    duration: u32,
  ) -> Result<String, Error> {
    let extrinsic = self
      .runtime()
      .tx()
      .tidefi_staking()
      .stake(currency_id, amount, duration)
      .create_signed(&self.signer, Default::default())
      .await?;

    let bytes: Bytes = extrinsic.encode().into();
    Ok(format!("0x{}", hex::encode(bytes.to_vec()).as_str()))
  }

  /// Request unstake for the current signer.
  pub async fn unstake(&self, stake_id: Hash, force_unstake: bool) -> Result<(), Error> {
    self
      .runtime()
      .tx()
      .tidefi_staking()
      .unstake(stake_id, force_unstake)
      .sign_and_submit(&self.signer)
      .await?;
    Ok(())
  }

  /// Request unstake for the current signer and wait the processing in block.
  pub async fn unstake_and_watch(&self, stake_id: Hash, force_unstake: bool) -> Result<(), Error> {
    self
      .runtime()
      .tx()
      .tidefi_staking()
      .unstake(stake_id, force_unstake)
      .sign_and_submit_then_watch(&self.signer)
      .await?
      .wait_for_finalized_success()
      .await
      .map_err(|err| Error::Other(err.to_string()))?;

    Ok(())
  }

  /// Unstake extrinsic for the current signer.
  pub async fn unstake_extrinsic(
    &self,
    stake_id: Hash,
    force_unstake: bool,
  ) -> Result<String, Error> {
    let extrinsic = self
      .runtime()
      .tx()
      .tidefi_staking()
      .unstake(stake_id, force_unstake)
      .create_signed(&self.signer, Default::default())
      .await?;

    let bytes: Bytes = extrinsic.encode().into();
    Ok(format!("0x{}", hex::encode(bytes.to_vec()).as_str()))
  }

  /// Transfer currency to another account id from the current signer.
  pub async fn transfer(
    &self,
    destination_id: &AccountId,
    currency_id: CurrencyId,
    amount: Balance,
  ) -> Result<(), Error> {
    self
      .runtime()
      .tx()
      .tidefi()
      .transfer(destination_id.clone(), currency_id, amount)
      .sign_and_submit(&self.signer)
      .await?;
    Ok(())
  }

  /// Transfer currency to another account id from the current signer and wait the processing in block.
  pub async fn transfer_and_watch(
    &self,
    destination_id: &AccountId,
    currency_id: CurrencyId,
    amount: Balance,
  ) -> Result<(), Error> {
    self
      .runtime()
      .tx()
      .tidefi()
      .transfer(destination_id.clone(), currency_id, amount)
      .sign_and_submit_then_watch(&self.signer)
      .await?
      .wait_for_finalized_success()
      .await
      .map_err(|err| Error::Other(err.to_string()))?;

    Ok(())
  }

  /// Transfer extrinsic for the current signer.
  pub async fn transfer_extrinsic(
    &self,
    destination_id: &AccountId,
    currency_id: CurrencyId,
    amount: Balance,
  ) -> Result<String, Error> {
    let extrinsic = self
      .runtime()
      .tx()
      .tidefi()
      .transfer(destination_id.clone(), currency_id, amount)
      .create_signed(&self.signer, Default::default())
      .await?;

    let bytes: Bytes = extrinsic.encode().into();
    Ok(format!("0x{}", hex::encode(bytes.to_vec()).as_str()))
  }

  /// Submit batch call.
  pub async fn submit_batch(&self, calls: Vec<TidechainCall>) -> Result<(), Error> {
    self
      .runtime()
      .tx()
      .utility()
      .batch(calls)
      .sign_and_submit(&self.signer)
      .await
      .map_err(|err| Error::Other(err.to_string()))?;

    Ok(())
  }
}
