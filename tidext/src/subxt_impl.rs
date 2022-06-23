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

use subxt::{Config, DefaultConfig};

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
}

#[subxt::subxt(runtime_metadata_path = "res/tidechain_metadata.scale")]
pub mod tidechain {
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
