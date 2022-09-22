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
#![allow(clippy::too_many_arguments)]

use sp_runtime::{MultiAddress, Permill};
use std::sync::Arc;
use subxt::{Config, PolkadotConfig, SubstrateConfig as DefaultConfig};
use tidefi_primitives::{AccountId, Balance, CurrencyId, Hash, SwapType};

macro_rules! with_config {
  {
    $mod_name:ident,
    $runtime_name:ident,
    $metadata_path:tt
  } => {
    #[subxt::subxt(runtime_metadata_path = $metadata_path)]
    pub mod $mod_name {
      #[subxt(substitute_type = "sp_runtime::bounded::bounded_vec::BoundedVec")]
      use ::Vec;
      #[subxt(substitute_type = "frame_support::storage::bounded_vec::BoundedVec")]
      use ::Vec;
      #[subxt(substitute_type = "frame_support::PalletId")]
      use ::frame_support::PalletId;
      #[subxt(substitute_type = "sp_arithmetic::per_things::Percent")]
      use ::sp_runtime::Percent;
      #[subxt(substitute_type = "sp_arithmetic::per_things::Permill")]
      use ::sp_runtime::Permill;
      #[subxt(substitute_type = "tidefi_primitives::ComplianceLevel")]
      use ::tidefi_primitives::ComplianceLevel;
      #[subxt(substitute_type = "tidefi_primitives::CurrencyId")]
      use ::tidefi_primitives::CurrencyId;
      #[subxt(substitute_type = "tidefi_primitives::Mint")]
      use ::tidefi_primitives::Mint;
      #[subxt(substitute_type = "tidefi_primitives::OracleImAlive")]
      use ::tidefi_primitives::OracleImAlive;
      #[subxt(substitute_type = "tidefi_primitives::ProposalType")]
      use ::tidefi_primitives::ProposalType;
      #[subxt(substitute_type = "tidefi_primitives::ProposalVotes")]
      use ::tidefi_primitives::ProposalVotes;
      #[subxt(substitute_type = "tidefi_primitives::Stake")]
      use ::tidefi_primitives::Stake;
      #[subxt(substitute_type = "tidefi_primitives::SwapConfirmation")]
      use ::tidefi_primitives::SwapConfirmation;
      #[subxt(substitute_type = "tidefi_primitives::SwapStatus")]
      use ::tidefi_primitives::SwapStatus;
      #[subxt(substitute_type = "tidefi_primitives::SwapType")]
      use ::tidefi_primitives::SwapType;
      #[subxt(substitute_type = "tidefi_primitives::Withdrawal")]
      use ::tidefi_primitives::Withdrawal;
    }

    impl From<TidechainCall> for $mod_name::runtime_types::$runtime_name::Call {
      fn from(current_call: TidechainCall) -> $mod_name::runtime_types::$runtime_name::Call {
        match current_call {
          TidechainCall::Tidefi(tidefi_call) => {
            $mod_name::runtime_types::$runtime_name::Call::Tidefi(tidefi_call.into())
          }
          TidechainCall::Quorum(quorum_call) => {
            $mod_name::runtime_types::$runtime_name::Call::Quorum(quorum_call.into())
          }
          TidechainCall::Oracle(oracle_call) => {
            $mod_name::runtime_types::$runtime_name::Call::Oracle(oracle_call.into())
          }
          TidechainCall::Staking(staking_call) => {
            $mod_name::runtime_types::$runtime_name::Call::Staking(staking_call.into())
          }
        }
      }
    }

    impl From<TidefiCall> for $mod_name::runtime_types::pallet_tidefi::pallet::Call {
      fn from(current_call: TidefiCall) -> $mod_name::runtime_types::pallet_tidefi::pallet::Call {
        match current_call {
          TidefiCall::Transfer {
            destination_id,
            currency_id,
            amount,
          } => $mod_name::runtime_types::pallet_tidefi::pallet::Call::transfer {
            destination_id,
            currency_id,
            amount,
          },
          TidefiCall::Withdrawal {
            currency_id,
            amount,
            external_address,
          } => $mod_name::runtime_types::pallet_tidefi::pallet::Call::withdrawal {
            currency_id,
            amount,
            external_address,
          },
          TidefiCall::Swap {
            currency_id_from,
            amount_from,
            currency_id_to,
            amount_to,
            swap_type,
            slippage_tolerance,
          } => $mod_name::runtime_types::pallet_tidefi::pallet::Call::swap {
            currency_id_from,
            amount_from,
            currency_id_to,
            amount_to,
            swap_type,
            slippage_tolerance,
          },
          TidefiCall::CancelSwap { request_id } => {
            $mod_name::runtime_types::pallet_tidefi::pallet::Call::cancel_swap { request_id }
          }
          TidefiCall::ClaimSunriseRewards { era_index } => {
            $mod_name::runtime_types::pallet_tidefi::pallet::Call::claim_sunrise_rewards { era_index }
          }
        }
      }
    }

    impl From<QuorumCall> for $mod_name::runtime_types::pallet_quorum::pallet::Call {
      fn from(current_call: QuorumCall) -> $mod_name::runtime_types::pallet_quorum::pallet::Call {
        match current_call {
          QuorumCall::SubmitProposal { proposal } => {
            $mod_name::runtime_types::pallet_quorum::pallet::Call::submit_proposal { proposal }
          }
          QuorumCall::AcknowledgeProposal { proposal } => {
            $mod_name::runtime_types::pallet_quorum::pallet::Call::acknowledge_proposal { proposal }
          }
          QuorumCall::AcknowledgeBurned { proposal } => {
            $mod_name::runtime_types::pallet_quorum::pallet::Call::acknowledge_burned { proposal }
          }
          QuorumCall::RejectProposal { proposal } => {
            $mod_name::runtime_types::pallet_quorum::pallet::Call::reject_proposal { proposal }
          }
          QuorumCall::EvalProposalState { proposal } => {
            $mod_name::runtime_types::pallet_quorum::pallet::Call::eval_proposal_state { proposal }
          }
          QuorumCall::SubmitPublicKeys { public_keys } => {
            $mod_name::runtime_types::pallet_quorum::pallet::Call::submit_public_keys { public_keys }
          }
        }
      }
    }

    impl From<OracleCall> for $mod_name::runtime_types::pallet_oracle::pallet::Call {
      fn from(current_call: OracleCall) -> $mod_name::runtime_types::pallet_oracle::pallet::Call {
        match current_call {
          OracleCall::ConfirmSwap {
            request_id,
            market_makers,
          } => $mod_name::runtime_types::pallet_oracle::pallet::Call::confirm_swap {
            request_id,
            market_makers,
          },
          OracleCall::CancelSwap { request_id } => {
            $mod_name::runtime_types::pallet_oracle::pallet::Call::cancel_swap { request_id }
          }
          OracleCall::SetAccountId { new_account_id } => {
            $mod_name::runtime_types::pallet_oracle::pallet::Call::set_account_id { new_account_id }
          }
          OracleCall::SetStatus { is_enabled } => {
            $mod_name::runtime_types::pallet_oracle::pallet::Call::set_status { is_enabled }
          }
          OracleCall::UpdateAssetsValue { value } => {
            $mod_name::runtime_types::pallet_oracle::pallet::Call::update_assets_value { value }
          }
          OracleCall::AddMarketMaker { account_id } => {
            $mod_name::runtime_types::pallet_oracle::pallet::Call::add_market_maker { account_id }
          }
          OracleCall::RemoveMarketMaker { account_id } => {
            $mod_name::runtime_types::pallet_oracle::pallet::Call::remove_market_maker { account_id }
          }
        }
      }
    }

    impl From<StakingCall> for $mod_name::runtime_types::pallet_staking::pallet::pallet::Call {
      fn from(current_call: StakingCall) -> $mod_name::runtime_types::pallet_staking::pallet::pallet::Call {
        match current_call {
          StakingCall::Bond {
            controller,
            value,
            payee,
          } => $mod_name::runtime_types::pallet_staking::pallet::pallet::Call::bond {
            controller,
            value,
            payee: payee.into()
          },
          StakingCall::Nominate {
            targets,
          } => $mod_name::runtime_types::pallet_staking::pallet::pallet::Call::nominate {
            targets
          },
        }
      }
    }

    impl<AccountId> From<RewardDestination> for $mod_name::runtime_types::pallet_staking::RewardDestination<AccountId> {
      fn from(
        dest: RewardDestination,
      ) -> $mod_name::runtime_types::pallet_staking::RewardDestination<AccountId> {
        match dest {
          RewardDestination::Controller => {
            $mod_name::runtime_types::pallet_staking::RewardDestination::Controller
          }
        }
      }
    }

  }
}

