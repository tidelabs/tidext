use super::{Runtime, SortRuntime};
use quote::format_ident;

pub fn expand(item_mod: syn::ItemMod, runtimes: Vec<Runtime>) -> proc_macro2::TokenStream {
  let enum_ident = format_ident!("{}", "TidefiRuntime");
  let mod_ident = item_mod.ident;

  // all enum items, example: `Tidechain6040(Arc<Tidechain6040Runtime>)`
  let all_items = runtimes.iter().map(|runtime| {
    let enum_item = &runtime.enum_name;
    let feature = format!("{}-native", &runtime.spec_name);
    quote::quote!(
      #[cfg(feature = #feature)]
      #enum_item(std::sync::Arc<#enum_item>)
    )
  });

  // all enum items, initiated
  let all_runtimes = runtimes.iter().map(|runtime| {
    let enum_item = &runtime.enum_name;
    let feature = format!("{}-native", &runtime.spec_name);
    quote::quote!(
      #[cfg(feature = #feature)]
      #enum_ident::#enum_item(Default::default())
    )
  });

  // create an alias `pub tidechain6030 as tidechain`;
  // to be used mainly to match events
  let get_first_runtime = |runtime_name: &str| {
    let all_runtimes = runtimes.sort_desc();
    let found_runtime = all_runtimes
      .iter()
      .find(|r| r.spec_name == runtime_name.to_string());
    if let Some(found_runtime) = found_runtime {
      let runtime_name_ident = format_ident!("{}", runtime_name);
      let mod_name = &found_runtime.mod_name;
      let feature = format!("{}-native", &found_runtime.spec_name);
      quote::quote!(
        #[cfg(feature = #feature)]
        pub use #mod_name as #runtime_name_ident;
      )
    } else {
      quote::quote!()
    }
  };

  let first_tidechain_alias = get_first_runtime("tidechain");
  let first_lagoon_alias = get_first_runtime("lagoon");

  // map the enum item to the spec name
  let spec_name = runtimes.iter().map(|runtime| {
    let enum_item = &runtime.enum_name;
    let feature = format!("{}-native", &runtime.spec_name);
    quote::quote!(
      #[cfg(feature = #feature)]
      #enum_ident::#enum_item(r) => r.spec_name()
    )
  });

  // map the enum item to the spec version
  let spec_version = runtimes.iter().map(|runtime| {
    let enum_item = &runtime.enum_name;
    let feature = format!("{}-native", &runtime.spec_name);
    quote::quote!(
      #[cfg(feature = #feature)]
      #enum_ident::#enum_item(r) => r.spec_version()
    )
  });

  // map the enum item to the id
  let ids = runtimes.iter().map(|runtime| {
    let enum_item = &runtime.enum_name;
    let feature = format!("{}-native", &runtime.spec_name);
    quote::quote!(
      #[cfg(feature = #feature)]
      #enum_ident::#enum_item(r) => r.id()
    )
  });

  // map the enum item to the id
  let fmt_display = runtimes.iter().map(|runtime| {
    let enum_item = &runtime.enum_name;
    let feature = format!("{}-native", &runtime.spec_name);
    quote::quote!(
      #[cfg(feature = #feature)]
      #enum_ident::#enum_item(r) => write!(f, "{}", r)
    )
  });

  // all runtimes without feature flag, for external use when lagoon and tidechain are enabled
  let macro_def_no_features = runtimes.iter().map(|runtime| {
    let enum_item = &runtime.enum_name;
    quote::quote!(
      $crate::#enum_ident::#enum_item($client) => { $( $code )* }
    )
  });

  // all runtimes without tidechain
  let macro_def_no_tidechain = runtimes
    .iter()
    .filter(|runtime| runtime.spec_name != "tidechain".to_string())
    .map(|runtime| {
      let enum_item = &runtime.enum_name;
      quote::quote!(
        $crate::#enum_ident::#enum_item($client) => { $( $code )* }
      )
    });

  // all runtimes without lagoon
  let macro_def_no_lagoon = runtimes
    .iter()
    .filter(|runtime| runtime.spec_name != "lagoon".to_string())
    .map(|runtime| {
      let enum_item = &runtime.enum_name;
      quote::quote!(
        $crate::#enum_ident::#enum_item($client) => { $( $code )* }
      )
    });

  let subxt_impl = runtimes.iter().map(|runtime| {
    let metadata_path = &runtime.runtime_metadata_path;
    let mod_name = &runtime.mod_name;
    let runtime_name = &runtime.runtime_name;
    let feature = format!("{}-native", &runtime.spec_name);

    quote::quote!(
      #[cfg(feature = #feature)]
      #[subxt::subxt(runtime_metadata_path = #metadata_path)]
      pub mod #mod_name {
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
        #[subxt(substitute_type = "tidefi_primitives::swap::swap::SwapConfirmation")]
        use ::tidefi_primitives::SwapConfirmation;
        #[subxt(substitute_type = "tidefi_primitives::swap::swap::SwapStatus")]
        use ::tidefi_primitives::SwapStatus;
        #[subxt(substitute_type = "tidefi_primitives::swap::swap::SwapType")]
        // required for 6030
        use ::tidefi_primitives::SwapType;
        #[subxt(substitute_type = "tidefi_primitives::SwapConfirmation")]
        use ::tidefi_primitives::SwapConfirmation;
        #[subxt(substitute_type = "tidefi_primitives::SwapStatus")]
        use ::tidefi_primitives::SwapStatus;
        #[subxt(substitute_type = "tidefi_primitives::SwapType")]
        use ::tidefi_primitives::SwapType;
        // -----
        #[subxt(substitute_type = "tidefi_primitives::Withdrawal")]
        use ::tidefi_primitives::Withdrawal;
      }

      #[cfg(feature = #feature)]
      impl From<TidechainCall> for #mod_name::runtime_types::#runtime_name::Call {
        fn from(current_call: TidechainCall) -> #mod_name::runtime_types::#runtime_name::Call {
          match current_call {
            TidechainCall::Assets(assets_call) => {
              #mod_name::runtime_types::#runtime_name::Call::Assets(assets_call.into())
            }
            TidechainCall::Balances(balances_call) => {
              #mod_name::runtime_types::#runtime_name::Call::Balances(balances_call.into())
            }
            TidechainCall::Tidefi(tidefi_call) => {
              #mod_name::runtime_types::#runtime_name::Call::Tidefi(tidefi_call.into())
            }
            TidechainCall::Quorum(quorum_call) => {
              #mod_name::runtime_types::#runtime_name::Call::Quorum(quorum_call.into())
            }
            TidechainCall::Oracle(oracle_call) => {
              #mod_name::runtime_types::#runtime_name::Call::Oracle(oracle_call.into())
            }
            TidechainCall::Staking(staking_call) => {
              #mod_name::runtime_types::#runtime_name::Call::Staking(staking_call.into())
            }
          }
        }
      }

      #[cfg(feature = #feature)]
      impl From<TidefiCall> for #mod_name::runtime_types::pallet_tidefi::pallet::Call {
        fn from(current_call: TidefiCall) -> #mod_name::runtime_types::pallet_tidefi::pallet::Call {
          match current_call {
            TidefiCall::Transfer {
              destination_id,
              currency_id,
              amount,
            } => #mod_name::runtime_types::pallet_tidefi::pallet::Call::transfer {
              destination_id,
              currency_id,
              amount,
            },
            TidefiCall::Withdrawal {
              currency_id,
              amount,
              external_address,
            } => #mod_name::runtime_types::pallet_tidefi::pallet::Call::withdrawal {
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
            } => #mod_name::runtime_types::pallet_tidefi::pallet::Call::swap {
              currency_id_from,
              amount_from,
              currency_id_to,
              amount_to,
              swap_type,
              slippage_tolerance,
            },
            TidefiCall::CancelSwap { request_id } => {
              #mod_name::runtime_types::pallet_tidefi::pallet::Call::cancel_swap { request_id }
            }
            TidefiCall::ClaimSunriseRewards { era_index } => {
              #mod_name::runtime_types::pallet_tidefi::pallet::Call::claim_sunrise_rewards { era_index }
            }
          }
        }
      }

      #[cfg(feature = #feature)]
      impl From<QuorumCall> for #mod_name::runtime_types::pallet_quorum::pallet::Call {
        fn from(current_call: QuorumCall) -> #mod_name::runtime_types::pallet_quorum::pallet::Call {
          match current_call {
            QuorumCall::SubmitProposal { proposal } => {
              #mod_name::runtime_types::pallet_quorum::pallet::Call::submit_proposal { proposal }
            }
            QuorumCall::AcknowledgeProposal { proposal } => {
              #mod_name::runtime_types::pallet_quorum::pallet::Call::acknowledge_proposal { proposal }
            }
            QuorumCall::AcknowledgeBurned { proposal } => {
              #mod_name::runtime_types::pallet_quorum::pallet::Call::acknowledge_burned { proposal }
            }
            QuorumCall::RejectProposal { proposal } => {
              #mod_name::runtime_types::pallet_quorum::pallet::Call::reject_proposal { proposal }
            }
            QuorumCall::EvalProposalState { proposal } => {
              #mod_name::runtime_types::pallet_quorum::pallet::Call::eval_proposal_state { proposal }
            }
            QuorumCall::SubmitPublicKeys { public_keys } => {
              #mod_name::runtime_types::pallet_quorum::pallet::Call::submit_public_keys { public_keys }
            }
          }
        }
      }

      #[cfg(feature = #feature)]
      impl From<OracleCall> for #mod_name::runtime_types::pallet_oracle::pallet::Call {
        fn from(current_call: OracleCall) -> #mod_name::runtime_types::pallet_oracle::pallet::Call {
          match current_call {
            OracleCall::ConfirmSwap {
              request_id,
              market_makers,
            } => #mod_name::runtime_types::pallet_oracle::pallet::Call::confirm_swap {
              request_id,
              market_makers,
            },
            OracleCall::CancelSwap { request_id } => {
              #mod_name::runtime_types::pallet_oracle::pallet::Call::cancel_swap { request_id }
            }
            OracleCall::SetAccountId { new_account_id } => {
              #mod_name::runtime_types::pallet_oracle::pallet::Call::set_account_id { new_account_id }
            }
            OracleCall::SetStatus { is_enabled } => {
              #mod_name::runtime_types::pallet_oracle::pallet::Call::set_status { is_enabled }
            }
            OracleCall::UpdateAssetsValue { value } => {
              #mod_name::runtime_types::pallet_oracle::pallet::Call::update_assets_value { value }
            }
            OracleCall::AddMarketMaker { account_id } => {
              #mod_name::runtime_types::pallet_oracle::pallet::Call::add_market_maker { account_id }
            }
            OracleCall::RemoveMarketMaker { account_id } => {
              #mod_name::runtime_types::pallet_oracle::pallet::Call::remove_market_maker { account_id }
            }
          }
        }
      }

      #[cfg(feature = #feature)]
      impl From<StakingCall> for #mod_name::runtime_types::pallet_staking::pallet::pallet::Call {
        fn from(current_call: StakingCall) -> #mod_name::runtime_types::pallet_staking::pallet::pallet::Call {
          match current_call {
            StakingCall::Bond {
              controller,
              value,
              payee,
            } => #mod_name::runtime_types::pallet_staking::pallet::pallet::Call::bond {
              controller,
              value,
              payee: payee.into()
            },
            StakingCall::Nominate {
              targets,
            } => #mod_name::runtime_types::pallet_staking::pallet::pallet::Call::nominate {
              targets
            },
          }
        }
      }

      #[cfg(feature = #feature)]
      impl From<AssetsCall> for #mod_name::runtime_types::pallet_assets::pallet::Call {
        fn from(current_call: AssetsCall) -> #mod_name::runtime_types::pallet_assets::pallet::Call {
          match current_call {
            AssetsCall::Mint { id, beneficiary, amount } => {
              #mod_name::runtime_types::pallet_assets::pallet::Call::mint { id, beneficiary: sp_runtime::MultiAddress::Id(beneficiary), amount }
            }
          }
        }
      }

      #[cfg(feature = #feature)]
      impl From<BalancesCall> for #mod_name::runtime_types::pallet_balances::pallet::Call {
        fn from(current_call: BalancesCall) -> #mod_name::runtime_types::pallet_balances::pallet::Call {
          match current_call {
            BalancesCall::SetBalance { who, new_free, new_reserved } => {
              #mod_name::runtime_types::pallet_balances::pallet::Call::set_balance { who: sp_runtime::MultiAddress::Id(who), new_free, new_reserved }
            }
          }
        }
      }

      #[cfg(feature = #feature)]
      impl From<RewardDestination> for #mod_name::runtime_types::pallet_staking::RewardDestination<sp_runtime::AccountId32> {
        fn from(
          dest: RewardDestination,
        ) -> #mod_name::runtime_types::pallet_staking::RewardDestination<sp_runtime::AccountId32> {
          match dest {
            RewardDestination::Staked => {
              #mod_name::runtime_types::pallet_staking::RewardDestination::Staked
            }
            RewardDestination::Stash => {
              #mod_name::runtime_types::pallet_staking::RewardDestination::Stash
            }
            RewardDestination::Controller => {
              #mod_name::runtime_types::pallet_staking::RewardDestination::Controller
            }
            RewardDestination::Account(account) => {
              #mod_name::runtime_types::pallet_staking::RewardDestination::Account(sp_runtime::AccountId32::new(account.into()))
            }
            RewardDestination::None => {
              #mod_name::runtime_types::pallet_staking::RewardDestination::None
            }
          }
        }
      }

      #[cfg(feature = #feature)]
      impl From<#mod_name::runtime_types::pallet_staking::RewardDestination<sp_runtime::AccountId32>> for RewardDestination {
        fn from(
          dest: #mod_name::runtime_types::pallet_staking::RewardDestination<sp_runtime::AccountId32>,
        ) -> RewardDestination {
          match dest {
            #mod_name::runtime_types::pallet_staking::RewardDestination::Staked => {
              RewardDestination::Staked
            }
            #mod_name::runtime_types::pallet_staking::RewardDestination::Stash => {
              RewardDestination::Stash
            }
            #mod_name::runtime_types::pallet_staking::RewardDestination::Controller => {
              RewardDestination::Controller
            }
            #mod_name::runtime_types::pallet_staking::RewardDestination::Account(account) => {
              RewardDestination::Account(account.into())
            }
            #mod_name::runtime_types::pallet_staking::RewardDestination::None => {
              RewardDestination::None
            }
          }
        }
      }
    )
  });

  // all tidext runtimes implementation
  let runtimes = runtimes.iter().map(|runtime| {
    let impl_name = &runtime.enum_name;
    let impl_name_string = runtime.enum_name.to_string();
    let mod_name = &runtime.mod_name;
    let spec_name = &runtime.spec_name;
    let version = &runtime.spec_version;
    let feature = format!("{}-native", &runtime.spec_name);

    quote::quote!(
      #[cfg(feature = #feature)]
      pub struct #impl_name {
        storage: #mod_name::StorageApi,
        constants: #mod_name::ConstantsApi,
        tx: #mod_name::TransactionApi,
      }

      #[cfg(feature = #feature)]
      impl #impl_name {
        pub fn storage(&self) -> &#mod_name::StorageApi {
          &self.storage
        }
        pub fn constants(&self) -> &#mod_name::ConstantsApi {
          &self.constants
        }
        pub fn tx(&self) -> &#mod_name::TransactionApi {
          &self.tx
        }
        pub fn spec_name(&self) -> String {
          #spec_name.to_string()
        }
        pub fn spec_version(&self) -> u32 {
          #version
        }
        pub fn id(&self) -> String {
          #impl_name_string.to_string()
        }
      }

      #[cfg(feature = #feature)]
      impl Default for #impl_name {
        fn default() -> Self {
          Self {
            storage: #mod_name::storage(),
            constants: #mod_name::constants(),
            tx: #mod_name::tx(),
          }
        }
      }

      #[cfg(feature = #feature)]
      impl std::fmt::Display for #impl_name {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
          write!(f, "{}", self.id())
        }
      }

      #[cfg(feature = #feature)]
      impl std::fmt::Debug for #impl_name {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
          f.debug_struct(&self.id())
            .field("spec_name", &self.spec_name())
            .field("spec_version", &self.spec_version())
            .finish()
        }
      }
    )
  });

  quote::quote!(
  pub mod #mod_ident {
    #![allow(clippy::too_many_arguments)]

    #first_tidechain_alias
    #first_lagoon_alias

    #[derive(Debug, Clone)]
    pub enum #enum_ident {
      #(#all_items),*
    }

    impl #enum_ident {
      pub fn runtimes() -> Vec<#enum_ident>  {
        vec![#(#all_runtimes),*]
      }

      pub fn select_runtime(spec_name: impl ToString, spec_version: u32) -> #enum_ident  {
        let mut failover = #enum_ident::runtimes()
          .into_iter()
          .filter(|x| x.spec_name() == spec_name.to_string())
          .collect::<Vec<#enum_ident>>();
        let mut runtimes = #enum_ident::runtimes()
          .into_iter()
          .filter(|x| x.spec_name() == spec_name.to_string() && x.spec_version() <= spec_version)
          .collect::<Vec<#enum_ident>>();

        runtimes.sort_by(|a, b| {
          b.spec_version()
            .partial_cmp(&a.spec_version())
            .unwrap_or(std::cmp::Ordering::Equal)
        });

        failover.sort_by(|a, b| {
          a.spec_version()
            .partial_cmp(&b.spec_version())
            .unwrap_or(std::cmp::Ordering::Equal)
        });

        runtimes.first().cloned().unwrap_or(
          failover.first().cloned().expect("at least one runtime")
        )
      }

      pub fn spec_name(&self) -> String  {
        match self {
          #(#spec_name),*
        }
      }

      pub fn spec_version(&self) -> u32  {
        match self {
          #(#spec_version),*
        }
      }

      pub fn id(&self) -> String  {
        match self {
          #(#ids),*
        }
      }
    }

    impl std::fmt::Display for #enum_ident {
      fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
          #(#fmt_display),*
        }
      }
    }

    #[macro_export]
    #[cfg(all(feature = "tidechain-native", feature = "lagoon-native"))]
    macro_rules! __with_tidext_runtime {
      {
        $self:ident,
        $client:ident,
        {
          $( $code:tt )*
        }
      } => {
        match $self.runtime_type() {
          #(#macro_def_no_features),*
        }
      }
    }

    #[macro_export]
    #[cfg(not(feature = "tidechain-native"))]
    macro_rules! __with_tidext_runtime {
      {
        $self:ident,
        $client:ident,
        {
          $( $code:tt )*
        }
      } => {
        match $self.runtime_type() {
          #(#macro_def_no_tidechain),*
        }
      }
    }

    #[macro_export]
    #[cfg(not(feature = "lagoon-native"))]
    macro_rules! __with_tidext_runtime {
      {
        $self:ident,
        $client:ident,
        {
          $( $code:tt )*
        }
      } => {
        match $self.runtime_type() {
          #(#macro_def_no_lagoon),*
        }
      }
    }

    #[doc(hidden)]
    pub use __with_tidext_runtime as with_runtime;

    pub enum TidechainCall {
        Assets(AssetsCall),
        Balances(BalancesCall),
        Tidefi(TidefiCall),
        Staking(StakingCall),
        Quorum(QuorumCall),
        Oracle(OracleCall),
      }

      pub enum TidefiCall {
        Transfer {
          destination_id: tidefi_primitives::AccountId,
          currency_id: tidefi_primitives::CurrencyId,
          amount: tidefi_primitives::Balance,
        },
        Withdrawal {
          currency_id: tidefi_primitives::CurrencyId,
          amount: tidefi_primitives::Balance,
          external_address: Vec<u8>,
        },
        Swap {
          currency_id_from: tidefi_primitives::CurrencyId,
          amount_from: tidefi_primitives::Balance,
          currency_id_to: tidefi_primitives::CurrencyId,
          amount_to: tidefi_primitives::Balance,
          swap_type: tidefi_primitives::SwapType,
          slippage_tolerance: Option<sp_runtime::Permill>,
        },
        CancelSwap {
          request_id: tidefi_primitives::Hash,
        },
        ClaimSunriseRewards {
          era_index: u32,
        },
      }

      pub enum StakingCall {
        Bond {
          controller: sp_runtime::MultiAddress<tidefi_primitives::AccountId, u32>,
          value: tidefi_primitives::Balance,
          payee: RewardDestination,
        },
        Nominate {
          targets: Vec<sp_runtime::MultiAddress<tidefi_primitives::AccountId, u32>>,
        },
      }

      pub enum QuorumCall {
        SubmitProposal {
          proposal: tidefi_primitives::ProposalType<tidefi_primitives::AccountId, u32, Vec<u8>, Vec<tidefi_primitives::AccountId>>,
        },
        AcknowledgeProposal {
          proposal: tidefi_primitives::Hash,
        },
        AcknowledgeBurned {
          proposal: tidefi_primitives::Hash,
        },
        RejectProposal {
          proposal: tidefi_primitives::Hash,
        },
        EvalProposalState {
          proposal: tidefi_primitives::Hash,
        },
        SubmitPublicKeys {
          public_keys: Vec<(u32, Vec<u8>)>,
        },
      }

      pub enum OracleCall {
        ConfirmSwap {
          request_id: tidefi_primitives::Hash,
          market_makers: Vec<tidefi_primitives::SwapConfirmation>,
        },
        CancelSwap {
          request_id: tidefi_primitives::Hash,
        },
        SetAccountId {
          new_account_id: tidefi_primitives::AccountId,
        },
        SetStatus {
          is_enabled: bool,
        },
        UpdateAssetsValue {
          value: Vec<(u32, u128)>,
        },
        AddMarketMaker {
          account_id: tidefi_primitives::AccountId,
        },
        RemoveMarketMaker {
          account_id: tidefi_primitives::AccountId,
        },
      }

      pub enum AssetsCall {
        Mint {
          id: u32,
          beneficiary: tidefi_primitives::AccountId,
          amount: tidefi_primitives::Balance,
        },
      }

      pub enum BalancesCall {
        SetBalance {
          who: tidefi_primitives::AccountId,
          new_free: tidefi_primitives::Balance,
          new_reserved: tidefi_primitives::Balance,
        },
      }

      /// A destination account for payment
      #[derive(Clone, Debug, Eq, PartialEq)]
      pub enum RewardDestination {
        /// Pay into the stash account, increasing the amount at stake accordingly
        Staked,
        /// Pay into the stash account, not increasing the amount at stake
        Stash,
        /// Pay into the controller account
        Controller,
        /// Pay into a specified account
        Account(tidefi_primitives::AccountId),
        /// Receive no reward
        None,
      }

      #(#runtimes)*
      #(#subxt_impl)*
    }
  )
}
