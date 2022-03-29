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