#[derive(Clone)]
pub enum TidefiRuntime {
  #[cfg(feature = "tidechain-native")]
  Tidechain(Arc<TidechainRuntime>),
  #[cfg(feature = "lagoon-native")]
  Lagoon(Arc<LagoonRuntime>),
}

impl std::fmt::Display for TidefiRuntime {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      #[cfg(feature = "tidechain-native")]
      TidefiRuntime::Tidechain(_) => write!(f, "Tidechain"),
      #[cfg(feature = "lagoon-native")]
      TidefiRuntime::Lagoon(_) => write!(f, "Lagoon"),
    }
  }
}

#[cfg(feature = "tidechain-native")]
pub struct TidechainRuntime {
  storage: tidechain::StorageApi,
  constants: tidechain::ConstantsApi,
  tx: tidechain::TransactionApi,
}

#[cfg(feature = "tidechain-native")]
impl TidechainRuntime {
  pub fn storage(&self) -> &tidechain::StorageApi {
    &self.storage
  }
  pub fn constants(&self) -> &tidechain::ConstantsApi {
    &self.constants
  }
  pub fn tx(&self) -> &tidechain::TransactionApi {
    &self.tx
  }
}

#[cfg(feature = "tidechain-native")]
impl Default for TidechainRuntime {
  fn default() -> Self {
    Self {
      storage: tidechain::storage(),
      constants: tidechain::constants(),
      tx: tidechain::tx(),
    }
  }
}

