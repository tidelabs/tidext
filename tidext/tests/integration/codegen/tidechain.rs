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

#[allow(dead_code, unused_imports, non_camel_case_types)]
pub mod api {
  use super::api as root_mod;
  #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
  pub enum Event {
    #[codec(index = 0)]
    System(system::Event),
    #[codec(index = 3)]
    Indices(indices::Event),
    #[codec(index = 4)]
    Balances(balances::Event),
    #[codec(index = 7)]
    Staking(staking::Event),
    #[codec(index = 8)]
    Offences(offences::Event),
    #[codec(index = 10)]
    Session(session::Event),
    #[codec(index = 11)]
    Grandpa(grandpa::Event),
    #[codec(index = 12)]
    ImOnline(im_online::Event),
    #[codec(index = 14)]
    Council(council::Event),
    #[codec(index = 15)]
    TechnicalCommittee(technical_committee::Event),
    #[codec(index = 16)]
    Elections(elections::Event),
    #[codec(index = 17)]
    TechnicalMembership(technical_membership::Event),
    #[codec(index = 18)]
    Treasury(treasury::Event),
    #[codec(index = 19)]
    Utility(utility::Event),
    #[codec(index = 20)]
    Identity(identity::Event),
    #[codec(index = 21)]
    ElectionProviderMultiPhase(election_provider_multi_phase::Event),
    #[codec(index = 22)]
    Recovery(recovery::Event),
    #[codec(index = 23)]
    Scheduler(scheduler::Event),
    #[codec(index = 24)]
    Proxy(proxy::Event),
    #[codec(index = 25)]
    Multisig(multisig::Event),
    #[codec(index = 26)]
    Bounties(bounties::Event),
    #[codec(index = 27)]
    Assets(assets::Event),
    #[codec(index = 28)]
    BagsList(bags_list::Event),
    #[codec(index = 29)]
    Preimage(preimage::Event),
    #[codec(index = 30)]
    Sudo(sudo::Event),
    #[codec(index = 50)]
    Tidefi(tidefi::Event),
    #[codec(index = 51)]
    TidefiStaking(tidefi_staking::Event),
    #[codec(index = 52)]
    Quorum(quorum::Event),
    #[codec(index = 53)]
    Oracle(oracle::Event),
    #[codec(index = 54)]
    Security(security::Event),
    #[codec(index = 55)]
    Fees(fees::Event),
    #[codec(index = 56)]
    AssetRegistry(asset_registry::Event),
  }
  pub mod system {
    use super::root_mod;
    use super::runtime_types;
    pub mod calls {
      use super::root_mod;
      use super::runtime_types;
      type DispatchError = runtime_types::sp_runtime::DispatchError;
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct FillBlock {
        pub ratio: runtime_types::sp_arithmetic::per_things::Perbill,
      }
      impl ::subxt::Call for FillBlock {
        const PALLET: &'static str = "System";
        const FUNCTION: &'static str = "fill_block";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Remark {
        pub remark: ::std::vec::Vec<::core::primitive::u8>,
      }
      impl ::subxt::Call for Remark {
        const PALLET: &'static str = "System";
        const FUNCTION: &'static str = "remark";
      }
      #[derive(
        :: subxt :: codec :: Encode,
        :: subxt :: codec :: Decode,
        Debug,
        :: subxt :: codec :: CompactAs,
      )]
      pub struct SetHeapPages {
        pub pages: ::core::primitive::u64,
      }
      impl ::subxt::Call for SetHeapPages {
        const PALLET: &'static str = "System";
        const FUNCTION: &'static str = "set_heap_pages";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct SetCode {
        pub code: ::std::vec::Vec<::core::primitive::u8>,
      }
      impl ::subxt::Call for SetCode {
        const PALLET: &'static str = "System";
        const FUNCTION: &'static str = "set_code";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct SetCodeWithoutChecks {
        pub code: ::std::vec::Vec<::core::primitive::u8>,
      }
      impl ::subxt::Call for SetCodeWithoutChecks {
        const PALLET: &'static str = "System";
        const FUNCTION: &'static str = "set_code_without_checks";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct SetStorage {
        pub items: ::std::vec::Vec<(
          ::std::vec::Vec<::core::primitive::u8>,
          ::std::vec::Vec<::core::primitive::u8>,
        )>,
      }
      impl ::subxt::Call for SetStorage {
        const PALLET: &'static str = "System";
        const FUNCTION: &'static str = "set_storage";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct KillStorage {
        pub keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
      }
      impl ::subxt::Call for KillStorage {
        const PALLET: &'static str = "System";
        const FUNCTION: &'static str = "kill_storage";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct KillPrefix {
        pub prefix: ::std::vec::Vec<::core::primitive::u8>,
        pub subkeys: ::core::primitive::u32,
      }
      impl ::subxt::Call for KillPrefix {
        const PALLET: &'static str = "System";
        const FUNCTION: &'static str = "kill_prefix";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct RemarkWithEvent {
        pub remark: ::std::vec::Vec<::core::primitive::u8>,
      }
      impl ::subxt::Call for RemarkWithEvent {
        const PALLET: &'static str = "System";
        const FUNCTION: &'static str = "remark_with_event";
      }
      pub struct TransactionApi<'a, T: ::subxt::Config, X> {
        client: &'a ::subxt::Client<T>,
        marker: ::core::marker::PhantomData<X>,
      }
      impl<'a, T, X> TransactionApi<'a, T, X>
      where
        T: ::subxt::Config,
        X: ::subxt::extrinsic::ExtrinsicParams<T>,
      {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self {
            client,
            marker: ::core::marker::PhantomData,
          }
        }
        pub fn fill_block(
          &self,
          ratio: runtime_types::sp_arithmetic::per_things::Perbill,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, FillBlock, DispatchError, root_mod::Event>
        {
          let call = FillBlock { ratio };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn remark(
          &self,
          remark: ::std::vec::Vec<::core::primitive::u8>,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, Remark, DispatchError, root_mod::Event>
        {
          let call = Remark { remark };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn set_heap_pages(
          &self,
          pages: ::core::primitive::u64,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, SetHeapPages, DispatchError, root_mod::Event>
        {
          let call = SetHeapPages { pages };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn set_code(
          &self,
          code: ::std::vec::Vec<::core::primitive::u8>,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, SetCode, DispatchError, root_mod::Event>
        {
          let call = SetCode { code };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn set_code_without_checks(
          &self,
          code: ::std::vec::Vec<::core::primitive::u8>,
        ) -> ::subxt::SubmittableExtrinsic<
          'a,
          T,
          X,
          SetCodeWithoutChecks,
          DispatchError,
          root_mod::Event,
        > {
          let call = SetCodeWithoutChecks { code };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn set_storage(
          &self,
          items: ::std::vec::Vec<(
            ::std::vec::Vec<::core::primitive::u8>,
            ::std::vec::Vec<::core::primitive::u8>,
          )>,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, SetStorage, DispatchError, root_mod::Event>
        {
          let call = SetStorage { items };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn kill_storage(
          &self,
          keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, KillStorage, DispatchError, root_mod::Event>
        {
          let call = KillStorage { keys };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn kill_prefix(
          &self,
          prefix: ::std::vec::Vec<::core::primitive::u8>,
          subkeys: ::core::primitive::u32,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, KillPrefix, DispatchError, root_mod::Event>
        {
          let call = KillPrefix { prefix, subkeys };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn remark_with_event(
          &self,
          remark: ::std::vec::Vec<::core::primitive::u8>,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, RemarkWithEvent, DispatchError, root_mod::Event>
        {
          let call = RemarkWithEvent { remark };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
      }
    }
    pub type Event = runtime_types::frame_system::pallet::Event;
    pub mod events {
      use super::runtime_types;
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct ExtrinsicSuccess {
        pub dispatch_info: runtime_types::frame_support::weights::DispatchInfo,
      }
      impl ::subxt::Event for ExtrinsicSuccess {
        const PALLET: &'static str = "System";
        const EVENT: &'static str = "ExtrinsicSuccess";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct ExtrinsicFailed {
        pub dispatch_error: runtime_types::sp_runtime::DispatchError,
        pub dispatch_info: runtime_types::frame_support::weights::DispatchInfo,
      }
      impl ::subxt::Event for ExtrinsicFailed {
        const PALLET: &'static str = "System";
        const EVENT: &'static str = "ExtrinsicFailed";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct CodeUpdated;
      impl ::subxt::Event for CodeUpdated {
        const PALLET: &'static str = "System";
        const EVENT: &'static str = "CodeUpdated";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct NewAccount {
        pub account: ::subxt::sp_core::crypto::AccountId32,
      }
      impl ::subxt::Event for NewAccount {
        const PALLET: &'static str = "System";
        const EVENT: &'static str = "NewAccount";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct KilledAccount {
        pub account: ::subxt::sp_core::crypto::AccountId32,
      }
      impl ::subxt::Event for KilledAccount {
        const PALLET: &'static str = "System";
        const EVENT: &'static str = "KilledAccount";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Remarked {
        pub sender: ::subxt::sp_core::crypto::AccountId32,
        pub hash: ::subxt::sp_core::H256,
      }
      impl ::subxt::Event for Remarked {
        const PALLET: &'static str = "System";
        const EVENT: &'static str = "Remarked";
      }
    }
    pub mod storage {
      use super::runtime_types;
      pub struct Account<'a>(pub &'a ::subxt::sp_core::crypto::AccountId32);
      impl ::subxt::StorageEntry for Account<'_> {
        const PALLET: &'static str = "System";
        const STORAGE: &'static str = "Account";
        type Value = runtime_types::frame_system::AccountInfo<
          ::core::primitive::u32,
          runtime_types::pallet_balances::AccountData<::core::primitive::u128>,
        >;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
            &self.0,
            ::subxt::StorageHasher::Blake2_128Concat,
          )])
        }
      }
      pub struct ExtrinsicCount;
      impl ::subxt::StorageEntry for ExtrinsicCount {
        const PALLET: &'static str = "System";
        const STORAGE: &'static str = "ExtrinsicCount";
        type Value = ::core::primitive::u32;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct BlockWeight;
      impl ::subxt::StorageEntry for BlockWeight {
        const PALLET: &'static str = "System";
        const STORAGE: &'static str = "BlockWeight";
        type Value =
          runtime_types::frame_support::weights::PerDispatchClass<::core::primitive::u64>;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct AllExtrinsicsLen;
      impl ::subxt::StorageEntry for AllExtrinsicsLen {
        const PALLET: &'static str = "System";
        const STORAGE: &'static str = "AllExtrinsicsLen";
        type Value = ::core::primitive::u32;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct BlockHash<'a>(pub &'a ::core::primitive::u32);
      impl ::subxt::StorageEntry for BlockHash<'_> {
        const PALLET: &'static str = "System";
        const STORAGE: &'static str = "BlockHash";
        type Value = ::subxt::sp_core::H256;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
            &self.0,
            ::subxt::StorageHasher::Twox64Concat,
          )])
        }
      }
      pub struct ExtrinsicData<'a>(pub &'a ::core::primitive::u32);
      impl ::subxt::StorageEntry for ExtrinsicData<'_> {
        const PALLET: &'static str = "System";
        const STORAGE: &'static str = "ExtrinsicData";
        type Value = ::std::vec::Vec<::core::primitive::u8>;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
            &self.0,
            ::subxt::StorageHasher::Twox64Concat,
          )])
        }
      }
      pub struct Number;
      impl ::subxt::StorageEntry for Number {
        const PALLET: &'static str = "System";
        const STORAGE: &'static str = "Number";
        type Value = ::core::primitive::u32;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct ParentHash;
      impl ::subxt::StorageEntry for ParentHash {
        const PALLET: &'static str = "System";
        const STORAGE: &'static str = "ParentHash";
        type Value = ::subxt::sp_core::H256;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct Digest;
      impl ::subxt::StorageEntry for Digest {
        const PALLET: &'static str = "System";
        const STORAGE: &'static str = "Digest";
        type Value = runtime_types::sp_runtime::generic::digest::Digest;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct Events;
      impl ::subxt::StorageEntry for Events {
        const PALLET: &'static str = "System";
        const STORAGE: &'static str = "Events";
        type Value = ::std::vec::Vec<
          runtime_types::frame_system::EventRecord<
            runtime_types::lagoon_runtime::Event,
            ::subxt::sp_core::H256,
          >,
        >;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct EventCount;
      impl ::subxt::StorageEntry for EventCount {
        const PALLET: &'static str = "System";
        const STORAGE: &'static str = "EventCount";
        type Value = ::core::primitive::u32;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct EventTopics<'a>(pub &'a ::subxt::sp_core::H256);
      impl ::subxt::StorageEntry for EventTopics<'_> {
        const PALLET: &'static str = "System";
        const STORAGE: &'static str = "EventTopics";
        type Value = ::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
            &self.0,
            ::subxt::StorageHasher::Blake2_128Concat,
          )])
        }
      }
      pub struct LastRuntimeUpgrade;
      impl ::subxt::StorageEntry for LastRuntimeUpgrade {
        const PALLET: &'static str = "System";
        const STORAGE: &'static str = "LastRuntimeUpgrade";
        type Value = runtime_types::frame_system::LastRuntimeUpgradeInfo;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct UpgradedToU32RefCount;
      impl ::subxt::StorageEntry for UpgradedToU32RefCount {
        const PALLET: &'static str = "System";
        const STORAGE: &'static str = "UpgradedToU32RefCount";
        type Value = ::core::primitive::bool;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct UpgradedToTripleRefCount;
      impl ::subxt::StorageEntry for UpgradedToTripleRefCount {
        const PALLET: &'static str = "System";
        const STORAGE: &'static str = "UpgradedToTripleRefCount";
        type Value = ::core::primitive::bool;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct ExecutionPhase;
      impl ::subxt::StorageEntry for ExecutionPhase {
        const PALLET: &'static str = "System";
        const STORAGE: &'static str = "ExecutionPhase";
        type Value = runtime_types::frame_system::Phase;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct StorageApi<'a, T: ::subxt::Config> {
        client: &'a ::subxt::Client<T>,
      }
      impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self { client }
        }
        pub async fn account(
          &self,
          _0: &::subxt::sp_core::crypto::AccountId32,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          runtime_types::frame_system::AccountInfo<
            ::core::primitive::u32,
            runtime_types::pallet_balances::AccountData<::core::primitive::u128>,
          >,
          ::subxt::BasicError,
        > {
          let entry = Account(_0);
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn account_iter(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Account<'a>>, ::subxt::BasicError>
        {
          self.client.storage().iter(hash).await
        }
        pub async fn extrinsic_count(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::core::option::Option<::core::primitive::u32>,
          ::subxt::BasicError,
        > {
          let entry = ExtrinsicCount;
          self.client.storage().fetch(&entry, hash).await
        }
        pub async fn block_weight(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          runtime_types::frame_support::weights::PerDispatchClass<::core::primitive::u64>,
          ::subxt::BasicError,
        > {
          let entry = BlockWeight;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn all_extrinsics_len(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::core::option::Option<::core::primitive::u32>,
          ::subxt::BasicError,
        > {
          let entry = AllExtrinsicsLen;
          self.client.storage().fetch(&entry, hash).await
        }
        pub async fn block_hash(
          &self,
          _0: &::core::primitive::u32,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::subxt::sp_core::H256, ::subxt::BasicError> {
          let entry = BlockHash(_0);
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn block_hash_iter(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::subxt::KeyIter<'a, T, BlockHash<'a>>, ::subxt::BasicError>
        {
          self.client.storage().iter(hash).await
        }
        pub async fn extrinsic_data(
          &self,
          _0: &::core::primitive::u32,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::std::vec::Vec<::core::primitive::u8>, ::subxt::BasicError>
        {
          let entry = ExtrinsicData(_0);
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn extrinsic_data_iter(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::subxt::KeyIter<'a, T, ExtrinsicData<'a>>, ::subxt::BasicError>
        {
          self.client.storage().iter(hash).await
        }
        pub async fn number(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
          let entry = Number;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn parent_hash(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::subxt::sp_core::H256, ::subxt::BasicError> {
          let entry = ParentHash;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn digest(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          runtime_types::sp_runtime::generic::digest::Digest,
          ::subxt::BasicError,
        > {
          let entry = Digest;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn events(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::std::vec::Vec<
            runtime_types::frame_system::EventRecord<
              runtime_types::lagoon_runtime::Event,
              ::subxt::sp_core::H256,
            >,
          >,
          ::subxt::BasicError,
        > {
          let entry = Events;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn event_count(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
          let entry = EventCount;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn event_topics(
          &self,
          _0: &::subxt::sp_core::H256,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>,
          ::subxt::BasicError,
        > {
          let entry = EventTopics(_0);
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn event_topics_iter(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::subxt::KeyIter<'a, T, EventTopics<'a>>, ::subxt::BasicError>
        {
          self.client.storage().iter(hash).await
        }
        pub async fn last_runtime_upgrade(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::core::option::Option<runtime_types::frame_system::LastRuntimeUpgradeInfo>,
          ::subxt::BasicError,
        > {
          let entry = LastRuntimeUpgrade;
          self.client.storage().fetch(&entry, hash).await
        }
        pub async fn upgraded_to_u32_ref_count(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::core::primitive::bool, ::subxt::BasicError> {
          let entry = UpgradedToU32RefCount;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn upgraded_to_triple_ref_count(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::core::primitive::bool, ::subxt::BasicError> {
          let entry = UpgradedToTripleRefCount;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn execution_phase(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::core::option::Option<runtime_types::frame_system::Phase>,
          ::subxt::BasicError,
        > {
          let entry = ExecutionPhase;
          self.client.storage().fetch(&entry, hash).await
        }
      }
    }
    pub mod constants {
      use super::runtime_types;
      pub struct ConstantsApi<'a, T: ::subxt::Config> {
        client: &'a ::subxt::Client<T>,
      }
      impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self { client }
        }
        pub fn block_weights(
          &self,
        ) -> ::core::result::Result<
          runtime_types::frame_system::limits::BlockWeights,
          ::subxt::BasicError,
        > {
          let pallet = self.client.metadata().pallet("System")?;
          let constant = pallet.constant("BlockWeights")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn block_length(
          &self,
        ) -> ::core::result::Result<
          runtime_types::frame_system::limits::BlockLength,
          ::subxt::BasicError,
        > {
          let pallet = self.client.metadata().pallet("System")?;
          let constant = pallet.constant("BlockLength")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn block_hash_count(
          &self,
        ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
          let pallet = self.client.metadata().pallet("System")?;
          let constant = pallet.constant("BlockHashCount")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn db_weight(
          &self,
        ) -> ::core::result::Result<
          runtime_types::frame_support::weights::RuntimeDbWeight,
          ::subxt::BasicError,
        > {
          let pallet = self.client.metadata().pallet("System")?;
          let constant = pallet.constant("DbWeight")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn version(
          &self,
        ) -> ::core::result::Result<runtime_types::sp_version::RuntimeVersion, ::subxt::BasicError>
        {
          let pallet = self.client.metadata().pallet("System")?;
          let constant = pallet.constant("Version")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn ss58_prefix(
          &self,
        ) -> ::core::result::Result<::core::primitive::u16, ::subxt::BasicError> {
          let pallet = self.client.metadata().pallet("System")?;
          let constant = pallet.constant("SS58Prefix")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
      }
    }
  }
  pub mod babe {
    use super::root_mod;
    use super::runtime_types;
    pub mod calls {
      use super::root_mod;
      use super::runtime_types;
      type DispatchError = runtime_types::sp_runtime::DispatchError;
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct ReportEquivocation {
        pub equivocation_proof: ::std::boxed::Box<
          runtime_types::sp_consensus_slots::EquivocationProof<
            runtime_types::sp_runtime::generic::header::Header<
              ::core::primitive::u32,
              runtime_types::sp_runtime::traits::BlakeTwo256,
            >,
            runtime_types::sp_consensus_babe::app::Public,
          >,
        >,
        pub key_owner_proof: runtime_types::sp_session::MembershipProof,
      }
      impl ::subxt::Call for ReportEquivocation {
        const PALLET: &'static str = "Babe";
        const FUNCTION: &'static str = "report_equivocation";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct ReportEquivocationUnsigned {
        pub equivocation_proof: ::std::boxed::Box<
          runtime_types::sp_consensus_slots::EquivocationProof<
            runtime_types::sp_runtime::generic::header::Header<
              ::core::primitive::u32,
              runtime_types::sp_runtime::traits::BlakeTwo256,
            >,
            runtime_types::sp_consensus_babe::app::Public,
          >,
        >,
        pub key_owner_proof: runtime_types::sp_session::MembershipProof,
      }
      impl ::subxt::Call for ReportEquivocationUnsigned {
        const PALLET: &'static str = "Babe";
        const FUNCTION: &'static str = "report_equivocation_unsigned";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct PlanConfigChange {
        pub config: runtime_types::sp_consensus_babe::digests::NextConfigDescriptor,
      }
      impl ::subxt::Call for PlanConfigChange {
        const PALLET: &'static str = "Babe";
        const FUNCTION: &'static str = "plan_config_change";
      }
      pub struct TransactionApi<'a, T: ::subxt::Config, X> {
        client: &'a ::subxt::Client<T>,
        marker: ::core::marker::PhantomData<X>,
      }
      impl<'a, T, X> TransactionApi<'a, T, X>
      where
        T: ::subxt::Config,
        X: ::subxt::extrinsic::ExtrinsicParams<T>,
      {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self {
            client,
            marker: ::core::marker::PhantomData,
          }
        }
        pub fn report_equivocation(
          &self,
          equivocation_proof: runtime_types::sp_consensus_slots::EquivocationProof<
            runtime_types::sp_runtime::generic::header::Header<
              ::core::primitive::u32,
              runtime_types::sp_runtime::traits::BlakeTwo256,
            >,
            runtime_types::sp_consensus_babe::app::Public,
          >,
          key_owner_proof: runtime_types::sp_session::MembershipProof,
        ) -> ::subxt::SubmittableExtrinsic<
          'a,
          T,
          X,
          ReportEquivocation,
          DispatchError,
          root_mod::Event,
        > {
          let call = ReportEquivocation {
            equivocation_proof: ::std::boxed::Box::new(equivocation_proof),
            key_owner_proof,
          };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn report_equivocation_unsigned(
          &self,
          equivocation_proof: runtime_types::sp_consensus_slots::EquivocationProof<
            runtime_types::sp_runtime::generic::header::Header<
              ::core::primitive::u32,
              runtime_types::sp_runtime::traits::BlakeTwo256,
            >,
            runtime_types::sp_consensus_babe::app::Public,
          >,
          key_owner_proof: runtime_types::sp_session::MembershipProof,
        ) -> ::subxt::SubmittableExtrinsic<
          'a,
          T,
          X,
          ReportEquivocationUnsigned,
          DispatchError,
          root_mod::Event,
        > {
          let call = ReportEquivocationUnsigned {
            equivocation_proof: ::std::boxed::Box::new(equivocation_proof),
            key_owner_proof,
          };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn plan_config_change(
          &self,
          config: runtime_types::sp_consensus_babe::digests::NextConfigDescriptor,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, PlanConfigChange, DispatchError, root_mod::Event>
        {
          let call = PlanConfigChange { config };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
      }
    }
    pub mod storage {
      use super::runtime_types;
      pub struct EpochIndex;
      impl ::subxt::StorageEntry for EpochIndex {
        const PALLET: &'static str = "Babe";
        const STORAGE: &'static str = "EpochIndex";
        type Value = ::core::primitive::u64;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct Authorities;
      impl ::subxt::StorageEntry for Authorities {
        const PALLET: &'static str = "Babe";
        const STORAGE: &'static str = "Authorities";
        type Value = runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<(
          runtime_types::sp_consensus_babe::app::Public,
          ::core::primitive::u64,
        )>;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct GenesisSlot;
      impl ::subxt::StorageEntry for GenesisSlot {
        const PALLET: &'static str = "Babe";
        const STORAGE: &'static str = "GenesisSlot";
        type Value = runtime_types::sp_consensus_slots::Slot;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct CurrentSlot;
      impl ::subxt::StorageEntry for CurrentSlot {
        const PALLET: &'static str = "Babe";
        const STORAGE: &'static str = "CurrentSlot";
        type Value = runtime_types::sp_consensus_slots::Slot;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct Randomness;
      impl ::subxt::StorageEntry for Randomness {
        const PALLET: &'static str = "Babe";
        const STORAGE: &'static str = "Randomness";
        type Value = [::core::primitive::u8; 32usize];
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct PendingEpochConfigChange;
      impl ::subxt::StorageEntry for PendingEpochConfigChange {
        const PALLET: &'static str = "Babe";
        const STORAGE: &'static str = "PendingEpochConfigChange";
        type Value = runtime_types::sp_consensus_babe::digests::NextConfigDescriptor;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct NextRandomness;
      impl ::subxt::StorageEntry for NextRandomness {
        const PALLET: &'static str = "Babe";
        const STORAGE: &'static str = "NextRandomness";
        type Value = [::core::primitive::u8; 32usize];
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct NextAuthorities;
      impl ::subxt::StorageEntry for NextAuthorities {
        const PALLET: &'static str = "Babe";
        const STORAGE: &'static str = "NextAuthorities";
        type Value = runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<(
          runtime_types::sp_consensus_babe::app::Public,
          ::core::primitive::u64,
        )>;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct SegmentIndex;
      impl ::subxt::StorageEntry for SegmentIndex {
        const PALLET: &'static str = "Babe";
        const STORAGE: &'static str = "SegmentIndex";
        type Value = ::core::primitive::u32;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct UnderConstruction<'a>(pub &'a ::core::primitive::u32);
      impl ::subxt::StorageEntry for UnderConstruction<'_> {
        const PALLET: &'static str = "Babe";
        const STORAGE: &'static str = "UnderConstruction";
        type Value = runtime_types::frame_support::storage::bounded_vec::BoundedVec<
          [::core::primitive::u8; 32usize],
        >;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
            &self.0,
            ::subxt::StorageHasher::Twox64Concat,
          )])
        }
      }
      pub struct Initialized;
      impl ::subxt::StorageEntry for Initialized {
        const PALLET: &'static str = "Babe";
        const STORAGE: &'static str = "Initialized";
        type Value = ::core::option::Option<[::core::primitive::u8; 32usize]>;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct AuthorVrfRandomness;
      impl ::subxt::StorageEntry for AuthorVrfRandomness {
        const PALLET: &'static str = "Babe";
        const STORAGE: &'static str = "AuthorVrfRandomness";
        type Value = ::core::option::Option<[::core::primitive::u8; 32usize]>;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct EpochStart;
      impl ::subxt::StorageEntry for EpochStart {
        const PALLET: &'static str = "Babe";
        const STORAGE: &'static str = "EpochStart";
        type Value = (::core::primitive::u32, ::core::primitive::u32);
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct Lateness;
      impl ::subxt::StorageEntry for Lateness {
        const PALLET: &'static str = "Babe";
        const STORAGE: &'static str = "Lateness";
        type Value = ::core::primitive::u32;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct EpochConfig;
      impl ::subxt::StorageEntry for EpochConfig {
        const PALLET: &'static str = "Babe";
        const STORAGE: &'static str = "EpochConfig";
        type Value = runtime_types::sp_consensus_babe::BabeEpochConfiguration;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct NextEpochConfig;
      impl ::subxt::StorageEntry for NextEpochConfig {
        const PALLET: &'static str = "Babe";
        const STORAGE: &'static str = "NextEpochConfig";
        type Value = runtime_types::sp_consensus_babe::BabeEpochConfiguration;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct StorageApi<'a, T: ::subxt::Config> {
        client: &'a ::subxt::Client<T>,
      }
      impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self { client }
        }
        pub async fn epoch_index(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError> {
          let entry = EpochIndex;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn authorities(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<(
            runtime_types::sp_consensus_babe::app::Public,
            ::core::primitive::u64,
          )>,
          ::subxt::BasicError,
        > {
          let entry = Authorities;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn genesis_slot(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<runtime_types::sp_consensus_slots::Slot, ::subxt::BasicError>
        {
          let entry = GenesisSlot;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn current_slot(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<runtime_types::sp_consensus_slots::Slot, ::subxt::BasicError>
        {
          let entry = CurrentSlot;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn randomness(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<[::core::primitive::u8; 32usize], ::subxt::BasicError> {
          let entry = Randomness;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn pending_epoch_config_change(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::core::option::Option<runtime_types::sp_consensus_babe::digests::NextConfigDescriptor>,
          ::subxt::BasicError,
        > {
          let entry = PendingEpochConfigChange;
          self.client.storage().fetch(&entry, hash).await
        }
        pub async fn next_randomness(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<[::core::primitive::u8; 32usize], ::subxt::BasicError> {
          let entry = NextRandomness;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn next_authorities(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<(
            runtime_types::sp_consensus_babe::app::Public,
            ::core::primitive::u64,
          )>,
          ::subxt::BasicError,
        > {
          let entry = NextAuthorities;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn segment_index(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
          let entry = SegmentIndex;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn under_construction(
          &self,
          _0: &::core::primitive::u32,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          runtime_types::frame_support::storage::bounded_vec::BoundedVec<
            [::core::primitive::u8; 32usize],
          >,
          ::subxt::BasicError,
        > {
          let entry = UnderConstruction(_0);
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn under_construction_iter(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::subxt::KeyIter<'a, T, UnderConstruction<'a>>,
          ::subxt::BasicError,
        > {
          self.client.storage().iter(hash).await
        }
        pub async fn initialized(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::core::option::Option<::core::option::Option<[::core::primitive::u8; 32usize]>>,
          ::subxt::BasicError,
        > {
          let entry = Initialized;
          self.client.storage().fetch(&entry, hash).await
        }
        pub async fn author_vrf_randomness(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::core::option::Option<[::core::primitive::u8; 32usize]>,
          ::subxt::BasicError,
        > {
          let entry = AuthorVrfRandomness;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn epoch_start(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          (::core::primitive::u32, ::core::primitive::u32),
          ::subxt::BasicError,
        > {
          let entry = EpochStart;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn lateness(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
          let entry = Lateness;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn epoch_config(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::core::option::Option<runtime_types::sp_consensus_babe::BabeEpochConfiguration>,
          ::subxt::BasicError,
        > {
          let entry = EpochConfig;
          self.client.storage().fetch(&entry, hash).await
        }
        pub async fn next_epoch_config(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::core::option::Option<runtime_types::sp_consensus_babe::BabeEpochConfiguration>,
          ::subxt::BasicError,
        > {
          let entry = NextEpochConfig;
          self.client.storage().fetch(&entry, hash).await
        }
      }
    }
    pub mod constants {
      use super::runtime_types;
      pub struct ConstantsApi<'a, T: ::subxt::Config> {
        client: &'a ::subxt::Client<T>,
      }
      impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self { client }
        }
        pub fn epoch_duration(
          &self,
        ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError> {
          let pallet = self.client.metadata().pallet("Babe")?;
          let constant = pallet.constant("EpochDuration")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn expected_block_time(
          &self,
        ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError> {
          let pallet = self.client.metadata().pallet("Babe")?;
          let constant = pallet.constant("ExpectedBlockTime")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn max_authorities(
          &self,
        ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
          let pallet = self.client.metadata().pallet("Babe")?;
          let constant = pallet.constant("MaxAuthorities")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
      }
    }
  }
  pub mod timestamp {
    use super::root_mod;
    use super::runtime_types;
    pub mod calls {
      use super::root_mod;
      use super::runtime_types;
      type DispatchError = runtime_types::sp_runtime::DispatchError;
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Set {
        #[codec(compact)]
        pub now: ::core::primitive::u64,
      }
      impl ::subxt::Call for Set {
        const PALLET: &'static str = "Timestamp";
        const FUNCTION: &'static str = "set";
      }
      pub struct TransactionApi<'a, T: ::subxt::Config, X> {
        client: &'a ::subxt::Client<T>,
        marker: ::core::marker::PhantomData<X>,
      }
      impl<'a, T, X> TransactionApi<'a, T, X>
      where
        T: ::subxt::Config,
        X: ::subxt::extrinsic::ExtrinsicParams<T>,
      {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self {
            client,
            marker: ::core::marker::PhantomData,
          }
        }
        pub fn set(
          &self,
          now: ::core::primitive::u64,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, Set, DispatchError, root_mod::Event> {
          let call = Set { now };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
      }
    }
    pub mod storage {
      use super::runtime_types;
      pub struct Now;
      impl ::subxt::StorageEntry for Now {
        const PALLET: &'static str = "Timestamp";
        const STORAGE: &'static str = "Now";
        type Value = ::core::primitive::u64;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct DidUpdate;
      impl ::subxt::StorageEntry for DidUpdate {
        const PALLET: &'static str = "Timestamp";
        const STORAGE: &'static str = "DidUpdate";
        type Value = ::core::primitive::bool;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct StorageApi<'a, T: ::subxt::Config> {
        client: &'a ::subxt::Client<T>,
      }
      impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self { client }
        }
        pub async fn now(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError> {
          let entry = Now;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn did_update(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::core::primitive::bool, ::subxt::BasicError> {
          let entry = DidUpdate;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
      }
    }
    pub mod constants {
      use super::runtime_types;
      pub struct ConstantsApi<'a, T: ::subxt::Config> {
        client: &'a ::subxt::Client<T>,
      }
      impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self { client }
        }
        pub fn minimum_period(
          &self,
        ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError> {
          let pallet = self.client.metadata().pallet("Timestamp")?;
          let constant = pallet.constant("MinimumPeriod")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
      }
    }
  }
  pub mod indices {
    use super::root_mod;
    use super::runtime_types;
    pub mod calls {
      use super::root_mod;
      use super::runtime_types;
      type DispatchError = runtime_types::sp_runtime::DispatchError;
      #[derive(
        :: subxt :: codec :: Encode,
        :: subxt :: codec :: Decode,
        Debug,
        :: subxt :: codec :: CompactAs,
      )]
      pub struct Claim {
        pub index: ::core::primitive::u32,
      }
      impl ::subxt::Call for Claim {
        const PALLET: &'static str = "Indices";
        const FUNCTION: &'static str = "claim";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Transfer {
        pub new: ::subxt::sp_core::crypto::AccountId32,
        pub index: ::core::primitive::u32,
      }
      impl ::subxt::Call for Transfer {
        const PALLET: &'static str = "Indices";
        const FUNCTION: &'static str = "transfer";
      }
      #[derive(
        :: subxt :: codec :: Encode,
        :: subxt :: codec :: Decode,
        Debug,
        :: subxt :: codec :: CompactAs,
      )]
      pub struct Free {
        pub index: ::core::primitive::u32,
      }
      impl ::subxt::Call for Free {
        const PALLET: &'static str = "Indices";
        const FUNCTION: &'static str = "free";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct ForceTransfer {
        pub new: ::subxt::sp_core::crypto::AccountId32,
        pub index: ::core::primitive::u32,
        pub freeze: ::core::primitive::bool,
      }
      impl ::subxt::Call for ForceTransfer {
        const PALLET: &'static str = "Indices";
        const FUNCTION: &'static str = "force_transfer";
      }
      #[derive(
        :: subxt :: codec :: Encode,
        :: subxt :: codec :: Decode,
        Debug,
        :: subxt :: codec :: CompactAs,
      )]
      pub struct Freeze {
        pub index: ::core::primitive::u32,
      }
      impl ::subxt::Call for Freeze {
        const PALLET: &'static str = "Indices";
        const FUNCTION: &'static str = "freeze";
      }
      pub struct TransactionApi<'a, T: ::subxt::Config, X> {
        client: &'a ::subxt::Client<T>,
        marker: ::core::marker::PhantomData<X>,
      }
      impl<'a, T, X> TransactionApi<'a, T, X>
      where
        T: ::subxt::Config,
        X: ::subxt::extrinsic::ExtrinsicParams<T>,
      {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self {
            client,
            marker: ::core::marker::PhantomData,
          }
        }
        pub fn claim(
          &self,
          index: ::core::primitive::u32,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, Claim, DispatchError, root_mod::Event>
        {
          let call = Claim { index };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn transfer(
          &self,
          new: ::subxt::sp_core::crypto::AccountId32,
          index: ::core::primitive::u32,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, Transfer, DispatchError, root_mod::Event>
        {
          let call = Transfer { new, index };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn free(
          &self,
          index: ::core::primitive::u32,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, Free, DispatchError, root_mod::Event> {
          let call = Free { index };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn force_transfer(
          &self,
          new: ::subxt::sp_core::crypto::AccountId32,
          index: ::core::primitive::u32,
          freeze: ::core::primitive::bool,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, ForceTransfer, DispatchError, root_mod::Event>
        {
          let call = ForceTransfer { new, index, freeze };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn freeze(
          &self,
          index: ::core::primitive::u32,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, Freeze, DispatchError, root_mod::Event>
        {
          let call = Freeze { index };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
      }
    }
    pub type Event = runtime_types::pallet_indices::pallet::Event;
    pub mod events {
      use super::runtime_types;
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct IndexAssigned {
        pub who: ::subxt::sp_core::crypto::AccountId32,
        pub index: ::core::primitive::u32,
      }
      impl ::subxt::Event for IndexAssigned {
        const PALLET: &'static str = "Indices";
        const EVENT: &'static str = "IndexAssigned";
      }
      #[derive(
        :: subxt :: codec :: Encode,
        :: subxt :: codec :: Decode,
        Debug,
        :: subxt :: codec :: CompactAs,
      )]
      pub struct IndexFreed {
        pub index: ::core::primitive::u32,
      }
      impl ::subxt::Event for IndexFreed {
        const PALLET: &'static str = "Indices";
        const EVENT: &'static str = "IndexFreed";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct IndexFrozen {
        pub index: ::core::primitive::u32,
        pub who: ::subxt::sp_core::crypto::AccountId32,
      }
      impl ::subxt::Event for IndexFrozen {
        const PALLET: &'static str = "Indices";
        const EVENT: &'static str = "IndexFrozen";
      }
    }
    pub mod storage {
      use super::runtime_types;
      pub struct Accounts<'a>(pub &'a ::core::primitive::u32);
      impl ::subxt::StorageEntry for Accounts<'_> {
        const PALLET: &'static str = "Indices";
        const STORAGE: &'static str = "Accounts";
        type Value = (
          ::subxt::sp_core::crypto::AccountId32,
          ::core::primitive::u128,
          ::core::primitive::bool,
        );
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
            &self.0,
            ::subxt::StorageHasher::Blake2_128Concat,
          )])
        }
      }
      pub struct StorageApi<'a, T: ::subxt::Config> {
        client: &'a ::subxt::Client<T>,
      }
      impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self { client }
        }
        pub async fn accounts(
          &self,
          _0: &::core::primitive::u32,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::core::option::Option<(
            ::subxt::sp_core::crypto::AccountId32,
            ::core::primitive::u128,
            ::core::primitive::bool,
          )>,
          ::subxt::BasicError,
        > {
          let entry = Accounts(_0);
          self.client.storage().fetch(&entry, hash).await
        }
        pub async fn accounts_iter(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Accounts<'a>>, ::subxt::BasicError>
        {
          self.client.storage().iter(hash).await
        }
      }
    }
    pub mod constants {
      use super::runtime_types;
      pub struct ConstantsApi<'a, T: ::subxt::Config> {
        client: &'a ::subxt::Client<T>,
      }
      impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self { client }
        }
        pub fn deposit(
          &self,
        ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError> {
          let pallet = self.client.metadata().pallet("Indices")?;
          let constant = pallet.constant("Deposit")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
      }
    }
  }
  pub mod balances {
    use super::root_mod;
    use super::runtime_types;
    pub mod calls {
      use super::root_mod;
      use super::runtime_types;
      type DispatchError = runtime_types::sp_runtime::DispatchError;
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Transfer {
        pub dest: ::subxt::sp_runtime::MultiAddress<
          ::subxt::sp_core::crypto::AccountId32,
          ::core::primitive::u32,
        >,
        #[codec(compact)]
        pub value: ::core::primitive::u128,
      }
      impl ::subxt::Call for Transfer {
        const PALLET: &'static str = "Balances";
        const FUNCTION: &'static str = "transfer";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct SetBalance {
        pub who: ::subxt::sp_runtime::MultiAddress<
          ::subxt::sp_core::crypto::AccountId32,
          ::core::primitive::u32,
        >,
        #[codec(compact)]
        pub new_free: ::core::primitive::u128,
        #[codec(compact)]
        pub new_reserved: ::core::primitive::u128,
      }
      impl ::subxt::Call for SetBalance {
        const PALLET: &'static str = "Balances";
        const FUNCTION: &'static str = "set_balance";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct ForceTransfer {
        pub source: ::subxt::sp_runtime::MultiAddress<
          ::subxt::sp_core::crypto::AccountId32,
          ::core::primitive::u32,
        >,
        pub dest: ::subxt::sp_runtime::MultiAddress<
          ::subxt::sp_core::crypto::AccountId32,
          ::core::primitive::u32,
        >,
        #[codec(compact)]
        pub value: ::core::primitive::u128,
      }
      impl ::subxt::Call for ForceTransfer {
        const PALLET: &'static str = "Balances";
        const FUNCTION: &'static str = "force_transfer";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct TransferKeepAlive {
        pub dest: ::subxt::sp_runtime::MultiAddress<
          ::subxt::sp_core::crypto::AccountId32,
          ::core::primitive::u32,
        >,
        #[codec(compact)]
        pub value: ::core::primitive::u128,
      }
      impl ::subxt::Call for TransferKeepAlive {
        const PALLET: &'static str = "Balances";
        const FUNCTION: &'static str = "transfer_keep_alive";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct TransferAll {
        pub dest: ::subxt::sp_runtime::MultiAddress<
          ::subxt::sp_core::crypto::AccountId32,
          ::core::primitive::u32,
        >,
        pub keep_alive: ::core::primitive::bool,
      }
      impl ::subxt::Call for TransferAll {
        const PALLET: &'static str = "Balances";
        const FUNCTION: &'static str = "transfer_all";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct ForceUnreserve {
        pub who: ::subxt::sp_runtime::MultiAddress<
          ::subxt::sp_core::crypto::AccountId32,
          ::core::primitive::u32,
        >,
        pub amount: ::core::primitive::u128,
      }
      impl ::subxt::Call for ForceUnreserve {
        const PALLET: &'static str = "Balances";
        const FUNCTION: &'static str = "force_unreserve";
      }
      pub struct TransactionApi<'a, T: ::subxt::Config, X> {
        client: &'a ::subxt::Client<T>,
        marker: ::core::marker::PhantomData<X>,
      }
      impl<'a, T, X> TransactionApi<'a, T, X>
      where
        T: ::subxt::Config,
        X: ::subxt::extrinsic::ExtrinsicParams<T>,
      {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self {
            client,
            marker: ::core::marker::PhantomData,
          }
        }
        pub fn transfer(
          &self,
          dest: ::subxt::sp_runtime::MultiAddress<
            ::subxt::sp_core::crypto::AccountId32,
            ::core::primitive::u32,
          >,
          value: ::core::primitive::u128,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, Transfer, DispatchError, root_mod::Event>
        {
          let call = Transfer { dest, value };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn set_balance(
          &self,
          who: ::subxt::sp_runtime::MultiAddress<
            ::subxt::sp_core::crypto::AccountId32,
            ::core::primitive::u32,
          >,
          new_free: ::core::primitive::u128,
          new_reserved: ::core::primitive::u128,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, SetBalance, DispatchError, root_mod::Event>
        {
          let call = SetBalance {
            who,
            new_free,
            new_reserved,
          };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn force_transfer(
          &self,
          source: ::subxt::sp_runtime::MultiAddress<
            ::subxt::sp_core::crypto::AccountId32,
            ::core::primitive::u32,
          >,
          dest: ::subxt::sp_runtime::MultiAddress<
            ::subxt::sp_core::crypto::AccountId32,
            ::core::primitive::u32,
          >,
          value: ::core::primitive::u128,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, ForceTransfer, DispatchError, root_mod::Event>
        {
          let call = ForceTransfer {
            source,
            dest,
            value,
          };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn transfer_keep_alive(
          &self,
          dest: ::subxt::sp_runtime::MultiAddress<
            ::subxt::sp_core::crypto::AccountId32,
            ::core::primitive::u32,
          >,
          value: ::core::primitive::u128,
        ) -> ::subxt::SubmittableExtrinsic<
          'a,
          T,
          X,
          TransferKeepAlive,
          DispatchError,
          root_mod::Event,
        > {
          let call = TransferKeepAlive { dest, value };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn transfer_all(
          &self,
          dest: ::subxt::sp_runtime::MultiAddress<
            ::subxt::sp_core::crypto::AccountId32,
            ::core::primitive::u32,
          >,
          keep_alive: ::core::primitive::bool,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, TransferAll, DispatchError, root_mod::Event>
        {
          let call = TransferAll { dest, keep_alive };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn force_unreserve(
          &self,
          who: ::subxt::sp_runtime::MultiAddress<
            ::subxt::sp_core::crypto::AccountId32,
            ::core::primitive::u32,
          >,
          amount: ::core::primitive::u128,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, ForceUnreserve, DispatchError, root_mod::Event>
        {
          let call = ForceUnreserve { who, amount };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
      }
    }
    pub type Event = runtime_types::pallet_balances::pallet::Event;
    pub mod events {
      use super::runtime_types;
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Endowed {
        pub account: ::subxt::sp_core::crypto::AccountId32,
        pub free_balance: ::core::primitive::u128,
      }
      impl ::subxt::Event for Endowed {
        const PALLET: &'static str = "Balances";
        const EVENT: &'static str = "Endowed";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct DustLost {
        pub account: ::subxt::sp_core::crypto::AccountId32,
        pub amount: ::core::primitive::u128,
      }
      impl ::subxt::Event for DustLost {
        const PALLET: &'static str = "Balances";
        const EVENT: &'static str = "DustLost";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Transfer {
        pub from: ::subxt::sp_core::crypto::AccountId32,
        pub to: ::subxt::sp_core::crypto::AccountId32,
        pub amount: ::core::primitive::u128,
      }
      impl ::subxt::Event for Transfer {
        const PALLET: &'static str = "Balances";
        const EVENT: &'static str = "Transfer";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct BalanceSet {
        pub who: ::subxt::sp_core::crypto::AccountId32,
        pub free: ::core::primitive::u128,
        pub reserved: ::core::primitive::u128,
      }
      impl ::subxt::Event for BalanceSet {
        const PALLET: &'static str = "Balances";
        const EVENT: &'static str = "BalanceSet";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Reserved {
        pub who: ::subxt::sp_core::crypto::AccountId32,
        pub amount: ::core::primitive::u128,
      }
      impl ::subxt::Event for Reserved {
        const PALLET: &'static str = "Balances";
        const EVENT: &'static str = "Reserved";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Unreserved {
        pub who: ::subxt::sp_core::crypto::AccountId32,
        pub amount: ::core::primitive::u128,
      }
      impl ::subxt::Event for Unreserved {
        const PALLET: &'static str = "Balances";
        const EVENT: &'static str = "Unreserved";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct ReserveRepatriated {
        pub from: ::subxt::sp_core::crypto::AccountId32,
        pub to: ::subxt::sp_core::crypto::AccountId32,
        pub amount: ::core::primitive::u128,
        pub destination_status: runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
      }
      impl ::subxt::Event for ReserveRepatriated {
        const PALLET: &'static str = "Balances";
        const EVENT: &'static str = "ReserveRepatriated";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Deposit {
        pub who: ::subxt::sp_core::crypto::AccountId32,
        pub amount: ::core::primitive::u128,
      }
      impl ::subxt::Event for Deposit {
        const PALLET: &'static str = "Balances";
        const EVENT: &'static str = "Deposit";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Withdraw {
        pub who: ::subxt::sp_core::crypto::AccountId32,
        pub amount: ::core::primitive::u128,
      }
      impl ::subxt::Event for Withdraw {
        const PALLET: &'static str = "Balances";
        const EVENT: &'static str = "Withdraw";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Slashed {
        pub who: ::subxt::sp_core::crypto::AccountId32,
        pub amount: ::core::primitive::u128,
      }
      impl ::subxt::Event for Slashed {
        const PALLET: &'static str = "Balances";
        const EVENT: &'static str = "Slashed";
      }
    }
    pub mod storage {
      use super::runtime_types;
      pub struct TotalIssuance;
      impl ::subxt::StorageEntry for TotalIssuance {
        const PALLET: &'static str = "Balances";
        const STORAGE: &'static str = "TotalIssuance";
        type Value = ::core::primitive::u128;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct Account<'a>(pub &'a ::subxt::sp_core::crypto::AccountId32);
      impl ::subxt::StorageEntry for Account<'_> {
        const PALLET: &'static str = "Balances";
        const STORAGE: &'static str = "Account";
        type Value = runtime_types::pallet_balances::AccountData<::core::primitive::u128>;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
            &self.0,
            ::subxt::StorageHasher::Blake2_128Concat,
          )])
        }
      }
      pub struct Locks<'a>(pub &'a ::subxt::sp_core::crypto::AccountId32);
      impl ::subxt::StorageEntry for Locks<'_> {
        const PALLET: &'static str = "Balances";
        const STORAGE: &'static str = "Locks";
        type Value = runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
          runtime_types::pallet_balances::BalanceLock<::core::primitive::u128>,
        >;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
            &self.0,
            ::subxt::StorageHasher::Blake2_128Concat,
          )])
        }
      }
      pub struct Reserves<'a>(pub &'a ::subxt::sp_core::crypto::AccountId32);
      impl ::subxt::StorageEntry for Reserves<'_> {
        const PALLET: &'static str = "Balances";
        const STORAGE: &'static str = "Reserves";
        type Value = runtime_types::frame_support::storage::bounded_vec::BoundedVec<
          runtime_types::pallet_balances::ReserveData<
            [::core::primitive::u8; 8usize],
            ::core::primitive::u128,
          >,
        >;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
            &self.0,
            ::subxt::StorageHasher::Blake2_128Concat,
          )])
        }
      }
      pub struct StorageVersion;
      impl ::subxt::StorageEntry for StorageVersion {
        const PALLET: &'static str = "Balances";
        const STORAGE: &'static str = "StorageVersion";
        type Value = runtime_types::pallet_balances::Releases;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct StorageApi<'a, T: ::subxt::Config> {
        client: &'a ::subxt::Client<T>,
      }
      impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self { client }
        }
        pub async fn total_issuance(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError> {
          let entry = TotalIssuance;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn account(
          &self,
          _0: &::subxt::sp_core::crypto::AccountId32,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          runtime_types::pallet_balances::AccountData<::core::primitive::u128>,
          ::subxt::BasicError,
        > {
          let entry = Account(_0);
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn account_iter(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Account<'a>>, ::subxt::BasicError>
        {
          self.client.storage().iter(hash).await
        }
        pub async fn locks(
          &self,
          _0: &::subxt::sp_core::crypto::AccountId32,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
            runtime_types::pallet_balances::BalanceLock<::core::primitive::u128>,
          >,
          ::subxt::BasicError,
        > {
          let entry = Locks(_0);
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn locks_iter(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Locks<'a>>, ::subxt::BasicError>
        {
          self.client.storage().iter(hash).await
        }
        pub async fn reserves(
          &self,
          _0: &::subxt::sp_core::crypto::AccountId32,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          runtime_types::frame_support::storage::bounded_vec::BoundedVec<
            runtime_types::pallet_balances::ReserveData<
              [::core::primitive::u8; 8usize],
              ::core::primitive::u128,
            >,
          >,
          ::subxt::BasicError,
        > {
          let entry = Reserves(_0);
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn reserves_iter(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Reserves<'a>>, ::subxt::BasicError>
        {
          self.client.storage().iter(hash).await
        }
        pub async fn storage_version(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<runtime_types::pallet_balances::Releases, ::subxt::BasicError>
        {
          let entry = StorageVersion;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
      }
    }
    pub mod constants {
      use super::runtime_types;
      pub struct ConstantsApi<'a, T: ::subxt::Config> {
        client: &'a ::subxt::Client<T>,
      }
      impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self { client }
        }
        pub fn existential_deposit(
          &self,
        ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError> {
          let pallet = self.client.metadata().pallet("Balances")?;
          let constant = pallet.constant("ExistentialDeposit")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn max_locks(
          &self,
        ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
          let pallet = self.client.metadata().pallet("Balances")?;
          let constant = pallet.constant("MaxLocks")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn max_reserves(
          &self,
        ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
          let pallet = self.client.metadata().pallet("Balances")?;
          let constant = pallet.constant("MaxReserves")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
      }
    }
  }
  pub mod transaction_payment {
    use super::root_mod;
    use super::runtime_types;
    pub mod storage {
      use super::runtime_types;
      pub struct NextFeeMultiplier;
      impl ::subxt::StorageEntry for NextFeeMultiplier {
        const PALLET: &'static str = "TransactionPayment";
        const STORAGE: &'static str = "NextFeeMultiplier";
        type Value = runtime_types::sp_arithmetic::fixed_point::FixedU128;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct StorageVersion;
      impl ::subxt::StorageEntry for StorageVersion {
        const PALLET: &'static str = "TransactionPayment";
        const STORAGE: &'static str = "StorageVersion";
        type Value = runtime_types::pallet_transaction_payment::Releases;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct StorageApi<'a, T: ::subxt::Config> {
        client: &'a ::subxt::Client<T>,
      }
      impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self { client }
        }
        pub async fn next_fee_multiplier(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          runtime_types::sp_arithmetic::fixed_point::FixedU128,
          ::subxt::BasicError,
        > {
          let entry = NextFeeMultiplier;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn storage_version(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          runtime_types::pallet_transaction_payment::Releases,
          ::subxt::BasicError,
        > {
          let entry = StorageVersion;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
      }
    }
    pub mod constants {
      use super::runtime_types;
      pub struct ConstantsApi<'a, T: ::subxt::Config> {
        client: &'a ::subxt::Client<T>,
      }
      impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self { client }
        }
        pub fn transaction_byte_fee(
          &self,
        ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError> {
          let pallet = self.client.metadata().pallet("TransactionPayment")?;
          let constant = pallet.constant("TransactionByteFee")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn operational_fee_multiplier(
          &self,
        ) -> ::core::result::Result<::core::primitive::u8, ::subxt::BasicError> {
          let pallet = self.client.metadata().pallet("TransactionPayment")?;
          let constant = pallet.constant("OperationalFeeMultiplier")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn weight_to_fee(
          &self,
        ) -> ::core::result::Result<
          ::std::vec::Vec<
            runtime_types::frame_support::weights::WeightToFeeCoefficient<::core::primitive::u128>,
          >,
          ::subxt::BasicError,
        > {
          let pallet = self.client.metadata().pallet("TransactionPayment")?;
          let constant = pallet.constant("WeightToFee")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
      }
    }
  }
  pub mod authorship {
    use super::root_mod;
    use super::runtime_types;
    pub mod calls {
      use super::root_mod;
      use super::runtime_types;
      type DispatchError = runtime_types::sp_runtime::DispatchError;
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct SetUncles {
        pub new_uncles: ::std::vec::Vec<
          runtime_types::sp_runtime::generic::header::Header<
            ::core::primitive::u32,
            runtime_types::sp_runtime::traits::BlakeTwo256,
          >,
        >,
      }
      impl ::subxt::Call for SetUncles {
        const PALLET: &'static str = "Authorship";
        const FUNCTION: &'static str = "set_uncles";
      }
      pub struct TransactionApi<'a, T: ::subxt::Config, X> {
        client: &'a ::subxt::Client<T>,
        marker: ::core::marker::PhantomData<X>,
      }
      impl<'a, T, X> TransactionApi<'a, T, X>
      where
        T: ::subxt::Config,
        X: ::subxt::extrinsic::ExtrinsicParams<T>,
      {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self {
            client,
            marker: ::core::marker::PhantomData,
          }
        }
        pub fn set_uncles(
          &self,
          new_uncles: ::std::vec::Vec<
            runtime_types::sp_runtime::generic::header::Header<
              ::core::primitive::u32,
              runtime_types::sp_runtime::traits::BlakeTwo256,
            >,
          >,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, SetUncles, DispatchError, root_mod::Event>
        {
          let call = SetUncles { new_uncles };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
      }
    }
    pub mod storage {
      use super::runtime_types;
      pub struct Uncles;
      impl ::subxt::StorageEntry for Uncles {
        const PALLET: &'static str = "Authorship";
        const STORAGE: &'static str = "Uncles";
        type Value = ::std::vec::Vec<
          runtime_types::pallet_authorship::UncleEntryItem<
            ::core::primitive::u32,
            ::subxt::sp_core::H256,
            ::subxt::sp_core::crypto::AccountId32,
          >,
        >;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct Author;
      impl ::subxt::StorageEntry for Author {
        const PALLET: &'static str = "Authorship";
        const STORAGE: &'static str = "Author";
        type Value = ::subxt::sp_core::crypto::AccountId32;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct DidSetUncles;
      impl ::subxt::StorageEntry for DidSetUncles {
        const PALLET: &'static str = "Authorship";
        const STORAGE: &'static str = "DidSetUncles";
        type Value = ::core::primitive::bool;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct StorageApi<'a, T: ::subxt::Config> {
        client: &'a ::subxt::Client<T>,
      }
      impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self { client }
        }
        pub async fn uncles(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::std::vec::Vec<
            runtime_types::pallet_authorship::UncleEntryItem<
              ::core::primitive::u32,
              ::subxt::sp_core::H256,
              ::subxt::sp_core::crypto::AccountId32,
            >,
          >,
          ::subxt::BasicError,
        > {
          let entry = Uncles;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn author(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
          ::subxt::BasicError,
        > {
          let entry = Author;
          self.client.storage().fetch(&entry, hash).await
        }
        pub async fn did_set_uncles(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::core::primitive::bool, ::subxt::BasicError> {
          let entry = DidSetUncles;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
      }
    }
    pub mod constants {
      use super::runtime_types;
      pub struct ConstantsApi<'a, T: ::subxt::Config> {
        client: &'a ::subxt::Client<T>,
      }
      impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self { client }
        }
        pub fn uncle_generations(
          &self,
        ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
          let pallet = self.client.metadata().pallet("Authorship")?;
          let constant = pallet.constant("UncleGenerations")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
      }
    }
  }
  pub mod staking {
    use super::root_mod;
    use super::runtime_types;
    pub mod calls {
      use super::root_mod;
      use super::runtime_types;
      type DispatchError = runtime_types::sp_runtime::DispatchError;
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Bond {
        pub controller: ::subxt::sp_runtime::MultiAddress<
          ::subxt::sp_core::crypto::AccountId32,
          ::core::primitive::u32,
        >,
        #[codec(compact)]
        pub value: ::core::primitive::u128,
        pub payee:
          runtime_types::pallet_staking::RewardDestination<::subxt::sp_core::crypto::AccountId32>,
      }
      impl ::subxt::Call for Bond {
        const PALLET: &'static str = "Staking";
        const FUNCTION: &'static str = "bond";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct BondExtra {
        #[codec(compact)]
        pub max_additional: ::core::primitive::u128,
      }
      impl ::subxt::Call for BondExtra {
        const PALLET: &'static str = "Staking";
        const FUNCTION: &'static str = "bond_extra";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Unbond {
        #[codec(compact)]
        pub value: ::core::primitive::u128,
      }
      impl ::subxt::Call for Unbond {
        const PALLET: &'static str = "Staking";
        const FUNCTION: &'static str = "unbond";
      }
      #[derive(
        :: subxt :: codec :: Encode,
        :: subxt :: codec :: Decode,
        Debug,
        :: subxt :: codec :: CompactAs,
      )]
      pub struct WithdrawUnbonded {
        pub num_slashing_spans: ::core::primitive::u32,
      }
      impl ::subxt::Call for WithdrawUnbonded {
        const PALLET: &'static str = "Staking";
        const FUNCTION: &'static str = "withdraw_unbonded";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Validate {
        pub prefs: runtime_types::pallet_staking::ValidatorPrefs,
      }
      impl ::subxt::Call for Validate {
        const PALLET: &'static str = "Staking";
        const FUNCTION: &'static str = "validate";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Nominate {
        pub targets: ::std::vec::Vec<
          ::subxt::sp_runtime::MultiAddress<
            ::subxt::sp_core::crypto::AccountId32,
            ::core::primitive::u32,
          >,
        >,
      }
      impl ::subxt::Call for Nominate {
        const PALLET: &'static str = "Staking";
        const FUNCTION: &'static str = "nominate";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Chill;
      impl ::subxt::Call for Chill {
        const PALLET: &'static str = "Staking";
        const FUNCTION: &'static str = "chill";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct SetPayee {
        pub payee:
          runtime_types::pallet_staking::RewardDestination<::subxt::sp_core::crypto::AccountId32>,
      }
      impl ::subxt::Call for SetPayee {
        const PALLET: &'static str = "Staking";
        const FUNCTION: &'static str = "set_payee";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct SetController {
        pub controller: ::subxt::sp_runtime::MultiAddress<
          ::subxt::sp_core::crypto::AccountId32,
          ::core::primitive::u32,
        >,
      }
      impl ::subxt::Call for SetController {
        const PALLET: &'static str = "Staking";
        const FUNCTION: &'static str = "set_controller";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct SetValidatorCount {
        #[codec(compact)]
        pub new: ::core::primitive::u32,
      }
      impl ::subxt::Call for SetValidatorCount {
        const PALLET: &'static str = "Staking";
        const FUNCTION: &'static str = "set_validator_count";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct IncreaseValidatorCount {
        #[codec(compact)]
        pub additional: ::core::primitive::u32,
      }
      impl ::subxt::Call for IncreaseValidatorCount {
        const PALLET: &'static str = "Staking";
        const FUNCTION: &'static str = "increase_validator_count";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct ScaleValidatorCount {
        pub factor: runtime_types::sp_arithmetic::per_things::Percent,
      }
      impl ::subxt::Call for ScaleValidatorCount {
        const PALLET: &'static str = "Staking";
        const FUNCTION: &'static str = "scale_validator_count";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct ForceNoEras;
      impl ::subxt::Call for ForceNoEras {
        const PALLET: &'static str = "Staking";
        const FUNCTION: &'static str = "force_no_eras";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct ForceNewEra;
      impl ::subxt::Call for ForceNewEra {
        const PALLET: &'static str = "Staking";
        const FUNCTION: &'static str = "force_new_era";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct SetInvulnerables {
        pub invulnerables: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
      }
      impl ::subxt::Call for SetInvulnerables {
        const PALLET: &'static str = "Staking";
        const FUNCTION: &'static str = "set_invulnerables";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct ForceUnstake {
        pub stash: ::subxt::sp_core::crypto::AccountId32,
        pub num_slashing_spans: ::core::primitive::u32,
      }
      impl ::subxt::Call for ForceUnstake {
        const PALLET: &'static str = "Staking";
        const FUNCTION: &'static str = "force_unstake";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct ForceNewEraAlways;
      impl ::subxt::Call for ForceNewEraAlways {
        const PALLET: &'static str = "Staking";
        const FUNCTION: &'static str = "force_new_era_always";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct CancelDeferredSlash {
        pub era: ::core::primitive::u32,
        pub slash_indices: ::std::vec::Vec<::core::primitive::u32>,
      }
      impl ::subxt::Call for CancelDeferredSlash {
        const PALLET: &'static str = "Staking";
        const FUNCTION: &'static str = "cancel_deferred_slash";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct PayoutStakers {
        pub validator_stash: ::subxt::sp_core::crypto::AccountId32,
        pub era: ::core::primitive::u32,
      }
      impl ::subxt::Call for PayoutStakers {
        const PALLET: &'static str = "Staking";
        const FUNCTION: &'static str = "payout_stakers";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Rebond {
        #[codec(compact)]
        pub value: ::core::primitive::u128,
      }
      impl ::subxt::Call for Rebond {
        const PALLET: &'static str = "Staking";
        const FUNCTION: &'static str = "rebond";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct SetHistoryDepth {
        #[codec(compact)]
        pub new_history_depth: ::core::primitive::u32,
        #[codec(compact)]
        pub era_items_deleted: ::core::primitive::u32,
      }
      impl ::subxt::Call for SetHistoryDepth {
        const PALLET: &'static str = "Staking";
        const FUNCTION: &'static str = "set_history_depth";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct ReapStash {
        pub stash: ::subxt::sp_core::crypto::AccountId32,
        pub num_slashing_spans: ::core::primitive::u32,
      }
      impl ::subxt::Call for ReapStash {
        const PALLET: &'static str = "Staking";
        const FUNCTION: &'static str = "reap_stash";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Kick {
        pub who: ::std::vec::Vec<
          ::subxt::sp_runtime::MultiAddress<
            ::subxt::sp_core::crypto::AccountId32,
            ::core::primitive::u32,
          >,
        >,
      }
      impl ::subxt::Call for Kick {
        const PALLET: &'static str = "Staking";
        const FUNCTION: &'static str = "kick";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct SetStakingConfigs {
        pub min_nominator_bond:
          runtime_types::pallet_staking::pallet::pallet::ConfigOp<::core::primitive::u128>,
        pub min_validator_bond:
          runtime_types::pallet_staking::pallet::pallet::ConfigOp<::core::primitive::u128>,
        pub max_nominator_count:
          runtime_types::pallet_staking::pallet::pallet::ConfigOp<::core::primitive::u32>,
        pub max_validator_count:
          runtime_types::pallet_staking::pallet::pallet::ConfigOp<::core::primitive::u32>,
        pub chill_threshold: runtime_types::pallet_staking::pallet::pallet::ConfigOp<
          runtime_types::sp_arithmetic::per_things::Percent,
        >,
        pub min_commission: runtime_types::pallet_staking::pallet::pallet::ConfigOp<
          runtime_types::sp_arithmetic::per_things::Perbill,
        >,
      }
      impl ::subxt::Call for SetStakingConfigs {
        const PALLET: &'static str = "Staking";
        const FUNCTION: &'static str = "set_staking_configs";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct ChillOther {
        pub controller: ::subxt::sp_core::crypto::AccountId32,
      }
      impl ::subxt::Call for ChillOther {
        const PALLET: &'static str = "Staking";
        const FUNCTION: &'static str = "chill_other";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct ForceApplyMinCommission {
        pub validator_stash: ::subxt::sp_core::crypto::AccountId32,
      }
      impl ::subxt::Call for ForceApplyMinCommission {
        const PALLET: &'static str = "Staking";
        const FUNCTION: &'static str = "force_apply_min_commission";
      }
      pub struct TransactionApi<'a, T: ::subxt::Config, X> {
        client: &'a ::subxt::Client<T>,
        marker: ::core::marker::PhantomData<X>,
      }
      impl<'a, T, X> TransactionApi<'a, T, X>
      where
        T: ::subxt::Config,
        X: ::subxt::extrinsic::ExtrinsicParams<T>,
      {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self {
            client,
            marker: ::core::marker::PhantomData,
          }
        }
        pub fn bond(
          &self,
          controller: ::subxt::sp_runtime::MultiAddress<
            ::subxt::sp_core::crypto::AccountId32,
            ::core::primitive::u32,
          >,
          value: ::core::primitive::u128,
          payee: runtime_types::pallet_staking::RewardDestination<
            ::subxt::sp_core::crypto::AccountId32,
          >,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, Bond, DispatchError, root_mod::Event> {
          let call = Bond {
            controller,
            value,
            payee,
          };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn bond_extra(
          &self,
          max_additional: ::core::primitive::u128,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, BondExtra, DispatchError, root_mod::Event>
        {
          let call = BondExtra { max_additional };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn unbond(
          &self,
          value: ::core::primitive::u128,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, Unbond, DispatchError, root_mod::Event>
        {
          let call = Unbond { value };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn withdraw_unbonded(
          &self,
          num_slashing_spans: ::core::primitive::u32,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, WithdrawUnbonded, DispatchError, root_mod::Event>
        {
          let call = WithdrawUnbonded { num_slashing_spans };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn validate(
          &self,
          prefs: runtime_types::pallet_staking::ValidatorPrefs,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, Validate, DispatchError, root_mod::Event>
        {
          let call = Validate { prefs };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn nominate(
          &self,
          targets: ::std::vec::Vec<
            ::subxt::sp_runtime::MultiAddress<
              ::subxt::sp_core::crypto::AccountId32,
              ::core::primitive::u32,
            >,
          >,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, Nominate, DispatchError, root_mod::Event>
        {
          let call = Nominate { targets };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn chill(
          &self,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, Chill, DispatchError, root_mod::Event>
        {
          let call = Chill {};
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn set_payee(
          &self,
          payee: runtime_types::pallet_staking::RewardDestination<
            ::subxt::sp_core::crypto::AccountId32,
          >,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, SetPayee, DispatchError, root_mod::Event>
        {
          let call = SetPayee { payee };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn set_controller(
          &self,
          controller: ::subxt::sp_runtime::MultiAddress<
            ::subxt::sp_core::crypto::AccountId32,
            ::core::primitive::u32,
          >,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, SetController, DispatchError, root_mod::Event>
        {
          let call = SetController { controller };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn set_validator_count(
          &self,
          new: ::core::primitive::u32,
        ) -> ::subxt::SubmittableExtrinsic<
          'a,
          T,
          X,
          SetValidatorCount,
          DispatchError,
          root_mod::Event,
        > {
          let call = SetValidatorCount { new };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn increase_validator_count(
          &self,
          additional: ::core::primitive::u32,
        ) -> ::subxt::SubmittableExtrinsic<
          'a,
          T,
          X,
          IncreaseValidatorCount,
          DispatchError,
          root_mod::Event,
        > {
          let call = IncreaseValidatorCount { additional };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn scale_validator_count(
          &self,
          factor: runtime_types::sp_arithmetic::per_things::Percent,
        ) -> ::subxt::SubmittableExtrinsic<
          'a,
          T,
          X,
          ScaleValidatorCount,
          DispatchError,
          root_mod::Event,
        > {
          let call = ScaleValidatorCount { factor };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn force_no_eras(
          &self,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, ForceNoEras, DispatchError, root_mod::Event>
        {
          let call = ForceNoEras {};
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn force_new_era(
          &self,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, ForceNewEra, DispatchError, root_mod::Event>
        {
          let call = ForceNewEra {};
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn set_invulnerables(
          &self,
          invulnerables: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, SetInvulnerables, DispatchError, root_mod::Event>
        {
          let call = SetInvulnerables { invulnerables };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn force_unstake(
          &self,
          stash: ::subxt::sp_core::crypto::AccountId32,
          num_slashing_spans: ::core::primitive::u32,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, ForceUnstake, DispatchError, root_mod::Event>
        {
          let call = ForceUnstake {
            stash,
            num_slashing_spans,
          };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn force_new_era_always(
          &self,
        ) -> ::subxt::SubmittableExtrinsic<
          'a,
          T,
          X,
          ForceNewEraAlways,
          DispatchError,
          root_mod::Event,
        > {
          let call = ForceNewEraAlways {};
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn cancel_deferred_slash(
          &self,
          era: ::core::primitive::u32,
          slash_indices: ::std::vec::Vec<::core::primitive::u32>,
        ) -> ::subxt::SubmittableExtrinsic<
          'a,
          T,
          X,
          CancelDeferredSlash,
          DispatchError,
          root_mod::Event,
        > {
          let call = CancelDeferredSlash { era, slash_indices };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn payout_stakers(
          &self,
          validator_stash: ::subxt::sp_core::crypto::AccountId32,
          era: ::core::primitive::u32,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, PayoutStakers, DispatchError, root_mod::Event>
        {
          let call = PayoutStakers {
            validator_stash,
            era,
          };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn rebond(
          &self,
          value: ::core::primitive::u128,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, Rebond, DispatchError, root_mod::Event>
        {
          let call = Rebond { value };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn set_history_depth(
          &self,
          new_history_depth: ::core::primitive::u32,
          era_items_deleted: ::core::primitive::u32,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, SetHistoryDepth, DispatchError, root_mod::Event>
        {
          let call = SetHistoryDepth {
            new_history_depth,
            era_items_deleted,
          };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn reap_stash(
          &self,
          stash: ::subxt::sp_core::crypto::AccountId32,
          num_slashing_spans: ::core::primitive::u32,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, ReapStash, DispatchError, root_mod::Event>
        {
          let call = ReapStash {
            stash,
            num_slashing_spans,
          };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn kick(
          &self,
          who: ::std::vec::Vec<
            ::subxt::sp_runtime::MultiAddress<
              ::subxt::sp_core::crypto::AccountId32,
              ::core::primitive::u32,
            >,
          >,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, Kick, DispatchError, root_mod::Event> {
          let call = Kick { who };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn set_staking_configs(
          &self,
          min_nominator_bond: runtime_types::pallet_staking::pallet::pallet::ConfigOp<
            ::core::primitive::u128,
          >,
          min_validator_bond: runtime_types::pallet_staking::pallet::pallet::ConfigOp<
            ::core::primitive::u128,
          >,
          max_nominator_count: runtime_types::pallet_staking::pallet::pallet::ConfigOp<
            ::core::primitive::u32,
          >,
          max_validator_count: runtime_types::pallet_staking::pallet::pallet::ConfigOp<
            ::core::primitive::u32,
          >,
          chill_threshold: runtime_types::pallet_staking::pallet::pallet::ConfigOp<
            runtime_types::sp_arithmetic::per_things::Percent,
          >,
          min_commission: runtime_types::pallet_staking::pallet::pallet::ConfigOp<
            runtime_types::sp_arithmetic::per_things::Perbill,
          >,
        ) -> ::subxt::SubmittableExtrinsic<
          'a,
          T,
          X,
          SetStakingConfigs,
          DispatchError,
          root_mod::Event,
        > {
          let call = SetStakingConfigs {
            min_nominator_bond,
            min_validator_bond,
            max_nominator_count,
            max_validator_count,
            chill_threshold,
            min_commission,
          };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn chill_other(
          &self,
          controller: ::subxt::sp_core::crypto::AccountId32,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, ChillOther, DispatchError, root_mod::Event>
        {
          let call = ChillOther { controller };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn force_apply_min_commission(
          &self,
          validator_stash: ::subxt::sp_core::crypto::AccountId32,
        ) -> ::subxt::SubmittableExtrinsic<
          'a,
          T,
          X,
          ForceApplyMinCommission,
          DispatchError,
          root_mod::Event,
        > {
          let call = ForceApplyMinCommission { validator_stash };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
      }
    }
    pub type Event = runtime_types::pallet_staking::pallet::pallet::Event;
    pub mod events {
      use super::runtime_types;
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct EraPaid(
        pub ::core::primitive::u32,
        pub ::core::primitive::u128,
        pub ::core::primitive::u128,
      );
      impl ::subxt::Event for EraPaid {
        const PALLET: &'static str = "Staking";
        const EVENT: &'static str = "EraPaid";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Rewarded(
        pub ::subxt::sp_core::crypto::AccountId32,
        pub ::core::primitive::u128,
      );
      impl ::subxt::Event for Rewarded {
        const PALLET: &'static str = "Staking";
        const EVENT: &'static str = "Rewarded";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Slashed(
        pub ::subxt::sp_core::crypto::AccountId32,
        pub ::core::primitive::u128,
      );
      impl ::subxt::Event for Slashed {
        const PALLET: &'static str = "Staking";
        const EVENT: &'static str = "Slashed";
      }
      #[derive(
        :: subxt :: codec :: Encode,
        :: subxt :: codec :: Decode,
        Debug,
        :: subxt :: codec :: CompactAs,
      )]
      pub struct OldSlashingReportDiscarded(pub ::core::primitive::u32);
      impl ::subxt::Event for OldSlashingReportDiscarded {
        const PALLET: &'static str = "Staking";
        const EVENT: &'static str = "OldSlashingReportDiscarded";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct StakersElected;
      impl ::subxt::Event for StakersElected {
        const PALLET: &'static str = "Staking";
        const EVENT: &'static str = "StakersElected";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Bonded(
        pub ::subxt::sp_core::crypto::AccountId32,
        pub ::core::primitive::u128,
      );
      impl ::subxt::Event for Bonded {
        const PALLET: &'static str = "Staking";
        const EVENT: &'static str = "Bonded";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Unbonded(
        pub ::subxt::sp_core::crypto::AccountId32,
        pub ::core::primitive::u128,
      );
      impl ::subxt::Event for Unbonded {
        const PALLET: &'static str = "Staking";
        const EVENT: &'static str = "Unbonded";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Withdrawn(
        pub ::subxt::sp_core::crypto::AccountId32,
        pub ::core::primitive::u128,
      );
      impl ::subxt::Event for Withdrawn {
        const PALLET: &'static str = "Staking";
        const EVENT: &'static str = "Withdrawn";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Kicked(
        pub ::subxt::sp_core::crypto::AccountId32,
        pub ::subxt::sp_core::crypto::AccountId32,
      );
      impl ::subxt::Event for Kicked {
        const PALLET: &'static str = "Staking";
        const EVENT: &'static str = "Kicked";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct StakingElectionFailed;
      impl ::subxt::Event for StakingElectionFailed {
        const PALLET: &'static str = "Staking";
        const EVENT: &'static str = "StakingElectionFailed";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Chilled(pub ::subxt::sp_core::crypto::AccountId32);
      impl ::subxt::Event for Chilled {
        const PALLET: &'static str = "Staking";
        const EVENT: &'static str = "Chilled";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct PayoutStarted(
        pub ::core::primitive::u32,
        pub ::subxt::sp_core::crypto::AccountId32,
      );
      impl ::subxt::Event for PayoutStarted {
        const PALLET: &'static str = "Staking";
        const EVENT: &'static str = "PayoutStarted";
      }
    }
    pub mod storage {
      use super::runtime_types;
      pub struct HistoryDepth;
      impl ::subxt::StorageEntry for HistoryDepth {
        const PALLET: &'static str = "Staking";
        const STORAGE: &'static str = "HistoryDepth";
        type Value = ::core::primitive::u32;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct ValidatorCount;
      impl ::subxt::StorageEntry for ValidatorCount {
        const PALLET: &'static str = "Staking";
        const STORAGE: &'static str = "ValidatorCount";
        type Value = ::core::primitive::u32;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct MinimumValidatorCount;
      impl ::subxt::StorageEntry for MinimumValidatorCount {
        const PALLET: &'static str = "Staking";
        const STORAGE: &'static str = "MinimumValidatorCount";
        type Value = ::core::primitive::u32;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct Invulnerables;
      impl ::subxt::StorageEntry for Invulnerables {
        const PALLET: &'static str = "Staking";
        const STORAGE: &'static str = "Invulnerables";
        type Value = ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct Bonded<'a>(pub &'a ::subxt::sp_core::crypto::AccountId32);
      impl ::subxt::StorageEntry for Bonded<'_> {
        const PALLET: &'static str = "Staking";
        const STORAGE: &'static str = "Bonded";
        type Value = ::subxt::sp_core::crypto::AccountId32;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
            &self.0,
            ::subxt::StorageHasher::Twox64Concat,
          )])
        }
      }
      pub struct MinNominatorBond;
      impl ::subxt::StorageEntry for MinNominatorBond {
        const PALLET: &'static str = "Staking";
        const STORAGE: &'static str = "MinNominatorBond";
        type Value = ::core::primitive::u128;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct MinValidatorBond;
      impl ::subxt::StorageEntry for MinValidatorBond {
        const PALLET: &'static str = "Staking";
        const STORAGE: &'static str = "MinValidatorBond";
        type Value = ::core::primitive::u128;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct MinCommission;
      impl ::subxt::StorageEntry for MinCommission {
        const PALLET: &'static str = "Staking";
        const STORAGE: &'static str = "MinCommission";
        type Value = runtime_types::sp_arithmetic::per_things::Perbill;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct Ledger<'a>(pub &'a ::subxt::sp_core::crypto::AccountId32);
      impl ::subxt::StorageEntry for Ledger<'_> {
        const PALLET: &'static str = "Staking";
        const STORAGE: &'static str = "Ledger";
        type Value = runtime_types::pallet_staking::StakingLedger<
          ::subxt::sp_core::crypto::AccountId32,
          ::core::primitive::u128,
        >;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
            &self.0,
            ::subxt::StorageHasher::Blake2_128Concat,
          )])
        }
      }
      pub struct Payee<'a>(pub &'a ::subxt::sp_core::crypto::AccountId32);
      impl ::subxt::StorageEntry for Payee<'_> {
        const PALLET: &'static str = "Staking";
        const STORAGE: &'static str = "Payee";
        type Value =
          runtime_types::pallet_staking::RewardDestination<::subxt::sp_core::crypto::AccountId32>;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
            &self.0,
            ::subxt::StorageHasher::Twox64Concat,
          )])
        }
      }
      pub struct Validators<'a>(pub &'a ::subxt::sp_core::crypto::AccountId32);
      impl ::subxt::StorageEntry for Validators<'_> {
        const PALLET: &'static str = "Staking";
        const STORAGE: &'static str = "Validators";
        type Value = runtime_types::pallet_staking::ValidatorPrefs;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
            &self.0,
            ::subxt::StorageHasher::Twox64Concat,
          )])
        }
      }
      pub struct CounterForValidators;
      impl ::subxt::StorageEntry for CounterForValidators {
        const PALLET: &'static str = "Staking";
        const STORAGE: &'static str = "CounterForValidators";
        type Value = ::core::primitive::u32;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct MaxValidatorsCount;
      impl ::subxt::StorageEntry for MaxValidatorsCount {
        const PALLET: &'static str = "Staking";
        const STORAGE: &'static str = "MaxValidatorsCount";
        type Value = ::core::primitive::u32;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct Nominators<'a>(pub &'a ::subxt::sp_core::crypto::AccountId32);
      impl ::subxt::StorageEntry for Nominators<'_> {
        const PALLET: &'static str = "Staking";
        const STORAGE: &'static str = "Nominators";
        type Value = runtime_types::pallet_staking::Nominations;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
            &self.0,
            ::subxt::StorageHasher::Twox64Concat,
          )])
        }
      }
      pub struct CounterForNominators;
      impl ::subxt::StorageEntry for CounterForNominators {
        const PALLET: &'static str = "Staking";
        const STORAGE: &'static str = "CounterForNominators";
        type Value = ::core::primitive::u32;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct MaxNominatorsCount;
      impl ::subxt::StorageEntry for MaxNominatorsCount {
        const PALLET: &'static str = "Staking";
        const STORAGE: &'static str = "MaxNominatorsCount";
        type Value = ::core::primitive::u32;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct CurrentEra;
      impl ::subxt::StorageEntry for CurrentEra {
        const PALLET: &'static str = "Staking";
        const STORAGE: &'static str = "CurrentEra";
        type Value = ::core::primitive::u32;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct ActiveEra;
      impl ::subxt::StorageEntry for ActiveEra {
        const PALLET: &'static str = "Staking";
        const STORAGE: &'static str = "ActiveEra";
        type Value = runtime_types::pallet_staking::ActiveEraInfo;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct ErasStartSessionIndex<'a>(pub &'a ::core::primitive::u32);
      impl ::subxt::StorageEntry for ErasStartSessionIndex<'_> {
        const PALLET: &'static str = "Staking";
        const STORAGE: &'static str = "ErasStartSessionIndex";
        type Value = ::core::primitive::u32;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
            &self.0,
            ::subxt::StorageHasher::Twox64Concat,
          )])
        }
      }
      pub struct ErasStakers<'a>(
        pub &'a ::core::primitive::u32,
        pub &'a ::subxt::sp_core::crypto::AccountId32,
      );
      impl ::subxt::StorageEntry for ErasStakers<'_> {
        const PALLET: &'static str = "Staking";
        const STORAGE: &'static str = "ErasStakers";
        type Value = runtime_types::pallet_staking::Exposure<
          ::subxt::sp_core::crypto::AccountId32,
          ::core::primitive::u128,
        >;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Map(vec![
            ::subxt::StorageMapKey::new(&self.0, ::subxt::StorageHasher::Twox64Concat),
            ::subxt::StorageMapKey::new(&self.1, ::subxt::StorageHasher::Twox64Concat),
          ])
        }
      }
      pub struct ErasStakersClipped<'a>(
        pub &'a ::core::primitive::u32,
        pub &'a ::subxt::sp_core::crypto::AccountId32,
      );
      impl ::subxt::StorageEntry for ErasStakersClipped<'_> {
        const PALLET: &'static str = "Staking";
        const STORAGE: &'static str = "ErasStakersClipped";
        type Value = runtime_types::pallet_staking::Exposure<
          ::subxt::sp_core::crypto::AccountId32,
          ::core::primitive::u128,
        >;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Map(vec![
            ::subxt::StorageMapKey::new(&self.0, ::subxt::StorageHasher::Twox64Concat),
            ::subxt::StorageMapKey::new(&self.1, ::subxt::StorageHasher::Twox64Concat),
          ])
        }
      }
      pub struct ErasValidatorPrefs<'a>(
        pub &'a ::core::primitive::u32,
        pub &'a ::subxt::sp_core::crypto::AccountId32,
      );
      impl ::subxt::StorageEntry for ErasValidatorPrefs<'_> {
        const PALLET: &'static str = "Staking";
        const STORAGE: &'static str = "ErasValidatorPrefs";
        type Value = runtime_types::pallet_staking::ValidatorPrefs;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Map(vec![
            ::subxt::StorageMapKey::new(&self.0, ::subxt::StorageHasher::Twox64Concat),
            ::subxt::StorageMapKey::new(&self.1, ::subxt::StorageHasher::Twox64Concat),
          ])
        }
      }
      pub struct ErasValidatorReward<'a>(pub &'a ::core::primitive::u32);
      impl ::subxt::StorageEntry for ErasValidatorReward<'_> {
        const PALLET: &'static str = "Staking";
        const STORAGE: &'static str = "ErasValidatorReward";
        type Value = ::core::primitive::u128;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
            &self.0,
            ::subxt::StorageHasher::Twox64Concat,
          )])
        }
      }
      pub struct ErasRewardPoints<'a>(pub &'a ::core::primitive::u32);
      impl ::subxt::StorageEntry for ErasRewardPoints<'_> {
        const PALLET: &'static str = "Staking";
        const STORAGE: &'static str = "ErasRewardPoints";
        type Value =
          runtime_types::pallet_staking::EraRewardPoints<::subxt::sp_core::crypto::AccountId32>;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
            &self.0,
            ::subxt::StorageHasher::Twox64Concat,
          )])
        }
      }
      pub struct ErasTotalStake<'a>(pub &'a ::core::primitive::u32);
      impl ::subxt::StorageEntry for ErasTotalStake<'_> {
        const PALLET: &'static str = "Staking";
        const STORAGE: &'static str = "ErasTotalStake";
        type Value = ::core::primitive::u128;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
            &self.0,
            ::subxt::StorageHasher::Twox64Concat,
          )])
        }
      }
      pub struct ForceEra;
      impl ::subxt::StorageEntry for ForceEra {
        const PALLET: &'static str = "Staking";
        const STORAGE: &'static str = "ForceEra";
        type Value = runtime_types::pallet_staking::Forcing;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct SlashRewardFraction;
      impl ::subxt::StorageEntry for SlashRewardFraction {
        const PALLET: &'static str = "Staking";
        const STORAGE: &'static str = "SlashRewardFraction";
        type Value = runtime_types::sp_arithmetic::per_things::Perbill;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct CanceledSlashPayout;
      impl ::subxt::StorageEntry for CanceledSlashPayout {
        const PALLET: &'static str = "Staking";
        const STORAGE: &'static str = "CanceledSlashPayout";
        type Value = ::core::primitive::u128;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct UnappliedSlashes<'a>(pub &'a ::core::primitive::u32);
      impl ::subxt::StorageEntry for UnappliedSlashes<'_> {
        const PALLET: &'static str = "Staking";
        const STORAGE: &'static str = "UnappliedSlashes";
        type Value = ::std::vec::Vec<
          runtime_types::pallet_staking::UnappliedSlash<
            ::subxt::sp_core::crypto::AccountId32,
            ::core::primitive::u128,
          >,
        >;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
            &self.0,
            ::subxt::StorageHasher::Twox64Concat,
          )])
        }
      }
      pub struct BondedEras;
      impl ::subxt::StorageEntry for BondedEras {
        const PALLET: &'static str = "Staking";
        const STORAGE: &'static str = "BondedEras";
        type Value = ::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct ValidatorSlashInEra<'a>(
        pub &'a ::core::primitive::u32,
        pub &'a ::subxt::sp_core::crypto::AccountId32,
      );
      impl ::subxt::StorageEntry for ValidatorSlashInEra<'_> {
        const PALLET: &'static str = "Staking";
        const STORAGE: &'static str = "ValidatorSlashInEra";
        type Value = (
          runtime_types::sp_arithmetic::per_things::Perbill,
          ::core::primitive::u128,
        );
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Map(vec![
            ::subxt::StorageMapKey::new(&self.0, ::subxt::StorageHasher::Twox64Concat),
            ::subxt::StorageMapKey::new(&self.1, ::subxt::StorageHasher::Twox64Concat),
          ])
        }
      }
      pub struct NominatorSlashInEra<'a>(
        pub &'a ::core::primitive::u32,
        pub &'a ::subxt::sp_core::crypto::AccountId32,
      );
      impl ::subxt::StorageEntry for NominatorSlashInEra<'_> {
        const PALLET: &'static str = "Staking";
        const STORAGE: &'static str = "NominatorSlashInEra";
        type Value = ::core::primitive::u128;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Map(vec![
            ::subxt::StorageMapKey::new(&self.0, ::subxt::StorageHasher::Twox64Concat),
            ::subxt::StorageMapKey::new(&self.1, ::subxt::StorageHasher::Twox64Concat),
          ])
        }
      }
      pub struct SlashingSpans<'a>(pub &'a ::subxt::sp_core::crypto::AccountId32);
      impl ::subxt::StorageEntry for SlashingSpans<'_> {
        const PALLET: &'static str = "Staking";
        const STORAGE: &'static str = "SlashingSpans";
        type Value = runtime_types::pallet_staking::slashing::SlashingSpans;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
            &self.0,
            ::subxt::StorageHasher::Twox64Concat,
          )])
        }
      }
      pub struct SpanSlash<'a>(
        pub &'a ::subxt::sp_core::crypto::AccountId32,
        pub &'a ::core::primitive::u32,
      );
      impl ::subxt::StorageEntry for SpanSlash<'_> {
        const PALLET: &'static str = "Staking";
        const STORAGE: &'static str = "SpanSlash";
        type Value = runtime_types::pallet_staking::slashing::SpanRecord<::core::primitive::u128>;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
            &(&self.0, &self.1),
            ::subxt::StorageHasher::Twox64Concat,
          )])
        }
      }
      pub struct EarliestUnappliedSlash;
      impl ::subxt::StorageEntry for EarliestUnappliedSlash {
        const PALLET: &'static str = "Staking";
        const STORAGE: &'static str = "EarliestUnappliedSlash";
        type Value = ::core::primitive::u32;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct CurrentPlannedSession;
      impl ::subxt::StorageEntry for CurrentPlannedSession {
        const PALLET: &'static str = "Staking";
        const STORAGE: &'static str = "CurrentPlannedSession";
        type Value = ::core::primitive::u32;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct OffendingValidators;
      impl ::subxt::StorageEntry for OffendingValidators {
        const PALLET: &'static str = "Staking";
        const STORAGE: &'static str = "OffendingValidators";
        type Value = ::std::vec::Vec<(::core::primitive::u32, ::core::primitive::bool)>;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct StorageVersion;
      impl ::subxt::StorageEntry for StorageVersion {
        const PALLET: &'static str = "Staking";
        const STORAGE: &'static str = "StorageVersion";
        type Value = runtime_types::pallet_staking::Releases;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct ChillThreshold;
      impl ::subxt::StorageEntry for ChillThreshold {
        const PALLET: &'static str = "Staking";
        const STORAGE: &'static str = "ChillThreshold";
        type Value = runtime_types::sp_arithmetic::per_things::Percent;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct StorageApi<'a, T: ::subxt::Config> {
        client: &'a ::subxt::Client<T>,
      }
      impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self { client }
        }
        pub async fn history_depth(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
          let entry = HistoryDepth;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn validator_count(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
          let entry = ValidatorCount;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn minimum_validator_count(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
          let entry = MinimumValidatorCount;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn invulnerables(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
          ::subxt::BasicError,
        > {
          let entry = Invulnerables;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn bonded(
          &self,
          _0: &::subxt::sp_core::crypto::AccountId32,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
          ::subxt::BasicError,
        > {
          let entry = Bonded(_0);
          self.client.storage().fetch(&entry, hash).await
        }
        pub async fn bonded_iter(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Bonded<'a>>, ::subxt::BasicError>
        {
          self.client.storage().iter(hash).await
        }
        pub async fn min_nominator_bond(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError> {
          let entry = MinNominatorBond;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn min_validator_bond(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError> {
          let entry = MinValidatorBond;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn min_commission(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          runtime_types::sp_arithmetic::per_things::Perbill,
          ::subxt::BasicError,
        > {
          let entry = MinCommission;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn ledger(
          &self,
          _0: &::subxt::sp_core::crypto::AccountId32,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::core::option::Option<
            runtime_types::pallet_staking::StakingLedger<
              ::subxt::sp_core::crypto::AccountId32,
              ::core::primitive::u128,
            >,
          >,
          ::subxt::BasicError,
        > {
          let entry = Ledger(_0);
          self.client.storage().fetch(&entry, hash).await
        }
        pub async fn ledger_iter(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Ledger<'a>>, ::subxt::BasicError>
        {
          self.client.storage().iter(hash).await
        }
        pub async fn payee(
          &self,
          _0: &::subxt::sp_core::crypto::AccountId32,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          runtime_types::pallet_staking::RewardDestination<::subxt::sp_core::crypto::AccountId32>,
          ::subxt::BasicError,
        > {
          let entry = Payee(_0);
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn payee_iter(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Payee<'a>>, ::subxt::BasicError>
        {
          self.client.storage().iter(hash).await
        }
        pub async fn validators(
          &self,
          _0: &::subxt::sp_core::crypto::AccountId32,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          runtime_types::pallet_staking::ValidatorPrefs,
          ::subxt::BasicError,
        > {
          let entry = Validators(_0);
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn validators_iter(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Validators<'a>>, ::subxt::BasicError>
        {
          self.client.storage().iter(hash).await
        }
        pub async fn counter_for_validators(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
          let entry = CounterForValidators;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn max_validators_count(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::core::option::Option<::core::primitive::u32>,
          ::subxt::BasicError,
        > {
          let entry = MaxValidatorsCount;
          self.client.storage().fetch(&entry, hash).await
        }
        pub async fn nominators(
          &self,
          _0: &::subxt::sp_core::crypto::AccountId32,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::core::option::Option<runtime_types::pallet_staking::Nominations>,
          ::subxt::BasicError,
        > {
          let entry = Nominators(_0);
          self.client.storage().fetch(&entry, hash).await
        }
        pub async fn nominators_iter(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Nominators<'a>>, ::subxt::BasicError>
        {
          self.client.storage().iter(hash).await
        }
        pub async fn counter_for_nominators(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
          let entry = CounterForNominators;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn max_nominators_count(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::core::option::Option<::core::primitive::u32>,
          ::subxt::BasicError,
        > {
          let entry = MaxNominatorsCount;
          self.client.storage().fetch(&entry, hash).await
        }
        pub async fn current_era(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::core::option::Option<::core::primitive::u32>,
          ::subxt::BasicError,
        > {
          let entry = CurrentEra;
          self.client.storage().fetch(&entry, hash).await
        }
        pub async fn active_era(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::core::option::Option<runtime_types::pallet_staking::ActiveEraInfo>,
          ::subxt::BasicError,
        > {
          let entry = ActiveEra;
          self.client.storage().fetch(&entry, hash).await
        }
        pub async fn eras_start_session_index(
          &self,
          _0: &::core::primitive::u32,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::core::option::Option<::core::primitive::u32>,
          ::subxt::BasicError,
        > {
          let entry = ErasStartSessionIndex(_0);
          self.client.storage().fetch(&entry, hash).await
        }
        pub async fn eras_start_session_index_iter(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::subxt::KeyIter<'a, T, ErasStartSessionIndex<'a>>,
          ::subxt::BasicError,
        > {
          self.client.storage().iter(hash).await
        }
        pub async fn eras_stakers(
          &self,
          _0: &::core::primitive::u32,
          _1: &::subxt::sp_core::crypto::AccountId32,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          runtime_types::pallet_staking::Exposure<
            ::subxt::sp_core::crypto::AccountId32,
            ::core::primitive::u128,
          >,
          ::subxt::BasicError,
        > {
          let entry = ErasStakers(_0, _1);
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn eras_stakers_iter(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::subxt::KeyIter<'a, T, ErasStakers<'a>>, ::subxt::BasicError>
        {
          self.client.storage().iter(hash).await
        }
        pub async fn eras_stakers_clipped(
          &self,
          _0: &::core::primitive::u32,
          _1: &::subxt::sp_core::crypto::AccountId32,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          runtime_types::pallet_staking::Exposure<
            ::subxt::sp_core::crypto::AccountId32,
            ::core::primitive::u128,
          >,
          ::subxt::BasicError,
        > {
          let entry = ErasStakersClipped(_0, _1);
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn eras_stakers_clipped_iter(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::subxt::KeyIter<'a, T, ErasStakersClipped<'a>>,
          ::subxt::BasicError,
        > {
          self.client.storage().iter(hash).await
        }
        pub async fn eras_validator_prefs(
          &self,
          _0: &::core::primitive::u32,
          _1: &::subxt::sp_core::crypto::AccountId32,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          runtime_types::pallet_staking::ValidatorPrefs,
          ::subxt::BasicError,
        > {
          let entry = ErasValidatorPrefs(_0, _1);
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn eras_validator_prefs_iter(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::subxt::KeyIter<'a, T, ErasValidatorPrefs<'a>>,
          ::subxt::BasicError,
        > {
          self.client.storage().iter(hash).await
        }
        pub async fn eras_validator_reward(
          &self,
          _0: &::core::primitive::u32,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::core::option::Option<::core::primitive::u128>,
          ::subxt::BasicError,
        > {
          let entry = ErasValidatorReward(_0);
          self.client.storage().fetch(&entry, hash).await
        }
        pub async fn eras_validator_reward_iter(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::subxt::KeyIter<'a, T, ErasValidatorReward<'a>>,
          ::subxt::BasicError,
        > {
          self.client.storage().iter(hash).await
        }
        pub async fn eras_reward_points(
          &self,
          _0: &::core::primitive::u32,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          runtime_types::pallet_staking::EraRewardPoints<::subxt::sp_core::crypto::AccountId32>,
          ::subxt::BasicError,
        > {
          let entry = ErasRewardPoints(_0);
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn eras_reward_points_iter(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::subxt::KeyIter<'a, T, ErasRewardPoints<'a>>,
          ::subxt::BasicError,
        > {
          self.client.storage().iter(hash).await
        }
        pub async fn eras_total_stake(
          &self,
          _0: &::core::primitive::u32,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError> {
          let entry = ErasTotalStake(_0);
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn eras_total_stake_iter(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::subxt::KeyIter<'a, T, ErasTotalStake<'a>>, ::subxt::BasicError>
        {
          self.client.storage().iter(hash).await
        }
        pub async fn force_era(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<runtime_types::pallet_staking::Forcing, ::subxt::BasicError>
        {
          let entry = ForceEra;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn slash_reward_fraction(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          runtime_types::sp_arithmetic::per_things::Perbill,
          ::subxt::BasicError,
        > {
          let entry = SlashRewardFraction;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn canceled_slash_payout(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError> {
          let entry = CanceledSlashPayout;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn unapplied_slashes(
          &self,
          _0: &::core::primitive::u32,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::std::vec::Vec<
            runtime_types::pallet_staking::UnappliedSlash<
              ::subxt::sp_core::crypto::AccountId32,
              ::core::primitive::u128,
            >,
          >,
          ::subxt::BasicError,
        > {
          let entry = UnappliedSlashes(_0);
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn unapplied_slashes_iter(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::subxt::KeyIter<'a, T, UnappliedSlashes<'a>>,
          ::subxt::BasicError,
        > {
          self.client.storage().iter(hash).await
        }
        pub async fn bonded_eras(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>,
          ::subxt::BasicError,
        > {
          let entry = BondedEras;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn validator_slash_in_era(
          &self,
          _0: &::core::primitive::u32,
          _1: &::subxt::sp_core::crypto::AccountId32,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::core::option::Option<(
            runtime_types::sp_arithmetic::per_things::Perbill,
            ::core::primitive::u128,
          )>,
          ::subxt::BasicError,
        > {
          let entry = ValidatorSlashInEra(_0, _1);
          self.client.storage().fetch(&entry, hash).await
        }
        pub async fn validator_slash_in_era_iter(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::subxt::KeyIter<'a, T, ValidatorSlashInEra<'a>>,
          ::subxt::BasicError,
        > {
          self.client.storage().iter(hash).await
        }
        pub async fn nominator_slash_in_era(
          &self,
          _0: &::core::primitive::u32,
          _1: &::subxt::sp_core::crypto::AccountId32,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::core::option::Option<::core::primitive::u128>,
          ::subxt::BasicError,
        > {
          let entry = NominatorSlashInEra(_0, _1);
          self.client.storage().fetch(&entry, hash).await
        }
        pub async fn nominator_slash_in_era_iter(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::subxt::KeyIter<'a, T, NominatorSlashInEra<'a>>,
          ::subxt::BasicError,
        > {
          self.client.storage().iter(hash).await
        }
        pub async fn slashing_spans(
          &self,
          _0: &::subxt::sp_core::crypto::AccountId32,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::core::option::Option<runtime_types::pallet_staking::slashing::SlashingSpans>,
          ::subxt::BasicError,
        > {
          let entry = SlashingSpans(_0);
          self.client.storage().fetch(&entry, hash).await
        }
        pub async fn slashing_spans_iter(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::subxt::KeyIter<'a, T, SlashingSpans<'a>>, ::subxt::BasicError>
        {
          self.client.storage().iter(hash).await
        }
        pub async fn span_slash(
          &self,
          _0: &::subxt::sp_core::crypto::AccountId32,
          _1: &::core::primitive::u32,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          runtime_types::pallet_staking::slashing::SpanRecord<::core::primitive::u128>,
          ::subxt::BasicError,
        > {
          let entry = SpanSlash(_0, _1);
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn span_slash_iter(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::subxt::KeyIter<'a, T, SpanSlash<'a>>, ::subxt::BasicError>
        {
          self.client.storage().iter(hash).await
        }
        pub async fn earliest_unapplied_slash(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::core::option::Option<::core::primitive::u32>,
          ::subxt::BasicError,
        > {
          let entry = EarliestUnappliedSlash;
          self.client.storage().fetch(&entry, hash).await
        }
        pub async fn current_planned_session(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
          let entry = CurrentPlannedSession;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn offending_validators(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::std::vec::Vec<(::core::primitive::u32, ::core::primitive::bool)>,
          ::subxt::BasicError,
        > {
          let entry = OffendingValidators;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn storage_version(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<runtime_types::pallet_staking::Releases, ::subxt::BasicError>
        {
          let entry = StorageVersion;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn chill_threshold(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::core::option::Option<runtime_types::sp_arithmetic::per_things::Percent>,
          ::subxt::BasicError,
        > {
          let entry = ChillThreshold;
          self.client.storage().fetch(&entry, hash).await
        }
      }
    }
    pub mod constants {
      use super::runtime_types;
      pub struct ConstantsApi<'a, T: ::subxt::Config> {
        client: &'a ::subxt::Client<T>,
      }
      impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self { client }
        }
        pub fn max_nominations(
          &self,
        ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
          let pallet = self.client.metadata().pallet("Staking")?;
          let constant = pallet.constant("MaxNominations")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn sessions_per_era(
          &self,
        ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
          let pallet = self.client.metadata().pallet("Staking")?;
          let constant = pallet.constant("SessionsPerEra")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn bonding_duration(
          &self,
        ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
          let pallet = self.client.metadata().pallet("Staking")?;
          let constant = pallet.constant("BondingDuration")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn slash_defer_duration(
          &self,
        ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
          let pallet = self.client.metadata().pallet("Staking")?;
          let constant = pallet.constant("SlashDeferDuration")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn max_nominator_rewarded_per_validator(
          &self,
        ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
          let pallet = self.client.metadata().pallet("Staking")?;
          let constant = pallet.constant("MaxNominatorRewardedPerValidator")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn max_unlocking_chunks(
          &self,
        ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
          let pallet = self.client.metadata().pallet("Staking")?;
          let constant = pallet.constant("MaxUnlockingChunks")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
      }
    }
  }
  pub mod offences {
    use super::root_mod;
    use super::runtime_types;
    pub type Event = runtime_types::pallet_offences::pallet::Event;
    pub mod events {
      use super::runtime_types;
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Offence {
        pub kind: [::core::primitive::u8; 16usize],
        pub timeslot: ::std::vec::Vec<::core::primitive::u8>,
      }
      impl ::subxt::Event for Offence {
        const PALLET: &'static str = "Offences";
        const EVENT: &'static str = "Offence";
      }
    }
    pub mod storage {
      use super::runtime_types;
      pub struct Reports<'a>(pub &'a ::subxt::sp_core::H256);
      impl ::subxt::StorageEntry for Reports<'_> {
        const PALLET: &'static str = "Offences";
        const STORAGE: &'static str = "Reports";
        type Value = runtime_types::sp_staking::offence::OffenceDetails<
          ::subxt::sp_core::crypto::AccountId32,
          (
            ::subxt::sp_core::crypto::AccountId32,
            runtime_types::pallet_staking::Exposure<
              ::subxt::sp_core::crypto::AccountId32,
              ::core::primitive::u128,
            >,
          ),
        >;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
            &self.0,
            ::subxt::StorageHasher::Twox64Concat,
          )])
        }
      }
      pub struct ConcurrentReportsIndex<'a>(
        pub &'a [::core::primitive::u8; 16usize],
        pub &'a [::core::primitive::u8],
      );
      impl ::subxt::StorageEntry for ConcurrentReportsIndex<'_> {
        const PALLET: &'static str = "Offences";
        const STORAGE: &'static str = "ConcurrentReportsIndex";
        type Value = ::std::vec::Vec<::subxt::sp_core::H256>;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Map(vec![
            ::subxt::StorageMapKey::new(&self.0, ::subxt::StorageHasher::Twox64Concat),
            ::subxt::StorageMapKey::new(&self.1, ::subxt::StorageHasher::Twox64Concat),
          ])
        }
      }
      pub struct ReportsByKindIndex<'a>(pub &'a [::core::primitive::u8; 16usize]);
      impl ::subxt::StorageEntry for ReportsByKindIndex<'_> {
        const PALLET: &'static str = "Offences";
        const STORAGE: &'static str = "ReportsByKindIndex";
        type Value = ::std::vec::Vec<::core::primitive::u8>;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
            &self.0,
            ::subxt::StorageHasher::Twox64Concat,
          )])
        }
      }
      pub struct StorageApi<'a, T: ::subxt::Config> {
        client: &'a ::subxt::Client<T>,
      }
      impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self { client }
        }
        pub async fn reports(
          &self,
          _0: &::subxt::sp_core::H256,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::core::option::Option<
            runtime_types::sp_staking::offence::OffenceDetails<
              ::subxt::sp_core::crypto::AccountId32,
              (
                ::subxt::sp_core::crypto::AccountId32,
                runtime_types::pallet_staking::Exposure<
                  ::subxt::sp_core::crypto::AccountId32,
                  ::core::primitive::u128,
                >,
              ),
            >,
          >,
          ::subxt::BasicError,
        > {
          let entry = Reports(_0);
          self.client.storage().fetch(&entry, hash).await
        }
        pub async fn reports_iter(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Reports<'a>>, ::subxt::BasicError>
        {
          self.client.storage().iter(hash).await
        }
        pub async fn concurrent_reports_index(
          &self,
          _0: &[::core::primitive::u8; 16usize],
          _1: &[::core::primitive::u8],
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::std::vec::Vec<::subxt::sp_core::H256>, ::subxt::BasicError>
        {
          let entry = ConcurrentReportsIndex(_0, _1);
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn concurrent_reports_index_iter(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::subxt::KeyIter<'a, T, ConcurrentReportsIndex<'a>>,
          ::subxt::BasicError,
        > {
          self.client.storage().iter(hash).await
        }
        pub async fn reports_by_kind_index(
          &self,
          _0: &[::core::primitive::u8; 16usize],
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::std::vec::Vec<::core::primitive::u8>, ::subxt::BasicError>
        {
          let entry = ReportsByKindIndex(_0);
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn reports_by_kind_index_iter(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::subxt::KeyIter<'a, T, ReportsByKindIndex<'a>>,
          ::subxt::BasicError,
        > {
          self.client.storage().iter(hash).await
        }
      }
    }
  }
  pub mod historical {
    use super::root_mod;
    use super::runtime_types;
  }
  pub mod session {
    use super::root_mod;
    use super::runtime_types;
    pub mod calls {
      use super::root_mod;
      use super::runtime_types;
      type DispatchError = runtime_types::sp_runtime::DispatchError;
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct SetKeys {
        pub keys: runtime_types::lagoon_runtime::SessionKeys,
        pub proof: ::std::vec::Vec<::core::primitive::u8>,
      }
      impl ::subxt::Call for SetKeys {
        const PALLET: &'static str = "Session";
        const FUNCTION: &'static str = "set_keys";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct PurgeKeys;
      impl ::subxt::Call for PurgeKeys {
        const PALLET: &'static str = "Session";
        const FUNCTION: &'static str = "purge_keys";
      }
      pub struct TransactionApi<'a, T: ::subxt::Config, X> {
        client: &'a ::subxt::Client<T>,
        marker: ::core::marker::PhantomData<X>,
      }
      impl<'a, T, X> TransactionApi<'a, T, X>
      where
        T: ::subxt::Config,
        X: ::subxt::extrinsic::ExtrinsicParams<T>,
      {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self {
            client,
            marker: ::core::marker::PhantomData,
          }
        }
        pub fn set_keys(
          &self,
          keys: runtime_types::lagoon_runtime::SessionKeys,
          proof: ::std::vec::Vec<::core::primitive::u8>,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, SetKeys, DispatchError, root_mod::Event>
        {
          let call = SetKeys { keys, proof };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn purge_keys(
          &self,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, PurgeKeys, DispatchError, root_mod::Event>
        {
          let call = PurgeKeys {};
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
      }
    }
    pub type Event = runtime_types::pallet_session::pallet::Event;
    pub mod events {
      use super::runtime_types;
      #[derive(
        :: subxt :: codec :: Encode,
        :: subxt :: codec :: Decode,
        Debug,
        :: subxt :: codec :: CompactAs,
      )]
      pub struct NewSession {
        pub session_index: ::core::primitive::u32,
      }
      impl ::subxt::Event for NewSession {
        const PALLET: &'static str = "Session";
        const EVENT: &'static str = "NewSession";
      }
    }
    pub mod storage {
      use super::runtime_types;
      pub struct Validators;
      impl ::subxt::StorageEntry for Validators {
        const PALLET: &'static str = "Session";
        const STORAGE: &'static str = "Validators";
        type Value = ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct CurrentIndex;
      impl ::subxt::StorageEntry for CurrentIndex {
        const PALLET: &'static str = "Session";
        const STORAGE: &'static str = "CurrentIndex";
        type Value = ::core::primitive::u32;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct QueuedChanged;
      impl ::subxt::StorageEntry for QueuedChanged {
        const PALLET: &'static str = "Session";
        const STORAGE: &'static str = "QueuedChanged";
        type Value = ::core::primitive::bool;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct QueuedKeys;
      impl ::subxt::StorageEntry for QueuedKeys {
        const PALLET: &'static str = "Session";
        const STORAGE: &'static str = "QueuedKeys";
        type Value = ::std::vec::Vec<(
          ::subxt::sp_core::crypto::AccountId32,
          runtime_types::lagoon_runtime::SessionKeys,
        )>;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct DisabledValidators;
      impl ::subxt::StorageEntry for DisabledValidators {
        const PALLET: &'static str = "Session";
        const STORAGE: &'static str = "DisabledValidators";
        type Value = ::std::vec::Vec<::core::primitive::u32>;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct NextKeys<'a>(pub &'a ::subxt::sp_core::crypto::AccountId32);
      impl ::subxt::StorageEntry for NextKeys<'_> {
        const PALLET: &'static str = "Session";
        const STORAGE: &'static str = "NextKeys";
        type Value = runtime_types::lagoon_runtime::SessionKeys;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
            &self.0,
            ::subxt::StorageHasher::Twox64Concat,
          )])
        }
      }
      pub struct KeyOwner<'a>(
        pub &'a runtime_types::sp_core::crypto::KeyTypeId,
        pub &'a [::core::primitive::u8],
      );
      impl ::subxt::StorageEntry for KeyOwner<'_> {
        const PALLET: &'static str = "Session";
        const STORAGE: &'static str = "KeyOwner";
        type Value = ::subxt::sp_core::crypto::AccountId32;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
            &(&self.0, &self.1),
            ::subxt::StorageHasher::Twox64Concat,
          )])
        }
      }
      pub struct StorageApi<'a, T: ::subxt::Config> {
        client: &'a ::subxt::Client<T>,
      }
      impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self { client }
        }
        pub async fn validators(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
          ::subxt::BasicError,
        > {
          let entry = Validators;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn current_index(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
          let entry = CurrentIndex;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn queued_changed(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::core::primitive::bool, ::subxt::BasicError> {
          let entry = QueuedChanged;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn queued_keys(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::std::vec::Vec<(
            ::subxt::sp_core::crypto::AccountId32,
            runtime_types::lagoon_runtime::SessionKeys,
          )>,
          ::subxt::BasicError,
        > {
          let entry = QueuedKeys;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn disabled_validators(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::std::vec::Vec<::core::primitive::u32>, ::subxt::BasicError>
        {
          let entry = DisabledValidators;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn next_keys(
          &self,
          _0: &::subxt::sp_core::crypto::AccountId32,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::core::option::Option<runtime_types::lagoon_runtime::SessionKeys>,
          ::subxt::BasicError,
        > {
          let entry = NextKeys(_0);
          self.client.storage().fetch(&entry, hash).await
        }
        pub async fn next_keys_iter(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::subxt::KeyIter<'a, T, NextKeys<'a>>, ::subxt::BasicError>
        {
          self.client.storage().iter(hash).await
        }
        pub async fn key_owner(
          &self,
          _0: &runtime_types::sp_core::crypto::KeyTypeId,
          _1: &[::core::primitive::u8],
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
          ::subxt::BasicError,
        > {
          let entry = KeyOwner(_0, _1);
          self.client.storage().fetch(&entry, hash).await
        }
        pub async fn key_owner_iter(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::subxt::KeyIter<'a, T, KeyOwner<'a>>, ::subxt::BasicError>
        {
          self.client.storage().iter(hash).await
        }
      }
    }
  }
  pub mod grandpa {
    use super::root_mod;
    use super::runtime_types;
    pub mod calls {
      use super::root_mod;
      use super::runtime_types;
      type DispatchError = runtime_types::sp_runtime::DispatchError;
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct ReportEquivocation {
        pub equivocation_proof: ::std::boxed::Box<
          runtime_types::sp_finality_grandpa::EquivocationProof<
            ::subxt::sp_core::H256,
            ::core::primitive::u32,
          >,
        >,
        pub key_owner_proof: runtime_types::sp_session::MembershipProof,
      }
      impl ::subxt::Call for ReportEquivocation {
        const PALLET: &'static str = "Grandpa";
        const FUNCTION: &'static str = "report_equivocation";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct ReportEquivocationUnsigned {
        pub equivocation_proof: ::std::boxed::Box<
          runtime_types::sp_finality_grandpa::EquivocationProof<
            ::subxt::sp_core::H256,
            ::core::primitive::u32,
          >,
        >,
        pub key_owner_proof: runtime_types::sp_session::MembershipProof,
      }
      impl ::subxt::Call for ReportEquivocationUnsigned {
        const PALLET: &'static str = "Grandpa";
        const FUNCTION: &'static str = "report_equivocation_unsigned";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct NoteStalled {
        pub delay: ::core::primitive::u32,
        pub best_finalized_block_number: ::core::primitive::u32,
      }
      impl ::subxt::Call for NoteStalled {
        const PALLET: &'static str = "Grandpa";
        const FUNCTION: &'static str = "note_stalled";
      }
      pub struct TransactionApi<'a, T: ::subxt::Config, X> {
        client: &'a ::subxt::Client<T>,
        marker: ::core::marker::PhantomData<X>,
      }
      impl<'a, T, X> TransactionApi<'a, T, X>
      where
        T: ::subxt::Config,
        X: ::subxt::extrinsic::ExtrinsicParams<T>,
      {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self {
            client,
            marker: ::core::marker::PhantomData,
          }
        }
        pub fn report_equivocation(
          &self,
          equivocation_proof: runtime_types::sp_finality_grandpa::EquivocationProof<
            ::subxt::sp_core::H256,
            ::core::primitive::u32,
          >,
          key_owner_proof: runtime_types::sp_session::MembershipProof,
        ) -> ::subxt::SubmittableExtrinsic<
          'a,
          T,
          X,
          ReportEquivocation,
          DispatchError,
          root_mod::Event,
        > {
          let call = ReportEquivocation {
            equivocation_proof: ::std::boxed::Box::new(equivocation_proof),
            key_owner_proof,
          };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn report_equivocation_unsigned(
          &self,
          equivocation_proof: runtime_types::sp_finality_grandpa::EquivocationProof<
            ::subxt::sp_core::H256,
            ::core::primitive::u32,
          >,
          key_owner_proof: runtime_types::sp_session::MembershipProof,
        ) -> ::subxt::SubmittableExtrinsic<
          'a,
          T,
          X,
          ReportEquivocationUnsigned,
          DispatchError,
          root_mod::Event,
        > {
          let call = ReportEquivocationUnsigned {
            equivocation_proof: ::std::boxed::Box::new(equivocation_proof),
            key_owner_proof,
          };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn note_stalled(
          &self,
          delay: ::core::primitive::u32,
          best_finalized_block_number: ::core::primitive::u32,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, NoteStalled, DispatchError, root_mod::Event>
        {
          let call = NoteStalled {
            delay,
            best_finalized_block_number,
          };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
      }
    }
    pub type Event = runtime_types::pallet_grandpa::pallet::Event;
    pub mod events {
      use super::runtime_types;
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct NewAuthorities {
        pub authority_set: ::std::vec::Vec<(
          runtime_types::sp_finality_grandpa::app::Public,
          ::core::primitive::u64,
        )>,
      }
      impl ::subxt::Event for NewAuthorities {
        const PALLET: &'static str = "Grandpa";
        const EVENT: &'static str = "NewAuthorities";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Paused;
      impl ::subxt::Event for Paused {
        const PALLET: &'static str = "Grandpa";
        const EVENT: &'static str = "Paused";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Resumed;
      impl ::subxt::Event for Resumed {
        const PALLET: &'static str = "Grandpa";
        const EVENT: &'static str = "Resumed";
      }
    }
    pub mod storage {
      use super::runtime_types;
      pub struct State;
      impl ::subxt::StorageEntry for State {
        const PALLET: &'static str = "Grandpa";
        const STORAGE: &'static str = "State";
        type Value = runtime_types::pallet_grandpa::StoredState<::core::primitive::u32>;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct PendingChange;
      impl ::subxt::StorageEntry for PendingChange {
        const PALLET: &'static str = "Grandpa";
        const STORAGE: &'static str = "PendingChange";
        type Value = runtime_types::pallet_grandpa::StoredPendingChange<::core::primitive::u32>;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct NextForced;
      impl ::subxt::StorageEntry for NextForced {
        const PALLET: &'static str = "Grandpa";
        const STORAGE: &'static str = "NextForced";
        type Value = ::core::primitive::u32;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct Stalled;
      impl ::subxt::StorageEntry for Stalled {
        const PALLET: &'static str = "Grandpa";
        const STORAGE: &'static str = "Stalled";
        type Value = (::core::primitive::u32, ::core::primitive::u32);
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct CurrentSetId;
      impl ::subxt::StorageEntry for CurrentSetId {
        const PALLET: &'static str = "Grandpa";
        const STORAGE: &'static str = "CurrentSetId";
        type Value = ::core::primitive::u64;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct SetIdSession<'a>(pub &'a ::core::primitive::u64);
      impl ::subxt::StorageEntry for SetIdSession<'_> {
        const PALLET: &'static str = "Grandpa";
        const STORAGE: &'static str = "SetIdSession";
        type Value = ::core::primitive::u32;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
            &self.0,
            ::subxt::StorageHasher::Twox64Concat,
          )])
        }
      }
      pub struct StorageApi<'a, T: ::subxt::Config> {
        client: &'a ::subxt::Client<T>,
      }
      impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self { client }
        }
        pub async fn state(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          runtime_types::pallet_grandpa::StoredState<::core::primitive::u32>,
          ::subxt::BasicError,
        > {
          let entry = State;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn pending_change(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::core::option::Option<
            runtime_types::pallet_grandpa::StoredPendingChange<::core::primitive::u32>,
          >,
          ::subxt::BasicError,
        > {
          let entry = PendingChange;
          self.client.storage().fetch(&entry, hash).await
        }
        pub async fn next_forced(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::core::option::Option<::core::primitive::u32>,
          ::subxt::BasicError,
        > {
          let entry = NextForced;
          self.client.storage().fetch(&entry, hash).await
        }
        pub async fn stalled(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
          ::subxt::BasicError,
        > {
          let entry = Stalled;
          self.client.storage().fetch(&entry, hash).await
        }
        pub async fn current_set_id(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError> {
          let entry = CurrentSetId;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn set_id_session(
          &self,
          _0: &::core::primitive::u64,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::core::option::Option<::core::primitive::u32>,
          ::subxt::BasicError,
        > {
          let entry = SetIdSession(_0);
          self.client.storage().fetch(&entry, hash).await
        }
        pub async fn set_id_session_iter(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::subxt::KeyIter<'a, T, SetIdSession<'a>>, ::subxt::BasicError>
        {
          self.client.storage().iter(hash).await
        }
      }
    }
    pub mod constants {
      use super::runtime_types;
      pub struct ConstantsApi<'a, T: ::subxt::Config> {
        client: &'a ::subxt::Client<T>,
      }
      impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self { client }
        }
        pub fn max_authorities(
          &self,
        ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
          let pallet = self.client.metadata().pallet("Grandpa")?;
          let constant = pallet.constant("MaxAuthorities")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
      }
    }
  }
  pub mod im_online {
    use super::root_mod;
    use super::runtime_types;
    pub mod calls {
      use super::root_mod;
      use super::runtime_types;
      type DispatchError = runtime_types::sp_runtime::DispatchError;
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Heartbeat {
        pub heartbeat: runtime_types::pallet_im_online::Heartbeat<::core::primitive::u32>,
        pub signature: runtime_types::pallet_im_online::sr25519::app_sr25519::Signature,
      }
      impl ::subxt::Call for Heartbeat {
        const PALLET: &'static str = "ImOnline";
        const FUNCTION: &'static str = "heartbeat";
      }
      pub struct TransactionApi<'a, T: ::subxt::Config, X> {
        client: &'a ::subxt::Client<T>,
        marker: ::core::marker::PhantomData<X>,
      }
      impl<'a, T, X> TransactionApi<'a, T, X>
      where
        T: ::subxt::Config,
        X: ::subxt::extrinsic::ExtrinsicParams<T>,
      {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self {
            client,
            marker: ::core::marker::PhantomData,
          }
        }
        pub fn heartbeat(
          &self,
          heartbeat: runtime_types::pallet_im_online::Heartbeat<::core::primitive::u32>,
          signature: runtime_types::pallet_im_online::sr25519::app_sr25519::Signature,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, Heartbeat, DispatchError, root_mod::Event>
        {
          let call = Heartbeat {
            heartbeat,
            signature,
          };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
      }
    }
    pub type Event = runtime_types::pallet_im_online::pallet::Event;
    pub mod events {
      use super::runtime_types;
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct HeartbeatReceived {
        pub authority_id: runtime_types::pallet_im_online::sr25519::app_sr25519::Public,
      }
      impl ::subxt::Event for HeartbeatReceived {
        const PALLET: &'static str = "ImOnline";
        const EVENT: &'static str = "HeartbeatReceived";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct AllGood;
      impl ::subxt::Event for AllGood {
        const PALLET: &'static str = "ImOnline";
        const EVENT: &'static str = "AllGood";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct SomeOffline {
        pub offline: ::std::vec::Vec<(
          ::subxt::sp_core::crypto::AccountId32,
          runtime_types::pallet_staking::Exposure<
            ::subxt::sp_core::crypto::AccountId32,
            ::core::primitive::u128,
          >,
        )>,
      }
      impl ::subxt::Event for SomeOffline {
        const PALLET: &'static str = "ImOnline";
        const EVENT: &'static str = "SomeOffline";
      }
    }
    pub mod storage {
      use super::runtime_types;
      pub struct HeartbeatAfter;
      impl ::subxt::StorageEntry for HeartbeatAfter {
        const PALLET: &'static str = "ImOnline";
        const STORAGE: &'static str = "HeartbeatAfter";
        type Value = ::core::primitive::u32;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct Keys;
      impl ::subxt::StorageEntry for Keys {
        const PALLET: &'static str = "ImOnline";
        const STORAGE: &'static str = "Keys";
        type Value = runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
          runtime_types::pallet_im_online::sr25519::app_sr25519::Public,
        >;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct ReceivedHeartbeats<'a>(
        pub &'a ::core::primitive::u32,
        pub &'a ::core::primitive::u32,
      );
      impl ::subxt::StorageEntry for ReceivedHeartbeats<'_> {
        const PALLET: &'static str = "ImOnline";
        const STORAGE: &'static str = "ReceivedHeartbeats";
        type Value = runtime_types::frame_support::traits::misc::WrapperOpaque<
          runtime_types::pallet_im_online::BoundedOpaqueNetworkState,
        >;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Map(vec![
            ::subxt::StorageMapKey::new(&self.0, ::subxt::StorageHasher::Twox64Concat),
            ::subxt::StorageMapKey::new(&self.1, ::subxt::StorageHasher::Twox64Concat),
          ])
        }
      }
      pub struct AuthoredBlocks<'a>(
        pub &'a ::core::primitive::u32,
        pub &'a ::subxt::sp_core::crypto::AccountId32,
      );
      impl ::subxt::StorageEntry for AuthoredBlocks<'_> {
        const PALLET: &'static str = "ImOnline";
        const STORAGE: &'static str = "AuthoredBlocks";
        type Value = ::core::primitive::u32;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Map(vec![
            ::subxt::StorageMapKey::new(&self.0, ::subxt::StorageHasher::Twox64Concat),
            ::subxt::StorageMapKey::new(&self.1, ::subxt::StorageHasher::Twox64Concat),
          ])
        }
      }
      pub struct StorageApi<'a, T: ::subxt::Config> {
        client: &'a ::subxt::Client<T>,
      }
      impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self { client }
        }
        pub async fn heartbeat_after(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
          let entry = HeartbeatAfter;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn keys(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
            runtime_types::pallet_im_online::sr25519::app_sr25519::Public,
          >,
          ::subxt::BasicError,
        > {
          let entry = Keys;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn received_heartbeats(
          &self,
          _0: &::core::primitive::u32,
          _1: &::core::primitive::u32,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::core::option::Option<
            runtime_types::frame_support::traits::misc::WrapperOpaque<
              runtime_types::pallet_im_online::BoundedOpaqueNetworkState,
            >,
          >,
          ::subxt::BasicError,
        > {
          let entry = ReceivedHeartbeats(_0, _1);
          self.client.storage().fetch(&entry, hash).await
        }
        pub async fn received_heartbeats_iter(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::subxt::KeyIter<'a, T, ReceivedHeartbeats<'a>>,
          ::subxt::BasicError,
        > {
          self.client.storage().iter(hash).await
        }
        pub async fn authored_blocks(
          &self,
          _0: &::core::primitive::u32,
          _1: &::subxt::sp_core::crypto::AccountId32,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
          let entry = AuthoredBlocks(_0, _1);
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn authored_blocks_iter(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::subxt::KeyIter<'a, T, AuthoredBlocks<'a>>, ::subxt::BasicError>
        {
          self.client.storage().iter(hash).await
        }
      }
    }
    pub mod constants {
      use super::runtime_types;
      pub struct ConstantsApi<'a, T: ::subxt::Config> {
        client: &'a ::subxt::Client<T>,
      }
      impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self { client }
        }
        pub fn unsigned_priority(
          &self,
        ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError> {
          let pallet = self.client.metadata().pallet("ImOnline")?;
          let constant = pallet.constant("UnsignedPriority")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
      }
    }
  }
  pub mod authority_discovery {
    use super::root_mod;
    use super::runtime_types;
  }
  pub mod council {
    use super::root_mod;
    use super::runtime_types;
    pub mod calls {
      use super::root_mod;
      use super::runtime_types;
      type DispatchError = runtime_types::sp_runtime::DispatchError;
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct SetMembers {
        pub new_members: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
        pub prime: ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
        pub old_count: ::core::primitive::u32,
      }
      impl ::subxt::Call for SetMembers {
        const PALLET: &'static str = "Council";
        const FUNCTION: &'static str = "set_members";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Execute {
        pub proposal: ::std::boxed::Box<runtime_types::lagoon_runtime::Call>,
        #[codec(compact)]
        pub length_bound: ::core::primitive::u32,
      }
      impl ::subxt::Call for Execute {
        const PALLET: &'static str = "Council";
        const FUNCTION: &'static str = "execute";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Propose {
        #[codec(compact)]
        pub threshold: ::core::primitive::u32,
        pub proposal: ::std::boxed::Box<runtime_types::lagoon_runtime::Call>,
        #[codec(compact)]
        pub length_bound: ::core::primitive::u32,
      }
      impl ::subxt::Call for Propose {
        const PALLET: &'static str = "Council";
        const FUNCTION: &'static str = "propose";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Vote {
        pub proposal: ::subxt::sp_core::H256,
        #[codec(compact)]
        pub index: ::core::primitive::u32,
        pub approve: ::core::primitive::bool,
      }
      impl ::subxt::Call for Vote {
        const PALLET: &'static str = "Council";
        const FUNCTION: &'static str = "vote";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Close {
        pub proposal_hash: ::subxt::sp_core::H256,
        #[codec(compact)]
        pub index: ::core::primitive::u32,
        #[codec(compact)]
        pub proposal_weight_bound: ::core::primitive::u64,
        #[codec(compact)]
        pub length_bound: ::core::primitive::u32,
      }
      impl ::subxt::Call for Close {
        const PALLET: &'static str = "Council";
        const FUNCTION: &'static str = "close";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct DisapproveProposal {
        pub proposal_hash: ::subxt::sp_core::H256,
      }
      impl ::subxt::Call for DisapproveProposal {
        const PALLET: &'static str = "Council";
        const FUNCTION: &'static str = "disapprove_proposal";
      }
      pub struct TransactionApi<'a, T: ::subxt::Config, X> {
        client: &'a ::subxt::Client<T>,
        marker: ::core::marker::PhantomData<X>,
      }
      impl<'a, T, X> TransactionApi<'a, T, X>
      where
        T: ::subxt::Config,
        X: ::subxt::extrinsic::ExtrinsicParams<T>,
      {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self {
            client,
            marker: ::core::marker::PhantomData,
          }
        }
        pub fn set_members(
          &self,
          new_members: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
          prime: ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
          old_count: ::core::primitive::u32,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, SetMembers, DispatchError, root_mod::Event>
        {
          let call = SetMembers {
            new_members,
            prime,
            old_count,
          };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn execute(
          &self,
          proposal: runtime_types::lagoon_runtime::Call,
          length_bound: ::core::primitive::u32,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, Execute, DispatchError, root_mod::Event>
        {
          let call = Execute {
            proposal: ::std::boxed::Box::new(proposal),
            length_bound,
          };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn propose(
          &self,
          threshold: ::core::primitive::u32,
          proposal: runtime_types::lagoon_runtime::Call,
          length_bound: ::core::primitive::u32,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, Propose, DispatchError, root_mod::Event>
        {
          let call = Propose {
            threshold,
            proposal: ::std::boxed::Box::new(proposal),
            length_bound,
          };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn vote(
          &self,
          proposal: ::subxt::sp_core::H256,
          index: ::core::primitive::u32,
          approve: ::core::primitive::bool,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, Vote, DispatchError, root_mod::Event> {
          let call = Vote {
            proposal,
            index,
            approve,
          };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn close(
          &self,
          proposal_hash: ::subxt::sp_core::H256,
          index: ::core::primitive::u32,
          proposal_weight_bound: ::core::primitive::u64,
          length_bound: ::core::primitive::u32,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, Close, DispatchError, root_mod::Event>
        {
          let call = Close {
            proposal_hash,
            index,
            proposal_weight_bound,
            length_bound,
          };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn disapprove_proposal(
          &self,
          proposal_hash: ::subxt::sp_core::H256,
        ) -> ::subxt::SubmittableExtrinsic<
          'a,
          T,
          X,
          DisapproveProposal,
          DispatchError,
          root_mod::Event,
        > {
          let call = DisapproveProposal { proposal_hash };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
      }
    }
    pub type Event = runtime_types::pallet_collective::pallet::Event;
    pub mod events {
      use super::runtime_types;
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Proposed {
        pub account: ::subxt::sp_core::crypto::AccountId32,
        pub proposal_index: ::core::primitive::u32,
        pub proposal_hash: ::subxt::sp_core::H256,
        pub threshold: ::core::primitive::u32,
      }
      impl ::subxt::Event for Proposed {
        const PALLET: &'static str = "Council";
        const EVENT: &'static str = "Proposed";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Voted {
        pub account: ::subxt::sp_core::crypto::AccountId32,
        pub proposal_hash: ::subxt::sp_core::H256,
        pub voted: ::core::primitive::bool,
        pub yes: ::core::primitive::u32,
        pub no: ::core::primitive::u32,
      }
      impl ::subxt::Event for Voted {
        const PALLET: &'static str = "Council";
        const EVENT: &'static str = "Voted";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Approved {
        pub proposal_hash: ::subxt::sp_core::H256,
      }
      impl ::subxt::Event for Approved {
        const PALLET: &'static str = "Council";
        const EVENT: &'static str = "Approved";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Disapproved {
        pub proposal_hash: ::subxt::sp_core::H256,
      }
      impl ::subxt::Event for Disapproved {
        const PALLET: &'static str = "Council";
        const EVENT: &'static str = "Disapproved";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Executed {
        pub proposal_hash: ::subxt::sp_core::H256,
        pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
      }
      impl ::subxt::Event for Executed {
        const PALLET: &'static str = "Council";
        const EVENT: &'static str = "Executed";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct MemberExecuted {
        pub proposal_hash: ::subxt::sp_core::H256,
        pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
      }
      impl ::subxt::Event for MemberExecuted {
        const PALLET: &'static str = "Council";
        const EVENT: &'static str = "MemberExecuted";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Closed {
        pub proposal_hash: ::subxt::sp_core::H256,
        pub yes: ::core::primitive::u32,
        pub no: ::core::primitive::u32,
      }
      impl ::subxt::Event for Closed {
        const PALLET: &'static str = "Council";
        const EVENT: &'static str = "Closed";
      }
    }
    pub mod storage {
      use super::runtime_types;
      pub struct Proposals;
      impl ::subxt::StorageEntry for Proposals {
        const PALLET: &'static str = "Council";
        const STORAGE: &'static str = "Proposals";
        type Value =
          runtime_types::frame_support::storage::bounded_vec::BoundedVec<::subxt::sp_core::H256>;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct ProposalOf<'a>(pub &'a ::subxt::sp_core::H256);
      impl ::subxt::StorageEntry for ProposalOf<'_> {
        const PALLET: &'static str = "Council";
        const STORAGE: &'static str = "ProposalOf";
        type Value = runtime_types::lagoon_runtime::Call;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
            &self.0,
            ::subxt::StorageHasher::Identity,
          )])
        }
      }
      pub struct Voting<'a>(pub &'a ::subxt::sp_core::H256);
      impl ::subxt::StorageEntry for Voting<'_> {
        const PALLET: &'static str = "Council";
        const STORAGE: &'static str = "Voting";
        type Value = runtime_types::pallet_collective::Votes<
          ::subxt::sp_core::crypto::AccountId32,
          ::core::primitive::u32,
        >;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
            &self.0,
            ::subxt::StorageHasher::Identity,
          )])
        }
      }
      pub struct ProposalCount;
      impl ::subxt::StorageEntry for ProposalCount {
        const PALLET: &'static str = "Council";
        const STORAGE: &'static str = "ProposalCount";
        type Value = ::core::primitive::u32;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct Members;
      impl ::subxt::StorageEntry for Members {
        const PALLET: &'static str = "Council";
        const STORAGE: &'static str = "Members";
        type Value = ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct Prime;
      impl ::subxt::StorageEntry for Prime {
        const PALLET: &'static str = "Council";
        const STORAGE: &'static str = "Prime";
        type Value = ::subxt::sp_core::crypto::AccountId32;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct StorageApi<'a, T: ::subxt::Config> {
        client: &'a ::subxt::Client<T>,
      }
      impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self { client }
        }
        pub async fn proposals(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          runtime_types::frame_support::storage::bounded_vec::BoundedVec<::subxt::sp_core::H256>,
          ::subxt::BasicError,
        > {
          let entry = Proposals;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn proposal_of(
          &self,
          _0: &::subxt::sp_core::H256,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::core::option::Option<runtime_types::lagoon_runtime::Call>,
          ::subxt::BasicError,
        > {
          let entry = ProposalOf(_0);
          self.client.storage().fetch(&entry, hash).await
        }
        pub async fn proposal_of_iter(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::subxt::KeyIter<'a, T, ProposalOf<'a>>, ::subxt::BasicError>
        {
          self.client.storage().iter(hash).await
        }
        pub async fn voting(
          &self,
          _0: &::subxt::sp_core::H256,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::core::option::Option<
            runtime_types::pallet_collective::Votes<
              ::subxt::sp_core::crypto::AccountId32,
              ::core::primitive::u32,
            >,
          >,
          ::subxt::BasicError,
        > {
          let entry = Voting(_0);
          self.client.storage().fetch(&entry, hash).await
        }
        pub async fn voting_iter(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Voting<'a>>, ::subxt::BasicError>
        {
          self.client.storage().iter(hash).await
        }
        pub async fn proposal_count(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
          let entry = ProposalCount;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn members(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
          ::subxt::BasicError,
        > {
          let entry = Members;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn prime(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
          ::subxt::BasicError,
        > {
          let entry = Prime;
          self.client.storage().fetch(&entry, hash).await
        }
      }
    }
  }
  pub mod technical_committee {
    use super::root_mod;
    use super::runtime_types;
    pub mod calls {
      use super::root_mod;
      use super::runtime_types;
      type DispatchError = runtime_types::sp_runtime::DispatchError;
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct SetMembers {
        pub new_members: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
        pub prime: ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
        pub old_count: ::core::primitive::u32,
      }
      impl ::subxt::Call for SetMembers {
        const PALLET: &'static str = "TechnicalCommittee";
        const FUNCTION: &'static str = "set_members";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Execute {
        pub proposal: ::std::boxed::Box<runtime_types::lagoon_runtime::Call>,
        #[codec(compact)]
        pub length_bound: ::core::primitive::u32,
      }
      impl ::subxt::Call for Execute {
        const PALLET: &'static str = "TechnicalCommittee";
        const FUNCTION: &'static str = "execute";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Propose {
        #[codec(compact)]
        pub threshold: ::core::primitive::u32,
        pub proposal: ::std::boxed::Box<runtime_types::lagoon_runtime::Call>,
        #[codec(compact)]
        pub length_bound: ::core::primitive::u32,
      }
      impl ::subxt::Call for Propose {
        const PALLET: &'static str = "TechnicalCommittee";
        const FUNCTION: &'static str = "propose";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Vote {
        pub proposal: ::subxt::sp_core::H256,
        #[codec(compact)]
        pub index: ::core::primitive::u32,
        pub approve: ::core::primitive::bool,
      }
      impl ::subxt::Call for Vote {
        const PALLET: &'static str = "TechnicalCommittee";
        const FUNCTION: &'static str = "vote";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Close {
        pub proposal_hash: ::subxt::sp_core::H256,
        #[codec(compact)]
        pub index: ::core::primitive::u32,
        #[codec(compact)]
        pub proposal_weight_bound: ::core::primitive::u64,
        #[codec(compact)]
        pub length_bound: ::core::primitive::u32,
      }
      impl ::subxt::Call for Close {
        const PALLET: &'static str = "TechnicalCommittee";
        const FUNCTION: &'static str = "close";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct DisapproveProposal {
        pub proposal_hash: ::subxt::sp_core::H256,
      }
      impl ::subxt::Call for DisapproveProposal {
        const PALLET: &'static str = "TechnicalCommittee";
        const FUNCTION: &'static str = "disapprove_proposal";
      }
      pub struct TransactionApi<'a, T: ::subxt::Config, X> {
        client: &'a ::subxt::Client<T>,
        marker: ::core::marker::PhantomData<X>,
      }
      impl<'a, T, X> TransactionApi<'a, T, X>
      where
        T: ::subxt::Config,
        X: ::subxt::extrinsic::ExtrinsicParams<T>,
      {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self {
            client,
            marker: ::core::marker::PhantomData,
          }
        }
        pub fn set_members(
          &self,
          new_members: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
          prime: ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
          old_count: ::core::primitive::u32,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, SetMembers, DispatchError, root_mod::Event>
        {
          let call = SetMembers {
            new_members,
            prime,
            old_count,
          };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn execute(
          &self,
          proposal: runtime_types::lagoon_runtime::Call,
          length_bound: ::core::primitive::u32,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, Execute, DispatchError, root_mod::Event>
        {
          let call = Execute {
            proposal: ::std::boxed::Box::new(proposal),
            length_bound,
          };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn propose(
          &self,
          threshold: ::core::primitive::u32,
          proposal: runtime_types::lagoon_runtime::Call,
          length_bound: ::core::primitive::u32,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, Propose, DispatchError, root_mod::Event>
        {
          let call = Propose {
            threshold,
            proposal: ::std::boxed::Box::new(proposal),
            length_bound,
          };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn vote(
          &self,
          proposal: ::subxt::sp_core::H256,
          index: ::core::primitive::u32,
          approve: ::core::primitive::bool,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, Vote, DispatchError, root_mod::Event> {
          let call = Vote {
            proposal,
            index,
            approve,
          };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn close(
          &self,
          proposal_hash: ::subxt::sp_core::H256,
          index: ::core::primitive::u32,
          proposal_weight_bound: ::core::primitive::u64,
          length_bound: ::core::primitive::u32,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, Close, DispatchError, root_mod::Event>
        {
          let call = Close {
            proposal_hash,
            index,
            proposal_weight_bound,
            length_bound,
          };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn disapprove_proposal(
          &self,
          proposal_hash: ::subxt::sp_core::H256,
        ) -> ::subxt::SubmittableExtrinsic<
          'a,
          T,
          X,
          DisapproveProposal,
          DispatchError,
          root_mod::Event,
        > {
          let call = DisapproveProposal { proposal_hash };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
      }
    }
    pub type Event = runtime_types::pallet_collective::pallet::Event;
    pub mod events {
      use super::runtime_types;
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Proposed {
        pub account: ::subxt::sp_core::crypto::AccountId32,
        pub proposal_index: ::core::primitive::u32,
        pub proposal_hash: ::subxt::sp_core::H256,
        pub threshold: ::core::primitive::u32,
      }
      impl ::subxt::Event for Proposed {
        const PALLET: &'static str = "TechnicalCommittee";
        const EVENT: &'static str = "Proposed";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Voted {
        pub account: ::subxt::sp_core::crypto::AccountId32,
        pub proposal_hash: ::subxt::sp_core::H256,
        pub voted: ::core::primitive::bool,
        pub yes: ::core::primitive::u32,
        pub no: ::core::primitive::u32,
      }
      impl ::subxt::Event for Voted {
        const PALLET: &'static str = "TechnicalCommittee";
        const EVENT: &'static str = "Voted";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Approved {
        pub proposal_hash: ::subxt::sp_core::H256,
      }
      impl ::subxt::Event for Approved {
        const PALLET: &'static str = "TechnicalCommittee";
        const EVENT: &'static str = "Approved";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Disapproved {
        pub proposal_hash: ::subxt::sp_core::H256,
      }
      impl ::subxt::Event for Disapproved {
        const PALLET: &'static str = "TechnicalCommittee";
        const EVENT: &'static str = "Disapproved";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Executed {
        pub proposal_hash: ::subxt::sp_core::H256,
        pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
      }
      impl ::subxt::Event for Executed {
        const PALLET: &'static str = "TechnicalCommittee";
        const EVENT: &'static str = "Executed";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct MemberExecuted {
        pub proposal_hash: ::subxt::sp_core::H256,
        pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
      }
      impl ::subxt::Event for MemberExecuted {
        const PALLET: &'static str = "TechnicalCommittee";
        const EVENT: &'static str = "MemberExecuted";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Closed {
        pub proposal_hash: ::subxt::sp_core::H256,
        pub yes: ::core::primitive::u32,
        pub no: ::core::primitive::u32,
      }
      impl ::subxt::Event for Closed {
        const PALLET: &'static str = "TechnicalCommittee";
        const EVENT: &'static str = "Closed";
      }
    }
    pub mod storage {
      use super::runtime_types;
      pub struct Proposals;
      impl ::subxt::StorageEntry for Proposals {
        const PALLET: &'static str = "TechnicalCommittee";
        const STORAGE: &'static str = "Proposals";
        type Value =
          runtime_types::frame_support::storage::bounded_vec::BoundedVec<::subxt::sp_core::H256>;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct ProposalOf<'a>(pub &'a ::subxt::sp_core::H256);
      impl ::subxt::StorageEntry for ProposalOf<'_> {
        const PALLET: &'static str = "TechnicalCommittee";
        const STORAGE: &'static str = "ProposalOf";
        type Value = runtime_types::lagoon_runtime::Call;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
            &self.0,
            ::subxt::StorageHasher::Identity,
          )])
        }
      }
      pub struct Voting<'a>(pub &'a ::subxt::sp_core::H256);
      impl ::subxt::StorageEntry for Voting<'_> {
        const PALLET: &'static str = "TechnicalCommittee";
        const STORAGE: &'static str = "Voting";
        type Value = runtime_types::pallet_collective::Votes<
          ::subxt::sp_core::crypto::AccountId32,
          ::core::primitive::u32,
        >;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
            &self.0,
            ::subxt::StorageHasher::Identity,
          )])
        }
      }
      pub struct ProposalCount;
      impl ::subxt::StorageEntry for ProposalCount {
        const PALLET: &'static str = "TechnicalCommittee";
        const STORAGE: &'static str = "ProposalCount";
        type Value = ::core::primitive::u32;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct Members;
      impl ::subxt::StorageEntry for Members {
        const PALLET: &'static str = "TechnicalCommittee";
        const STORAGE: &'static str = "Members";
        type Value = ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct Prime;
      impl ::subxt::StorageEntry for Prime {
        const PALLET: &'static str = "TechnicalCommittee";
        const STORAGE: &'static str = "Prime";
        type Value = ::subxt::sp_core::crypto::AccountId32;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct StorageApi<'a, T: ::subxt::Config> {
        client: &'a ::subxt::Client<T>,
      }
      impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self { client }
        }
        pub async fn proposals(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          runtime_types::frame_support::storage::bounded_vec::BoundedVec<::subxt::sp_core::H256>,
          ::subxt::BasicError,
        > {
          let entry = Proposals;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn proposal_of(
          &self,
          _0: &::subxt::sp_core::H256,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::core::option::Option<runtime_types::lagoon_runtime::Call>,
          ::subxt::BasicError,
        > {
          let entry = ProposalOf(_0);
          self.client.storage().fetch(&entry, hash).await
        }
        pub async fn proposal_of_iter(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::subxt::KeyIter<'a, T, ProposalOf<'a>>, ::subxt::BasicError>
        {
          self.client.storage().iter(hash).await
        }
        pub async fn voting(
          &self,
          _0: &::subxt::sp_core::H256,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::core::option::Option<
            runtime_types::pallet_collective::Votes<
              ::subxt::sp_core::crypto::AccountId32,
              ::core::primitive::u32,
            >,
          >,
          ::subxt::BasicError,
        > {
          let entry = Voting(_0);
          self.client.storage().fetch(&entry, hash).await
        }
        pub async fn voting_iter(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Voting<'a>>, ::subxt::BasicError>
        {
          self.client.storage().iter(hash).await
        }
        pub async fn proposal_count(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
          let entry = ProposalCount;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn members(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
          ::subxt::BasicError,
        > {
          let entry = Members;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn prime(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
          ::subxt::BasicError,
        > {
          let entry = Prime;
          self.client.storage().fetch(&entry, hash).await
        }
      }
    }
  }
  pub mod elections {
    use super::root_mod;
    use super::runtime_types;
    pub mod calls {
      use super::root_mod;
      use super::runtime_types;
      type DispatchError = runtime_types::sp_runtime::DispatchError;
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Vote {
        pub votes: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
        #[codec(compact)]
        pub value: ::core::primitive::u128,
      }
      impl ::subxt::Call for Vote {
        const PALLET: &'static str = "Elections";
        const FUNCTION: &'static str = "vote";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct RemoveVoter;
      impl ::subxt::Call for RemoveVoter {
        const PALLET: &'static str = "Elections";
        const FUNCTION: &'static str = "remove_voter";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct SubmitCandidacy {
        #[codec(compact)]
        pub candidate_count: ::core::primitive::u32,
      }
      impl ::subxt::Call for SubmitCandidacy {
        const PALLET: &'static str = "Elections";
        const FUNCTION: &'static str = "submit_candidacy";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct RenounceCandidacy {
        pub renouncing: runtime_types::pallet_elections_phragmen::Renouncing,
      }
      impl ::subxt::Call for RenounceCandidacy {
        const PALLET: &'static str = "Elections";
        const FUNCTION: &'static str = "renounce_candidacy";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct RemoveMember {
        pub who: ::subxt::sp_runtime::MultiAddress<
          ::subxt::sp_core::crypto::AccountId32,
          ::core::primitive::u32,
        >,
        pub has_replacement: ::core::primitive::bool,
      }
      impl ::subxt::Call for RemoveMember {
        const PALLET: &'static str = "Elections";
        const FUNCTION: &'static str = "remove_member";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct CleanDefunctVoters {
        pub num_voters: ::core::primitive::u32,
        pub num_defunct: ::core::primitive::u32,
      }
      impl ::subxt::Call for CleanDefunctVoters {
        const PALLET: &'static str = "Elections";
        const FUNCTION: &'static str = "clean_defunct_voters";
      }
      pub struct TransactionApi<'a, T: ::subxt::Config, X> {
        client: &'a ::subxt::Client<T>,
        marker: ::core::marker::PhantomData<X>,
      }
      impl<'a, T, X> TransactionApi<'a, T, X>
      where
        T: ::subxt::Config,
        X: ::subxt::extrinsic::ExtrinsicParams<T>,
      {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self {
            client,
            marker: ::core::marker::PhantomData,
          }
        }
        pub fn vote(
          &self,
          votes: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
          value: ::core::primitive::u128,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, Vote, DispatchError, root_mod::Event> {
          let call = Vote { votes, value };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn remove_voter(
          &self,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, RemoveVoter, DispatchError, root_mod::Event>
        {
          let call = RemoveVoter {};
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn submit_candidacy(
          &self,
          candidate_count: ::core::primitive::u32,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, SubmitCandidacy, DispatchError, root_mod::Event>
        {
          let call = SubmitCandidacy { candidate_count };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn renounce_candidacy(
          &self,
          renouncing: runtime_types::pallet_elections_phragmen::Renouncing,
        ) -> ::subxt::SubmittableExtrinsic<
          'a,
          T,
          X,
          RenounceCandidacy,
          DispatchError,
          root_mod::Event,
        > {
          let call = RenounceCandidacy { renouncing };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn remove_member(
          &self,
          who: ::subxt::sp_runtime::MultiAddress<
            ::subxt::sp_core::crypto::AccountId32,
            ::core::primitive::u32,
          >,
          has_replacement: ::core::primitive::bool,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, RemoveMember, DispatchError, root_mod::Event>
        {
          let call = RemoveMember {
            who,
            has_replacement,
          };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn clean_defunct_voters(
          &self,
          num_voters: ::core::primitive::u32,
          num_defunct: ::core::primitive::u32,
        ) -> ::subxt::SubmittableExtrinsic<
          'a,
          T,
          X,
          CleanDefunctVoters,
          DispatchError,
          root_mod::Event,
        > {
          let call = CleanDefunctVoters {
            num_voters,
            num_defunct,
          };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
      }
    }
    pub type Event = runtime_types::pallet_elections_phragmen::pallet::Event;
    pub mod events {
      use super::runtime_types;
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct NewTerm {
        pub new_members: ::std::vec::Vec<(
          ::subxt::sp_core::crypto::AccountId32,
          ::core::primitive::u128,
        )>,
      }
      impl ::subxt::Event for NewTerm {
        const PALLET: &'static str = "Elections";
        const EVENT: &'static str = "NewTerm";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct EmptyTerm;
      impl ::subxt::Event for EmptyTerm {
        const PALLET: &'static str = "Elections";
        const EVENT: &'static str = "EmptyTerm";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct ElectionError;
      impl ::subxt::Event for ElectionError {
        const PALLET: &'static str = "Elections";
        const EVENT: &'static str = "ElectionError";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct MemberKicked {
        pub member: ::subxt::sp_core::crypto::AccountId32,
      }
      impl ::subxt::Event for MemberKicked {
        const PALLET: &'static str = "Elections";
        const EVENT: &'static str = "MemberKicked";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Renounced {
        pub candidate: ::subxt::sp_core::crypto::AccountId32,
      }
      impl ::subxt::Event for Renounced {
        const PALLET: &'static str = "Elections";
        const EVENT: &'static str = "Renounced";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct CandidateSlashed {
        pub candidate: ::subxt::sp_core::crypto::AccountId32,
        pub amount: ::core::primitive::u128,
      }
      impl ::subxt::Event for CandidateSlashed {
        const PALLET: &'static str = "Elections";
        const EVENT: &'static str = "CandidateSlashed";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct SeatHolderSlashed {
        pub seat_holder: ::subxt::sp_core::crypto::AccountId32,
        pub amount: ::core::primitive::u128,
      }
      impl ::subxt::Event for SeatHolderSlashed {
        const PALLET: &'static str = "Elections";
        const EVENT: &'static str = "SeatHolderSlashed";
      }
    }
    pub mod storage {
      use super::runtime_types;
      pub struct Members;
      impl ::subxt::StorageEntry for Members {
        const PALLET: &'static str = "Elections";
        const STORAGE: &'static str = "Members";
        type Value = ::std::vec::Vec<
          runtime_types::pallet_elections_phragmen::SeatHolder<
            ::subxt::sp_core::crypto::AccountId32,
            ::core::primitive::u128,
          >,
        >;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct RunnersUp;
      impl ::subxt::StorageEntry for RunnersUp {
        const PALLET: &'static str = "Elections";
        const STORAGE: &'static str = "RunnersUp";
        type Value = ::std::vec::Vec<
          runtime_types::pallet_elections_phragmen::SeatHolder<
            ::subxt::sp_core::crypto::AccountId32,
            ::core::primitive::u128,
          >,
        >;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct Candidates;
      impl ::subxt::StorageEntry for Candidates {
        const PALLET: &'static str = "Elections";
        const STORAGE: &'static str = "Candidates";
        type Value = ::std::vec::Vec<(
          ::subxt::sp_core::crypto::AccountId32,
          ::core::primitive::u128,
        )>;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct ElectionRounds;
      impl ::subxt::StorageEntry for ElectionRounds {
        const PALLET: &'static str = "Elections";
        const STORAGE: &'static str = "ElectionRounds";
        type Value = ::core::primitive::u32;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct Voting<'a>(pub &'a ::subxt::sp_core::crypto::AccountId32);
      impl ::subxt::StorageEntry for Voting<'_> {
        const PALLET: &'static str = "Elections";
        const STORAGE: &'static str = "Voting";
        type Value = runtime_types::pallet_elections_phragmen::Voter<
          ::subxt::sp_core::crypto::AccountId32,
          ::core::primitive::u128,
        >;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
            &self.0,
            ::subxt::StorageHasher::Twox64Concat,
          )])
        }
      }
      pub struct StorageApi<'a, T: ::subxt::Config> {
        client: &'a ::subxt::Client<T>,
      }
      impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self { client }
        }
        pub async fn members(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::std::vec::Vec<
            runtime_types::pallet_elections_phragmen::SeatHolder<
              ::subxt::sp_core::crypto::AccountId32,
              ::core::primitive::u128,
            >,
          >,
          ::subxt::BasicError,
        > {
          let entry = Members;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn runners_up(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::std::vec::Vec<
            runtime_types::pallet_elections_phragmen::SeatHolder<
              ::subxt::sp_core::crypto::AccountId32,
              ::core::primitive::u128,
            >,
          >,
          ::subxt::BasicError,
        > {
          let entry = RunnersUp;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn candidates(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::std::vec::Vec<(
            ::subxt::sp_core::crypto::AccountId32,
            ::core::primitive::u128,
          )>,
          ::subxt::BasicError,
        > {
          let entry = Candidates;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn election_rounds(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
          let entry = ElectionRounds;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn voting(
          &self,
          _0: &::subxt::sp_core::crypto::AccountId32,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          runtime_types::pallet_elections_phragmen::Voter<
            ::subxt::sp_core::crypto::AccountId32,
            ::core::primitive::u128,
          >,
          ::subxt::BasicError,
        > {
          let entry = Voting(_0);
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn voting_iter(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Voting<'a>>, ::subxt::BasicError>
        {
          self.client.storage().iter(hash).await
        }
      }
    }
    pub mod constants {
      use super::runtime_types;
      pub struct ConstantsApi<'a, T: ::subxt::Config> {
        client: &'a ::subxt::Client<T>,
      }
      impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self { client }
        }
        pub fn pallet_id(
          &self,
        ) -> ::core::result::Result<[::core::primitive::u8; 8usize], ::subxt::BasicError> {
          let pallet = self.client.metadata().pallet("Elections")?;
          let constant = pallet.constant("PalletId")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn candidacy_bond(
          &self,
        ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError> {
          let pallet = self.client.metadata().pallet("Elections")?;
          let constant = pallet.constant("CandidacyBond")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn voting_bond_base(
          &self,
        ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError> {
          let pallet = self.client.metadata().pallet("Elections")?;
          let constant = pallet.constant("VotingBondBase")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn voting_bond_factor(
          &self,
        ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError> {
          let pallet = self.client.metadata().pallet("Elections")?;
          let constant = pallet.constant("VotingBondFactor")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn desired_members(
          &self,
        ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
          let pallet = self.client.metadata().pallet("Elections")?;
          let constant = pallet.constant("DesiredMembers")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn desired_runners_up(
          &self,
        ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
          let pallet = self.client.metadata().pallet("Elections")?;
          let constant = pallet.constant("DesiredRunnersUp")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn term_duration(
          &self,
        ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
          let pallet = self.client.metadata().pallet("Elections")?;
          let constant = pallet.constant("TermDuration")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
      }
    }
  }
  pub mod technical_membership {
    use super::root_mod;
    use super::runtime_types;
    pub mod calls {
      use super::root_mod;
      use super::runtime_types;
      type DispatchError = runtime_types::sp_runtime::DispatchError;
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct AddMember {
        pub who: ::subxt::sp_core::crypto::AccountId32,
      }
      impl ::subxt::Call for AddMember {
        const PALLET: &'static str = "TechnicalMembership";
        const FUNCTION: &'static str = "add_member";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct RemoveMember {
        pub who: ::subxt::sp_core::crypto::AccountId32,
      }
      impl ::subxt::Call for RemoveMember {
        const PALLET: &'static str = "TechnicalMembership";
        const FUNCTION: &'static str = "remove_member";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct SwapMember {
        pub remove: ::subxt::sp_core::crypto::AccountId32,
        pub add: ::subxt::sp_core::crypto::AccountId32,
      }
      impl ::subxt::Call for SwapMember {
        const PALLET: &'static str = "TechnicalMembership";
        const FUNCTION: &'static str = "swap_member";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct ResetMembers {
        pub members: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
      }
      impl ::subxt::Call for ResetMembers {
        const PALLET: &'static str = "TechnicalMembership";
        const FUNCTION: &'static str = "reset_members";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct ChangeKey {
        pub new: ::subxt::sp_core::crypto::AccountId32,
      }
      impl ::subxt::Call for ChangeKey {
        const PALLET: &'static str = "TechnicalMembership";
        const FUNCTION: &'static str = "change_key";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct SetPrime {
        pub who: ::subxt::sp_core::crypto::AccountId32,
      }
      impl ::subxt::Call for SetPrime {
        const PALLET: &'static str = "TechnicalMembership";
        const FUNCTION: &'static str = "set_prime";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct ClearPrime;
      impl ::subxt::Call for ClearPrime {
        const PALLET: &'static str = "TechnicalMembership";
        const FUNCTION: &'static str = "clear_prime";
      }
      pub struct TransactionApi<'a, T: ::subxt::Config, X> {
        client: &'a ::subxt::Client<T>,
        marker: ::core::marker::PhantomData<X>,
      }
      impl<'a, T, X> TransactionApi<'a, T, X>
      where
        T: ::subxt::Config,
        X: ::subxt::extrinsic::ExtrinsicParams<T>,
      {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self {
            client,
            marker: ::core::marker::PhantomData,
          }
        }
        pub fn add_member(
          &self,
          who: ::subxt::sp_core::crypto::AccountId32,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, AddMember, DispatchError, root_mod::Event>
        {
          let call = AddMember { who };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn remove_member(
          &self,
          who: ::subxt::sp_core::crypto::AccountId32,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, RemoveMember, DispatchError, root_mod::Event>
        {
          let call = RemoveMember { who };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn swap_member(
          &self,
          remove: ::subxt::sp_core::crypto::AccountId32,
          add: ::subxt::sp_core::crypto::AccountId32,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, SwapMember, DispatchError, root_mod::Event>
        {
          let call = SwapMember { remove, add };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn reset_members(
          &self,
          members: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, ResetMembers, DispatchError, root_mod::Event>
        {
          let call = ResetMembers { members };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn change_key(
          &self,
          new: ::subxt::sp_core::crypto::AccountId32,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, ChangeKey, DispatchError, root_mod::Event>
        {
          let call = ChangeKey { new };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn set_prime(
          &self,
          who: ::subxt::sp_core::crypto::AccountId32,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, SetPrime, DispatchError, root_mod::Event>
        {
          let call = SetPrime { who };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn clear_prime(
          &self,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, ClearPrime, DispatchError, root_mod::Event>
        {
          let call = ClearPrime {};
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
      }
    }
    pub type Event = runtime_types::pallet_membership::pallet::Event;
    pub mod events {
      use super::runtime_types;
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct MemberAdded;
      impl ::subxt::Event for MemberAdded {
        const PALLET: &'static str = "TechnicalMembership";
        const EVENT: &'static str = "MemberAdded";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct MemberRemoved;
      impl ::subxt::Event for MemberRemoved {
        const PALLET: &'static str = "TechnicalMembership";
        const EVENT: &'static str = "MemberRemoved";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct MembersSwapped;
      impl ::subxt::Event for MembersSwapped {
        const PALLET: &'static str = "TechnicalMembership";
        const EVENT: &'static str = "MembersSwapped";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct MembersReset;
      impl ::subxt::Event for MembersReset {
        const PALLET: &'static str = "TechnicalMembership";
        const EVENT: &'static str = "MembersReset";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct KeyChanged;
      impl ::subxt::Event for KeyChanged {
        const PALLET: &'static str = "TechnicalMembership";
        const EVENT: &'static str = "KeyChanged";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Dummy;
      impl ::subxt::Event for Dummy {
        const PALLET: &'static str = "TechnicalMembership";
        const EVENT: &'static str = "Dummy";
      }
    }
    pub mod storage {
      use super::runtime_types;
      pub struct Members;
      impl ::subxt::StorageEntry for Members {
        const PALLET: &'static str = "TechnicalMembership";
        const STORAGE: &'static str = "Members";
        type Value = ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct Prime;
      impl ::subxt::StorageEntry for Prime {
        const PALLET: &'static str = "TechnicalMembership";
        const STORAGE: &'static str = "Prime";
        type Value = ::subxt::sp_core::crypto::AccountId32;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct StorageApi<'a, T: ::subxt::Config> {
        client: &'a ::subxt::Client<T>,
      }
      impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self { client }
        }
        pub async fn members(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
          ::subxt::BasicError,
        > {
          let entry = Members;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn prime(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
          ::subxt::BasicError,
        > {
          let entry = Prime;
          self.client.storage().fetch(&entry, hash).await
        }
      }
    }
  }
  pub mod treasury {
    use super::root_mod;
    use super::runtime_types;
    pub mod calls {
      use super::root_mod;
      use super::runtime_types;
      type DispatchError = runtime_types::sp_runtime::DispatchError;
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct ProposeSpend {
        #[codec(compact)]
        pub value: ::core::primitive::u128,
        pub beneficiary: ::subxt::sp_runtime::MultiAddress<
          ::subxt::sp_core::crypto::AccountId32,
          ::core::primitive::u32,
        >,
      }
      impl ::subxt::Call for ProposeSpend {
        const PALLET: &'static str = "Treasury";
        const FUNCTION: &'static str = "propose_spend";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct RejectProposal {
        #[codec(compact)]
        pub proposal_id: ::core::primitive::u32,
      }
      impl ::subxt::Call for RejectProposal {
        const PALLET: &'static str = "Treasury";
        const FUNCTION: &'static str = "reject_proposal";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct ApproveProposal {
        #[codec(compact)]
        pub proposal_id: ::core::primitive::u32,
      }
      impl ::subxt::Call for ApproveProposal {
        const PALLET: &'static str = "Treasury";
        const FUNCTION: &'static str = "approve_proposal";
      }
      pub struct TransactionApi<'a, T: ::subxt::Config, X> {
        client: &'a ::subxt::Client<T>,
        marker: ::core::marker::PhantomData<X>,
      }
      impl<'a, T, X> TransactionApi<'a, T, X>
      where
        T: ::subxt::Config,
        X: ::subxt::extrinsic::ExtrinsicParams<T>,
      {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self {
            client,
            marker: ::core::marker::PhantomData,
          }
        }
        pub fn propose_spend(
          &self,
          value: ::core::primitive::u128,
          beneficiary: ::subxt::sp_runtime::MultiAddress<
            ::subxt::sp_core::crypto::AccountId32,
            ::core::primitive::u32,
          >,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, ProposeSpend, DispatchError, root_mod::Event>
        {
          let call = ProposeSpend { value, beneficiary };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn reject_proposal(
          &self,
          proposal_id: ::core::primitive::u32,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, RejectProposal, DispatchError, root_mod::Event>
        {
          let call = RejectProposal { proposal_id };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn approve_proposal(
          &self,
          proposal_id: ::core::primitive::u32,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, ApproveProposal, DispatchError, root_mod::Event>
        {
          let call = ApproveProposal { proposal_id };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
      }
    }
    pub type Event = runtime_types::pallet_treasury::pallet::Event;
    pub mod events {
      use super::runtime_types;
      #[derive(
        :: subxt :: codec :: Encode,
        :: subxt :: codec :: Decode,
        Debug,
        :: subxt :: codec :: CompactAs,
      )]
      pub struct Proposed {
        pub proposal_index: ::core::primitive::u32,
      }
      impl ::subxt::Event for Proposed {
        const PALLET: &'static str = "Treasury";
        const EVENT: &'static str = "Proposed";
      }
      #[derive(
        :: subxt :: codec :: Encode,
        :: subxt :: codec :: Decode,
        Debug,
        :: subxt :: codec :: CompactAs,
      )]
      pub struct Spending {
        pub budget_remaining: ::core::primitive::u128,
      }
      impl ::subxt::Event for Spending {
        const PALLET: &'static str = "Treasury";
        const EVENT: &'static str = "Spending";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Awarded {
        pub proposal_index: ::core::primitive::u32,
        pub award: ::core::primitive::u128,
        pub account: ::subxt::sp_core::crypto::AccountId32,
      }
      impl ::subxt::Event for Awarded {
        const PALLET: &'static str = "Treasury";
        const EVENT: &'static str = "Awarded";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Rejected {
        pub proposal_index: ::core::primitive::u32,
        pub slashed: ::core::primitive::u128,
      }
      impl ::subxt::Event for Rejected {
        const PALLET: &'static str = "Treasury";
        const EVENT: &'static str = "Rejected";
      }
      #[derive(
        :: subxt :: codec :: Encode,
        :: subxt :: codec :: Decode,
        Debug,
        :: subxt :: codec :: CompactAs,
      )]
      pub struct Burnt {
        pub burnt_funds: ::core::primitive::u128,
      }
      impl ::subxt::Event for Burnt {
        const PALLET: &'static str = "Treasury";
        const EVENT: &'static str = "Burnt";
      }
      #[derive(
        :: subxt :: codec :: Encode,
        :: subxt :: codec :: Decode,
        Debug,
        :: subxt :: codec :: CompactAs,
      )]
      pub struct Rollover {
        pub rollover_balance: ::core::primitive::u128,
      }
      impl ::subxt::Event for Rollover {
        const PALLET: &'static str = "Treasury";
        const EVENT: &'static str = "Rollover";
      }
      #[derive(
        :: subxt :: codec :: Encode,
        :: subxt :: codec :: Decode,
        Debug,
        :: subxt :: codec :: CompactAs,
      )]
      pub struct Deposit {
        pub value: ::core::primitive::u128,
      }
      impl ::subxt::Event for Deposit {
        const PALLET: &'static str = "Treasury";
        const EVENT: &'static str = "Deposit";
      }
    }
    pub mod storage {
      use super::runtime_types;
      pub struct ProposalCount;
      impl ::subxt::StorageEntry for ProposalCount {
        const PALLET: &'static str = "Treasury";
        const STORAGE: &'static str = "ProposalCount";
        type Value = ::core::primitive::u32;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct Proposals<'a>(pub &'a ::core::primitive::u32);
      impl ::subxt::StorageEntry for Proposals<'_> {
        const PALLET: &'static str = "Treasury";
        const STORAGE: &'static str = "Proposals";
        type Value = runtime_types::pallet_treasury::Proposal<
          ::subxt::sp_core::crypto::AccountId32,
          ::core::primitive::u128,
        >;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
            &self.0,
            ::subxt::StorageHasher::Twox64Concat,
          )])
        }
      }
      pub struct Approvals;
      impl ::subxt::StorageEntry for Approvals {
        const PALLET: &'static str = "Treasury";
        const STORAGE: &'static str = "Approvals";
        type Value =
          runtime_types::frame_support::storage::bounded_vec::BoundedVec<::core::primitive::u32>;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct StorageApi<'a, T: ::subxt::Config> {
        client: &'a ::subxt::Client<T>,
      }
      impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self { client }
        }
        pub async fn proposal_count(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
          let entry = ProposalCount;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn proposals(
          &self,
          _0: &::core::primitive::u32,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::core::option::Option<
            runtime_types::pallet_treasury::Proposal<
              ::subxt::sp_core::crypto::AccountId32,
              ::core::primitive::u128,
            >,
          >,
          ::subxt::BasicError,
        > {
          let entry = Proposals(_0);
          self.client.storage().fetch(&entry, hash).await
        }
        pub async fn proposals_iter(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Proposals<'a>>, ::subxt::BasicError>
        {
          self.client.storage().iter(hash).await
        }
        pub async fn approvals(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          runtime_types::frame_support::storage::bounded_vec::BoundedVec<::core::primitive::u32>,
          ::subxt::BasicError,
        > {
          let entry = Approvals;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
      }
    }
    pub mod constants {
      use super::runtime_types;
      pub struct ConstantsApi<'a, T: ::subxt::Config> {
        client: &'a ::subxt::Client<T>,
      }
      impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self { client }
        }
        pub fn proposal_bond(
          &self,
        ) -> ::core::result::Result<
          runtime_types::sp_arithmetic::per_things::Permill,
          ::subxt::BasicError,
        > {
          let pallet = self.client.metadata().pallet("Treasury")?;
          let constant = pallet.constant("ProposalBond")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn proposal_bond_minimum(
          &self,
        ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError> {
          let pallet = self.client.metadata().pallet("Treasury")?;
          let constant = pallet.constant("ProposalBondMinimum")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn proposal_bond_maximum(
          &self,
        ) -> ::core::result::Result<
          ::core::option::Option<::core::primitive::u128>,
          ::subxt::BasicError,
        > {
          let pallet = self.client.metadata().pallet("Treasury")?;
          let constant = pallet.constant("ProposalBondMaximum")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn spend_period(
          &self,
        ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
          let pallet = self.client.metadata().pallet("Treasury")?;
          let constant = pallet.constant("SpendPeriod")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn burn(
          &self,
        ) -> ::core::result::Result<
          runtime_types::sp_arithmetic::per_things::Permill,
          ::subxt::BasicError,
        > {
          let pallet = self.client.metadata().pallet("Treasury")?;
          let constant = pallet.constant("Burn")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn pallet_id(
          &self,
        ) -> ::core::result::Result<runtime_types::frame_support::PalletId, ::subxt::BasicError>
        {
          let pallet = self.client.metadata().pallet("Treasury")?;
          let constant = pallet.constant("PalletId")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn max_approvals(
          &self,
        ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
          let pallet = self.client.metadata().pallet("Treasury")?;
          let constant = pallet.constant("MaxApprovals")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
      }
    }
  }
  pub mod utility {
    use super::root_mod;
    use super::runtime_types;
    pub mod calls {
      use super::root_mod;
      use super::runtime_types;
      type DispatchError = runtime_types::sp_runtime::DispatchError;
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Batch {
        pub calls: ::std::vec::Vec<runtime_types::lagoon_runtime::Call>,
      }
      impl ::subxt::Call for Batch {
        const PALLET: &'static str = "Utility";
        const FUNCTION: &'static str = "batch";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct AsDerivative {
        pub index: ::core::primitive::u16,
        pub call: ::std::boxed::Box<runtime_types::lagoon_runtime::Call>,
      }
      impl ::subxt::Call for AsDerivative {
        const PALLET: &'static str = "Utility";
        const FUNCTION: &'static str = "as_derivative";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct BatchAll {
        pub calls: ::std::vec::Vec<runtime_types::lagoon_runtime::Call>,
      }
      impl ::subxt::Call for BatchAll {
        const PALLET: &'static str = "Utility";
        const FUNCTION: &'static str = "batch_all";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct DispatchAs {
        pub as_origin: ::std::boxed::Box<runtime_types::lagoon_runtime::OriginCaller>,
        pub call: ::std::boxed::Box<runtime_types::lagoon_runtime::Call>,
      }
      impl ::subxt::Call for DispatchAs {
        const PALLET: &'static str = "Utility";
        const FUNCTION: &'static str = "dispatch_as";
      }
      pub struct TransactionApi<'a, T: ::subxt::Config, X> {
        client: &'a ::subxt::Client<T>,
        marker: ::core::marker::PhantomData<X>,
      }
      impl<'a, T, X> TransactionApi<'a, T, X>
      where
        T: ::subxt::Config,
        X: ::subxt::extrinsic::ExtrinsicParams<T>,
      {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self {
            client,
            marker: ::core::marker::PhantomData,
          }
        }
        pub fn batch(
          &self,
          calls: ::std::vec::Vec<runtime_types::lagoon_runtime::Call>,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, Batch, DispatchError, root_mod::Event>
        {
          let call = Batch { calls };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn as_derivative(
          &self,
          index: ::core::primitive::u16,
          call: runtime_types::lagoon_runtime::Call,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, AsDerivative, DispatchError, root_mod::Event>
        {
          let call = AsDerivative {
            index,
            call: ::std::boxed::Box::new(call),
          };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn batch_all(
          &self,
          calls: ::std::vec::Vec<runtime_types::lagoon_runtime::Call>,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, BatchAll, DispatchError, root_mod::Event>
        {
          let call = BatchAll { calls };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn dispatch_as(
          &self,
          as_origin: runtime_types::lagoon_runtime::OriginCaller,
          call: runtime_types::lagoon_runtime::Call,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, DispatchAs, DispatchError, root_mod::Event>
        {
          let call = DispatchAs {
            as_origin: ::std::boxed::Box::new(as_origin),
            call: ::std::boxed::Box::new(call),
          };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
      }
    }
    pub type Event = runtime_types::pallet_utility::pallet::Event;
    pub mod events {
      use super::runtime_types;
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct BatchInterrupted {
        pub index: ::core::primitive::u32,
        pub error: runtime_types::sp_runtime::DispatchError,
      }
      impl ::subxt::Event for BatchInterrupted {
        const PALLET: &'static str = "Utility";
        const EVENT: &'static str = "BatchInterrupted";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct BatchCompleted;
      impl ::subxt::Event for BatchCompleted {
        const PALLET: &'static str = "Utility";
        const EVENT: &'static str = "BatchCompleted";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct ItemCompleted;
      impl ::subxt::Event for ItemCompleted {
        const PALLET: &'static str = "Utility";
        const EVENT: &'static str = "ItemCompleted";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct DispatchedAs {
        pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
      }
      impl ::subxt::Event for DispatchedAs {
        const PALLET: &'static str = "Utility";
        const EVENT: &'static str = "DispatchedAs";
      }
    }
    pub mod constants {
      use super::runtime_types;
      pub struct ConstantsApi<'a, T: ::subxt::Config> {
        client: &'a ::subxt::Client<T>,
      }
      impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self { client }
        }
        pub fn batched_calls_limit(
          &self,
        ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
          let pallet = self.client.metadata().pallet("Utility")?;
          let constant = pallet.constant("batched_calls_limit")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
      }
    }
  }
  pub mod identity {
    use super::root_mod;
    use super::runtime_types;
    pub mod calls {
      use super::root_mod;
      use super::runtime_types;
      type DispatchError = runtime_types::sp_runtime::DispatchError;
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct AddRegistrar {
        pub account: ::subxt::sp_core::crypto::AccountId32,
      }
      impl ::subxt::Call for AddRegistrar {
        const PALLET: &'static str = "Identity";
        const FUNCTION: &'static str = "add_registrar";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct SetIdentity {
        pub info: ::std::boxed::Box<runtime_types::pallet_identity::types::IdentityInfo>,
      }
      impl ::subxt::Call for SetIdentity {
        const PALLET: &'static str = "Identity";
        const FUNCTION: &'static str = "set_identity";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct SetSubs {
        pub subs: ::std::vec::Vec<(
          ::subxt::sp_core::crypto::AccountId32,
          runtime_types::pallet_identity::types::Data,
        )>,
      }
      impl ::subxt::Call for SetSubs {
        const PALLET: &'static str = "Identity";
        const FUNCTION: &'static str = "set_subs";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct ClearIdentity;
      impl ::subxt::Call for ClearIdentity {
        const PALLET: &'static str = "Identity";
        const FUNCTION: &'static str = "clear_identity";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct RequestJudgement {
        #[codec(compact)]
        pub reg_index: ::core::primitive::u32,
        #[codec(compact)]
        pub max_fee: ::core::primitive::u128,
      }
      impl ::subxt::Call for RequestJudgement {
        const PALLET: &'static str = "Identity";
        const FUNCTION: &'static str = "request_judgement";
      }
      #[derive(
        :: subxt :: codec :: Encode,
        :: subxt :: codec :: Decode,
        Debug,
        :: subxt :: codec :: CompactAs,
      )]
      pub struct CancelRequest {
        pub reg_index: ::core::primitive::u32,
      }
      impl ::subxt::Call for CancelRequest {
        const PALLET: &'static str = "Identity";
        const FUNCTION: &'static str = "cancel_request";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct SetFee {
        #[codec(compact)]
        pub index: ::core::primitive::u32,
        #[codec(compact)]
        pub fee: ::core::primitive::u128,
      }
      impl ::subxt::Call for SetFee {
        const PALLET: &'static str = "Identity";
        const FUNCTION: &'static str = "set_fee";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct SetAccountId {
        #[codec(compact)]
        pub index: ::core::primitive::u32,
        pub new: ::subxt::sp_core::crypto::AccountId32,
      }
      impl ::subxt::Call for SetAccountId {
        const PALLET: &'static str = "Identity";
        const FUNCTION: &'static str = "set_account_id";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct SetFields {
        #[codec(compact)]
        pub index: ::core::primitive::u32,
        pub fields: runtime_types::pallet_identity::types::BitFlags<
          runtime_types::pallet_identity::types::IdentityField,
        >,
      }
      impl ::subxt::Call for SetFields {
        const PALLET: &'static str = "Identity";
        const FUNCTION: &'static str = "set_fields";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct ProvideJudgement {
        #[codec(compact)]
        pub reg_index: ::core::primitive::u32,
        pub target: ::subxt::sp_runtime::MultiAddress<
          ::subxt::sp_core::crypto::AccountId32,
          ::core::primitive::u32,
        >,
        pub judgement: runtime_types::pallet_identity::types::Judgement<::core::primitive::u128>,
      }
      impl ::subxt::Call for ProvideJudgement {
        const PALLET: &'static str = "Identity";
        const FUNCTION: &'static str = "provide_judgement";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct KillIdentity {
        pub target: ::subxt::sp_runtime::MultiAddress<
          ::subxt::sp_core::crypto::AccountId32,
          ::core::primitive::u32,
        >,
      }
      impl ::subxt::Call for KillIdentity {
        const PALLET: &'static str = "Identity";
        const FUNCTION: &'static str = "kill_identity";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct AddSub {
        pub sub: ::subxt::sp_runtime::MultiAddress<
          ::subxt::sp_core::crypto::AccountId32,
          ::core::primitive::u32,
        >,
        pub data: runtime_types::pallet_identity::types::Data,
      }
      impl ::subxt::Call for AddSub {
        const PALLET: &'static str = "Identity";
        const FUNCTION: &'static str = "add_sub";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct RenameSub {
        pub sub: ::subxt::sp_runtime::MultiAddress<
          ::subxt::sp_core::crypto::AccountId32,
          ::core::primitive::u32,
        >,
        pub data: runtime_types::pallet_identity::types::Data,
      }
      impl ::subxt::Call for RenameSub {
        const PALLET: &'static str = "Identity";
        const FUNCTION: &'static str = "rename_sub";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct RemoveSub {
        pub sub: ::subxt::sp_runtime::MultiAddress<
          ::subxt::sp_core::crypto::AccountId32,
          ::core::primitive::u32,
        >,
      }
      impl ::subxt::Call for RemoveSub {
        const PALLET: &'static str = "Identity";
        const FUNCTION: &'static str = "remove_sub";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct QuitSub;
      impl ::subxt::Call for QuitSub {
        const PALLET: &'static str = "Identity";
        const FUNCTION: &'static str = "quit_sub";
      }
      pub struct TransactionApi<'a, T: ::subxt::Config, X> {
        client: &'a ::subxt::Client<T>,
        marker: ::core::marker::PhantomData<X>,
      }
      impl<'a, T, X> TransactionApi<'a, T, X>
      where
        T: ::subxt::Config,
        X: ::subxt::extrinsic::ExtrinsicParams<T>,
      {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self {
            client,
            marker: ::core::marker::PhantomData,
          }
        }
        pub fn add_registrar(
          &self,
          account: ::subxt::sp_core::crypto::AccountId32,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, AddRegistrar, DispatchError, root_mod::Event>
        {
          let call = AddRegistrar { account };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn set_identity(
          &self,
          info: runtime_types::pallet_identity::types::IdentityInfo,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, SetIdentity, DispatchError, root_mod::Event>
        {
          let call = SetIdentity {
            info: ::std::boxed::Box::new(info),
          };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn set_subs(
          &self,
          subs: ::std::vec::Vec<(
            ::subxt::sp_core::crypto::AccountId32,
            runtime_types::pallet_identity::types::Data,
          )>,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, SetSubs, DispatchError, root_mod::Event>
        {
          let call = SetSubs { subs };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn clear_identity(
          &self,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, ClearIdentity, DispatchError, root_mod::Event>
        {
          let call = ClearIdentity {};
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn request_judgement(
          &self,
          reg_index: ::core::primitive::u32,
          max_fee: ::core::primitive::u128,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, RequestJudgement, DispatchError, root_mod::Event>
        {
          let call = RequestJudgement { reg_index, max_fee };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn cancel_request(
          &self,
          reg_index: ::core::primitive::u32,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, CancelRequest, DispatchError, root_mod::Event>
        {
          let call = CancelRequest { reg_index };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn set_fee(
          &self,
          index: ::core::primitive::u32,
          fee: ::core::primitive::u128,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, SetFee, DispatchError, root_mod::Event>
        {
          let call = SetFee { index, fee };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn set_account_id(
          &self,
          index: ::core::primitive::u32,
          new: ::subxt::sp_core::crypto::AccountId32,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, SetAccountId, DispatchError, root_mod::Event>
        {
          let call = SetAccountId { index, new };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn set_fields(
          &self,
          index: ::core::primitive::u32,
          fields: runtime_types::pallet_identity::types::BitFlags<
            runtime_types::pallet_identity::types::IdentityField,
          >,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, SetFields, DispatchError, root_mod::Event>
        {
          let call = SetFields { index, fields };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn provide_judgement(
          &self,
          reg_index: ::core::primitive::u32,
          target: ::subxt::sp_runtime::MultiAddress<
            ::subxt::sp_core::crypto::AccountId32,
            ::core::primitive::u32,
          >,
          judgement: runtime_types::pallet_identity::types::Judgement<::core::primitive::u128>,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, ProvideJudgement, DispatchError, root_mod::Event>
        {
          let call = ProvideJudgement {
            reg_index,
            target,
            judgement,
          };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn kill_identity(
          &self,
          target: ::subxt::sp_runtime::MultiAddress<
            ::subxt::sp_core::crypto::AccountId32,
            ::core::primitive::u32,
          >,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, KillIdentity, DispatchError, root_mod::Event>
        {
          let call = KillIdentity { target };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn add_sub(
          &self,
          sub: ::subxt::sp_runtime::MultiAddress<
            ::subxt::sp_core::crypto::AccountId32,
            ::core::primitive::u32,
          >,
          data: runtime_types::pallet_identity::types::Data,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, AddSub, DispatchError, root_mod::Event>
        {
          let call = AddSub { sub, data };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn rename_sub(
          &self,
          sub: ::subxt::sp_runtime::MultiAddress<
            ::subxt::sp_core::crypto::AccountId32,
            ::core::primitive::u32,
          >,
          data: runtime_types::pallet_identity::types::Data,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, RenameSub, DispatchError, root_mod::Event>
        {
          let call = RenameSub { sub, data };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn remove_sub(
          &self,
          sub: ::subxt::sp_runtime::MultiAddress<
            ::subxt::sp_core::crypto::AccountId32,
            ::core::primitive::u32,
          >,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, RemoveSub, DispatchError, root_mod::Event>
        {
          let call = RemoveSub { sub };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn quit_sub(
          &self,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, QuitSub, DispatchError, root_mod::Event>
        {
          let call = QuitSub {};
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
      }
    }
    pub type Event = runtime_types::pallet_identity::pallet::Event;
    pub mod events {
      use super::runtime_types;
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct IdentitySet {
        pub who: ::subxt::sp_core::crypto::AccountId32,
      }
      impl ::subxt::Event for IdentitySet {
        const PALLET: &'static str = "Identity";
        const EVENT: &'static str = "IdentitySet";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct IdentityCleared {
        pub who: ::subxt::sp_core::crypto::AccountId32,
        pub deposit: ::core::primitive::u128,
      }
      impl ::subxt::Event for IdentityCleared {
        const PALLET: &'static str = "Identity";
        const EVENT: &'static str = "IdentityCleared";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct IdentityKilled {
        pub who: ::subxt::sp_core::crypto::AccountId32,
        pub deposit: ::core::primitive::u128,
      }
      impl ::subxt::Event for IdentityKilled {
        const PALLET: &'static str = "Identity";
        const EVENT: &'static str = "IdentityKilled";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct JudgementRequested {
        pub who: ::subxt::sp_core::crypto::AccountId32,
        pub registrar_index: ::core::primitive::u32,
      }
      impl ::subxt::Event for JudgementRequested {
        const PALLET: &'static str = "Identity";
        const EVENT: &'static str = "JudgementRequested";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct JudgementUnrequested {
        pub who: ::subxt::sp_core::crypto::AccountId32,
        pub registrar_index: ::core::primitive::u32,
      }
      impl ::subxt::Event for JudgementUnrequested {
        const PALLET: &'static str = "Identity";
        const EVENT: &'static str = "JudgementUnrequested";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct JudgementGiven {
        pub target: ::subxt::sp_core::crypto::AccountId32,
        pub registrar_index: ::core::primitive::u32,
      }
      impl ::subxt::Event for JudgementGiven {
        const PALLET: &'static str = "Identity";
        const EVENT: &'static str = "JudgementGiven";
      }
      #[derive(
        :: subxt :: codec :: Encode,
        :: subxt :: codec :: Decode,
        Debug,
        :: subxt :: codec :: CompactAs,
      )]
      pub struct RegistrarAdded {
        pub registrar_index: ::core::primitive::u32,
      }
      impl ::subxt::Event for RegistrarAdded {
        const PALLET: &'static str = "Identity";
        const EVENT: &'static str = "RegistrarAdded";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct SubIdentityAdded {
        pub sub: ::subxt::sp_core::crypto::AccountId32,
        pub main: ::subxt::sp_core::crypto::AccountId32,
        pub deposit: ::core::primitive::u128,
      }
      impl ::subxt::Event for SubIdentityAdded {
        const PALLET: &'static str = "Identity";
        const EVENT: &'static str = "SubIdentityAdded";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct SubIdentityRemoved {
        pub sub: ::subxt::sp_core::crypto::AccountId32,
        pub main: ::subxt::sp_core::crypto::AccountId32,
        pub deposit: ::core::primitive::u128,
      }
      impl ::subxt::Event for SubIdentityRemoved {
        const PALLET: &'static str = "Identity";
        const EVENT: &'static str = "SubIdentityRemoved";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct SubIdentityRevoked {
        pub sub: ::subxt::sp_core::crypto::AccountId32,
        pub main: ::subxt::sp_core::crypto::AccountId32,
        pub deposit: ::core::primitive::u128,
      }
      impl ::subxt::Event for SubIdentityRevoked {
        const PALLET: &'static str = "Identity";
        const EVENT: &'static str = "SubIdentityRevoked";
      }
    }
    pub mod storage {
      use super::runtime_types;
      pub struct IdentityOf<'a>(pub &'a ::subxt::sp_core::crypto::AccountId32);
      impl ::subxt::StorageEntry for IdentityOf<'_> {
        const PALLET: &'static str = "Identity";
        const STORAGE: &'static str = "IdentityOf";
        type Value = runtime_types::pallet_identity::types::Registration<::core::primitive::u128>;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
            &self.0,
            ::subxt::StorageHasher::Twox64Concat,
          )])
        }
      }
      pub struct SuperOf<'a>(pub &'a ::subxt::sp_core::crypto::AccountId32);
      impl ::subxt::StorageEntry for SuperOf<'_> {
        const PALLET: &'static str = "Identity";
        const STORAGE: &'static str = "SuperOf";
        type Value = (
          ::subxt::sp_core::crypto::AccountId32,
          runtime_types::pallet_identity::types::Data,
        );
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
            &self.0,
            ::subxt::StorageHasher::Blake2_128Concat,
          )])
        }
      }
      pub struct SubsOf<'a>(pub &'a ::subxt::sp_core::crypto::AccountId32);
      impl ::subxt::StorageEntry for SubsOf<'_> {
        const PALLET: &'static str = "Identity";
        const STORAGE: &'static str = "SubsOf";
        type Value = (
          ::core::primitive::u128,
          runtime_types::frame_support::storage::bounded_vec::BoundedVec<
            ::subxt::sp_core::crypto::AccountId32,
          >,
        );
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
            &self.0,
            ::subxt::StorageHasher::Twox64Concat,
          )])
        }
      }
      pub struct Registrars;
      impl ::subxt::StorageEntry for Registrars {
        const PALLET: &'static str = "Identity";
        const STORAGE: &'static str = "Registrars";
        type Value = runtime_types::frame_support::storage::bounded_vec::BoundedVec<
          ::core::option::Option<
            runtime_types::pallet_identity::types::RegistrarInfo<
              ::core::primitive::u128,
              ::subxt::sp_core::crypto::AccountId32,
            >,
          >,
        >;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct StorageApi<'a, T: ::subxt::Config> {
        client: &'a ::subxt::Client<T>,
      }
      impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self { client }
        }
        pub async fn identity_of(
          &self,
          _0: &::subxt::sp_core::crypto::AccountId32,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::core::option::Option<
            runtime_types::pallet_identity::types::Registration<::core::primitive::u128>,
          >,
          ::subxt::BasicError,
        > {
          let entry = IdentityOf(_0);
          self.client.storage().fetch(&entry, hash).await
        }
        pub async fn identity_of_iter(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::subxt::KeyIter<'a, T, IdentityOf<'a>>, ::subxt::BasicError>
        {
          self.client.storage().iter(hash).await
        }
        pub async fn super_of(
          &self,
          _0: &::subxt::sp_core::crypto::AccountId32,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::core::option::Option<(
            ::subxt::sp_core::crypto::AccountId32,
            runtime_types::pallet_identity::types::Data,
          )>,
          ::subxt::BasicError,
        > {
          let entry = SuperOf(_0);
          self.client.storage().fetch(&entry, hash).await
        }
        pub async fn super_of_iter(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::subxt::KeyIter<'a, T, SuperOf<'a>>, ::subxt::BasicError>
        {
          self.client.storage().iter(hash).await
        }
        pub async fn subs_of(
          &self,
          _0: &::subxt::sp_core::crypto::AccountId32,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          (
            ::core::primitive::u128,
            runtime_types::frame_support::storage::bounded_vec::BoundedVec<
              ::subxt::sp_core::crypto::AccountId32,
            >,
          ),
          ::subxt::BasicError,
        > {
          let entry = SubsOf(_0);
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn subs_of_iter(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::subxt::KeyIter<'a, T, SubsOf<'a>>, ::subxt::BasicError>
        {
          self.client.storage().iter(hash).await
        }
        pub async fn registrars(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          runtime_types::frame_support::storage::bounded_vec::BoundedVec<
            ::core::option::Option<
              runtime_types::pallet_identity::types::RegistrarInfo<
                ::core::primitive::u128,
                ::subxt::sp_core::crypto::AccountId32,
              >,
            >,
          >,
          ::subxt::BasicError,
        > {
          let entry = Registrars;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
      }
    }
    pub mod constants {
      use super::runtime_types;
      pub struct ConstantsApi<'a, T: ::subxt::Config> {
        client: &'a ::subxt::Client<T>,
      }
      impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self { client }
        }
        pub fn basic_deposit(
          &self,
        ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError> {
          let pallet = self.client.metadata().pallet("Identity")?;
          let constant = pallet.constant("BasicDeposit")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn field_deposit(
          &self,
        ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError> {
          let pallet = self.client.metadata().pallet("Identity")?;
          let constant = pallet.constant("FieldDeposit")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn sub_account_deposit(
          &self,
        ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError> {
          let pallet = self.client.metadata().pallet("Identity")?;
          let constant = pallet.constant("SubAccountDeposit")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn max_sub_accounts(
          &self,
        ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
          let pallet = self.client.metadata().pallet("Identity")?;
          let constant = pallet.constant("MaxSubAccounts")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn max_additional_fields(
          &self,
        ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
          let pallet = self.client.metadata().pallet("Identity")?;
          let constant = pallet.constant("MaxAdditionalFields")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn max_registrars(
          &self,
        ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
          let pallet = self.client.metadata().pallet("Identity")?;
          let constant = pallet.constant("MaxRegistrars")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
      }
    }
  }
  pub mod election_provider_multi_phase {
    use super::root_mod;
    use super::runtime_types;
    pub mod calls {
      use super::root_mod;
      use super::runtime_types;
      type DispatchError = runtime_types::sp_runtime::DispatchError;
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct SubmitUnsigned {
        pub raw_solution: ::std::boxed::Box<
          runtime_types::pallet_election_provider_multi_phase::RawSolution<
            runtime_types::lagoon_runtime::config::consensus::NposCompactSolution16,
          >,
        >,
        pub witness: runtime_types::pallet_election_provider_multi_phase::SolutionOrSnapshotSize,
      }
      impl ::subxt::Call for SubmitUnsigned {
        const PALLET: &'static str = "ElectionProviderMultiPhase";
        const FUNCTION: &'static str = "submit_unsigned";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct SetMinimumUntrustedScore {
        pub maybe_next_score:
          ::core::option::Option<runtime_types::sp_npos_elections::ElectionScore>,
      }
      impl ::subxt::Call for SetMinimumUntrustedScore {
        const PALLET: &'static str = "ElectionProviderMultiPhase";
        const FUNCTION: &'static str = "set_minimum_untrusted_score";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct SetEmergencyElectionResult {
        pub supports: ::std::vec::Vec<(
          ::subxt::sp_core::crypto::AccountId32,
          runtime_types::sp_npos_elections::Support<::subxt::sp_core::crypto::AccountId32>,
        )>,
      }
      impl ::subxt::Call for SetEmergencyElectionResult {
        const PALLET: &'static str = "ElectionProviderMultiPhase";
        const FUNCTION: &'static str = "set_emergency_election_result";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Submit {
        pub raw_solution: ::std::boxed::Box<
          runtime_types::pallet_election_provider_multi_phase::RawSolution<
            runtime_types::lagoon_runtime::config::consensus::NposCompactSolution16,
          >,
        >,
      }
      impl ::subxt::Call for Submit {
        const PALLET: &'static str = "ElectionProviderMultiPhase";
        const FUNCTION: &'static str = "submit";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct GovernanceFallback {
        pub maybe_max_voters: ::core::option::Option<::core::primitive::u32>,
        pub maybe_max_targets: ::core::option::Option<::core::primitive::u32>,
      }
      impl ::subxt::Call for GovernanceFallback {
        const PALLET: &'static str = "ElectionProviderMultiPhase";
        const FUNCTION: &'static str = "governance_fallback";
      }
      pub struct TransactionApi<'a, T: ::subxt::Config, X> {
        client: &'a ::subxt::Client<T>,
        marker: ::core::marker::PhantomData<X>,
      }
      impl<'a, T, X> TransactionApi<'a, T, X>
      where
        T: ::subxt::Config,
        X: ::subxt::extrinsic::ExtrinsicParams<T>,
      {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self {
            client,
            marker: ::core::marker::PhantomData,
          }
        }
        pub fn submit_unsigned(
          &self,
          raw_solution: runtime_types::pallet_election_provider_multi_phase::RawSolution<
            runtime_types::lagoon_runtime::config::consensus::NposCompactSolution16,
          >,
          witness: runtime_types::pallet_election_provider_multi_phase::SolutionOrSnapshotSize,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, SubmitUnsigned, DispatchError, root_mod::Event>
        {
          let call = SubmitUnsigned {
            raw_solution: ::std::boxed::Box::new(raw_solution),
            witness,
          };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn set_minimum_untrusted_score(
          &self,
          maybe_next_score: ::core::option::Option<runtime_types::sp_npos_elections::ElectionScore>,
        ) -> ::subxt::SubmittableExtrinsic<
          'a,
          T,
          X,
          SetMinimumUntrustedScore,
          DispatchError,
          root_mod::Event,
        > {
          let call = SetMinimumUntrustedScore { maybe_next_score };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn set_emergency_election_result(
          &self,
          supports: ::std::vec::Vec<(
            ::subxt::sp_core::crypto::AccountId32,
            runtime_types::sp_npos_elections::Support<::subxt::sp_core::crypto::AccountId32>,
          )>,
        ) -> ::subxt::SubmittableExtrinsic<
          'a,
          T,
          X,
          SetEmergencyElectionResult,
          DispatchError,
          root_mod::Event,
        > {
          let call = SetEmergencyElectionResult { supports };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn submit(
          &self,
          raw_solution: runtime_types::pallet_election_provider_multi_phase::RawSolution<
            runtime_types::lagoon_runtime::config::consensus::NposCompactSolution16,
          >,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, Submit, DispatchError, root_mod::Event>
        {
          let call = Submit {
            raw_solution: ::std::boxed::Box::new(raw_solution),
          };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn governance_fallback(
          &self,
          maybe_max_voters: ::core::option::Option<::core::primitive::u32>,
          maybe_max_targets: ::core::option::Option<::core::primitive::u32>,
        ) -> ::subxt::SubmittableExtrinsic<
          'a,
          T,
          X,
          GovernanceFallback,
          DispatchError,
          root_mod::Event,
        > {
          let call = GovernanceFallback {
            maybe_max_voters,
            maybe_max_targets,
          };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
      }
    }
    pub type Event = runtime_types::pallet_election_provider_multi_phase::pallet::Event;
    pub mod events {
      use super::runtime_types;
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct SolutionStored {
        pub election_compute: runtime_types::pallet_election_provider_multi_phase::ElectionCompute,
        pub prev_ejected: ::core::primitive::bool,
      }
      impl ::subxt::Event for SolutionStored {
        const PALLET: &'static str = "ElectionProviderMultiPhase";
        const EVENT: &'static str = "SolutionStored";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct ElectionFinalized {
        pub election_compute: ::core::option::Option<
          runtime_types::pallet_election_provider_multi_phase::ElectionCompute,
        >,
      }
      impl ::subxt::Event for ElectionFinalized {
        const PALLET: &'static str = "ElectionProviderMultiPhase";
        const EVENT: &'static str = "ElectionFinalized";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Rewarded {
        pub account: ::subxt::sp_core::crypto::AccountId32,
        pub value: ::core::primitive::u128,
      }
      impl ::subxt::Event for Rewarded {
        const PALLET: &'static str = "ElectionProviderMultiPhase";
        const EVENT: &'static str = "Rewarded";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Slashed {
        pub account: ::subxt::sp_core::crypto::AccountId32,
        pub value: ::core::primitive::u128,
      }
      impl ::subxt::Event for Slashed {
        const PALLET: &'static str = "ElectionProviderMultiPhase";
        const EVENT: &'static str = "Slashed";
      }
      #[derive(
        :: subxt :: codec :: Encode,
        :: subxt :: codec :: Decode,
        Debug,
        :: subxt :: codec :: CompactAs,
      )]
      pub struct SignedPhaseStarted {
        pub round: ::core::primitive::u32,
      }
      impl ::subxt::Event for SignedPhaseStarted {
        const PALLET: &'static str = "ElectionProviderMultiPhase";
        const EVENT: &'static str = "SignedPhaseStarted";
      }
      #[derive(
        :: subxt :: codec :: Encode,
        :: subxt :: codec :: Decode,
        Debug,
        :: subxt :: codec :: CompactAs,
      )]
      pub struct UnsignedPhaseStarted {
        pub round: ::core::primitive::u32,
      }
      impl ::subxt::Event for UnsignedPhaseStarted {
        const PALLET: &'static str = "ElectionProviderMultiPhase";
        const EVENT: &'static str = "UnsignedPhaseStarted";
      }
    }
    pub mod storage {
      use super::runtime_types;
      pub struct Round;
      impl ::subxt::StorageEntry for Round {
        const PALLET: &'static str = "ElectionProviderMultiPhase";
        const STORAGE: &'static str = "Round";
        type Value = ::core::primitive::u32;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct CurrentPhase;
      impl ::subxt::StorageEntry for CurrentPhase {
        const PALLET: &'static str = "ElectionProviderMultiPhase";
        const STORAGE: &'static str = "CurrentPhase";
        type Value =
          runtime_types::pallet_election_provider_multi_phase::Phase<::core::primitive::u32>;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct QueuedSolution;
      impl ::subxt::StorageEntry for QueuedSolution {
        const PALLET: &'static str = "ElectionProviderMultiPhase";
        const STORAGE: &'static str = "QueuedSolution";
        type Value = runtime_types::pallet_election_provider_multi_phase::ReadySolution<
          ::subxt::sp_core::crypto::AccountId32,
        >;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct Snapshot;
      impl ::subxt::StorageEntry for Snapshot {
        const PALLET: &'static str = "ElectionProviderMultiPhase";
        const STORAGE: &'static str = "Snapshot";
        type Value = runtime_types::pallet_election_provider_multi_phase::RoundSnapshot;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct DesiredTargets;
      impl ::subxt::StorageEntry for DesiredTargets {
        const PALLET: &'static str = "ElectionProviderMultiPhase";
        const STORAGE: &'static str = "DesiredTargets";
        type Value = ::core::primitive::u32;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct SnapshotMetadata;
      impl ::subxt::StorageEntry for SnapshotMetadata {
        const PALLET: &'static str = "ElectionProviderMultiPhase";
        const STORAGE: &'static str = "SnapshotMetadata";
        type Value = runtime_types::pallet_election_provider_multi_phase::SolutionOrSnapshotSize;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct SignedSubmissionNextIndex;
      impl ::subxt::StorageEntry for SignedSubmissionNextIndex {
        const PALLET: &'static str = "ElectionProviderMultiPhase";
        const STORAGE: &'static str = "SignedSubmissionNextIndex";
        type Value = ::core::primitive::u32;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct SignedSubmissionIndices;
      impl ::subxt::StorageEntry for SignedSubmissionIndices {
        const PALLET: &'static str = "ElectionProviderMultiPhase";
        const STORAGE: &'static str = "SignedSubmissionIndices";
        type Value = runtime_types::frame_support::storage::bounded_btree_map::BoundedBTreeMap<
          runtime_types::sp_npos_elections::ElectionScore,
          ::core::primitive::u32,
        >;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct SignedSubmissionsMap<'a>(pub &'a ::core::primitive::u32);
      impl ::subxt::StorageEntry for SignedSubmissionsMap<'_> {
        const PALLET: &'static str = "ElectionProviderMultiPhase";
        const STORAGE: &'static str = "SignedSubmissionsMap";
        type Value = runtime_types::pallet_election_provider_multi_phase::signed::SignedSubmission<
          ::subxt::sp_core::crypto::AccountId32,
          ::core::primitive::u128,
          runtime_types::lagoon_runtime::config::consensus::NposCompactSolution16,
        >;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
            &self.0,
            ::subxt::StorageHasher::Twox64Concat,
          )])
        }
      }
      pub struct MinimumUntrustedScore;
      impl ::subxt::StorageEntry for MinimumUntrustedScore {
        const PALLET: &'static str = "ElectionProviderMultiPhase";
        const STORAGE: &'static str = "MinimumUntrustedScore";
        type Value = runtime_types::sp_npos_elections::ElectionScore;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct StorageApi<'a, T: ::subxt::Config> {
        client: &'a ::subxt::Client<T>,
      }
      impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self { client }
        }
        pub async fn round(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
          let entry = Round;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn current_phase(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          runtime_types::pallet_election_provider_multi_phase::Phase<::core::primitive::u32>,
          ::subxt::BasicError,
        > {
          let entry = CurrentPhase;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn queued_solution(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::core::option::Option<
            runtime_types::pallet_election_provider_multi_phase::ReadySolution<
              ::subxt::sp_core::crypto::AccountId32,
            >,
          >,
          ::subxt::BasicError,
        > {
          let entry = QueuedSolution;
          self.client.storage().fetch(&entry, hash).await
        }
        pub async fn snapshot(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::core::option::Option<
            runtime_types::pallet_election_provider_multi_phase::RoundSnapshot,
          >,
          ::subxt::BasicError,
        > {
          let entry = Snapshot;
          self.client.storage().fetch(&entry, hash).await
        }
        pub async fn desired_targets(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::core::option::Option<::core::primitive::u32>,
          ::subxt::BasicError,
        > {
          let entry = DesiredTargets;
          self.client.storage().fetch(&entry, hash).await
        }
        pub async fn snapshot_metadata(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::core::option::Option<
            runtime_types::pallet_election_provider_multi_phase::SolutionOrSnapshotSize,
          >,
          ::subxt::BasicError,
        > {
          let entry = SnapshotMetadata;
          self.client.storage().fetch(&entry, hash).await
        }
        pub async fn signed_submission_next_index(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
          let entry = SignedSubmissionNextIndex;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn signed_submission_indices(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          runtime_types::frame_support::storage::bounded_btree_map::BoundedBTreeMap<
            runtime_types::sp_npos_elections::ElectionScore,
            ::core::primitive::u32,
          >,
          ::subxt::BasicError,
        > {
          let entry = SignedSubmissionIndices;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn signed_submissions_map(
          &self,
          _0: &::core::primitive::u32,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::core::option::Option<
            runtime_types::pallet_election_provider_multi_phase::signed::SignedSubmission<
              ::subxt::sp_core::crypto::AccountId32,
              ::core::primitive::u128,
              runtime_types::lagoon_runtime::config::consensus::NposCompactSolution16,
            >,
          >,
          ::subxt::BasicError,
        > {
          let entry = SignedSubmissionsMap(_0);
          self.client.storage().fetch(&entry, hash).await
        }
        pub async fn signed_submissions_map_iter(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::subxt::KeyIter<'a, T, SignedSubmissionsMap<'a>>,
          ::subxt::BasicError,
        > {
          self.client.storage().iter(hash).await
        }
        pub async fn minimum_untrusted_score(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::core::option::Option<runtime_types::sp_npos_elections::ElectionScore>,
          ::subxt::BasicError,
        > {
          let entry = MinimumUntrustedScore;
          self.client.storage().fetch(&entry, hash).await
        }
      }
    }
    pub mod constants {
      use super::runtime_types;
      pub struct ConstantsApi<'a, T: ::subxt::Config> {
        client: &'a ::subxt::Client<T>,
      }
      impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self { client }
        }
        pub fn unsigned_phase(
          &self,
        ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
          let pallet = self
            .client
            .metadata()
            .pallet("ElectionProviderMultiPhase")?;
          let constant = pallet.constant("UnsignedPhase")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn signed_phase(
          &self,
        ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
          let pallet = self
            .client
            .metadata()
            .pallet("ElectionProviderMultiPhase")?;
          let constant = pallet.constant("SignedPhase")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn solution_improvement_threshold(
          &self,
        ) -> ::core::result::Result<
          runtime_types::sp_arithmetic::per_things::Perbill,
          ::subxt::BasicError,
        > {
          let pallet = self
            .client
            .metadata()
            .pallet("ElectionProviderMultiPhase")?;
          let constant = pallet.constant("SolutionImprovementThreshold")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn offchain_repeat(
          &self,
        ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
          let pallet = self
            .client
            .metadata()
            .pallet("ElectionProviderMultiPhase")?;
          let constant = pallet.constant("OffchainRepeat")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn miner_tx_priority(
          &self,
        ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError> {
          let pallet = self
            .client
            .metadata()
            .pallet("ElectionProviderMultiPhase")?;
          let constant = pallet.constant("MinerTxPriority")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn miner_max_weight(
          &self,
        ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError> {
          let pallet = self
            .client
            .metadata()
            .pallet("ElectionProviderMultiPhase")?;
          let constant = pallet.constant("MinerMaxWeight")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn signed_max_submissions(
          &self,
        ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
          let pallet = self
            .client
            .metadata()
            .pallet("ElectionProviderMultiPhase")?;
          let constant = pallet.constant("SignedMaxSubmissions")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn signed_max_weight(
          &self,
        ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError> {
          let pallet = self
            .client
            .metadata()
            .pallet("ElectionProviderMultiPhase")?;
          let constant = pallet.constant("SignedMaxWeight")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn signed_reward_base(
          &self,
        ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError> {
          let pallet = self
            .client
            .metadata()
            .pallet("ElectionProviderMultiPhase")?;
          let constant = pallet.constant("SignedRewardBase")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn signed_deposit_base(
          &self,
        ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError> {
          let pallet = self
            .client
            .metadata()
            .pallet("ElectionProviderMultiPhase")?;
          let constant = pallet.constant("SignedDepositBase")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn signed_deposit_byte(
          &self,
        ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError> {
          let pallet = self
            .client
            .metadata()
            .pallet("ElectionProviderMultiPhase")?;
          let constant = pallet.constant("SignedDepositByte")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn signed_deposit_weight(
          &self,
        ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError> {
          let pallet = self
            .client
            .metadata()
            .pallet("ElectionProviderMultiPhase")?;
          let constant = pallet.constant("SignedDepositWeight")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn max_electing_voters(
          &self,
        ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
          let pallet = self
            .client
            .metadata()
            .pallet("ElectionProviderMultiPhase")?;
          let constant = pallet.constant("MaxElectingVoters")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn max_electable_targets(
          &self,
        ) -> ::core::result::Result<::core::primitive::u16, ::subxt::BasicError> {
          let pallet = self
            .client
            .metadata()
            .pallet("ElectionProviderMultiPhase")?;
          let constant = pallet.constant("MaxElectableTargets")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn miner_max_length(
          &self,
        ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
          let pallet = self
            .client
            .metadata()
            .pallet("ElectionProviderMultiPhase")?;
          let constant = pallet.constant("MinerMaxLength")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
      }
    }
  }
  pub mod recovery {
    use super::root_mod;
    use super::runtime_types;
    pub mod calls {
      use super::root_mod;
      use super::runtime_types;
      type DispatchError = runtime_types::sp_runtime::DispatchError;
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct AsRecovered {
        pub account: ::subxt::sp_core::crypto::AccountId32,
        pub call: ::std::boxed::Box<runtime_types::lagoon_runtime::Call>,
      }
      impl ::subxt::Call for AsRecovered {
        const PALLET: &'static str = "Recovery";
        const FUNCTION: &'static str = "as_recovered";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct SetRecovered {
        pub lost: ::subxt::sp_core::crypto::AccountId32,
        pub rescuer: ::subxt::sp_core::crypto::AccountId32,
      }
      impl ::subxt::Call for SetRecovered {
        const PALLET: &'static str = "Recovery";
        const FUNCTION: &'static str = "set_recovered";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct CreateRecovery {
        pub friends: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
        pub threshold: ::core::primitive::u16,
        pub delay_period: ::core::primitive::u32,
      }
      impl ::subxt::Call for CreateRecovery {
        const PALLET: &'static str = "Recovery";
        const FUNCTION: &'static str = "create_recovery";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct InitiateRecovery {
        pub account: ::subxt::sp_core::crypto::AccountId32,
      }
      impl ::subxt::Call for InitiateRecovery {
        const PALLET: &'static str = "Recovery";
        const FUNCTION: &'static str = "initiate_recovery";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct VouchRecovery {
        pub lost: ::subxt::sp_core::crypto::AccountId32,
        pub rescuer: ::subxt::sp_core::crypto::AccountId32,
      }
      impl ::subxt::Call for VouchRecovery {
        const PALLET: &'static str = "Recovery";
        const FUNCTION: &'static str = "vouch_recovery";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct ClaimRecovery {
        pub account: ::subxt::sp_core::crypto::AccountId32,
      }
      impl ::subxt::Call for ClaimRecovery {
        const PALLET: &'static str = "Recovery";
        const FUNCTION: &'static str = "claim_recovery";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct CloseRecovery {
        pub rescuer: ::subxt::sp_core::crypto::AccountId32,
      }
      impl ::subxt::Call for CloseRecovery {
        const PALLET: &'static str = "Recovery";
        const FUNCTION: &'static str = "close_recovery";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct RemoveRecovery;
      impl ::subxt::Call for RemoveRecovery {
        const PALLET: &'static str = "Recovery";
        const FUNCTION: &'static str = "remove_recovery";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct CancelRecovered {
        pub account: ::subxt::sp_core::crypto::AccountId32,
      }
      impl ::subxt::Call for CancelRecovered {
        const PALLET: &'static str = "Recovery";
        const FUNCTION: &'static str = "cancel_recovered";
      }
      pub struct TransactionApi<'a, T: ::subxt::Config, X> {
        client: &'a ::subxt::Client<T>,
        marker: ::core::marker::PhantomData<X>,
      }
      impl<'a, T, X> TransactionApi<'a, T, X>
      where
        T: ::subxt::Config,
        X: ::subxt::extrinsic::ExtrinsicParams<T>,
      {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self {
            client,
            marker: ::core::marker::PhantomData,
          }
        }
        pub fn as_recovered(
          &self,
          account: ::subxt::sp_core::crypto::AccountId32,
          call: runtime_types::lagoon_runtime::Call,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, AsRecovered, DispatchError, root_mod::Event>
        {
          let call = AsRecovered {
            account,
            call: ::std::boxed::Box::new(call),
          };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn set_recovered(
          &self,
          lost: ::subxt::sp_core::crypto::AccountId32,
          rescuer: ::subxt::sp_core::crypto::AccountId32,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, SetRecovered, DispatchError, root_mod::Event>
        {
          let call = SetRecovered { lost, rescuer };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn create_recovery(
          &self,
          friends: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
          threshold: ::core::primitive::u16,
          delay_period: ::core::primitive::u32,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, CreateRecovery, DispatchError, root_mod::Event>
        {
          let call = CreateRecovery {
            friends,
            threshold,
            delay_period,
          };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn initiate_recovery(
          &self,
          account: ::subxt::sp_core::crypto::AccountId32,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, InitiateRecovery, DispatchError, root_mod::Event>
        {
          let call = InitiateRecovery { account };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn vouch_recovery(
          &self,
          lost: ::subxt::sp_core::crypto::AccountId32,
          rescuer: ::subxt::sp_core::crypto::AccountId32,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, VouchRecovery, DispatchError, root_mod::Event>
        {
          let call = VouchRecovery { lost, rescuer };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn claim_recovery(
          &self,
          account: ::subxt::sp_core::crypto::AccountId32,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, ClaimRecovery, DispatchError, root_mod::Event>
        {
          let call = ClaimRecovery { account };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn close_recovery(
          &self,
          rescuer: ::subxt::sp_core::crypto::AccountId32,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, CloseRecovery, DispatchError, root_mod::Event>
        {
          let call = CloseRecovery { rescuer };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn remove_recovery(
          &self,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, RemoveRecovery, DispatchError, root_mod::Event>
        {
          let call = RemoveRecovery {};
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn cancel_recovered(
          &self,
          account: ::subxt::sp_core::crypto::AccountId32,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, CancelRecovered, DispatchError, root_mod::Event>
        {
          let call = CancelRecovered { account };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
      }
    }
    pub type Event = runtime_types::pallet_recovery::pallet::Event;
    pub mod events {
      use super::runtime_types;
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct RecoveryCreated {
        pub account: ::subxt::sp_core::crypto::AccountId32,
      }
      impl ::subxt::Event for RecoveryCreated {
        const PALLET: &'static str = "Recovery";
        const EVENT: &'static str = "RecoveryCreated";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct RecoveryInitiated {
        pub lost_account: ::subxt::sp_core::crypto::AccountId32,
        pub rescuer_account: ::subxt::sp_core::crypto::AccountId32,
      }
      impl ::subxt::Event for RecoveryInitiated {
        const PALLET: &'static str = "Recovery";
        const EVENT: &'static str = "RecoveryInitiated";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct RecoveryVouched {
        pub lost_account: ::subxt::sp_core::crypto::AccountId32,
        pub rescuer_account: ::subxt::sp_core::crypto::AccountId32,
        pub sender: ::subxt::sp_core::crypto::AccountId32,
      }
      impl ::subxt::Event for RecoveryVouched {
        const PALLET: &'static str = "Recovery";
        const EVENT: &'static str = "RecoveryVouched";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct RecoveryClosed {
        pub lost_account: ::subxt::sp_core::crypto::AccountId32,
        pub rescuer_account: ::subxt::sp_core::crypto::AccountId32,
      }
      impl ::subxt::Event for RecoveryClosed {
        const PALLET: &'static str = "Recovery";
        const EVENT: &'static str = "RecoveryClosed";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct AccountRecovered {
        pub lost_account: ::subxt::sp_core::crypto::AccountId32,
        pub rescuer_account: ::subxt::sp_core::crypto::AccountId32,
      }
      impl ::subxt::Event for AccountRecovered {
        const PALLET: &'static str = "Recovery";
        const EVENT: &'static str = "AccountRecovered";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct RecoveryRemoved {
        pub lost_account: ::subxt::sp_core::crypto::AccountId32,
      }
      impl ::subxt::Event for RecoveryRemoved {
        const PALLET: &'static str = "Recovery";
        const EVENT: &'static str = "RecoveryRemoved";
      }
    }
    pub mod storage {
      use super::runtime_types;
      pub struct Recoverable<'a>(pub &'a ::subxt::sp_core::crypto::AccountId32);
      impl ::subxt::StorageEntry for Recoverable<'_> {
        const PALLET: &'static str = "Recovery";
        const STORAGE: &'static str = "Recoverable";
        type Value = runtime_types::pallet_recovery::RecoveryConfig<
          ::core::primitive::u32,
          ::core::primitive::u128,
          runtime_types::frame_support::storage::bounded_vec::BoundedVec<
            ::subxt::sp_core::crypto::AccountId32,
          >,
        >;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
            &self.0,
            ::subxt::StorageHasher::Twox64Concat,
          )])
        }
      }
      pub struct ActiveRecoveries<'a>(
        pub &'a ::subxt::sp_core::crypto::AccountId32,
        pub &'a ::subxt::sp_core::crypto::AccountId32,
      );
      impl ::subxt::StorageEntry for ActiveRecoveries<'_> {
        const PALLET: &'static str = "Recovery";
        const STORAGE: &'static str = "ActiveRecoveries";
        type Value = runtime_types::pallet_recovery::ActiveRecovery<
          ::core::primitive::u32,
          ::core::primitive::u128,
          runtime_types::frame_support::storage::bounded_vec::BoundedVec<
            ::subxt::sp_core::crypto::AccountId32,
          >,
        >;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Map(vec![
            ::subxt::StorageMapKey::new(&self.0, ::subxt::StorageHasher::Twox64Concat),
            ::subxt::StorageMapKey::new(&self.1, ::subxt::StorageHasher::Twox64Concat),
          ])
        }
      }
      pub struct Proxy<'a>(pub &'a ::subxt::sp_core::crypto::AccountId32);
      impl ::subxt::StorageEntry for Proxy<'_> {
        const PALLET: &'static str = "Recovery";
        const STORAGE: &'static str = "Proxy";
        type Value = ::subxt::sp_core::crypto::AccountId32;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
            &self.0,
            ::subxt::StorageHasher::Blake2_128Concat,
          )])
        }
      }
      pub struct StorageApi<'a, T: ::subxt::Config> {
        client: &'a ::subxt::Client<T>,
      }
      impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self { client }
        }
        pub async fn recoverable(
          &self,
          _0: &::subxt::sp_core::crypto::AccountId32,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::core::option::Option<
            runtime_types::pallet_recovery::RecoveryConfig<
              ::core::primitive::u32,
              ::core::primitive::u128,
              runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                ::subxt::sp_core::crypto::AccountId32,
              >,
            >,
          >,
          ::subxt::BasicError,
        > {
          let entry = Recoverable(_0);
          self.client.storage().fetch(&entry, hash).await
        }
        pub async fn recoverable_iter(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Recoverable<'a>>, ::subxt::BasicError>
        {
          self.client.storage().iter(hash).await
        }
        pub async fn active_recoveries(
          &self,
          _0: &::subxt::sp_core::crypto::AccountId32,
          _1: &::subxt::sp_core::crypto::AccountId32,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::core::option::Option<
            runtime_types::pallet_recovery::ActiveRecovery<
              ::core::primitive::u32,
              ::core::primitive::u128,
              runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                ::subxt::sp_core::crypto::AccountId32,
              >,
            >,
          >,
          ::subxt::BasicError,
        > {
          let entry = ActiveRecoveries(_0, _1);
          self.client.storage().fetch(&entry, hash).await
        }
        pub async fn active_recoveries_iter(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::subxt::KeyIter<'a, T, ActiveRecoveries<'a>>,
          ::subxt::BasicError,
        > {
          self.client.storage().iter(hash).await
        }
        pub async fn proxy(
          &self,
          _0: &::subxt::sp_core::crypto::AccountId32,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
          ::subxt::BasicError,
        > {
          let entry = Proxy(_0);
          self.client.storage().fetch(&entry, hash).await
        }
        pub async fn proxy_iter(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Proxy<'a>>, ::subxt::BasicError>
        {
          self.client.storage().iter(hash).await
        }
      }
    }
    pub mod constants {
      use super::runtime_types;
      pub struct ConstantsApi<'a, T: ::subxt::Config> {
        client: &'a ::subxt::Client<T>,
      }
      impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self { client }
        }
        pub fn config_deposit_base(
          &self,
        ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError> {
          let pallet = self.client.metadata().pallet("Recovery")?;
          let constant = pallet.constant("ConfigDepositBase")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn friend_deposit_factor(
          &self,
        ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError> {
          let pallet = self.client.metadata().pallet("Recovery")?;
          let constant = pallet.constant("FriendDepositFactor")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn max_friends(
          &self,
        ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
          let pallet = self.client.metadata().pallet("Recovery")?;
          let constant = pallet.constant("MaxFriends")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn recovery_deposit(
          &self,
        ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError> {
          let pallet = self.client.metadata().pallet("Recovery")?;
          let constant = pallet.constant("RecoveryDeposit")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
      }
    }
  }
  pub mod scheduler {
    use super::root_mod;
    use super::runtime_types;
    pub mod calls {
      use super::root_mod;
      use super::runtime_types;
      type DispatchError = runtime_types::sp_runtime::DispatchError;
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Schedule {
        pub when: ::core::primitive::u32,
        pub maybe_periodic:
          ::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
        pub priority: ::core::primitive::u8,
        pub call: ::std::boxed::Box<
          runtime_types::frame_support::traits::schedule::MaybeHashed<
            runtime_types::lagoon_runtime::Call,
            ::subxt::sp_core::H256,
          >,
        >,
      }
      impl ::subxt::Call for Schedule {
        const PALLET: &'static str = "Scheduler";
        const FUNCTION: &'static str = "schedule";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Cancel {
        pub when: ::core::primitive::u32,
        pub index: ::core::primitive::u32,
      }
      impl ::subxt::Call for Cancel {
        const PALLET: &'static str = "Scheduler";
        const FUNCTION: &'static str = "cancel";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct ScheduleNamed {
        pub id: ::std::vec::Vec<::core::primitive::u8>,
        pub when: ::core::primitive::u32,
        pub maybe_periodic:
          ::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
        pub priority: ::core::primitive::u8,
        pub call: ::std::boxed::Box<
          runtime_types::frame_support::traits::schedule::MaybeHashed<
            runtime_types::lagoon_runtime::Call,
            ::subxt::sp_core::H256,
          >,
        >,
      }
      impl ::subxt::Call for ScheduleNamed {
        const PALLET: &'static str = "Scheduler";
        const FUNCTION: &'static str = "schedule_named";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct CancelNamed {
        pub id: ::std::vec::Vec<::core::primitive::u8>,
      }
      impl ::subxt::Call for CancelNamed {
        const PALLET: &'static str = "Scheduler";
        const FUNCTION: &'static str = "cancel_named";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct ScheduleAfter {
        pub after: ::core::primitive::u32,
        pub maybe_periodic:
          ::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
        pub priority: ::core::primitive::u8,
        pub call: ::std::boxed::Box<
          runtime_types::frame_support::traits::schedule::MaybeHashed<
            runtime_types::lagoon_runtime::Call,
            ::subxt::sp_core::H256,
          >,
        >,
      }
      impl ::subxt::Call for ScheduleAfter {
        const PALLET: &'static str = "Scheduler";
        const FUNCTION: &'static str = "schedule_after";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct ScheduleNamedAfter {
        pub id: ::std::vec::Vec<::core::primitive::u8>,
        pub after: ::core::primitive::u32,
        pub maybe_periodic:
          ::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
        pub priority: ::core::primitive::u8,
        pub call: ::std::boxed::Box<
          runtime_types::frame_support::traits::schedule::MaybeHashed<
            runtime_types::lagoon_runtime::Call,
            ::subxt::sp_core::H256,
          >,
        >,
      }
      impl ::subxt::Call for ScheduleNamedAfter {
        const PALLET: &'static str = "Scheduler";
        const FUNCTION: &'static str = "schedule_named_after";
      }
      pub struct TransactionApi<'a, T: ::subxt::Config, X> {
        client: &'a ::subxt::Client<T>,
        marker: ::core::marker::PhantomData<X>,
      }
      impl<'a, T, X> TransactionApi<'a, T, X>
      where
        T: ::subxt::Config,
        X: ::subxt::extrinsic::ExtrinsicParams<T>,
      {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self {
            client,
            marker: ::core::marker::PhantomData,
          }
        }
        pub fn schedule(
          &self,
          when: ::core::primitive::u32,
          maybe_periodic: ::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
          priority: ::core::primitive::u8,
          call: runtime_types::frame_support::traits::schedule::MaybeHashed<
            runtime_types::lagoon_runtime::Call,
            ::subxt::sp_core::H256,
          >,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, Schedule, DispatchError, root_mod::Event>
        {
          let call = Schedule {
            when,
            maybe_periodic,
            priority,
            call: ::std::boxed::Box::new(call),
          };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn cancel(
          &self,
          when: ::core::primitive::u32,
          index: ::core::primitive::u32,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, Cancel, DispatchError, root_mod::Event>
        {
          let call = Cancel { when, index };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn schedule_named(
          &self,
          id: ::std::vec::Vec<::core::primitive::u8>,
          when: ::core::primitive::u32,
          maybe_periodic: ::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
          priority: ::core::primitive::u8,
          call: runtime_types::frame_support::traits::schedule::MaybeHashed<
            runtime_types::lagoon_runtime::Call,
            ::subxt::sp_core::H256,
          >,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, ScheduleNamed, DispatchError, root_mod::Event>
        {
          let call = ScheduleNamed {
            id,
            when,
            maybe_periodic,
            priority,
            call: ::std::boxed::Box::new(call),
          };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn cancel_named(
          &self,
          id: ::std::vec::Vec<::core::primitive::u8>,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, CancelNamed, DispatchError, root_mod::Event>
        {
          let call = CancelNamed { id };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn schedule_after(
          &self,
          after: ::core::primitive::u32,
          maybe_periodic: ::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
          priority: ::core::primitive::u8,
          call: runtime_types::frame_support::traits::schedule::MaybeHashed<
            runtime_types::lagoon_runtime::Call,
            ::subxt::sp_core::H256,
          >,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, ScheduleAfter, DispatchError, root_mod::Event>
        {
          let call = ScheduleAfter {
            after,
            maybe_periodic,
            priority,
            call: ::std::boxed::Box::new(call),
          };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn schedule_named_after(
          &self,
          id: ::std::vec::Vec<::core::primitive::u8>,
          after: ::core::primitive::u32,
          maybe_periodic: ::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
          priority: ::core::primitive::u8,
          call: runtime_types::frame_support::traits::schedule::MaybeHashed<
            runtime_types::lagoon_runtime::Call,
            ::subxt::sp_core::H256,
          >,
        ) -> ::subxt::SubmittableExtrinsic<
          'a,
          T,
          X,
          ScheduleNamedAfter,
          DispatchError,
          root_mod::Event,
        > {
          let call = ScheduleNamedAfter {
            id,
            after,
            maybe_periodic,
            priority,
            call: ::std::boxed::Box::new(call),
          };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
      }
    }
    pub type Event = runtime_types::pallet_scheduler::pallet::Event;
    pub mod events {
      use super::runtime_types;
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Scheduled {
        pub when: ::core::primitive::u32,
        pub index: ::core::primitive::u32,
      }
      impl ::subxt::Event for Scheduled {
        const PALLET: &'static str = "Scheduler";
        const EVENT: &'static str = "Scheduled";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Canceled {
        pub when: ::core::primitive::u32,
        pub index: ::core::primitive::u32,
      }
      impl ::subxt::Event for Canceled {
        const PALLET: &'static str = "Scheduler";
        const EVENT: &'static str = "Canceled";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Dispatched {
        pub task: (::core::primitive::u32, ::core::primitive::u32),
        pub id: ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
        pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
      }
      impl ::subxt::Event for Dispatched {
        const PALLET: &'static str = "Scheduler";
        const EVENT: &'static str = "Dispatched";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct CallLookupFailed {
        pub task: (::core::primitive::u32, ::core::primitive::u32),
        pub id: ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
        pub error: runtime_types::frame_support::traits::schedule::LookupError,
      }
      impl ::subxt::Event for CallLookupFailed {
        const PALLET: &'static str = "Scheduler";
        const EVENT: &'static str = "CallLookupFailed";
      }
    }
    pub mod storage {
      use super::runtime_types;
      pub struct Agenda<'a>(pub &'a ::core::primitive::u32);
      impl ::subxt::StorageEntry for Agenda<'_> {
        const PALLET: &'static str = "Scheduler";
        const STORAGE: &'static str = "Agenda";
        type Value = ::std::vec::Vec<
          ::core::option::Option<
            runtime_types::pallet_scheduler::ScheduledV3<
              runtime_types::frame_support::traits::schedule::MaybeHashed<
                runtime_types::lagoon_runtime::Call,
                ::subxt::sp_core::H256,
              >,
              ::core::primitive::u32,
              runtime_types::lagoon_runtime::OriginCaller,
              ::subxt::sp_core::crypto::AccountId32,
            >,
          >,
        >;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
            &self.0,
            ::subxt::StorageHasher::Twox64Concat,
          )])
        }
      }
      pub struct Lookup<'a>(pub &'a [::core::primitive::u8]);
      impl ::subxt::StorageEntry for Lookup<'_> {
        const PALLET: &'static str = "Scheduler";
        const STORAGE: &'static str = "Lookup";
        type Value = (::core::primitive::u32, ::core::primitive::u32);
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
            &self.0,
            ::subxt::StorageHasher::Twox64Concat,
          )])
        }
      }
      pub struct StorageApi<'a, T: ::subxt::Config> {
        client: &'a ::subxt::Client<T>,
      }
      impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self { client }
        }
        pub async fn agenda(
          &self,
          _0: &::core::primitive::u32,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::std::vec::Vec<
            ::core::option::Option<
              runtime_types::pallet_scheduler::ScheduledV3<
                runtime_types::frame_support::traits::schedule::MaybeHashed<
                  runtime_types::lagoon_runtime::Call,
                  ::subxt::sp_core::H256,
                >,
                ::core::primitive::u32,
                runtime_types::lagoon_runtime::OriginCaller,
                ::subxt::sp_core::crypto::AccountId32,
              >,
            >,
          >,
          ::subxt::BasicError,
        > {
          let entry = Agenda(_0);
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn agenda_iter(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Agenda<'a>>, ::subxt::BasicError>
        {
          self.client.storage().iter(hash).await
        }
        pub async fn lookup(
          &self,
          _0: &[::core::primitive::u8],
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
          ::subxt::BasicError,
        > {
          let entry = Lookup(_0);
          self.client.storage().fetch(&entry, hash).await
        }
        pub async fn lookup_iter(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Lookup<'a>>, ::subxt::BasicError>
        {
          self.client.storage().iter(hash).await
        }
      }
    }
    pub mod constants {
      use super::runtime_types;
      pub struct ConstantsApi<'a, T: ::subxt::Config> {
        client: &'a ::subxt::Client<T>,
      }
      impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self { client }
        }
        pub fn maximum_weight(
          &self,
        ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError> {
          let pallet = self.client.metadata().pallet("Scheduler")?;
          let constant = pallet.constant("MaximumWeight")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn max_scheduled_per_block(
          &self,
        ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
          let pallet = self.client.metadata().pallet("Scheduler")?;
          let constant = pallet.constant("MaxScheduledPerBlock")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
      }
    }
  }
  pub mod proxy {
    use super::root_mod;
    use super::runtime_types;
    pub mod calls {
      use super::root_mod;
      use super::runtime_types;
      type DispatchError = runtime_types::sp_runtime::DispatchError;
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Proxy {
        pub real: ::subxt::sp_core::crypto::AccountId32,
        pub force_proxy_type:
          ::core::option::Option<runtime_types::lagoon_runtime::config::proxy::ProxyType>,
        pub call: ::std::boxed::Box<runtime_types::lagoon_runtime::Call>,
      }
      impl ::subxt::Call for Proxy {
        const PALLET: &'static str = "Proxy";
        const FUNCTION: &'static str = "proxy";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct AddProxy {
        pub delegate: ::subxt::sp_core::crypto::AccountId32,
        pub proxy_type: runtime_types::lagoon_runtime::config::proxy::ProxyType,
        pub delay: ::core::primitive::u32,
      }
      impl ::subxt::Call for AddProxy {
        const PALLET: &'static str = "Proxy";
        const FUNCTION: &'static str = "add_proxy";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct RemoveProxy {
        pub delegate: ::subxt::sp_core::crypto::AccountId32,
        pub proxy_type: runtime_types::lagoon_runtime::config::proxy::ProxyType,
        pub delay: ::core::primitive::u32,
      }
      impl ::subxt::Call for RemoveProxy {
        const PALLET: &'static str = "Proxy";
        const FUNCTION: &'static str = "remove_proxy";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct RemoveProxies;
      impl ::subxt::Call for RemoveProxies {
        const PALLET: &'static str = "Proxy";
        const FUNCTION: &'static str = "remove_proxies";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Anonymous {
        pub proxy_type: runtime_types::lagoon_runtime::config::proxy::ProxyType,
        pub delay: ::core::primitive::u32,
        pub index: ::core::primitive::u16,
      }
      impl ::subxt::Call for Anonymous {
        const PALLET: &'static str = "Proxy";
        const FUNCTION: &'static str = "anonymous";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct KillAnonymous {
        pub spawner: ::subxt::sp_core::crypto::AccountId32,
        pub proxy_type: runtime_types::lagoon_runtime::config::proxy::ProxyType,
        pub index: ::core::primitive::u16,
        #[codec(compact)]
        pub height: ::core::primitive::u32,
        #[codec(compact)]
        pub ext_index: ::core::primitive::u32,
      }
      impl ::subxt::Call for KillAnonymous {
        const PALLET: &'static str = "Proxy";
        const FUNCTION: &'static str = "kill_anonymous";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Announce {
        pub real: ::subxt::sp_core::crypto::AccountId32,
        pub call_hash: ::subxt::sp_core::H256,
      }
      impl ::subxt::Call for Announce {
        const PALLET: &'static str = "Proxy";
        const FUNCTION: &'static str = "announce";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct RemoveAnnouncement {
        pub real: ::subxt::sp_core::crypto::AccountId32,
        pub call_hash: ::subxt::sp_core::H256,
      }
      impl ::subxt::Call for RemoveAnnouncement {
        const PALLET: &'static str = "Proxy";
        const FUNCTION: &'static str = "remove_announcement";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct RejectAnnouncement {
        pub delegate: ::subxt::sp_core::crypto::AccountId32,
        pub call_hash: ::subxt::sp_core::H256,
      }
      impl ::subxt::Call for RejectAnnouncement {
        const PALLET: &'static str = "Proxy";
        const FUNCTION: &'static str = "reject_announcement";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct ProxyAnnounced {
        pub delegate: ::subxt::sp_core::crypto::AccountId32,
        pub real: ::subxt::sp_core::crypto::AccountId32,
        pub force_proxy_type:
          ::core::option::Option<runtime_types::lagoon_runtime::config::proxy::ProxyType>,
        pub call: ::std::boxed::Box<runtime_types::lagoon_runtime::Call>,
      }
      impl ::subxt::Call for ProxyAnnounced {
        const PALLET: &'static str = "Proxy";
        const FUNCTION: &'static str = "proxy_announced";
      }
      pub struct TransactionApi<'a, T: ::subxt::Config, X> {
        client: &'a ::subxt::Client<T>,
        marker: ::core::marker::PhantomData<X>,
      }
      impl<'a, T, X> TransactionApi<'a, T, X>
      where
        T: ::subxt::Config,
        X: ::subxt::extrinsic::ExtrinsicParams<T>,
      {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self {
            client,
            marker: ::core::marker::PhantomData,
          }
        }
        pub fn proxy(
          &self,
          real: ::subxt::sp_core::crypto::AccountId32,
          force_proxy_type: ::core::option::Option<
            runtime_types::lagoon_runtime::config::proxy::ProxyType,
          >,
          call: runtime_types::lagoon_runtime::Call,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, Proxy, DispatchError, root_mod::Event>
        {
          let call = Proxy {
            real,
            force_proxy_type,
            call: ::std::boxed::Box::new(call),
          };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn add_proxy(
          &self,
          delegate: ::subxt::sp_core::crypto::AccountId32,
          proxy_type: runtime_types::lagoon_runtime::config::proxy::ProxyType,
          delay: ::core::primitive::u32,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, AddProxy, DispatchError, root_mod::Event>
        {
          let call = AddProxy {
            delegate,
            proxy_type,
            delay,
          };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn remove_proxy(
          &self,
          delegate: ::subxt::sp_core::crypto::AccountId32,
          proxy_type: runtime_types::lagoon_runtime::config::proxy::ProxyType,
          delay: ::core::primitive::u32,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, RemoveProxy, DispatchError, root_mod::Event>
        {
          let call = RemoveProxy {
            delegate,
            proxy_type,
            delay,
          };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn remove_proxies(
          &self,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, RemoveProxies, DispatchError, root_mod::Event>
        {
          let call = RemoveProxies {};
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn anonymous(
          &self,
          proxy_type: runtime_types::lagoon_runtime::config::proxy::ProxyType,
          delay: ::core::primitive::u32,
          index: ::core::primitive::u16,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, Anonymous, DispatchError, root_mod::Event>
        {
          let call = Anonymous {
            proxy_type,
            delay,
            index,
          };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn kill_anonymous(
          &self,
          spawner: ::subxt::sp_core::crypto::AccountId32,
          proxy_type: runtime_types::lagoon_runtime::config::proxy::ProxyType,
          index: ::core::primitive::u16,
          height: ::core::primitive::u32,
          ext_index: ::core::primitive::u32,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, KillAnonymous, DispatchError, root_mod::Event>
        {
          let call = KillAnonymous {
            spawner,
            proxy_type,
            index,
            height,
            ext_index,
          };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn announce(
          &self,
          real: ::subxt::sp_core::crypto::AccountId32,
          call_hash: ::subxt::sp_core::H256,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, Announce, DispatchError, root_mod::Event>
        {
          let call = Announce { real, call_hash };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn remove_announcement(
          &self,
          real: ::subxt::sp_core::crypto::AccountId32,
          call_hash: ::subxt::sp_core::H256,
        ) -> ::subxt::SubmittableExtrinsic<
          'a,
          T,
          X,
          RemoveAnnouncement,
          DispatchError,
          root_mod::Event,
        > {
          let call = RemoveAnnouncement { real, call_hash };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn reject_announcement(
          &self,
          delegate: ::subxt::sp_core::crypto::AccountId32,
          call_hash: ::subxt::sp_core::H256,
        ) -> ::subxt::SubmittableExtrinsic<
          'a,
          T,
          X,
          RejectAnnouncement,
          DispatchError,
          root_mod::Event,
        > {
          let call = RejectAnnouncement {
            delegate,
            call_hash,
          };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn proxy_announced(
          &self,
          delegate: ::subxt::sp_core::crypto::AccountId32,
          real: ::subxt::sp_core::crypto::AccountId32,
          force_proxy_type: ::core::option::Option<
            runtime_types::lagoon_runtime::config::proxy::ProxyType,
          >,
          call: runtime_types::lagoon_runtime::Call,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, ProxyAnnounced, DispatchError, root_mod::Event>
        {
          let call = ProxyAnnounced {
            delegate,
            real,
            force_proxy_type,
            call: ::std::boxed::Box::new(call),
          };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
      }
    }
    pub type Event = runtime_types::pallet_proxy::pallet::Event;
    pub mod events {
      use super::runtime_types;
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct ProxyExecuted {
        pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
      }
      impl ::subxt::Event for ProxyExecuted {
        const PALLET: &'static str = "Proxy";
        const EVENT: &'static str = "ProxyExecuted";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct AnonymousCreated {
        pub anonymous: ::subxt::sp_core::crypto::AccountId32,
        pub who: ::subxt::sp_core::crypto::AccountId32,
        pub proxy_type: runtime_types::lagoon_runtime::config::proxy::ProxyType,
        pub disambiguation_index: ::core::primitive::u16,
      }
      impl ::subxt::Event for AnonymousCreated {
        const PALLET: &'static str = "Proxy";
        const EVENT: &'static str = "AnonymousCreated";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Announced {
        pub real: ::subxt::sp_core::crypto::AccountId32,
        pub proxy: ::subxt::sp_core::crypto::AccountId32,
        pub call_hash: ::subxt::sp_core::H256,
      }
      impl ::subxt::Event for Announced {
        const PALLET: &'static str = "Proxy";
        const EVENT: &'static str = "Announced";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct ProxyAdded {
        pub delegator: ::subxt::sp_core::crypto::AccountId32,
        pub delegatee: ::subxt::sp_core::crypto::AccountId32,
        pub proxy_type: runtime_types::lagoon_runtime::config::proxy::ProxyType,
        pub delay: ::core::primitive::u32,
      }
      impl ::subxt::Event for ProxyAdded {
        const PALLET: &'static str = "Proxy";
        const EVENT: &'static str = "ProxyAdded";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct ProxyRemoved {
        pub delegator: ::subxt::sp_core::crypto::AccountId32,
        pub delegatee: ::subxt::sp_core::crypto::AccountId32,
        pub proxy_type: runtime_types::lagoon_runtime::config::proxy::ProxyType,
        pub delay: ::core::primitive::u32,
      }
      impl ::subxt::Event for ProxyRemoved {
        const PALLET: &'static str = "Proxy";
        const EVENT: &'static str = "ProxyRemoved";
      }
    }
    pub mod storage {
      use super::runtime_types;
      pub struct Proxies<'a>(pub &'a ::subxt::sp_core::crypto::AccountId32);
      impl ::subxt::StorageEntry for Proxies<'_> {
        const PALLET: &'static str = "Proxy";
        const STORAGE: &'static str = "Proxies";
        type Value = (
          runtime_types::frame_support::storage::bounded_vec::BoundedVec<
            runtime_types::pallet_proxy::ProxyDefinition<
              ::subxt::sp_core::crypto::AccountId32,
              runtime_types::lagoon_runtime::config::proxy::ProxyType,
              ::core::primitive::u32,
            >,
          >,
          ::core::primitive::u128,
        );
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
            &self.0,
            ::subxt::StorageHasher::Twox64Concat,
          )])
        }
      }
      pub struct Announcements<'a>(pub &'a ::subxt::sp_core::crypto::AccountId32);
      impl ::subxt::StorageEntry for Announcements<'_> {
        const PALLET: &'static str = "Proxy";
        const STORAGE: &'static str = "Announcements";
        type Value = (
          runtime_types::frame_support::storage::bounded_vec::BoundedVec<
            runtime_types::pallet_proxy::Announcement<
              ::subxt::sp_core::crypto::AccountId32,
              ::subxt::sp_core::H256,
              ::core::primitive::u32,
            >,
          >,
          ::core::primitive::u128,
        );
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
            &self.0,
            ::subxt::StorageHasher::Twox64Concat,
          )])
        }
      }
      pub struct StorageApi<'a, T: ::subxt::Config> {
        client: &'a ::subxt::Client<T>,
      }
      impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self { client }
        }
        pub async fn proxies(
          &self,
          _0: &::subxt::sp_core::crypto::AccountId32,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          (
            runtime_types::frame_support::storage::bounded_vec::BoundedVec<
              runtime_types::pallet_proxy::ProxyDefinition<
                ::subxt::sp_core::crypto::AccountId32,
                runtime_types::lagoon_runtime::config::proxy::ProxyType,
                ::core::primitive::u32,
              >,
            >,
            ::core::primitive::u128,
          ),
          ::subxt::BasicError,
        > {
          let entry = Proxies(_0);
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn proxies_iter(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Proxies<'a>>, ::subxt::BasicError>
        {
          self.client.storage().iter(hash).await
        }
        pub async fn announcements(
          &self,
          _0: &::subxt::sp_core::crypto::AccountId32,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          (
            runtime_types::frame_support::storage::bounded_vec::BoundedVec<
              runtime_types::pallet_proxy::Announcement<
                ::subxt::sp_core::crypto::AccountId32,
                ::subxt::sp_core::H256,
                ::core::primitive::u32,
              >,
            >,
            ::core::primitive::u128,
          ),
          ::subxt::BasicError,
        > {
          let entry = Announcements(_0);
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn announcements_iter(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Announcements<'a>>, ::subxt::BasicError>
        {
          self.client.storage().iter(hash).await
        }
      }
    }
    pub mod constants {
      use super::runtime_types;
      pub struct ConstantsApi<'a, T: ::subxt::Config> {
        client: &'a ::subxt::Client<T>,
      }
      impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self { client }
        }
        pub fn proxy_deposit_base(
          &self,
        ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError> {
          let pallet = self.client.metadata().pallet("Proxy")?;
          let constant = pallet.constant("ProxyDepositBase")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn proxy_deposit_factor(
          &self,
        ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError> {
          let pallet = self.client.metadata().pallet("Proxy")?;
          let constant = pallet.constant("ProxyDepositFactor")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn max_proxies(
          &self,
        ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
          let pallet = self.client.metadata().pallet("Proxy")?;
          let constant = pallet.constant("MaxProxies")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn max_pending(
          &self,
        ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
          let pallet = self.client.metadata().pallet("Proxy")?;
          let constant = pallet.constant("MaxPending")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn announcement_deposit_base(
          &self,
        ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError> {
          let pallet = self.client.metadata().pallet("Proxy")?;
          let constant = pallet.constant("AnnouncementDepositBase")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn announcement_deposit_factor(
          &self,
        ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError> {
          let pallet = self.client.metadata().pallet("Proxy")?;
          let constant = pallet.constant("AnnouncementDepositFactor")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
      }
    }
  }
  pub mod multisig {
    use super::root_mod;
    use super::runtime_types;
    pub mod calls {
      use super::root_mod;
      use super::runtime_types;
      type DispatchError = runtime_types::sp_runtime::DispatchError;
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct AsMultiThreshold1 {
        pub other_signatories: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
        pub call: ::std::boxed::Box<runtime_types::lagoon_runtime::Call>,
      }
      impl ::subxt::Call for AsMultiThreshold1 {
        const PALLET: &'static str = "Multisig";
        const FUNCTION: &'static str = "as_multi_threshold1";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct AsMulti {
        pub threshold: ::core::primitive::u16,
        pub other_signatories: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
        pub maybe_timepoint:
          ::core::option::Option<runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>>,
        pub call: ::subxt::WrapperKeepOpaque<runtime_types::lagoon_runtime::Call>,
        pub store_call: ::core::primitive::bool,
        pub max_weight: ::core::primitive::u64,
      }
      impl ::subxt::Call for AsMulti {
        const PALLET: &'static str = "Multisig";
        const FUNCTION: &'static str = "as_multi";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct ApproveAsMulti {
        pub threshold: ::core::primitive::u16,
        pub other_signatories: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
        pub maybe_timepoint:
          ::core::option::Option<runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>>,
        pub call_hash: [::core::primitive::u8; 32usize],
        pub max_weight: ::core::primitive::u64,
      }
      impl ::subxt::Call for ApproveAsMulti {
        const PALLET: &'static str = "Multisig";
        const FUNCTION: &'static str = "approve_as_multi";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct CancelAsMulti {
        pub threshold: ::core::primitive::u16,
        pub other_signatories: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
        pub timepoint: runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
        pub call_hash: [::core::primitive::u8; 32usize],
      }
      impl ::subxt::Call for CancelAsMulti {
        const PALLET: &'static str = "Multisig";
        const FUNCTION: &'static str = "cancel_as_multi";
      }
      pub struct TransactionApi<'a, T: ::subxt::Config, X> {
        client: &'a ::subxt::Client<T>,
        marker: ::core::marker::PhantomData<X>,
      }
      impl<'a, T, X> TransactionApi<'a, T, X>
      where
        T: ::subxt::Config,
        X: ::subxt::extrinsic::ExtrinsicParams<T>,
      {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self {
            client,
            marker: ::core::marker::PhantomData,
          }
        }
        pub fn as_multi_threshold1(
          &self,
          other_signatories: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
          call: runtime_types::lagoon_runtime::Call,
        ) -> ::subxt::SubmittableExtrinsic<
          'a,
          T,
          X,
          AsMultiThreshold1,
          DispatchError,
          root_mod::Event,
        > {
          let call = AsMultiThreshold1 {
            other_signatories,
            call: ::std::boxed::Box::new(call),
          };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn as_multi(
          &self,
          threshold: ::core::primitive::u16,
          other_signatories: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
          maybe_timepoint: ::core::option::Option<
            runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
          >,
          call: ::subxt::WrapperKeepOpaque<runtime_types::lagoon_runtime::Call>,
          store_call: ::core::primitive::bool,
          max_weight: ::core::primitive::u64,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, AsMulti, DispatchError, root_mod::Event>
        {
          let call = AsMulti {
            threshold,
            other_signatories,
            maybe_timepoint,
            call,
            store_call,
            max_weight,
          };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn approve_as_multi(
          &self,
          threshold: ::core::primitive::u16,
          other_signatories: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
          maybe_timepoint: ::core::option::Option<
            runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
          >,
          call_hash: [::core::primitive::u8; 32usize],
          max_weight: ::core::primitive::u64,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, ApproveAsMulti, DispatchError, root_mod::Event>
        {
          let call = ApproveAsMulti {
            threshold,
            other_signatories,
            maybe_timepoint,
            call_hash,
            max_weight,
          };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn cancel_as_multi(
          &self,
          threshold: ::core::primitive::u16,
          other_signatories: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
          timepoint: runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
          call_hash: [::core::primitive::u8; 32usize],
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, CancelAsMulti, DispatchError, root_mod::Event>
        {
          let call = CancelAsMulti {
            threshold,
            other_signatories,
            timepoint,
            call_hash,
          };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
      }
    }
    pub type Event = runtime_types::pallet_multisig::pallet::Event;
    pub mod events {
      use super::runtime_types;
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct NewMultisig {
        pub approving: ::subxt::sp_core::crypto::AccountId32,
        pub multisig: ::subxt::sp_core::crypto::AccountId32,
        pub call_hash: [::core::primitive::u8; 32usize],
      }
      impl ::subxt::Event for NewMultisig {
        const PALLET: &'static str = "Multisig";
        const EVENT: &'static str = "NewMultisig";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct MultisigApproval {
        pub approving: ::subxt::sp_core::crypto::AccountId32,
        pub timepoint: runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
        pub multisig: ::subxt::sp_core::crypto::AccountId32,
        pub call_hash: [::core::primitive::u8; 32usize],
      }
      impl ::subxt::Event for MultisigApproval {
        const PALLET: &'static str = "Multisig";
        const EVENT: &'static str = "MultisigApproval";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct MultisigExecuted {
        pub approving: ::subxt::sp_core::crypto::AccountId32,
        pub timepoint: runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
        pub multisig: ::subxt::sp_core::crypto::AccountId32,
        pub call_hash: [::core::primitive::u8; 32usize],
        pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
      }
      impl ::subxt::Event for MultisigExecuted {
        const PALLET: &'static str = "Multisig";
        const EVENT: &'static str = "MultisigExecuted";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct MultisigCancelled {
        pub cancelling: ::subxt::sp_core::crypto::AccountId32,
        pub timepoint: runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
        pub multisig: ::subxt::sp_core::crypto::AccountId32,
        pub call_hash: [::core::primitive::u8; 32usize],
      }
      impl ::subxt::Event for MultisigCancelled {
        const PALLET: &'static str = "Multisig";
        const EVENT: &'static str = "MultisigCancelled";
      }
    }
    pub mod storage {
      use super::runtime_types;
      pub struct Multisigs<'a>(
        pub &'a ::subxt::sp_core::crypto::AccountId32,
        pub &'a [::core::primitive::u8; 32usize],
      );
      impl ::subxt::StorageEntry for Multisigs<'_> {
        const PALLET: &'static str = "Multisig";
        const STORAGE: &'static str = "Multisigs";
        type Value = runtime_types::pallet_multisig::Multisig<
          ::core::primitive::u32,
          ::core::primitive::u128,
          ::subxt::sp_core::crypto::AccountId32,
        >;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Map(vec![
            ::subxt::StorageMapKey::new(&self.0, ::subxt::StorageHasher::Twox64Concat),
            ::subxt::StorageMapKey::new(&self.1, ::subxt::StorageHasher::Blake2_128Concat),
          ])
        }
      }
      pub struct Calls<'a>(pub &'a [::core::primitive::u8; 32usize]);
      impl ::subxt::StorageEntry for Calls<'_> {
        const PALLET: &'static str = "Multisig";
        const STORAGE: &'static str = "Calls";
        type Value = (
          ::subxt::WrapperKeepOpaque<runtime_types::lagoon_runtime::Call>,
          ::subxt::sp_core::crypto::AccountId32,
          ::core::primitive::u128,
        );
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
            &self.0,
            ::subxt::StorageHasher::Identity,
          )])
        }
      }
      pub struct StorageApi<'a, T: ::subxt::Config> {
        client: &'a ::subxt::Client<T>,
      }
      impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self { client }
        }
        pub async fn multisigs(
          &self,
          _0: &::subxt::sp_core::crypto::AccountId32,
          _1: &[::core::primitive::u8; 32usize],
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::core::option::Option<
            runtime_types::pallet_multisig::Multisig<
              ::core::primitive::u32,
              ::core::primitive::u128,
              ::subxt::sp_core::crypto::AccountId32,
            >,
          >,
          ::subxt::BasicError,
        > {
          let entry = Multisigs(_0, _1);
          self.client.storage().fetch(&entry, hash).await
        }
        pub async fn multisigs_iter(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Multisigs<'a>>, ::subxt::BasicError>
        {
          self.client.storage().iter(hash).await
        }
        pub async fn calls(
          &self,
          _0: &[::core::primitive::u8; 32usize],
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::core::option::Option<(
            ::subxt::WrapperKeepOpaque<runtime_types::lagoon_runtime::Call>,
            ::subxt::sp_core::crypto::AccountId32,
            ::core::primitive::u128,
          )>,
          ::subxt::BasicError,
        > {
          let entry = Calls(_0);
          self.client.storage().fetch(&entry, hash).await
        }
        pub async fn calls_iter(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Calls<'a>>, ::subxt::BasicError>
        {
          self.client.storage().iter(hash).await
        }
      }
    }
    pub mod constants {
      use super::runtime_types;
      pub struct ConstantsApi<'a, T: ::subxt::Config> {
        client: &'a ::subxt::Client<T>,
      }
      impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self { client }
        }
        pub fn deposit_base(
          &self,
        ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError> {
          let pallet = self.client.metadata().pallet("Multisig")?;
          let constant = pallet.constant("DepositBase")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn deposit_factor(
          &self,
        ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError> {
          let pallet = self.client.metadata().pallet("Multisig")?;
          let constant = pallet.constant("DepositFactor")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn max_signatories(
          &self,
        ) -> ::core::result::Result<::core::primitive::u16, ::subxt::BasicError> {
          let pallet = self.client.metadata().pallet("Multisig")?;
          let constant = pallet.constant("MaxSignatories")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
      }
    }
  }
  pub mod bounties {
    use super::root_mod;
    use super::runtime_types;
    pub mod calls {
      use super::root_mod;
      use super::runtime_types;
      type DispatchError = runtime_types::sp_runtime::DispatchError;
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct ProposeBounty {
        #[codec(compact)]
        pub value: ::core::primitive::u128,
        pub description: ::std::vec::Vec<::core::primitive::u8>,
      }
      impl ::subxt::Call for ProposeBounty {
        const PALLET: &'static str = "Bounties";
        const FUNCTION: &'static str = "propose_bounty";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct ApproveBounty {
        #[codec(compact)]
        pub bounty_id: ::core::primitive::u32,
      }
      impl ::subxt::Call for ApproveBounty {
        const PALLET: &'static str = "Bounties";
        const FUNCTION: &'static str = "approve_bounty";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct ProposeCurator {
        #[codec(compact)]
        pub bounty_id: ::core::primitive::u32,
        pub curator: ::subxt::sp_runtime::MultiAddress<
          ::subxt::sp_core::crypto::AccountId32,
          ::core::primitive::u32,
        >,
        #[codec(compact)]
        pub fee: ::core::primitive::u128,
      }
      impl ::subxt::Call for ProposeCurator {
        const PALLET: &'static str = "Bounties";
        const FUNCTION: &'static str = "propose_curator";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct UnassignCurator {
        #[codec(compact)]
        pub bounty_id: ::core::primitive::u32,
      }
      impl ::subxt::Call for UnassignCurator {
        const PALLET: &'static str = "Bounties";
        const FUNCTION: &'static str = "unassign_curator";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct AcceptCurator {
        #[codec(compact)]
        pub bounty_id: ::core::primitive::u32,
      }
      impl ::subxt::Call for AcceptCurator {
        const PALLET: &'static str = "Bounties";
        const FUNCTION: &'static str = "accept_curator";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct AwardBounty {
        #[codec(compact)]
        pub bounty_id: ::core::primitive::u32,
        pub beneficiary: ::subxt::sp_runtime::MultiAddress<
          ::subxt::sp_core::crypto::AccountId32,
          ::core::primitive::u32,
        >,
      }
      impl ::subxt::Call for AwardBounty {
        const PALLET: &'static str = "Bounties";
        const FUNCTION: &'static str = "award_bounty";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct ClaimBounty {
        #[codec(compact)]
        pub bounty_id: ::core::primitive::u32,
      }
      impl ::subxt::Call for ClaimBounty {
        const PALLET: &'static str = "Bounties";
        const FUNCTION: &'static str = "claim_bounty";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct CloseBounty {
        #[codec(compact)]
        pub bounty_id: ::core::primitive::u32,
      }
      impl ::subxt::Call for CloseBounty {
        const PALLET: &'static str = "Bounties";
        const FUNCTION: &'static str = "close_bounty";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct ExtendBountyExpiry {
        #[codec(compact)]
        pub bounty_id: ::core::primitive::u32,
        pub remark: ::std::vec::Vec<::core::primitive::u8>,
      }
      impl ::subxt::Call for ExtendBountyExpiry {
        const PALLET: &'static str = "Bounties";
        const FUNCTION: &'static str = "extend_bounty_expiry";
      }
      pub struct TransactionApi<'a, T: ::subxt::Config, X> {
        client: &'a ::subxt::Client<T>,
        marker: ::core::marker::PhantomData<X>,
      }
      impl<'a, T, X> TransactionApi<'a, T, X>
      where
        T: ::subxt::Config,
        X: ::subxt::extrinsic::ExtrinsicParams<T>,
      {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self {
            client,
            marker: ::core::marker::PhantomData,
          }
        }
        pub fn propose_bounty(
          &self,
          value: ::core::primitive::u128,
          description: ::std::vec::Vec<::core::primitive::u8>,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, ProposeBounty, DispatchError, root_mod::Event>
        {
          let call = ProposeBounty { value, description };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn approve_bounty(
          &self,
          bounty_id: ::core::primitive::u32,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, ApproveBounty, DispatchError, root_mod::Event>
        {
          let call = ApproveBounty { bounty_id };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn propose_curator(
          &self,
          bounty_id: ::core::primitive::u32,
          curator: ::subxt::sp_runtime::MultiAddress<
            ::subxt::sp_core::crypto::AccountId32,
            ::core::primitive::u32,
          >,
          fee: ::core::primitive::u128,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, ProposeCurator, DispatchError, root_mod::Event>
        {
          let call = ProposeCurator {
            bounty_id,
            curator,
            fee,
          };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn unassign_curator(
          &self,
          bounty_id: ::core::primitive::u32,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, UnassignCurator, DispatchError, root_mod::Event>
        {
          let call = UnassignCurator { bounty_id };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn accept_curator(
          &self,
          bounty_id: ::core::primitive::u32,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, AcceptCurator, DispatchError, root_mod::Event>
        {
          let call = AcceptCurator { bounty_id };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn award_bounty(
          &self,
          bounty_id: ::core::primitive::u32,
          beneficiary: ::subxt::sp_runtime::MultiAddress<
            ::subxt::sp_core::crypto::AccountId32,
            ::core::primitive::u32,
          >,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, AwardBounty, DispatchError, root_mod::Event>
        {
          let call = AwardBounty {
            bounty_id,
            beneficiary,
          };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn claim_bounty(
          &self,
          bounty_id: ::core::primitive::u32,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, ClaimBounty, DispatchError, root_mod::Event>
        {
          let call = ClaimBounty { bounty_id };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn close_bounty(
          &self,
          bounty_id: ::core::primitive::u32,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, CloseBounty, DispatchError, root_mod::Event>
        {
          let call = CloseBounty { bounty_id };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn extend_bounty_expiry(
          &self,
          bounty_id: ::core::primitive::u32,
          remark: ::std::vec::Vec<::core::primitive::u8>,
        ) -> ::subxt::SubmittableExtrinsic<
          'a,
          T,
          X,
          ExtendBountyExpiry,
          DispatchError,
          root_mod::Event,
        > {
          let call = ExtendBountyExpiry { bounty_id, remark };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
      }
    }
    pub type Event = runtime_types::pallet_bounties::pallet::Event;
    pub mod events {
      use super::runtime_types;
      #[derive(
        :: subxt :: codec :: Encode,
        :: subxt :: codec :: Decode,
        Debug,
        :: subxt :: codec :: CompactAs,
      )]
      pub struct BountyProposed {
        pub index: ::core::primitive::u32,
      }
      impl ::subxt::Event for BountyProposed {
        const PALLET: &'static str = "Bounties";
        const EVENT: &'static str = "BountyProposed";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct BountyRejected {
        pub index: ::core::primitive::u32,
        pub bond: ::core::primitive::u128,
      }
      impl ::subxt::Event for BountyRejected {
        const PALLET: &'static str = "Bounties";
        const EVENT: &'static str = "BountyRejected";
      }
      #[derive(
        :: subxt :: codec :: Encode,
        :: subxt :: codec :: Decode,
        Debug,
        :: subxt :: codec :: CompactAs,
      )]
      pub struct BountyBecameActive {
        pub index: ::core::primitive::u32,
      }
      impl ::subxt::Event for BountyBecameActive {
        const PALLET: &'static str = "Bounties";
        const EVENT: &'static str = "BountyBecameActive";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct BountyAwarded {
        pub index: ::core::primitive::u32,
        pub beneficiary: ::subxt::sp_core::crypto::AccountId32,
      }
      impl ::subxt::Event for BountyAwarded {
        const PALLET: &'static str = "Bounties";
        const EVENT: &'static str = "BountyAwarded";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct BountyClaimed {
        pub index: ::core::primitive::u32,
        pub payout: ::core::primitive::u128,
        pub beneficiary: ::subxt::sp_core::crypto::AccountId32,
      }
      impl ::subxt::Event for BountyClaimed {
        const PALLET: &'static str = "Bounties";
        const EVENT: &'static str = "BountyClaimed";
      }
      #[derive(
        :: subxt :: codec :: Encode,
        :: subxt :: codec :: Decode,
        Debug,
        :: subxt :: codec :: CompactAs,
      )]
      pub struct BountyCanceled {
        pub index: ::core::primitive::u32,
      }
      impl ::subxt::Event for BountyCanceled {
        const PALLET: &'static str = "Bounties";
        const EVENT: &'static str = "BountyCanceled";
      }
      #[derive(
        :: subxt :: codec :: Encode,
        :: subxt :: codec :: Decode,
        Debug,
        :: subxt :: codec :: CompactAs,
      )]
      pub struct BountyExtended {
        pub index: ::core::primitive::u32,
      }
      impl ::subxt::Event for BountyExtended {
        const PALLET: &'static str = "Bounties";
        const EVENT: &'static str = "BountyExtended";
      }
    }
    pub mod storage {
      use super::runtime_types;
      pub struct BountyCount;
      impl ::subxt::StorageEntry for BountyCount {
        const PALLET: &'static str = "Bounties";
        const STORAGE: &'static str = "BountyCount";
        type Value = ::core::primitive::u32;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct Bounties<'a>(pub &'a ::core::primitive::u32);
      impl ::subxt::StorageEntry for Bounties<'_> {
        const PALLET: &'static str = "Bounties";
        const STORAGE: &'static str = "Bounties";
        type Value = runtime_types::pallet_bounties::Bounty<
          ::subxt::sp_core::crypto::AccountId32,
          ::core::primitive::u128,
          ::core::primitive::u32,
        >;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
            &self.0,
            ::subxt::StorageHasher::Twox64Concat,
          )])
        }
      }
      pub struct BountyDescriptions<'a>(pub &'a ::core::primitive::u32);
      impl ::subxt::StorageEntry for BountyDescriptions<'_> {
        const PALLET: &'static str = "Bounties";
        const STORAGE: &'static str = "BountyDescriptions";
        type Value =
          runtime_types::frame_support::storage::bounded_vec::BoundedVec<::core::primitive::u8>;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
            &self.0,
            ::subxt::StorageHasher::Twox64Concat,
          )])
        }
      }
      pub struct BountyApprovals;
      impl ::subxt::StorageEntry for BountyApprovals {
        const PALLET: &'static str = "Bounties";
        const STORAGE: &'static str = "BountyApprovals";
        type Value =
          runtime_types::frame_support::storage::bounded_vec::BoundedVec<::core::primitive::u32>;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct StorageApi<'a, T: ::subxt::Config> {
        client: &'a ::subxt::Client<T>,
      }
      impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self { client }
        }
        pub async fn bounty_count(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
          let entry = BountyCount;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn bounties(
          &self,
          _0: &::core::primitive::u32,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::core::option::Option<
            runtime_types::pallet_bounties::Bounty<
              ::subxt::sp_core::crypto::AccountId32,
              ::core::primitive::u128,
              ::core::primitive::u32,
            >,
          >,
          ::subxt::BasicError,
        > {
          let entry = Bounties(_0);
          self.client.storage().fetch(&entry, hash).await
        }
        pub async fn bounties_iter(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Bounties<'a>>, ::subxt::BasicError>
        {
          self.client.storage().iter(hash).await
        }
        pub async fn bounty_descriptions(
          &self,
          _0: &::core::primitive::u32,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::core::option::Option<
            runtime_types::frame_support::storage::bounded_vec::BoundedVec<::core::primitive::u8>,
          >,
          ::subxt::BasicError,
        > {
          let entry = BountyDescriptions(_0);
          self.client.storage().fetch(&entry, hash).await
        }
        pub async fn bounty_descriptions_iter(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::subxt::KeyIter<'a, T, BountyDescriptions<'a>>,
          ::subxt::BasicError,
        > {
          self.client.storage().iter(hash).await
        }
        pub async fn bounty_approvals(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          runtime_types::frame_support::storage::bounded_vec::BoundedVec<::core::primitive::u32>,
          ::subxt::BasicError,
        > {
          let entry = BountyApprovals;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
      }
    }
    pub mod constants {
      use super::runtime_types;
      pub struct ConstantsApi<'a, T: ::subxt::Config> {
        client: &'a ::subxt::Client<T>,
      }
      impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self { client }
        }
        pub fn bounty_deposit_base(
          &self,
        ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError> {
          let pallet = self.client.metadata().pallet("Bounties")?;
          let constant = pallet.constant("BountyDepositBase")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn bounty_deposit_payout_delay(
          &self,
        ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
          let pallet = self.client.metadata().pallet("Bounties")?;
          let constant = pallet.constant("BountyDepositPayoutDelay")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn bounty_update_period(
          &self,
        ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
          let pallet = self.client.metadata().pallet("Bounties")?;
          let constant = pallet.constant("BountyUpdatePeriod")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn bounty_curator_deposit(
          &self,
        ) -> ::core::result::Result<
          runtime_types::sp_arithmetic::per_things::Permill,
          ::subxt::BasicError,
        > {
          let pallet = self.client.metadata().pallet("Bounties")?;
          let constant = pallet.constant("BountyCuratorDeposit")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn bounty_value_minimum(
          &self,
        ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError> {
          let pallet = self.client.metadata().pallet("Bounties")?;
          let constant = pallet.constant("BountyValueMinimum")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn data_deposit_per_byte(
          &self,
        ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError> {
          let pallet = self.client.metadata().pallet("Bounties")?;
          let constant = pallet.constant("DataDepositPerByte")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn maximum_reason_length(
          &self,
        ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
          let pallet = self.client.metadata().pallet("Bounties")?;
          let constant = pallet.constant("MaximumReasonLength")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
      }
    }
  }
  pub mod assets {
    use super::root_mod;
    use super::runtime_types;
    pub mod calls {
      use super::root_mod;
      use super::runtime_types;
      type DispatchError = runtime_types::sp_runtime::DispatchError;
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Create {
        #[codec(compact)]
        pub id: ::core::primitive::u32,
        pub admin: ::subxt::sp_runtime::MultiAddress<
          ::subxt::sp_core::crypto::AccountId32,
          ::core::primitive::u32,
        >,
        pub min_balance: ::core::primitive::u128,
      }
      impl ::subxt::Call for Create {
        const PALLET: &'static str = "Assets";
        const FUNCTION: &'static str = "create";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct ForceCreate {
        #[codec(compact)]
        pub id: ::core::primitive::u32,
        pub owner: ::subxt::sp_runtime::MultiAddress<
          ::subxt::sp_core::crypto::AccountId32,
          ::core::primitive::u32,
        >,
        pub is_sufficient: ::core::primitive::bool,
        #[codec(compact)]
        pub min_balance: ::core::primitive::u128,
      }
      impl ::subxt::Call for ForceCreate {
        const PALLET: &'static str = "Assets";
        const FUNCTION: &'static str = "force_create";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Destroy {
        #[codec(compact)]
        pub id: ::core::primitive::u32,
        pub witness: runtime_types::pallet_assets::types::DestroyWitness,
      }
      impl ::subxt::Call for Destroy {
        const PALLET: &'static str = "Assets";
        const FUNCTION: &'static str = "destroy";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Mint {
        #[codec(compact)]
        pub id: ::core::primitive::u32,
        pub beneficiary: ::subxt::sp_runtime::MultiAddress<
          ::subxt::sp_core::crypto::AccountId32,
          ::core::primitive::u32,
        >,
        #[codec(compact)]
        pub amount: ::core::primitive::u128,
      }
      impl ::subxt::Call for Mint {
        const PALLET: &'static str = "Assets";
        const FUNCTION: &'static str = "mint";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Burn {
        #[codec(compact)]
        pub id: ::core::primitive::u32,
        pub who: ::subxt::sp_runtime::MultiAddress<
          ::subxt::sp_core::crypto::AccountId32,
          ::core::primitive::u32,
        >,
        #[codec(compact)]
        pub amount: ::core::primitive::u128,
      }
      impl ::subxt::Call for Burn {
        const PALLET: &'static str = "Assets";
        const FUNCTION: &'static str = "burn";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Transfer {
        #[codec(compact)]
        pub id: ::core::primitive::u32,
        pub target: ::subxt::sp_runtime::MultiAddress<
          ::subxt::sp_core::crypto::AccountId32,
          ::core::primitive::u32,
        >,
        #[codec(compact)]
        pub amount: ::core::primitive::u128,
      }
      impl ::subxt::Call for Transfer {
        const PALLET: &'static str = "Assets";
        const FUNCTION: &'static str = "transfer";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct TransferKeepAlive {
        #[codec(compact)]
        pub id: ::core::primitive::u32,
        pub target: ::subxt::sp_runtime::MultiAddress<
          ::subxt::sp_core::crypto::AccountId32,
          ::core::primitive::u32,
        >,
        #[codec(compact)]
        pub amount: ::core::primitive::u128,
      }
      impl ::subxt::Call for TransferKeepAlive {
        const PALLET: &'static str = "Assets";
        const FUNCTION: &'static str = "transfer_keep_alive";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct ForceTransfer {
        #[codec(compact)]
        pub id: ::core::primitive::u32,
        pub source: ::subxt::sp_runtime::MultiAddress<
          ::subxt::sp_core::crypto::AccountId32,
          ::core::primitive::u32,
        >,
        pub dest: ::subxt::sp_runtime::MultiAddress<
          ::subxt::sp_core::crypto::AccountId32,
          ::core::primitive::u32,
        >,
        #[codec(compact)]
        pub amount: ::core::primitive::u128,
      }
      impl ::subxt::Call for ForceTransfer {
        const PALLET: &'static str = "Assets";
        const FUNCTION: &'static str = "force_transfer";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Freeze {
        #[codec(compact)]
        pub id: ::core::primitive::u32,
        pub who: ::subxt::sp_runtime::MultiAddress<
          ::subxt::sp_core::crypto::AccountId32,
          ::core::primitive::u32,
        >,
      }
      impl ::subxt::Call for Freeze {
        const PALLET: &'static str = "Assets";
        const FUNCTION: &'static str = "freeze";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Thaw {
        #[codec(compact)]
        pub id: ::core::primitive::u32,
        pub who: ::subxt::sp_runtime::MultiAddress<
          ::subxt::sp_core::crypto::AccountId32,
          ::core::primitive::u32,
        >,
      }
      impl ::subxt::Call for Thaw {
        const PALLET: &'static str = "Assets";
        const FUNCTION: &'static str = "thaw";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct FreezeAsset {
        #[codec(compact)]
        pub id: ::core::primitive::u32,
      }
      impl ::subxt::Call for FreezeAsset {
        const PALLET: &'static str = "Assets";
        const FUNCTION: &'static str = "freeze_asset";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct ThawAsset {
        #[codec(compact)]
        pub id: ::core::primitive::u32,
      }
      impl ::subxt::Call for ThawAsset {
        const PALLET: &'static str = "Assets";
        const FUNCTION: &'static str = "thaw_asset";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct TransferOwnership {
        #[codec(compact)]
        pub id: ::core::primitive::u32,
        pub owner: ::subxt::sp_runtime::MultiAddress<
          ::subxt::sp_core::crypto::AccountId32,
          ::core::primitive::u32,
        >,
      }
      impl ::subxt::Call for TransferOwnership {
        const PALLET: &'static str = "Assets";
        const FUNCTION: &'static str = "transfer_ownership";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct SetTeam {
        #[codec(compact)]
        pub id: ::core::primitive::u32,
        pub issuer: ::subxt::sp_runtime::MultiAddress<
          ::subxt::sp_core::crypto::AccountId32,
          ::core::primitive::u32,
        >,
        pub admin: ::subxt::sp_runtime::MultiAddress<
          ::subxt::sp_core::crypto::AccountId32,
          ::core::primitive::u32,
        >,
        pub freezer: ::subxt::sp_runtime::MultiAddress<
          ::subxt::sp_core::crypto::AccountId32,
          ::core::primitive::u32,
        >,
      }
      impl ::subxt::Call for SetTeam {
        const PALLET: &'static str = "Assets";
        const FUNCTION: &'static str = "set_team";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct SetMetadata {
        #[codec(compact)]
        pub id: ::core::primitive::u32,
        pub name: ::std::vec::Vec<::core::primitive::u8>,
        pub symbol: ::std::vec::Vec<::core::primitive::u8>,
        pub decimals: ::core::primitive::u8,
      }
      impl ::subxt::Call for SetMetadata {
        const PALLET: &'static str = "Assets";
        const FUNCTION: &'static str = "set_metadata";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct ClearMetadata {
        #[codec(compact)]
        pub id: ::core::primitive::u32,
      }
      impl ::subxt::Call for ClearMetadata {
        const PALLET: &'static str = "Assets";
        const FUNCTION: &'static str = "clear_metadata";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct ForceSetMetadata {
        #[codec(compact)]
        pub id: ::core::primitive::u32,
        pub name: ::std::vec::Vec<::core::primitive::u8>,
        pub symbol: ::std::vec::Vec<::core::primitive::u8>,
        pub decimals: ::core::primitive::u8,
        pub is_frozen: ::core::primitive::bool,
      }
      impl ::subxt::Call for ForceSetMetadata {
        const PALLET: &'static str = "Assets";
        const FUNCTION: &'static str = "force_set_metadata";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct ForceClearMetadata {
        #[codec(compact)]
        pub id: ::core::primitive::u32,
      }
      impl ::subxt::Call for ForceClearMetadata {
        const PALLET: &'static str = "Assets";
        const FUNCTION: &'static str = "force_clear_metadata";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct ForceAssetStatus {
        #[codec(compact)]
        pub id: ::core::primitive::u32,
        pub owner: ::subxt::sp_runtime::MultiAddress<
          ::subxt::sp_core::crypto::AccountId32,
          ::core::primitive::u32,
        >,
        pub issuer: ::subxt::sp_runtime::MultiAddress<
          ::subxt::sp_core::crypto::AccountId32,
          ::core::primitive::u32,
        >,
        pub admin: ::subxt::sp_runtime::MultiAddress<
          ::subxt::sp_core::crypto::AccountId32,
          ::core::primitive::u32,
        >,
        pub freezer: ::subxt::sp_runtime::MultiAddress<
          ::subxt::sp_core::crypto::AccountId32,
          ::core::primitive::u32,
        >,
        #[codec(compact)]
        pub min_balance: ::core::primitive::u128,
        pub is_sufficient: ::core::primitive::bool,
        pub is_frozen: ::core::primitive::bool,
      }
      impl ::subxt::Call for ForceAssetStatus {
        const PALLET: &'static str = "Assets";
        const FUNCTION: &'static str = "force_asset_status";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct ApproveTransfer {
        #[codec(compact)]
        pub id: ::core::primitive::u32,
        pub delegate: ::subxt::sp_runtime::MultiAddress<
          ::subxt::sp_core::crypto::AccountId32,
          ::core::primitive::u32,
        >,
        #[codec(compact)]
        pub amount: ::core::primitive::u128,
      }
      impl ::subxt::Call for ApproveTransfer {
        const PALLET: &'static str = "Assets";
        const FUNCTION: &'static str = "approve_transfer";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct CancelApproval {
        #[codec(compact)]
        pub id: ::core::primitive::u32,
        pub delegate: ::subxt::sp_runtime::MultiAddress<
          ::subxt::sp_core::crypto::AccountId32,
          ::core::primitive::u32,
        >,
      }
      impl ::subxt::Call for CancelApproval {
        const PALLET: &'static str = "Assets";
        const FUNCTION: &'static str = "cancel_approval";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct ForceCancelApproval {
        #[codec(compact)]
        pub id: ::core::primitive::u32,
        pub owner: ::subxt::sp_runtime::MultiAddress<
          ::subxt::sp_core::crypto::AccountId32,
          ::core::primitive::u32,
        >,
        pub delegate: ::subxt::sp_runtime::MultiAddress<
          ::subxt::sp_core::crypto::AccountId32,
          ::core::primitive::u32,
        >,
      }
      impl ::subxt::Call for ForceCancelApproval {
        const PALLET: &'static str = "Assets";
        const FUNCTION: &'static str = "force_cancel_approval";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct TransferApproved {
        #[codec(compact)]
        pub id: ::core::primitive::u32,
        pub owner: ::subxt::sp_runtime::MultiAddress<
          ::subxt::sp_core::crypto::AccountId32,
          ::core::primitive::u32,
        >,
        pub destination: ::subxt::sp_runtime::MultiAddress<
          ::subxt::sp_core::crypto::AccountId32,
          ::core::primitive::u32,
        >,
        #[codec(compact)]
        pub amount: ::core::primitive::u128,
      }
      impl ::subxt::Call for TransferApproved {
        const PALLET: &'static str = "Assets";
        const FUNCTION: &'static str = "transfer_approved";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Touch {
        #[codec(compact)]
        pub id: ::core::primitive::u32,
      }
      impl ::subxt::Call for Touch {
        const PALLET: &'static str = "Assets";
        const FUNCTION: &'static str = "touch";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Refund {
        #[codec(compact)]
        pub id: ::core::primitive::u32,
        pub allow_burn: ::core::primitive::bool,
      }
      impl ::subxt::Call for Refund {
        const PALLET: &'static str = "Assets";
        const FUNCTION: &'static str = "refund";
      }
      pub struct TransactionApi<'a, T: ::subxt::Config, X> {
        client: &'a ::subxt::Client<T>,
        marker: ::core::marker::PhantomData<X>,
      }
      impl<'a, T, X> TransactionApi<'a, T, X>
      where
        T: ::subxt::Config,
        X: ::subxt::extrinsic::ExtrinsicParams<T>,
      {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self {
            client,
            marker: ::core::marker::PhantomData,
          }
        }
        pub fn create(
          &self,
          id: ::core::primitive::u32,
          admin: ::subxt::sp_runtime::MultiAddress<
            ::subxt::sp_core::crypto::AccountId32,
            ::core::primitive::u32,
          >,
          min_balance: ::core::primitive::u128,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, Create, DispatchError, root_mod::Event>
        {
          let call = Create {
            id,
            admin,
            min_balance,
          };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn force_create(
          &self,
          id: ::core::primitive::u32,
          owner: ::subxt::sp_runtime::MultiAddress<
            ::subxt::sp_core::crypto::AccountId32,
            ::core::primitive::u32,
          >,
          is_sufficient: ::core::primitive::bool,
          min_balance: ::core::primitive::u128,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, ForceCreate, DispatchError, root_mod::Event>
        {
          let call = ForceCreate {
            id,
            owner,
            is_sufficient,
            min_balance,
          };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn destroy(
          &self,
          id: ::core::primitive::u32,
          witness: runtime_types::pallet_assets::types::DestroyWitness,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, Destroy, DispatchError, root_mod::Event>
        {
          let call = Destroy { id, witness };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn mint(
          &self,
          id: ::core::primitive::u32,
          beneficiary: ::subxt::sp_runtime::MultiAddress<
            ::subxt::sp_core::crypto::AccountId32,
            ::core::primitive::u32,
          >,
          amount: ::core::primitive::u128,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, Mint, DispatchError, root_mod::Event> {
          let call = Mint {
            id,
            beneficiary,
            amount,
          };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn burn(
          &self,
          id: ::core::primitive::u32,
          who: ::subxt::sp_runtime::MultiAddress<
            ::subxt::sp_core::crypto::AccountId32,
            ::core::primitive::u32,
          >,
          amount: ::core::primitive::u128,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, Burn, DispatchError, root_mod::Event> {
          let call = Burn { id, who, amount };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn transfer(
          &self,
          id: ::core::primitive::u32,
          target: ::subxt::sp_runtime::MultiAddress<
            ::subxt::sp_core::crypto::AccountId32,
            ::core::primitive::u32,
          >,
          amount: ::core::primitive::u128,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, Transfer, DispatchError, root_mod::Event>
        {
          let call = Transfer { id, target, amount };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn transfer_keep_alive(
          &self,
          id: ::core::primitive::u32,
          target: ::subxt::sp_runtime::MultiAddress<
            ::subxt::sp_core::crypto::AccountId32,
            ::core::primitive::u32,
          >,
          amount: ::core::primitive::u128,
        ) -> ::subxt::SubmittableExtrinsic<
          'a,
          T,
          X,
          TransferKeepAlive,
          DispatchError,
          root_mod::Event,
        > {
          let call = TransferKeepAlive { id, target, amount };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn force_transfer(
          &self,
          id: ::core::primitive::u32,
          source: ::subxt::sp_runtime::MultiAddress<
            ::subxt::sp_core::crypto::AccountId32,
            ::core::primitive::u32,
          >,
          dest: ::subxt::sp_runtime::MultiAddress<
            ::subxt::sp_core::crypto::AccountId32,
            ::core::primitive::u32,
          >,
          amount: ::core::primitive::u128,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, ForceTransfer, DispatchError, root_mod::Event>
        {
          let call = ForceTransfer {
            id,
            source,
            dest,
            amount,
          };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn freeze(
          &self,
          id: ::core::primitive::u32,
          who: ::subxt::sp_runtime::MultiAddress<
            ::subxt::sp_core::crypto::AccountId32,
            ::core::primitive::u32,
          >,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, Freeze, DispatchError, root_mod::Event>
        {
          let call = Freeze { id, who };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn thaw(
          &self,
          id: ::core::primitive::u32,
          who: ::subxt::sp_runtime::MultiAddress<
            ::subxt::sp_core::crypto::AccountId32,
            ::core::primitive::u32,
          >,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, Thaw, DispatchError, root_mod::Event> {
          let call = Thaw { id, who };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn freeze_asset(
          &self,
          id: ::core::primitive::u32,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, FreezeAsset, DispatchError, root_mod::Event>
        {
          let call = FreezeAsset { id };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn thaw_asset(
          &self,
          id: ::core::primitive::u32,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, ThawAsset, DispatchError, root_mod::Event>
        {
          let call = ThawAsset { id };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn transfer_ownership(
          &self,
          id: ::core::primitive::u32,
          owner: ::subxt::sp_runtime::MultiAddress<
            ::subxt::sp_core::crypto::AccountId32,
            ::core::primitive::u32,
          >,
        ) -> ::subxt::SubmittableExtrinsic<
          'a,
          T,
          X,
          TransferOwnership,
          DispatchError,
          root_mod::Event,
        > {
          let call = TransferOwnership { id, owner };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn set_team(
          &self,
          id: ::core::primitive::u32,
          issuer: ::subxt::sp_runtime::MultiAddress<
            ::subxt::sp_core::crypto::AccountId32,
            ::core::primitive::u32,
          >,
          admin: ::subxt::sp_runtime::MultiAddress<
            ::subxt::sp_core::crypto::AccountId32,
            ::core::primitive::u32,
          >,
          freezer: ::subxt::sp_runtime::MultiAddress<
            ::subxt::sp_core::crypto::AccountId32,
            ::core::primitive::u32,
          >,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, SetTeam, DispatchError, root_mod::Event>
        {
          let call = SetTeam {
            id,
            issuer,
            admin,
            freezer,
          };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn set_metadata(
          &self,
          id: ::core::primitive::u32,
          name: ::std::vec::Vec<::core::primitive::u8>,
          symbol: ::std::vec::Vec<::core::primitive::u8>,
          decimals: ::core::primitive::u8,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, SetMetadata, DispatchError, root_mod::Event>
        {
          let call = SetMetadata {
            id,
            name,
            symbol,
            decimals,
          };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn clear_metadata(
          &self,
          id: ::core::primitive::u32,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, ClearMetadata, DispatchError, root_mod::Event>
        {
          let call = ClearMetadata { id };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn force_set_metadata(
          &self,
          id: ::core::primitive::u32,
          name: ::std::vec::Vec<::core::primitive::u8>,
          symbol: ::std::vec::Vec<::core::primitive::u8>,
          decimals: ::core::primitive::u8,
          is_frozen: ::core::primitive::bool,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, ForceSetMetadata, DispatchError, root_mod::Event>
        {
          let call = ForceSetMetadata {
            id,
            name,
            symbol,
            decimals,
            is_frozen,
          };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn force_clear_metadata(
          &self,
          id: ::core::primitive::u32,
        ) -> ::subxt::SubmittableExtrinsic<
          'a,
          T,
          X,
          ForceClearMetadata,
          DispatchError,
          root_mod::Event,
        > {
          let call = ForceClearMetadata { id };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn force_asset_status(
          &self,
          id: ::core::primitive::u32,
          owner: ::subxt::sp_runtime::MultiAddress<
            ::subxt::sp_core::crypto::AccountId32,
            ::core::primitive::u32,
          >,
          issuer: ::subxt::sp_runtime::MultiAddress<
            ::subxt::sp_core::crypto::AccountId32,
            ::core::primitive::u32,
          >,
          admin: ::subxt::sp_runtime::MultiAddress<
            ::subxt::sp_core::crypto::AccountId32,
            ::core::primitive::u32,
          >,
          freezer: ::subxt::sp_runtime::MultiAddress<
            ::subxt::sp_core::crypto::AccountId32,
            ::core::primitive::u32,
          >,
          min_balance: ::core::primitive::u128,
          is_sufficient: ::core::primitive::bool,
          is_frozen: ::core::primitive::bool,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, ForceAssetStatus, DispatchError, root_mod::Event>
        {
          let call = ForceAssetStatus {
            id,
            owner,
            issuer,
            admin,
            freezer,
            min_balance,
            is_sufficient,
            is_frozen,
          };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn approve_transfer(
          &self,
          id: ::core::primitive::u32,
          delegate: ::subxt::sp_runtime::MultiAddress<
            ::subxt::sp_core::crypto::AccountId32,
            ::core::primitive::u32,
          >,
          amount: ::core::primitive::u128,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, ApproveTransfer, DispatchError, root_mod::Event>
        {
          let call = ApproveTransfer {
            id,
            delegate,
            amount,
          };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn cancel_approval(
          &self,
          id: ::core::primitive::u32,
          delegate: ::subxt::sp_runtime::MultiAddress<
            ::subxt::sp_core::crypto::AccountId32,
            ::core::primitive::u32,
          >,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, CancelApproval, DispatchError, root_mod::Event>
        {
          let call = CancelApproval { id, delegate };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn force_cancel_approval(
          &self,
          id: ::core::primitive::u32,
          owner: ::subxt::sp_runtime::MultiAddress<
            ::subxt::sp_core::crypto::AccountId32,
            ::core::primitive::u32,
          >,
          delegate: ::subxt::sp_runtime::MultiAddress<
            ::subxt::sp_core::crypto::AccountId32,
            ::core::primitive::u32,
          >,
        ) -> ::subxt::SubmittableExtrinsic<
          'a,
          T,
          X,
          ForceCancelApproval,
          DispatchError,
          root_mod::Event,
        > {
          let call = ForceCancelApproval {
            id,
            owner,
            delegate,
          };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn transfer_approved(
          &self,
          id: ::core::primitive::u32,
          owner: ::subxt::sp_runtime::MultiAddress<
            ::subxt::sp_core::crypto::AccountId32,
            ::core::primitive::u32,
          >,
          destination: ::subxt::sp_runtime::MultiAddress<
            ::subxt::sp_core::crypto::AccountId32,
            ::core::primitive::u32,
          >,
          amount: ::core::primitive::u128,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, TransferApproved, DispatchError, root_mod::Event>
        {
          let call = TransferApproved {
            id,
            owner,
            destination,
            amount,
          };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn touch(
          &self,
          id: ::core::primitive::u32,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, Touch, DispatchError, root_mod::Event>
        {
          let call = Touch { id };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn refund(
          &self,
          id: ::core::primitive::u32,
          allow_burn: ::core::primitive::bool,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, Refund, DispatchError, root_mod::Event>
        {
          let call = Refund { id, allow_burn };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
      }
    }
    pub type Event = runtime_types::pallet_assets::pallet::Event;
    pub mod events {
      use super::runtime_types;
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Created {
        pub asset_id: ::core::primitive::u32,
        pub creator: ::subxt::sp_core::crypto::AccountId32,
        pub owner: ::subxt::sp_core::crypto::AccountId32,
      }
      impl ::subxt::Event for Created {
        const PALLET: &'static str = "Assets";
        const EVENT: &'static str = "Created";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Issued {
        pub asset_id: ::core::primitive::u32,
        pub owner: ::subxt::sp_core::crypto::AccountId32,
        pub total_supply: ::core::primitive::u128,
      }
      impl ::subxt::Event for Issued {
        const PALLET: &'static str = "Assets";
        const EVENT: &'static str = "Issued";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Transferred {
        pub asset_id: ::core::primitive::u32,
        pub from: ::subxt::sp_core::crypto::AccountId32,
        pub to: ::subxt::sp_core::crypto::AccountId32,
        pub amount: ::core::primitive::u128,
      }
      impl ::subxt::Event for Transferred {
        const PALLET: &'static str = "Assets";
        const EVENT: &'static str = "Transferred";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Burned {
        pub asset_id: ::core::primitive::u32,
        pub owner: ::subxt::sp_core::crypto::AccountId32,
        pub balance: ::core::primitive::u128,
      }
      impl ::subxt::Event for Burned {
        const PALLET: &'static str = "Assets";
        const EVENT: &'static str = "Burned";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct TeamChanged {
        pub asset_id: ::core::primitive::u32,
        pub issuer: ::subxt::sp_core::crypto::AccountId32,
        pub admin: ::subxt::sp_core::crypto::AccountId32,
        pub freezer: ::subxt::sp_core::crypto::AccountId32,
      }
      impl ::subxt::Event for TeamChanged {
        const PALLET: &'static str = "Assets";
        const EVENT: &'static str = "TeamChanged";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct OwnerChanged {
        pub asset_id: ::core::primitive::u32,
        pub owner: ::subxt::sp_core::crypto::AccountId32,
      }
      impl ::subxt::Event for OwnerChanged {
        const PALLET: &'static str = "Assets";
        const EVENT: &'static str = "OwnerChanged";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Frozen {
        pub asset_id: ::core::primitive::u32,
        pub who: ::subxt::sp_core::crypto::AccountId32,
      }
      impl ::subxt::Event for Frozen {
        const PALLET: &'static str = "Assets";
        const EVENT: &'static str = "Frozen";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Thawed {
        pub asset_id: ::core::primitive::u32,
        pub who: ::subxt::sp_core::crypto::AccountId32,
      }
      impl ::subxt::Event for Thawed {
        const PALLET: &'static str = "Assets";
        const EVENT: &'static str = "Thawed";
      }
      #[derive(
        :: subxt :: codec :: Encode,
        :: subxt :: codec :: Decode,
        Debug,
        :: subxt :: codec :: CompactAs,
      )]
      pub struct AssetFrozen {
        pub asset_id: ::core::primitive::u32,
      }
      impl ::subxt::Event for AssetFrozen {
        const PALLET: &'static str = "Assets";
        const EVENT: &'static str = "AssetFrozen";
      }
      #[derive(
        :: subxt :: codec :: Encode,
        :: subxt :: codec :: Decode,
        Debug,
        :: subxt :: codec :: CompactAs,
      )]
      pub struct AssetThawed {
        pub asset_id: ::core::primitive::u32,
      }
      impl ::subxt::Event for AssetThawed {
        const PALLET: &'static str = "Assets";
        const EVENT: &'static str = "AssetThawed";
      }
      #[derive(
        :: subxt :: codec :: Encode,
        :: subxt :: codec :: Decode,
        Debug,
        :: subxt :: codec :: CompactAs,
      )]
      pub struct Destroyed {
        pub asset_id: ::core::primitive::u32,
      }
      impl ::subxt::Event for Destroyed {
        const PALLET: &'static str = "Assets";
        const EVENT: &'static str = "Destroyed";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct ForceCreated {
        pub asset_id: ::core::primitive::u32,
        pub owner: ::subxt::sp_core::crypto::AccountId32,
      }
      impl ::subxt::Event for ForceCreated {
        const PALLET: &'static str = "Assets";
        const EVENT: &'static str = "ForceCreated";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct MetadataSet {
        pub asset_id: ::core::primitive::u32,
        pub name: ::std::vec::Vec<::core::primitive::u8>,
        pub symbol: ::std::vec::Vec<::core::primitive::u8>,
        pub decimals: ::core::primitive::u8,
        pub is_frozen: ::core::primitive::bool,
      }
      impl ::subxt::Event for MetadataSet {
        const PALLET: &'static str = "Assets";
        const EVENT: &'static str = "MetadataSet";
      }
      #[derive(
        :: subxt :: codec :: Encode,
        :: subxt :: codec :: Decode,
        Debug,
        :: subxt :: codec :: CompactAs,
      )]
      pub struct MetadataCleared {
        pub asset_id: ::core::primitive::u32,
      }
      impl ::subxt::Event for MetadataCleared {
        const PALLET: &'static str = "Assets";
        const EVENT: &'static str = "MetadataCleared";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct ApprovedTransfer {
        pub asset_id: ::core::primitive::u32,
        pub source: ::subxt::sp_core::crypto::AccountId32,
        pub delegate: ::subxt::sp_core::crypto::AccountId32,
        pub amount: ::core::primitive::u128,
      }
      impl ::subxt::Event for ApprovedTransfer {
        const PALLET: &'static str = "Assets";
        const EVENT: &'static str = "ApprovedTransfer";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct ApprovalCancelled {
        pub asset_id: ::core::primitive::u32,
        pub owner: ::subxt::sp_core::crypto::AccountId32,
        pub delegate: ::subxt::sp_core::crypto::AccountId32,
      }
      impl ::subxt::Event for ApprovalCancelled {
        const PALLET: &'static str = "Assets";
        const EVENT: &'static str = "ApprovalCancelled";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct TransferredApproved {
        pub asset_id: ::core::primitive::u32,
        pub owner: ::subxt::sp_core::crypto::AccountId32,
        pub delegate: ::subxt::sp_core::crypto::AccountId32,
        pub destination: ::subxt::sp_core::crypto::AccountId32,
        pub amount: ::core::primitive::u128,
      }
      impl ::subxt::Event for TransferredApproved {
        const PALLET: &'static str = "Assets";
        const EVENT: &'static str = "TransferredApproved";
      }
      #[derive(
        :: subxt :: codec :: Encode,
        :: subxt :: codec :: Decode,
        Debug,
        :: subxt :: codec :: CompactAs,
      )]
      pub struct AssetStatusChanged {
        pub asset_id: ::core::primitive::u32,
      }
      impl ::subxt::Event for AssetStatusChanged {
        const PALLET: &'static str = "Assets";
        const EVENT: &'static str = "AssetStatusChanged";
      }
    }
    pub mod storage {
      use super::runtime_types;
      pub struct Asset<'a>(pub &'a ::core::primitive::u32);
      impl ::subxt::StorageEntry for Asset<'_> {
        const PALLET: &'static str = "Assets";
        const STORAGE: &'static str = "Asset";
        type Value = runtime_types::pallet_assets::types::AssetDetails<
          ::core::primitive::u128,
          ::subxt::sp_core::crypto::AccountId32,
          ::core::primitive::u128,
        >;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
            &self.0,
            ::subxt::StorageHasher::Blake2_128Concat,
          )])
        }
      }
      pub struct Account<'a>(
        pub &'a ::subxt::sp_core::crypto::AccountId32,
        pub &'a ::core::primitive::u32,
      );
      impl ::subxt::StorageEntry for Account<'_> {
        const PALLET: &'static str = "Assets";
        const STORAGE: &'static str = "Account";
        type Value = runtime_types::pallet_assets::types::AssetAccount<
          ::core::primitive::u128,
          ::core::primitive::u128,
          (),
        >;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Map(vec![
            ::subxt::StorageMapKey::new(&self.0, ::subxt::StorageHasher::Blake2_128Concat),
            ::subxt::StorageMapKey::new(&self.1, ::subxt::StorageHasher::Blake2_128Concat),
          ])
        }
      }
      pub struct Approvals<'a>(
        pub &'a ::core::primitive::u32,
        pub &'a ::subxt::sp_core::crypto::AccountId32,
        pub &'a ::subxt::sp_core::crypto::AccountId32,
      );
      impl ::subxt::StorageEntry for Approvals<'_> {
        const PALLET: &'static str = "Assets";
        const STORAGE: &'static str = "Approvals";
        type Value = runtime_types::pallet_assets::types::Approval<
          ::core::primitive::u128,
          ::core::primitive::u128,
        >;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Map(vec![
            ::subxt::StorageMapKey::new(&self.0, ::subxt::StorageHasher::Blake2_128Concat),
            ::subxt::StorageMapKey::new(&self.1, ::subxt::StorageHasher::Blake2_128Concat),
            ::subxt::StorageMapKey::new(&self.2, ::subxt::StorageHasher::Blake2_128Concat),
          ])
        }
      }
      pub struct Metadata<'a>(pub &'a ::core::primitive::u32);
      impl ::subxt::StorageEntry for Metadata<'_> {
        const PALLET: &'static str = "Assets";
        const STORAGE: &'static str = "Metadata";
        type Value = runtime_types::pallet_assets::types::AssetMetadata<
          ::core::primitive::u128,
          runtime_types::frame_support::storage::bounded_vec::BoundedVec<::core::primitive::u8>,
        >;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
            &self.0,
            ::subxt::StorageHasher::Blake2_128Concat,
          )])
        }
      }
      pub struct StorageApi<'a, T: ::subxt::Config> {
        client: &'a ::subxt::Client<T>,
      }
      impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self { client }
        }
        pub async fn asset(
          &self,
          _0: &::core::primitive::u32,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::core::option::Option<
            runtime_types::pallet_assets::types::AssetDetails<
              ::core::primitive::u128,
              ::subxt::sp_core::crypto::AccountId32,
              ::core::primitive::u128,
            >,
          >,
          ::subxt::BasicError,
        > {
          let entry = Asset(_0);
          self.client.storage().fetch(&entry, hash).await
        }
        pub async fn asset_iter(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Asset<'a>>, ::subxt::BasicError>
        {
          self.client.storage().iter(hash).await
        }
        pub async fn account(
          &self,
          _0: &::subxt::sp_core::crypto::AccountId32,
          _1: &::core::primitive::u32,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::core::option::Option<
            runtime_types::pallet_assets::types::AssetAccount<
              ::core::primitive::u128,
              ::core::primitive::u128,
              (),
            >,
          >,
          ::subxt::BasicError,
        > {
          let entry = Account(_0, _1);
          self.client.storage().fetch(&entry, hash).await
        }
        pub async fn account_iter(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Account<'a>>, ::subxt::BasicError>
        {
          self.client.storage().iter(hash).await
        }
        pub async fn approvals(
          &self,
          _0: &::core::primitive::u32,
          _1: &::subxt::sp_core::crypto::AccountId32,
          _2: &::subxt::sp_core::crypto::AccountId32,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::core::option::Option<
            runtime_types::pallet_assets::types::Approval<
              ::core::primitive::u128,
              ::core::primitive::u128,
            >,
          >,
          ::subxt::BasicError,
        > {
          let entry = Approvals(_0, _1, _2);
          self.client.storage().fetch(&entry, hash).await
        }
        pub async fn approvals_iter(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Approvals<'a>>, ::subxt::BasicError>
        {
          self.client.storage().iter(hash).await
        }
        pub async fn metadata(
          &self,
          _0: &::core::primitive::u32,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          runtime_types::pallet_assets::types::AssetMetadata<
            ::core::primitive::u128,
            runtime_types::frame_support::storage::bounded_vec::BoundedVec<::core::primitive::u8>,
          >,
          ::subxt::BasicError,
        > {
          let entry = Metadata(_0);
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn metadata_iter(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Metadata<'a>>, ::subxt::BasicError>
        {
          self.client.storage().iter(hash).await
        }
      }
    }
    pub mod constants {
      use super::runtime_types;
      pub struct ConstantsApi<'a, T: ::subxt::Config> {
        client: &'a ::subxt::Client<T>,
      }
      impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self { client }
        }
        pub fn asset_deposit(
          &self,
        ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError> {
          let pallet = self.client.metadata().pallet("Assets")?;
          let constant = pallet.constant("AssetDeposit")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn asset_account_deposit(
          &self,
        ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError> {
          let pallet = self.client.metadata().pallet("Assets")?;
          let constant = pallet.constant("AssetAccountDeposit")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn metadata_deposit_base(
          &self,
        ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError> {
          let pallet = self.client.metadata().pallet("Assets")?;
          let constant = pallet.constant("MetadataDepositBase")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn metadata_deposit_per_byte(
          &self,
        ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError> {
          let pallet = self.client.metadata().pallet("Assets")?;
          let constant = pallet.constant("MetadataDepositPerByte")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn approval_deposit(
          &self,
        ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError> {
          let pallet = self.client.metadata().pallet("Assets")?;
          let constant = pallet.constant("ApprovalDeposit")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn string_limit(
          &self,
        ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
          let pallet = self.client.metadata().pallet("Assets")?;
          let constant = pallet.constant("StringLimit")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
      }
    }
  }
  pub mod bags_list {
    use super::root_mod;
    use super::runtime_types;
    pub mod calls {
      use super::root_mod;
      use super::runtime_types;
      type DispatchError = runtime_types::sp_runtime::DispatchError;
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Rebag {
        pub dislocated: ::subxt::sp_core::crypto::AccountId32,
      }
      impl ::subxt::Call for Rebag {
        const PALLET: &'static str = "BagsList";
        const FUNCTION: &'static str = "rebag";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct PutInFrontOf {
        pub lighter: ::subxt::sp_core::crypto::AccountId32,
      }
      impl ::subxt::Call for PutInFrontOf {
        const PALLET: &'static str = "BagsList";
        const FUNCTION: &'static str = "put_in_front_of";
      }
      pub struct TransactionApi<'a, T: ::subxt::Config, X> {
        client: &'a ::subxt::Client<T>,
        marker: ::core::marker::PhantomData<X>,
      }
      impl<'a, T, X> TransactionApi<'a, T, X>
      where
        T: ::subxt::Config,
        X: ::subxt::extrinsic::ExtrinsicParams<T>,
      {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self {
            client,
            marker: ::core::marker::PhantomData,
          }
        }
        pub fn rebag(
          &self,
          dislocated: ::subxt::sp_core::crypto::AccountId32,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, Rebag, DispatchError, root_mod::Event>
        {
          let call = Rebag { dislocated };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn put_in_front_of(
          &self,
          lighter: ::subxt::sp_core::crypto::AccountId32,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, PutInFrontOf, DispatchError, root_mod::Event>
        {
          let call = PutInFrontOf { lighter };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
      }
    }
    pub type Event = runtime_types::pallet_bags_list::pallet::Event;
    pub mod events {
      use super::runtime_types;
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Rebagged {
        pub who: ::subxt::sp_core::crypto::AccountId32,
        pub from: ::core::primitive::u64,
        pub to: ::core::primitive::u64,
      }
      impl ::subxt::Event for Rebagged {
        const PALLET: &'static str = "BagsList";
        const EVENT: &'static str = "Rebagged";
      }
    }
    pub mod storage {
      use super::runtime_types;
      pub struct ListNodes<'a>(pub &'a ::subxt::sp_core::crypto::AccountId32);
      impl ::subxt::StorageEntry for ListNodes<'_> {
        const PALLET: &'static str = "BagsList";
        const STORAGE: &'static str = "ListNodes";
        type Value = runtime_types::pallet_bags_list::list::Node;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
            &self.0,
            ::subxt::StorageHasher::Twox64Concat,
          )])
        }
      }
      pub struct CounterForListNodes;
      impl ::subxt::StorageEntry for CounterForListNodes {
        const PALLET: &'static str = "BagsList";
        const STORAGE: &'static str = "CounterForListNodes";
        type Value = ::core::primitive::u32;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct ListBags<'a>(pub &'a ::core::primitive::u64);
      impl ::subxt::StorageEntry for ListBags<'_> {
        const PALLET: &'static str = "BagsList";
        const STORAGE: &'static str = "ListBags";
        type Value = runtime_types::pallet_bags_list::list::Bag;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
            &self.0,
            ::subxt::StorageHasher::Twox64Concat,
          )])
        }
      }
      pub struct StorageApi<'a, T: ::subxt::Config> {
        client: &'a ::subxt::Client<T>,
      }
      impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self { client }
        }
        pub async fn list_nodes(
          &self,
          _0: &::subxt::sp_core::crypto::AccountId32,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::core::option::Option<runtime_types::pallet_bags_list::list::Node>,
          ::subxt::BasicError,
        > {
          let entry = ListNodes(_0);
          self.client.storage().fetch(&entry, hash).await
        }
        pub async fn list_nodes_iter(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::subxt::KeyIter<'a, T, ListNodes<'a>>, ::subxt::BasicError>
        {
          self.client.storage().iter(hash).await
        }
        pub async fn counter_for_list_nodes(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
          let entry = CounterForListNodes;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn list_bags(
          &self,
          _0: &::core::primitive::u64,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::core::option::Option<runtime_types::pallet_bags_list::list::Bag>,
          ::subxt::BasicError,
        > {
          let entry = ListBags(_0);
          self.client.storage().fetch(&entry, hash).await
        }
        pub async fn list_bags_iter(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::subxt::KeyIter<'a, T, ListBags<'a>>, ::subxt::BasicError>
        {
          self.client.storage().iter(hash).await
        }
      }
    }
    pub mod constants {
      use super::runtime_types;
      pub struct ConstantsApi<'a, T: ::subxt::Config> {
        client: &'a ::subxt::Client<T>,
      }
      impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self { client }
        }
        pub fn bag_thresholds(
          &self,
        ) -> ::core::result::Result<::std::vec::Vec<::core::primitive::u64>, ::subxt::BasicError>
        {
          let pallet = self.client.metadata().pallet("BagsList")?;
          let constant = pallet.constant("BagThresholds")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
      }
    }
  }
  pub mod preimage {
    use super::root_mod;
    use super::runtime_types;
    pub mod calls {
      use super::root_mod;
      use super::runtime_types;
      type DispatchError = runtime_types::sp_runtime::DispatchError;
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct NotePreimage {
        pub bytes: ::std::vec::Vec<::core::primitive::u8>,
      }
      impl ::subxt::Call for NotePreimage {
        const PALLET: &'static str = "Preimage";
        const FUNCTION: &'static str = "note_preimage";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct UnnotePreimage {
        pub hash: ::subxt::sp_core::H256,
      }
      impl ::subxt::Call for UnnotePreimage {
        const PALLET: &'static str = "Preimage";
        const FUNCTION: &'static str = "unnote_preimage";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct RequestPreimage {
        pub hash: ::subxt::sp_core::H256,
      }
      impl ::subxt::Call for RequestPreimage {
        const PALLET: &'static str = "Preimage";
        const FUNCTION: &'static str = "request_preimage";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct UnrequestPreimage {
        pub hash: ::subxt::sp_core::H256,
      }
      impl ::subxt::Call for UnrequestPreimage {
        const PALLET: &'static str = "Preimage";
        const FUNCTION: &'static str = "unrequest_preimage";
      }
      pub struct TransactionApi<'a, T: ::subxt::Config, X> {
        client: &'a ::subxt::Client<T>,
        marker: ::core::marker::PhantomData<X>,
      }
      impl<'a, T, X> TransactionApi<'a, T, X>
      where
        T: ::subxt::Config,
        X: ::subxt::extrinsic::ExtrinsicParams<T>,
      {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self {
            client,
            marker: ::core::marker::PhantomData,
          }
        }
        pub fn note_preimage(
          &self,
          bytes: ::std::vec::Vec<::core::primitive::u8>,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, NotePreimage, DispatchError, root_mod::Event>
        {
          let call = NotePreimage { bytes };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn unnote_preimage(
          &self,
          hash: ::subxt::sp_core::H256,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, UnnotePreimage, DispatchError, root_mod::Event>
        {
          let call = UnnotePreimage { hash };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn request_preimage(
          &self,
          hash: ::subxt::sp_core::H256,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, RequestPreimage, DispatchError, root_mod::Event>
        {
          let call = RequestPreimage { hash };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn unrequest_preimage(
          &self,
          hash: ::subxt::sp_core::H256,
        ) -> ::subxt::SubmittableExtrinsic<
          'a,
          T,
          X,
          UnrequestPreimage,
          DispatchError,
          root_mod::Event,
        > {
          let call = UnrequestPreimage { hash };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
      }
    }
    pub type Event = runtime_types::pallet_preimage::pallet::Event;
    pub mod events {
      use super::runtime_types;
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Noted {
        pub hash: ::subxt::sp_core::H256,
      }
      impl ::subxt::Event for Noted {
        const PALLET: &'static str = "Preimage";
        const EVENT: &'static str = "Noted";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Requested {
        pub hash: ::subxt::sp_core::H256,
      }
      impl ::subxt::Event for Requested {
        const PALLET: &'static str = "Preimage";
        const EVENT: &'static str = "Requested";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Cleared {
        pub hash: ::subxt::sp_core::H256,
      }
      impl ::subxt::Event for Cleared {
        const PALLET: &'static str = "Preimage";
        const EVENT: &'static str = "Cleared";
      }
    }
    pub mod storage {
      use super::runtime_types;
      pub struct StatusFor<'a>(pub &'a ::subxt::sp_core::H256);
      impl ::subxt::StorageEntry for StatusFor<'_> {
        const PALLET: &'static str = "Preimage";
        const STORAGE: &'static str = "StatusFor";
        type Value = runtime_types::pallet_preimage::RequestStatus<
          ::subxt::sp_core::crypto::AccountId32,
          ::core::primitive::u128,
        >;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
            &self.0,
            ::subxt::StorageHasher::Identity,
          )])
        }
      }
      pub struct PreimageFor<'a>(pub &'a ::subxt::sp_core::H256);
      impl ::subxt::StorageEntry for PreimageFor<'_> {
        const PALLET: &'static str = "Preimage";
        const STORAGE: &'static str = "PreimageFor";
        type Value =
          runtime_types::frame_support::storage::bounded_vec::BoundedVec<::core::primitive::u8>;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
            &self.0,
            ::subxt::StorageHasher::Identity,
          )])
        }
      }
      pub struct StorageApi<'a, T: ::subxt::Config> {
        client: &'a ::subxt::Client<T>,
      }
      impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self { client }
        }
        pub async fn status_for(
          &self,
          _0: &::subxt::sp_core::H256,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::core::option::Option<
            runtime_types::pallet_preimage::RequestStatus<
              ::subxt::sp_core::crypto::AccountId32,
              ::core::primitive::u128,
            >,
          >,
          ::subxt::BasicError,
        > {
          let entry = StatusFor(_0);
          self.client.storage().fetch(&entry, hash).await
        }
        pub async fn status_for_iter(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::subxt::KeyIter<'a, T, StatusFor<'a>>, ::subxt::BasicError>
        {
          self.client.storage().iter(hash).await
        }
        pub async fn preimage_for(
          &self,
          _0: &::subxt::sp_core::H256,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::core::option::Option<
            runtime_types::frame_support::storage::bounded_vec::BoundedVec<::core::primitive::u8>,
          >,
          ::subxt::BasicError,
        > {
          let entry = PreimageFor(_0);
          self.client.storage().fetch(&entry, hash).await
        }
        pub async fn preimage_for_iter(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::subxt::KeyIter<'a, T, PreimageFor<'a>>, ::subxt::BasicError>
        {
          self.client.storage().iter(hash).await
        }
      }
    }
  }
  pub mod sudo {
    use super::root_mod;
    use super::runtime_types;
    pub mod calls {
      use super::root_mod;
      use super::runtime_types;
      type DispatchError = runtime_types::sp_runtime::DispatchError;
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Sudo {
        pub call: ::std::boxed::Box<runtime_types::lagoon_runtime::Call>,
      }
      impl ::subxt::Call for Sudo {
        const PALLET: &'static str = "Sudo";
        const FUNCTION: &'static str = "sudo";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct SudoUncheckedWeight {
        pub call: ::std::boxed::Box<runtime_types::lagoon_runtime::Call>,
        pub weight: ::core::primitive::u64,
      }
      impl ::subxt::Call for SudoUncheckedWeight {
        const PALLET: &'static str = "Sudo";
        const FUNCTION: &'static str = "sudo_unchecked_weight";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct SetKey {
        pub new: ::subxt::sp_runtime::MultiAddress<
          ::subxt::sp_core::crypto::AccountId32,
          ::core::primitive::u32,
        >,
      }
      impl ::subxt::Call for SetKey {
        const PALLET: &'static str = "Sudo";
        const FUNCTION: &'static str = "set_key";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct SudoAs {
        pub who: ::subxt::sp_runtime::MultiAddress<
          ::subxt::sp_core::crypto::AccountId32,
          ::core::primitive::u32,
        >,
        pub call: ::std::boxed::Box<runtime_types::lagoon_runtime::Call>,
      }
      impl ::subxt::Call for SudoAs {
        const PALLET: &'static str = "Sudo";
        const FUNCTION: &'static str = "sudo_as";
      }
      pub struct TransactionApi<'a, T: ::subxt::Config, X> {
        client: &'a ::subxt::Client<T>,
        marker: ::core::marker::PhantomData<X>,
      }
      impl<'a, T, X> TransactionApi<'a, T, X>
      where
        T: ::subxt::Config,
        X: ::subxt::extrinsic::ExtrinsicParams<T>,
      {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self {
            client,
            marker: ::core::marker::PhantomData,
          }
        }
        pub fn sudo(
          &self,
          call: runtime_types::lagoon_runtime::Call,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, Sudo, DispatchError, root_mod::Event> {
          let call = Sudo {
            call: ::std::boxed::Box::new(call),
          };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn sudo_unchecked_weight(
          &self,
          call: runtime_types::lagoon_runtime::Call,
          weight: ::core::primitive::u64,
        ) -> ::subxt::SubmittableExtrinsic<
          'a,
          T,
          X,
          SudoUncheckedWeight,
          DispatchError,
          root_mod::Event,
        > {
          let call = SudoUncheckedWeight {
            call: ::std::boxed::Box::new(call),
            weight,
          };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn set_key(
          &self,
          new: ::subxt::sp_runtime::MultiAddress<
            ::subxt::sp_core::crypto::AccountId32,
            ::core::primitive::u32,
          >,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, SetKey, DispatchError, root_mod::Event>
        {
          let call = SetKey { new };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn sudo_as(
          &self,
          who: ::subxt::sp_runtime::MultiAddress<
            ::subxt::sp_core::crypto::AccountId32,
            ::core::primitive::u32,
          >,
          call: runtime_types::lagoon_runtime::Call,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, SudoAs, DispatchError, root_mod::Event>
        {
          let call = SudoAs {
            who,
            call: ::std::boxed::Box::new(call),
          };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
      }
    }
    pub type Event = runtime_types::pallet_sudo::pallet::Event;
    pub mod events {
      use super::runtime_types;
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Sudid {
        pub sudo_result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
      }
      impl ::subxt::Event for Sudid {
        const PALLET: &'static str = "Sudo";
        const EVENT: &'static str = "Sudid";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct KeyChanged {
        pub old_sudoer: ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
      }
      impl ::subxt::Event for KeyChanged {
        const PALLET: &'static str = "Sudo";
        const EVENT: &'static str = "KeyChanged";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct SudoAsDone {
        pub sudo_result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
      }
      impl ::subxt::Event for SudoAsDone {
        const PALLET: &'static str = "Sudo";
        const EVENT: &'static str = "SudoAsDone";
      }
    }
    pub mod storage {
      use super::runtime_types;
      pub struct Key;
      impl ::subxt::StorageEntry for Key {
        const PALLET: &'static str = "Sudo";
        const STORAGE: &'static str = "Key";
        type Value = ::subxt::sp_core::crypto::AccountId32;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct StorageApi<'a, T: ::subxt::Config> {
        client: &'a ::subxt::Client<T>,
      }
      impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self { client }
        }
        pub async fn key(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
          ::subxt::BasicError,
        > {
          let entry = Key;
          self.client.storage().fetch(&entry, hash).await
        }
      }
    }
  }
  pub mod tidefi {
    use super::root_mod;
    use super::runtime_types;
    pub mod calls {
      use super::root_mod;
      use super::runtime_types;
      type DispatchError = runtime_types::sp_runtime::DispatchError;
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Transfer {
        pub destination_id: ::subxt::sp_core::crypto::AccountId32,
        pub currency_id: runtime_types::tidefi_primitives::CurrencyId,
        pub amount: ::core::primitive::u128,
      }
      impl ::subxt::Call for Transfer {
        const PALLET: &'static str = "Tidefi";
        const FUNCTION: &'static str = "transfer";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Withdrawal {
        pub currency_id: runtime_types::tidefi_primitives::CurrencyId,
        pub amount: ::core::primitive::u128,
        pub external_address: ::std::vec::Vec<::core::primitive::u8>,
      }
      impl ::subxt::Call for Withdrawal {
        const PALLET: &'static str = "Tidefi";
        const FUNCTION: &'static str = "withdrawal";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Swap {
        pub currency_id_from: runtime_types::tidefi_primitives::CurrencyId,
        pub amount_from: ::core::primitive::u128,
        pub currency_id_to: runtime_types::tidefi_primitives::CurrencyId,
        pub amount_to: ::core::primitive::u128,
        pub swap_type: runtime_types::tidefi_primitives::SwapType,
        pub slippage_tolerance:
          ::core::option::Option<runtime_types::sp_arithmetic::per_things::Permill>,
      }
      impl ::subxt::Call for Swap {
        const PALLET: &'static str = "Tidefi";
        const FUNCTION: &'static str = "swap";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct CancelSwap {
        pub request_id: ::subxt::sp_core::H256,
      }
      impl ::subxt::Call for CancelSwap {
        const PALLET: &'static str = "Tidefi";
        const FUNCTION: &'static str = "cancel_swap";
      }
      pub struct TransactionApi<'a, T: ::subxt::Config, X> {
        client: &'a ::subxt::Client<T>,
        marker: ::core::marker::PhantomData<X>,
      }
      impl<'a, T, X> TransactionApi<'a, T, X>
      where
        T: ::subxt::Config,
        X: ::subxt::extrinsic::ExtrinsicParams<T>,
      {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self {
            client,
            marker: ::core::marker::PhantomData,
          }
        }
        pub fn transfer(
          &self,
          destination_id: ::subxt::sp_core::crypto::AccountId32,
          currency_id: runtime_types::tidefi_primitives::CurrencyId,
          amount: ::core::primitive::u128,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, Transfer, DispatchError, root_mod::Event>
        {
          let call = Transfer {
            destination_id,
            currency_id,
            amount,
          };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn withdrawal(
          &self,
          currency_id: runtime_types::tidefi_primitives::CurrencyId,
          amount: ::core::primitive::u128,
          external_address: ::std::vec::Vec<::core::primitive::u8>,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, Withdrawal, DispatchError, root_mod::Event>
        {
          let call = Withdrawal {
            currency_id,
            amount,
            external_address,
          };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn swap(
          &self,
          currency_id_from: runtime_types::tidefi_primitives::CurrencyId,
          amount_from: ::core::primitive::u128,
          currency_id_to: runtime_types::tidefi_primitives::CurrencyId,
          amount_to: ::core::primitive::u128,
          swap_type: runtime_types::tidefi_primitives::SwapType,
          slippage_tolerance: ::core::option::Option<
            runtime_types::sp_arithmetic::per_things::Permill,
          >,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, Swap, DispatchError, root_mod::Event> {
          let call = Swap {
            currency_id_from,
            amount_from,
            currency_id_to,
            amount_to,
            swap_type,
            slippage_tolerance,
          };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn cancel_swap(
          &self,
          request_id: ::subxt::sp_core::H256,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, CancelSwap, DispatchError, root_mod::Event>
        {
          let call = CancelSwap { request_id };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
      }
    }
    pub type Event = runtime_types::pallet_tidefi::pallet::Event;
    pub mod events {
      use super::runtime_types;
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Transfer {
        pub from_account_id: ::subxt::sp_core::crypto::AccountId32,
        pub to_account_id: ::subxt::sp_core::crypto::AccountId32,
        pub currency_id: runtime_types::tidefi_primitives::CurrencyId,
        pub amount: ::core::primitive::u128,
      }
      impl ::subxt::Event for Transfer {
        const PALLET: &'static str = "Tidefi";
        const EVENT: &'static str = "Transfer";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Withdrawal {
        pub account: ::subxt::sp_core::crypto::AccountId32,
        pub currency_id: runtime_types::tidefi_primitives::CurrencyId,
        pub amount: ::core::primitive::u128,
        pub external_address: ::std::vec::Vec<::core::primitive::u8>,
      }
      impl ::subxt::Event for Withdrawal {
        const PALLET: &'static str = "Tidefi";
        const EVENT: &'static str = "Withdrawal";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Swap {
        pub request_id: ::subxt::sp_core::H256,
        pub account: ::subxt::sp_core::crypto::AccountId32,
        pub currency_id_from: runtime_types::tidefi_primitives::CurrencyId,
        pub amount_from: ::core::primitive::u128,
        pub currency_id_to: runtime_types::tidefi_primitives::CurrencyId,
        pub amount_to: ::core::primitive::u128,
        pub extrinsic_hash: [::core::primitive::u8; 32usize],
        pub slippage_tolerance: runtime_types::sp_arithmetic::per_things::Permill,
        pub swap_type: runtime_types::tidefi_primitives::SwapType,
        pub is_market_maker: ::core::primitive::bool,
      }
      impl ::subxt::Event for Swap {
        const PALLET: &'static str = "Tidefi";
        const EVENT: &'static str = "Swap";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct SwapCancelled {
        pub request_id: ::subxt::sp_core::H256,
      }
      impl ::subxt::Event for SwapCancelled {
        const PALLET: &'static str = "Tidefi";
        const EVENT: &'static str = "SwapCancelled";
      }
    }
    pub mod storage {
      use super::runtime_types;
      pub struct StorageApi<'a, T: ::subxt::Config> {
        client: &'a ::subxt::Client<T>,
      }
      impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self { client }
        }
      }
    }
  }
  pub mod tidefi_staking {
    use super::root_mod;
    use super::runtime_types;
    pub mod calls {
      use super::root_mod;
      use super::runtime_types;
      type DispatchError = runtime_types::sp_runtime::DispatchError;
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Stake {
        pub currency_id: runtime_types::tidefi_primitives::CurrencyId,
        pub amount: ::core::primitive::u128,
        pub duration: ::core::primitive::u32,
      }
      impl ::subxt::Call for Stake {
        const PALLET: &'static str = "TidefiStaking";
        const FUNCTION: &'static str = "stake";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Unstake {
        pub stake_id: ::subxt::sp_core::H256,
        pub force_unstake: ::core::primitive::bool,
      }
      impl ::subxt::Call for Unstake {
        const PALLET: &'static str = "TidefiStaking";
        const FUNCTION: &'static str = "unstake";
      }
      pub struct TransactionApi<'a, T: ::subxt::Config, X> {
        client: &'a ::subxt::Client<T>,
        marker: ::core::marker::PhantomData<X>,
      }
      impl<'a, T, X> TransactionApi<'a, T, X>
      where
        T: ::subxt::Config,
        X: ::subxt::extrinsic::ExtrinsicParams<T>,
      {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self {
            client,
            marker: ::core::marker::PhantomData,
          }
        }
        pub fn stake(
          &self,
          currency_id: runtime_types::tidefi_primitives::CurrencyId,
          amount: ::core::primitive::u128,
          duration: ::core::primitive::u32,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, Stake, DispatchError, root_mod::Event>
        {
          let call = Stake {
            currency_id,
            amount,
            duration,
          };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn unstake(
          &self,
          stake_id: ::subxt::sp_core::H256,
          force_unstake: ::core::primitive::bool,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, Unstake, DispatchError, root_mod::Event>
        {
          let call = Unstake {
            stake_id,
            force_unstake,
          };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
      }
    }
    pub type Event = runtime_types::pallet_tidefi_stake::pallet::Event;
    pub mod events {
      use super::runtime_types;
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Staked {
        pub request_id: ::subxt::sp_core::H256,
        pub account_id: ::subxt::sp_core::crypto::AccountId32,
        pub currency_id: runtime_types::tidefi_primitives::CurrencyId,
        pub amount: ::core::primitive::u128,
        pub duration: ::core::primitive::u32,
      }
      impl ::subxt::Event for Staked {
        const PALLET: &'static str = "TidefiStaking";
        const EVENT: &'static str = "Staked";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct UnstakeQueued {
        pub request_id: ::subxt::sp_core::H256,
        pub account_id: ::subxt::sp_core::crypto::AccountId32,
      }
      impl ::subxt::Event for UnstakeQueued {
        const PALLET: &'static str = "TidefiStaking";
        const EVENT: &'static str = "UnstakeQueued";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Unstaked {
        pub request_id: ::subxt::sp_core::H256,
        pub account_id: ::subxt::sp_core::crypto::AccountId32,
        pub currency_id: runtime_types::tidefi_primitives::CurrencyId,
        pub initial_balance: ::core::primitive::u128,
        pub final_balance: ::core::primitive::u128,
      }
      impl ::subxt::Event for Unstaked {
        const PALLET: &'static str = "TidefiStaking";
        const EVENT: &'static str = "Unstaked";
      }
    }
    pub mod storage {
      use super::runtime_types;
      pub struct StakingPool<'a>(pub &'a runtime_types::tidefi_primitives::CurrencyId);
      impl ::subxt::StorageEntry for StakingPool<'_> {
        const PALLET: &'static str = "TidefiStaking";
        const STORAGE: &'static str = "StakingPool";
        type Value = ::core::primitive::u128;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
            &self.0,
            ::subxt::StorageHasher::Blake2_128Concat,
          )])
        }
      }
      pub struct StakingPeriodRewards;
      impl ::subxt::StorageEntry for StakingPeriodRewards {
        const PALLET: &'static str = "TidefiStaking";
        const STORAGE: &'static str = "StakingPeriodRewards";
        type Value = runtime_types::frame_support::storage::bounded_vec::BoundedVec<(
          ::core::primitive::u32,
          runtime_types::sp_arithmetic::per_things::Percent,
        )>;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct StakingCurrencyMeta<'a>(pub &'a runtime_types::tidefi_primitives::CurrencyId);
      impl ::subxt::StorageEntry for StakingCurrencyMeta<'_> {
        const PALLET: &'static str = "TidefiStaking";
        const STORAGE: &'static str = "StakingCurrencyMeta";
        type Value = runtime_types::tidefi_primitives::StakeCurrencyMeta<::core::primitive::u128>;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
            &self.0,
            ::subxt::StorageHasher::Blake2_128Concat,
          )])
        }
      }
      pub struct UnstakeFee;
      impl ::subxt::StorageEntry for UnstakeFee {
        const PALLET: &'static str = "TidefiStaking";
        const STORAGE: &'static str = "UnstakeFee";
        type Value = runtime_types::sp_arithmetic::per_things::Percent;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct InterestCompoundLastSession;
      impl ::subxt::StorageEntry for InterestCompoundLastSession {
        const PALLET: &'static str = "TidefiStaking";
        const STORAGE: &'static str = "InterestCompoundLastSession";
        type Value = ::core::primitive::u64;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct UnstakeQueue;
      impl ::subxt::StorageEntry for UnstakeQueue {
        const PALLET: &'static str = "TidefiStaking";
        const STORAGE: &'static str = "UnstakeQueue";
        type Value = runtime_types::frame_support::storage::bounded_vec::BoundedVec<(
          ::subxt::sp_core::crypto::AccountId32,
          ::subxt::sp_core::H256,
          ::core::primitive::u32,
        )>;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct PendingStoredSessions<'a>(pub &'a ::core::primitive::u64);
      impl ::subxt::StorageEntry for PendingStoredSessions<'_> {
        const PALLET: &'static str = "TidefiStaking";
        const STORAGE: &'static str = "PendingStoredSessions";
        type Value = ();
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
            &self.0,
            ::subxt::StorageHasher::Blake2_128Concat,
          )])
        }
      }
      pub struct CounterForPendingStoredSessions;
      impl ::subxt::StorageEntry for CounterForPendingStoredSessions {
        const PALLET: &'static str = "TidefiStaking";
        const STORAGE: &'static str = "CounterForPendingStoredSessions";
        type Value = ::core::primitive::u32;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct SessionTotalFees<'a>(
        pub &'a ::core::primitive::u64,
        pub &'a runtime_types::tidefi_primitives::CurrencyId,
      );
      impl ::subxt::StorageEntry for SessionTotalFees<'_> {
        const PALLET: &'static str = "TidefiStaking";
        const STORAGE: &'static str = "SessionTotalFees";
        type Value = ::core::primitive::u128;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Map(vec![
            ::subxt::StorageMapKey::new(&self.0, ::subxt::StorageHasher::Blake2_128Concat),
            ::subxt::StorageMapKey::new(&self.1, ::subxt::StorageHasher::Blake2_128Concat),
          ])
        }
      }
      pub struct AccountStakes<'a>(pub &'a ::subxt::sp_core::crypto::AccountId32);
      impl ::subxt::StorageEntry for AccountStakes<'_> {
        const PALLET: &'static str = "TidefiStaking";
        const STORAGE: &'static str = "AccountStakes";
        type Value = runtime_types::frame_support::storage::bounded_vec::BoundedVec<
          runtime_types::tidefi_primitives::Stake<::core::primitive::u128, ::core::primitive::u32>,
        >;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
            &self.0,
            ::subxt::StorageHasher::Blake2_128Concat,
          )])
        }
      }
      pub struct CounterForAccountStakes;
      impl ::subxt::StorageEntry for CounterForAccountStakes {
        const PALLET: &'static str = "TidefiStaking";
        const STORAGE: &'static str = "CounterForAccountStakes";
        type Value = ::core::primitive::u32;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct StorageApi<'a, T: ::subxt::Config> {
        client: &'a ::subxt::Client<T>,
      }
      impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self { client }
        }
        pub async fn staking_pool(
          &self,
          _0: &runtime_types::tidefi_primitives::CurrencyId,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::core::option::Option<::core::primitive::u128>,
          ::subxt::BasicError,
        > {
          let entry = StakingPool(_0);
          self.client.storage().fetch(&entry, hash).await
        }
        pub async fn staking_pool_iter(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::subxt::KeyIter<'a, T, StakingPool<'a>>, ::subxt::BasicError>
        {
          self.client.storage().iter(hash).await
        }
        pub async fn staking_period_rewards(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          runtime_types::frame_support::storage::bounded_vec::BoundedVec<(
            ::core::primitive::u32,
            runtime_types::sp_arithmetic::per_things::Percent,
          )>,
          ::subxt::BasicError,
        > {
          let entry = StakingPeriodRewards;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn staking_currency_meta(
          &self,
          _0: &runtime_types::tidefi_primitives::CurrencyId,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::core::option::Option<
            runtime_types::tidefi_primitives::StakeCurrencyMeta<::core::primitive::u128>,
          >,
          ::subxt::BasicError,
        > {
          let entry = StakingCurrencyMeta(_0);
          self.client.storage().fetch(&entry, hash).await
        }
        pub async fn staking_currency_meta_iter(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::subxt::KeyIter<'a, T, StakingCurrencyMeta<'a>>,
          ::subxt::BasicError,
        > {
          self.client.storage().iter(hash).await
        }
        pub async fn unstake_fee(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          runtime_types::sp_arithmetic::per_things::Percent,
          ::subxt::BasicError,
        > {
          let entry = UnstakeFee;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn interest_compound_last_session(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError> {
          let entry = InterestCompoundLastSession;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn unstake_queue(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          runtime_types::frame_support::storage::bounded_vec::BoundedVec<(
            ::subxt::sp_core::crypto::AccountId32,
            ::subxt::sp_core::H256,
            ::core::primitive::u32,
          )>,
          ::subxt::BasicError,
        > {
          let entry = UnstakeQueue;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn pending_stored_sessions(
          &self,
          _0: &::core::primitive::u64,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::core::option::Option<()>, ::subxt::BasicError> {
          let entry = PendingStoredSessions(_0);
          self.client.storage().fetch(&entry, hash).await
        }
        pub async fn pending_stored_sessions_iter(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::subxt::KeyIter<'a, T, PendingStoredSessions<'a>>,
          ::subxt::BasicError,
        > {
          self.client.storage().iter(hash).await
        }
        pub async fn counter_for_pending_stored_sessions(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
          let entry = CounterForPendingStoredSessions;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn session_total_fees(
          &self,
          _0: &::core::primitive::u64,
          _1: &runtime_types::tidefi_primitives::CurrencyId,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError> {
          let entry = SessionTotalFees(_0, _1);
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn session_total_fees_iter(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::subxt::KeyIter<'a, T, SessionTotalFees<'a>>,
          ::subxt::BasicError,
        > {
          self.client.storage().iter(hash).await
        }
        pub async fn account_stakes(
          &self,
          _0: &::subxt::sp_core::crypto::AccountId32,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          runtime_types::frame_support::storage::bounded_vec::BoundedVec<
            runtime_types::tidefi_primitives::Stake<
              ::core::primitive::u128,
              ::core::primitive::u32,
            >,
          >,
          ::subxt::BasicError,
        > {
          let entry = AccountStakes(_0);
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn account_stakes_iter(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::subxt::KeyIter<'a, T, AccountStakes<'a>>, ::subxt::BasicError>
        {
          self.client.storage().iter(hash).await
        }
        pub async fn counter_for_account_stakes(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
          let entry = CounterForAccountStakes;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
      }
    }
    pub mod constants {
      use super::runtime_types;
      pub struct ConstantsApi<'a, T: ::subxt::Config> {
        client: &'a ::subxt::Client<T>,
      }
      impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self { client }
        }
        pub fn stake_pallet_id(
          &self,
        ) -> ::core::result::Result<runtime_types::frame_support::PalletId, ::subxt::BasicError>
        {
          let pallet = self.client.metadata().pallet("TidefiStaking")?;
          let constant = pallet.constant("StakePalletId")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn unstake_queue_cap(
          &self,
        ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
          let pallet = self.client.metadata().pallet("TidefiStaking")?;
          let constant = pallet.constant("UnstakeQueueCap")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn stake_account_cap(
          &self,
        ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
          let pallet = self.client.metadata().pallet("TidefiStaking")?;
          let constant = pallet.constant("StakeAccountCap")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn staking_reward_cap(
          &self,
        ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
          let pallet = self.client.metadata().pallet("TidefiStaking")?;
          let constant = pallet.constant("StakingRewardCap")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn blocks_force_unstake(
          &self,
        ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
          let pallet = self.client.metadata().pallet("TidefiStaking")?;
          let constant = pallet.constant("BlocksForceUnstake")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
      }
    }
  }
  pub mod quorum {
    use super::root_mod;
    use super::runtime_types;
    pub mod calls {
      use super::root_mod;
      use super::runtime_types;
      type DispatchError = runtime_types::sp_runtime::DispatchError;
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct SubmitProposal {
        pub proposal: runtime_types::tidefi_primitives::ProposalType<
          ::subxt::sp_core::crypto::AccountId32,
          ::core::primitive::u32,
          ::std::vec::Vec<::core::primitive::u8>,
          ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
        >,
      }
      impl ::subxt::Call for SubmitProposal {
        const PALLET: &'static str = "Quorum";
        const FUNCTION: &'static str = "submit_proposal";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct AcknowledgeProposal {
        pub proposal: ::subxt::sp_core::H256,
      }
      impl ::subxt::Call for AcknowledgeProposal {
        const PALLET: &'static str = "Quorum";
        const FUNCTION: &'static str = "acknowledge_proposal";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct AcknowledgeBurned {
        pub proposal: ::subxt::sp_core::H256,
      }
      impl ::subxt::Call for AcknowledgeBurned {
        const PALLET: &'static str = "Quorum";
        const FUNCTION: &'static str = "acknowledge_burned";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct RejectProposal {
        pub proposal: ::subxt::sp_core::H256,
      }
      impl ::subxt::Call for RejectProposal {
        const PALLET: &'static str = "Quorum";
        const FUNCTION: &'static str = "reject_proposal";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct EvalProposalState {
        pub proposal: ::subxt::sp_core::H256,
      }
      impl ::subxt::Call for EvalProposalState {
        const PALLET: &'static str = "Quorum";
        const FUNCTION: &'static str = "eval_proposal_state";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct SubmitPublicKeys {
        pub public_keys: ::std::vec::Vec<(
          ::core::primitive::u32,
          ::std::vec::Vec<::core::primitive::u8>,
        )>,
      }
      impl ::subxt::Call for SubmitPublicKeys {
        const PALLET: &'static str = "Quorum";
        const FUNCTION: &'static str = "submit_public_keys";
      }
      pub struct TransactionApi<'a, T: ::subxt::Config, X> {
        client: &'a ::subxt::Client<T>,
        marker: ::core::marker::PhantomData<X>,
      }
      impl<'a, T, X> TransactionApi<'a, T, X>
      where
        T: ::subxt::Config,
        X: ::subxt::extrinsic::ExtrinsicParams<T>,
      {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self {
            client,
            marker: ::core::marker::PhantomData,
          }
        }
        pub fn submit_proposal(
          &self,
          proposal: runtime_types::tidefi_primitives::ProposalType<
            ::subxt::sp_core::crypto::AccountId32,
            ::core::primitive::u32,
            ::std::vec::Vec<::core::primitive::u8>,
            ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
          >,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, SubmitProposal, DispatchError, root_mod::Event>
        {
          let call = SubmitProposal { proposal };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn acknowledge_proposal(
          &self,
          proposal: ::subxt::sp_core::H256,
        ) -> ::subxt::SubmittableExtrinsic<
          'a,
          T,
          X,
          AcknowledgeProposal,
          DispatchError,
          root_mod::Event,
        > {
          let call = AcknowledgeProposal { proposal };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn acknowledge_burned(
          &self,
          proposal: ::subxt::sp_core::H256,
        ) -> ::subxt::SubmittableExtrinsic<
          'a,
          T,
          X,
          AcknowledgeBurned,
          DispatchError,
          root_mod::Event,
        > {
          let call = AcknowledgeBurned { proposal };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn reject_proposal(
          &self,
          proposal: ::subxt::sp_core::H256,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, RejectProposal, DispatchError, root_mod::Event>
        {
          let call = RejectProposal { proposal };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn eval_proposal_state(
          &self,
          proposal: ::subxt::sp_core::H256,
        ) -> ::subxt::SubmittableExtrinsic<
          'a,
          T,
          X,
          EvalProposalState,
          DispatchError,
          root_mod::Event,
        > {
          let call = EvalProposalState { proposal };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn submit_public_keys(
          &self,
          public_keys: ::std::vec::Vec<(
            ::core::primitive::u32,
            ::std::vec::Vec<::core::primitive::u8>,
          )>,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, SubmitPublicKeys, DispatchError, root_mod::Event>
        {
          let call = SubmitPublicKeys { public_keys };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
      }
    }
    pub type Event = runtime_types::pallet_quorum::pallet::Event;
    pub mod events {
      use super::runtime_types;
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct StatusChanged {
        pub is_enabled: ::core::primitive::bool,
      }
      impl ::subxt::Event for StatusChanged {
        const PALLET: &'static str = "Quorum";
        const EVENT: &'static str = "StatusChanged";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct AccountChanged {
        pub account_id: ::subxt::sp_core::crypto::AccountId32,
      }
      impl ::subxt::Event for AccountChanged {
        const PALLET: &'static str = "Quorum";
        const EVENT: &'static str = "AccountChanged";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Minted {
        pub proposal_id: ::subxt::sp_core::H256,
        pub account_id: ::subxt::sp_core::crypto::AccountId32,
        pub currency_id: runtime_types::tidefi_primitives::CurrencyId,
        pub amount: ::core::primitive::u128,
        pub transaction_id: ::std::vec::Vec<::core::primitive::u8>,
        pub compliance_level: runtime_types::tidefi_primitives::ComplianceLevel,
      }
      impl ::subxt::Event for Minted {
        const PALLET: &'static str = "Quorum";
        const EVENT: &'static str = "Minted";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct WatchTransactionAdded {
        pub account_id: ::subxt::sp_core::crypto::AccountId32,
        pub currency_id: runtime_types::tidefi_primitives::CurrencyId,
        pub amount: ::core::primitive::u128,
        pub compliance_level: runtime_types::tidefi_primitives::ComplianceLevel,
        pub transaction_id: ::std::vec::Vec<::core::primitive::u8>,
        pub watch_action: runtime_types::tidefi_primitives::WatchListAction,
      }
      impl ::subxt::Event for WatchTransactionAdded {
        const PALLET: &'static str = "Quorum";
        const EVENT: &'static str = "WatchTransactionAdded";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct BurnedInitialized {
        pub proposal_id: ::subxt::sp_core::H256,
        pub account_id: ::subxt::sp_core::crypto::AccountId32,
        pub currency_id: runtime_types::tidefi_primitives::CurrencyId,
        pub amount: ::core::primitive::u128,
      }
      impl ::subxt::Event for BurnedInitialized {
        const PALLET: &'static str = "Quorum";
        const EVENT: &'static str = "BurnedInitialized";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct BurnedAcknowledged {
        pub proposal_id: ::subxt::sp_core::H256,
      }
      impl ::subxt::Event for BurnedAcknowledged {
        const PALLET: &'static str = "Quorum";
        const EVENT: &'static str = "BurnedAcknowledged";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct VoteFor {
        pub account_id: ::subxt::sp_core::crypto::AccountId32,
        pub proposal_id: ::subxt::sp_core::H256,
      }
      impl ::subxt::Event for VoteFor {
        const PALLET: &'static str = "Quorum";
        const EVENT: &'static str = "VoteFor";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct VoteAgainst {
        pub account_id: ::subxt::sp_core::crypto::AccountId32,
        pub proposal_id: ::subxt::sp_core::H256,
      }
      impl ::subxt::Event for VoteAgainst {
        const PALLET: &'static str = "Quorum";
        const EVENT: &'static str = "VoteAgainst";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct ProposalSubmitted {
        pub proposal_id: ::subxt::sp_core::H256,
      }
      impl ::subxt::Event for ProposalSubmitted {
        const PALLET: &'static str = "Quorum";
        const EVENT: &'static str = "ProposalSubmitted";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct ProposalApproved {
        pub proposal_id: ::subxt::sp_core::H256,
      }
      impl ::subxt::Event for ProposalApproved {
        const PALLET: &'static str = "Quorum";
        const EVENT: &'static str = "ProposalApproved";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct ProposalProcessed {
        pub proposal_id: ::subxt::sp_core::H256,
      }
      impl ::subxt::Event for ProposalProcessed {
        const PALLET: &'static str = "Quorum";
        const EVENT: &'static str = "ProposalProcessed";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct ProposalRejected {
        pub proposal_id: ::subxt::sp_core::H256,
      }
      impl ::subxt::Event for ProposalRejected {
        const PALLET: &'static str = "Quorum";
        const EVENT: &'static str = "ProposalRejected";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct ConfigurationUpdated {
        pub members: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
        pub threshold: ::core::primitive::u16,
      }
      impl ::subxt::Event for ConfigurationUpdated {
        const PALLET: &'static str = "Quorum";
        const EVENT: &'static str = "ConfigurationUpdated";
      }
    }
    pub mod storage {
      use super::runtime_types;
      pub struct QuorumStatus;
      impl ::subxt::StorageEntry for QuorumStatus {
        const PALLET: &'static str = "Quorum";
        const STORAGE: &'static str = "QuorumStatus";
        type Value = ::core::primitive::bool;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct PublicKeys<'a>(pub &'a ::core::primitive::u32);
      impl ::subxt::StorageEntry for PublicKeys<'_> {
        const PALLET: &'static str = "Quorum";
        const STORAGE: &'static str = "PublicKeys";
        type Value = runtime_types::frame_support::storage::bounded_vec::BoundedVec<(
          ::subxt::sp_core::crypto::AccountId32,
          runtime_types::frame_support::storage::bounded_vec::BoundedVec<::core::primitive::u8>,
        )>;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
            &self.0,
            ::subxt::StorageHasher::Blake2_128Concat,
          )])
        }
      }
      pub struct AccountWatchList<'a>(pub &'a ::subxt::sp_core::crypto::AccountId32);
      impl ::subxt::StorageEntry for AccountWatchList<'_> {
        const PALLET: &'static str = "Quorum";
        const STORAGE: &'static str = "AccountWatchList";
        type Value = runtime_types::frame_support::storage::bounded_vec::BoundedVec<
          runtime_types::tidefi_primitives::WatchList<
            ::core::primitive::u32,
            runtime_types::frame_support::storage::bounded_vec::BoundedVec<::core::primitive::u8>,
          >,
        >;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
            &self.0,
            ::subxt::StorageHasher::Blake2_128Concat,
          )])
        }
      }
      pub struct Threshold;
      impl ::subxt::StorageEntry for Threshold {
        const PALLET: &'static str = "Quorum";
        const STORAGE: &'static str = "Threshold";
        type Value = ::core::primitive::u16;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct Proposals;
      impl ::subxt::StorageEntry for Proposals {
        const PALLET: &'static str = "Quorum";
        const STORAGE: &'static str = "Proposals";
        type Value = runtime_types::frame_support::storage::bounded_vec::BoundedVec<(
          ::subxt::sp_core::H256,
          ::core::primitive::u32,
          runtime_types::tidefi_primitives::ProposalType<
            ::subxt::sp_core::crypto::AccountId32,
            ::core::primitive::u32,
            runtime_types::frame_support::storage::bounded_vec::BoundedVec<::core::primitive::u8>,
            runtime_types::frame_support::storage::bounded_vec::BoundedVec<
              ::subxt::sp_core::crypto::AccountId32,
            >,
          >,
        )>;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct Votes<'a>(pub &'a ::subxt::sp_core::H256);
      impl ::subxt::StorageEntry for Votes<'_> {
        const PALLET: &'static str = "Quorum";
        const STORAGE: &'static str = "Votes";
        type Value = runtime_types::tidefi_primitives::ProposalVotes<
          ::core::primitive::u32,
          runtime_types::frame_support::storage::bounded_vec::BoundedVec<
            ::subxt::sp_core::crypto::AccountId32,
          >,
        >;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
            &self.0,
            ::subxt::StorageHasher::Blake2_128Concat,
          )])
        }
      }
      pub struct Members<'a>(pub &'a ::subxt::sp_core::crypto::AccountId32);
      impl ::subxt::StorageEntry for Members<'_> {
        const PALLET: &'static str = "Quorum";
        const STORAGE: &'static str = "Members";
        type Value = ::core::primitive::bool;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
            &self.0,
            ::subxt::StorageHasher::Blake2_128Concat,
          )])
        }
      }
      pub struct CounterForMembers;
      impl ::subxt::StorageEntry for CounterForMembers {
        const PALLET: &'static str = "Quorum";
        const STORAGE: &'static str = "CounterForMembers";
        type Value = ::core::primitive::u32;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct BurnedQueue;
      impl ::subxt::StorageEntry for BurnedQueue {
        const PALLET: &'static str = "Quorum";
        const STORAGE: &'static str = "BurnedQueue";
        type Value = runtime_types::frame_support::storage::bounded_vec::BoundedVec<(
          ::subxt::sp_core::H256,
          runtime_types::tidefi_primitives::Withdrawal<
            ::subxt::sp_core::crypto::AccountId32,
            ::core::primitive::u32,
            runtime_types::frame_support::storage::bounded_vec::BoundedVec<::core::primitive::u8>,
          >,
        )>;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct StorageApi<'a, T: ::subxt::Config> {
        client: &'a ::subxt::Client<T>,
      }
      impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self { client }
        }
        pub async fn quorum_status(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::core::primitive::bool, ::subxt::BasicError> {
          let entry = QuorumStatus;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn public_keys(
          &self,
          _0: &::core::primitive::u32,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          runtime_types::frame_support::storage::bounded_vec::BoundedVec<(
            ::subxt::sp_core::crypto::AccountId32,
            runtime_types::frame_support::storage::bounded_vec::BoundedVec<::core::primitive::u8>,
          )>,
          ::subxt::BasicError,
        > {
          let entry = PublicKeys(_0);
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn public_keys_iter(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::subxt::KeyIter<'a, T, PublicKeys<'a>>, ::subxt::BasicError>
        {
          self.client.storage().iter(hash).await
        }
        pub async fn account_watch_list(
          &self,
          _0: &::subxt::sp_core::crypto::AccountId32,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::core::option::Option<
            runtime_types::frame_support::storage::bounded_vec::BoundedVec<
              runtime_types::tidefi_primitives::WatchList<
                ::core::primitive::u32,
                runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                  ::core::primitive::u8,
                >,
              >,
            >,
          >,
          ::subxt::BasicError,
        > {
          let entry = AccountWatchList(_0);
          self.client.storage().fetch(&entry, hash).await
        }
        pub async fn account_watch_list_iter(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::subxt::KeyIter<'a, T, AccountWatchList<'a>>,
          ::subxt::BasicError,
        > {
          self.client.storage().iter(hash).await
        }
        pub async fn threshold(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::core::primitive::u16, ::subxt::BasicError> {
          let entry = Threshold;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn proposals(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          runtime_types::frame_support::storage::bounded_vec::BoundedVec<(
            ::subxt::sp_core::H256,
            ::core::primitive::u32,
            runtime_types::tidefi_primitives::ProposalType<
              ::subxt::sp_core::crypto::AccountId32,
              ::core::primitive::u32,
              runtime_types::frame_support::storage::bounded_vec::BoundedVec<::core::primitive::u8>,
              runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                ::subxt::sp_core::crypto::AccountId32,
              >,
            >,
          )>,
          ::subxt::BasicError,
        > {
          let entry = Proposals;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn votes(
          &self,
          _0: &::subxt::sp_core::H256,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::core::option::Option<
            runtime_types::tidefi_primitives::ProposalVotes<
              ::core::primitive::u32,
              runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                ::subxt::sp_core::crypto::AccountId32,
              >,
            >,
          >,
          ::subxt::BasicError,
        > {
          let entry = Votes(_0);
          self.client.storage().fetch(&entry, hash).await
        }
        pub async fn votes_iter(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Votes<'a>>, ::subxt::BasicError>
        {
          self.client.storage().iter(hash).await
        }
        pub async fn members(
          &self,
          _0: &::subxt::sp_core::crypto::AccountId32,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::core::option::Option<::core::primitive::bool>,
          ::subxt::BasicError,
        > {
          let entry = Members(_0);
          self.client.storage().fetch(&entry, hash).await
        }
        pub async fn members_iter(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Members<'a>>, ::subxt::BasicError>
        {
          self.client.storage().iter(hash).await
        }
        pub async fn counter_for_members(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
          let entry = CounterForMembers;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn burned_queue(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          runtime_types::frame_support::storage::bounded_vec::BoundedVec<(
            ::subxt::sp_core::H256,
            runtime_types::tidefi_primitives::Withdrawal<
              ::subxt::sp_core::crypto::AccountId32,
              ::core::primitive::u32,
              runtime_types::frame_support::storage::bounded_vec::BoundedVec<::core::primitive::u8>,
            >,
          )>,
          ::subxt::BasicError,
        > {
          let entry = BurnedQueue;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
      }
    }
    pub mod constants {
      use super::runtime_types;
      pub struct ConstantsApi<'a, T: ::subxt::Config> {
        client: &'a ::subxt::Client<T>,
      }
      impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self { client }
        }
        pub fn quorum_pallet_id(
          &self,
        ) -> ::core::result::Result<runtime_types::frame_support::PalletId, ::subxt::BasicError>
        {
          let pallet = self.client.metadata().pallet("Quorum")?;
          let constant = pallet.constant("QuorumPalletId")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn proposals_cap(
          &self,
        ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
          let pallet = self.client.metadata().pallet("Quorum")?;
          let constant = pallet.constant("ProposalsCap")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn burned_cap(
          &self,
        ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
          let pallet = self.client.metadata().pallet("Quorum")?;
          let constant = pallet.constant("BurnedCap")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn proposal_lifetime(
          &self,
        ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
          let pallet = self.client.metadata().pallet("Quorum")?;
          let constant = pallet.constant("ProposalLifetime")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn string_limit(
          &self,
        ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
          let pallet = self.client.metadata().pallet("Quorum")?;
          let constant = pallet.constant("StringLimit")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn votes_limit(
          &self,
        ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
          let pallet = self.client.metadata().pallet("Quorum")?;
          let constant = pallet.constant("VotesLimit")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn watch_list_limit(
          &self,
        ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
          let pallet = self.client.metadata().pallet("Quorum")?;
          let constant = pallet.constant("WatchListLimit")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn pubkey_limit_per_asset(
          &self,
        ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
          let pallet = self.client.metadata().pallet("Quorum")?;
          let constant = pallet.constant("PubkeyLimitPerAsset")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
      }
    }
  }
  pub mod oracle {
    use super::root_mod;
    use super::runtime_types;
    pub mod calls {
      use super::root_mod;
      use super::runtime_types;
      type DispatchError = runtime_types::sp_runtime::DispatchError;
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct ConfirmSwap {
        pub request_id: ::subxt::sp_core::H256,
        pub market_makers: ::std::vec::Vec<runtime_types::tidefi_primitives::SwapConfirmation>,
      }
      impl ::subxt::Call for ConfirmSwap {
        const PALLET: &'static str = "Oracle";
        const FUNCTION: &'static str = "confirm_swap";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct CancelSwap {
        pub request_id: ::subxt::sp_core::H256,
      }
      impl ::subxt::Call for CancelSwap {
        const PALLET: &'static str = "Oracle";
        const FUNCTION: &'static str = "cancel_swap";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct SetAccountId {
        pub new_account_id: ::subxt::sp_core::crypto::AccountId32,
      }
      impl ::subxt::Call for SetAccountId {
        const PALLET: &'static str = "Oracle";
        const FUNCTION: &'static str = "set_account_id";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct SetStatus {
        pub is_enabled: ::core::primitive::bool,
      }
      impl ::subxt::Call for SetStatus {
        const PALLET: &'static str = "Oracle";
        const FUNCTION: &'static str = "set_status";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct ImAlive {
        pub im_alive: runtime_types::tidefi_primitives::OracleImAlive,
      }
      impl ::subxt::Call for ImAlive {
        const PALLET: &'static str = "Oracle";
        const FUNCTION: &'static str = "im_alive";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct AddMarketMaker {
        pub account_id: ::subxt::sp_core::crypto::AccountId32,
      }
      impl ::subxt::Call for AddMarketMaker {
        const PALLET: &'static str = "Oracle";
        const FUNCTION: &'static str = "add_market_maker";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct RemoveMarketMaker {
        pub account_id: ::subxt::sp_core::crypto::AccountId32,
      }
      impl ::subxt::Call for RemoveMarketMaker {
        const PALLET: &'static str = "Oracle";
        const FUNCTION: &'static str = "remove_market_maker";
      }
      pub struct TransactionApi<'a, T: ::subxt::Config, X> {
        client: &'a ::subxt::Client<T>,
        marker: ::core::marker::PhantomData<X>,
      }
      impl<'a, T, X> TransactionApi<'a, T, X>
      where
        T: ::subxt::Config,
        X: ::subxt::extrinsic::ExtrinsicParams<T>,
      {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self {
            client,
            marker: ::core::marker::PhantomData,
          }
        }
        pub fn confirm_swap(
          &self,
          request_id: ::subxt::sp_core::H256,
          market_makers: ::std::vec::Vec<runtime_types::tidefi_primitives::SwapConfirmation>,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, ConfirmSwap, DispatchError, root_mod::Event>
        {
          let call = ConfirmSwap {
            request_id,
            market_makers,
          };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn cancel_swap(
          &self,
          request_id: ::subxt::sp_core::H256,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, CancelSwap, DispatchError, root_mod::Event>
        {
          let call = CancelSwap { request_id };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn set_account_id(
          &self,
          new_account_id: ::subxt::sp_core::crypto::AccountId32,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, SetAccountId, DispatchError, root_mod::Event>
        {
          let call = SetAccountId { new_account_id };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn set_status(
          &self,
          is_enabled: ::core::primitive::bool,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, SetStatus, DispatchError, root_mod::Event>
        {
          let call = SetStatus { is_enabled };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn im_alive(
          &self,
          im_alive: runtime_types::tidefi_primitives::OracleImAlive,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, ImAlive, DispatchError, root_mod::Event>
        {
          let call = ImAlive { im_alive };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn add_market_maker(
          &self,
          account_id: ::subxt::sp_core::crypto::AccountId32,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, AddMarketMaker, DispatchError, root_mod::Event>
        {
          let call = AddMarketMaker { account_id };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn remove_market_maker(
          &self,
          account_id: ::subxt::sp_core::crypto::AccountId32,
        ) -> ::subxt::SubmittableExtrinsic<
          'a,
          T,
          X,
          RemoveMarketMaker,
          DispatchError,
          root_mod::Event,
        > {
          let call = RemoveMarketMaker { account_id };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
      }
    }
    pub type Event = runtime_types::pallet_oracle::pallet::Event;
    pub mod events {
      use super::runtime_types;
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct StatusChanged {
        pub is_enabled: ::core::primitive::bool,
      }
      impl ::subxt::Event for StatusChanged {
        const PALLET: &'static str = "Oracle";
        const EVENT: &'static str = "StatusChanged";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct AccountChanged {
        pub account_id: ::subxt::sp_core::crypto::AccountId32,
      }
      impl ::subxt::Event for AccountChanged {
        const PALLET: &'static str = "Oracle";
        const EVENT: &'static str = "AccountChanged";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct MarketMakerAdded {
        pub account_id: ::subxt::sp_core::crypto::AccountId32,
      }
      impl ::subxt::Event for MarketMakerAdded {
        const PALLET: &'static str = "Oracle";
        const EVENT: &'static str = "MarketMakerAdded";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct MarketMakerRemoved {
        pub account_id: ::subxt::sp_core::crypto::AccountId32,
      }
      impl ::subxt::Event for MarketMakerRemoved {
        const PALLET: &'static str = "Oracle";
        const EVENT: &'static str = "MarketMakerRemoved";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct SwapProcessed {
        pub request_id: ::subxt::sp_core::H256,
        pub status: runtime_types::tidefi_primitives::SwapStatus,
        pub account_id: ::subxt::sp_core::crypto::AccountId32,
        pub currency_from: runtime_types::tidefi_primitives::CurrencyId,
        pub currency_amount_from: ::core::primitive::u128,
        pub currency_to: runtime_types::tidefi_primitives::CurrencyId,
        pub currency_amount_to: ::core::primitive::u128,
        pub initial_extrinsic_hash: [::core::primitive::u8; 32usize],
      }
      impl ::subxt::Event for SwapProcessed {
        const PALLET: &'static str = "Oracle";
        const EVENT: &'static str = "SwapProcessed";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct SwapCancelled {
        pub request_id: ::subxt::sp_core::H256,
      }
      impl ::subxt::Event for SwapCancelled {
        const PALLET: &'static str = "Oracle";
        const EVENT: &'static str = "SwapCancelled";
      }
    }
    pub mod storage {
      use super::runtime_types;
      pub struct OracleStatus;
      impl ::subxt::StorageEntry for OracleStatus {
        const PALLET: &'static str = "Oracle";
        const STORAGE: &'static str = "OracleStatus";
        type Value = ::core::primitive::bool;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct LastSeen;
      impl ::subxt::StorageEntry for LastSeen {
        const PALLET: &'static str = "Oracle";
        const STORAGE: &'static str = "LastSeen";
        type Value = ::core::primitive::u32;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct OracleAccountId;
      impl ::subxt::StorageEntry for OracleAccountId {
        const PALLET: &'static str = "Oracle";
        const STORAGE: &'static str = "OracleAccountId";
        type Value = ::subxt::sp_core::crypto::AccountId32;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct Swaps<'a>(pub &'a ::subxt::sp_core::H256);
      impl ::subxt::StorageEntry for Swaps<'_> {
        const PALLET: &'static str = "Oracle";
        const STORAGE: &'static str = "Swaps";
        type Value = runtime_types::tidefi_primitives::Swap<
          ::subxt::sp_core::crypto::AccountId32,
          ::core::primitive::u32,
        >;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
            &self.0,
            ::subxt::StorageHasher::Blake2_128Concat,
          )])
        }
      }
      pub struct AccountSwaps<'a>(pub &'a ::subxt::sp_core::crypto::AccountId32);
      impl ::subxt::StorageEntry for AccountSwaps<'_> {
        const PALLET: &'static str = "Oracle";
        const STORAGE: &'static str = "AccountSwaps";
        type Value = runtime_types::frame_support::storage::bounded_vec::BoundedVec<(
          ::subxt::sp_core::H256,
          runtime_types::tidefi_primitives::SwapStatus,
        )>;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
            &self.0,
            ::subxt::StorageHasher::Blake2_128Concat,
          )])
        }
      }
      pub struct CounterForAccountSwaps;
      impl ::subxt::StorageEntry for CounterForAccountSwaps {
        const PALLET: &'static str = "Oracle";
        const STORAGE: &'static str = "CounterForAccountSwaps";
        type Value = ::core::primitive::u32;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct MarketMakers<'a>(pub &'a ::subxt::sp_core::crypto::AccountId32);
      impl ::subxt::StorageEntry for MarketMakers<'_> {
        const PALLET: &'static str = "Oracle";
        const STORAGE: &'static str = "MarketMakers";
        type Value = ::core::primitive::bool;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
            &self.0,
            ::subxt::StorageHasher::Blake2_128Concat,
          )])
        }
      }
      pub struct StorageApi<'a, T: ::subxt::Config> {
        client: &'a ::subxt::Client<T>,
      }
      impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self { client }
        }
        pub async fn oracle_status(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::core::primitive::bool, ::subxt::BasicError> {
          let entry = OracleStatus;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn last_seen(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
          let entry = LastSeen;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn oracle_account_id(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
          ::subxt::BasicError,
        > {
          let entry = OracleAccountId;
          self.client.storage().fetch(&entry, hash).await
        }
        pub async fn swaps(
          &self,
          _0: &::subxt::sp_core::H256,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::core::option::Option<
            runtime_types::tidefi_primitives::Swap<
              ::subxt::sp_core::crypto::AccountId32,
              ::core::primitive::u32,
            >,
          >,
          ::subxt::BasicError,
        > {
          let entry = Swaps(_0);
          self.client.storage().fetch(&entry, hash).await
        }
        pub async fn swaps_iter(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Swaps<'a>>, ::subxt::BasicError>
        {
          self.client.storage().iter(hash).await
        }
        pub async fn account_swaps(
          &self,
          _0: &::subxt::sp_core::crypto::AccountId32,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::core::option::Option<
            runtime_types::frame_support::storage::bounded_vec::BoundedVec<(
              ::subxt::sp_core::H256,
              runtime_types::tidefi_primitives::SwapStatus,
            )>,
          >,
          ::subxt::BasicError,
        > {
          let entry = AccountSwaps(_0);
          self.client.storage().fetch(&entry, hash).await
        }
        pub async fn account_swaps_iter(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::subxt::KeyIter<'a, T, AccountSwaps<'a>>, ::subxt::BasicError>
        {
          self.client.storage().iter(hash).await
        }
        pub async fn counter_for_account_swaps(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
          let entry = CounterForAccountSwaps;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn market_makers(
          &self,
          _0: &::subxt::sp_core::crypto::AccountId32,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::core::option::Option<::core::primitive::bool>,
          ::subxt::BasicError,
        > {
          let entry = MarketMakers(_0);
          self.client.storage().fetch(&entry, hash).await
        }
        pub async fn market_makers_iter(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::subxt::KeyIter<'a, T, MarketMakers<'a>>, ::subxt::BasicError>
        {
          self.client.storage().iter(hash).await
        }
      }
    }
    pub mod constants {
      use super::runtime_types;
      pub struct ConstantsApi<'a, T: ::subxt::Config> {
        client: &'a ::subxt::Client<T>,
      }
      impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self { client }
        }
        pub fn oracle_pallet_id(
          &self,
        ) -> ::core::result::Result<runtime_types::frame_support::PalletId, ::subxt::BasicError>
        {
          let pallet = self.client.metadata().pallet("Oracle")?;
          let constant = pallet.constant("OraclePalletId")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn swap_limit_by_account(
          &self,
        ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
          let pallet = self.client.metadata().pallet("Oracle")?;
          let constant = pallet.constant("SwapLimitByAccount")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
      }
    }
  }
  pub mod security {
    use super::root_mod;
    use super::runtime_types;
    pub mod calls {
      use super::root_mod;
      use super::runtime_types;
      type DispatchError = runtime_types::sp_runtime::DispatchError;
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct SetStatus {
        pub status_code: runtime_types::tidefi_primitives::StatusCode,
      }
      impl ::subxt::Call for SetStatus {
        const PALLET: &'static str = "Security";
        const FUNCTION: &'static str = "set_status";
      }
      pub struct TransactionApi<'a, T: ::subxt::Config, X> {
        client: &'a ::subxt::Client<T>,
        marker: ::core::marker::PhantomData<X>,
      }
      impl<'a, T, X> TransactionApi<'a, T, X>
      where
        T: ::subxt::Config,
        X: ::subxt::extrinsic::ExtrinsicParams<T>,
      {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self {
            client,
            marker: ::core::marker::PhantomData,
          }
        }
        pub fn set_status(
          &self,
          status_code: runtime_types::tidefi_primitives::StatusCode,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, SetStatus, DispatchError, root_mod::Event>
        {
          let call = SetStatus { status_code };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
      }
    }
    pub type Event = runtime_types::pallet_security::pallet::Event;
    pub mod events {
      use super::runtime_types;
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct StatusChanged(pub runtime_types::tidefi_primitives::StatusCode);
      impl ::subxt::Event for StatusChanged {
        const PALLET: &'static str = "Security";
        const EVENT: &'static str = "StatusChanged";
      }
      #[derive(
        :: subxt :: codec :: Encode,
        :: subxt :: codec :: Decode,
        Debug,
        :: subxt :: codec :: CompactAs,
      )]
      pub struct UpdateCurrentBlock(pub ::core::primitive::u32);
      impl ::subxt::Event for UpdateCurrentBlock {
        const PALLET: &'static str = "Security";
        const EVENT: &'static str = "UpdateCurrentBlock";
      }
    }
    pub mod storage {
      use super::runtime_types;
      pub struct Nonce;
      impl ::subxt::StorageEntry for Nonce {
        const PALLET: &'static str = "Security";
        const STORAGE: &'static str = "Nonce";
        type Value = runtime_types::primitive_types::U256;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct ChainStatus;
      impl ::subxt::StorageEntry for ChainStatus {
        const PALLET: &'static str = "Security";
        const STORAGE: &'static str = "ChainStatus";
        type Value = runtime_types::tidefi_primitives::StatusCode;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct CurrentBlockCount;
      impl ::subxt::StorageEntry for CurrentBlockCount {
        const PALLET: &'static str = "Security";
        const STORAGE: &'static str = "CurrentBlockCount";
        type Value = ::core::primitive::u32;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct StorageApi<'a, T: ::subxt::Config> {
        client: &'a ::subxt::Client<T>,
      }
      impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self { client }
        }
        pub async fn nonce(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<runtime_types::primitive_types::U256, ::subxt::BasicError>
        {
          let entry = Nonce;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn chain_status(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<runtime_types::tidefi_primitives::StatusCode, ::subxt::BasicError>
        {
          let entry = ChainStatus;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn current_block_count(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
          let entry = CurrentBlockCount;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
      }
    }
  }
  pub mod fees {
    use super::root_mod;
    use super::runtime_types;
    pub mod calls {
      use super::root_mod;
      use super::runtime_types;
      type DispatchError = runtime_types::sp_runtime::DispatchError;
      #[derive(
        :: subxt :: codec :: Encode,
        :: subxt :: codec :: Decode,
        Debug,
        :: subxt :: codec :: CompactAs,
      )]
      pub struct ClaimSunriseRewards {
        pub era_index: ::core::primitive::u32,
      }
      impl ::subxt::Call for ClaimSunriseRewards {
        const PALLET: &'static str = "Fees";
        const FUNCTION: &'static str = "claim_sunrise_rewards";
      }
      pub struct TransactionApi<'a, T: ::subxt::Config, X> {
        client: &'a ::subxt::Client<T>,
        marker: ::core::marker::PhantomData<X>,
      }
      impl<'a, T, X> TransactionApi<'a, T, X>
      where
        T: ::subxt::Config,
        X: ::subxt::extrinsic::ExtrinsicParams<T>,
      {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self {
            client,
            marker: ::core::marker::PhantomData,
          }
        }
        pub fn claim_sunrise_rewards(
          &self,
          era_index: ::core::primitive::u32,
        ) -> ::subxt::SubmittableExtrinsic<
          'a,
          T,
          X,
          ClaimSunriseRewards,
          DispatchError,
          root_mod::Event,
        > {
          let call = ClaimSunriseRewards { era_index };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
      }
    }
    pub type Event = runtime_types::pallet_fees::pallet::Event;
    pub mod events {
      use super::runtime_types;
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct FeesPercentageUpdated(pub runtime_types::sp_arithmetic::per_things::Percent);
      impl ::subxt::Event for FeesPercentageUpdated {
        const PALLET: &'static str = "Fees";
        const EVENT: &'static str = "FeesPercentageUpdated";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct SessionEnded {
        pub era_index: ::core::primitive::u32,
        pub session_index: ::core::primitive::u64,
        pub session_fees_by_currency: ::std::vec::Vec<(
          runtime_types::tidefi_primitives::CurrencyId,
          ::core::primitive::u128,
        )>,
      }
      impl ::subxt::Event for SessionEnded {
        const PALLET: &'static str = "Fees";
        const EVENT: &'static str = "SessionEnded";
      }
      #[derive(
        :: subxt :: codec :: Encode,
        :: subxt :: codec :: Decode,
        Debug,
        :: subxt :: codec :: CompactAs,
      )]
      pub struct EraStarted {
        pub era_index: ::core::primitive::u32,
      }
      impl ::subxt::Event for EraStarted {
        const PALLET: &'static str = "Fees";
        const EVENT: &'static str = "EraStarted";
      }
      #[derive(
        :: subxt :: codec :: Encode,
        :: subxt :: codec :: Decode,
        Debug,
        :: subxt :: codec :: CompactAs,
      )]
      pub struct EraEnded {
        pub era_index: ::core::primitive::u32,
      }
      impl ::subxt::Event for EraEnded {
        const PALLET: &'static str = "Fees";
        const EVENT: &'static str = "EraEnded";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct SunriseRewarded {
        pub era_index: ::core::primitive::u32,
        pub pool_id: ::core::primitive::u8,
        pub account_id: ::subxt::sp_core::crypto::AccountId32,
        pub reward: ::core::primitive::u128,
      }
      impl ::subxt::Event for SunriseRewarded {
        const PALLET: &'static str = "Fees";
        const EVENT: &'static str = "SunriseRewarded";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct SunriseClaimed {
        pub era_index: ::core::primitive::u32,
        pub account_id: ::subxt::sp_core::crypto::AccountId32,
        pub reward: ::core::primitive::u128,
      }
      impl ::subxt::Event for SunriseClaimed {
        const PALLET: &'static str = "Fees";
        const EVENT: &'static str = "SunriseClaimed";
      }
    }
    pub mod storage {
      use super::runtime_types;
      pub struct ActiveEra;
      impl ::subxt::StorageEntry for ActiveEra {
        const PALLET: &'static str = "Fees";
        const STORAGE: &'static str = "ActiveEra";
        type Value = runtime_types::tidefi_primitives::ActiveEraInfo<::core::primitive::u32>;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct SunrisePools;
      impl ::subxt::StorageEntry for SunrisePools {
        const PALLET: &'static str = "Fees";
        const STORAGE: &'static str = "SunrisePools";
        type Value = runtime_types::frame_support::storage::bounded_vec::BoundedVec<
          runtime_types::tidefi_primitives::SunriseSwapPool,
        >;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct CurrentSession;
      impl ::subxt::StorageEntry for CurrentSession {
        const PALLET: &'static str = "Fees";
        const STORAGE: &'static str = "CurrentSession";
        type Value = ::core::primitive::u64;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct EraTotalFees<'a>(
        pub &'a ::core::primitive::u32,
        pub &'a runtime_types::tidefi_primitives::CurrencyId,
      );
      impl ::subxt::StorageEntry for EraTotalFees<'_> {
        const PALLET: &'static str = "Fees";
        const STORAGE: &'static str = "EraTotalFees";
        type Value = runtime_types::tidefi_primitives::Fee;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Map(vec![
            ::subxt::StorageMapKey::new(&self.0, ::subxt::StorageHasher::Blake2_128Concat),
            ::subxt::StorageMapKey::new(&self.1, ::subxt::StorageHasher::Blake2_128Concat),
          ])
        }
      }
      pub struct StoredSessions<'a>(pub &'a ::core::primitive::u64);
      impl ::subxt::StorageEntry for StoredSessions<'_> {
        const PALLET: &'static str = "Fees";
        const STORAGE: &'static str = "StoredSessions";
        type Value = ();
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
            &self.0,
            ::subxt::StorageHasher::Blake2_128Concat,
          )])
        }
      }
      pub struct OrderBookPrice<'a>(
        pub &'a runtime_types::tidefi_primitives::CurrencyId,
        pub &'a runtime_types::tidefi_primitives::CurrencyId,
      );
      impl ::subxt::StorageEntry for OrderBookPrice<'_> {
        const PALLET: &'static str = "Fees";
        const STORAGE: &'static str = "OrderBookPrice";
        type Value = runtime_types::sp_arithmetic::fixed_point::FixedU128;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Map(vec![
            ::subxt::StorageMapKey::new(&self.0, ::subxt::StorageHasher::Blake2_128Concat),
            ::subxt::StorageMapKey::new(&self.1, ::subxt::StorageHasher::Blake2_128Concat),
          ])
        }
      }
      pub struct SessionTotalFees<'a>(
        pub &'a ::core::primitive::u64,
        pub &'a runtime_types::tidefi_primitives::CurrencyId,
      );
      impl ::subxt::StorageEntry for SessionTotalFees<'_> {
        const PALLET: &'static str = "Fees";
        const STORAGE: &'static str = "SessionTotalFees";
        type Value = runtime_types::tidefi_primitives::Fee;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Map(vec![
            ::subxt::StorageMapKey::new(&self.0, ::subxt::StorageHasher::Blake2_128Concat),
            ::subxt::StorageMapKey::new(&self.1, ::subxt::StorageHasher::Blake2_128Concat),
          ])
        }
      }
      pub struct AccountFees<'a>(
        pub &'a ::core::primitive::u32,
        pub &'a ::subxt::sp_core::crypto::AccountId32,
      );
      impl ::subxt::StorageEntry for AccountFees<'_> {
        const PALLET: &'static str = "Fees";
        const STORAGE: &'static str = "AccountFees";
        type Value = runtime_types::frame_support::storage::bounded_vec::BoundedVec<(
          runtime_types::tidefi_primitives::CurrencyId,
          runtime_types::tidefi_primitives::Fee,
        )>;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Map(vec![
            ::subxt::StorageMapKey::new(&self.0, ::subxt::StorageHasher::Blake2_128Concat),
            ::subxt::StorageMapKey::new(&self.1, ::subxt::StorageHasher::Blake2_128Concat),
          ])
        }
      }
      pub struct SunriseRewards<'a>(
        pub &'a ::subxt::sp_core::crypto::AccountId32,
        pub &'a ::core::primitive::u32,
      );
      impl ::subxt::StorageEntry for SunriseRewards<'_> {
        const PALLET: &'static str = "Fees";
        const STORAGE: &'static str = "SunriseRewards";
        type Value = ::core::primitive::u128;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Map(vec![
            ::subxt::StorageMapKey::new(&self.0, ::subxt::StorageHasher::Blake2_128Concat),
            ::subxt::StorageMapKey::new(&self.1, ::subxt::StorageHasher::Blake2_128Concat),
          ])
        }
      }
      pub struct StorageApi<'a, T: ::subxt::Config> {
        client: &'a ::subxt::Client<T>,
      }
      impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self { client }
        }
        pub async fn active_era(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::core::option::Option<
            runtime_types::tidefi_primitives::ActiveEraInfo<::core::primitive::u32>,
          >,
          ::subxt::BasicError,
        > {
          let entry = ActiveEra;
          self.client.storage().fetch(&entry, hash).await
        }
        pub async fn sunrise_pools(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          runtime_types::frame_support::storage::bounded_vec::BoundedVec<
            runtime_types::tidefi_primitives::SunriseSwapPool,
          >,
          ::subxt::BasicError,
        > {
          let entry = SunrisePools;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn current_session(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError> {
          let entry = CurrentSession;
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn era_total_fees(
          &self,
          _0: &::core::primitive::u32,
          _1: &runtime_types::tidefi_primitives::CurrencyId,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<runtime_types::tidefi_primitives::Fee, ::subxt::BasicError>
        {
          let entry = EraTotalFees(_0, _1);
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn era_total_fees_iter(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::subxt::KeyIter<'a, T, EraTotalFees<'a>>, ::subxt::BasicError>
        {
          self.client.storage().iter(hash).await
        }
        pub async fn stored_sessions(
          &self,
          _0: &::core::primitive::u64,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::core::option::Option<()>, ::subxt::BasicError> {
          let entry = StoredSessions(_0);
          self.client.storage().fetch(&entry, hash).await
        }
        pub async fn stored_sessions_iter(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::subxt::KeyIter<'a, T, StoredSessions<'a>>, ::subxt::BasicError>
        {
          self.client.storage().iter(hash).await
        }
        pub async fn order_book_price(
          &self,
          _0: &runtime_types::tidefi_primitives::CurrencyId,
          _1: &runtime_types::tidefi_primitives::CurrencyId,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          runtime_types::sp_arithmetic::fixed_point::FixedU128,
          ::subxt::BasicError,
        > {
          let entry = OrderBookPrice(_0, _1);
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn order_book_price_iter(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::subxt::KeyIter<'a, T, OrderBookPrice<'a>>, ::subxt::BasicError>
        {
          self.client.storage().iter(hash).await
        }
        pub async fn session_total_fees(
          &self,
          _0: &::core::primitive::u64,
          _1: &runtime_types::tidefi_primitives::CurrencyId,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<runtime_types::tidefi_primitives::Fee, ::subxt::BasicError>
        {
          let entry = SessionTotalFees(_0, _1);
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn session_total_fees_iter(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::subxt::KeyIter<'a, T, SessionTotalFees<'a>>,
          ::subxt::BasicError,
        > {
          self.client.storage().iter(hash).await
        }
        pub async fn account_fees(
          &self,
          _0: &::core::primitive::u32,
          _1: &::subxt::sp_core::crypto::AccountId32,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          runtime_types::frame_support::storage::bounded_vec::BoundedVec<(
            runtime_types::tidefi_primitives::CurrencyId,
            runtime_types::tidefi_primitives::Fee,
          )>,
          ::subxt::BasicError,
        > {
          let entry = AccountFees(_0, _1);
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn account_fees_iter(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::subxt::KeyIter<'a, T, AccountFees<'a>>, ::subxt::BasicError>
        {
          self.client.storage().iter(hash).await
        }
        pub async fn sunrise_rewards(
          &self,
          _0: &::subxt::sp_core::crypto::AccountId32,
          _1: &::core::primitive::u32,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError> {
          let entry = SunriseRewards(_0, _1);
          self.client.storage().fetch_or_default(&entry, hash).await
        }
        pub async fn sunrise_rewards_iter(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<::subxt::KeyIter<'a, T, SunriseRewards<'a>>, ::subxt::BasicError>
        {
          self.client.storage().iter(hash).await
        }
      }
    }
    pub mod constants {
      use super::runtime_types;
      pub struct ConstantsApi<'a, T: ::subxt::Config> {
        client: &'a ::subxt::Client<T>,
      }
      impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self { client }
        }
        pub fn fees_pallet_id(
          &self,
        ) -> ::core::result::Result<runtime_types::frame_support::PalletId, ::subxt::BasicError>
        {
          let pallet = self.client.metadata().pallet("Fees")?;
          let constant = pallet.constant("FeesPalletId")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn sessions_per_era(
          &self,
        ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError> {
          let pallet = self.client.metadata().pallet("Fees")?;
          let constant = pallet.constant("SessionsPerEra")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn sessions_archive(
          &self,
        ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError> {
          let pallet = self.client.metadata().pallet("Fees")?;
          let constant = pallet.constant("SessionsArchive")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn blocks_per_session(
          &self,
        ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
          let pallet = self.client.metadata().pallet("Fees")?;
          let constant = pallet.constant("BlocksPerSession")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn blocks_sunrise_claims(
          &self,
        ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
          let pallet = self.client.metadata().pallet("Fees")?;
          let constant = pallet.constant("BlocksSunriseClaims")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn fee_amount(
          &self,
        ) -> ::core::result::Result<
          runtime_types::sp_arithmetic::per_things::Permill,
          ::subxt::BasicError,
        > {
          let pallet = self.client.metadata().pallet("Fees")?;
          let constant = pallet.constant("FeeAmount")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
        pub fn market_maker_fee_amount(
          &self,
        ) -> ::core::result::Result<
          runtime_types::sp_arithmetic::per_things::Permill,
          ::subxt::BasicError,
        > {
          let pallet = self.client.metadata().pallet("Fees")?;
          let constant = pallet.constant("MarketMakerFeeAmount")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
      }
    }
  }
  pub mod asset_registry {
    use super::root_mod;
    use super::runtime_types;
    pub mod calls {
      use super::root_mod;
      use super::runtime_types;
      type DispatchError = runtime_types::sp_runtime::DispatchError;
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Register {
        pub currency_id: runtime_types::tidefi_primitives::CurrencyId,
        pub name: ::std::vec::Vec<::core::primitive::u8>,
        pub symbol: ::std::vec::Vec<::core::primitive::u8>,
        pub decimals: ::core::primitive::u8,
        pub existential_deposit: ::core::primitive::u128,
      }
      impl ::subxt::Call for Register {
        const PALLET: &'static str = "AssetRegistry";
        const FUNCTION: &'static str = "register";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct SetStatus {
        pub currency_id: runtime_types::tidefi_primitives::CurrencyId,
        pub is_enabled: ::core::primitive::bool,
      }
      impl ::subxt::Call for SetStatus {
        const PALLET: &'static str = "AssetRegistry";
        const FUNCTION: &'static str = "set_status";
      }
      pub struct TransactionApi<'a, T: ::subxt::Config, X> {
        client: &'a ::subxt::Client<T>,
        marker: ::core::marker::PhantomData<X>,
      }
      impl<'a, T, X> TransactionApi<'a, T, X>
      where
        T: ::subxt::Config,
        X: ::subxt::extrinsic::ExtrinsicParams<T>,
      {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self {
            client,
            marker: ::core::marker::PhantomData,
          }
        }
        pub fn register(
          &self,
          currency_id: runtime_types::tidefi_primitives::CurrencyId,
          name: ::std::vec::Vec<::core::primitive::u8>,
          symbol: ::std::vec::Vec<::core::primitive::u8>,
          decimals: ::core::primitive::u8,
          existential_deposit: ::core::primitive::u128,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, Register, DispatchError, root_mod::Event>
        {
          let call = Register {
            currency_id,
            name,
            symbol,
            decimals,
            existential_deposit,
          };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
        pub fn set_status(
          &self,
          currency_id: runtime_types::tidefi_primitives::CurrencyId,
          is_enabled: ::core::primitive::bool,
        ) -> ::subxt::SubmittableExtrinsic<'a, T, X, SetStatus, DispatchError, root_mod::Event>
        {
          let call = SetStatus {
            currency_id,
            is_enabled,
          };
          ::subxt::SubmittableExtrinsic::new(self.client, call)
        }
      }
    }
    pub type Event = runtime_types::pallet_asset_registry::pallet::Event;
    pub mod events {
      use super::runtime_types;
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Registered(pub runtime_types::tidefi_primitives::CurrencyId);
      impl ::subxt::Event for Registered {
        const PALLET: &'static str = "AssetRegistry";
        const EVENT: &'static str = "Registered";
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct StatusChanged(
        pub runtime_types::tidefi_primitives::CurrencyId,
        pub ::core::primitive::bool,
      );
      impl ::subxt::Event for StatusChanged {
        const PALLET: &'static str = "AssetRegistry";
        const EVENT: &'static str = "StatusChanged";
      }
    }
    pub mod storage {
      use super::runtime_types;
      pub struct AssetRegistryAccountId;
      impl ::subxt::StorageEntry for AssetRegistryAccountId {
        const PALLET: &'static str = "AssetRegistry";
        const STORAGE: &'static str = "AssetRegistryAccountId";
        type Value = ::subxt::sp_core::crypto::AccountId32;
        fn key(&self) -> ::subxt::StorageEntryKey {
          ::subxt::StorageEntryKey::Plain
        }
      }
      pub struct StorageApi<'a, T: ::subxt::Config> {
        client: &'a ::subxt::Client<T>,
      }
      impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self { client }
        }
        pub async fn asset_registry_account_id(
          &self,
          hash: ::core::option::Option<T::Hash>,
        ) -> ::core::result::Result<
          ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
          ::subxt::BasicError,
        > {
          let entry = AssetRegistryAccountId;
          self.client.storage().fetch(&entry, hash).await
        }
      }
    }
    pub mod constants {
      use super::runtime_types;
      pub struct ConstantsApi<'a, T: ::subxt::Config> {
        client: &'a ::subxt::Client<T>,
      }
      impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
        pub fn new(client: &'a ::subxt::Client<T>) -> Self {
          Self { client }
        }
        pub fn asset_registry_pallet_id(
          &self,
        ) -> ::core::result::Result<runtime_types::frame_support::PalletId, ::subxt::BasicError>
        {
          let pallet = self.client.metadata().pallet("AssetRegistry")?;
          let constant = pallet.constant("AssetRegistryPalletId")?;
          let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
          Ok(value)
        }
      }
    }
  }
  pub mod runtime_types {
    use super::runtime_types;
    pub mod finality_grandpa {
      use super::runtime_types;
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Equivocation<_0, _1, _2> {
        pub round_number: ::core::primitive::u64,
        pub identity: _0,
        pub first: (_1, _2),
        pub second: (_1, _2),
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Precommit<_0, _1> {
        pub target_hash: _0,
        pub target_number: _1,
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Prevote<_0, _1> {
        pub target_hash: _0,
        pub target_number: _1,
      }
    }
    pub mod frame_support {
      use super::runtime_types;
      pub mod dispatch {
        use super::runtime_types;
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum RawOrigin<_0> {
          #[codec(index = 0)]
          Root,
          #[codec(index = 1)]
          Signed(_0),
          #[codec(index = 2)]
          None,
        }
      }
      pub mod storage {
        use super::runtime_types;
        pub mod bounded_btree_map {
          use super::runtime_types;
          #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
          pub struct BoundedBTreeMap<_0, _1>(pub ::subxt::KeyedVec<_0, _1>);
        }
        pub mod bounded_vec {
          use super::runtime_types;
          #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
          pub struct BoundedVec<_0>(pub ::std::vec::Vec<_0>);
        }
        pub mod weak_bounded_vec {
          use super::runtime_types;
          #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
          pub struct WeakBoundedVec<_0>(pub ::std::vec::Vec<_0>);
        }
      }
      pub mod traits {
        use super::runtime_types;
        pub mod misc {
          use super::runtime_types;
          #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
          pub struct WrapperKeepOpaque<_0>(#[codec(compact)] pub ::core::primitive::u32, pub _0);
          #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
          pub struct WrapperOpaque<_0>(#[codec(compact)] pub ::core::primitive::u32, pub _0);
        }
        pub mod schedule {
          use super::runtime_types;
          #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
          pub enum LookupError {
            #[codec(index = 0)]
            Unknown,
            #[codec(index = 1)]
            BadFormat,
          }
          #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
          pub enum MaybeHashed<_0, _1> {
            #[codec(index = 0)]
            Value(_0),
            #[codec(index = 1)]
            Hash(_1),
          }
        }
        pub mod tokens {
          use super::runtime_types;
          pub mod misc {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub enum BalanceStatus {
              #[codec(index = 0)]
              Free,
              #[codec(index = 1)]
              Reserved,
            }
          }
        }
      }
      pub mod weights {
        use super::runtime_types;
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum DispatchClass {
          #[codec(index = 0)]
          Normal,
          #[codec(index = 1)]
          Operational,
          #[codec(index = 2)]
          Mandatory,
        }
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub struct DispatchInfo {
          pub weight: ::core::primitive::u64,
          pub class: runtime_types::frame_support::weights::DispatchClass,
          pub pays_fee: runtime_types::frame_support::weights::Pays,
        }
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Pays {
          #[codec(index = 0)]
          Yes,
          #[codec(index = 1)]
          No,
        }
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub struct PerDispatchClass<_0> {
          pub normal: _0,
          pub operational: _0,
          pub mandatory: _0,
        }
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub struct RuntimeDbWeight {
          pub read: ::core::primitive::u64,
          pub write: ::core::primitive::u64,
        }
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub struct WeightToFeeCoefficient<_0> {
          pub coeff_integer: _0,
          pub coeff_frac: runtime_types::sp_arithmetic::per_things::Perbill,
          pub negative: ::core::primitive::bool,
          pub degree: ::core::primitive::u8,
        }
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct PalletId(pub [::core::primitive::u8; 8usize]);
    }
    pub mod frame_system {
      use super::runtime_types;
      pub mod extensions {
        use super::runtime_types;
        pub mod check_genesis {
          use super::runtime_types;
          #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
          pub struct CheckGenesis;
        }
        pub mod check_mortality {
          use super::runtime_types;
          #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
          pub struct CheckMortality(pub runtime_types::sp_runtime::generic::era::Era);
        }
        pub mod check_nonce {
          use super::runtime_types;
          #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
          pub struct CheckNonce(#[codec(compact)] pub ::core::primitive::u32);
        }
        pub mod check_spec_version {
          use super::runtime_types;
          #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
          pub struct CheckSpecVersion;
        }
        pub mod check_tx_version {
          use super::runtime_types;
          #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
          pub struct CheckTxVersion;
        }
        pub mod check_weight {
          use super::runtime_types;
          #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
          pub struct CheckWeight;
        }
      }
      pub mod limits {
        use super::runtime_types;
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub struct BlockLength {
          pub max: runtime_types::frame_support::weights::PerDispatchClass<::core::primitive::u32>,
        }
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub struct BlockWeights {
          pub base_block: ::core::primitive::u64,
          pub max_block: ::core::primitive::u64,
          pub per_class: runtime_types::frame_support::weights::PerDispatchClass<
            runtime_types::frame_system::limits::WeightsPerClass,
          >,
        }
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub struct WeightsPerClass {
          pub base_extrinsic: ::core::primitive::u64,
          pub max_extrinsic: ::core::option::Option<::core::primitive::u64>,
          pub max_total: ::core::option::Option<::core::primitive::u64>,
          pub reserved: ::core::option::Option<::core::primitive::u64>,
        }
      }
      pub mod pallet {
        use super::runtime_types;
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Call {
          #[codec(index = 0)]
          fill_block {
            ratio: runtime_types::sp_arithmetic::per_things::Perbill,
          },
          #[codec(index = 1)]
          remark {
            remark: ::std::vec::Vec<::core::primitive::u8>,
          },
          #[codec(index = 2)]
          set_heap_pages { pages: ::core::primitive::u64 },
          #[codec(index = 3)]
          set_code {
            code: ::std::vec::Vec<::core::primitive::u8>,
          },
          #[codec(index = 4)]
          set_code_without_checks {
            code: ::std::vec::Vec<::core::primitive::u8>,
          },
          #[codec(index = 5)]
          set_storage {
            items: ::std::vec::Vec<(
              ::std::vec::Vec<::core::primitive::u8>,
              ::std::vec::Vec<::core::primitive::u8>,
            )>,
          },
          #[codec(index = 6)]
          kill_storage {
            keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
          },
          #[codec(index = 7)]
          kill_prefix {
            prefix: ::std::vec::Vec<::core::primitive::u8>,
            subkeys: ::core::primitive::u32,
          },
          #[codec(index = 8)]
          remark_with_event {
            remark: ::std::vec::Vec<::core::primitive::u8>,
          },
        }
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Error {
          #[codec(index = 0)]
          InvalidSpecName,
          #[codec(index = 1)]
          SpecVersionNeedsToIncrease,
          #[codec(index = 2)]
          FailedToExtractRuntimeVersion,
          #[codec(index = 3)]
          NonDefaultComposite,
          #[codec(index = 4)]
          NonZeroRefCount,
          #[codec(index = 5)]
          CallFiltered,
        }
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Event {
          #[codec(index = 0)]
          ExtrinsicSuccess {
            dispatch_info: runtime_types::frame_support::weights::DispatchInfo,
          },
          #[codec(index = 1)]
          ExtrinsicFailed {
            dispatch_error: runtime_types::sp_runtime::DispatchError,
            dispatch_info: runtime_types::frame_support::weights::DispatchInfo,
          },
          #[codec(index = 2)]
          CodeUpdated,
          #[codec(index = 3)]
          NewAccount {
            account: ::subxt::sp_core::crypto::AccountId32,
          },
          #[codec(index = 4)]
          KilledAccount {
            account: ::subxt::sp_core::crypto::AccountId32,
          },
          #[codec(index = 5)]
          Remarked {
            sender: ::subxt::sp_core::crypto::AccountId32,
            hash: ::subxt::sp_core::H256,
          },
        }
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct AccountInfo<_0, _1> {
        pub nonce: _0,
        pub consumers: _0,
        pub providers: _0,
        pub sufficients: _0,
        pub data: _1,
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct EventRecord<_0, _1> {
        pub phase: runtime_types::frame_system::Phase,
        pub event: _0,
        pub topics: ::std::vec::Vec<_1>,
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct LastRuntimeUpgradeInfo {
        #[codec(compact)]
        pub spec_version: ::core::primitive::u32,
        pub spec_name: ::std::string::String,
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub enum Phase {
        #[codec(index = 0)]
        ApplyExtrinsic(::core::primitive::u32),
        #[codec(index = 1)]
        Finalization,
        #[codec(index = 2)]
        Initialization,
      }
    }
    pub mod lagoon_runtime {
      use super::runtime_types;
      pub mod config {
        use super::runtime_types;
        pub mod consensus {
          use super::runtime_types;
          #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
          pub struct NposCompactSolution16 {
            pub votes1: ::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u16)>,
            pub votes2: ::std::vec::Vec<(
              ::core::primitive::u32,
              (
                ::core::primitive::u16,
                runtime_types::sp_arithmetic::per_things::PerU16,
              ),
              ::core::primitive::u16,
            )>,
            pub votes3: ::std::vec::Vec<(
              ::core::primitive::u32,
              [(
                ::core::primitive::u16,
                runtime_types::sp_arithmetic::per_things::PerU16,
              ); 2usize],
              ::core::primitive::u16,
            )>,
            pub votes4: ::std::vec::Vec<(
              ::core::primitive::u32,
              [(
                ::core::primitive::u16,
                runtime_types::sp_arithmetic::per_things::PerU16,
              ); 3usize],
              ::core::primitive::u16,
            )>,
            pub votes5: ::std::vec::Vec<(
              ::core::primitive::u32,
              [(
                ::core::primitive::u16,
                runtime_types::sp_arithmetic::per_things::PerU16,
              ); 4usize],
              ::core::primitive::u16,
            )>,
            pub votes6: ::std::vec::Vec<(
              ::core::primitive::u32,
              [(
                ::core::primitive::u16,
                runtime_types::sp_arithmetic::per_things::PerU16,
              ); 5usize],
              ::core::primitive::u16,
            )>,
            pub votes7: ::std::vec::Vec<(
              ::core::primitive::u32,
              [(
                ::core::primitive::u16,
                runtime_types::sp_arithmetic::per_things::PerU16,
              ); 6usize],
              ::core::primitive::u16,
            )>,
            pub votes8: ::std::vec::Vec<(
              ::core::primitive::u32,
              [(
                ::core::primitive::u16,
                runtime_types::sp_arithmetic::per_things::PerU16,
              ); 7usize],
              ::core::primitive::u16,
            )>,
            pub votes9: ::std::vec::Vec<(
              ::core::primitive::u32,
              [(
                ::core::primitive::u16,
                runtime_types::sp_arithmetic::per_things::PerU16,
              ); 8usize],
              ::core::primitive::u16,
            )>,
            pub votes10: ::std::vec::Vec<(
              ::core::primitive::u32,
              [(
                ::core::primitive::u16,
                runtime_types::sp_arithmetic::per_things::PerU16,
              ); 9usize],
              ::core::primitive::u16,
            )>,
            pub votes11: ::std::vec::Vec<(
              ::core::primitive::u32,
              [(
                ::core::primitive::u16,
                runtime_types::sp_arithmetic::per_things::PerU16,
              ); 10usize],
              ::core::primitive::u16,
            )>,
            pub votes12: ::std::vec::Vec<(
              ::core::primitive::u32,
              [(
                ::core::primitive::u16,
                runtime_types::sp_arithmetic::per_things::PerU16,
              ); 11usize],
              ::core::primitive::u16,
            )>,
            pub votes13: ::std::vec::Vec<(
              ::core::primitive::u32,
              [(
                ::core::primitive::u16,
                runtime_types::sp_arithmetic::per_things::PerU16,
              ); 12usize],
              ::core::primitive::u16,
            )>,
            pub votes14: ::std::vec::Vec<(
              ::core::primitive::u32,
              [(
                ::core::primitive::u16,
                runtime_types::sp_arithmetic::per_things::PerU16,
              ); 13usize],
              ::core::primitive::u16,
            )>,
            pub votes15: ::std::vec::Vec<(
              ::core::primitive::u32,
              [(
                ::core::primitive::u16,
                runtime_types::sp_arithmetic::per_things::PerU16,
              ); 14usize],
              ::core::primitive::u16,
            )>,
            pub votes16: ::std::vec::Vec<(
              ::core::primitive::u32,
              [(
                ::core::primitive::u16,
                runtime_types::sp_arithmetic::per_things::PerU16,
              ); 15usize],
              ::core::primitive::u16,
            )>,
          }
        }
        pub mod proxy {
          use super::runtime_types;
          #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
          pub enum ProxyType {
            #[codec(index = 0)]
            Any,
            #[codec(index = 1)]
            NonTransfer,
            #[codec(index = 2)]
            Governance,
            #[codec(index = 3)]
            Staking,
          }
        }
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub enum Call {
        #[codec(index = 0)]
        System(runtime_types::frame_system::pallet::Call),
        #[codec(index = 1)]
        Babe(runtime_types::pallet_babe::pallet::Call),
        #[codec(index = 2)]
        Timestamp(runtime_types::pallet_timestamp::pallet::Call),
        #[codec(index = 3)]
        Indices(runtime_types::pallet_indices::pallet::Call),
        #[codec(index = 4)]
        Balances(runtime_types::pallet_balances::pallet::Call),
        #[codec(index = 6)]
        Authorship(runtime_types::pallet_authorship::pallet::Call),
        #[codec(index = 7)]
        Staking(runtime_types::pallet_staking::pallet::pallet::Call),
        #[codec(index = 10)]
        Session(runtime_types::pallet_session::pallet::Call),
        #[codec(index = 11)]
        Grandpa(runtime_types::pallet_grandpa::pallet::Call),
        #[codec(index = 12)]
        ImOnline(runtime_types::pallet_im_online::pallet::Call),
        #[codec(index = 14)]
        Council(runtime_types::pallet_collective::pallet::Call),
        #[codec(index = 15)]
        TechnicalCommittee(runtime_types::pallet_collective::pallet::Call),
        #[codec(index = 16)]
        Elections(runtime_types::pallet_elections_phragmen::pallet::Call),
        #[codec(index = 17)]
        TechnicalMembership(runtime_types::pallet_membership::pallet::Call),
        #[codec(index = 18)]
        Treasury(runtime_types::pallet_treasury::pallet::Call),
        #[codec(index = 19)]
        Utility(runtime_types::pallet_utility::pallet::Call),
        #[codec(index = 20)]
        Identity(runtime_types::pallet_identity::pallet::Call),
        #[codec(index = 21)]
        ElectionProviderMultiPhase(
          runtime_types::pallet_election_provider_multi_phase::pallet::Call,
        ),
        #[codec(index = 22)]
        Recovery(runtime_types::pallet_recovery::pallet::Call),
        #[codec(index = 23)]
        Scheduler(runtime_types::pallet_scheduler::pallet::Call),
        #[codec(index = 24)]
        Proxy(runtime_types::pallet_proxy::pallet::Call),
        #[codec(index = 25)]
        Multisig(runtime_types::pallet_multisig::pallet::Call),
        #[codec(index = 26)]
        Bounties(runtime_types::pallet_bounties::pallet::Call),
        #[codec(index = 27)]
        Assets(runtime_types::pallet_assets::pallet::Call),
        #[codec(index = 28)]
        BagsList(runtime_types::pallet_bags_list::pallet::Call),
        #[codec(index = 29)]
        Preimage(runtime_types::pallet_preimage::pallet::Call),
        #[codec(index = 30)]
        Sudo(runtime_types::pallet_sudo::pallet::Call),
        #[codec(index = 50)]
        Tidefi(runtime_types::pallet_tidefi::pallet::Call),
        #[codec(index = 51)]
        TidefiStaking(runtime_types::pallet_tidefi_stake::pallet::Call),
        #[codec(index = 52)]
        Quorum(runtime_types::pallet_quorum::pallet::Call),
        #[codec(index = 53)]
        Oracle(runtime_types::pallet_oracle::pallet::Call),
        #[codec(index = 54)]
        Security(runtime_types::pallet_security::pallet::Call),
        #[codec(index = 55)]
        Fees(runtime_types::pallet_fees::pallet::Call),
        #[codec(index = 56)]
        AssetRegistry(runtime_types::pallet_asset_registry::pallet::Call),
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub enum Event {
        #[codec(index = 0)]
        System(runtime_types::frame_system::pallet::Event),
        #[codec(index = 3)]
        Indices(runtime_types::pallet_indices::pallet::Event),
        #[codec(index = 4)]
        Balances(runtime_types::pallet_balances::pallet::Event),
        #[codec(index = 7)]
        Staking(runtime_types::pallet_staking::pallet::pallet::Event),
        #[codec(index = 8)]
        Offences(runtime_types::pallet_offences::pallet::Event),
        #[codec(index = 10)]
        Session(runtime_types::pallet_session::pallet::Event),
        #[codec(index = 11)]
        Grandpa(runtime_types::pallet_grandpa::pallet::Event),
        #[codec(index = 12)]
        ImOnline(runtime_types::pallet_im_online::pallet::Event),
        #[codec(index = 14)]
        Council(runtime_types::pallet_collective::pallet::Event),
        #[codec(index = 15)]
        TechnicalCommittee(runtime_types::pallet_collective::pallet::Event),
        #[codec(index = 16)]
        Elections(runtime_types::pallet_elections_phragmen::pallet::Event),
        #[codec(index = 17)]
        TechnicalMembership(runtime_types::pallet_membership::pallet::Event),
        #[codec(index = 18)]
        Treasury(runtime_types::pallet_treasury::pallet::Event),
        #[codec(index = 19)]
        Utility(runtime_types::pallet_utility::pallet::Event),
        #[codec(index = 20)]
        Identity(runtime_types::pallet_identity::pallet::Event),
        #[codec(index = 21)]
        ElectionProviderMultiPhase(
          runtime_types::pallet_election_provider_multi_phase::pallet::Event,
        ),
        #[codec(index = 22)]
        Recovery(runtime_types::pallet_recovery::pallet::Event),
        #[codec(index = 23)]
        Scheduler(runtime_types::pallet_scheduler::pallet::Event),
        #[codec(index = 24)]
        Proxy(runtime_types::pallet_proxy::pallet::Event),
        #[codec(index = 25)]
        Multisig(runtime_types::pallet_multisig::pallet::Event),
        #[codec(index = 26)]
        Bounties(runtime_types::pallet_bounties::pallet::Event),
        #[codec(index = 27)]
        Assets(runtime_types::pallet_assets::pallet::Event),
        #[codec(index = 28)]
        BagsList(runtime_types::pallet_bags_list::pallet::Event),
        #[codec(index = 29)]
        Preimage(runtime_types::pallet_preimage::pallet::Event),
        #[codec(index = 30)]
        Sudo(runtime_types::pallet_sudo::pallet::Event),
        #[codec(index = 50)]
        Tidefi(runtime_types::pallet_tidefi::pallet::Event),
        #[codec(index = 51)]
        TidefiStaking(runtime_types::pallet_tidefi_stake::pallet::Event),
        #[codec(index = 52)]
        Quorum(runtime_types::pallet_quorum::pallet::Event),
        #[codec(index = 53)]
        Oracle(runtime_types::pallet_oracle::pallet::Event),
        #[codec(index = 54)]
        Security(runtime_types::pallet_security::pallet::Event),
        #[codec(index = 55)]
        Fees(runtime_types::pallet_fees::pallet::Event),
        #[codec(index = 56)]
        AssetRegistry(runtime_types::pallet_asset_registry::pallet::Event),
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub enum OriginCaller {
        #[codec(index = 0)]
        system(
          runtime_types::frame_support::dispatch::RawOrigin<::subxt::sp_core::crypto::AccountId32>,
        ),
        #[codec(index = 14)]
        Council(runtime_types::pallet_collective::RawOrigin<::subxt::sp_core::crypto::AccountId32>),
        #[codec(index = 15)]
        TechnicalCommittee(
          runtime_types::pallet_collective::RawOrigin<::subxt::sp_core::crypto::AccountId32>,
        ),
        #[codec(index = 3)]
        Void(runtime_types::sp_core::Void),
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Runtime;
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct SessionKeys {
        pub grandpa: runtime_types::sp_finality_grandpa::app::Public,
        pub babe: runtime_types::sp_consensus_babe::app::Public,
        pub im_online: runtime_types::pallet_im_online::sr25519::app_sr25519::Public,
        pub authority_discovery: runtime_types::sp_authority_discovery::app::Public,
      }
    }
    pub mod pallet_asset_registry {
      use super::runtime_types;
      pub mod pallet {
        use super::runtime_types;
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Call {
          #[codec(index = 0)]
          register {
            currency_id: runtime_types::tidefi_primitives::CurrencyId,
            name: ::std::vec::Vec<::core::primitive::u8>,
            symbol: ::std::vec::Vec<::core::primitive::u8>,
            decimals: ::core::primitive::u8,
            existential_deposit: ::core::primitive::u128,
          },
          #[codec(index = 1)]
          set_status {
            currency_id: runtime_types::tidefi_primitives::CurrencyId,
            is_enabled: ::core::primitive::bool,
          },
        }
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Error {
          #[codec(index = 0)]
          AccessDenied,
          #[codec(index = 1)]
          AssetNotRegistered,
          #[codec(index = 2)]
          NoStatusChangeRequested,
          #[codec(index = 3)]
          AssetAlreadyRegistered,
          #[codec(index = 4)]
          CurrencyIdNotValid,
        }
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Event {
          #[codec(index = 0)]
          Registered(runtime_types::tidefi_primitives::CurrencyId),
          #[codec(index = 1)]
          StatusChanged(
            runtime_types::tidefi_primitives::CurrencyId,
            ::core::primitive::bool,
          ),
        }
      }
    }
    pub mod pallet_assets {
      use super::runtime_types;
      pub mod pallet {
        use super::runtime_types;
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Call {
          #[codec(index = 0)]
          create {
            #[codec(compact)]
            id: ::core::primitive::u32,
            admin: ::subxt::sp_runtime::MultiAddress<
              ::subxt::sp_core::crypto::AccountId32,
              ::core::primitive::u32,
            >,
            min_balance: ::core::primitive::u128,
          },
          #[codec(index = 1)]
          force_create {
            #[codec(compact)]
            id: ::core::primitive::u32,
            owner: ::subxt::sp_runtime::MultiAddress<
              ::subxt::sp_core::crypto::AccountId32,
              ::core::primitive::u32,
            >,
            is_sufficient: ::core::primitive::bool,
            #[codec(compact)]
            min_balance: ::core::primitive::u128,
          },
          #[codec(index = 2)]
          destroy {
            #[codec(compact)]
            id: ::core::primitive::u32,
            witness: runtime_types::pallet_assets::types::DestroyWitness,
          },
          #[codec(index = 3)]
          mint {
            #[codec(compact)]
            id: ::core::primitive::u32,
            beneficiary: ::subxt::sp_runtime::MultiAddress<
              ::subxt::sp_core::crypto::AccountId32,
              ::core::primitive::u32,
            >,
            #[codec(compact)]
            amount: ::core::primitive::u128,
          },
          #[codec(index = 4)]
          burn {
            #[codec(compact)]
            id: ::core::primitive::u32,
            who: ::subxt::sp_runtime::MultiAddress<
              ::subxt::sp_core::crypto::AccountId32,
              ::core::primitive::u32,
            >,
            #[codec(compact)]
            amount: ::core::primitive::u128,
          },
          #[codec(index = 5)]
          transfer {
            #[codec(compact)]
            id: ::core::primitive::u32,
            target: ::subxt::sp_runtime::MultiAddress<
              ::subxt::sp_core::crypto::AccountId32,
              ::core::primitive::u32,
            >,
            #[codec(compact)]
            amount: ::core::primitive::u128,
          },
          #[codec(index = 6)]
          transfer_keep_alive {
            #[codec(compact)]
            id: ::core::primitive::u32,
            target: ::subxt::sp_runtime::MultiAddress<
              ::subxt::sp_core::crypto::AccountId32,
              ::core::primitive::u32,
            >,
            #[codec(compact)]
            amount: ::core::primitive::u128,
          },
          #[codec(index = 7)]
          force_transfer {
            #[codec(compact)]
            id: ::core::primitive::u32,
            source: ::subxt::sp_runtime::MultiAddress<
              ::subxt::sp_core::crypto::AccountId32,
              ::core::primitive::u32,
            >,
            dest: ::subxt::sp_runtime::MultiAddress<
              ::subxt::sp_core::crypto::AccountId32,
              ::core::primitive::u32,
            >,
            #[codec(compact)]
            amount: ::core::primitive::u128,
          },
          #[codec(index = 8)]
          freeze {
            #[codec(compact)]
            id: ::core::primitive::u32,
            who: ::subxt::sp_runtime::MultiAddress<
              ::subxt::sp_core::crypto::AccountId32,
              ::core::primitive::u32,
            >,
          },
          #[codec(index = 9)]
          thaw {
            #[codec(compact)]
            id: ::core::primitive::u32,
            who: ::subxt::sp_runtime::MultiAddress<
              ::subxt::sp_core::crypto::AccountId32,
              ::core::primitive::u32,
            >,
          },
          #[codec(index = 10)]
          freeze_asset {
            #[codec(compact)]
            id: ::core::primitive::u32,
          },
          #[codec(index = 11)]
          thaw_asset {
            #[codec(compact)]
            id: ::core::primitive::u32,
          },
          #[codec(index = 12)]
          transfer_ownership {
            #[codec(compact)]
            id: ::core::primitive::u32,
            owner: ::subxt::sp_runtime::MultiAddress<
              ::subxt::sp_core::crypto::AccountId32,
              ::core::primitive::u32,
            >,
          },
          #[codec(index = 13)]
          set_team {
            #[codec(compact)]
            id: ::core::primitive::u32,
            issuer: ::subxt::sp_runtime::MultiAddress<
              ::subxt::sp_core::crypto::AccountId32,
              ::core::primitive::u32,
            >,
            admin: ::subxt::sp_runtime::MultiAddress<
              ::subxt::sp_core::crypto::AccountId32,
              ::core::primitive::u32,
            >,
            freezer: ::subxt::sp_runtime::MultiAddress<
              ::subxt::sp_core::crypto::AccountId32,
              ::core::primitive::u32,
            >,
          },
          #[codec(index = 14)]
          set_metadata {
            #[codec(compact)]
            id: ::core::primitive::u32,
            name: ::std::vec::Vec<::core::primitive::u8>,
            symbol: ::std::vec::Vec<::core::primitive::u8>,
            decimals: ::core::primitive::u8,
          },
          #[codec(index = 15)]
          clear_metadata {
            #[codec(compact)]
            id: ::core::primitive::u32,
          },
          #[codec(index = 16)]
          force_set_metadata {
            #[codec(compact)]
            id: ::core::primitive::u32,
            name: ::std::vec::Vec<::core::primitive::u8>,
            symbol: ::std::vec::Vec<::core::primitive::u8>,
            decimals: ::core::primitive::u8,
            is_frozen: ::core::primitive::bool,
          },
          #[codec(index = 17)]
          force_clear_metadata {
            #[codec(compact)]
            id: ::core::primitive::u32,
          },
          #[codec(index = 18)]
          force_asset_status {
            #[codec(compact)]
            id: ::core::primitive::u32,
            owner: ::subxt::sp_runtime::MultiAddress<
              ::subxt::sp_core::crypto::AccountId32,
              ::core::primitive::u32,
            >,
            issuer: ::subxt::sp_runtime::MultiAddress<
              ::subxt::sp_core::crypto::AccountId32,
              ::core::primitive::u32,
            >,
            admin: ::subxt::sp_runtime::MultiAddress<
              ::subxt::sp_core::crypto::AccountId32,
              ::core::primitive::u32,
            >,
            freezer: ::subxt::sp_runtime::MultiAddress<
              ::subxt::sp_core::crypto::AccountId32,
              ::core::primitive::u32,
            >,
            #[codec(compact)]
            min_balance: ::core::primitive::u128,
            is_sufficient: ::core::primitive::bool,
            is_frozen: ::core::primitive::bool,
          },
          #[codec(index = 19)]
          approve_transfer {
            #[codec(compact)]
            id: ::core::primitive::u32,
            delegate: ::subxt::sp_runtime::MultiAddress<
              ::subxt::sp_core::crypto::AccountId32,
              ::core::primitive::u32,
            >,
            #[codec(compact)]
            amount: ::core::primitive::u128,
          },
          #[codec(index = 20)]
          cancel_approval {
            #[codec(compact)]
            id: ::core::primitive::u32,
            delegate: ::subxt::sp_runtime::MultiAddress<
              ::subxt::sp_core::crypto::AccountId32,
              ::core::primitive::u32,
            >,
          },
          #[codec(index = 21)]
          force_cancel_approval {
            #[codec(compact)]
            id: ::core::primitive::u32,
            owner: ::subxt::sp_runtime::MultiAddress<
              ::subxt::sp_core::crypto::AccountId32,
              ::core::primitive::u32,
            >,
            delegate: ::subxt::sp_runtime::MultiAddress<
              ::subxt::sp_core::crypto::AccountId32,
              ::core::primitive::u32,
            >,
          },
          #[codec(index = 22)]
          transfer_approved {
            #[codec(compact)]
            id: ::core::primitive::u32,
            owner: ::subxt::sp_runtime::MultiAddress<
              ::subxt::sp_core::crypto::AccountId32,
              ::core::primitive::u32,
            >,
            destination: ::subxt::sp_runtime::MultiAddress<
              ::subxt::sp_core::crypto::AccountId32,
              ::core::primitive::u32,
            >,
            #[codec(compact)]
            amount: ::core::primitive::u128,
          },
          #[codec(index = 23)]
          touch {
            #[codec(compact)]
            id: ::core::primitive::u32,
          },
          #[codec(index = 24)]
          refund {
            #[codec(compact)]
            id: ::core::primitive::u32,
            allow_burn: ::core::primitive::bool,
          },
        }
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Error {
          #[codec(index = 0)]
          BalanceLow,
          #[codec(index = 1)]
          NoAccount,
          #[codec(index = 2)]
          NoPermission,
          #[codec(index = 3)]
          Unknown,
          #[codec(index = 4)]
          Frozen,
          #[codec(index = 5)]
          InUse,
          #[codec(index = 6)]
          BadWitness,
          #[codec(index = 7)]
          MinBalanceZero,
          #[codec(index = 8)]
          NoProvider,
          #[codec(index = 9)]
          BadMetadata,
          #[codec(index = 10)]
          Unapproved,
          #[codec(index = 11)]
          WouldDie,
          #[codec(index = 12)]
          AlreadyExists,
          #[codec(index = 13)]
          NoDeposit,
          #[codec(index = 14)]
          WouldBurn,
        }
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Event {
          #[codec(index = 0)]
          Created {
            asset_id: ::core::primitive::u32,
            creator: ::subxt::sp_core::crypto::AccountId32,
            owner: ::subxt::sp_core::crypto::AccountId32,
          },
          #[codec(index = 1)]
          Issued {
            asset_id: ::core::primitive::u32,
            owner: ::subxt::sp_core::crypto::AccountId32,
            total_supply: ::core::primitive::u128,
          },
          #[codec(index = 2)]
          Transferred {
            asset_id: ::core::primitive::u32,
            from: ::subxt::sp_core::crypto::AccountId32,
            to: ::subxt::sp_core::crypto::AccountId32,
            amount: ::core::primitive::u128,
          },
          #[codec(index = 3)]
          Burned {
            asset_id: ::core::primitive::u32,
            owner: ::subxt::sp_core::crypto::AccountId32,
            balance: ::core::primitive::u128,
          },
          #[codec(index = 4)]
          TeamChanged {
            asset_id: ::core::primitive::u32,
            issuer: ::subxt::sp_core::crypto::AccountId32,
            admin: ::subxt::sp_core::crypto::AccountId32,
            freezer: ::subxt::sp_core::crypto::AccountId32,
          },
          #[codec(index = 5)]
          OwnerChanged {
            asset_id: ::core::primitive::u32,
            owner: ::subxt::sp_core::crypto::AccountId32,
          },
          #[codec(index = 6)]
          Frozen {
            asset_id: ::core::primitive::u32,
            who: ::subxt::sp_core::crypto::AccountId32,
          },
          #[codec(index = 7)]
          Thawed {
            asset_id: ::core::primitive::u32,
            who: ::subxt::sp_core::crypto::AccountId32,
          },
          #[codec(index = 8)]
          AssetFrozen { asset_id: ::core::primitive::u32 },
          #[codec(index = 9)]
          AssetThawed { asset_id: ::core::primitive::u32 },
          #[codec(index = 10)]
          Destroyed { asset_id: ::core::primitive::u32 },
          #[codec(index = 11)]
          ForceCreated {
            asset_id: ::core::primitive::u32,
            owner: ::subxt::sp_core::crypto::AccountId32,
          },
          #[codec(index = 12)]
          MetadataSet {
            asset_id: ::core::primitive::u32,
            name: ::std::vec::Vec<::core::primitive::u8>,
            symbol: ::std::vec::Vec<::core::primitive::u8>,
            decimals: ::core::primitive::u8,
            is_frozen: ::core::primitive::bool,
          },
          #[codec(index = 13)]
          MetadataCleared { asset_id: ::core::primitive::u32 },
          #[codec(index = 14)]
          ApprovedTransfer {
            asset_id: ::core::primitive::u32,
            source: ::subxt::sp_core::crypto::AccountId32,
            delegate: ::subxt::sp_core::crypto::AccountId32,
            amount: ::core::primitive::u128,
          },
          #[codec(index = 15)]
          ApprovalCancelled {
            asset_id: ::core::primitive::u32,
            owner: ::subxt::sp_core::crypto::AccountId32,
            delegate: ::subxt::sp_core::crypto::AccountId32,
          },
          #[codec(index = 16)]
          TransferredApproved {
            asset_id: ::core::primitive::u32,
            owner: ::subxt::sp_core::crypto::AccountId32,
            delegate: ::subxt::sp_core::crypto::AccountId32,
            destination: ::subxt::sp_core::crypto::AccountId32,
            amount: ::core::primitive::u128,
          },
          #[codec(index = 17)]
          AssetStatusChanged { asset_id: ::core::primitive::u32 },
        }
      }
      pub mod types {
        use super::runtime_types;
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub struct Approval<_0, _1> {
          pub amount: _0,
          pub deposit: _0,
          #[codec(skip)]
          pub __subxt_unused_type_params: ::core::marker::PhantomData<_1>,
        }
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub struct AssetAccount<_0, _1, _2> {
          pub balance: _0,
          pub reserved: _0,
          pub is_frozen: ::core::primitive::bool,
          pub reason: runtime_types::pallet_assets::types::ExistenceReason<_0>,
          pub extra: _2,
          #[codec(skip)]
          pub __subxt_unused_type_params: ::core::marker::PhantomData<_1>,
        }
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub struct AssetDetails<_0, _1, _2> {
          pub owner: _1,
          pub issuer: _1,
          pub admin: _1,
          pub freezer: _1,
          pub supply: _0,
          pub deposit: _0,
          pub min_balance: _0,
          pub is_sufficient: ::core::primitive::bool,
          pub accounts: ::core::primitive::u32,
          pub sufficients: ::core::primitive::u32,
          pub approvals: ::core::primitive::u32,
          pub is_frozen: ::core::primitive::bool,
          #[codec(skip)]
          pub __subxt_unused_type_params: ::core::marker::PhantomData<_2>,
        }
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub struct AssetMetadata<_0, _1> {
          pub deposit: _0,
          pub name: _1,
          pub symbol: _1,
          pub decimals: ::core::primitive::u8,
          pub is_frozen: ::core::primitive::bool,
        }
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub struct DestroyWitness {
          #[codec(compact)]
          pub accounts: ::core::primitive::u32,
          #[codec(compact)]
          pub sufficients: ::core::primitive::u32,
          #[codec(compact)]
          pub approvals: ::core::primitive::u32,
        }
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum ExistenceReason<_0> {
          #[codec(index = 0)]
          Consumer,
          #[codec(index = 1)]
          Sufficient,
          #[codec(index = 2)]
          DepositHeld(_0),
          #[codec(index = 3)]
          DepositRefunded,
        }
      }
    }
    pub mod pallet_authorship {
      use super::runtime_types;
      pub mod pallet {
        use super::runtime_types;
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Call {
          #[codec(index = 0)]
          set_uncles {
            new_uncles: ::std::vec::Vec<
              runtime_types::sp_runtime::generic::header::Header<
                ::core::primitive::u32,
                runtime_types::sp_runtime::traits::BlakeTwo256,
              >,
            >,
          },
        }
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Error {
          #[codec(index = 0)]
          InvalidUncleParent,
          #[codec(index = 1)]
          UnclesAlreadySet,
          #[codec(index = 2)]
          TooManyUncles,
          #[codec(index = 3)]
          GenesisUncle,
          #[codec(index = 4)]
          TooHighUncle,
          #[codec(index = 5)]
          UncleAlreadyIncluded,
          #[codec(index = 6)]
          OldUncle,
        }
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub enum UncleEntryItem<_0, _1, _2> {
        #[codec(index = 0)]
        InclusionHeight(_0),
        #[codec(index = 1)]
        Uncle(_1, ::core::option::Option<_2>),
      }
    }
    pub mod pallet_babe {
      use super::runtime_types;
      pub mod pallet {
        use super::runtime_types;
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Call {
          #[codec(index = 0)]
          report_equivocation {
            equivocation_proof: ::std::boxed::Box<
              runtime_types::sp_consensus_slots::EquivocationProof<
                runtime_types::sp_runtime::generic::header::Header<
                  ::core::primitive::u32,
                  runtime_types::sp_runtime::traits::BlakeTwo256,
                >,
                runtime_types::sp_consensus_babe::app::Public,
              >,
            >,
            key_owner_proof: runtime_types::sp_session::MembershipProof,
          },
          #[codec(index = 1)]
          report_equivocation_unsigned {
            equivocation_proof: ::std::boxed::Box<
              runtime_types::sp_consensus_slots::EquivocationProof<
                runtime_types::sp_runtime::generic::header::Header<
                  ::core::primitive::u32,
                  runtime_types::sp_runtime::traits::BlakeTwo256,
                >,
                runtime_types::sp_consensus_babe::app::Public,
              >,
            >,
            key_owner_proof: runtime_types::sp_session::MembershipProof,
          },
          #[codec(index = 2)]
          plan_config_change {
            config: runtime_types::sp_consensus_babe::digests::NextConfigDescriptor,
          },
        }
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Error {
          #[codec(index = 0)]
          InvalidEquivocationProof,
          #[codec(index = 1)]
          InvalidKeyOwnershipProof,
          #[codec(index = 2)]
          DuplicateOffenceReport,
        }
      }
    }
    pub mod pallet_bags_list {
      use super::runtime_types;
      pub mod list {
        use super::runtime_types;
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub struct Bag {
          pub head: ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
          pub tail: ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
        }
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub struct Node {
          pub id: ::subxt::sp_core::crypto::AccountId32,
          pub prev: ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
          pub next: ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
          pub bag_upper: ::core::primitive::u64,
        }
      }
      pub mod pallet {
        use super::runtime_types;
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Call {
          #[codec(index = 0)]
          rebag {
            dislocated: ::subxt::sp_core::crypto::AccountId32,
          },
          #[codec(index = 1)]
          put_in_front_of {
            lighter: ::subxt::sp_core::crypto::AccountId32,
          },
        }
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Error {
          #[codec(index = 0)]
          NotInSameBag,
          #[codec(index = 1)]
          IdNotFound,
          #[codec(index = 2)]
          NotHeavier,
        }
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Event {
          #[codec(index = 0)]
          Rebagged {
            who: ::subxt::sp_core::crypto::AccountId32,
            from: ::core::primitive::u64,
            to: ::core::primitive::u64,
          },
        }
      }
    }
    pub mod pallet_balances {
      use super::runtime_types;
      pub mod pallet {
        use super::runtime_types;
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Call {
          #[codec(index = 0)]
          transfer {
            dest: ::subxt::sp_runtime::MultiAddress<
              ::subxt::sp_core::crypto::AccountId32,
              ::core::primitive::u32,
            >,
            #[codec(compact)]
            value: ::core::primitive::u128,
          },
          #[codec(index = 1)]
          set_balance {
            who: ::subxt::sp_runtime::MultiAddress<
              ::subxt::sp_core::crypto::AccountId32,
              ::core::primitive::u32,
            >,
            #[codec(compact)]
            new_free: ::core::primitive::u128,
            #[codec(compact)]
            new_reserved: ::core::primitive::u128,
          },
          #[codec(index = 2)]
          force_transfer {
            source: ::subxt::sp_runtime::MultiAddress<
              ::subxt::sp_core::crypto::AccountId32,
              ::core::primitive::u32,
            >,
            dest: ::subxt::sp_runtime::MultiAddress<
              ::subxt::sp_core::crypto::AccountId32,
              ::core::primitive::u32,
            >,
            #[codec(compact)]
            value: ::core::primitive::u128,
          },
          #[codec(index = 3)]
          transfer_keep_alive {
            dest: ::subxt::sp_runtime::MultiAddress<
              ::subxt::sp_core::crypto::AccountId32,
              ::core::primitive::u32,
            >,
            #[codec(compact)]
            value: ::core::primitive::u128,
          },
          #[codec(index = 4)]
          transfer_all {
            dest: ::subxt::sp_runtime::MultiAddress<
              ::subxt::sp_core::crypto::AccountId32,
              ::core::primitive::u32,
            >,
            keep_alive: ::core::primitive::bool,
          },
          #[codec(index = 5)]
          force_unreserve {
            who: ::subxt::sp_runtime::MultiAddress<
              ::subxt::sp_core::crypto::AccountId32,
              ::core::primitive::u32,
            >,
            amount: ::core::primitive::u128,
          },
        }
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Error {
          #[codec(index = 0)]
          VestingBalance,
          #[codec(index = 1)]
          LiquidityRestrictions,
          #[codec(index = 2)]
          InsufficientBalance,
          #[codec(index = 3)]
          ExistentialDeposit,
          #[codec(index = 4)]
          KeepAlive,
          #[codec(index = 5)]
          ExistingVestingSchedule,
          #[codec(index = 6)]
          DeadAccount,
          #[codec(index = 7)]
          TooManyReserves,
        }
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Event {
          #[codec(index = 0)]
          Endowed {
            account: ::subxt::sp_core::crypto::AccountId32,
            free_balance: ::core::primitive::u128,
          },
          #[codec(index = 1)]
          DustLost {
            account: ::subxt::sp_core::crypto::AccountId32,
            amount: ::core::primitive::u128,
          },
          #[codec(index = 2)]
          Transfer {
            from: ::subxt::sp_core::crypto::AccountId32,
            to: ::subxt::sp_core::crypto::AccountId32,
            amount: ::core::primitive::u128,
          },
          #[codec(index = 3)]
          BalanceSet {
            who: ::subxt::sp_core::crypto::AccountId32,
            free: ::core::primitive::u128,
            reserved: ::core::primitive::u128,
          },
          #[codec(index = 4)]
          Reserved {
            who: ::subxt::sp_core::crypto::AccountId32,
            amount: ::core::primitive::u128,
          },
          #[codec(index = 5)]
          Unreserved {
            who: ::subxt::sp_core::crypto::AccountId32,
            amount: ::core::primitive::u128,
          },
          #[codec(index = 6)]
          ReserveRepatriated {
            from: ::subxt::sp_core::crypto::AccountId32,
            to: ::subxt::sp_core::crypto::AccountId32,
            amount: ::core::primitive::u128,
            destination_status: runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
          },
          #[codec(index = 7)]
          Deposit {
            who: ::subxt::sp_core::crypto::AccountId32,
            amount: ::core::primitive::u128,
          },
          #[codec(index = 8)]
          Withdraw {
            who: ::subxt::sp_core::crypto::AccountId32,
            amount: ::core::primitive::u128,
          },
          #[codec(index = 9)]
          Slashed {
            who: ::subxt::sp_core::crypto::AccountId32,
            amount: ::core::primitive::u128,
          },
        }
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct AccountData<_0> {
        pub free: _0,
        pub reserved: _0,
        pub misc_frozen: _0,
        pub fee_frozen: _0,
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct BalanceLock<_0> {
        pub id: [::core::primitive::u8; 8usize],
        pub amount: _0,
        pub reasons: runtime_types::pallet_balances::Reasons,
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub enum Reasons {
        #[codec(index = 0)]
        Fee,
        #[codec(index = 1)]
        Misc,
        #[codec(index = 2)]
        All,
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub enum Releases {
        #[codec(index = 0)]
        V1_0_0,
        #[codec(index = 1)]
        V2_0_0,
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct ReserveData<_0, _1> {
        pub id: _0,
        pub amount: _1,
      }
    }
    pub mod pallet_bounties {
      use super::runtime_types;
      pub mod pallet {
        use super::runtime_types;
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Call {
          #[codec(index = 0)]
          propose_bounty {
            #[codec(compact)]
            value: ::core::primitive::u128,
            description: ::std::vec::Vec<::core::primitive::u8>,
          },
          #[codec(index = 1)]
          approve_bounty {
            #[codec(compact)]
            bounty_id: ::core::primitive::u32,
          },
          #[codec(index = 2)]
          propose_curator {
            #[codec(compact)]
            bounty_id: ::core::primitive::u32,
            curator: ::subxt::sp_runtime::MultiAddress<
              ::subxt::sp_core::crypto::AccountId32,
              ::core::primitive::u32,
            >,
            #[codec(compact)]
            fee: ::core::primitive::u128,
          },
          #[codec(index = 3)]
          unassign_curator {
            #[codec(compact)]
            bounty_id: ::core::primitive::u32,
          },
          #[codec(index = 4)]
          accept_curator {
            #[codec(compact)]
            bounty_id: ::core::primitive::u32,
          },
          #[codec(index = 5)]
          award_bounty {
            #[codec(compact)]
            bounty_id: ::core::primitive::u32,
            beneficiary: ::subxt::sp_runtime::MultiAddress<
              ::subxt::sp_core::crypto::AccountId32,
              ::core::primitive::u32,
            >,
          },
          #[codec(index = 6)]
          claim_bounty {
            #[codec(compact)]
            bounty_id: ::core::primitive::u32,
          },
          #[codec(index = 7)]
          close_bounty {
            #[codec(compact)]
            bounty_id: ::core::primitive::u32,
          },
          #[codec(index = 8)]
          extend_bounty_expiry {
            #[codec(compact)]
            bounty_id: ::core::primitive::u32,
            remark: ::std::vec::Vec<::core::primitive::u8>,
          },
        }
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Error {
          #[codec(index = 0)]
          InsufficientProposersBalance,
          #[codec(index = 1)]
          InvalidIndex,
          #[codec(index = 2)]
          ReasonTooBig,
          #[codec(index = 3)]
          UnexpectedStatus,
          #[codec(index = 4)]
          RequireCurator,
          #[codec(index = 5)]
          InvalidValue,
          #[codec(index = 6)]
          InvalidFee,
          #[codec(index = 7)]
          PendingPayout,
          #[codec(index = 8)]
          Premature,
          #[codec(index = 9)]
          HasActiveChildBounty,
          #[codec(index = 10)]
          TooManyQueued,
        }
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Event {
          #[codec(index = 0)]
          BountyProposed { index: ::core::primitive::u32 },
          #[codec(index = 1)]
          BountyRejected {
            index: ::core::primitive::u32,
            bond: ::core::primitive::u128,
          },
          #[codec(index = 2)]
          BountyBecameActive { index: ::core::primitive::u32 },
          #[codec(index = 3)]
          BountyAwarded {
            index: ::core::primitive::u32,
            beneficiary: ::subxt::sp_core::crypto::AccountId32,
          },
          #[codec(index = 4)]
          BountyClaimed {
            index: ::core::primitive::u32,
            payout: ::core::primitive::u128,
            beneficiary: ::subxt::sp_core::crypto::AccountId32,
          },
          #[codec(index = 5)]
          BountyCanceled { index: ::core::primitive::u32 },
          #[codec(index = 6)]
          BountyExtended { index: ::core::primitive::u32 },
        }
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Bounty<_0, _1, _2> {
        pub proposer: _0,
        pub value: _1,
        pub fee: _1,
        pub curator_deposit: _1,
        pub bond: _1,
        pub status: runtime_types::pallet_bounties::BountyStatus<_0, _2>,
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub enum BountyStatus<_0, _1> {
        #[codec(index = 0)]
        Proposed,
        #[codec(index = 1)]
        Approved,
        #[codec(index = 2)]
        Funded,
        #[codec(index = 3)]
        CuratorProposed { curator: _0 },
        #[codec(index = 4)]
        Active { curator: _0, update_due: _1 },
        #[codec(index = 5)]
        PendingPayout {
          curator: _0,
          beneficiary: _0,
          unlock_at: _1,
        },
      }
    }
    pub mod pallet_collective {
      use super::runtime_types;
      pub mod pallet {
        use super::runtime_types;
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Call {
          #[codec(index = 0)]
          set_members {
            new_members: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
            prime: ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
            old_count: ::core::primitive::u32,
          },
          #[codec(index = 1)]
          execute {
            proposal: ::std::boxed::Box<runtime_types::lagoon_runtime::Call>,
            #[codec(compact)]
            length_bound: ::core::primitive::u32,
          },
          #[codec(index = 2)]
          propose {
            #[codec(compact)]
            threshold: ::core::primitive::u32,
            proposal: ::std::boxed::Box<runtime_types::lagoon_runtime::Call>,
            #[codec(compact)]
            length_bound: ::core::primitive::u32,
          },
          #[codec(index = 3)]
          vote {
            proposal: ::subxt::sp_core::H256,
            #[codec(compact)]
            index: ::core::primitive::u32,
            approve: ::core::primitive::bool,
          },
          #[codec(index = 4)]
          close {
            proposal_hash: ::subxt::sp_core::H256,
            #[codec(compact)]
            index: ::core::primitive::u32,
            #[codec(compact)]
            proposal_weight_bound: ::core::primitive::u64,
            #[codec(compact)]
            length_bound: ::core::primitive::u32,
          },
          #[codec(index = 5)]
          disapprove_proposal {
            proposal_hash: ::subxt::sp_core::H256,
          },
        }
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Error {
          #[codec(index = 0)]
          NotMember,
          #[codec(index = 1)]
          DuplicateProposal,
          #[codec(index = 2)]
          ProposalMissing,
          #[codec(index = 3)]
          WrongIndex,
          #[codec(index = 4)]
          DuplicateVote,
          #[codec(index = 5)]
          AlreadyInitialized,
          #[codec(index = 6)]
          TooEarly,
          #[codec(index = 7)]
          TooManyProposals,
          #[codec(index = 8)]
          WrongProposalWeight,
          #[codec(index = 9)]
          WrongProposalLength,
        }
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Event {
          #[codec(index = 0)]
          Proposed {
            account: ::subxt::sp_core::crypto::AccountId32,
            proposal_index: ::core::primitive::u32,
            proposal_hash: ::subxt::sp_core::H256,
            threshold: ::core::primitive::u32,
          },
          #[codec(index = 1)]
          Voted {
            account: ::subxt::sp_core::crypto::AccountId32,
            proposal_hash: ::subxt::sp_core::H256,
            voted: ::core::primitive::bool,
            yes: ::core::primitive::u32,
            no: ::core::primitive::u32,
          },
          #[codec(index = 2)]
          Approved {
            proposal_hash: ::subxt::sp_core::H256,
          },
          #[codec(index = 3)]
          Disapproved {
            proposal_hash: ::subxt::sp_core::H256,
          },
          #[codec(index = 4)]
          Executed {
            proposal_hash: ::subxt::sp_core::H256,
            result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
          },
          #[codec(index = 5)]
          MemberExecuted {
            proposal_hash: ::subxt::sp_core::H256,
            result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
          },
          #[codec(index = 6)]
          Closed {
            proposal_hash: ::subxt::sp_core::H256,
            yes: ::core::primitive::u32,
            no: ::core::primitive::u32,
          },
        }
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub enum RawOrigin<_0> {
        #[codec(index = 0)]
        Members(::core::primitive::u32, ::core::primitive::u32),
        #[codec(index = 1)]
        Member(_0),
        #[codec(index = 2)]
        _Phantom,
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Votes<_0, _1> {
        pub index: _1,
        pub threshold: _1,
        pub ayes: ::std::vec::Vec<_0>,
        pub nays: ::std::vec::Vec<_0>,
        pub end: _1,
      }
    }
    pub mod pallet_election_provider_multi_phase {
      use super::runtime_types;
      pub mod pallet {
        use super::runtime_types;
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Call {
          #[codec(index = 0)]
          submit_unsigned {
            raw_solution: ::std::boxed::Box<
              runtime_types::pallet_election_provider_multi_phase::RawSolution<
                runtime_types::lagoon_runtime::config::consensus::NposCompactSolution16,
              >,
            >,
            witness: runtime_types::pallet_election_provider_multi_phase::SolutionOrSnapshotSize,
          },
          #[codec(index = 1)]
          set_minimum_untrusted_score {
            maybe_next_score:
              ::core::option::Option<runtime_types::sp_npos_elections::ElectionScore>,
          },
          #[codec(index = 2)]
          set_emergency_election_result {
            supports: ::std::vec::Vec<(
              ::subxt::sp_core::crypto::AccountId32,
              runtime_types::sp_npos_elections::Support<::subxt::sp_core::crypto::AccountId32>,
            )>,
          },
          #[codec(index = 3)]
          submit {
            raw_solution: ::std::boxed::Box<
              runtime_types::pallet_election_provider_multi_phase::RawSolution<
                runtime_types::lagoon_runtime::config::consensus::NposCompactSolution16,
              >,
            >,
          },
          #[codec(index = 4)]
          governance_fallback {
            maybe_max_voters: ::core::option::Option<::core::primitive::u32>,
            maybe_max_targets: ::core::option::Option<::core::primitive::u32>,
          },
        }
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Error {
          #[codec(index = 0)]
          PreDispatchEarlySubmission,
          #[codec(index = 1)]
          PreDispatchWrongWinnerCount,
          #[codec(index = 2)]
          PreDispatchWeakSubmission,
          #[codec(index = 3)]
          SignedQueueFull,
          #[codec(index = 4)]
          SignedCannotPayDeposit,
          #[codec(index = 5)]
          SignedInvalidWitness,
          #[codec(index = 6)]
          SignedTooMuchWeight,
          #[codec(index = 7)]
          OcwCallWrongEra,
          #[codec(index = 8)]
          MissingSnapshotMetadata,
          #[codec(index = 9)]
          InvalidSubmissionIndex,
          #[codec(index = 10)]
          CallNotAllowed,
          #[codec(index = 11)]
          FallbackFailed,
        }
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Event {
          #[codec(index = 0)]
          SolutionStored {
            election_compute: runtime_types::pallet_election_provider_multi_phase::ElectionCompute,
            prev_ejected: ::core::primitive::bool,
          },
          #[codec(index = 1)]
          ElectionFinalized {
            election_compute: ::core::option::Option<
              runtime_types::pallet_election_provider_multi_phase::ElectionCompute,
            >,
          },
          #[codec(index = 2)]
          Rewarded {
            account: ::subxt::sp_core::crypto::AccountId32,
            value: ::core::primitive::u128,
          },
          #[codec(index = 3)]
          Slashed {
            account: ::subxt::sp_core::crypto::AccountId32,
            value: ::core::primitive::u128,
          },
          #[codec(index = 4)]
          SignedPhaseStarted { round: ::core::primitive::u32 },
          #[codec(index = 5)]
          UnsignedPhaseStarted { round: ::core::primitive::u32 },
        }
      }
      pub mod signed {
        use super::runtime_types;
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub struct SignedSubmission<_0, _1, _2> {
          pub who: _0,
          pub deposit: _1,
          pub raw_solution: runtime_types::pallet_election_provider_multi_phase::RawSolution<_2>,
          pub reward: _1,
        }
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub enum ElectionCompute {
        #[codec(index = 0)]
        OnChain,
        #[codec(index = 1)]
        Signed,
        #[codec(index = 2)]
        Unsigned,
        #[codec(index = 3)]
        Fallback,
        #[codec(index = 4)]
        Emergency,
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub enum Phase<_0> {
        #[codec(index = 0)]
        Off,
        #[codec(index = 1)]
        Signed,
        #[codec(index = 2)]
        Unsigned((::core::primitive::bool, _0)),
        #[codec(index = 3)]
        Emergency,
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct RawSolution<_0> {
        pub solution: _0,
        pub score: runtime_types::sp_npos_elections::ElectionScore,
        pub round: ::core::primitive::u32,
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct ReadySolution<_0> {
        pub supports: ::std::vec::Vec<(_0, runtime_types::sp_npos_elections::Support<_0>)>,
        pub score: runtime_types::sp_npos_elections::ElectionScore,
        pub compute: runtime_types::pallet_election_provider_multi_phase::ElectionCompute,
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct RoundSnapshot {
        pub voters: ::std::vec::Vec<(
          ::subxt::sp_core::crypto::AccountId32,
          ::core::primitive::u64,
          runtime_types::frame_support::storage::bounded_vec::BoundedVec<
            ::subxt::sp_core::crypto::AccountId32,
          >,
        )>,
        pub targets: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct SolutionOrSnapshotSize {
        #[codec(compact)]
        pub voters: ::core::primitive::u32,
        #[codec(compact)]
        pub targets: ::core::primitive::u32,
      }
    }
    pub mod pallet_elections_phragmen {
      use super::runtime_types;
      pub mod pallet {
        use super::runtime_types;
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Call {
          #[codec(index = 0)]
          vote {
            votes: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
            #[codec(compact)]
            value: ::core::primitive::u128,
          },
          #[codec(index = 1)]
          remove_voter,
          #[codec(index = 2)]
          submit_candidacy {
            #[codec(compact)]
            candidate_count: ::core::primitive::u32,
          },
          #[codec(index = 3)]
          renounce_candidacy {
            renouncing: runtime_types::pallet_elections_phragmen::Renouncing,
          },
          #[codec(index = 4)]
          remove_member {
            who: ::subxt::sp_runtime::MultiAddress<
              ::subxt::sp_core::crypto::AccountId32,
              ::core::primitive::u32,
            >,
            has_replacement: ::core::primitive::bool,
          },
          #[codec(index = 5)]
          clean_defunct_voters {
            num_voters: ::core::primitive::u32,
            num_defunct: ::core::primitive::u32,
          },
        }
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Error {
          #[codec(index = 0)]
          UnableToVote,
          #[codec(index = 1)]
          NoVotes,
          #[codec(index = 2)]
          TooManyVotes,
          #[codec(index = 3)]
          MaximumVotesExceeded,
          #[codec(index = 4)]
          LowBalance,
          #[codec(index = 5)]
          UnableToPayBond,
          #[codec(index = 6)]
          MustBeVoter,
          #[codec(index = 7)]
          ReportSelf,
          #[codec(index = 8)]
          DuplicatedCandidate,
          #[codec(index = 9)]
          MemberSubmit,
          #[codec(index = 10)]
          RunnerUpSubmit,
          #[codec(index = 11)]
          InsufficientCandidateFunds,
          #[codec(index = 12)]
          NotMember,
          #[codec(index = 13)]
          InvalidWitnessData,
          #[codec(index = 14)]
          InvalidVoteCount,
          #[codec(index = 15)]
          InvalidRenouncing,
          #[codec(index = 16)]
          InvalidReplacement,
        }
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Event {
          #[codec(index = 0)]
          NewTerm {
            new_members: ::std::vec::Vec<(
              ::subxt::sp_core::crypto::AccountId32,
              ::core::primitive::u128,
            )>,
          },
          #[codec(index = 1)]
          EmptyTerm,
          #[codec(index = 2)]
          ElectionError,
          #[codec(index = 3)]
          MemberKicked {
            member: ::subxt::sp_core::crypto::AccountId32,
          },
          #[codec(index = 4)]
          Renounced {
            candidate: ::subxt::sp_core::crypto::AccountId32,
          },
          #[codec(index = 5)]
          CandidateSlashed {
            candidate: ::subxt::sp_core::crypto::AccountId32,
            amount: ::core::primitive::u128,
          },
          #[codec(index = 6)]
          SeatHolderSlashed {
            seat_holder: ::subxt::sp_core::crypto::AccountId32,
            amount: ::core::primitive::u128,
          },
        }
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub enum Renouncing {
        #[codec(index = 0)]
        Member,
        #[codec(index = 1)]
        RunnerUp,
        #[codec(index = 2)]
        Candidate(#[codec(compact)] ::core::primitive::u32),
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct SeatHolder<_0, _1> {
        pub who: _0,
        pub stake: _1,
        pub deposit: _1,
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Voter<_0, _1> {
        pub votes: ::std::vec::Vec<_0>,
        pub stake: _1,
        pub deposit: _1,
      }
    }
    pub mod pallet_fees {
      use super::runtime_types;
      pub mod pallet {
        use super::runtime_types;
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Call {
          #[codec(index = 0)]
          claim_sunrise_rewards { era_index: ::core::primitive::u32 },
        }
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Error {
          #[codec(index = 0)]
          InvalidSunrisePool,
          #[codec(index = 1)]
          NoRewardsAvailable,
          #[codec(index = 2)]
          InvalidEra,
          #[codec(index = 3)]
          InvalidAsset,
          #[codec(index = 4)]
          NoActiveEra,
          #[codec(index = 5)]
          EraNotReady,
          #[codec(index = 6)]
          AccountFeeOverflow,
          #[codec(index = 7)]
          BalanceOverflow,
        }
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Event {
          #[codec(index = 0)]
          FeesPercentageUpdated(runtime_types::sp_arithmetic::per_things::Percent),
          #[codec(index = 1)]
          SessionEnded {
            era_index: ::core::primitive::u32,
            session_index: ::core::primitive::u64,
            session_fees_by_currency: ::std::vec::Vec<(
              runtime_types::tidefi_primitives::CurrencyId,
              ::core::primitive::u128,
            )>,
          },
          #[codec(index = 2)]
          EraStarted { era_index: ::core::primitive::u32 },
          #[codec(index = 3)]
          EraEnded { era_index: ::core::primitive::u32 },
          #[codec(index = 4)]
          SunriseRewarded {
            era_index: ::core::primitive::u32,
            pool_id: ::core::primitive::u8,
            account_id: ::subxt::sp_core::crypto::AccountId32,
            reward: ::core::primitive::u128,
          },
          #[codec(index = 5)]
          SunriseClaimed {
            era_index: ::core::primitive::u32,
            account_id: ::subxt::sp_core::crypto::AccountId32,
            reward: ::core::primitive::u128,
          },
        }
      }
    }
    pub mod pallet_grandpa {
      use super::runtime_types;
      pub mod pallet {
        use super::runtime_types;
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Call {
          #[codec(index = 0)]
          report_equivocation {
            equivocation_proof: ::std::boxed::Box<
              runtime_types::sp_finality_grandpa::EquivocationProof<
                ::subxt::sp_core::H256,
                ::core::primitive::u32,
              >,
            >,
            key_owner_proof: runtime_types::sp_session::MembershipProof,
          },
          #[codec(index = 1)]
          report_equivocation_unsigned {
            equivocation_proof: ::std::boxed::Box<
              runtime_types::sp_finality_grandpa::EquivocationProof<
                ::subxt::sp_core::H256,
                ::core::primitive::u32,
              >,
            >,
            key_owner_proof: runtime_types::sp_session::MembershipProof,
          },
          #[codec(index = 2)]
          note_stalled {
            delay: ::core::primitive::u32,
            best_finalized_block_number: ::core::primitive::u32,
          },
        }
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Error {
          #[codec(index = 0)]
          PauseFailed,
          #[codec(index = 1)]
          ResumeFailed,
          #[codec(index = 2)]
          ChangePending,
          #[codec(index = 3)]
          TooSoon,
          #[codec(index = 4)]
          InvalidKeyOwnershipProof,
          #[codec(index = 5)]
          InvalidEquivocationProof,
          #[codec(index = 6)]
          DuplicateOffenceReport,
        }
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Event {
          #[codec(index = 0)]
          NewAuthorities {
            authority_set: ::std::vec::Vec<(
              runtime_types::sp_finality_grandpa::app::Public,
              ::core::primitive::u64,
            )>,
          },
          #[codec(index = 1)]
          Paused,
          #[codec(index = 2)]
          Resumed,
        }
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct StoredPendingChange<_0> {
        pub scheduled_at: _0,
        pub delay: _0,
        pub next_authorities:
          runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<(
            runtime_types::sp_finality_grandpa::app::Public,
            ::core::primitive::u64,
          )>,
        pub forced: ::core::option::Option<_0>,
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub enum StoredState<_0> {
        #[codec(index = 0)]
        Live,
        #[codec(index = 1)]
        PendingPause { scheduled_at: _0, delay: _0 },
        #[codec(index = 2)]
        Paused,
        #[codec(index = 3)]
        PendingResume { scheduled_at: _0, delay: _0 },
      }
    }
    pub mod pallet_identity {
      use super::runtime_types;
      pub mod pallet {
        use super::runtime_types;
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Call {
          #[codec(index = 0)]
          add_registrar {
            account: ::subxt::sp_core::crypto::AccountId32,
          },
          #[codec(index = 1)]
          set_identity {
            info: ::std::boxed::Box<runtime_types::pallet_identity::types::IdentityInfo>,
          },
          #[codec(index = 2)]
          set_subs {
            subs: ::std::vec::Vec<(
              ::subxt::sp_core::crypto::AccountId32,
              runtime_types::pallet_identity::types::Data,
            )>,
          },
          #[codec(index = 3)]
          clear_identity,
          #[codec(index = 4)]
          request_judgement {
            #[codec(compact)]
            reg_index: ::core::primitive::u32,
            #[codec(compact)]
            max_fee: ::core::primitive::u128,
          },
          #[codec(index = 5)]
          cancel_request { reg_index: ::core::primitive::u32 },
          #[codec(index = 6)]
          set_fee {
            #[codec(compact)]
            index: ::core::primitive::u32,
            #[codec(compact)]
            fee: ::core::primitive::u128,
          },
          #[codec(index = 7)]
          set_account_id {
            #[codec(compact)]
            index: ::core::primitive::u32,
            new: ::subxt::sp_core::crypto::AccountId32,
          },
          #[codec(index = 8)]
          set_fields {
            #[codec(compact)]
            index: ::core::primitive::u32,
            fields: runtime_types::pallet_identity::types::BitFlags<
              runtime_types::pallet_identity::types::IdentityField,
            >,
          },
          #[codec(index = 9)]
          provide_judgement {
            #[codec(compact)]
            reg_index: ::core::primitive::u32,
            target: ::subxt::sp_runtime::MultiAddress<
              ::subxt::sp_core::crypto::AccountId32,
              ::core::primitive::u32,
            >,
            judgement: runtime_types::pallet_identity::types::Judgement<::core::primitive::u128>,
          },
          #[codec(index = 10)]
          kill_identity {
            target: ::subxt::sp_runtime::MultiAddress<
              ::subxt::sp_core::crypto::AccountId32,
              ::core::primitive::u32,
            >,
          },
          #[codec(index = 11)]
          add_sub {
            sub: ::subxt::sp_runtime::MultiAddress<
              ::subxt::sp_core::crypto::AccountId32,
              ::core::primitive::u32,
            >,
            data: runtime_types::pallet_identity::types::Data,
          },
          #[codec(index = 12)]
          rename_sub {
            sub: ::subxt::sp_runtime::MultiAddress<
              ::subxt::sp_core::crypto::AccountId32,
              ::core::primitive::u32,
            >,
            data: runtime_types::pallet_identity::types::Data,
          },
          #[codec(index = 13)]
          remove_sub {
            sub: ::subxt::sp_runtime::MultiAddress<
              ::subxt::sp_core::crypto::AccountId32,
              ::core::primitive::u32,
            >,
          },
          #[codec(index = 14)]
          quit_sub,
        }
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Error {
          #[codec(index = 0)]
          TooManySubAccounts,
          #[codec(index = 1)]
          NotFound,
          #[codec(index = 2)]
          NotNamed,
          #[codec(index = 3)]
          EmptyIndex,
          #[codec(index = 4)]
          FeeChanged,
          #[codec(index = 5)]
          NoIdentity,
          #[codec(index = 6)]
          StickyJudgement,
          #[codec(index = 7)]
          JudgementGiven,
          #[codec(index = 8)]
          InvalidJudgement,
          #[codec(index = 9)]
          InvalidIndex,
          #[codec(index = 10)]
          InvalidTarget,
          #[codec(index = 11)]
          TooManyFields,
          #[codec(index = 12)]
          TooManyRegistrars,
          #[codec(index = 13)]
          AlreadyClaimed,
          #[codec(index = 14)]
          NotSub,
          #[codec(index = 15)]
          NotOwned,
        }
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Event {
          #[codec(index = 0)]
          IdentitySet {
            who: ::subxt::sp_core::crypto::AccountId32,
          },
          #[codec(index = 1)]
          IdentityCleared {
            who: ::subxt::sp_core::crypto::AccountId32,
            deposit: ::core::primitive::u128,
          },
          #[codec(index = 2)]
          IdentityKilled {
            who: ::subxt::sp_core::crypto::AccountId32,
            deposit: ::core::primitive::u128,
          },
          #[codec(index = 3)]
          JudgementRequested {
            who: ::subxt::sp_core::crypto::AccountId32,
            registrar_index: ::core::primitive::u32,
          },
          #[codec(index = 4)]
          JudgementUnrequested {
            who: ::subxt::sp_core::crypto::AccountId32,
            registrar_index: ::core::primitive::u32,
          },
          #[codec(index = 5)]
          JudgementGiven {
            target: ::subxt::sp_core::crypto::AccountId32,
            registrar_index: ::core::primitive::u32,
          },
          #[codec(index = 6)]
          RegistrarAdded {
            registrar_index: ::core::primitive::u32,
          },
          #[codec(index = 7)]
          SubIdentityAdded {
            sub: ::subxt::sp_core::crypto::AccountId32,
            main: ::subxt::sp_core::crypto::AccountId32,
            deposit: ::core::primitive::u128,
          },
          #[codec(index = 8)]
          SubIdentityRemoved {
            sub: ::subxt::sp_core::crypto::AccountId32,
            main: ::subxt::sp_core::crypto::AccountId32,
            deposit: ::core::primitive::u128,
          },
          #[codec(index = 9)]
          SubIdentityRevoked {
            sub: ::subxt::sp_core::crypto::AccountId32,
            main: ::subxt::sp_core::crypto::AccountId32,
            deposit: ::core::primitive::u128,
          },
        }
      }
      pub mod types {
        use super::runtime_types;
        #[derive(
          :: subxt :: codec :: Encode,
          :: subxt :: codec :: Decode,
          Debug,
          :: subxt :: codec :: CompactAs,
        )]
        pub struct BitFlags<_0>(
          pub ::core::primitive::u64,
          #[codec(skip)] pub ::core::marker::PhantomData<_0>,
        );
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Data {
          #[codec(index = 0)]
          None,
          #[codec(index = 1)]
          Raw0([::core::primitive::u8; 0usize]),
          #[codec(index = 2)]
          Raw1([::core::primitive::u8; 1usize]),
          #[codec(index = 3)]
          Raw2([::core::primitive::u8; 2usize]),
          #[codec(index = 4)]
          Raw3([::core::primitive::u8; 3usize]),
          #[codec(index = 5)]
          Raw4([::core::primitive::u8; 4usize]),
          #[codec(index = 6)]
          Raw5([::core::primitive::u8; 5usize]),
          #[codec(index = 7)]
          Raw6([::core::primitive::u8; 6usize]),
          #[codec(index = 8)]
          Raw7([::core::primitive::u8; 7usize]),
          #[codec(index = 9)]
          Raw8([::core::primitive::u8; 8usize]),
          #[codec(index = 10)]
          Raw9([::core::primitive::u8; 9usize]),
          #[codec(index = 11)]
          Raw10([::core::primitive::u8; 10usize]),
          #[codec(index = 12)]
          Raw11([::core::primitive::u8; 11usize]),
          #[codec(index = 13)]
          Raw12([::core::primitive::u8; 12usize]),
          #[codec(index = 14)]
          Raw13([::core::primitive::u8; 13usize]),
          #[codec(index = 15)]
          Raw14([::core::primitive::u8; 14usize]),
          #[codec(index = 16)]
          Raw15([::core::primitive::u8; 15usize]),
          #[codec(index = 17)]
          Raw16([::core::primitive::u8; 16usize]),
          #[codec(index = 18)]
          Raw17([::core::primitive::u8; 17usize]),
          #[codec(index = 19)]
          Raw18([::core::primitive::u8; 18usize]),
          #[codec(index = 20)]
          Raw19([::core::primitive::u8; 19usize]),
          #[codec(index = 21)]
          Raw20([::core::primitive::u8; 20usize]),
          #[codec(index = 22)]
          Raw21([::core::primitive::u8; 21usize]),
          #[codec(index = 23)]
          Raw22([::core::primitive::u8; 22usize]),
          #[codec(index = 24)]
          Raw23([::core::primitive::u8; 23usize]),
          #[codec(index = 25)]
          Raw24([::core::primitive::u8; 24usize]),
          #[codec(index = 26)]
          Raw25([::core::primitive::u8; 25usize]),
          #[codec(index = 27)]
          Raw26([::core::primitive::u8; 26usize]),
          #[codec(index = 28)]
          Raw27([::core::primitive::u8; 27usize]),
          #[codec(index = 29)]
          Raw28([::core::primitive::u8; 28usize]),
          #[codec(index = 30)]
          Raw29([::core::primitive::u8; 29usize]),
          #[codec(index = 31)]
          Raw30([::core::primitive::u8; 30usize]),
          #[codec(index = 32)]
          Raw31([::core::primitive::u8; 31usize]),
          #[codec(index = 33)]
          Raw32([::core::primitive::u8; 32usize]),
          #[codec(index = 34)]
          BlakeTwo256([::core::primitive::u8; 32usize]),
          #[codec(index = 35)]
          Sha256([::core::primitive::u8; 32usize]),
          #[codec(index = 36)]
          Keccak256([::core::primitive::u8; 32usize]),
          #[codec(index = 37)]
          ShaThree256([::core::primitive::u8; 32usize]),
        }
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum IdentityField {
          #[codec(index = 1)]
          Display,
          #[codec(index = 2)]
          Legal,
          #[codec(index = 4)]
          Web,
          #[codec(index = 8)]
          Riot,
          #[codec(index = 16)]
          Email,
          #[codec(index = 32)]
          PgpFingerprint,
          #[codec(index = 64)]
          Image,
          #[codec(index = 128)]
          Twitter,
        }
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub struct IdentityInfo {
          pub additional: runtime_types::frame_support::storage::bounded_vec::BoundedVec<(
            runtime_types::pallet_identity::types::Data,
            runtime_types::pallet_identity::types::Data,
          )>,
          pub display: runtime_types::pallet_identity::types::Data,
          pub legal: runtime_types::pallet_identity::types::Data,
          pub web: runtime_types::pallet_identity::types::Data,
          pub riot: runtime_types::pallet_identity::types::Data,
          pub email: runtime_types::pallet_identity::types::Data,
          pub pgp_fingerprint: ::core::option::Option<[::core::primitive::u8; 20usize]>,
          pub image: runtime_types::pallet_identity::types::Data,
          pub twitter: runtime_types::pallet_identity::types::Data,
        }
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Judgement<_0> {
          #[codec(index = 0)]
          Unknown,
          #[codec(index = 1)]
          FeePaid(_0),
          #[codec(index = 2)]
          Reasonable,
          #[codec(index = 3)]
          KnownGood,
          #[codec(index = 4)]
          OutOfDate,
          #[codec(index = 5)]
          LowQuality,
          #[codec(index = 6)]
          Erroneous,
        }
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub struct RegistrarInfo<_0, _1> {
          pub account: _1,
          pub fee: _0,
          pub fields: runtime_types::pallet_identity::types::BitFlags<
            runtime_types::pallet_identity::types::IdentityField,
          >,
        }
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub struct Registration<_0> {
          pub judgements: runtime_types::frame_support::storage::bounded_vec::BoundedVec<(
            ::core::primitive::u32,
            runtime_types::pallet_identity::types::Judgement<_0>,
          )>,
          pub deposit: _0,
          pub info: runtime_types::pallet_identity::types::IdentityInfo,
        }
      }
    }
    pub mod pallet_im_online {
      use super::runtime_types;
      pub mod pallet {
        use super::runtime_types;
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Call {
          #[codec(index = 0)]
          heartbeat {
            heartbeat: runtime_types::pallet_im_online::Heartbeat<::core::primitive::u32>,
            signature: runtime_types::pallet_im_online::sr25519::app_sr25519::Signature,
          },
        }
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Error {
          #[codec(index = 0)]
          InvalidKey,
          #[codec(index = 1)]
          DuplicatedHeartbeat,
        }
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Event {
          #[codec(index = 0)]
          HeartbeatReceived {
            authority_id: runtime_types::pallet_im_online::sr25519::app_sr25519::Public,
          },
          #[codec(index = 1)]
          AllGood,
          #[codec(index = 2)]
          SomeOffline {
            offline: ::std::vec::Vec<(
              ::subxt::sp_core::crypto::AccountId32,
              runtime_types::pallet_staking::Exposure<
                ::subxt::sp_core::crypto::AccountId32,
                ::core::primitive::u128,
              >,
            )>,
          },
        }
      }
      pub mod sr25519 {
        use super::runtime_types;
        pub mod app_sr25519 {
          use super::runtime_types;
          #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
          pub struct Public(pub runtime_types::sp_core::sr25519::Public);
          #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
          pub struct Signature(pub runtime_types::sp_core::sr25519::Signature);
        }
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct BoundedOpaqueNetworkState {
        pub peer_id: runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
          ::core::primitive::u8,
        >,
        pub external_addresses:
          runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
            runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
              ::core::primitive::u8,
            >,
          >,
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Heartbeat<_0> {
        pub block_number: _0,
        pub network_state: runtime_types::sp_core::offchain::OpaqueNetworkState,
        pub session_index: _0,
        pub authority_index: _0,
        pub validators_len: _0,
      }
    }
    pub mod pallet_indices {
      use super::runtime_types;
      pub mod pallet {
        use super::runtime_types;
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Call {
          #[codec(index = 0)]
          claim { index: ::core::primitive::u32 },
          #[codec(index = 1)]
          transfer {
            new: ::subxt::sp_core::crypto::AccountId32,
            index: ::core::primitive::u32,
          },
          #[codec(index = 2)]
          free { index: ::core::primitive::u32 },
          #[codec(index = 3)]
          force_transfer {
            new: ::subxt::sp_core::crypto::AccountId32,
            index: ::core::primitive::u32,
            freeze: ::core::primitive::bool,
          },
          #[codec(index = 4)]
          freeze { index: ::core::primitive::u32 },
        }
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Error {
          #[codec(index = 0)]
          NotAssigned,
          #[codec(index = 1)]
          NotOwner,
          #[codec(index = 2)]
          InUse,
          #[codec(index = 3)]
          NotTransfer,
          #[codec(index = 4)]
          Permanent,
        }
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Event {
          #[codec(index = 0)]
          IndexAssigned {
            who: ::subxt::sp_core::crypto::AccountId32,
            index: ::core::primitive::u32,
          },
          #[codec(index = 1)]
          IndexFreed { index: ::core::primitive::u32 },
          #[codec(index = 2)]
          IndexFrozen {
            index: ::core::primitive::u32,
            who: ::subxt::sp_core::crypto::AccountId32,
          },
        }
      }
    }
    pub mod pallet_membership {
      use super::runtime_types;
      pub mod pallet {
        use super::runtime_types;
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Call {
          #[codec(index = 0)]
          add_member {
            who: ::subxt::sp_core::crypto::AccountId32,
          },
          #[codec(index = 1)]
          remove_member {
            who: ::subxt::sp_core::crypto::AccountId32,
          },
          #[codec(index = 2)]
          swap_member {
            remove: ::subxt::sp_core::crypto::AccountId32,
            add: ::subxt::sp_core::crypto::AccountId32,
          },
          #[codec(index = 3)]
          reset_members {
            members: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
          },
          #[codec(index = 4)]
          change_key {
            new: ::subxt::sp_core::crypto::AccountId32,
          },
          #[codec(index = 5)]
          set_prime {
            who: ::subxt::sp_core::crypto::AccountId32,
          },
          #[codec(index = 6)]
          clear_prime,
        }
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Error {
          #[codec(index = 0)]
          AlreadyMember,
          #[codec(index = 1)]
          NotMember,
        }
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Event {
          #[codec(index = 0)]
          MemberAdded,
          #[codec(index = 1)]
          MemberRemoved,
          #[codec(index = 2)]
          MembersSwapped,
          #[codec(index = 3)]
          MembersReset,
          #[codec(index = 4)]
          KeyChanged,
          #[codec(index = 5)]
          Dummy,
        }
      }
    }
    pub mod pallet_multisig {
      use super::runtime_types;
      pub mod pallet {
        use super::runtime_types;
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Call {
          #[codec(index = 0)]
          as_multi_threshold_1 {
            other_signatories: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
            call: ::std::boxed::Box<runtime_types::lagoon_runtime::Call>,
          },
          #[codec(index = 1)]
          as_multi {
            threshold: ::core::primitive::u16,
            other_signatories: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
            maybe_timepoint: ::core::option::Option<
              runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
            >,
            call: ::subxt::WrapperKeepOpaque<runtime_types::lagoon_runtime::Call>,
            store_call: ::core::primitive::bool,
            max_weight: ::core::primitive::u64,
          },
          #[codec(index = 2)]
          approve_as_multi {
            threshold: ::core::primitive::u16,
            other_signatories: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
            maybe_timepoint: ::core::option::Option<
              runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
            >,
            call_hash: [::core::primitive::u8; 32usize],
            max_weight: ::core::primitive::u64,
          },
          #[codec(index = 3)]
          cancel_as_multi {
            threshold: ::core::primitive::u16,
            other_signatories: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
            timepoint: runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
            call_hash: [::core::primitive::u8; 32usize],
          },
        }
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Error {
          #[codec(index = 0)]
          MinimumThreshold,
          #[codec(index = 1)]
          AlreadyApproved,
          #[codec(index = 2)]
          NoApprovalsNeeded,
          #[codec(index = 3)]
          TooFewSignatories,
          #[codec(index = 4)]
          TooManySignatories,
          #[codec(index = 5)]
          SignatoriesOutOfOrder,
          #[codec(index = 6)]
          SenderInSignatories,
          #[codec(index = 7)]
          NotFound,
          #[codec(index = 8)]
          NotOwner,
          #[codec(index = 9)]
          NoTimepoint,
          #[codec(index = 10)]
          WrongTimepoint,
          #[codec(index = 11)]
          UnexpectedTimepoint,
          #[codec(index = 12)]
          MaxWeightTooLow,
          #[codec(index = 13)]
          AlreadyStored,
        }
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Event {
          #[codec(index = 0)]
          NewMultisig {
            approving: ::subxt::sp_core::crypto::AccountId32,
            multisig: ::subxt::sp_core::crypto::AccountId32,
            call_hash: [::core::primitive::u8; 32usize],
          },
          #[codec(index = 1)]
          MultisigApproval {
            approving: ::subxt::sp_core::crypto::AccountId32,
            timepoint: runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
            multisig: ::subxt::sp_core::crypto::AccountId32,
            call_hash: [::core::primitive::u8; 32usize],
          },
          #[codec(index = 2)]
          MultisigExecuted {
            approving: ::subxt::sp_core::crypto::AccountId32,
            timepoint: runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
            multisig: ::subxt::sp_core::crypto::AccountId32,
            call_hash: [::core::primitive::u8; 32usize],
            result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
          },
          #[codec(index = 3)]
          MultisigCancelled {
            cancelling: ::subxt::sp_core::crypto::AccountId32,
            timepoint: runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
            multisig: ::subxt::sp_core::crypto::AccountId32,
            call_hash: [::core::primitive::u8; 32usize],
          },
        }
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Multisig<_0, _1, _2> {
        pub when: runtime_types::pallet_multisig::Timepoint<_0>,
        pub deposit: _1,
        pub depositor: _2,
        pub approvals: ::std::vec::Vec<_2>,
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Timepoint<_0> {
        pub height: _0,
        pub index: _0,
      }
    }
    pub mod pallet_offences {
      use super::runtime_types;
      pub mod pallet {
        use super::runtime_types;
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Event {
          #[codec(index = 0)]
          Offence {
            kind: [::core::primitive::u8; 16usize],
            timeslot: ::std::vec::Vec<::core::primitive::u8>,
          },
        }
      }
    }
    pub mod pallet_oracle {
      use super::runtime_types;
      pub mod pallet {
        use super::runtime_types;
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Call {
          #[codec(index = 0)]
          confirm_swap {
            request_id: ::subxt::sp_core::H256,
            market_makers: ::std::vec::Vec<runtime_types::tidefi_primitives::SwapConfirmation>,
          },
          #[codec(index = 1)]
          cancel_swap { request_id: ::subxt::sp_core::H256 },
          #[codec(index = 2)]
          set_account_id {
            new_account_id: ::subxt::sp_core::crypto::AccountId32,
          },
          #[codec(index = 3)]
          set_status { is_enabled: ::core::primitive::bool },
          #[codec(index = 4)]
          im_alive {
            im_alive: runtime_types::tidefi_primitives::OracleImAlive,
          },
          #[codec(index = 5)]
          add_market_maker {
            account_id: ::subxt::sp_core::crypto::AccountId32,
          },
          #[codec(index = 6)]
          remove_market_maker {
            account_id: ::subxt::sp_core::crypto::AccountId32,
          },
        }
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Error {
          #[codec(index = 0)]
          OraclePaused,
          #[codec(index = 1)]
          AccessDenied,
          #[codec(index = 2)]
          InvalidRequestId,
          #[codec(index = 3)]
          InvalidRequestStatus,
          #[codec(index = 4)]
          InvalidMarketMakerRequest,
          #[codec(index = 5)]
          Conflict,
          #[codec(index = 6)]
          TransferFailed,
          #[codec(index = 7)]
          BurnFailed,
          #[codec(index = 8)]
          MintFailed,
          #[codec(index = 9)]
          ReleaseFailed,
          #[codec(index = 10)]
          FeesFailed,
          #[codec(index = 11)]
          UnknownAsset,
          #[codec(index = 12)]
          NoFunds,
          #[codec(index = 13)]
          Overflow,
          #[codec(index = 14)]
          MarketMakerOverflow,
          #[codec(index = 15)]
          MarketMakerNoFunds,
          #[codec(index = 16)]
          MarketMakerCantDeposit,
          #[codec(index = 17)]
          SwapOverflow,
          #[codec(index = 18)]
          UnknownError,
        }
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Event {
          #[codec(index = 0)]
          StatusChanged { is_enabled: ::core::primitive::bool },
          #[codec(index = 1)]
          AccountChanged {
            account_id: ::subxt::sp_core::crypto::AccountId32,
          },
          #[codec(index = 2)]
          MarketMakerAdded {
            account_id: ::subxt::sp_core::crypto::AccountId32,
          },
          #[codec(index = 3)]
          MarketMakerRemoved {
            account_id: ::subxt::sp_core::crypto::AccountId32,
          },
          #[codec(index = 4)]
          SwapProcessed {
            request_id: ::subxt::sp_core::H256,
            status: runtime_types::tidefi_primitives::SwapStatus,
            account_id: ::subxt::sp_core::crypto::AccountId32,
            currency_from: runtime_types::tidefi_primitives::CurrencyId,
            currency_amount_from: ::core::primitive::u128,
            currency_to: runtime_types::tidefi_primitives::CurrencyId,
            currency_amount_to: ::core::primitive::u128,
            initial_extrinsic_hash: [::core::primitive::u8; 32usize],
          },
          #[codec(index = 5)]
          SwapCancelled { request_id: ::subxt::sp_core::H256 },
        }
      }
    }
    pub mod pallet_preimage {
      use super::runtime_types;
      pub mod pallet {
        use super::runtime_types;
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Call {
          #[codec(index = 0)]
          note_preimage {
            bytes: ::std::vec::Vec<::core::primitive::u8>,
          },
          #[codec(index = 1)]
          unnote_preimage { hash: ::subxt::sp_core::H256 },
          #[codec(index = 2)]
          request_preimage { hash: ::subxt::sp_core::H256 },
          #[codec(index = 3)]
          unrequest_preimage { hash: ::subxt::sp_core::H256 },
        }
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Error {
          #[codec(index = 0)]
          TooLarge,
          #[codec(index = 1)]
          AlreadyNoted,
          #[codec(index = 2)]
          NotAuthorized,
          #[codec(index = 3)]
          NotNoted,
          #[codec(index = 4)]
          Requested,
          #[codec(index = 5)]
          NotRequested,
        }
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Event {
          #[codec(index = 0)]
          Noted { hash: ::subxt::sp_core::H256 },
          #[codec(index = 1)]
          Requested { hash: ::subxt::sp_core::H256 },
          #[codec(index = 2)]
          Cleared { hash: ::subxt::sp_core::H256 },
        }
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub enum RequestStatus<_0, _1> {
        #[codec(index = 0)]
        Unrequested(::core::option::Option<(_0, _1)>),
        #[codec(index = 1)]
        Requested(::core::primitive::u32),
      }
    }
    pub mod pallet_proxy {
      use super::runtime_types;
      pub mod pallet {
        use super::runtime_types;
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Call {
          #[codec(index = 0)]
          proxy {
            real: ::subxt::sp_core::crypto::AccountId32,
            force_proxy_type:
              ::core::option::Option<runtime_types::lagoon_runtime::config::proxy::ProxyType>,
            call: ::std::boxed::Box<runtime_types::lagoon_runtime::Call>,
          },
          #[codec(index = 1)]
          add_proxy {
            delegate: ::subxt::sp_core::crypto::AccountId32,
            proxy_type: runtime_types::lagoon_runtime::config::proxy::ProxyType,
            delay: ::core::primitive::u32,
          },
          #[codec(index = 2)]
          remove_proxy {
            delegate: ::subxt::sp_core::crypto::AccountId32,
            proxy_type: runtime_types::lagoon_runtime::config::proxy::ProxyType,
            delay: ::core::primitive::u32,
          },
          #[codec(index = 3)]
          remove_proxies,
          #[codec(index = 4)]
          anonymous {
            proxy_type: runtime_types::lagoon_runtime::config::proxy::ProxyType,
            delay: ::core::primitive::u32,
            index: ::core::primitive::u16,
          },
          #[codec(index = 5)]
          kill_anonymous {
            spawner: ::subxt::sp_core::crypto::AccountId32,
            proxy_type: runtime_types::lagoon_runtime::config::proxy::ProxyType,
            index: ::core::primitive::u16,
            #[codec(compact)]
            height: ::core::primitive::u32,
            #[codec(compact)]
            ext_index: ::core::primitive::u32,
          },
          #[codec(index = 6)]
          announce {
            real: ::subxt::sp_core::crypto::AccountId32,
            call_hash: ::subxt::sp_core::H256,
          },
          #[codec(index = 7)]
          remove_announcement {
            real: ::subxt::sp_core::crypto::AccountId32,
            call_hash: ::subxt::sp_core::H256,
          },
          #[codec(index = 8)]
          reject_announcement {
            delegate: ::subxt::sp_core::crypto::AccountId32,
            call_hash: ::subxt::sp_core::H256,
          },
          #[codec(index = 9)]
          proxy_announced {
            delegate: ::subxt::sp_core::crypto::AccountId32,
            real: ::subxt::sp_core::crypto::AccountId32,
            force_proxy_type:
              ::core::option::Option<runtime_types::lagoon_runtime::config::proxy::ProxyType>,
            call: ::std::boxed::Box<runtime_types::lagoon_runtime::Call>,
          },
        }
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Error {
          #[codec(index = 0)]
          TooMany,
          #[codec(index = 1)]
          NotFound,
          #[codec(index = 2)]
          NotProxy,
          #[codec(index = 3)]
          Unproxyable,
          #[codec(index = 4)]
          Duplicate,
          #[codec(index = 5)]
          NoPermission,
          #[codec(index = 6)]
          Unannounced,
          #[codec(index = 7)]
          NoSelfProxy,
        }
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Event {
          #[codec(index = 0)]
          ProxyExecuted {
            result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
          },
          #[codec(index = 1)]
          AnonymousCreated {
            anonymous: ::subxt::sp_core::crypto::AccountId32,
            who: ::subxt::sp_core::crypto::AccountId32,
            proxy_type: runtime_types::lagoon_runtime::config::proxy::ProxyType,
            disambiguation_index: ::core::primitive::u16,
          },
          #[codec(index = 2)]
          Announced {
            real: ::subxt::sp_core::crypto::AccountId32,
            proxy: ::subxt::sp_core::crypto::AccountId32,
            call_hash: ::subxt::sp_core::H256,
          },
          #[codec(index = 3)]
          ProxyAdded {
            delegator: ::subxt::sp_core::crypto::AccountId32,
            delegatee: ::subxt::sp_core::crypto::AccountId32,
            proxy_type: runtime_types::lagoon_runtime::config::proxy::ProxyType,
            delay: ::core::primitive::u32,
          },
          #[codec(index = 4)]
          ProxyRemoved {
            delegator: ::subxt::sp_core::crypto::AccountId32,
            delegatee: ::subxt::sp_core::crypto::AccountId32,
            proxy_type: runtime_types::lagoon_runtime::config::proxy::ProxyType,
            delay: ::core::primitive::u32,
          },
        }
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Announcement<_0, _1, _2> {
        pub real: _0,
        pub call_hash: _1,
        pub height: _2,
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct ProxyDefinition<_0, _1, _2> {
        pub delegate: _0,
        pub proxy_type: _1,
        pub delay: _2,
      }
    }
    pub mod pallet_quorum {
      use super::runtime_types;
      pub mod pallet {
        use super::runtime_types;
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Call {
          #[codec(index = 0)]
          submit_proposal {
            proposal: runtime_types::tidefi_primitives::ProposalType<
              ::subxt::sp_core::crypto::AccountId32,
              ::core::primitive::u32,
              ::std::vec::Vec<::core::primitive::u8>,
              ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
            >,
          },
          #[codec(index = 1)]
          acknowledge_proposal { proposal: ::subxt::sp_core::H256 },
          #[codec(index = 2)]
          acknowledge_burned { proposal: ::subxt::sp_core::H256 },
          #[codec(index = 3)]
          reject_proposal { proposal: ::subxt::sp_core::H256 },
          #[codec(index = 4)]
          eval_proposal_state { proposal: ::subxt::sp_core::H256 },
          #[codec(index = 5)]
          submit_public_keys {
            public_keys: ::std::vec::Vec<(
              ::core::primitive::u32,
              ::std::vec::Vec<::core::primitive::u8>,
            )>,
          },
        }
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Error {
          #[codec(index = 0)]
          AssetDisabled,
          #[codec(index = 1)]
          AccessDenied,
          #[codec(index = 2)]
          BurnFailed,
          #[codec(index = 3)]
          ProposalsCapExceeded,
          #[codec(index = 4)]
          ProposalDoesNotExist,
          #[codec(index = 5)]
          ProposalAlreadyComplete,
          #[codec(index = 6)]
          ProposalExpired,
          #[codec(index = 7)]
          MemberAlreadyVoted,
          #[codec(index = 8)]
          MintFailed,
          #[codec(index = 9)]
          BadProposal,
          #[codec(index = 10)]
          BadPublicKey,
          #[codec(index = 11)]
          BadTransactionId,
          #[codec(index = 12)]
          BadExternalAddress,
          #[codec(index = 13)]
          BurnedQueueOverflow,
          #[codec(index = 14)]
          WatchlistOverflow,
          #[codec(index = 15)]
          MembersOverflow,
          #[codec(index = 16)]
          VotesOverflow,
          #[codec(index = 17)]
          PublicKeysOverflow,
          #[codec(index = 18)]
          UnknownError,
        }
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Event {
          #[codec(index = 0)]
          StatusChanged { is_enabled: ::core::primitive::bool },
          #[codec(index = 1)]
          AccountChanged {
            account_id: ::subxt::sp_core::crypto::AccountId32,
          },
          #[codec(index = 2)]
          Minted {
            proposal_id: ::subxt::sp_core::H256,
            account_id: ::subxt::sp_core::crypto::AccountId32,
            currency_id: runtime_types::tidefi_primitives::CurrencyId,
            amount: ::core::primitive::u128,
            transaction_id: ::std::vec::Vec<::core::primitive::u8>,
            compliance_level: runtime_types::tidefi_primitives::ComplianceLevel,
          },
          #[codec(index = 3)]
          WatchTransactionAdded {
            account_id: ::subxt::sp_core::crypto::AccountId32,
            currency_id: runtime_types::tidefi_primitives::CurrencyId,
            amount: ::core::primitive::u128,
            compliance_level: runtime_types::tidefi_primitives::ComplianceLevel,
            transaction_id: ::std::vec::Vec<::core::primitive::u8>,
            watch_action: runtime_types::tidefi_primitives::WatchListAction,
          },
          #[codec(index = 4)]
          BurnedInitialized {
            proposal_id: ::subxt::sp_core::H256,
            account_id: ::subxt::sp_core::crypto::AccountId32,
            currency_id: runtime_types::tidefi_primitives::CurrencyId,
            amount: ::core::primitive::u128,
          },
          #[codec(index = 5)]
          BurnedAcknowledged { proposal_id: ::subxt::sp_core::H256 },
          #[codec(index = 6)]
          VoteFor {
            account_id: ::subxt::sp_core::crypto::AccountId32,
            proposal_id: ::subxt::sp_core::H256,
          },
          #[codec(index = 7)]
          VoteAgainst {
            account_id: ::subxt::sp_core::crypto::AccountId32,
            proposal_id: ::subxt::sp_core::H256,
          },
          #[codec(index = 8)]
          ProposalSubmitted { proposal_id: ::subxt::sp_core::H256 },
          #[codec(index = 9)]
          ProposalApproved { proposal_id: ::subxt::sp_core::H256 },
          #[codec(index = 10)]
          ProposalProcessed { proposal_id: ::subxt::sp_core::H256 },
          #[codec(index = 11)]
          ProposalRejected { proposal_id: ::subxt::sp_core::H256 },
          #[codec(index = 12)]
          ConfigurationUpdated {
            members: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
            threshold: ::core::primitive::u16,
          },
        }
      }
    }
    pub mod pallet_recovery {
      use super::runtime_types;
      pub mod pallet {
        use super::runtime_types;
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Call {
          #[codec(index = 0)]
          as_recovered {
            account: ::subxt::sp_core::crypto::AccountId32,
            call: ::std::boxed::Box<runtime_types::lagoon_runtime::Call>,
          },
          #[codec(index = 1)]
          set_recovered {
            lost: ::subxt::sp_core::crypto::AccountId32,
            rescuer: ::subxt::sp_core::crypto::AccountId32,
          },
          #[codec(index = 2)]
          create_recovery {
            friends: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
            threshold: ::core::primitive::u16,
            delay_period: ::core::primitive::u32,
          },
          #[codec(index = 3)]
          initiate_recovery {
            account: ::subxt::sp_core::crypto::AccountId32,
          },
          #[codec(index = 4)]
          vouch_recovery {
            lost: ::subxt::sp_core::crypto::AccountId32,
            rescuer: ::subxt::sp_core::crypto::AccountId32,
          },
          #[codec(index = 5)]
          claim_recovery {
            account: ::subxt::sp_core::crypto::AccountId32,
          },
          #[codec(index = 6)]
          close_recovery {
            rescuer: ::subxt::sp_core::crypto::AccountId32,
          },
          #[codec(index = 7)]
          remove_recovery,
          #[codec(index = 8)]
          cancel_recovered {
            account: ::subxt::sp_core::crypto::AccountId32,
          },
        }
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Error {
          #[codec(index = 0)]
          NotAllowed,
          #[codec(index = 1)]
          ZeroThreshold,
          #[codec(index = 2)]
          NotEnoughFriends,
          #[codec(index = 3)]
          MaxFriends,
          #[codec(index = 4)]
          NotSorted,
          #[codec(index = 5)]
          NotRecoverable,
          #[codec(index = 6)]
          AlreadyRecoverable,
          #[codec(index = 7)]
          AlreadyStarted,
          #[codec(index = 8)]
          NotStarted,
          #[codec(index = 9)]
          NotFriend,
          #[codec(index = 10)]
          DelayPeriod,
          #[codec(index = 11)]
          AlreadyVouched,
          #[codec(index = 12)]
          Threshold,
          #[codec(index = 13)]
          StillActive,
          #[codec(index = 14)]
          AlreadyProxy,
          #[codec(index = 15)]
          BadState,
        }
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Event {
          #[codec(index = 0)]
          RecoveryCreated {
            account: ::subxt::sp_core::crypto::AccountId32,
          },
          #[codec(index = 1)]
          RecoveryInitiated {
            lost_account: ::subxt::sp_core::crypto::AccountId32,
            rescuer_account: ::subxt::sp_core::crypto::AccountId32,
          },
          #[codec(index = 2)]
          RecoveryVouched {
            lost_account: ::subxt::sp_core::crypto::AccountId32,
            rescuer_account: ::subxt::sp_core::crypto::AccountId32,
            sender: ::subxt::sp_core::crypto::AccountId32,
          },
          #[codec(index = 3)]
          RecoveryClosed {
            lost_account: ::subxt::sp_core::crypto::AccountId32,
            rescuer_account: ::subxt::sp_core::crypto::AccountId32,
          },
          #[codec(index = 4)]
          AccountRecovered {
            lost_account: ::subxt::sp_core::crypto::AccountId32,
            rescuer_account: ::subxt::sp_core::crypto::AccountId32,
          },
          #[codec(index = 5)]
          RecoveryRemoved {
            lost_account: ::subxt::sp_core::crypto::AccountId32,
          },
        }
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct ActiveRecovery<_0, _1, _2> {
        pub created: _0,
        pub deposit: _1,
        pub friends: _2,
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct RecoveryConfig<_0, _1, _2> {
        pub delay_period: _0,
        pub deposit: _1,
        pub friends: _2,
        pub threshold: ::core::primitive::u16,
      }
    }
    pub mod pallet_scheduler {
      use super::runtime_types;
      pub mod pallet {
        use super::runtime_types;
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Call {
          #[codec(index = 0)]
          schedule {
            when: ::core::primitive::u32,
            maybe_periodic:
              ::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
            priority: ::core::primitive::u8,
            call: ::std::boxed::Box<
              runtime_types::frame_support::traits::schedule::MaybeHashed<
                runtime_types::lagoon_runtime::Call,
                ::subxt::sp_core::H256,
              >,
            >,
          },
          #[codec(index = 1)]
          cancel {
            when: ::core::primitive::u32,
            index: ::core::primitive::u32,
          },
          #[codec(index = 2)]
          schedule_named {
            id: ::std::vec::Vec<::core::primitive::u8>,
            when: ::core::primitive::u32,
            maybe_periodic:
              ::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
            priority: ::core::primitive::u8,
            call: ::std::boxed::Box<
              runtime_types::frame_support::traits::schedule::MaybeHashed<
                runtime_types::lagoon_runtime::Call,
                ::subxt::sp_core::H256,
              >,
            >,
          },
          #[codec(index = 3)]
          cancel_named {
            id: ::std::vec::Vec<::core::primitive::u8>,
          },
          #[codec(index = 4)]
          schedule_after {
            after: ::core::primitive::u32,
            maybe_periodic:
              ::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
            priority: ::core::primitive::u8,
            call: ::std::boxed::Box<
              runtime_types::frame_support::traits::schedule::MaybeHashed<
                runtime_types::lagoon_runtime::Call,
                ::subxt::sp_core::H256,
              >,
            >,
          },
          #[codec(index = 5)]
          schedule_named_after {
            id: ::std::vec::Vec<::core::primitive::u8>,
            after: ::core::primitive::u32,
            maybe_periodic:
              ::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
            priority: ::core::primitive::u8,
            call: ::std::boxed::Box<
              runtime_types::frame_support::traits::schedule::MaybeHashed<
                runtime_types::lagoon_runtime::Call,
                ::subxt::sp_core::H256,
              >,
            >,
          },
        }
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Error {
          #[codec(index = 0)]
          FailedToSchedule,
          #[codec(index = 1)]
          NotFound,
          #[codec(index = 2)]
          TargetBlockNumberInPast,
          #[codec(index = 3)]
          RescheduleNoChange,
        }
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Event {
          #[codec(index = 0)]
          Scheduled {
            when: ::core::primitive::u32,
            index: ::core::primitive::u32,
          },
          #[codec(index = 1)]
          Canceled {
            when: ::core::primitive::u32,
            index: ::core::primitive::u32,
          },
          #[codec(index = 2)]
          Dispatched {
            task: (::core::primitive::u32, ::core::primitive::u32),
            id: ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
            result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
          },
          #[codec(index = 3)]
          CallLookupFailed {
            task: (::core::primitive::u32, ::core::primitive::u32),
            id: ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
            error: runtime_types::frame_support::traits::schedule::LookupError,
          },
        }
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct ScheduledV3<_0, _1, _2, _3> {
        pub maybe_id: ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
        pub priority: ::core::primitive::u8,
        pub call: _0,
        pub maybe_periodic: ::core::option::Option<(_1, _1)>,
        pub origin: _2,
        #[codec(skip)]
        pub __subxt_unused_type_params: ::core::marker::PhantomData<_3>,
      }
    }
    pub mod pallet_security {
      use super::runtime_types;
      pub mod pallet {
        use super::runtime_types;
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Call {
          #[codec(index = 0)]
          set_status {
            status_code: runtime_types::tidefi_primitives::StatusCode,
          },
        }
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Error {
          #[codec(index = 0)]
          ChainMaintenanceMode,
        }
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Event {
          #[codec(index = 0)]
          StatusChanged(runtime_types::tidefi_primitives::StatusCode),
          #[codec(index = 1)]
          UpdateCurrentBlock(::core::primitive::u32),
        }
      }
    }
    pub mod pallet_session {
      use super::runtime_types;
      pub mod pallet {
        use super::runtime_types;
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Call {
          #[codec(index = 0)]
          set_keys {
            keys: runtime_types::lagoon_runtime::SessionKeys,
            proof: ::std::vec::Vec<::core::primitive::u8>,
          },
          #[codec(index = 1)]
          purge_keys,
        }
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Error {
          #[codec(index = 0)]
          InvalidProof,
          #[codec(index = 1)]
          NoAssociatedValidatorId,
          #[codec(index = 2)]
          DuplicatedKey,
          #[codec(index = 3)]
          NoKeys,
          #[codec(index = 4)]
          NoAccount,
        }
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Event {
          #[codec(index = 0)]
          NewSession {
            session_index: ::core::primitive::u32,
          },
        }
      }
    }
    pub mod pallet_staking {
      use super::runtime_types;
      pub mod pallet {
        use super::runtime_types;
        pub mod pallet {
          use super::runtime_types;
          #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
          pub enum Call {
            #[codec(index = 0)]
            bond {
              controller: ::subxt::sp_runtime::MultiAddress<
                ::subxt::sp_core::crypto::AccountId32,
                ::core::primitive::u32,
              >,
              #[codec(compact)]
              value: ::core::primitive::u128,
              payee: runtime_types::pallet_staking::RewardDestination<
                ::subxt::sp_core::crypto::AccountId32,
              >,
            },
            #[codec(index = 1)]
            bond_extra {
              #[codec(compact)]
              max_additional: ::core::primitive::u128,
            },
            #[codec(index = 2)]
            unbond {
              #[codec(compact)]
              value: ::core::primitive::u128,
            },
            #[codec(index = 3)]
            withdraw_unbonded {
              num_slashing_spans: ::core::primitive::u32,
            },
            #[codec(index = 4)]
            validate {
              prefs: runtime_types::pallet_staking::ValidatorPrefs,
            },
            #[codec(index = 5)]
            nominate {
              targets: ::std::vec::Vec<
                ::subxt::sp_runtime::MultiAddress<
                  ::subxt::sp_core::crypto::AccountId32,
                  ::core::primitive::u32,
                >,
              >,
            },
            #[codec(index = 6)]
            chill,
            #[codec(index = 7)]
            set_payee {
              payee: runtime_types::pallet_staking::RewardDestination<
                ::subxt::sp_core::crypto::AccountId32,
              >,
            },
            #[codec(index = 8)]
            set_controller {
              controller: ::subxt::sp_runtime::MultiAddress<
                ::subxt::sp_core::crypto::AccountId32,
                ::core::primitive::u32,
              >,
            },
            #[codec(index = 9)]
            set_validator_count {
              #[codec(compact)]
              new: ::core::primitive::u32,
            },
            #[codec(index = 10)]
            increase_validator_count {
              #[codec(compact)]
              additional: ::core::primitive::u32,
            },
            #[codec(index = 11)]
            scale_validator_count {
              factor: runtime_types::sp_arithmetic::per_things::Percent,
            },
            #[codec(index = 12)]
            force_no_eras,
            #[codec(index = 13)]
            force_new_era,
            #[codec(index = 14)]
            set_invulnerables {
              invulnerables: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
            },
            #[codec(index = 15)]
            force_unstake {
              stash: ::subxt::sp_core::crypto::AccountId32,
              num_slashing_spans: ::core::primitive::u32,
            },
            #[codec(index = 16)]
            force_new_era_always,
            #[codec(index = 17)]
            cancel_deferred_slash {
              era: ::core::primitive::u32,
              slash_indices: ::std::vec::Vec<::core::primitive::u32>,
            },
            #[codec(index = 18)]
            payout_stakers {
              validator_stash: ::subxt::sp_core::crypto::AccountId32,
              era: ::core::primitive::u32,
            },
            #[codec(index = 19)]
            rebond {
              #[codec(compact)]
              value: ::core::primitive::u128,
            },
            #[codec(index = 20)]
            set_history_depth {
              #[codec(compact)]
              new_history_depth: ::core::primitive::u32,
              #[codec(compact)]
              era_items_deleted: ::core::primitive::u32,
            },
            #[codec(index = 21)]
            reap_stash {
              stash: ::subxt::sp_core::crypto::AccountId32,
              num_slashing_spans: ::core::primitive::u32,
            },
            #[codec(index = 22)]
            kick {
              who: ::std::vec::Vec<
                ::subxt::sp_runtime::MultiAddress<
                  ::subxt::sp_core::crypto::AccountId32,
                  ::core::primitive::u32,
                >,
              >,
            },
            #[codec(index = 23)]
            set_staking_configs {
              min_nominator_bond:
                runtime_types::pallet_staking::pallet::pallet::ConfigOp<::core::primitive::u128>,
              min_validator_bond:
                runtime_types::pallet_staking::pallet::pallet::ConfigOp<::core::primitive::u128>,
              max_nominator_count:
                runtime_types::pallet_staking::pallet::pallet::ConfigOp<::core::primitive::u32>,
              max_validator_count:
                runtime_types::pallet_staking::pallet::pallet::ConfigOp<::core::primitive::u32>,
              chill_threshold: runtime_types::pallet_staking::pallet::pallet::ConfigOp<
                runtime_types::sp_arithmetic::per_things::Percent,
              >,
              min_commission: runtime_types::pallet_staking::pallet::pallet::ConfigOp<
                runtime_types::sp_arithmetic::per_things::Perbill,
              >,
            },
            #[codec(index = 24)]
            chill_other {
              controller: ::subxt::sp_core::crypto::AccountId32,
            },
            #[codec(index = 25)]
            force_apply_min_commission {
              validator_stash: ::subxt::sp_core::crypto::AccountId32,
            },
          }
          #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
          pub enum ConfigOp<_0> {
            #[codec(index = 0)]
            Noop,
            #[codec(index = 1)]
            Set(_0),
            #[codec(index = 2)]
            Remove,
          }
          #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
          pub enum Error {
            #[codec(index = 0)]
            NotController,
            #[codec(index = 1)]
            NotStash,
            #[codec(index = 2)]
            AlreadyBonded,
            #[codec(index = 3)]
            AlreadyPaired,
            #[codec(index = 4)]
            EmptyTargets,
            #[codec(index = 5)]
            DuplicateIndex,
            #[codec(index = 6)]
            InvalidSlashIndex,
            #[codec(index = 7)]
            InsufficientBond,
            #[codec(index = 8)]
            NoMoreChunks,
            #[codec(index = 9)]
            NoUnlockChunk,
            #[codec(index = 10)]
            FundedTarget,
            #[codec(index = 11)]
            InvalidEraToReward,
            #[codec(index = 12)]
            InvalidNumberOfNominations,
            #[codec(index = 13)]
            NotSortedAndUnique,
            #[codec(index = 14)]
            AlreadyClaimed,
            #[codec(index = 15)]
            IncorrectHistoryDepth,
            #[codec(index = 16)]
            IncorrectSlashingSpans,
            #[codec(index = 17)]
            BadState,
            #[codec(index = 18)]
            TooManyTargets,
            #[codec(index = 19)]
            BadTarget,
            #[codec(index = 20)]
            CannotChillOther,
            #[codec(index = 21)]
            TooManyNominators,
            #[codec(index = 22)]
            TooManyValidators,
            #[codec(index = 23)]
            CommissionTooLow,
          }
          #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
          pub enum Event {
            #[codec(index = 0)]
            EraPaid(
              ::core::primitive::u32,
              ::core::primitive::u128,
              ::core::primitive::u128,
            ),
            #[codec(index = 1)]
            Rewarded(
              ::subxt::sp_core::crypto::AccountId32,
              ::core::primitive::u128,
            ),
            #[codec(index = 2)]
            Slashed(
              ::subxt::sp_core::crypto::AccountId32,
              ::core::primitive::u128,
            ),
            #[codec(index = 3)]
            OldSlashingReportDiscarded(::core::primitive::u32),
            #[codec(index = 4)]
            StakersElected,
            #[codec(index = 5)]
            Bonded(
              ::subxt::sp_core::crypto::AccountId32,
              ::core::primitive::u128,
            ),
            #[codec(index = 6)]
            Unbonded(
              ::subxt::sp_core::crypto::AccountId32,
              ::core::primitive::u128,
            ),
            #[codec(index = 7)]
            Withdrawn(
              ::subxt::sp_core::crypto::AccountId32,
              ::core::primitive::u128,
            ),
            #[codec(index = 8)]
            Kicked(
              ::subxt::sp_core::crypto::AccountId32,
              ::subxt::sp_core::crypto::AccountId32,
            ),
            #[codec(index = 9)]
            StakingElectionFailed,
            #[codec(index = 10)]
            Chilled(::subxt::sp_core::crypto::AccountId32),
            #[codec(index = 11)]
            PayoutStarted(
              ::core::primitive::u32,
              ::subxt::sp_core::crypto::AccountId32,
            ),
          }
        }
      }
      pub mod slashing {
        use super::runtime_types;
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub struct SlashingSpans {
          pub span_index: ::core::primitive::u32,
          pub last_start: ::core::primitive::u32,
          pub last_nonzero_slash: ::core::primitive::u32,
          pub prior: ::std::vec::Vec<::core::primitive::u32>,
        }
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub struct SpanRecord<_0> {
          pub slashed: _0,
          pub paid_out: _0,
        }
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct ActiveEraInfo {
        pub index: ::core::primitive::u32,
        pub start: ::core::option::Option<::core::primitive::u64>,
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct EraRewardPoints<_0> {
        pub total: ::core::primitive::u32,
        pub individual: ::subxt::KeyedVec<_0, ::core::primitive::u32>,
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Exposure<_0, _1> {
        #[codec(compact)]
        pub total: _1,
        #[codec(compact)]
        pub own: _1,
        pub others: ::std::vec::Vec<runtime_types::pallet_staking::IndividualExposure<_0, _1>>,
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub enum Forcing {
        #[codec(index = 0)]
        NotForcing,
        #[codec(index = 1)]
        ForceNew,
        #[codec(index = 2)]
        ForceNone,
        #[codec(index = 3)]
        ForceAlways,
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct IndividualExposure<_0, _1> {
        pub who: _0,
        #[codec(compact)]
        pub value: _1,
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Nominations {
        pub targets: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
          ::subxt::sp_core::crypto::AccountId32,
        >,
        pub submitted_in: ::core::primitive::u32,
        pub suppressed: ::core::primitive::bool,
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub enum Releases {
        #[codec(index = 0)]
        V1_0_0Ancient,
        #[codec(index = 1)]
        V2_0_0,
        #[codec(index = 2)]
        V3_0_0,
        #[codec(index = 3)]
        V4_0_0,
        #[codec(index = 4)]
        V5_0_0,
        #[codec(index = 5)]
        V6_0_0,
        #[codec(index = 6)]
        V7_0_0,
        #[codec(index = 7)]
        V8_0_0,
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub enum RewardDestination<_0> {
        #[codec(index = 0)]
        Staked,
        #[codec(index = 1)]
        Stash,
        #[codec(index = 2)]
        Controller,
        #[codec(index = 3)]
        Account(_0),
        #[codec(index = 4)]
        None,
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct StakingLedger<_0, _1> {
        pub stash: _0,
        #[codec(compact)]
        pub total: _1,
        #[codec(compact)]
        pub active: _1,
        pub unlocking: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
          runtime_types::pallet_staking::UnlockChunk<_1>,
        >,
        pub claimed_rewards: ::std::vec::Vec<::core::primitive::u32>,
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct UnappliedSlash<_0, _1> {
        pub validator: _0,
        pub own: _1,
        pub others: ::std::vec::Vec<(_0, _1)>,
        pub reporters: ::std::vec::Vec<_0>,
        pub payout: _1,
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct UnlockChunk<_0> {
        #[codec(compact)]
        pub value: _0,
        #[codec(compact)]
        pub era: ::core::primitive::u32,
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct ValidatorPrefs {
        #[codec(compact)]
        pub commission: runtime_types::sp_arithmetic::per_things::Perbill,
        pub blocked: ::core::primitive::bool,
      }
    }
    pub mod pallet_sudo {
      use super::runtime_types;
      pub mod pallet {
        use super::runtime_types;
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Call {
          #[codec(index = 0)]
          sudo {
            call: ::std::boxed::Box<runtime_types::lagoon_runtime::Call>,
          },
          #[codec(index = 1)]
          sudo_unchecked_weight {
            call: ::std::boxed::Box<runtime_types::lagoon_runtime::Call>,
            weight: ::core::primitive::u64,
          },
          #[codec(index = 2)]
          set_key {
            new: ::subxt::sp_runtime::MultiAddress<
              ::subxt::sp_core::crypto::AccountId32,
              ::core::primitive::u32,
            >,
          },
          #[codec(index = 3)]
          sudo_as {
            who: ::subxt::sp_runtime::MultiAddress<
              ::subxt::sp_core::crypto::AccountId32,
              ::core::primitive::u32,
            >,
            call: ::std::boxed::Box<runtime_types::lagoon_runtime::Call>,
          },
        }
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Error {
          #[codec(index = 0)]
          RequireSudo,
        }
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Event {
          #[codec(index = 0)]
          Sudid {
            sudo_result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
          },
          #[codec(index = 1)]
          KeyChanged {
            old_sudoer: ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
          },
          #[codec(index = 2)]
          SudoAsDone {
            sudo_result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
          },
        }
      }
    }
    pub mod pallet_tidefi {
      use super::runtime_types;
      pub mod pallet {
        use super::runtime_types;
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Call {
          #[codec(index = 0)]
          transfer {
            destination_id: ::subxt::sp_core::crypto::AccountId32,
            currency_id: runtime_types::tidefi_primitives::CurrencyId,
            amount: ::core::primitive::u128,
          },
          #[codec(index = 1)]
          withdrawal {
            currency_id: runtime_types::tidefi_primitives::CurrencyId,
            amount: ::core::primitive::u128,
            external_address: ::std::vec::Vec<::core::primitive::u8>,
          },
          #[codec(index = 2)]
          swap {
            currency_id_from: runtime_types::tidefi_primitives::CurrencyId,
            amount_from: ::core::primitive::u128,
            currency_id_to: runtime_types::tidefi_primitives::CurrencyId,
            amount_to: ::core::primitive::u128,
            swap_type: runtime_types::tidefi_primitives::SwapType,
            slippage_tolerance:
              ::core::option::Option<runtime_types::sp_arithmetic::per_things::Permill>,
          },
          #[codec(index = 3)]
          cancel_swap { request_id: ::subxt::sp_core::H256 },
        }
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Error {
          #[codec(index = 0)]
          AssetDisabled,
          #[codec(index = 1)]
          UnknownAsset,
          #[codec(index = 2)]
          NoFunds,
          #[codec(index = 3)]
          UnknownError,
          #[codec(index = 4)]
          QuorumPaused,
          #[codec(index = 5)]
          OraclePaused,
        }
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Event {
          #[codec(index = 0)]
          Transfer {
            from_account_id: ::subxt::sp_core::crypto::AccountId32,
            to_account_id: ::subxt::sp_core::crypto::AccountId32,
            currency_id: runtime_types::tidefi_primitives::CurrencyId,
            amount: ::core::primitive::u128,
          },
          #[codec(index = 1)]
          Withdrawal {
            account: ::subxt::sp_core::crypto::AccountId32,
            currency_id: runtime_types::tidefi_primitives::CurrencyId,
            amount: ::core::primitive::u128,
            external_address: ::std::vec::Vec<::core::primitive::u8>,
          },
          #[codec(index = 2)]
          Swap {
            request_id: ::subxt::sp_core::H256,
            account: ::subxt::sp_core::crypto::AccountId32,
            currency_id_from: runtime_types::tidefi_primitives::CurrencyId,
            amount_from: ::core::primitive::u128,
            currency_id_to: runtime_types::tidefi_primitives::CurrencyId,
            amount_to: ::core::primitive::u128,
            extrinsic_hash: [::core::primitive::u8; 32usize],
            slippage_tolerance: runtime_types::sp_arithmetic::per_things::Permill,
            swap_type: runtime_types::tidefi_primitives::SwapType,
            is_market_maker: ::core::primitive::bool,
          },
          #[codec(index = 3)]
          SwapCancelled { request_id: ::subxt::sp_core::H256 },
        }
      }
    }
    pub mod pallet_tidefi_stake {
      use super::runtime_types;
      pub mod pallet {
        use super::runtime_types;
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Call {
          #[codec(index = 0)]
          stake {
            currency_id: runtime_types::tidefi_primitives::CurrencyId,
            amount: ::core::primitive::u128,
            duration: ::core::primitive::u32,
          },
          #[codec(index = 1)]
          unstake {
            stake_id: ::subxt::sp_core::H256,
            force_unstake: ::core::primitive::bool,
          },
        }
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Error {
          #[codec(index = 0)]
          InvalidDuration,
          #[codec(index = 1)]
          UnstakeQueueCapExceeded,
          #[codec(index = 2)]
          InsufficientBalance,
          #[codec(index = 3)]
          InvalidStakeId,
          #[codec(index = 4)]
          StakingNotReady,
          #[codec(index = 5)]
          TransferFeesFailed,
          #[codec(index = 6)]
          TransferFailed,
          #[codec(index = 7)]
          AmountTooSmall,
          #[codec(index = 8)]
          AmountTooLarge,
        }
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Event {
          #[codec(index = 0)]
          Staked {
            request_id: ::subxt::sp_core::H256,
            account_id: ::subxt::sp_core::crypto::AccountId32,
            currency_id: runtime_types::tidefi_primitives::CurrencyId,
            amount: ::core::primitive::u128,
            duration: ::core::primitive::u32,
          },
          #[codec(index = 1)]
          UnstakeQueued {
            request_id: ::subxt::sp_core::H256,
            account_id: ::subxt::sp_core::crypto::AccountId32,
          },
          #[codec(index = 2)]
          Unstaked {
            request_id: ::subxt::sp_core::H256,
            account_id: ::subxt::sp_core::crypto::AccountId32,
            currency_id: runtime_types::tidefi_primitives::CurrencyId,
            initial_balance: ::core::primitive::u128,
            final_balance: ::core::primitive::u128,
          },
        }
      }
    }
    pub mod pallet_timestamp {
      use super::runtime_types;
      pub mod pallet {
        use super::runtime_types;
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Call {
          #[codec(index = 0)]
          set {
            #[codec(compact)]
            now: ::core::primitive::u64,
          },
        }
      }
    }
    pub mod pallet_transaction_payment {
      use super::runtime_types;
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct ChargeTransactionPayment(#[codec(compact)] pub ::core::primitive::u128);
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub enum Releases {
        #[codec(index = 0)]
        V1Ancient,
        #[codec(index = 1)]
        V2,
      }
    }
    pub mod pallet_treasury {
      use super::runtime_types;
      pub mod pallet {
        use super::runtime_types;
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Call {
          #[codec(index = 0)]
          propose_spend {
            #[codec(compact)]
            value: ::core::primitive::u128,
            beneficiary: ::subxt::sp_runtime::MultiAddress<
              ::subxt::sp_core::crypto::AccountId32,
              ::core::primitive::u32,
            >,
          },
          #[codec(index = 1)]
          reject_proposal {
            #[codec(compact)]
            proposal_id: ::core::primitive::u32,
          },
          #[codec(index = 2)]
          approve_proposal {
            #[codec(compact)]
            proposal_id: ::core::primitive::u32,
          },
        }
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Error {
          #[codec(index = 0)]
          InsufficientProposersBalance,
          #[codec(index = 1)]
          InvalidIndex,
          #[codec(index = 2)]
          TooManyApprovals,
        }
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Event {
          #[codec(index = 0)]
          Proposed {
            proposal_index: ::core::primitive::u32,
          },
          #[codec(index = 1)]
          Spending {
            budget_remaining: ::core::primitive::u128,
          },
          #[codec(index = 2)]
          Awarded {
            proposal_index: ::core::primitive::u32,
            award: ::core::primitive::u128,
            account: ::subxt::sp_core::crypto::AccountId32,
          },
          #[codec(index = 3)]
          Rejected {
            proposal_index: ::core::primitive::u32,
            slashed: ::core::primitive::u128,
          },
          #[codec(index = 4)]
          Burnt {
            burnt_funds: ::core::primitive::u128,
          },
          #[codec(index = 5)]
          Rollover {
            rollover_balance: ::core::primitive::u128,
          },
          #[codec(index = 6)]
          Deposit { value: ::core::primitive::u128 },
        }
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Proposal<_0, _1> {
        pub proposer: _0,
        pub value: _1,
        pub beneficiary: _0,
        pub bond: _1,
      }
    }
    pub mod pallet_utility {
      use super::runtime_types;
      pub mod pallet {
        use super::runtime_types;
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Call {
          #[codec(index = 0)]
          batch {
            calls: ::std::vec::Vec<runtime_types::lagoon_runtime::Call>,
          },
          #[codec(index = 1)]
          as_derivative {
            index: ::core::primitive::u16,
            call: ::std::boxed::Box<runtime_types::lagoon_runtime::Call>,
          },
          #[codec(index = 2)]
          batch_all {
            calls: ::std::vec::Vec<runtime_types::lagoon_runtime::Call>,
          },
          #[codec(index = 3)]
          dispatch_as {
            as_origin: ::std::boxed::Box<runtime_types::lagoon_runtime::OriginCaller>,
            call: ::std::boxed::Box<runtime_types::lagoon_runtime::Call>,
          },
        }
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Error {
          #[codec(index = 0)]
          TooManyCalls,
        }
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum Event {
          #[codec(index = 0)]
          BatchInterrupted {
            index: ::core::primitive::u32,
            error: runtime_types::sp_runtime::DispatchError,
          },
          #[codec(index = 1)]
          BatchCompleted,
          #[codec(index = 2)]
          ItemCompleted,
          #[codec(index = 3)]
          DispatchedAs {
            result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
          },
        }
      }
    }
    pub mod primitive_types {
      use super::runtime_types;
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct H256(pub [::core::primitive::u8; 32usize]);
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct U256(pub [::core::primitive::u64; 4usize]);
    }
    pub mod sp_arithmetic {
      use super::runtime_types;
      pub mod fixed_point {
        use super::runtime_types;
        #[derive(
          :: subxt :: codec :: Encode,
          :: subxt :: codec :: Decode,
          Debug,
          :: subxt :: codec :: CompactAs,
        )]
        pub struct FixedU128(pub ::core::primitive::u128);
      }
      pub mod per_things {
        use super::runtime_types;
        #[derive(
          :: subxt :: codec :: Encode,
          :: subxt :: codec :: Decode,
          Debug,
          :: subxt :: codec :: CompactAs,
        )]
        pub struct PerU16(pub ::core::primitive::u16);
        #[derive(
          :: subxt :: codec :: Encode,
          :: subxt :: codec :: Decode,
          Debug,
          :: subxt :: codec :: CompactAs,
        )]
        pub struct Perbill(pub ::core::primitive::u32);
        #[derive(
          :: subxt :: codec :: Encode,
          :: subxt :: codec :: Decode,
          Debug,
          :: subxt :: codec :: CompactAs,
        )]
        pub struct Percent(pub ::core::primitive::u8);
        #[derive(
          :: subxt :: codec :: Encode,
          :: subxt :: codec :: Decode,
          Debug,
          :: subxt :: codec :: CompactAs,
        )]
        pub struct Permill(pub ::core::primitive::u32);
      }
    }
    pub mod sp_authority_discovery {
      use super::runtime_types;
      pub mod app {
        use super::runtime_types;
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub struct Public(pub runtime_types::sp_core::sr25519::Public);
      }
    }
    pub mod sp_consensus_babe {
      use super::runtime_types;
      pub mod app {
        use super::runtime_types;
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub struct Public(pub runtime_types::sp_core::sr25519::Public);
      }
      pub mod digests {
        use super::runtime_types;
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum NextConfigDescriptor {
          #[codec(index = 1)]
          V1 {
            c: (::core::primitive::u64, ::core::primitive::u64),
            allowed_slots: runtime_types::sp_consensus_babe::AllowedSlots,
          },
        }
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub enum AllowedSlots {
        #[codec(index = 0)]
        PrimarySlots,
        #[codec(index = 1)]
        PrimaryAndSecondaryPlainSlots,
        #[codec(index = 2)]
        PrimaryAndSecondaryVRFSlots,
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct BabeEpochConfiguration {
        pub c: (::core::primitive::u64, ::core::primitive::u64),
        pub allowed_slots: runtime_types::sp_consensus_babe::AllowedSlots,
      }
    }
    pub mod sp_consensus_slots {
      use super::runtime_types;
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct EquivocationProof<_0, _1> {
        pub offender: _1,
        pub slot: runtime_types::sp_consensus_slots::Slot,
        pub first_header: _0,
        pub second_header: _0,
      }
      #[derive(
        :: subxt :: codec :: Encode,
        :: subxt :: codec :: Decode,
        Debug,
        :: subxt :: codec :: CompactAs,
      )]
      pub struct Slot(pub ::core::primitive::u64);
    }
    pub mod sp_core {
      use super::runtime_types;
      pub mod crypto {
        use super::runtime_types;
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub struct AccountId32(pub [::core::primitive::u8; 32usize]);
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub struct KeyTypeId(pub [::core::primitive::u8; 4usize]);
      }
      pub mod ecdsa {
        use super::runtime_types;
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub struct Signature(pub [::core::primitive::u8; 65usize]);
      }
      pub mod ed25519 {
        use super::runtime_types;
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub struct Public(pub [::core::primitive::u8; 32usize]);
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub struct Signature(pub [::core::primitive::u8; 64usize]);
      }
      pub mod offchain {
        use super::runtime_types;
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub struct OpaqueMultiaddr(pub ::std::vec::Vec<::core::primitive::u8>);
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub struct OpaqueNetworkState {
          pub peer_id: runtime_types::sp_core::OpaquePeerId,
          pub external_addresses:
            ::std::vec::Vec<runtime_types::sp_core::offchain::OpaqueMultiaddr>,
        }
      }
      pub mod sr25519 {
        use super::runtime_types;
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub struct Public(pub [::core::primitive::u8; 32usize]);
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub struct Signature(pub [::core::primitive::u8; 64usize]);
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct OpaquePeerId(pub ::std::vec::Vec<::core::primitive::u8>);
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub enum Void {}
    }
    pub mod sp_finality_grandpa {
      use super::runtime_types;
      pub mod app {
        use super::runtime_types;
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub struct Public(pub runtime_types::sp_core::ed25519::Public);
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub struct Signature(pub runtime_types::sp_core::ed25519::Signature);
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub enum Equivocation<_0, _1> {
        #[codec(index = 0)]
        Prevote(
          runtime_types::finality_grandpa::Equivocation<
            runtime_types::sp_finality_grandpa::app::Public,
            runtime_types::finality_grandpa::Prevote<_0, _1>,
            runtime_types::sp_finality_grandpa::app::Signature,
          >,
        ),
        #[codec(index = 1)]
        Precommit(
          runtime_types::finality_grandpa::Equivocation<
            runtime_types::sp_finality_grandpa::app::Public,
            runtime_types::finality_grandpa::Precommit<_0, _1>,
            runtime_types::sp_finality_grandpa::app::Signature,
          >,
        ),
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct EquivocationProof<_0, _1> {
        pub set_id: ::core::primitive::u64,
        pub equivocation: runtime_types::sp_finality_grandpa::Equivocation<_0, _1>,
      }
    }
    pub mod sp_npos_elections {
      use super::runtime_types;
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct ElectionScore {
        pub minimal_stake: ::core::primitive::u128,
        pub sum_stake: ::core::primitive::u128,
        pub sum_stake_squared: ::core::primitive::u128,
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Support<_0> {
        pub total: ::core::primitive::u128,
        pub voters: ::std::vec::Vec<(_0, ::core::primitive::u128)>,
      }
    }
    pub mod sp_runtime {
      use super::runtime_types;
      pub mod generic {
        use super::runtime_types;
        pub mod digest {
          use super::runtime_types;
          #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
          pub struct Digest {
            pub logs: ::std::vec::Vec<runtime_types::sp_runtime::generic::digest::DigestItem>,
          }
          #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
          pub enum DigestItem {
            #[codec(index = 6)]
            PreRuntime(
              [::core::primitive::u8; 4usize],
              ::std::vec::Vec<::core::primitive::u8>,
            ),
            #[codec(index = 4)]
            Consensus(
              [::core::primitive::u8; 4usize],
              ::std::vec::Vec<::core::primitive::u8>,
            ),
            #[codec(index = 5)]
            Seal(
              [::core::primitive::u8; 4usize],
              ::std::vec::Vec<::core::primitive::u8>,
            ),
            #[codec(index = 0)]
            Other(::std::vec::Vec<::core::primitive::u8>),
            #[codec(index = 8)]
            RuntimeEnvironmentUpdated,
          }
        }
        pub mod era {
          use super::runtime_types;
          #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
          pub enum Era {
            #[codec(index = 0)]
            Immortal,
            #[codec(index = 1)]
            Mortal1(::core::primitive::u8),
            #[codec(index = 2)]
            Mortal2(::core::primitive::u8),
            #[codec(index = 3)]
            Mortal3(::core::primitive::u8),
            #[codec(index = 4)]
            Mortal4(::core::primitive::u8),
            #[codec(index = 5)]
            Mortal5(::core::primitive::u8),
            #[codec(index = 6)]
            Mortal6(::core::primitive::u8),
            #[codec(index = 7)]
            Mortal7(::core::primitive::u8),
            #[codec(index = 8)]
            Mortal8(::core::primitive::u8),
            #[codec(index = 9)]
            Mortal9(::core::primitive::u8),
            #[codec(index = 10)]
            Mortal10(::core::primitive::u8),
            #[codec(index = 11)]
            Mortal11(::core::primitive::u8),
            #[codec(index = 12)]
            Mortal12(::core::primitive::u8),
            #[codec(index = 13)]
            Mortal13(::core::primitive::u8),
            #[codec(index = 14)]
            Mortal14(::core::primitive::u8),
            #[codec(index = 15)]
            Mortal15(::core::primitive::u8),
            #[codec(index = 16)]
            Mortal16(::core::primitive::u8),
            #[codec(index = 17)]
            Mortal17(::core::primitive::u8),
            #[codec(index = 18)]
            Mortal18(::core::primitive::u8),
            #[codec(index = 19)]
            Mortal19(::core::primitive::u8),
            #[codec(index = 20)]
            Mortal20(::core::primitive::u8),
            #[codec(index = 21)]
            Mortal21(::core::primitive::u8),
            #[codec(index = 22)]
            Mortal22(::core::primitive::u8),
            #[codec(index = 23)]
            Mortal23(::core::primitive::u8),
            #[codec(index = 24)]
            Mortal24(::core::primitive::u8),
            #[codec(index = 25)]
            Mortal25(::core::primitive::u8),
            #[codec(index = 26)]
            Mortal26(::core::primitive::u8),
            #[codec(index = 27)]
            Mortal27(::core::primitive::u8),
            #[codec(index = 28)]
            Mortal28(::core::primitive::u8),
            #[codec(index = 29)]
            Mortal29(::core::primitive::u8),
            #[codec(index = 30)]
            Mortal30(::core::primitive::u8),
            #[codec(index = 31)]
            Mortal31(::core::primitive::u8),
            #[codec(index = 32)]
            Mortal32(::core::primitive::u8),
            #[codec(index = 33)]
            Mortal33(::core::primitive::u8),
            #[codec(index = 34)]
            Mortal34(::core::primitive::u8),
            #[codec(index = 35)]
            Mortal35(::core::primitive::u8),
            #[codec(index = 36)]
            Mortal36(::core::primitive::u8),
            #[codec(index = 37)]
            Mortal37(::core::primitive::u8),
            #[codec(index = 38)]
            Mortal38(::core::primitive::u8),
            #[codec(index = 39)]
            Mortal39(::core::primitive::u8),
            #[codec(index = 40)]
            Mortal40(::core::primitive::u8),
            #[codec(index = 41)]
            Mortal41(::core::primitive::u8),
            #[codec(index = 42)]
            Mortal42(::core::primitive::u8),
            #[codec(index = 43)]
            Mortal43(::core::primitive::u8),
            #[codec(index = 44)]
            Mortal44(::core::primitive::u8),
            #[codec(index = 45)]
            Mortal45(::core::primitive::u8),
            #[codec(index = 46)]
            Mortal46(::core::primitive::u8),
            #[codec(index = 47)]
            Mortal47(::core::primitive::u8),
            #[codec(index = 48)]
            Mortal48(::core::primitive::u8),
            #[codec(index = 49)]
            Mortal49(::core::primitive::u8),
            #[codec(index = 50)]
            Mortal50(::core::primitive::u8),
            #[codec(index = 51)]
            Mortal51(::core::primitive::u8),
            #[codec(index = 52)]
            Mortal52(::core::primitive::u8),
            #[codec(index = 53)]
            Mortal53(::core::primitive::u8),
            #[codec(index = 54)]
            Mortal54(::core::primitive::u8),
            #[codec(index = 55)]
            Mortal55(::core::primitive::u8),
            #[codec(index = 56)]
            Mortal56(::core::primitive::u8),
            #[codec(index = 57)]
            Mortal57(::core::primitive::u8),
            #[codec(index = 58)]
            Mortal58(::core::primitive::u8),
            #[codec(index = 59)]
            Mortal59(::core::primitive::u8),
            #[codec(index = 60)]
            Mortal60(::core::primitive::u8),
            #[codec(index = 61)]
            Mortal61(::core::primitive::u8),
            #[codec(index = 62)]
            Mortal62(::core::primitive::u8),
            #[codec(index = 63)]
            Mortal63(::core::primitive::u8),
            #[codec(index = 64)]
            Mortal64(::core::primitive::u8),
            #[codec(index = 65)]
            Mortal65(::core::primitive::u8),
            #[codec(index = 66)]
            Mortal66(::core::primitive::u8),
            #[codec(index = 67)]
            Mortal67(::core::primitive::u8),
            #[codec(index = 68)]
            Mortal68(::core::primitive::u8),
            #[codec(index = 69)]
            Mortal69(::core::primitive::u8),
            #[codec(index = 70)]
            Mortal70(::core::primitive::u8),
            #[codec(index = 71)]
            Mortal71(::core::primitive::u8),
            #[codec(index = 72)]
            Mortal72(::core::primitive::u8),
            #[codec(index = 73)]
            Mortal73(::core::primitive::u8),
            #[codec(index = 74)]
            Mortal74(::core::primitive::u8),
            #[codec(index = 75)]
            Mortal75(::core::primitive::u8),
            #[codec(index = 76)]
            Mortal76(::core::primitive::u8),
            #[codec(index = 77)]
            Mortal77(::core::primitive::u8),
            #[codec(index = 78)]
            Mortal78(::core::primitive::u8),
            #[codec(index = 79)]
            Mortal79(::core::primitive::u8),
            #[codec(index = 80)]
            Mortal80(::core::primitive::u8),
            #[codec(index = 81)]
            Mortal81(::core::primitive::u8),
            #[codec(index = 82)]
            Mortal82(::core::primitive::u8),
            #[codec(index = 83)]
            Mortal83(::core::primitive::u8),
            #[codec(index = 84)]
            Mortal84(::core::primitive::u8),
            #[codec(index = 85)]
            Mortal85(::core::primitive::u8),
            #[codec(index = 86)]
            Mortal86(::core::primitive::u8),
            #[codec(index = 87)]
            Mortal87(::core::primitive::u8),
            #[codec(index = 88)]
            Mortal88(::core::primitive::u8),
            #[codec(index = 89)]
            Mortal89(::core::primitive::u8),
            #[codec(index = 90)]
            Mortal90(::core::primitive::u8),
            #[codec(index = 91)]
            Mortal91(::core::primitive::u8),
            #[codec(index = 92)]
            Mortal92(::core::primitive::u8),
            #[codec(index = 93)]
            Mortal93(::core::primitive::u8),
            #[codec(index = 94)]
            Mortal94(::core::primitive::u8),
            #[codec(index = 95)]
            Mortal95(::core::primitive::u8),
            #[codec(index = 96)]
            Mortal96(::core::primitive::u8),
            #[codec(index = 97)]
            Mortal97(::core::primitive::u8),
            #[codec(index = 98)]
            Mortal98(::core::primitive::u8),
            #[codec(index = 99)]
            Mortal99(::core::primitive::u8),
            #[codec(index = 100)]
            Mortal100(::core::primitive::u8),
            #[codec(index = 101)]
            Mortal101(::core::primitive::u8),
            #[codec(index = 102)]
            Mortal102(::core::primitive::u8),
            #[codec(index = 103)]
            Mortal103(::core::primitive::u8),
            #[codec(index = 104)]
            Mortal104(::core::primitive::u8),
            #[codec(index = 105)]
            Mortal105(::core::primitive::u8),
            #[codec(index = 106)]
            Mortal106(::core::primitive::u8),
            #[codec(index = 107)]
            Mortal107(::core::primitive::u8),
            #[codec(index = 108)]
            Mortal108(::core::primitive::u8),
            #[codec(index = 109)]
            Mortal109(::core::primitive::u8),
            #[codec(index = 110)]
            Mortal110(::core::primitive::u8),
            #[codec(index = 111)]
            Mortal111(::core::primitive::u8),
            #[codec(index = 112)]
            Mortal112(::core::primitive::u8),
            #[codec(index = 113)]
            Mortal113(::core::primitive::u8),
            #[codec(index = 114)]
            Mortal114(::core::primitive::u8),
            #[codec(index = 115)]
            Mortal115(::core::primitive::u8),
            #[codec(index = 116)]
            Mortal116(::core::primitive::u8),
            #[codec(index = 117)]
            Mortal117(::core::primitive::u8),
            #[codec(index = 118)]
            Mortal118(::core::primitive::u8),
            #[codec(index = 119)]
            Mortal119(::core::primitive::u8),
            #[codec(index = 120)]
            Mortal120(::core::primitive::u8),
            #[codec(index = 121)]
            Mortal121(::core::primitive::u8),
            #[codec(index = 122)]
            Mortal122(::core::primitive::u8),
            #[codec(index = 123)]
            Mortal123(::core::primitive::u8),
            #[codec(index = 124)]
            Mortal124(::core::primitive::u8),
            #[codec(index = 125)]
            Mortal125(::core::primitive::u8),
            #[codec(index = 126)]
            Mortal126(::core::primitive::u8),
            #[codec(index = 127)]
            Mortal127(::core::primitive::u8),
            #[codec(index = 128)]
            Mortal128(::core::primitive::u8),
            #[codec(index = 129)]
            Mortal129(::core::primitive::u8),
            #[codec(index = 130)]
            Mortal130(::core::primitive::u8),
            #[codec(index = 131)]
            Mortal131(::core::primitive::u8),
            #[codec(index = 132)]
            Mortal132(::core::primitive::u8),
            #[codec(index = 133)]
            Mortal133(::core::primitive::u8),
            #[codec(index = 134)]
            Mortal134(::core::primitive::u8),
            #[codec(index = 135)]
            Mortal135(::core::primitive::u8),
            #[codec(index = 136)]
            Mortal136(::core::primitive::u8),
            #[codec(index = 137)]
            Mortal137(::core::primitive::u8),
            #[codec(index = 138)]
            Mortal138(::core::primitive::u8),
            #[codec(index = 139)]
            Mortal139(::core::primitive::u8),
            #[codec(index = 140)]
            Mortal140(::core::primitive::u8),
            #[codec(index = 141)]
            Mortal141(::core::primitive::u8),
            #[codec(index = 142)]
            Mortal142(::core::primitive::u8),
            #[codec(index = 143)]
            Mortal143(::core::primitive::u8),
            #[codec(index = 144)]
            Mortal144(::core::primitive::u8),
            #[codec(index = 145)]
            Mortal145(::core::primitive::u8),
            #[codec(index = 146)]
            Mortal146(::core::primitive::u8),
            #[codec(index = 147)]
            Mortal147(::core::primitive::u8),
            #[codec(index = 148)]
            Mortal148(::core::primitive::u8),
            #[codec(index = 149)]
            Mortal149(::core::primitive::u8),
            #[codec(index = 150)]
            Mortal150(::core::primitive::u8),
            #[codec(index = 151)]
            Mortal151(::core::primitive::u8),
            #[codec(index = 152)]
            Mortal152(::core::primitive::u8),
            #[codec(index = 153)]
            Mortal153(::core::primitive::u8),
            #[codec(index = 154)]
            Mortal154(::core::primitive::u8),
            #[codec(index = 155)]
            Mortal155(::core::primitive::u8),
            #[codec(index = 156)]
            Mortal156(::core::primitive::u8),
            #[codec(index = 157)]
            Mortal157(::core::primitive::u8),
            #[codec(index = 158)]
            Mortal158(::core::primitive::u8),
            #[codec(index = 159)]
            Mortal159(::core::primitive::u8),
            #[codec(index = 160)]
            Mortal160(::core::primitive::u8),
            #[codec(index = 161)]
            Mortal161(::core::primitive::u8),
            #[codec(index = 162)]
            Mortal162(::core::primitive::u8),
            #[codec(index = 163)]
            Mortal163(::core::primitive::u8),
            #[codec(index = 164)]
            Mortal164(::core::primitive::u8),
            #[codec(index = 165)]
            Mortal165(::core::primitive::u8),
            #[codec(index = 166)]
            Mortal166(::core::primitive::u8),
            #[codec(index = 167)]
            Mortal167(::core::primitive::u8),
            #[codec(index = 168)]
            Mortal168(::core::primitive::u8),
            #[codec(index = 169)]
            Mortal169(::core::primitive::u8),
            #[codec(index = 170)]
            Mortal170(::core::primitive::u8),
            #[codec(index = 171)]
            Mortal171(::core::primitive::u8),
            #[codec(index = 172)]
            Mortal172(::core::primitive::u8),
            #[codec(index = 173)]
            Mortal173(::core::primitive::u8),
            #[codec(index = 174)]
            Mortal174(::core::primitive::u8),
            #[codec(index = 175)]
            Mortal175(::core::primitive::u8),
            #[codec(index = 176)]
            Mortal176(::core::primitive::u8),
            #[codec(index = 177)]
            Mortal177(::core::primitive::u8),
            #[codec(index = 178)]
            Mortal178(::core::primitive::u8),
            #[codec(index = 179)]
            Mortal179(::core::primitive::u8),
            #[codec(index = 180)]
            Mortal180(::core::primitive::u8),
            #[codec(index = 181)]
            Mortal181(::core::primitive::u8),
            #[codec(index = 182)]
            Mortal182(::core::primitive::u8),
            #[codec(index = 183)]
            Mortal183(::core::primitive::u8),
            #[codec(index = 184)]
            Mortal184(::core::primitive::u8),
            #[codec(index = 185)]
            Mortal185(::core::primitive::u8),
            #[codec(index = 186)]
            Mortal186(::core::primitive::u8),
            #[codec(index = 187)]
            Mortal187(::core::primitive::u8),
            #[codec(index = 188)]
            Mortal188(::core::primitive::u8),
            #[codec(index = 189)]
            Mortal189(::core::primitive::u8),
            #[codec(index = 190)]
            Mortal190(::core::primitive::u8),
            #[codec(index = 191)]
            Mortal191(::core::primitive::u8),
            #[codec(index = 192)]
            Mortal192(::core::primitive::u8),
            #[codec(index = 193)]
            Mortal193(::core::primitive::u8),
            #[codec(index = 194)]
            Mortal194(::core::primitive::u8),
            #[codec(index = 195)]
            Mortal195(::core::primitive::u8),
            #[codec(index = 196)]
            Mortal196(::core::primitive::u8),
            #[codec(index = 197)]
            Mortal197(::core::primitive::u8),
            #[codec(index = 198)]
            Mortal198(::core::primitive::u8),
            #[codec(index = 199)]
            Mortal199(::core::primitive::u8),
            #[codec(index = 200)]
            Mortal200(::core::primitive::u8),
            #[codec(index = 201)]
            Mortal201(::core::primitive::u8),
            #[codec(index = 202)]
            Mortal202(::core::primitive::u8),
            #[codec(index = 203)]
            Mortal203(::core::primitive::u8),
            #[codec(index = 204)]
            Mortal204(::core::primitive::u8),
            #[codec(index = 205)]
            Mortal205(::core::primitive::u8),
            #[codec(index = 206)]
            Mortal206(::core::primitive::u8),
            #[codec(index = 207)]
            Mortal207(::core::primitive::u8),
            #[codec(index = 208)]
            Mortal208(::core::primitive::u8),
            #[codec(index = 209)]
            Mortal209(::core::primitive::u8),
            #[codec(index = 210)]
            Mortal210(::core::primitive::u8),
            #[codec(index = 211)]
            Mortal211(::core::primitive::u8),
            #[codec(index = 212)]
            Mortal212(::core::primitive::u8),
            #[codec(index = 213)]
            Mortal213(::core::primitive::u8),
            #[codec(index = 214)]
            Mortal214(::core::primitive::u8),
            #[codec(index = 215)]
            Mortal215(::core::primitive::u8),
            #[codec(index = 216)]
            Mortal216(::core::primitive::u8),
            #[codec(index = 217)]
            Mortal217(::core::primitive::u8),
            #[codec(index = 218)]
            Mortal218(::core::primitive::u8),
            #[codec(index = 219)]
            Mortal219(::core::primitive::u8),
            #[codec(index = 220)]
            Mortal220(::core::primitive::u8),
            #[codec(index = 221)]
            Mortal221(::core::primitive::u8),
            #[codec(index = 222)]
            Mortal222(::core::primitive::u8),
            #[codec(index = 223)]
            Mortal223(::core::primitive::u8),
            #[codec(index = 224)]
            Mortal224(::core::primitive::u8),
            #[codec(index = 225)]
            Mortal225(::core::primitive::u8),
            #[codec(index = 226)]
            Mortal226(::core::primitive::u8),
            #[codec(index = 227)]
            Mortal227(::core::primitive::u8),
            #[codec(index = 228)]
            Mortal228(::core::primitive::u8),
            #[codec(index = 229)]
            Mortal229(::core::primitive::u8),
            #[codec(index = 230)]
            Mortal230(::core::primitive::u8),
            #[codec(index = 231)]
            Mortal231(::core::primitive::u8),
            #[codec(index = 232)]
            Mortal232(::core::primitive::u8),
            #[codec(index = 233)]
            Mortal233(::core::primitive::u8),
            #[codec(index = 234)]
            Mortal234(::core::primitive::u8),
            #[codec(index = 235)]
            Mortal235(::core::primitive::u8),
            #[codec(index = 236)]
            Mortal236(::core::primitive::u8),
            #[codec(index = 237)]
            Mortal237(::core::primitive::u8),
            #[codec(index = 238)]
            Mortal238(::core::primitive::u8),
            #[codec(index = 239)]
            Mortal239(::core::primitive::u8),
            #[codec(index = 240)]
            Mortal240(::core::primitive::u8),
            #[codec(index = 241)]
            Mortal241(::core::primitive::u8),
            #[codec(index = 242)]
            Mortal242(::core::primitive::u8),
            #[codec(index = 243)]
            Mortal243(::core::primitive::u8),
            #[codec(index = 244)]
            Mortal244(::core::primitive::u8),
            #[codec(index = 245)]
            Mortal245(::core::primitive::u8),
            #[codec(index = 246)]
            Mortal246(::core::primitive::u8),
            #[codec(index = 247)]
            Mortal247(::core::primitive::u8),
            #[codec(index = 248)]
            Mortal248(::core::primitive::u8),
            #[codec(index = 249)]
            Mortal249(::core::primitive::u8),
            #[codec(index = 250)]
            Mortal250(::core::primitive::u8),
            #[codec(index = 251)]
            Mortal251(::core::primitive::u8),
            #[codec(index = 252)]
            Mortal252(::core::primitive::u8),
            #[codec(index = 253)]
            Mortal253(::core::primitive::u8),
            #[codec(index = 254)]
            Mortal254(::core::primitive::u8),
            #[codec(index = 255)]
            Mortal255(::core::primitive::u8),
          }
        }
        pub mod header {
          use super::runtime_types;
          #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
          pub struct Header<_0, _1> {
            pub parent_hash: ::subxt::sp_core::H256,
            #[codec(compact)]
            pub number: _0,
            pub state_root: ::subxt::sp_core::H256,
            pub extrinsics_root: ::subxt::sp_core::H256,
            pub digest: runtime_types::sp_runtime::generic::digest::Digest,
            #[codec(skip)]
            pub __subxt_unused_type_params: ::core::marker::PhantomData<_1>,
          }
        }
        pub mod unchecked_extrinsic {
          use super::runtime_types;
          #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
          pub struct UncheckedExtrinsic<_0, _1, _2, _3>(
            pub ::std::vec::Vec<::core::primitive::u8>,
            #[codec(skip)] pub ::core::marker::PhantomData<(_0, _1, _2, _3)>,
          );
        }
      }
      pub mod multiaddress {
        use super::runtime_types;
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub enum MultiAddress<_0, _1> {
          #[codec(index = 0)]
          Id(_0),
          #[codec(index = 1)]
          Index(#[codec(compact)] _1),
          #[codec(index = 2)]
          Raw(::std::vec::Vec<::core::primitive::u8>),
          #[codec(index = 3)]
          Address32([::core::primitive::u8; 32usize]),
          #[codec(index = 4)]
          Address20([::core::primitive::u8; 20usize]),
        }
      }
      pub mod traits {
        use super::runtime_types;
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub struct BlakeTwo256;
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub enum ArithmeticError {
        #[codec(index = 0)]
        Underflow,
        #[codec(index = 1)]
        Overflow,
        #[codec(index = 2)]
        DivisionByZero,
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub enum DispatchError {
        #[codec(index = 0)]
        Other,
        #[codec(index = 1)]
        CannotLookup,
        #[codec(index = 2)]
        BadOrigin,
        #[codec(index = 3)]
        Module(runtime_types::sp_runtime::ModuleError),
        #[codec(index = 4)]
        ConsumerRemaining,
        #[codec(index = 5)]
        NoProviders,
        #[codec(index = 6)]
        TooManyConsumers,
        #[codec(index = 7)]
        Token(runtime_types::sp_runtime::TokenError),
        #[codec(index = 8)]
        Arithmetic(runtime_types::sp_runtime::ArithmeticError),
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct ModuleError {
        pub index: ::core::primitive::u8,
        pub error: ::core::primitive::u8,
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub enum MultiSignature {
        #[codec(index = 0)]
        Ed25519(runtime_types::sp_core::ed25519::Signature),
        #[codec(index = 1)]
        Sr25519(runtime_types::sp_core::sr25519::Signature),
        #[codec(index = 2)]
        Ecdsa(runtime_types::sp_core::ecdsa::Signature),
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub enum TokenError {
        #[codec(index = 0)]
        NoFunds,
        #[codec(index = 1)]
        WouldDie,
        #[codec(index = 2)]
        BelowMinimum,
        #[codec(index = 3)]
        CannotCreate,
        #[codec(index = 4)]
        UnknownAsset,
        #[codec(index = 5)]
        Frozen,
        #[codec(index = 6)]
        Unsupported,
      }
    }
    pub mod sp_session {
      use super::runtime_types;
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct MembershipProof {
        pub session: ::core::primitive::u32,
        pub trie_nodes: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
        pub validator_count: ::core::primitive::u32,
      }
    }
    pub mod sp_staking {
      use super::runtime_types;
      pub mod offence {
        use super::runtime_types;
        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
        pub struct OffenceDetails<_0, _1> {
          pub offender: _1,
          pub reporters: ::std::vec::Vec<_0>,
        }
      }
    }
    pub mod sp_version {
      use super::runtime_types;
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct RuntimeVersion {
        pub spec_name: ::std::string::String,
        pub impl_name: ::std::string::String,
        pub authoring_version: ::core::primitive::u32,
        pub spec_version: ::core::primitive::u32,
        pub impl_version: ::core::primitive::u32,
        pub apis: ::std::vec::Vec<([::core::primitive::u8; 8usize], ::core::primitive::u32)>,
        pub transaction_version: ::core::primitive::u32,
        pub state_version: ::core::primitive::u8,
      }
    }
    pub mod tidefi_primitives {
      use super::runtime_types;
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct ActiveEraInfo<_0> {
        pub index: _0,
        pub start_block: ::core::option::Option<_0>,
        pub start_session_index: ::core::option::Option<::core::primitive::u64>,
        pub last_session_block: ::core::option::Option<_0>,
        pub start: ::core::option::Option<::core::primitive::u64>,
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub enum ComplianceLevel {
        #[codec(index = 0)]
        Green,
        #[codec(index = 1)]
        Amber,
        #[codec(index = 2)]
        Red,
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub enum CurrencyId {
        #[codec(index = 0)]
        Tide,
        #[codec(index = 1)]
        Wrapped(::core::primitive::u32),
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Fee {
        pub amount: ::core::primitive::u128,
        pub fee: ::core::primitive::u128,
        pub fee_usdt: ::core::primitive::u128,
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Mint<_0, _1> {
        pub account_id: _0,
        pub currency_id: runtime_types::tidefi_primitives::CurrencyId,
        pub mint_amount: ::core::primitive::u128,
        pub transaction_id: _1,
        pub compliance_level: runtime_types::tidefi_primitives::ComplianceLevel,
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct OracleImAlive {
        pub usdt_value: ::std::vec::Vec<(
          runtime_types::tidefi_primitives::CurrencyId,
          ::core::primitive::u128,
        )>,
        pub tide_value: ::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u128)>,
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub enum ProposalStatus {
        #[codec(index = 0)]
        Initiated,
        #[codec(index = 1)]
        Approved,
        #[codec(index = 2)]
        Rejected,
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub enum ProposalType<_0, _1, _2, _3> {
        #[codec(index = 0)]
        Mint(runtime_types::tidefi_primitives::Mint<_0, _2>),
        #[codec(index = 1)]
        Withdrawal(runtime_types::tidefi_primitives::Withdrawal<_0, _1, _2>),
        #[codec(index = 2)]
        UpdateConfiguration(_3, ::core::primitive::u16),
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct ProposalVotes<_0, _1> {
        pub votes_for: _1,
        pub votes_against: _1,
        pub status: runtime_types::tidefi_primitives::ProposalStatus,
        pub expiry: _0,
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Stake<_0, _1> {
        pub currency_id: runtime_types::tidefi_primitives::CurrencyId,
        pub unique_id: ::subxt::sp_core::H256,
        pub last_session_index_compound: ::core::primitive::u64,
        pub initial_block: _1,
        pub initial_balance: _0,
        pub principal: _0,
        pub duration: _1,
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct StakeCurrencyMeta<_0> {
        pub minimum_amount: _0,
        pub maximum_amount: _0,
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub enum StatusCode {
        #[codec(index = 0)]
        Running,
        #[codec(index = 1)]
        Maintenance,
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct SunriseSwapPool {
        pub id: ::core::primitive::u8,
        pub minimum_usdt_value: ::core::primitive::u128,
        pub transactions_remaining: ::core::primitive::u32,
        pub balance: ::core::primitive::u128,
        pub rebates: runtime_types::sp_arithmetic::fixed_point::FixedU128,
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Swap<_0, _1> {
        pub extrinsic_hash: [::core::primitive::u8; 32usize],
        pub account_id: _0,
        pub is_market_maker: ::core::primitive::bool,
        pub token_from: runtime_types::tidefi_primitives::CurrencyId,
        pub amount_from: ::core::primitive::u128,
        pub amount_from_filled: ::core::primitive::u128,
        pub token_to: runtime_types::tidefi_primitives::CurrencyId,
        pub amount_to: ::core::primitive::u128,
        pub amount_to_filled: ::core::primitive::u128,
        pub status: runtime_types::tidefi_primitives::SwapStatus,
        pub swap_type: runtime_types::tidefi_primitives::SwapType,
        pub block_number: _1,
        pub slippage: runtime_types::sp_arithmetic::per_things::Permill,
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct SwapConfirmation {
        pub request_id: ::subxt::sp_core::H256,
        pub amount_to_receive: ::core::primitive::u128,
        pub amount_to_send: ::core::primitive::u128,
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub enum SwapStatus {
        #[codec(index = 0)]
        Pending,
        #[codec(index = 1)]
        Cancelled,
        #[codec(index = 2)]
        PartiallyFilled,
        #[codec(index = 3)]
        Completed,
        #[codec(index = 4)]
        Rejected,
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub enum SwapType {
        #[codec(index = 0)]
        Market,
        #[codec(index = 1)]
        Limit,
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct WatchList<_0, _1> {
        pub compliance_level: runtime_types::tidefi_primitives::ComplianceLevel,
        pub currency_id: runtime_types::tidefi_primitives::CurrencyId,
        pub amount: ::core::primitive::u128,
        pub transaction_id: _1,
        pub watch_action: runtime_types::tidefi_primitives::WatchListAction,
        pub block_number: _0,
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub enum WatchListAction {
        #[codec(index = 0)]
        Mint,
      }
      #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
      pub struct Withdrawal<_0, _1, _2> {
        pub account_id: _0,
        pub asset_id: runtime_types::tidefi_primitives::CurrencyId,
        pub amount: ::core::primitive::u128,
        pub external_address: _2,
        pub block_number: _1,
      }
    }
  }
  #[doc = r" The default error type returned when there is a runtime issue."]
  pub type DispatchError = runtime_types::sp_runtime::DispatchError;
  impl ::subxt::HasModuleError for runtime_types::sp_runtime::DispatchError {
    fn module_error_data(&self) -> Option<::subxt::ModuleErrorData> {
      if let Self::Module(module_error) = self {
        Some(::subxt::ModuleErrorData {
          pallet_index: module_error.index,
          error: [module_error.error, 0, 0, 0],
        })
      } else {
        None
      }
    }
  }
  pub struct RuntimeApi<T: ::subxt::Config, X> {
    pub client: ::subxt::Client<T>,
    marker: ::core::marker::PhantomData<X>,
  }
  impl<T, X> ::core::convert::From<::subxt::Client<T>> for RuntimeApi<T, X>
  where
    T: ::subxt::Config,
    X: ::subxt::extrinsic::ExtrinsicParams<T>,
  {
    fn from(client: ::subxt::Client<T>) -> Self {
      Self {
        client,
        marker: ::core::marker::PhantomData,
      }
    }
  }
  impl<'a, T, X> RuntimeApi<T, X>
  where
    T: ::subxt::Config,
    X: ::subxt::extrinsic::ExtrinsicParams<T>,
  {
    pub fn constants(&'a self) -> ConstantsApi<'a, T> {
      ConstantsApi {
        client: &self.client,
      }
    }
    pub fn storage(&'a self) -> StorageApi<'a, T> {
      StorageApi {
        client: &self.client,
      }
    }
    pub fn tx(&'a self) -> TransactionApi<'a, T, X> {
      TransactionApi {
        client: &self.client,
        marker: ::core::marker::PhantomData,
      }
    }
    pub fn events(&'a self) -> EventsApi<'a, T> {
      EventsApi {
        client: &self.client,
      }
    }
  }
  pub struct EventsApi<'a, T: ::subxt::Config> {
    client: &'a ::subxt::Client<T>,
  }
  impl<'a, T: ::subxt::Config> EventsApi<'a, T> {
    pub async fn at(
      &self,
      block_hash: T::Hash,
    ) -> Result<::subxt::events::Events<'a, T, Event>, ::subxt::BasicError> {
      ::subxt::events::at::<T, Event>(self.client, block_hash).await
    }
    pub async fn subscribe(
      &self,
    ) -> Result<
      ::subxt::events::EventSubscription<'a, ::subxt::events::EventSub<T::Header>, T, Event>,
      ::subxt::BasicError,
    > {
      ::subxt::events::subscribe::<T, Event>(self.client).await
    }
    pub async fn subscribe_finalized(
      &self,
    ) -> Result<
      ::subxt::events::EventSubscription<
        'a,
        ::subxt::events::FinalizedEventSub<'a, T::Header>,
        T,
        Event,
      >,
      ::subxt::BasicError,
    > {
      ::subxt::events::subscribe_finalized::<T, Event>(self.client).await
    }
  }
  pub struct ConstantsApi<'a, T: ::subxt::Config> {
    client: &'a ::subxt::Client<T>,
  }
  impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
    pub fn system(&self) -> system::constants::ConstantsApi<'a, T> {
      system::constants::ConstantsApi::new(self.client)
    }
    pub fn babe(&self) -> babe::constants::ConstantsApi<'a, T> {
      babe::constants::ConstantsApi::new(self.client)
    }
    pub fn timestamp(&self) -> timestamp::constants::ConstantsApi<'a, T> {
      timestamp::constants::ConstantsApi::new(self.client)
    }
    pub fn indices(&self) -> indices::constants::ConstantsApi<'a, T> {
      indices::constants::ConstantsApi::new(self.client)
    }
    pub fn balances(&self) -> balances::constants::ConstantsApi<'a, T> {
      balances::constants::ConstantsApi::new(self.client)
    }
    pub fn transaction_payment(&self) -> transaction_payment::constants::ConstantsApi<'a, T> {
      transaction_payment::constants::ConstantsApi::new(self.client)
    }
    pub fn authorship(&self) -> authorship::constants::ConstantsApi<'a, T> {
      authorship::constants::ConstantsApi::new(self.client)
    }
    pub fn staking(&self) -> staking::constants::ConstantsApi<'a, T> {
      staking::constants::ConstantsApi::new(self.client)
    }
    pub fn grandpa(&self) -> grandpa::constants::ConstantsApi<'a, T> {
      grandpa::constants::ConstantsApi::new(self.client)
    }
    pub fn im_online(&self) -> im_online::constants::ConstantsApi<'a, T> {
      im_online::constants::ConstantsApi::new(self.client)
    }
    pub fn elections(&self) -> elections::constants::ConstantsApi<'a, T> {
      elections::constants::ConstantsApi::new(self.client)
    }
    pub fn treasury(&self) -> treasury::constants::ConstantsApi<'a, T> {
      treasury::constants::ConstantsApi::new(self.client)
    }
    pub fn utility(&self) -> utility::constants::ConstantsApi<'a, T> {
      utility::constants::ConstantsApi::new(self.client)
    }
    pub fn identity(&self) -> identity::constants::ConstantsApi<'a, T> {
      identity::constants::ConstantsApi::new(self.client)
    }
    pub fn election_provider_multi_phase(
      &self,
    ) -> election_provider_multi_phase::constants::ConstantsApi<'a, T> {
      election_provider_multi_phase::constants::ConstantsApi::new(self.client)
    }
    pub fn recovery(&self) -> recovery::constants::ConstantsApi<'a, T> {
      recovery::constants::ConstantsApi::new(self.client)
    }
    pub fn scheduler(&self) -> scheduler::constants::ConstantsApi<'a, T> {
      scheduler::constants::ConstantsApi::new(self.client)
    }
    pub fn proxy(&self) -> proxy::constants::ConstantsApi<'a, T> {
      proxy::constants::ConstantsApi::new(self.client)
    }
    pub fn multisig(&self) -> multisig::constants::ConstantsApi<'a, T> {
      multisig::constants::ConstantsApi::new(self.client)
    }
    pub fn bounties(&self) -> bounties::constants::ConstantsApi<'a, T> {
      bounties::constants::ConstantsApi::new(self.client)
    }
    pub fn assets(&self) -> assets::constants::ConstantsApi<'a, T> {
      assets::constants::ConstantsApi::new(self.client)
    }
    pub fn bags_list(&self) -> bags_list::constants::ConstantsApi<'a, T> {
      bags_list::constants::ConstantsApi::new(self.client)
    }
    pub fn tidefi_staking(&self) -> tidefi_staking::constants::ConstantsApi<'a, T> {
      tidefi_staking::constants::ConstantsApi::new(self.client)
    }
    pub fn quorum(&self) -> quorum::constants::ConstantsApi<'a, T> {
      quorum::constants::ConstantsApi::new(self.client)
    }
    pub fn oracle(&self) -> oracle::constants::ConstantsApi<'a, T> {
      oracle::constants::ConstantsApi::new(self.client)
    }
    pub fn fees(&self) -> fees::constants::ConstantsApi<'a, T> {
      fees::constants::ConstantsApi::new(self.client)
    }
    pub fn asset_registry(&self) -> asset_registry::constants::ConstantsApi<'a, T> {
      asset_registry::constants::ConstantsApi::new(self.client)
    }
  }
  pub struct StorageApi<'a, T: ::subxt::Config> {
    client: &'a ::subxt::Client<T>,
  }
  impl<'a, T> StorageApi<'a, T>
  where
    T: ::subxt::Config,
  {
    pub fn system(&self) -> system::storage::StorageApi<'a, T> {
      system::storage::StorageApi::new(self.client)
    }
    pub fn babe(&self) -> babe::storage::StorageApi<'a, T> {
      babe::storage::StorageApi::new(self.client)
    }
    pub fn timestamp(&self) -> timestamp::storage::StorageApi<'a, T> {
      timestamp::storage::StorageApi::new(self.client)
    }
    pub fn indices(&self) -> indices::storage::StorageApi<'a, T> {
      indices::storage::StorageApi::new(self.client)
    }
    pub fn balances(&self) -> balances::storage::StorageApi<'a, T> {
      balances::storage::StorageApi::new(self.client)
    }
    pub fn transaction_payment(&self) -> transaction_payment::storage::StorageApi<'a, T> {
      transaction_payment::storage::StorageApi::new(self.client)
    }
    pub fn authorship(&self) -> authorship::storage::StorageApi<'a, T> {
      authorship::storage::StorageApi::new(self.client)
    }
    pub fn staking(&self) -> staking::storage::StorageApi<'a, T> {
      staking::storage::StorageApi::new(self.client)
    }
    pub fn offences(&self) -> offences::storage::StorageApi<'a, T> {
      offences::storage::StorageApi::new(self.client)
    }
    pub fn session(&self) -> session::storage::StorageApi<'a, T> {
      session::storage::StorageApi::new(self.client)
    }
    pub fn grandpa(&self) -> grandpa::storage::StorageApi<'a, T> {
      grandpa::storage::StorageApi::new(self.client)
    }
    pub fn im_online(&self) -> im_online::storage::StorageApi<'a, T> {
      im_online::storage::StorageApi::new(self.client)
    }
    pub fn council(&self) -> council::storage::StorageApi<'a, T> {
      council::storage::StorageApi::new(self.client)
    }
    pub fn technical_committee(&self) -> technical_committee::storage::StorageApi<'a, T> {
      technical_committee::storage::StorageApi::new(self.client)
    }
    pub fn elections(&self) -> elections::storage::StorageApi<'a, T> {
      elections::storage::StorageApi::new(self.client)
    }
    pub fn technical_membership(&self) -> technical_membership::storage::StorageApi<'a, T> {
      technical_membership::storage::StorageApi::new(self.client)
    }
    pub fn treasury(&self) -> treasury::storage::StorageApi<'a, T> {
      treasury::storage::StorageApi::new(self.client)
    }
    pub fn identity(&self) -> identity::storage::StorageApi<'a, T> {
      identity::storage::StorageApi::new(self.client)
    }
    pub fn election_provider_multi_phase(
      &self,
    ) -> election_provider_multi_phase::storage::StorageApi<'a, T> {
      election_provider_multi_phase::storage::StorageApi::new(self.client)
    }
    pub fn recovery(&self) -> recovery::storage::StorageApi<'a, T> {
      recovery::storage::StorageApi::new(self.client)
    }
    pub fn scheduler(&self) -> scheduler::storage::StorageApi<'a, T> {
      scheduler::storage::StorageApi::new(self.client)
    }
    pub fn proxy(&self) -> proxy::storage::StorageApi<'a, T> {
      proxy::storage::StorageApi::new(self.client)
    }
    pub fn multisig(&self) -> multisig::storage::StorageApi<'a, T> {
      multisig::storage::StorageApi::new(self.client)
    }
    pub fn bounties(&self) -> bounties::storage::StorageApi<'a, T> {
      bounties::storage::StorageApi::new(self.client)
    }
    pub fn assets(&self) -> assets::storage::StorageApi<'a, T> {
      assets::storage::StorageApi::new(self.client)
    }
    pub fn bags_list(&self) -> bags_list::storage::StorageApi<'a, T> {
      bags_list::storage::StorageApi::new(self.client)
    }
    pub fn preimage(&self) -> preimage::storage::StorageApi<'a, T> {
      preimage::storage::StorageApi::new(self.client)
    }
    pub fn sudo(&self) -> sudo::storage::StorageApi<'a, T> {
      sudo::storage::StorageApi::new(self.client)
    }
    pub fn tidefi(&self) -> tidefi::storage::StorageApi<'a, T> {
      tidefi::storage::StorageApi::new(self.client)
    }
    pub fn tidefi_staking(&self) -> tidefi_staking::storage::StorageApi<'a, T> {
      tidefi_staking::storage::StorageApi::new(self.client)
    }
    pub fn quorum(&self) -> quorum::storage::StorageApi<'a, T> {
      quorum::storage::StorageApi::new(self.client)
    }
    pub fn oracle(&self) -> oracle::storage::StorageApi<'a, T> {
      oracle::storage::StorageApi::new(self.client)
    }
    pub fn security(&self) -> security::storage::StorageApi<'a, T> {
      security::storage::StorageApi::new(self.client)
    }
    pub fn fees(&self) -> fees::storage::StorageApi<'a, T> {
      fees::storage::StorageApi::new(self.client)
    }
    pub fn asset_registry(&self) -> asset_registry::storage::StorageApi<'a, T> {
      asset_registry::storage::StorageApi::new(self.client)
    }
  }
  pub struct TransactionApi<'a, T: ::subxt::Config, X> {
    client: &'a ::subxt::Client<T>,
    marker: ::core::marker::PhantomData<X>,
  }
  impl<'a, T, X> TransactionApi<'a, T, X>
  where
    T: ::subxt::Config,
    X: ::subxt::extrinsic::ExtrinsicParams<T>,
  {
    pub fn system(&self) -> system::calls::TransactionApi<'a, T, X> {
      system::calls::TransactionApi::new(self.client)
    }
    pub fn babe(&self) -> babe::calls::TransactionApi<'a, T, X> {
      babe::calls::TransactionApi::new(self.client)
    }
    pub fn timestamp(&self) -> timestamp::calls::TransactionApi<'a, T, X> {
      timestamp::calls::TransactionApi::new(self.client)
    }
    pub fn indices(&self) -> indices::calls::TransactionApi<'a, T, X> {
      indices::calls::TransactionApi::new(self.client)
    }
    pub fn balances(&self) -> balances::calls::TransactionApi<'a, T, X> {
      balances::calls::TransactionApi::new(self.client)
    }
    pub fn authorship(&self) -> authorship::calls::TransactionApi<'a, T, X> {
      authorship::calls::TransactionApi::new(self.client)
    }
    pub fn staking(&self) -> staking::calls::TransactionApi<'a, T, X> {
      staking::calls::TransactionApi::new(self.client)
    }
    pub fn session(&self) -> session::calls::TransactionApi<'a, T, X> {
      session::calls::TransactionApi::new(self.client)
    }
    pub fn grandpa(&self) -> grandpa::calls::TransactionApi<'a, T, X> {
      grandpa::calls::TransactionApi::new(self.client)
    }
    pub fn im_online(&self) -> im_online::calls::TransactionApi<'a, T, X> {
      im_online::calls::TransactionApi::new(self.client)
    }
    pub fn council(&self) -> council::calls::TransactionApi<'a, T, X> {
      council::calls::TransactionApi::new(self.client)
    }
    pub fn technical_committee(&self) -> technical_committee::calls::TransactionApi<'a, T, X> {
      technical_committee::calls::TransactionApi::new(self.client)
    }
    pub fn elections(&self) -> elections::calls::TransactionApi<'a, T, X> {
      elections::calls::TransactionApi::new(self.client)
    }
    pub fn technical_membership(&self) -> technical_membership::calls::TransactionApi<'a, T, X> {
      technical_membership::calls::TransactionApi::new(self.client)
    }
    pub fn treasury(&self) -> treasury::calls::TransactionApi<'a, T, X> {
      treasury::calls::TransactionApi::new(self.client)
    }
    pub fn utility(&self) -> utility::calls::TransactionApi<'a, T, X> {
      utility::calls::TransactionApi::new(self.client)
    }
    pub fn identity(&self) -> identity::calls::TransactionApi<'a, T, X> {
      identity::calls::TransactionApi::new(self.client)
    }
    pub fn election_provider_multi_phase(
      &self,
    ) -> election_provider_multi_phase::calls::TransactionApi<'a, T, X> {
      election_provider_multi_phase::calls::TransactionApi::new(self.client)
    }
    pub fn recovery(&self) -> recovery::calls::TransactionApi<'a, T, X> {
      recovery::calls::TransactionApi::new(self.client)
    }
    pub fn scheduler(&self) -> scheduler::calls::TransactionApi<'a, T, X> {
      scheduler::calls::TransactionApi::new(self.client)
    }
    pub fn proxy(&self) -> proxy::calls::TransactionApi<'a, T, X> {
      proxy::calls::TransactionApi::new(self.client)
    }
    pub fn multisig(&self) -> multisig::calls::TransactionApi<'a, T, X> {
      multisig::calls::TransactionApi::new(self.client)
    }
    pub fn bounties(&self) -> bounties::calls::TransactionApi<'a, T, X> {
      bounties::calls::TransactionApi::new(self.client)
    }
    pub fn assets(&self) -> assets::calls::TransactionApi<'a, T, X> {
      assets::calls::TransactionApi::new(self.client)
    }
    pub fn bags_list(&self) -> bags_list::calls::TransactionApi<'a, T, X> {
      bags_list::calls::TransactionApi::new(self.client)
    }
    pub fn preimage(&self) -> preimage::calls::TransactionApi<'a, T, X> {
      preimage::calls::TransactionApi::new(self.client)
    }
    pub fn sudo(&self) -> sudo::calls::TransactionApi<'a, T, X> {
      sudo::calls::TransactionApi::new(self.client)
    }
    pub fn tidefi(&self) -> tidefi::calls::TransactionApi<'a, T, X> {
      tidefi::calls::TransactionApi::new(self.client)
    }
    pub fn tidefi_staking(&self) -> tidefi_staking::calls::TransactionApi<'a, T, X> {
      tidefi_staking::calls::TransactionApi::new(self.client)
    }
    pub fn quorum(&self) -> quorum::calls::TransactionApi<'a, T, X> {
      quorum::calls::TransactionApi::new(self.client)
    }
    pub fn oracle(&self) -> oracle::calls::TransactionApi<'a, T, X> {
      oracle::calls::TransactionApi::new(self.client)
    }
    pub fn security(&self) -> security::calls::TransactionApi<'a, T, X> {
      security::calls::TransactionApi::new(self.client)
    }
    pub fn fees(&self) -> fees::calls::TransactionApi<'a, T, X> {
      fees::calls::TransactionApi::new(self.client)
    }
    pub fn asset_registry(&self) -> asset_registry::calls::TransactionApi<'a, T, X> {
      asset_registry::calls::TransactionApi::new(self.client)
    }
  }
}