#[cfg(feature = "lagoon-native")]
pub struct LagoonRuntime {
  storage: lagoon::StorageApi,
  constants: lagoon::ConstantsApi,
  tx: lagoon::TransactionApi,
}

#[cfg(feature = "lagoon-native")]
impl LagoonRuntime {
  pub fn storage(&self) -> &lagoon::StorageApi {
    &self.storage
  }
  pub fn constants(&self) -> &lagoon::ConstantsApi {
    &self.constants
  }
  pub fn tx(&self) -> &lagoon::TransactionApi {
    &self.tx
  }
}

#[cfg(feature = "lagoon-native")]
impl Default for LagoonRuntime {
  fn default() -> Self {
    Self {
      storage: lagoon::storage(),
      constants: lagoon::constants(),
      tx: lagoon::tx(),
    }
  }
}

/// Custom [`Config`] implementation with Clone
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
  type ExtrinsicParams = <PolkadotConfig as Config>::ExtrinsicParams;
}

/// Simple batch calls
pub enum TidechainCall {
  Tidefi(TidefiCall),
  Staking(StakingCall),
  Quorum(QuorumCall),
  Oracle(OracleCall),
}

pub enum TidefiCall {
  Transfer {
    destination_id: AccountId,
    currency_id: CurrencyId,
    amount: Balance,
  },
  Withdrawal {
    currency_id: CurrencyId,
    amount: Balance,
    external_address: Vec<u8>,
  },
  Swap {
    currency_id_from: CurrencyId,
    amount_from: Balance,
    currency_id_to: CurrencyId,
    amount_to: Balance,
    swap_type: SwapType,
    slippage_tolerance: Option<Permill>,
  },
  CancelSwap {
    request_id: Hash,
  },
  ClaimSunriseRewards {
    era_index: u32,
  },
}

pub enum StakingCall {
  Bond {
    controller: MultiAddress<AccountId, u32>,
    value: Balance,
    payee: RewardDestination,
  },
  Nominate {
    targets: Vec<MultiAddress<AccountId, u32>>,
  },
}

pub enum QuorumCall {
  SubmitProposal {
    proposal: tidefi_primitives::ProposalType<AccountId, u32, Vec<u8>, Vec<AccountId>>,
  },
  AcknowledgeProposal {
    proposal: Hash,
  },
  AcknowledgeBurned {
    proposal: Hash,
  },
  RejectProposal {
    proposal: Hash,
  },
  EvalProposalState {
    proposal: Hash,
  },
  SubmitPublicKeys {
    public_keys: Vec<(u32, Vec<u8>)>,
  },
}

pub enum OracleCall {
  ConfirmSwap {
    request_id: Hash,
    market_makers: Vec<tidefi_primitives::SwapConfirmation>,
  },
  CancelSwap {
    request_id: Hash,
  },
  SetAccountId {
    new_account_id: AccountId,
  },
  SetStatus {
    is_enabled: bool,
  },
  UpdateAssetsValue {
    value: Vec<(u32, u128)>,
  },
  AddMarketMaker {
    account_id: AccountId,
  },
  RemoveMarketMaker {
    account_id: AccountId,
  },
}

pub enum RewardDestination {
  Controller,
}

with_config! {
  tidechain,
  tidechain_runtime,
  "../artifacts/tidechain_metadata.scale"
}

with_config! {
  lagoon,
  lagoon_runtime,
  "../artifacts/lagoon_metadata.scale"
}
