#[allow(clippy::all)]
#[allow(dead_code, unused_imports, non_camel_case_types)]
pub mod api {
    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
    pub enum Event {
        #[codec(index = 0)]
        System(system::Event),
        #[codec(index = 4)]
        Sudo(sudo::Event),
        #[codec(index = 20)]
        ParachainSystem(parachain_system::Event),
        #[codec(index = 30)]
        Balances(balances::Event),
        #[codec(index = 31)]
        Vesting(vesting::Event),
        #[codec(index = 40)]
        ParachainStaking(parachain_staking::Event),
        #[codec(index = 42)]
        AuthorFilter(author_filter::Event),
        #[codec(index = 43)]
        AuthorMapping(author_mapping::Event),
        #[codec(index = 50)]
        XcmpQueue(xcmp_queue::Event),
        #[codec(index = 51)]
        PolkadotXcm(polkadot_xcm::Event),
        #[codec(index = 52)]
        CumulusXcm(cumulus_xcm::Event),
        #[codec(index = 53)]
        DmpQueue(dmp_queue::Event),
        #[codec(index = 54)]
        Mvm(mvm::Event),
        #[codec(index = 55)]
        MultiSig(multi_sig::Event),
    }
    pub mod system {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct FillBlock {
                pub ratio: runtime_types::sp_arithmetic::per_things::Perbill,
            }
            impl ::subxt::Call for FillBlock {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "fill_block";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Remark {
                pub remark: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for Remark {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "remark";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetHeapPages {
                pub pages: ::core::primitive::u64,
            }
            impl ::subxt::Call for SetHeapPages {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "set_heap_pages";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetCode {
                pub code: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for SetCode {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "set_code";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetCodeWithoutChecks {
                pub code: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for SetCodeWithoutChecks {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "set_code_without_checks";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetChangesTrieConfig {
                pub changes_trie_config: ::core::option::Option<
                    runtime_types::sp_core::changes_trie::ChangesTrieConfiguration,
                >,
            }
            impl ::subxt::Call for SetChangesTrieConfig {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "set_changes_trie_config";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
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
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct KillStorage {
                pub keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
            }
            impl ::subxt::Call for KillStorage {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "kill_storage";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct KillPrefix {
                pub prefix: ::std::vec::Vec<::core::primitive::u8>,
                pub subkeys: ::core::primitive::u32,
            }
            impl ::subxt::Call for KillPrefix {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "kill_prefix";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct RemarkWithEvent {
                pub remark: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for RemarkWithEvent {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "remark_with_event";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn fill_block(
                    &self,
                    ratio: runtime_types::sp_arithmetic::per_things::Perbill,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, FillBlock> {
                    let call = FillBlock { ratio };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn remark(
                    &self,
                    remark: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, Remark> {
                    let call = Remark { remark };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_heap_pages(
                    &self,
                    pages: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, SetHeapPages> {
                    let call = SetHeapPages { pages };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_code(
                    &self,
                    code: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, SetCode> {
                    let call = SetCode { code };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_code_without_checks(
                    &self,
                    code: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, SetCodeWithoutChecks> {
                    let call = SetCodeWithoutChecks { code };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_changes_trie_config(
                    &self,
                    changes_trie_config: ::core::option::Option<
                        runtime_types::sp_core::changes_trie::ChangesTrieConfiguration,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, SetChangesTrieConfig> {
                    let call = SetChangesTrieConfig {
                        changes_trie_config,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_storage(
                    &self,
                    items: ::std::vec::Vec<(
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::std::vec::Vec<::core::primitive::u8>,
                    )>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, SetStorage> {
                    let call = SetStorage { items };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn kill_storage(
                    &self,
                    keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, KillStorage> {
                    let call = KillStorage { keys };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn kill_prefix(
                    &self,
                    prefix: ::std::vec::Vec<::core::primitive::u8>,
                    subkeys: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, KillPrefix> {
                    let call = KillPrefix { prefix, subkeys };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn remark_with_event(
                    &self,
                    remark: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, RemarkWithEvent> {
                    let call = RemarkWithEvent { remark };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::frame_system::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ExtrinsicSuccess(pub runtime_types::frame_support::weights::DispatchInfo);
            impl ::subxt::Event for ExtrinsicSuccess {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "ExtrinsicSuccess";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ExtrinsicFailed(
                pub runtime_types::sp_runtime::DispatchError,
                pub runtime_types::frame_support::weights::DispatchInfo,
            );
            impl ::subxt::Event for ExtrinsicFailed {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "ExtrinsicFailed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct CodeUpdated {}
            impl ::subxt::Event for CodeUpdated {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "CodeUpdated";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct NewAccount(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::Event for NewAccount {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "NewAccount";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct KilledAccount(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::Event for KilledAccount {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "KilledAccount";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Remarked(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::H256,
            );
            impl ::subxt::Event for Remarked {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "Remarked";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Account(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Account {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "Account";
                type Value = runtime_types::frame_system::AccountInfo<
                    ::core::primitive::u32,
                    runtime_types::pallet_balances::AccountData<::core::primitive::u64>,
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
                type Value = runtime_types::frame_support::weights::PerDispatchClass<
                    ::core::primitive::u64,
                >;
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
            pub struct BlockHash(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for BlockHash {
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
            pub struct ExtrinsicData(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for ExtrinsicData {
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
                type Value =
                    runtime_types::sp_runtime::generic::digest::Digest<::subxt::sp_core::H256>;
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
                        runtime_types::pontem_runtime::Event,
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
            pub struct EventTopics(pub ::subxt::sp_core::H256);
            impl ::subxt::StorageEntry for EventTopics {
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
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_system::AccountInfo<
                        ::core::primitive::u32,
                        runtime_types::pallet_balances::AccountData<::core::primitive::u64>,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Account(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn account_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Account>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn extrinsic_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::Error,
                > {
                    let entry = ExtrinsicCount;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn block_weight(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::weights::PerDispatchClass<
                        ::core::primitive::u64,
                    >,
                    ::subxt::Error,
                > {
                    let entry = BlockWeight;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn all_extrinsics_len(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::Error,
                > {
                    let entry = AllExtrinsicsLen;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn block_hash(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::sp_core::H256, ::subxt::Error>
                {
                    let entry = BlockHash(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn block_hash_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, BlockHash>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn extrinsic_data(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::std::vec::Vec<::core::primitive::u8>, ::subxt::Error>
                {
                    let entry = ExtrinsicData(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn extrinsic_data_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, ExtrinsicData>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn number(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = Number;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn parent_hash(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::sp_core::H256, ::subxt::Error>
                {
                    let entry = ParentHash;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn digest(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::sp_runtime::generic::digest::Digest<::subxt::sp_core::H256>,
                    ::subxt::Error,
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
                            runtime_types::pontem_runtime::Event,
                            ::subxt::sp_core::H256,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Events;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn event_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = EventCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn event_topics(
                    &self,
                    _0: ::subxt::sp_core::H256,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>,
                    ::subxt::Error,
                > {
                    let entry = EventTopics(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn event_topics_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, EventTopics>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn last_runtime_upgrade(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<runtime_types::frame_system::LastRuntimeUpgradeInfo>,
                    ::subxt::Error,
                > {
                    let entry = LastRuntimeUpgrade;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn upgraded_to_u32_ref_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::bool, ::subxt::Error>
                {
                    let entry = UpgradedToU32RefCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn upgraded_to_triple_ref_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::bool, ::subxt::Error>
                {
                    let entry = UpgradedToTripleRefCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn execution_phase(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<runtime_types::frame_system::Phase>,
                    ::subxt::Error,
                > {
                    let entry = ExecutionPhase;
                    self.client.storage().fetch(&entry, hash).await
                }
            }
        }
    }
    pub mod randomness_collective_flip {
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub struct RandomMaterial;
            impl ::subxt::StorageEntry for RandomMaterial {
                const PALLET: &'static str = "RandomnessCollectiveFlip";
                const STORAGE: &'static str = "RandomMaterial";
                type Value = ::std::vec::Vec<::subxt::sp_core::H256>;
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
                pub async fn random_material(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::std::vec::Vec<::subxt::sp_core::H256>, ::subxt::Error>
                {
                    let entry = RandomMaterial;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod timestamp {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Set {
                #[codec(compact)]
                pub now: ::core::primitive::u64,
            }
            impl ::subxt::Call for Set {
                const PALLET: &'static str = "Timestamp";
                const FUNCTION: &'static str = "set";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn set(
                    &self,
                    now: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, Set> {
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
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::Error>
                {
                    let entry = Now;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn did_update(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::bool, ::subxt::Error>
                {
                    let entry = DidUpdate;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod transaction_payment {
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
                    ::subxt::Error,
                > {
                    let entry = NextFeeMultiplier;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn storage_version(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_transaction_payment::Releases,
                    ::subxt::Error,
                > {
                    let entry = StorageVersion;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod sudo {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Sudo {
                pub call: runtime_types::pontem_runtime::Call,
            }
            impl ::subxt::Call for Sudo {
                const PALLET: &'static str = "Sudo";
                const FUNCTION: &'static str = "sudo";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SudoUncheckedWeight {
                pub call: runtime_types::pontem_runtime::Call,
                pub weight: ::core::primitive::u64,
            }
            impl ::subxt::Call for SudoUncheckedWeight {
                const PALLET: &'static str = "Sudo";
                const FUNCTION: &'static str = "sudo_unchecked_weight";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetKey {
                pub new:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
            }
            impl ::subxt::Call for SetKey {
                const PALLET: &'static str = "Sudo";
                const FUNCTION: &'static str = "set_key";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SudoAs {
                pub who:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub call: runtime_types::pontem_runtime::Call,
            }
            impl ::subxt::Call for SudoAs {
                const PALLET: &'static str = "Sudo";
                const FUNCTION: &'static str = "sudo_as";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn sudo(
                    &self,
                    call: runtime_types::pontem_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, Sudo> {
                    let call = Sudo { call };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn sudo_unchecked_weight(
                    &self,
                    call: runtime_types::pontem_runtime::Call,
                    weight: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, SudoUncheckedWeight> {
                    let call = SudoUncheckedWeight { call, weight };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_key(
                    &self,
                    new: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, SetKey> {
                    let call = SetKey { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn sudo_as(
                    &self,
                    who: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    call: runtime_types::pontem_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, SudoAs> {
                    let call = SudoAs { who, call };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_sudo::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Sudid(
                pub ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            );
            impl ::subxt::Event for Sudid {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "Sudid";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct KeyChanged(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::Event for KeyChanged {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "KeyChanged";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SudoAsDone(
                pub ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            );
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
                ) -> ::core::result::Result<::subxt::sp_core::crypto::AccountId32, ::subxt::Error>
                {
                    let entry = Key;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod parachain_system {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetUpgradeBlock {
                pub relay_chain_block: ::core::primitive::u32,
            }
            impl ::subxt::Call for SetUpgradeBlock {
                const PALLET: &'static str = "ParachainSystem";
                const FUNCTION: &'static str = "set_upgrade_block";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetValidationData {
                pub data:
                    runtime_types::cumulus_primitives_parachain_inherent::ParachainInherentData,
            }
            impl ::subxt::Call for SetValidationData {
                const PALLET: &'static str = "ParachainSystem";
                const FUNCTION: &'static str = "set_validation_data";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SudoSendUpwardMessage {
                pub message: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for SudoSendUpwardMessage {
                const PALLET: &'static str = "ParachainSystem";
                const FUNCTION: &'static str = "sudo_send_upward_message";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct AuthorizeUpgrade {
                pub code_hash: ::subxt::sp_core::H256,
            }
            impl ::subxt::Call for AuthorizeUpgrade {
                const PALLET: &'static str = "ParachainSystem";
                const FUNCTION: &'static str = "authorize_upgrade";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct EnactAuthorizedUpgrade {
                pub code: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for EnactAuthorizedUpgrade {
                const PALLET: &'static str = "ParachainSystem";
                const FUNCTION: &'static str = "enact_authorized_upgrade";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn set_upgrade_block(
                    &self,
                    relay_chain_block: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, SetUpgradeBlock> {
                    let call = SetUpgradeBlock { relay_chain_block };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_validation_data(
                    &self,
                    data : runtime_types :: cumulus_primitives_parachain_inherent :: ParachainInherentData,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, SetValidationData> {
                    let call = SetValidationData { data };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn sudo_send_upward_message(
                    &self,
                    message: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, SudoSendUpwardMessage> {
                    let call = SudoSendUpwardMessage { message };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn authorize_upgrade(
                    &self,
                    code_hash: ::subxt::sp_core::H256,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, AuthorizeUpgrade> {
                    let call = AuthorizeUpgrade { code_hash };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn enact_authorized_upgrade(
                    &self,
                    code: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, EnactAuthorizedUpgrade>
                {
                    let call = EnactAuthorizedUpgrade { code };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::cumulus_pallet_parachain_system::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ValidationFunctionStored(pub ::core::primitive::u32);
            impl ::subxt::Event for ValidationFunctionStored {
                const PALLET: &'static str = "ParachainSystem";
                const EVENT: &'static str = "ValidationFunctionStored";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ValidationFunctionApplied(pub ::core::primitive::u32);
            impl ::subxt::Event for ValidationFunctionApplied {
                const PALLET: &'static str = "ParachainSystem";
                const EVENT: &'static str = "ValidationFunctionApplied";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct UpgradeAuthorized(pub ::subxt::sp_core::H256);
            impl ::subxt::Event for UpgradeAuthorized {
                const PALLET: &'static str = "ParachainSystem";
                const EVENT: &'static str = "UpgradeAuthorized";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct DownwardMessagesReceived(pub ::core::primitive::u32);
            impl ::subxt::Event for DownwardMessagesReceived {
                const PALLET: &'static str = "ParachainSystem";
                const EVENT: &'static str = "DownwardMessagesReceived";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct DownwardMessagesProcessed(
                pub ::core::primitive::u64,
                pub ::subxt::sp_core::H256,
            );
            impl ::subxt::Event for DownwardMessagesProcessed {
                const PALLET: &'static str = "ParachainSystem";
                const EVENT: &'static str = "DownwardMessagesProcessed";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct PendingRelayChainBlockNumber;
            impl ::subxt::StorageEntry for PendingRelayChainBlockNumber {
                const PALLET: &'static str = "ParachainSystem";
                const STORAGE: &'static str = "PendingRelayChainBlockNumber";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct PendingValidationCode;
            impl ::subxt::StorageEntry for PendingValidationCode {
                const PALLET: &'static str = "ParachainSystem";
                const STORAGE: &'static str = "PendingValidationCode";
                type Value = ::std::vec::Vec<::core::primitive::u8>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ValidationData;
            impl ::subxt::StorageEntry for ValidationData {
                const PALLET: &'static str = "ParachainSystem";
                const STORAGE: &'static str = "ValidationData";
                type Value = runtime_types::polkadot_primitives::v1::PersistedValidationData<
                    ::subxt::sp_core::H256,
                    ::core::primitive::u32,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct DidSetValidationCode;
            impl ::subxt::StorageEntry for DidSetValidationCode {
                const PALLET: &'static str = "ParachainSystem";
                const STORAGE: &'static str = "DidSetValidationCode";
                type Value = ::core::primitive::bool;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct LastUpgrade;
            impl ::subxt::StorageEntry for LastUpgrade {
                const PALLET: &'static str = "ParachainSystem";
                const STORAGE: &'static str = "LastUpgrade";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct RelevantMessagingState;
            impl ::subxt::StorageEntry for RelevantMessagingState {
                const PALLET: &'static str = "ParachainSystem";
                const STORAGE: &'static str = "RelevantMessagingState";
                type Value = runtime_types :: cumulus_pallet_parachain_system :: relay_state_snapshot :: MessagingStateSnapshot ;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct HostConfiguration;
            impl ::subxt::StorageEntry for HostConfiguration {
                const PALLET: &'static str = "ParachainSystem";
                const STORAGE: &'static str = "HostConfiguration";
                type Value = runtime_types::polkadot_primitives::v1::AbridgedHostConfiguration;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct LastDmqMqcHead;
            impl ::subxt::StorageEntry for LastDmqMqcHead {
                const PALLET: &'static str = "ParachainSystem";
                const STORAGE: &'static str = "LastDmqMqcHead";
                type Value = runtime_types::cumulus_pallet_parachain_system::MessageQueueChain;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct LastHrmpMqcHeads;
            impl ::subxt::StorageEntry for LastHrmpMqcHeads {
                const PALLET: &'static str = "ParachainSystem";
                const STORAGE: &'static str = "LastHrmpMqcHeads";
                type Value = ::std::collections::BTreeMap<
                    runtime_types::polkadot_parachain::primitives::Id,
                    runtime_types::cumulus_pallet_parachain_system::MessageQueueChain,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ProcessedDownwardMessages;
            impl ::subxt::StorageEntry for ProcessedDownwardMessages {
                const PALLET: &'static str = "ParachainSystem";
                const STORAGE: &'static str = "ProcessedDownwardMessages";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NewValidationCode;
            impl ::subxt::StorageEntry for NewValidationCode {
                const PALLET: &'static str = "ParachainSystem";
                const STORAGE: &'static str = "NewValidationCode";
                type Value = ::std::vec::Vec<::core::primitive::u8>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct HrmpWatermark;
            impl ::subxt::StorageEntry for HrmpWatermark {
                const PALLET: &'static str = "ParachainSystem";
                const STORAGE: &'static str = "HrmpWatermark";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct HrmpOutboundMessages;
            impl ::subxt::StorageEntry for HrmpOutboundMessages {
                const PALLET: &'static str = "ParachainSystem";
                const STORAGE: &'static str = "HrmpOutboundMessages";
                type Value = ::std::vec::Vec<
                    runtime_types::polkadot_core_primitives::OutboundHrmpMessage<
                        runtime_types::polkadot_parachain::primitives::Id,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct UpwardMessages;
            impl ::subxt::StorageEntry for UpwardMessages {
                const PALLET: &'static str = "ParachainSystem";
                const STORAGE: &'static str = "UpwardMessages";
                type Value = ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct PendingUpwardMessages;
            impl ::subxt::StorageEntry for PendingUpwardMessages {
                const PALLET: &'static str = "ParachainSystem";
                const STORAGE: &'static str = "PendingUpwardMessages";
                type Value = ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct AnnouncedHrmpMessagesPerCandidate;
            impl ::subxt::StorageEntry for AnnouncedHrmpMessagesPerCandidate {
                const PALLET: &'static str = "ParachainSystem";
                const STORAGE: &'static str = "AnnouncedHrmpMessagesPerCandidate";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ReservedXcmpWeightOverride;
            impl ::subxt::StorageEntry for ReservedXcmpWeightOverride {
                const PALLET: &'static str = "ParachainSystem";
                const STORAGE: &'static str = "ReservedXcmpWeightOverride";
                type Value = ::core::primitive::u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ReservedDmpWeightOverride;
            impl ::subxt::StorageEntry for ReservedDmpWeightOverride {
                const PALLET: &'static str = "ParachainSystem";
                const STORAGE: &'static str = "ReservedDmpWeightOverride";
                type Value = ::core::primitive::u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct AuthorizedUpgrade;
            impl ::subxt::StorageEntry for AuthorizedUpgrade {
                const PALLET: &'static str = "ParachainSystem";
                const STORAGE: &'static str = "AuthorizedUpgrade";
                type Value = ::subxt::sp_core::H256;
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
                pub async fn pending_relay_chain_block_number(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::Error,
                > {
                    let entry = PendingRelayChainBlockNumber;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn pending_validation_code(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::std::vec::Vec<::core::primitive::u8>, ::subxt::Error>
                {
                    let entry = PendingValidationCode;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn validation_data(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::polkadot_primitives::v1::PersistedValidationData<
                            ::subxt::sp_core::H256,
                            ::core::primitive::u32,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = ValidationData;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn did_set_validation_code(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::bool, ::subxt::Error>
                {
                    let entry = DidSetValidationCode;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn last_upgrade(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = LastUpgrade;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }                pub async fn relevant_messaging_state (& self , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < :: core :: option :: Option < runtime_types :: cumulus_pallet_parachain_system :: relay_state_snapshot :: MessagingStateSnapshot > , :: subxt :: Error >{
                    let entry = RelevantMessagingState;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn host_configuration(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::polkadot_primitives::v1::AbridgedHostConfiguration,
                    >,
                    ::subxt::Error,
                > {
                    let entry = HostConfiguration;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn last_dmq_mqc_head(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::cumulus_pallet_parachain_system::MessageQueueChain,
                    ::subxt::Error,
                > {
                    let entry = LastDmqMqcHead;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn last_hrmp_mqc_heads(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::collections::BTreeMap<
                        runtime_types::polkadot_parachain::primitives::Id,
                        runtime_types::cumulus_pallet_parachain_system::MessageQueueChain,
                    >,
                    ::subxt::Error,
                > {
                    let entry = LastHrmpMqcHeads;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn processed_downward_messages(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = ProcessedDownwardMessages;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn new_validation_code(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
                    ::subxt::Error,
                > {
                    let entry = NewValidationCode;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn hrmp_watermark(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = HrmpWatermark;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn hrmp_outbound_messages(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<
                        runtime_types::polkadot_core_primitives::OutboundHrmpMessage<
                            runtime_types::polkadot_parachain::primitives::Id,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = HrmpOutboundMessages;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn upward_messages(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                    ::subxt::Error,
                > {
                    let entry = UpwardMessages;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn pending_upward_messages(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                    ::subxt::Error,
                > {
                    let entry = PendingUpwardMessages;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn announced_hrmp_messages_per_candidate(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = AnnouncedHrmpMessagesPerCandidate;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn reserved_xcmp_weight_override(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u64>,
                    ::subxt::Error,
                > {
                    let entry = ReservedXcmpWeightOverride;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn reserved_dmp_weight_override(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u64>,
                    ::subxt::Error,
                > {
                    let entry = ReservedDmpWeightOverride;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn authorized_upgrade(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::subxt::sp_core::H256>,
                    ::subxt::Error,
                > {
                    let entry = AuthorizedUpgrade;
                    self.client.storage().fetch(&entry, hash).await
                }
            }
        }
    }
    pub mod parachain_info {
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub struct ParachainId;
            impl ::subxt::StorageEntry for ParachainId {
                const PALLET: &'static str = "ParachainInfo";
                const STORAGE: &'static str = "ParachainId";
                type Value = runtime_types::polkadot_parachain::primitives::Id;
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
                pub async fn parachain_id(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::polkadot_parachain::primitives::Id,
                    ::subxt::Error,
                > {
                    let entry = ParachainId;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod balances {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Transfer {
                pub dest:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                #[codec(compact)]
                pub value: ::core::primitive::u64,
            }
            impl ::subxt::Call for Transfer {
                const PALLET: &'static str = "Balances";
                const FUNCTION: &'static str = "transfer";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetBalance {
                pub who:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                #[codec(compact)]
                pub new_free: ::core::primitive::u64,
                #[codec(compact)]
                pub new_reserved: ::core::primitive::u64,
            }
            impl ::subxt::Call for SetBalance {
                const PALLET: &'static str = "Balances";
                const FUNCTION: &'static str = "set_balance";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ForceTransfer {
                pub source:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub dest:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                #[codec(compact)]
                pub value: ::core::primitive::u64,
            }
            impl ::subxt::Call for ForceTransfer {
                const PALLET: &'static str = "Balances";
                const FUNCTION: &'static str = "force_transfer";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct TransferKeepAlive {
                pub dest:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                #[codec(compact)]
                pub value: ::core::primitive::u64,
            }
            impl ::subxt::Call for TransferKeepAlive {
                const PALLET: &'static str = "Balances";
                const FUNCTION: &'static str = "transfer_keep_alive";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct TransferAll {
                pub dest:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub keep_alive: ::core::primitive::bool,
            }
            impl ::subxt::Call for TransferAll {
                const PALLET: &'static str = "Balances";
                const FUNCTION: &'static str = "transfer_all";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ForceUnreserve {
                pub who:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub amount: ::core::primitive::u64,
            }
            impl ::subxt::Call for ForceUnreserve {
                const PALLET: &'static str = "Balances";
                const FUNCTION: &'static str = "force_unreserve";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn transfer(
                    &self,
                    dest: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    value: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, Transfer> {
                    let call = Transfer { dest, value };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_balance(
                    &self,
                    who: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    new_free: ::core::primitive::u64,
                    new_reserved: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, SetBalance> {
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
                        (),
                    >,
                    dest: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    value: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, ForceTransfer> {
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
                        (),
                    >,
                    value: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, TransferKeepAlive> {
                    let call = TransferKeepAlive { dest, value };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn transfer_all(
                    &self,
                    dest: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    keep_alive: ::core::primitive::bool,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, TransferAll> {
                    let call = TransferAll { dest, keep_alive };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_unreserve(
                    &self,
                    who: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    amount: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, ForceUnreserve> {
                    let call = ForceUnreserve { who, amount };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_balances::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Endowed(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u64,
            );
            impl ::subxt::Event for Endowed {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Endowed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct DustLost(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u64,
            );
            impl ::subxt::Event for DustLost {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "DustLost";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Transfer(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u64,
            );
            impl ::subxt::Event for Transfer {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Transfer";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct BalanceSet(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u64,
                pub ::core::primitive::u64,
            );
            impl ::subxt::Event for BalanceSet {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "BalanceSet";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Deposit(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u64,
            );
            impl ::subxt::Event for Deposit {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Deposit";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Reserved(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u64,
            );
            impl ::subxt::Event for Reserved {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Reserved";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Unreserved(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u64,
            );
            impl ::subxt::Event for Unreserved {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Unreserved";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ReserveRepatriated(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u64,
                pub runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
            );
            impl ::subxt::Event for ReserveRepatriated {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "ReserveRepatriated";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct TotalIssuance;
            impl ::subxt::StorageEntry for TotalIssuance {
                const PALLET: &'static str = "Balances";
                const STORAGE: &'static str = "TotalIssuance";
                type Value = ::core::primitive::u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Account(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Account {
                const PALLET: &'static str = "Balances";
                const STORAGE: &'static str = "Account";
                type Value = runtime_types::pallet_balances::AccountData<::core::primitive::u64>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct Locks(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Locks {
                const PALLET: &'static str = "Balances";
                const STORAGE: &'static str = "Locks";
                type Value =
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
                        runtime_types::pallet_balances::BalanceLock<::core::primitive::u64>,
                    >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct Reserves(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Reserves {
                const PALLET: &'static str = "Balances";
                const STORAGE: &'static str = "Reserves";
                type Value = runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                    runtime_types::pallet_balances::ReserveData<
                        [::core::primitive::u8; 8usize],
                        ::core::primitive::u64,
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
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::Error>
                {
                    let entry = TotalIssuance;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn account(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_balances::AccountData<::core::primitive::u64>,
                    ::subxt::Error,
                > {
                    let entry = Account(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn account_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Account>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn locks(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
                        runtime_types::pallet_balances::BalanceLock<::core::primitive::u64>,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Locks(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn locks_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Locks>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn reserves(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        runtime_types::pallet_balances::ReserveData<
                            [::core::primitive::u8; 8usize],
                            ::core::primitive::u64,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Reserves(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn reserves_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Reserves>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn storage_version(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_balances::Releases,
                    ::subxt::Error,
                > {
                    let entry = StorageVersion;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod vesting {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Vest {}
            impl ::subxt::Call for Vest {
                const PALLET: &'static str = "Vesting";
                const FUNCTION: &'static str = "vest";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct VestOther {
                pub target:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
            }
            impl ::subxt::Call for VestOther {
                const PALLET: &'static str = "Vesting";
                const FUNCTION: &'static str = "vest_other";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct VestedTransfer {
                pub target:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub schedule: runtime_types::pallet_vesting::vesting_info::VestingInfo<
                    ::core::primitive::u64,
                    ::core::primitive::u32,
                >,
            }
            impl ::subxt::Call for VestedTransfer {
                const PALLET: &'static str = "Vesting";
                const FUNCTION: &'static str = "vested_transfer";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ForceVestedTransfer {
                pub source:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub target:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub schedule: runtime_types::pallet_vesting::vesting_info::VestingInfo<
                    ::core::primitive::u64,
                    ::core::primitive::u32,
                >,
            }
            impl ::subxt::Call for ForceVestedTransfer {
                const PALLET: &'static str = "Vesting";
                const FUNCTION: &'static str = "force_vested_transfer";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct MergeSchedules {
                pub schedule1_index: ::core::primitive::u32,
                pub schedule2_index: ::core::primitive::u32,
            }
            impl ::subxt::Call for MergeSchedules {
                const PALLET: &'static str = "Vesting";
                const FUNCTION: &'static str = "merge_schedules";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn vest(&self) -> ::subxt::SubmittableExtrinsic<'a, T, Vest> {
                    let call = Vest {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn vest_other(
                    &self,
                    target: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, VestOther> {
                    let call = VestOther { target };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn vested_transfer(
                    &self,
                    target: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    schedule: runtime_types::pallet_vesting::vesting_info::VestingInfo<
                        ::core::primitive::u64,
                        ::core::primitive::u32,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, VestedTransfer> {
                    let call = VestedTransfer { target, schedule };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_vested_transfer(
                    &self,
                    source: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    target: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    schedule: runtime_types::pallet_vesting::vesting_info::VestingInfo<
                        ::core::primitive::u64,
                        ::core::primitive::u32,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, ForceVestedTransfer> {
                    let call = ForceVestedTransfer {
                        source,
                        target,
                        schedule,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn merge_schedules(
                    &self,
                    schedule1_index: ::core::primitive::u32,
                    schedule2_index: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, MergeSchedules> {
                    let call = MergeSchedules {
                        schedule1_index,
                        schedule2_index,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_vesting::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct VestingUpdated(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u64,
            );
            impl ::subxt::Event for VestingUpdated {
                const PALLET: &'static str = "Vesting";
                const EVENT: &'static str = "VestingUpdated";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct VestingCompleted(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::Event for VestingCompleted {
                const PALLET: &'static str = "Vesting";
                const EVENT: &'static str = "VestingCompleted";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Vesting(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Vesting {
                const PALLET: &'static str = "Vesting";
                const STORAGE: &'static str = "Vesting";
                type Value = runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                    runtime_types::pallet_vesting::vesting_info::VestingInfo<
                        ::core::primitive::u64,
                        ::core::primitive::u32,
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
                const PALLET: &'static str = "Vesting";
                const STORAGE: &'static str = "StorageVersion";
                type Value = runtime_types::pallet_vesting::Releases;
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
                pub async fn vesting(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                            runtime_types::pallet_vesting::vesting_info::VestingInfo<
                                ::core::primitive::u64,
                                ::core::primitive::u32,
                            >,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Vesting(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn vesting_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Vesting>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn storage_version(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<runtime_types::pallet_vesting::Releases, ::subxt::Error>
                {
                    let entry = StorageVersion;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod parachain_staking {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetStakingExpectations {
                pub expectations:
                    runtime_types::parachain_staking::inflation::Range<::core::primitive::u64>,
            }
            impl ::subxt::Call for SetStakingExpectations {
                const PALLET: &'static str = "ParachainStaking";
                const FUNCTION: &'static str = "set_staking_expectations";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetInflation {
                pub schedule: runtime_types::parachain_staking::inflation::Range<
                    runtime_types::sp_arithmetic::per_things::Perbill,
                >,
            }
            impl ::subxt::Call for SetInflation {
                const PALLET: &'static str = "ParachainStaking";
                const FUNCTION: &'static str = "set_inflation";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetParachainBondAccount {
                pub new: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for SetParachainBondAccount {
                const PALLET: &'static str = "ParachainStaking";
                const FUNCTION: &'static str = "set_parachain_bond_account";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetParachainBondReservePercent {
                pub new: runtime_types::sp_arithmetic::per_things::Percent,
            }
            impl ::subxt::Call for SetParachainBondReservePercent {
                const PALLET: &'static str = "ParachainStaking";
                const FUNCTION: &'static str = "set_parachain_bond_reserve_percent";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetTotalSelected {
                pub new: ::core::primitive::u32,
            }
            impl ::subxt::Call for SetTotalSelected {
                const PALLET: &'static str = "ParachainStaking";
                const FUNCTION: &'static str = "set_total_selected";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetCollatorCommission {
                pub new: runtime_types::sp_arithmetic::per_things::Perbill,
            }
            impl ::subxt::Call for SetCollatorCommission {
                const PALLET: &'static str = "ParachainStaking";
                const FUNCTION: &'static str = "set_collator_commission";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetBlocksPerRound {
                pub new: ::core::primitive::u32,
            }
            impl ::subxt::Call for SetBlocksPerRound {
                const PALLET: &'static str = "ParachainStaking";
                const FUNCTION: &'static str = "set_blocks_per_round";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct JoinCandidates {
                pub bond: ::core::primitive::u64,
                pub candidate_count: ::core::primitive::u32,
            }
            impl ::subxt::Call for JoinCandidates {
                const PALLET: &'static str = "ParachainStaking";
                const FUNCTION: &'static str = "join_candidates";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct LeaveCandidates {
                pub candidate_count: ::core::primitive::u32,
            }
            impl ::subxt::Call for LeaveCandidates {
                const PALLET: &'static str = "ParachainStaking";
                const FUNCTION: &'static str = "leave_candidates";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct GoOffline {}
            impl ::subxt::Call for GoOffline {
                const PALLET: &'static str = "ParachainStaking";
                const FUNCTION: &'static str = "go_offline";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct GoOnline {}
            impl ::subxt::Call for GoOnline {
                const PALLET: &'static str = "ParachainStaking";
                const FUNCTION: &'static str = "go_online";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct CandidateBondMore {
                pub more: ::core::primitive::u64,
            }
            impl ::subxt::Call for CandidateBondMore {
                const PALLET: &'static str = "ParachainStaking";
                const FUNCTION: &'static str = "candidate_bond_more";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct CandidateBondLess {
                pub less: ::core::primitive::u64,
            }
            impl ::subxt::Call for CandidateBondLess {
                const PALLET: &'static str = "ParachainStaking";
                const FUNCTION: &'static str = "candidate_bond_less";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Nominate {
                pub collator: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u64,
                pub collator_nominator_count: ::core::primitive::u32,
                pub nomination_count: ::core::primitive::u32,
            }
            impl ::subxt::Call for Nominate {
                const PALLET: &'static str = "ParachainStaking";
                const FUNCTION: &'static str = "nominate";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct LeaveNominators {
                pub nomination_count: ::core::primitive::u32,
            }
            impl ::subxt::Call for LeaveNominators {
                const PALLET: &'static str = "ParachainStaking";
                const FUNCTION: &'static str = "leave_nominators";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct RevokeNomination {
                pub collator: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for RevokeNomination {
                const PALLET: &'static str = "ParachainStaking";
                const FUNCTION: &'static str = "revoke_nomination";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct NominatorBondMore {
                pub candidate: ::subxt::sp_core::crypto::AccountId32,
                pub more: ::core::primitive::u64,
            }
            impl ::subxt::Call for NominatorBondMore {
                const PALLET: &'static str = "ParachainStaking";
                const FUNCTION: &'static str = "nominator_bond_more";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct NominatorBondLess {
                pub candidate: ::subxt::sp_core::crypto::AccountId32,
                pub less: ::core::primitive::u64,
            }
            impl ::subxt::Call for NominatorBondLess {
                const PALLET: &'static str = "ParachainStaking";
                const FUNCTION: &'static str = "nominator_bond_less";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn set_staking_expectations(
                    &self,
                    expectations: runtime_types::parachain_staking::inflation::Range<
                        ::core::primitive::u64,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, SetStakingExpectations>
                {
                    let call = SetStakingExpectations { expectations };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_inflation(
                    &self,
                    schedule: runtime_types::parachain_staking::inflation::Range<
                        runtime_types::sp_arithmetic::per_things::Perbill,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, SetInflation> {
                    let call = SetInflation { schedule };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_parachain_bond_account(
                    &self,
                    new: ::subxt::sp_core::crypto::AccountId32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, SetParachainBondAccount>
                {
                    let call = SetParachainBondAccount { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_parachain_bond_reserve_percent(
                    &self,
                    new: runtime_types::sp_arithmetic::per_things::Percent,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, SetParachainBondReservePercent>
                {
                    let call = SetParachainBondReservePercent { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_total_selected(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, SetTotalSelected> {
                    let call = SetTotalSelected { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_collator_commission(
                    &self,
                    new: runtime_types::sp_arithmetic::per_things::Perbill,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, SetCollatorCommission> {
                    let call = SetCollatorCommission { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_blocks_per_round(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, SetBlocksPerRound> {
                    let call = SetBlocksPerRound { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn join_candidates(
                    &self,
                    bond: ::core::primitive::u64,
                    candidate_count: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, JoinCandidates> {
                    let call = JoinCandidates {
                        bond,
                        candidate_count,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn leave_candidates(
                    &self,
                    candidate_count: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, LeaveCandidates> {
                    let call = LeaveCandidates { candidate_count };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn go_offline(&self) -> ::subxt::SubmittableExtrinsic<'a, T, GoOffline> {
                    let call = GoOffline {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn go_online(&self) -> ::subxt::SubmittableExtrinsic<'a, T, GoOnline> {
                    let call = GoOnline {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn candidate_bond_more(
                    &self,
                    more: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, CandidateBondMore> {
                    let call = CandidateBondMore { more };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn candidate_bond_less(
                    &self,
                    less: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, CandidateBondLess> {
                    let call = CandidateBondLess { less };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn nominate(
                    &self,
                    collator: ::subxt::sp_core::crypto::AccountId32,
                    amount: ::core::primitive::u64,
                    collator_nominator_count: ::core::primitive::u32,
                    nomination_count: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, Nominate> {
                    let call = Nominate {
                        collator,
                        amount,
                        collator_nominator_count,
                        nomination_count,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn leave_nominators(
                    &self,
                    nomination_count: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, LeaveNominators> {
                    let call = LeaveNominators { nomination_count };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn revoke_nomination(
                    &self,
                    collator: ::subxt::sp_core::crypto::AccountId32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, RevokeNomination> {
                    let call = RevokeNomination { collator };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn nominator_bond_more(
                    &self,
                    candidate: ::subxt::sp_core::crypto::AccountId32,
                    more: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, NominatorBondMore> {
                    let call = NominatorBondMore { candidate, more };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn nominator_bond_less(
                    &self,
                    candidate: ::subxt::sp_core::crypto::AccountId32,
                    less: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, NominatorBondLess> {
                    let call = NominatorBondLess { candidate, less };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::parachain_staking::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct NewRound(
                pub ::core::primitive::u32,
                pub ::core::primitive::u32,
                pub ::core::primitive::u32,
                pub ::core::primitive::u64,
            );
            impl ::subxt::Event for NewRound {
                const PALLET: &'static str = "ParachainStaking";
                const EVENT: &'static str = "NewRound";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct JoinedCollatorCandidates(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u64,
                pub ::core::primitive::u64,
            );
            impl ::subxt::Event for JoinedCollatorCandidates {
                const PALLET: &'static str = "ParachainStaking";
                const EVENT: &'static str = "JoinedCollatorCandidates";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct CollatorChosen(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u64,
            );
            impl ::subxt::Event for CollatorChosen {
                const PALLET: &'static str = "ParachainStaking";
                const EVENT: &'static str = "CollatorChosen";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct CollatorBondedMore(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u64,
                pub ::core::primitive::u64,
            );
            impl ::subxt::Event for CollatorBondedMore {
                const PALLET: &'static str = "ParachainStaking";
                const EVENT: &'static str = "CollatorBondedMore";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct CollatorBondedLess(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u64,
                pub ::core::primitive::u64,
            );
            impl ::subxt::Event for CollatorBondedLess {
                const PALLET: &'static str = "ParachainStaking";
                const EVENT: &'static str = "CollatorBondedLess";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct CollatorWentOffline(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for CollatorWentOffline {
                const PALLET: &'static str = "ParachainStaking";
                const EVENT: &'static str = "CollatorWentOffline";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct CollatorBackOnline(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for CollatorBackOnline {
                const PALLET: &'static str = "ParachainStaking";
                const EVENT: &'static str = "CollatorBackOnline";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct CollatorScheduledExit(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u32,
            );
            impl ::subxt::Event for CollatorScheduledExit {
                const PALLET: &'static str = "ParachainStaking";
                const EVENT: &'static str = "CollatorScheduledExit";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct CollatorLeft(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u64,
                pub ::core::primitive::u64,
            );
            impl ::subxt::Event for CollatorLeft {
                const PALLET: &'static str = "ParachainStaking";
                const EVENT: &'static str = "CollatorLeft";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct NominationIncreased(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u64,
                pub ::core::primitive::bool,
                pub ::core::primitive::u64,
            );
            impl ::subxt::Event for NominationIncreased {
                const PALLET: &'static str = "ParachainStaking";
                const EVENT: &'static str = "NominationIncreased";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct NominationDecreased(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u64,
                pub ::core::primitive::bool,
                pub ::core::primitive::u64,
            );
            impl ::subxt::Event for NominationDecreased {
                const PALLET: &'static str = "ParachainStaking";
                const EVENT: &'static str = "NominationDecreased";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct NominatorExitScheduled(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u32,
            );
            impl ::subxt::Event for NominatorExitScheduled {
                const PALLET: &'static str = "ParachainStaking";
                const EVENT: &'static str = "NominatorExitScheduled";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct NominationRevocationScheduled(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u32,
            );
            impl ::subxt::Event for NominationRevocationScheduled {
                const PALLET: &'static str = "ParachainStaking";
                const EVENT: &'static str = "NominationRevocationScheduled";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct NominatorLeft(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u64,
            );
            impl ::subxt::Event for NominatorLeft {
                const PALLET: &'static str = "ParachainStaking";
                const EVENT: &'static str = "NominatorLeft";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Nomination(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u64,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub  runtime_types::parachain_staking::pallet::NominatorAdded<
                    ::core::primitive::u64,
                >,
            );
            impl ::subxt::Event for Nomination {
                const PALLET: &'static str = "ParachainStaking";
                const EVENT: &'static str = "Nomination";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct NominatorLeftCollator(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u64,
                pub ::core::primitive::u64,
            );
            impl ::subxt::Event for NominatorLeftCollator {
                const PALLET: &'static str = "ParachainStaking";
                const EVENT: &'static str = "NominatorLeftCollator";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Rewarded(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u64,
            );
            impl ::subxt::Event for Rewarded {
                const PALLET: &'static str = "ParachainStaking";
                const EVENT: &'static str = "Rewarded";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ReservedForParachainBond(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u64,
            );
            impl ::subxt::Event for ReservedForParachainBond {
                const PALLET: &'static str = "ParachainStaking";
                const EVENT: &'static str = "ReservedForParachainBond";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ParachainBondAccountSet(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for ParachainBondAccountSet {
                const PALLET: &'static str = "ParachainStaking";
                const EVENT: &'static str = "ParachainBondAccountSet";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ParachainBondReservePercentSet(
                pub runtime_types::sp_arithmetic::per_things::Percent,
                pub runtime_types::sp_arithmetic::per_things::Percent,
            );
            impl ::subxt::Event for ParachainBondReservePercentSet {
                const PALLET: &'static str = "ParachainStaking";
                const EVENT: &'static str = "ParachainBondReservePercentSet";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct InflationSet(
                pub runtime_types::sp_arithmetic::per_things::Perbill,
                pub runtime_types::sp_arithmetic::per_things::Perbill,
                pub runtime_types::sp_arithmetic::per_things::Perbill,
                pub runtime_types::sp_arithmetic::per_things::Perbill,
                pub runtime_types::sp_arithmetic::per_things::Perbill,
                pub runtime_types::sp_arithmetic::per_things::Perbill,
            );
            impl ::subxt::Event for InflationSet {
                const PALLET: &'static str = "ParachainStaking";
                const EVENT: &'static str = "InflationSet";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct StakeExpectationsSet(
                pub ::core::primitive::u64,
                pub ::core::primitive::u64,
                pub ::core::primitive::u64,
            );
            impl ::subxt::Event for StakeExpectationsSet {
                const PALLET: &'static str = "ParachainStaking";
                const EVENT: &'static str = "StakeExpectationsSet";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct TotalSelectedSet(pub ::core::primitive::u32, pub ::core::primitive::u32);
            impl ::subxt::Event for TotalSelectedSet {
                const PALLET: &'static str = "ParachainStaking";
                const EVENT: &'static str = "TotalSelectedSet";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct CollatorCommissionSet(
                pub runtime_types::sp_arithmetic::per_things::Perbill,
                pub runtime_types::sp_arithmetic::per_things::Perbill,
            );
            impl ::subxt::Event for CollatorCommissionSet {
                const PALLET: &'static str = "ParachainStaking";
                const EVENT: &'static str = "CollatorCommissionSet";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct BlocksPerRoundSet(
                pub ::core::primitive::u32,
                pub ::core::primitive::u32,
                pub ::core::primitive::u32,
                pub ::core::primitive::u32,
                pub runtime_types::sp_arithmetic::per_things::Perbill,
                pub runtime_types::sp_arithmetic::per_things::Perbill,
                pub runtime_types::sp_arithmetic::per_things::Perbill,
            );
            impl ::subxt::Event for BlocksPerRoundSet {
                const PALLET: &'static str = "ParachainStaking";
                const EVENT: &'static str = "BlocksPerRoundSet";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct DelayNominationExitsMigrationExecuted {}
            impl ::subxt::Event for DelayNominationExitsMigrationExecuted {
                const PALLET: &'static str = "ParachainStaking";
                const EVENT: &'static str = "DelayNominationExitsMigrationExecuted";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct DelayNominationExitsMigration;
            impl ::subxt::StorageEntry for DelayNominationExitsMigration {
                const PALLET: &'static str = "ParachainStaking";
                const STORAGE: &'static str = "DelayNominationExitsMigration";
                type Value = ::core::primitive::bool;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct CollatorCommission;
            impl ::subxt::StorageEntry for CollatorCommission {
                const PALLET: &'static str = "ParachainStaking";
                const STORAGE: &'static str = "CollatorCommission";
                type Value = runtime_types::sp_arithmetic::per_things::Perbill;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct TotalSelected;
            impl ::subxt::StorageEntry for TotalSelected {
                const PALLET: &'static str = "ParachainStaking";
                const STORAGE: &'static str = "TotalSelected";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ParachainBondInfo;
            impl ::subxt::StorageEntry for ParachainBondInfo {
                const PALLET: &'static str = "ParachainStaking";
                const STORAGE: &'static str = "ParachainBondInfo";
                type Value = runtime_types::parachain_staking::pallet::ParachainBondConfig<
                    ::subxt::sp_core::crypto::AccountId32,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Round;
            impl ::subxt::StorageEntry for Round {
                const PALLET: &'static str = "ParachainStaking";
                const STORAGE: &'static str = "Round";
                type Value =
                    runtime_types::parachain_staking::pallet::RoundInfo<::core::primitive::u32>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NominatorState(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for NominatorState {
                const PALLET: &'static str = "ParachainStaking";
                const STORAGE: &'static str = "NominatorState";
                type Value = runtime_types::parachain_staking::pallet::Nominator<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u64,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct NominatorState2(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for NominatorState2 {
                const PALLET: &'static str = "ParachainStaking";
                const STORAGE: &'static str = "NominatorState2";
                type Value = runtime_types::parachain_staking::pallet::Nominator2<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u64,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct CollatorState2(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for CollatorState2 {
                const PALLET: &'static str = "ParachainStaking";
                const STORAGE: &'static str = "CollatorState2";
                type Value = runtime_types::parachain_staking::pallet::Collator2<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u64,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct SelectedCandidates;
            impl ::subxt::StorageEntry for SelectedCandidates {
                const PALLET: &'static str = "ParachainStaking";
                const STORAGE: &'static str = "SelectedCandidates";
                type Value = ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Total;
            impl ::subxt::StorageEntry for Total {
                const PALLET: &'static str = "ParachainStaking";
                const STORAGE: &'static str = "Total";
                type Value = ::core::primitive::u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct CandidatePool;
            impl ::subxt::StorageEntry for CandidatePool {
                const PALLET: &'static str = "ParachainStaking";
                const STORAGE: &'static str = "CandidatePool";
                type Value = runtime_types::parachain_staking::set::OrderedSet<
                    runtime_types::parachain_staking::pallet::Bond<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u64,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ExitQueue;
            impl ::subxt::StorageEntry for ExitQueue {
                const PALLET: &'static str = "ParachainStaking";
                const STORAGE: &'static str = "ExitQueue";
                type Value = ::std::vec::Vec<(
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                )>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ExitQueue2;
            impl ::subxt::StorageEntry for ExitQueue2 {
                const PALLET: &'static str = "ParachainStaking";
                const STORAGE: &'static str = "ExitQueue2";
                type Value = runtime_types::parachain_staking::pallet::ExitQ<
                    ::subxt::sp_core::crypto::AccountId32,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct AtStake(
                ::core::primitive::u32,
                ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::StorageEntry for AtStake {
                const PALLET: &'static str = "ParachainStaking";
                const STORAGE: &'static str = "AtStake";
                type Value = runtime_types::parachain_staking::pallet::CollatorSnapshot<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u64,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct Staked(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for Staked {
                const PALLET: &'static str = "ParachainStaking";
                const STORAGE: &'static str = "Staked";
                type Value = ::core::primitive::u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct InflationConfig;
            impl ::subxt::StorageEntry for InflationConfig {
                const PALLET: &'static str = "ParachainStaking";
                const STORAGE: &'static str = "InflationConfig";
                type Value = runtime_types::parachain_staking::inflation::InflationInfo<
                    ::core::primitive::u64,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Points(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for Points {
                const PALLET: &'static str = "ParachainStaking";
                const STORAGE: &'static str = "Points";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct AwardedPts(
                ::core::primitive::u32,
                ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::StorageEntry for AwardedPts {
                const PALLET: &'static str = "ParachainStaking";
                const STORAGE: &'static str = "AwardedPts";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
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
                pub async fn delay_nomination_exits_migration(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::bool, ::subxt::Error>
                {
                    let entry = DelayNominationExitsMigration;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn collator_commission(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::sp_arithmetic::per_things::Perbill,
                    ::subxt::Error,
                > {
                    let entry = CollatorCommission;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn total_selected(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = TotalSelected;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn parachain_bond_info(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::parachain_staking::pallet::ParachainBondConfig<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                    ::subxt::Error,
                > {
                    let entry = ParachainBondInfo;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn round(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::parachain_staking::pallet::RoundInfo<::core::primitive::u32>,
                    ::subxt::Error,
                > {
                    let entry = Round;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn nominator_state(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::parachain_staking::pallet::Nominator<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u64,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = NominatorState(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn nominator_state_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, NominatorState>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn nominator_state2(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::parachain_staking::pallet::Nominator2<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u64,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = NominatorState2(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn nominator_state2_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, NominatorState2>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn collator_state2(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::parachain_staking::pallet::Collator2<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u64,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = CollatorState2(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn collator_state2_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, CollatorState2>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn selected_candidates(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                    ::subxt::Error,
                > {
                    let entry = SelectedCandidates;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn total(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::Error>
                {
                    let entry = Total;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn candidate_pool(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::parachain_staking::set::OrderedSet<
                        runtime_types::parachain_staking::pallet::Bond<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u64,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = CandidatePool;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn exit_queue(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    )>,
                    ::subxt::Error,
                > {
                    let entry = ExitQueue;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn exit_queue2(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::parachain_staking::pallet::ExitQ<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                    ::subxt::Error,
                > {
                    let entry = ExitQueue2;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn at_stake(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::parachain_staking::pallet::CollatorSnapshot<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u64,
                    >,
                    ::subxt::Error,
                > {
                    let entry = AtStake(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn at_stake_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, AtStake>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn staked(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::Error>
                {
                    let entry = Staked(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn staked_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Staked>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn inflation_config(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::parachain_staking::inflation::InflationInfo<
                        ::core::primitive::u64,
                    >,
                    ::subxt::Error,
                > {
                    let entry = InflationConfig;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn points(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = Points(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn points_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Points>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn awarded_pts(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = AwardedPts(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn awarded_pts_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, AwardedPts>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod author_inherent {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetAuthor {
                pub author: runtime_types::nimbus_primitives::nimbus_crypto::Public,
            }
            impl ::subxt::Call for SetAuthor {
                const PALLET: &'static str = "AuthorInherent";
                const FUNCTION: &'static str = "set_author";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn set_author(
                    &self,
                    author: runtime_types::nimbus_primitives::nimbus_crypto::Public,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, SetAuthor> {
                    let call = SetAuthor { author };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Author;
            impl ::subxt::StorageEntry for Author {
                const PALLET: &'static str = "AuthorInherent";
                const STORAGE: &'static str = "Author";
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
                pub async fn author(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
                    ::subxt::Error,
                > {
                    let entry = Author;
                    self.client.storage().fetch(&entry, hash).await
                }
            }
        }
    }
    pub mod author_filter {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetEligible {
                pub new: runtime_types::sp_arithmetic::per_things::Percent,
            }
            impl ::subxt::Call for SetEligible {
                const PALLET: &'static str = "AuthorFilter";
                const FUNCTION: &'static str = "set_eligible";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn set_eligible(
                    &self,
                    new: runtime_types::sp_arithmetic::per_things::Percent,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, SetEligible> {
                    let call = SetEligible { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_author_slot_filter::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct EligibleUpdated(pub runtime_types::sp_arithmetic::per_things::Percent);
            impl ::subxt::Event for EligibleUpdated {
                const PALLET: &'static str = "AuthorFilter";
                const EVENT: &'static str = "EligibleUpdated";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct EligibleRatio;
            impl ::subxt::StorageEntry for EligibleRatio {
                const PALLET: &'static str = "AuthorFilter";
                const STORAGE: &'static str = "EligibleRatio";
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
                pub async fn eligible_ratio(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::sp_arithmetic::per_things::Percent,
                    ::subxt::Error,
                > {
                    let entry = EligibleRatio;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod author_mapping {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct AddAssociation {
                pub author_id: runtime_types::nimbus_primitives::nimbus_crypto::Public,
            }
            impl ::subxt::Call for AddAssociation {
                const PALLET: &'static str = "AuthorMapping";
                const FUNCTION: &'static str = "add_association";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct UpdateAssociation {
                pub old_author_id: runtime_types::nimbus_primitives::nimbus_crypto::Public,
                pub new_author_id: runtime_types::nimbus_primitives::nimbus_crypto::Public,
            }
            impl ::subxt::Call for UpdateAssociation {
                const PALLET: &'static str = "AuthorMapping";
                const FUNCTION: &'static str = "update_association";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ClearAssociation {
                pub author_id: runtime_types::nimbus_primitives::nimbus_crypto::Public,
            }
            impl ::subxt::Call for ClearAssociation {
                const PALLET: &'static str = "AuthorMapping";
                const FUNCTION: &'static str = "clear_association";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn add_association(
                    &self,
                    author_id: runtime_types::nimbus_primitives::nimbus_crypto::Public,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, AddAssociation> {
                    let call = AddAssociation { author_id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn update_association(
                    &self,
                    old_author_id: runtime_types::nimbus_primitives::nimbus_crypto::Public,
                    new_author_id: runtime_types::nimbus_primitives::nimbus_crypto::Public,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, UpdateAssociation> {
                    let call = UpdateAssociation {
                        old_author_id,
                        new_author_id,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn clear_association(
                    &self,
                    author_id: runtime_types::nimbus_primitives::nimbus_crypto::Public,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, ClearAssociation> {
                    let call = ClearAssociation { author_id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_author_mapping::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct AuthorRegistered(
                pub runtime_types::nimbus_primitives::nimbus_crypto::Public,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for AuthorRegistered {
                const PALLET: &'static str = "AuthorMapping";
                const EVENT: &'static str = "AuthorRegistered";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct AuthorDeRegistered(
                pub runtime_types::nimbus_primitives::nimbus_crypto::Public,
            );
            impl ::subxt::Event for AuthorDeRegistered {
                const PALLET: &'static str = "AuthorMapping";
                const EVENT: &'static str = "AuthorDeRegistered";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct AuthorRotated(
                pub runtime_types::nimbus_primitives::nimbus_crypto::Public,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for AuthorRotated {
                const PALLET: &'static str = "AuthorMapping";
                const EVENT: &'static str = "AuthorRotated";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct DefunctAuthorBusted(
                pub runtime_types::nimbus_primitives::nimbus_crypto::Public,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for DefunctAuthorBusted {
                const PALLET: &'static str = "AuthorMapping";
                const EVENT: &'static str = "DefunctAuthorBusted";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct MappingWithDeposit(
                pub runtime_types::nimbus_primitives::nimbus_crypto::Public,
            );
            impl ::subxt::StorageEntry for MappingWithDeposit {
                const PALLET: &'static str = "AuthorMapping";
                const STORAGE: &'static str = "MappingWithDeposit";
                type Value = runtime_types::pallet_author_mapping::pallet::RegistrationInfo<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u64,
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
                pub async fn mapping_with_deposit(
                    &self,
                    _0: runtime_types::nimbus_primitives::nimbus_crypto::Public,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_author_mapping::pallet::RegistrationInfo<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u64,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = MappingWithDeposit(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn mapping_with_deposit_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, MappingWithDeposit>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod xcmp_queue {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
            }
        }
        pub type Event = runtime_types::cumulus_pallet_xcmp_queue::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Success(pub ::core::option::Option<::subxt::sp_core::H256>);
            impl ::subxt::Event for Success {
                const PALLET: &'static str = "XcmpQueue";
                const EVENT: &'static str = "Success";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Fail(
                pub ::core::option::Option<::subxt::sp_core::H256>,
                pub runtime_types::xcm::v2::traits::Error,
            );
            impl ::subxt::Event for Fail {
                const PALLET: &'static str = "XcmpQueue";
                const EVENT: &'static str = "Fail";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct BadVersion(pub ::core::option::Option<::subxt::sp_core::H256>);
            impl ::subxt::Event for BadVersion {
                const PALLET: &'static str = "XcmpQueue";
                const EVENT: &'static str = "BadVersion";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct BadFormat(pub ::core::option::Option<::subxt::sp_core::H256>);
            impl ::subxt::Event for BadFormat {
                const PALLET: &'static str = "XcmpQueue";
                const EVENT: &'static str = "BadFormat";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct UpwardMessageSent(pub ::core::option::Option<::subxt::sp_core::H256>);
            impl ::subxt::Event for UpwardMessageSent {
                const PALLET: &'static str = "XcmpQueue";
                const EVENT: &'static str = "UpwardMessageSent";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct XcmpMessageSent(pub ::core::option::Option<::subxt::sp_core::H256>);
            impl ::subxt::Event for XcmpMessageSent {
                const PALLET: &'static str = "XcmpQueue";
                const EVENT: &'static str = "XcmpMessageSent";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct InboundXcmpStatus;
            impl ::subxt::StorageEntry for InboundXcmpStatus {
                const PALLET: &'static str = "XcmpQueue";
                const STORAGE: &'static str = "InboundXcmpStatus";
                type Value = ::std::vec::Vec<(
                    runtime_types::polkadot_parachain::primitives::Id,
                    runtime_types::cumulus_pallet_xcmp_queue::InboundStatus,
                    ::std::vec::Vec<(
                        ::core::primitive::u32,
                        runtime_types::polkadot_parachain::primitives::XcmpMessageFormat,
                    )>,
                )>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct InboundXcmpMessages(
                runtime_types::polkadot_parachain::primitives::Id,
                ::core::primitive::u32,
            );
            impl ::subxt::StorageEntry for InboundXcmpMessages {
                const PALLET: &'static str = "XcmpQueue";
                const STORAGE: &'static str = "InboundXcmpMessages";
                type Value = ::std::vec::Vec<::core::primitive::u8>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct OutboundXcmpStatus;
            impl ::subxt::StorageEntry for OutboundXcmpStatus {
                const PALLET: &'static str = "XcmpQueue";
                const STORAGE: &'static str = "OutboundXcmpStatus";
                type Value = ::std::vec::Vec<(
                    runtime_types::polkadot_parachain::primitives::Id,
                    runtime_types::cumulus_pallet_xcmp_queue::OutboundStatus,
                    ::core::primitive::bool,
                    ::core::primitive::u16,
                    ::core::primitive::u16,
                )>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct OutboundXcmpMessages(
                runtime_types::polkadot_parachain::primitives::Id,
                ::core::primitive::u16,
            );
            impl ::subxt::StorageEntry for OutboundXcmpMessages {
                const PALLET: &'static str = "XcmpQueue";
                const STORAGE: &'static str = "OutboundXcmpMessages";
                type Value = ::std::vec::Vec<::core::primitive::u8>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                    ])
                }
            }
            pub struct SignalMessages(pub runtime_types::polkadot_parachain::primitives::Id);
            impl ::subxt::StorageEntry for SignalMessages {
                const PALLET: &'static str = "XcmpQueue";
                const STORAGE: &'static str = "SignalMessages";
                type Value = ::std::vec::Vec<::core::primitive::u8>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct QueueConfig;
            impl ::subxt::StorageEntry for QueueConfig {
                const PALLET: &'static str = "XcmpQueue";
                const STORAGE: &'static str = "QueueConfig";
                type Value = runtime_types::cumulus_pallet_xcmp_queue::QueueConfigData;
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
                pub async fn inbound_xcmp_status(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(
                        runtime_types::polkadot_parachain::primitives::Id,
                        runtime_types::cumulus_pallet_xcmp_queue::InboundStatus,
                        ::std::vec::Vec<(
                            ::core::primitive::u32,
                            runtime_types::polkadot_parachain::primitives::XcmpMessageFormat,
                        )>,
                    )>,
                    ::subxt::Error,
                > {
                    let entry = InboundXcmpStatus;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn inbound_xcmp_messages(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::Id,
                    _1: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::std::vec::Vec<::core::primitive::u8>, ::subxt::Error>
                {
                    let entry = InboundXcmpMessages(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn inbound_xcmp_messages_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, InboundXcmpMessages>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn outbound_xcmp_status(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(
                        runtime_types::polkadot_parachain::primitives::Id,
                        runtime_types::cumulus_pallet_xcmp_queue::OutboundStatus,
                        ::core::primitive::bool,
                        ::core::primitive::u16,
                        ::core::primitive::u16,
                    )>,
                    ::subxt::Error,
                > {
                    let entry = OutboundXcmpStatus;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn outbound_xcmp_messages(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::Id,
                    _1: ::core::primitive::u16,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::std::vec::Vec<::core::primitive::u8>, ::subxt::Error>
                {
                    let entry = OutboundXcmpMessages(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn outbound_xcmp_messages_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, OutboundXcmpMessages>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn signal_messages(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::Id,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::std::vec::Vec<::core::primitive::u8>, ::subxt::Error>
                {
                    let entry = SignalMessages(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn signal_messages_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, SignalMessages>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn queue_config(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::cumulus_pallet_xcmp_queue::QueueConfigData,
                    ::subxt::Error,
                > {
                    let entry = QueueConfig;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod polkadot_xcm {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Send {
                pub dest: runtime_types::xcm::VersionedMultiLocation,
                pub message: runtime_types::xcm::VersionedXcm,
            }
            impl ::subxt::Call for Send {
                const PALLET: &'static str = "PolkadotXcm";
                const FUNCTION: &'static str = "send";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct TeleportAssets {
                pub dest: runtime_types::xcm::VersionedMultiLocation,
                pub beneficiary: runtime_types::xcm::VersionedMultiLocation,
                pub assets: runtime_types::xcm::VersionedMultiAssets,
                pub fee_asset_item: ::core::primitive::u32,
            }
            impl ::subxt::Call for TeleportAssets {
                const PALLET: &'static str = "PolkadotXcm";
                const FUNCTION: &'static str = "teleport_assets";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ReserveTransferAssets {
                pub dest: runtime_types::xcm::VersionedMultiLocation,
                pub beneficiary: runtime_types::xcm::VersionedMultiLocation,
                pub assets: runtime_types::xcm::VersionedMultiAssets,
                pub fee_asset_item: ::core::primitive::u32,
            }
            impl ::subxt::Call for ReserveTransferAssets {
                const PALLET: &'static str = "PolkadotXcm";
                const FUNCTION: &'static str = "reserve_transfer_assets";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Execute {
                pub message: runtime_types::xcm::VersionedXcm,
                pub max_weight: ::core::primitive::u64,
            }
            impl ::subxt::Call for Execute {
                const PALLET: &'static str = "PolkadotXcm";
                const FUNCTION: &'static str = "execute";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ForceXcmVersion {
                pub location: runtime_types::xcm::v1::multilocation::MultiLocation,
                pub xcm_version: ::core::primitive::u32,
            }
            impl ::subxt::Call for ForceXcmVersion {
                const PALLET: &'static str = "PolkadotXcm";
                const FUNCTION: &'static str = "force_xcm_version";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ForceDefaultXcmVersion {
                pub maybe_xcm_version: ::core::option::Option<::core::primitive::u32>,
            }
            impl ::subxt::Call for ForceDefaultXcmVersion {
                const PALLET: &'static str = "PolkadotXcm";
                const FUNCTION: &'static str = "force_default_xcm_version";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ForceSubscribeVersionNotify {
                pub location: runtime_types::xcm::VersionedMultiLocation,
            }
            impl ::subxt::Call for ForceSubscribeVersionNotify {
                const PALLET: &'static str = "PolkadotXcm";
                const FUNCTION: &'static str = "force_subscribe_version_notify";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ForceUnsubscribeVersionNotify {
                pub location: runtime_types::xcm::VersionedMultiLocation,
            }
            impl ::subxt::Call for ForceUnsubscribeVersionNotify {
                const PALLET: &'static str = "PolkadotXcm";
                const FUNCTION: &'static str = "force_unsubscribe_version_notify";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn send(
                    &self,
                    dest: runtime_types::xcm::VersionedMultiLocation,
                    message: runtime_types::xcm::VersionedXcm,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, Send> {
                    let call = Send { dest, message };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn teleport_assets(
                    &self,
                    dest: runtime_types::xcm::VersionedMultiLocation,
                    beneficiary: runtime_types::xcm::VersionedMultiLocation,
                    assets: runtime_types::xcm::VersionedMultiAssets,
                    fee_asset_item: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, TeleportAssets> {
                    let call = TeleportAssets {
                        dest,
                        beneficiary,
                        assets,
                        fee_asset_item,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn reserve_transfer_assets(
                    &self,
                    dest: runtime_types::xcm::VersionedMultiLocation,
                    beneficiary: runtime_types::xcm::VersionedMultiLocation,
                    assets: runtime_types::xcm::VersionedMultiAssets,
                    fee_asset_item: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, ReserveTransferAssets> {
                    let call = ReserveTransferAssets {
                        dest,
                        beneficiary,
                        assets,
                        fee_asset_item,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn execute(
                    &self,
                    message: runtime_types::xcm::VersionedXcm,
                    max_weight: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, Execute> {
                    let call = Execute {
                        message,
                        max_weight,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_xcm_version(
                    &self,
                    location: runtime_types::xcm::v1::multilocation::MultiLocation,
                    xcm_version: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, ForceXcmVersion> {
                    let call = ForceXcmVersion {
                        location,
                        xcm_version,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_default_xcm_version(
                    &self,
                    maybe_xcm_version: ::core::option::Option<::core::primitive::u32>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, ForceDefaultXcmVersion>
                {
                    let call = ForceDefaultXcmVersion { maybe_xcm_version };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_subscribe_version_notify(
                    &self,
                    location: runtime_types::xcm::VersionedMultiLocation,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, ForceSubscribeVersionNotify>
                {
                    let call = ForceSubscribeVersionNotify { location };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_unsubscribe_version_notify(
                    &self,
                    location: runtime_types::xcm::VersionedMultiLocation,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, ForceUnsubscribeVersionNotify>
                {
                    let call = ForceUnsubscribeVersionNotify { location };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_xcm::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Attempted(pub runtime_types::xcm::v2::traits::Outcome);
            impl ::subxt::Event for Attempted {
                const PALLET: &'static str = "PolkadotXcm";
                const EVENT: &'static str = "Attempted";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Sent(
                pub runtime_types::xcm::v1::multilocation::MultiLocation,
                pub runtime_types::xcm::v1::multilocation::MultiLocation,
                pub runtime_types::xcm::v2::Xcm,
            );
            impl ::subxt::Event for Sent {
                const PALLET: &'static str = "PolkadotXcm";
                const EVENT: &'static str = "Sent";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct UnexpectedResponse(
                pub runtime_types::xcm::v1::multilocation::MultiLocation,
                pub ::core::primitive::u64,
            );
            impl ::subxt::Event for UnexpectedResponse {
                const PALLET: &'static str = "PolkadotXcm";
                const EVENT: &'static str = "UnexpectedResponse";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ResponseReady(
                pub ::core::primitive::u64,
                pub runtime_types::xcm::v2::Response,
            );
            impl ::subxt::Event for ResponseReady {
                const PALLET: &'static str = "PolkadotXcm";
                const EVENT: &'static str = "ResponseReady";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Notified(
                pub ::core::primitive::u64,
                pub ::core::primitive::u8,
                pub ::core::primitive::u8,
            );
            impl ::subxt::Event for Notified {
                const PALLET: &'static str = "PolkadotXcm";
                const EVENT: &'static str = "Notified";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct NotifyOverweight(
                pub ::core::primitive::u64,
                pub ::core::primitive::u8,
                pub ::core::primitive::u8,
                pub ::core::primitive::u64,
                pub ::core::primitive::u64,
            );
            impl ::subxt::Event for NotifyOverweight {
                const PALLET: &'static str = "PolkadotXcm";
                const EVENT: &'static str = "NotifyOverweight";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct NotifyDispatchError(
                pub ::core::primitive::u64,
                pub ::core::primitive::u8,
                pub ::core::primitive::u8,
            );
            impl ::subxt::Event for NotifyDispatchError {
                const PALLET: &'static str = "PolkadotXcm";
                const EVENT: &'static str = "NotifyDispatchError";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct NotifyDecodeFailed(
                pub ::core::primitive::u64,
                pub ::core::primitive::u8,
                pub ::core::primitive::u8,
            );
            impl ::subxt::Event for NotifyDecodeFailed {
                const PALLET: &'static str = "PolkadotXcm";
                const EVENT: &'static str = "NotifyDecodeFailed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct InvalidResponder(
                pub runtime_types::xcm::v1::multilocation::MultiLocation,
                pub ::core::primitive::u64,
                pub ::core::option::Option<runtime_types::xcm::v1::multilocation::MultiLocation>,
            );
            impl ::subxt::Event for InvalidResponder {
                const PALLET: &'static str = "PolkadotXcm";
                const EVENT: &'static str = "InvalidResponder";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct InvalidResponderVersion(
                pub runtime_types::xcm::v1::multilocation::MultiLocation,
                pub ::core::primitive::u64,
            );
            impl ::subxt::Event for InvalidResponderVersion {
                const PALLET: &'static str = "PolkadotXcm";
                const EVENT: &'static str = "InvalidResponderVersion";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ResponseTaken(pub ::core::primitive::u64);
            impl ::subxt::Event for ResponseTaken {
                const PALLET: &'static str = "PolkadotXcm";
                const EVENT: &'static str = "ResponseTaken";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct AssetsTrapped(
                pub ::subxt::sp_core::H256,
                pub runtime_types::xcm::v1::multilocation::MultiLocation,
                pub runtime_types::xcm::VersionedMultiAssets,
            );
            impl ::subxt::Event for AssetsTrapped {
                const PALLET: &'static str = "PolkadotXcm";
                const EVENT: &'static str = "AssetsTrapped";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct VersionChangeNotified(
                pub runtime_types::xcm::v1::multilocation::MultiLocation,
                pub ::core::primitive::u32,
            );
            impl ::subxt::Event for VersionChangeNotified {
                const PALLET: &'static str = "PolkadotXcm";
                const EVENT: &'static str = "VersionChangeNotified";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SupportedVersionChanged(
                pub runtime_types::xcm::v1::multilocation::MultiLocation,
                pub ::core::primitive::u32,
            );
            impl ::subxt::Event for SupportedVersionChanged {
                const PALLET: &'static str = "PolkadotXcm";
                const EVENT: &'static str = "SupportedVersionChanged";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct NotifyTargetSendFail(
                pub runtime_types::xcm::v1::multilocation::MultiLocation,
                pub ::core::primitive::u64,
                pub runtime_types::xcm::v2::traits::Error,
            );
            impl ::subxt::Event for NotifyTargetSendFail {
                const PALLET: &'static str = "PolkadotXcm";
                const EVENT: &'static str = "NotifyTargetSendFail";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct NotifyTargetMigrationFail(
                pub runtime_types::xcm::VersionedMultiLocation,
                pub ::core::primitive::u64,
            );
            impl ::subxt::Event for NotifyTargetMigrationFail {
                const PALLET: &'static str = "PolkadotXcm";
                const EVENT: &'static str = "NotifyTargetMigrationFail";
            }
        }
    }
    pub mod cumulus_xcm {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
            }
        }
        pub type Event = runtime_types::cumulus_pallet_xcm::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct InvalidFormat(pub [::core::primitive::u8; 8usize]);
            impl ::subxt::Event for InvalidFormat {
                const PALLET: &'static str = "CumulusXcm";
                const EVENT: &'static str = "InvalidFormat";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct UnsupportedVersion(pub [::core::primitive::u8; 8usize]);
            impl ::subxt::Event for UnsupportedVersion {
                const PALLET: &'static str = "CumulusXcm";
                const EVENT: &'static str = "UnsupportedVersion";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ExecutedDownward(
                pub [::core::primitive::u8; 8usize],
                pub runtime_types::xcm::v2::traits::Outcome,
            );
            impl ::subxt::Event for ExecutedDownward {
                const PALLET: &'static str = "CumulusXcm";
                const EVENT: &'static str = "ExecutedDownward";
            }
        }
    }
    pub mod dmp_queue {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ServiceOverweight {
                pub index: ::core::primitive::u64,
                pub weight_limit: ::core::primitive::u64,
            }
            impl ::subxt::Call for ServiceOverweight {
                const PALLET: &'static str = "DmpQueue";
                const FUNCTION: &'static str = "service_overweight";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn service_overweight(
                    &self,
                    index: ::core::primitive::u64,
                    weight_limit: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, ServiceOverweight> {
                    let call = ServiceOverweight {
                        index,
                        weight_limit,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::cumulus_pallet_dmp_queue::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct InvalidFormat(pub [::core::primitive::u8; 32usize]);
            impl ::subxt::Event for InvalidFormat {
                const PALLET: &'static str = "DmpQueue";
                const EVENT: &'static str = "InvalidFormat";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct UnsupportedVersion(pub [::core::primitive::u8; 32usize]);
            impl ::subxt::Event for UnsupportedVersion {
                const PALLET: &'static str = "DmpQueue";
                const EVENT: &'static str = "UnsupportedVersion";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ExecutedDownward(
                pub [::core::primitive::u8; 32usize],
                pub runtime_types::xcm::v2::traits::Outcome,
            );
            impl ::subxt::Event for ExecutedDownward {
                const PALLET: &'static str = "DmpQueue";
                const EVENT: &'static str = "ExecutedDownward";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct WeightExhausted(
                pub [::core::primitive::u8; 32usize],
                pub ::core::primitive::u64,
                pub ::core::primitive::u64,
            );
            impl ::subxt::Event for WeightExhausted {
                const PALLET: &'static str = "DmpQueue";
                const EVENT: &'static str = "WeightExhausted";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct OverweightEnqueued(
                pub [::core::primitive::u8; 32usize],
                pub ::core::primitive::u64,
                pub ::core::primitive::u64,
            );
            impl ::subxt::Event for OverweightEnqueued {
                const PALLET: &'static str = "DmpQueue";
                const EVENT: &'static str = "OverweightEnqueued";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct OverweightServiced(pub ::core::primitive::u64, pub ::core::primitive::u64);
            impl ::subxt::Event for OverweightServiced {
                const PALLET: &'static str = "DmpQueue";
                const EVENT: &'static str = "OverweightServiced";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Configuration;
            impl ::subxt::StorageEntry for Configuration {
                const PALLET: &'static str = "DmpQueue";
                const STORAGE: &'static str = "Configuration";
                type Value = runtime_types::cumulus_pallet_dmp_queue::ConfigData;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct PageIndex;
            impl ::subxt::StorageEntry for PageIndex {
                const PALLET: &'static str = "DmpQueue";
                const STORAGE: &'static str = "PageIndex";
                type Value = runtime_types::cumulus_pallet_dmp_queue::PageIndexData;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Pages(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for Pages {
                const PALLET: &'static str = "DmpQueue";
                const STORAGE: &'static str = "Pages";
                type Value = ::std::vec::Vec<(
                    ::core::primitive::u32,
                    ::std::vec::Vec<::core::primitive::u8>,
                )>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct Overweight(pub ::core::primitive::u64);
            impl ::subxt::StorageEntry for Overweight {
                const PALLET: &'static str = "DmpQueue";
                const STORAGE: &'static str = "Overweight";
                type Value = (
                    ::core::primitive::u32,
                    ::std::vec::Vec<::core::primitive::u8>,
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
                pub async fn configuration(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::cumulus_pallet_dmp_queue::ConfigData,
                    ::subxt::Error,
                > {
                    let entry = Configuration;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn page_index(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::cumulus_pallet_dmp_queue::PageIndexData,
                    ::subxt::Error,
                > {
                    let entry = PageIndex;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn pages(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(
                        ::core::primitive::u32,
                        ::std::vec::Vec<::core::primitive::u8>,
                    )>,
                    ::subxt::Error,
                > {
                    let entry = Pages(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn pages_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Pages>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn overweight(
                    &self,
                    _0: ::core::primitive::u64,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<(
                        ::core::primitive::u32,
                        ::std::vec::Vec<::core::primitive::u8>,
                    )>,
                    ::subxt::Error,
                > {
                    let entry = Overweight(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn overweight_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Overweight>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod mvm {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Execute {
                pub tx_bc: ::std::vec::Vec<::core::primitive::u8>,
                pub gas_limit: ::core::primitive::u64,
            }
            impl ::subxt::Call for Execute {
                const PALLET: &'static str = "Mvm";
                const FUNCTION: &'static str = "execute";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct PublishModule {
                pub module_bc: ::std::vec::Vec<::core::primitive::u8>,
                pub gas_limit: ::core::primitive::u64,
            }
            impl ::subxt::Call for PublishModule {
                const PALLET: &'static str = "Mvm";
                const FUNCTION: &'static str = "publish_module";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct PublishPackage {
                pub package: ::std::vec::Vec<::core::primitive::u8>,
                pub gas_limit: ::core::primitive::u64,
            }
            impl ::subxt::Call for PublishPackage {
                const PALLET: &'static str = "Mvm";
                const FUNCTION: &'static str = "publish_package";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn execute(
                    &self,
                    tx_bc: ::std::vec::Vec<::core::primitive::u8>,
                    gas_limit: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, Execute> {
                    let call = Execute { tx_bc, gas_limit };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn publish_module(
                    &self,
                    module_bc: ::std::vec::Vec<::core::primitive::u8>,
                    gas_limit: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, PublishModule> {
                    let call = PublishModule {
                        module_bc,
                        gas_limit,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn publish_package(
                    &self,
                    package: ::std::vec::Vec<::core::primitive::u8>,
                    gas_limit: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, PublishPackage> {
                    let call = PublishPackage { package, gas_limit };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::sp_mvm::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Event(
                pub ::std::vec::Vec<::core::primitive::u8>,
                pub ::std::vec::Vec<::core::primitive::u8>,
                pub ::std::vec::Vec<::core::primitive::u8>,
            );
            impl ::subxt::Event for Event {
                const PALLET: &'static str = "Mvm";
                const EVENT: &'static str = "Event";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ModulePublished(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::Event for ModulePublished {
                const PALLET: &'static str = "Mvm";
                const EVENT: &'static str = "ModulePublished";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct StdModulePublished {}
            impl ::subxt::Event for StdModulePublished {
                const PALLET: &'static str = "Mvm";
                const EVENT: &'static str = "StdModulePublished";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct VMStorage(pub ::std::vec::Vec<::core::primitive::u8>);
            impl ::subxt::StorageEntry for VMStorage {
                const PALLET: &'static str = "Mvm";
                const STORAGE: &'static str = "VMStorage";
                type Value = ::std::vec::Vec<::core::primitive::u8>;
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
                pub async fn vm_storage(
                    &self,
                    _0: ::std::vec::Vec<::core::primitive::u8>,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
                    ::subxt::Error,
                > {
                    let entry = VMStorage(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn vm_storage_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, VMStorage>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod multi_sig {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct AsMultiThreshold1 {
                pub other_signatories: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                pub call: runtime_types::pontem_runtime::Call,
            }
            impl ::subxt::Call for AsMultiThreshold1 {
                const PALLET: &'static str = "MultiSig";
                const FUNCTION: &'static str = "as_multi_threshold1";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct AsMulti {
                pub threshold: ::core::primitive::u16,
                pub other_signatories: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                pub maybe_timepoint: ::core::option::Option<
                    runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                >,
                pub call: ::std::vec::Vec<::core::primitive::u8>,
                pub store_call: ::core::primitive::bool,
                pub max_weight: ::core::primitive::u64,
            }
            impl ::subxt::Call for AsMulti {
                const PALLET: &'static str = "MultiSig";
                const FUNCTION: &'static str = "as_multi";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ApproveAsMulti {
                pub threshold: ::core::primitive::u16,
                pub other_signatories: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                pub maybe_timepoint: ::core::option::Option<
                    runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                >,
                pub call_hash: [::core::primitive::u8; 32usize],
                pub max_weight: ::core::primitive::u64,
            }
            impl ::subxt::Call for ApproveAsMulti {
                const PALLET: &'static str = "MultiSig";
                const FUNCTION: &'static str = "approve_as_multi";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct CancelAsMulti {
                pub threshold: ::core::primitive::u16,
                pub other_signatories: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                pub timepoint: runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                pub call_hash: [::core::primitive::u8; 32usize],
            }
            impl ::subxt::Call for CancelAsMulti {
                const PALLET: &'static str = "MultiSig";
                const FUNCTION: &'static str = "cancel_as_multi";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn as_multi_threshold1(
                    &self,
                    other_signatories: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                    call: runtime_types::pontem_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, AsMultiThreshold1> {
                    let call = AsMultiThreshold1 {
                        other_signatories,
                        call,
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
                    call: ::std::vec::Vec<::core::primitive::u8>,
                    store_call: ::core::primitive::bool,
                    max_weight: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, AsMulti> {
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
                ) -> ::subxt::SubmittableExtrinsic<'a, T, ApproveAsMulti> {
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
                ) -> ::subxt::SubmittableExtrinsic<'a, T, CancelAsMulti> {
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
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct NewMultisig(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub [::core::primitive::u8; 32usize],
            );
            impl ::subxt::Event for NewMultisig {
                const PALLET: &'static str = "MultiSig";
                const EVENT: &'static str = "NewMultisig";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct MultisigApproval(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub [::core::primitive::u8; 32usize],
            );
            impl ::subxt::Event for MultisigApproval {
                const PALLET: &'static str = "MultiSig";
                const EVENT: &'static str = "MultisigApproval";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct MultisigExecuted(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub [::core::primitive::u8; 32usize],
                pub ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            );
            impl ::subxt::Event for MultisigExecuted {
                const PALLET: &'static str = "MultiSig";
                const EVENT: &'static str = "MultisigExecuted";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct MultisigCancelled(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub [::core::primitive::u8; 32usize],
            );
            impl ::subxt::Event for MultisigCancelled {
                const PALLET: &'static str = "MultiSig";
                const EVENT: &'static str = "MultisigCancelled";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Multisigs(
                ::subxt::sp_core::crypto::AccountId32,
                [::core::primitive::u8; 32usize],
            );
            impl ::subxt::StorageEntry for Multisigs {
                const PALLET: &'static str = "MultiSig";
                const STORAGE: &'static str = "Multisigs";
                type Value = runtime_types::pallet_multisig::Multisig<
                    ::core::primitive::u32,
                    ::core::primitive::u64,
                    ::subxt::sp_core::crypto::AccountId32,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Twox64Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct Calls(pub [::core::primitive::u8; 32usize]);
            impl ::subxt::StorageEntry for Calls {
                const PALLET: &'static str = "MultiSig";
                const STORAGE: &'static str = "Calls";
                type Value = (
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u64,
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
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    _1: [::core::primitive::u8; 32usize],
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_multisig::Multisig<
                            ::core::primitive::u32,
                            ::core::primitive::u64,
                            ::subxt::sp_core::crypto::AccountId32,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Multisigs(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn multisigs_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Multisigs>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn calls(
                    &self,
                    _0: [::core::primitive::u8; 32usize],
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<(
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u64,
                    )>,
                    ::subxt::Error,
                > {
                    let entry = Calls(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn calls_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Calls>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod runtime_types {
        use super::runtime_types;
        pub mod cumulus_pallet_dmp_queue {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    #[codec(index = 0)]
                    service_overweight {
                        index: ::core::primitive::u64,
                        weight_limit: ::core::primitive::u64,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {
                    #[codec(index = 0)]
                    Unknown,
                    #[codec(index = 1)]
                    OverLimit,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    #[codec(index = 0)]
                    InvalidFormat([::core::primitive::u8; 32usize]),
                    #[codec(index = 1)]
                    UnsupportedVersion([::core::primitive::u8; 32usize]),
                    #[codec(index = 2)]
                    ExecutedDownward(
                        [::core::primitive::u8; 32usize],
                        runtime_types::xcm::v2::traits::Outcome,
                    ),
                    #[codec(index = 3)]
                    WeightExhausted(
                        [::core::primitive::u8; 32usize],
                        ::core::primitive::u64,
                        ::core::primitive::u64,
                    ),
                    #[codec(index = 4)]
                    OverweightEnqueued(
                        [::core::primitive::u8; 32usize],
                        ::core::primitive::u64,
                        ::core::primitive::u64,
                    ),
                    #[codec(index = 5)]
                    OverweightServiced(::core::primitive::u64, ::core::primitive::u64),
                }
            }
            #[derive(
                :: subxt :: codec :: CompactAs,
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
            )]
            pub struct ConfigData {
                pub max_individual: ::core::primitive::u64,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct PageIndexData {
                pub begin_used: ::core::primitive::u32,
                pub end_used: ::core::primitive::u32,
                pub overweight_count: ::core::primitive::u64,
            }
        }
        pub mod cumulus_pallet_parachain_system {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    # [codec (index = 0)] set_upgrade_block { relay_chain_block : :: core :: primitive :: u32 , } , # [codec (index = 1)] set_validation_data { data : runtime_types :: cumulus_primitives_parachain_inherent :: ParachainInherentData , } , # [codec (index = 2)] sudo_send_upward_message { message : :: std :: vec :: Vec < :: core :: primitive :: u8 > , } , # [codec (index = 3)] authorize_upgrade { code_hash : :: subxt :: sp_core :: H256 , } , # [codec (index = 4)] enact_authorized_upgrade { code : :: std :: vec :: Vec < :: core :: primitive :: u8 > , } , }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {
                    #[codec(index = 0)]
                    OverlappingUpgrades,
                    #[codec(index = 1)]
                    ProhibitedByPolkadot,
                    #[codec(index = 2)]
                    TooBig,
                    #[codec(index = 3)]
                    ValidationDataNotAvailable,
                    #[codec(index = 4)]
                    HostConfigurationNotAvailable,
                    #[codec(index = 5)]
                    NotScheduled,
                    #[codec(index = 6)]
                    NothingAuthorized,
                    #[codec(index = 7)]
                    Unauthorized,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    #[codec(index = 0)]
                    ValidationFunctionStored(::core::primitive::u32),
                    #[codec(index = 1)]
                    ValidationFunctionApplied(::core::primitive::u32),
                    #[codec(index = 2)]
                    UpgradeAuthorized(::subxt::sp_core::H256),
                    #[codec(index = 3)]
                    DownwardMessagesReceived(::core::primitive::u32),
                    #[codec(index = 4)]
                    DownwardMessagesProcessed(::core::primitive::u64, ::subxt::sp_core::H256),
                }
            }
            pub mod relay_state_snapshot {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct MessagingStateSnapshot {
                    pub dmq_mqc_head: ::subxt::sp_core::H256,
                    pub relay_dispatch_queue_size:
                        (::core::primitive::u32, ::core::primitive::u32),
                    pub ingress_channels: ::std::vec::Vec<(
                        runtime_types::polkadot_parachain::primitives::Id,
                        runtime_types::polkadot_primitives::v1::AbridgedHrmpChannel,
                    )>,
                    pub egress_channels: ::std::vec::Vec<(
                        runtime_types::polkadot_parachain::primitives::Id,
                        runtime_types::polkadot_primitives::v1::AbridgedHrmpChannel,
                    )>,
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct MessageQueueChain(pub ::subxt::sp_core::H256);
        }
        pub mod cumulus_pallet_xcm {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {}
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {}
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    #[codec(index = 0)]
                    InvalidFormat([::core::primitive::u8; 8usize]),
                    #[codec(index = 1)]
                    UnsupportedVersion([::core::primitive::u8; 8usize]),
                    #[codec(index = 2)]
                    ExecutedDownward(
                        [::core::primitive::u8; 8usize],
                        runtime_types::xcm::v2::traits::Outcome,
                    ),
                }
            }
        }
        pub mod cumulus_pallet_xcmp_queue {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {}
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {
                    #[codec(index = 0)]
                    FailedToSend,
                    #[codec(index = 1)]
                    BadXcmOrigin,
                    #[codec(index = 2)]
                    BadXcm,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    #[codec(index = 0)]
                    Success(::core::option::Option<::subxt::sp_core::H256>),
                    #[codec(index = 1)]
                    Fail(
                        ::core::option::Option<::subxt::sp_core::H256>,
                        runtime_types::xcm::v2::traits::Error,
                    ),
                    #[codec(index = 2)]
                    BadVersion(::core::option::Option<::subxt::sp_core::H256>),
                    #[codec(index = 3)]
                    BadFormat(::core::option::Option<::subxt::sp_core::H256>),
                    #[codec(index = 4)]
                    UpwardMessageSent(::core::option::Option<::subxt::sp_core::H256>),
                    #[codec(index = 5)]
                    XcmpMessageSent(::core::option::Option<::subxt::sp_core::H256>),
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum InboundStatus {
                #[codec(index = 0)]
                Ok,
                #[codec(index = 1)]
                Suspended,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum OutboundStatus {
                #[codec(index = 0)]
                Ok,
                #[codec(index = 1)]
                Suspended,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct QueueConfigData {
                pub suspend_threshold: ::core::primitive::u32,
                pub drop_threshold: ::core::primitive::u32,
                pub resume_threshold: ::core::primitive::u32,
                pub threshold_weight: ::core::primitive::u64,
                pub weight_restrict_decay: ::core::primitive::u64,
            }
        }
        pub mod cumulus_primitives_parachain_inherent {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ParachainInherentData {
                pub validation_data:
                    runtime_types::polkadot_primitives::v1::PersistedValidationData<
                        ::subxt::sp_core::H256,
                        ::core::primitive::u32,
                    >,
                pub relay_chain_state: runtime_types::sp_trie::storage_proof::StorageProof,
                pub downward_messages: ::std::vec::Vec<
                    runtime_types::polkadot_core_primitives::InboundDownwardMessage<
                        ::core::primitive::u32,
                    >,
                >,
                pub horizontal_messages: ::std::collections::BTreeMap<
                    runtime_types::polkadot_parachain::primitives::Id,
                    ::std::vec::Vec<
                        runtime_types::polkadot_core_primitives::InboundHrmpMessage<
                            ::core::primitive::u32,
                        >,
                    >,
                >,
            }
        }
        pub mod frame_support {
            use super::runtime_types;
            pub mod storage {
                use super::runtime_types;
                pub mod bounded_vec {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub struct BoundedVec<_0>(pub ::std::vec::Vec<_0>);
                }
                pub mod weak_bounded_vec {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub struct WeakBoundedVec<_0>(pub ::std::vec::Vec<_0>);
                }
            }
            pub mod traits {
                use super::runtime_types;
                pub mod tokens {
                    use super::runtime_types;
                    pub mod misc {
                        use super::runtime_types;
                        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
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
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct PerDispatchClass<_0> {
                    pub normal: _0,
                    pub operational: _0,
                    pub mandatory: _0,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct RuntimeDbWeight {
                    pub read: ::core::primitive::u64,
                    pub write: ::core::primitive::u64,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct WeightToFeeCoefficient<_0> {
                    pub coeff_integer: _0,
                    pub coeff_frac: runtime_types::sp_arithmetic::per_things::Perbill,
                    pub negative: ::core::primitive::bool,
                    pub degree: ::core::primitive::u8,
                }
            }
        }
        pub mod frame_system {
            use super::runtime_types;
            pub mod extensions {
                use super::runtime_types;
                pub mod check_genesis {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub struct CheckGenesis {}
                }
                pub mod check_mortality {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub struct CheckMortality(pub runtime_types::sp_runtime::generic::era::Era);
                }
                pub mod check_nonce {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub struct CheckNonce(#[codec(compact)] pub ::core::primitive::u32);
                }
                pub mod check_spec_version {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub struct CheckSpecVersion {}
                }
                pub mod check_tx_version {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub struct CheckTxVersion {}
                }
                pub mod check_weight {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub struct CheckWeight {}
                }
            }
            pub mod limits {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct BlockLength {
                    pub max: runtime_types::frame_support::weights::PerDispatchClass<
                        ::core::primitive::u32,
                    >,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct BlockWeights {
                    pub base_block: ::core::primitive::u64,
                    pub max_block: ::core::primitive::u64,
                    pub per_class: runtime_types::frame_support::weights::PerDispatchClass<
                        runtime_types::frame_system::limits::WeightsPerClass,
                    >,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct WeightsPerClass {
                    pub base_extrinsic: ::core::primitive::u64,
                    pub max_extrinsic: ::core::option::Option<::core::primitive::u64>,
                    pub max_total: ::core::option::Option<::core::primitive::u64>,
                    pub reserved: ::core::option::Option<::core::primitive::u64>,
                }
            }
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
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
                    set_changes_trie_config {
                        changes_trie_config: ::core::option::Option<
                            runtime_types::sp_core::changes_trie::ChangesTrieConfiguration,
                        >,
                    },
                    #[codec(index = 6)]
                    set_storage {
                        items: ::std::vec::Vec<(
                            ::std::vec::Vec<::core::primitive::u8>,
                            ::std::vec::Vec<::core::primitive::u8>,
                        )>,
                    },
                    #[codec(index = 7)]
                    kill_storage {
                        keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                    },
                    #[codec(index = 8)]
                    kill_prefix {
                        prefix: ::std::vec::Vec<::core::primitive::u8>,
                        subkeys: ::core::primitive::u32,
                    },
                    #[codec(index = 9)]
                    remark_with_event {
                        remark: ::std::vec::Vec<::core::primitive::u8>,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
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
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    #[codec(index = 0)]
                    ExtrinsicSuccess(runtime_types::frame_support::weights::DispatchInfo),
                    #[codec(index = 1)]
                    ExtrinsicFailed(
                        runtime_types::sp_runtime::DispatchError,
                        runtime_types::frame_support::weights::DispatchInfo,
                    ),
                    #[codec(index = 2)]
                    CodeUpdated,
                    #[codec(index = 3)]
                    NewAccount(::subxt::sp_core::crypto::AccountId32),
                    #[codec(index = 4)]
                    KilledAccount(::subxt::sp_core::crypto::AccountId32),
                    #[codec(index = 5)]
                    Remarked(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::H256,
                    ),
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct AccountInfo<_0, _1> {
                pub nonce: _0,
                pub consumers: _0,
                pub providers: _0,
                pub sufficients: _0,
                pub data: _1,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct EventRecord<_0, _1> {
                pub phase: runtime_types::frame_system::Phase,
                pub event: _0,
                pub topics: ::std::vec::Vec<_1>,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct LastRuntimeUpgradeInfo {
                #[codec(compact)]
                pub spec_version: ::core::primitive::u32,
                pub spec_name: ::std::string::String,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum Phase {
                #[codec(index = 0)]
                ApplyExtrinsic(::core::primitive::u32),
                #[codec(index = 1)]
                Finalization,
                #[codec(index = 2)]
                Initialization,
            }
        }
        pub mod nimbus_primitives {
            use super::runtime_types;
            pub mod nimbus_crypto {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct Public(pub runtime_types::sp_core::sr25519::Public);
            }
        }
        pub mod pallet_author_inherent {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    #[codec(index = 0)]
                    set_author {
                        author: runtime_types::nimbus_primitives::nimbus_crypto::Public,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {
                    #[codec(index = 0)]
                    AuthorAlreadySet,
                    #[codec(index = 1)]
                    NoAccountId,
                    #[codec(index = 2)]
                    CannotBeAuthor,
                }
            }
        }
        pub mod pallet_author_mapping {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    #[codec(index = 0)]
                    add_association {
                        author_id: runtime_types::nimbus_primitives::nimbus_crypto::Public,
                    },
                    #[codec(index = 1)]
                    update_association {
                        old_author_id: runtime_types::nimbus_primitives::nimbus_crypto::Public,
                        new_author_id: runtime_types::nimbus_primitives::nimbus_crypto::Public,
                    },
                    #[codec(index = 2)]
                    clear_association {
                        author_id: runtime_types::nimbus_primitives::nimbus_crypto::Public,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {
                    #[codec(index = 0)]
                    AssociationNotFound,
                    #[codec(index = 1)]
                    NotYourAssociation,
                    #[codec(index = 2)]
                    CannotAffordSecurityDeposit,
                    #[codec(index = 3)]
                    AlreadyAssociated,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    #[codec(index = 0)]
                    AuthorRegistered(
                        runtime_types::nimbus_primitives::nimbus_crypto::Public,
                        ::subxt::sp_core::crypto::AccountId32,
                    ),
                    #[codec(index = 1)]
                    AuthorDeRegistered(runtime_types::nimbus_primitives::nimbus_crypto::Public),
                    #[codec(index = 2)]
                    AuthorRotated(
                        runtime_types::nimbus_primitives::nimbus_crypto::Public,
                        ::subxt::sp_core::crypto::AccountId32,
                    ),
                    #[codec(index = 3)]
                    DefunctAuthorBusted(
                        runtime_types::nimbus_primitives::nimbus_crypto::Public,
                        ::subxt::sp_core::crypto::AccountId32,
                    ),
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct RegistrationInfo<_0, _1> {
                    pub account: _0,
                    pub deposit: _1,
                }
            }
        }
        pub mod pallet_author_slot_filter {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    #[codec(index = 0)]
                    set_eligible {
                        new: runtime_types::sp_arithmetic::per_things::Percent,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    #[codec(index = 0)]
                    EligibleUpdated(runtime_types::sp_arithmetic::per_things::Percent),
                }
            }
        }
        pub mod pallet_balances {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    #[codec(index = 0)]
                    transfer {
                        dest: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        value: ::core::primitive::u64,
                    },
                    #[codec(index = 1)]
                    set_balance {
                        who: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        new_free: ::core::primitive::u64,
                        #[codec(compact)]
                        new_reserved: ::core::primitive::u64,
                    },
                    #[codec(index = 2)]
                    force_transfer {
                        source: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        dest: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        value: ::core::primitive::u64,
                    },
                    #[codec(index = 3)]
                    transfer_keep_alive {
                        dest: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        value: ::core::primitive::u64,
                    },
                    #[codec(index = 4)]
                    transfer_all {
                        dest: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        keep_alive: ::core::primitive::bool,
                    },
                    #[codec(index = 5)]
                    force_unreserve {
                        who: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        amount: ::core::primitive::u64,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
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
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    #[codec(index = 0)]
                    Endowed(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u64,
                    ),
                    #[codec(index = 1)]
                    DustLost(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u64,
                    ),
                    #[codec(index = 2)]
                    Transfer(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u64,
                    ),
                    #[codec(index = 3)]
                    BalanceSet(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u64,
                        ::core::primitive::u64,
                    ),
                    #[codec(index = 4)]
                    Deposit(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u64,
                    ),
                    #[codec(index = 5)]
                    Reserved(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u64,
                    ),
                    #[codec(index = 6)]
                    Unreserved(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u64,
                    ),
                    #[codec(index = 7)]
                    ReserveRepatriated(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u64,
                        runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
                    ),
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct AccountData<_0> {
                pub free: _0,
                pub reserved: _0,
                pub misc_frozen: _0,
                pub fee_frozen: _0,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct BalanceLock<_0> {
                pub id: [::core::primitive::u8; 8usize],
                pub amount: _0,
                pub reasons: runtime_types::pallet_balances::Reasons,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum Reasons {
                #[codec(index = 0)]
                Fee,
                #[codec(index = 1)]
                Misc,
                #[codec(index = 2)]
                All,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum Releases {
                #[codec(index = 0)]
                V1_0_0,
                #[codec(index = 1)]
                V2_0_0,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ReserveData<_0, _1> {
                pub id: _0,
                pub amount: _1,
            }
        }
        pub mod pallet_multisig {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    #[codec(index = 0)]
                    as_multi_threshold_1 {
                        other_signatories: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                        call: ::std::boxed::Box<runtime_types::pontem_runtime::Call>,
                    },
                    #[codec(index = 1)]
                    as_multi {
                        threshold: ::core::primitive::u16,
                        other_signatories: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                        maybe_timepoint: ::core::option::Option<
                            runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                        >,
                        call: ::std::vec::Vec<::core::primitive::u8>,
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
                        timepoint:
                            runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                        call_hash: [::core::primitive::u8; 32usize],
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
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
                    WeightTooLow,
                    #[codec(index = 13)]
                    AlreadyStored,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    #[codec(index = 0)]
                    NewMultisig(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                        [::core::primitive::u8; 32usize],
                    ),
                    #[codec(index = 1)]
                    MultisigApproval(
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                        ::subxt::sp_core::crypto::AccountId32,
                        [::core::primitive::u8; 32usize],
                    ),
                    #[codec(index = 2)]
                    MultisigExecuted(
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                        ::subxt::sp_core::crypto::AccountId32,
                        [::core::primitive::u8; 32usize],
                        ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    ),
                    #[codec(index = 3)]
                    MultisigCancelled(
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                        ::subxt::sp_core::crypto::AccountId32,
                        [::core::primitive::u8; 32usize],
                    ),
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Multisig<_0, _1, _2> {
                pub when: runtime_types::pallet_multisig::Timepoint<_0>,
                pub deposit: _1,
                pub depositor: _2,
                pub approvals: ::std::vec::Vec<_2>,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Timepoint<_0> {
                pub height: _0,
                pub index: _0,
            }
        }
        pub mod pallet_sudo {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    #[codec(index = 0)]
                    sudo {
                        call: ::std::boxed::Box<runtime_types::pontem_runtime::Call>,
                    },
                    #[codec(index = 1)]
                    sudo_unchecked_weight {
                        call: ::std::boxed::Box<runtime_types::pontem_runtime::Call>,
                        weight: ::core::primitive::u64,
                    },
                    #[codec(index = 2)]
                    set_key {
                        new: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 3)]
                    sudo_as {
                        who: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        call: ::std::boxed::Box<runtime_types::pontem_runtime::Call>,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {
                    #[codec(index = 0)]
                    RequireSudo,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    #[codec(index = 0)]
                    Sudid(::core::result::Result<(), runtime_types::sp_runtime::DispatchError>),
                    #[codec(index = 1)]
                    KeyChanged(::subxt::sp_core::crypto::AccountId32),
                    #[codec(index = 2)]
                    SudoAsDone(
                        ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    ),
                }
            }
        }
        pub mod pallet_timestamp {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
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
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ChargeTransactionPayment(#[codec(compact)] pub ::core::primitive::u64);
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum Releases {
                #[codec(index = 0)]
                V1Ancient,
                #[codec(index = 1)]
                V2,
            }
        }
        pub mod pallet_vesting {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    #[codec(index = 0)]
                    vest,
                    #[codec(index = 1)]
                    vest_other {
                        target: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 2)]
                    vested_transfer {
                        target: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        schedule: runtime_types::pallet_vesting::vesting_info::VestingInfo<
                            ::core::primitive::u64,
                            ::core::primitive::u32,
                        >,
                    },
                    #[codec(index = 3)]
                    force_vested_transfer {
                        source: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        target: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        schedule: runtime_types::pallet_vesting::vesting_info::VestingInfo<
                            ::core::primitive::u64,
                            ::core::primitive::u32,
                        >,
                    },
                    #[codec(index = 4)]
                    merge_schedules {
                        schedule1_index: ::core::primitive::u32,
                        schedule2_index: ::core::primitive::u32,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {
                    #[codec(index = 0)]
                    NotVesting,
                    #[codec(index = 1)]
                    AtMaxVestingSchedules,
                    #[codec(index = 2)]
                    AmountLow,
                    #[codec(index = 3)]
                    ScheduleIndexOutOfBounds,
                    #[codec(index = 4)]
                    InvalidScheduleParams,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    #[codec(index = 0)]
                    VestingUpdated(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u64,
                    ),
                    #[codec(index = 1)]
                    VestingCompleted(::subxt::sp_core::crypto::AccountId32),
                }
            }
            pub mod vesting_info {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct VestingInfo<_0, _1> {
                    pub locked: _0,
                    pub per_block: _0,
                    pub starting_block: _1,
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum Releases {
                #[codec(index = 0)]
                V0,
                #[codec(index = 1)]
                V1,
            }
        }
        pub mod pallet_xcm {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    #[codec(index = 0)]
                    send {
                        dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                        message: ::std::boxed::Box<runtime_types::xcm::VersionedXcm>,
                    },
                    #[codec(index = 1)]
                    teleport_assets {
                        dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                        beneficiary:
                            ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                        assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
                        fee_asset_item: ::core::primitive::u32,
                    },
                    #[codec(index = 2)]
                    reserve_transfer_assets {
                        dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                        beneficiary:
                            ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                        assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
                        fee_asset_item: ::core::primitive::u32,
                    },
                    #[codec(index = 3)]
                    execute {
                        message: ::std::boxed::Box<runtime_types::xcm::VersionedXcm>,
                        max_weight: ::core::primitive::u64,
                    },
                    #[codec(index = 4)]
                    force_xcm_version {
                        location: ::std::boxed::Box<
                            runtime_types::xcm::v1::multilocation::MultiLocation,
                        >,
                        xcm_version: ::core::primitive::u32,
                    },
                    #[codec(index = 5)]
                    force_default_xcm_version {
                        maybe_xcm_version: ::core::option::Option<::core::primitive::u32>,
                    },
                    #[codec(index = 6)]
                    force_subscribe_version_notify {
                        location: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                    },
                    #[codec(index = 7)]
                    force_unsubscribe_version_notify {
                        location: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {
                    #[codec(index = 0)]
                    Unreachable,
                    #[codec(index = 1)]
                    SendFailure,
                    #[codec(index = 2)]
                    Filtered,
                    #[codec(index = 3)]
                    UnweighableMessage,
                    #[codec(index = 4)]
                    DestinationNotInvertible,
                    #[codec(index = 5)]
                    Empty,
                    #[codec(index = 6)]
                    CannotReanchor,
                    #[codec(index = 7)]
                    TooManyAssets,
                    #[codec(index = 8)]
                    InvalidOrigin,
                    #[codec(index = 9)]
                    BadVersion,
                    #[codec(index = 10)]
                    BadLocation,
                    #[codec(index = 11)]
                    NoSubscription,
                    #[codec(index = 12)]
                    AlreadySubscribed,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    #[codec(index = 0)]
                    Attempted(runtime_types::xcm::v2::traits::Outcome),
                    #[codec(index = 1)]
                    Sent(
                        runtime_types::xcm::v1::multilocation::MultiLocation,
                        runtime_types::xcm::v1::multilocation::MultiLocation,
                        runtime_types::xcm::v2::Xcm,
                    ),
                    #[codec(index = 2)]
                    UnexpectedResponse(
                        runtime_types::xcm::v1::multilocation::MultiLocation,
                        ::core::primitive::u64,
                    ),
                    #[codec(index = 3)]
                    ResponseReady(::core::primitive::u64, runtime_types::xcm::v2::Response),
                    #[codec(index = 4)]
                    Notified(
                        ::core::primitive::u64,
                        ::core::primitive::u8,
                        ::core::primitive::u8,
                    ),
                    #[codec(index = 5)]
                    NotifyOverweight(
                        ::core::primitive::u64,
                        ::core::primitive::u8,
                        ::core::primitive::u8,
                        ::core::primitive::u64,
                        ::core::primitive::u64,
                    ),
                    #[codec(index = 6)]
                    NotifyDispatchError(
                        ::core::primitive::u64,
                        ::core::primitive::u8,
                        ::core::primitive::u8,
                    ),
                    #[codec(index = 7)]
                    NotifyDecodeFailed(
                        ::core::primitive::u64,
                        ::core::primitive::u8,
                        ::core::primitive::u8,
                    ),
                    #[codec(index = 8)]
                    InvalidResponder(
                        runtime_types::xcm::v1::multilocation::MultiLocation,
                        ::core::primitive::u64,
                        ::core::option::Option<
                            runtime_types::xcm::v1::multilocation::MultiLocation,
                        >,
                    ),
                    #[codec(index = 9)]
                    InvalidResponderVersion(
                        runtime_types::xcm::v1::multilocation::MultiLocation,
                        ::core::primitive::u64,
                    ),
                    #[codec(index = 10)]
                    ResponseTaken(::core::primitive::u64),
                    #[codec(index = 11)]
                    AssetsTrapped(
                        ::subxt::sp_core::H256,
                        runtime_types::xcm::v1::multilocation::MultiLocation,
                        runtime_types::xcm::VersionedMultiAssets,
                    ),
                    #[codec(index = 12)]
                    VersionChangeNotified(
                        runtime_types::xcm::v1::multilocation::MultiLocation,
                        ::core::primitive::u32,
                    ),
                    #[codec(index = 13)]
                    SupportedVersionChanged(
                        runtime_types::xcm::v1::multilocation::MultiLocation,
                        ::core::primitive::u32,
                    ),
                    #[codec(index = 14)]
                    NotifyTargetSendFail(
                        runtime_types::xcm::v1::multilocation::MultiLocation,
                        ::core::primitive::u64,
                        runtime_types::xcm::v2::traits::Error,
                    ),
                    #[codec(index = 15)]
                    NotifyTargetMigrationFail(
                        runtime_types::xcm::VersionedMultiLocation,
                        ::core::primitive::u64,
                    ),
                }
            }
        }
        pub mod parachain_staking {
            use super::runtime_types;
            pub mod inflation {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct InflationInfo<_0> {
                    pub expect: runtime_types::parachain_staking::inflation::Range<_0>,
                    pub annual: runtime_types::parachain_staking::inflation::Range<
                        runtime_types::sp_arithmetic::per_things::Perbill,
                    >,
                    pub round: runtime_types::parachain_staking::inflation::Range<
                        runtime_types::sp_arithmetic::per_things::Perbill,
                    >,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct Range<_0> {
                    pub min: _0,
                    pub ideal: _0,
                    pub max: _0,
                }
            }
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct Bond<_0, _1> {
                    pub owner: _0,
                    pub amount: _1,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    #[codec(index = 0)]
                    set_staking_expectations {
                        expectations: runtime_types::parachain_staking::inflation::Range<
                            ::core::primitive::u64,
                        >,
                    },
                    #[codec(index = 1)]
                    set_inflation {
                        schedule: runtime_types::parachain_staking::inflation::Range<
                            runtime_types::sp_arithmetic::per_things::Perbill,
                        >,
                    },
                    #[codec(index = 2)]
                    set_parachain_bond_account {
                        new: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 3)]
                    set_parachain_bond_reserve_percent {
                        new: runtime_types::sp_arithmetic::per_things::Percent,
                    },
                    #[codec(index = 4)]
                    set_total_selected { new: ::core::primitive::u32 },
                    #[codec(index = 5)]
                    set_collator_commission {
                        new: runtime_types::sp_arithmetic::per_things::Perbill,
                    },
                    #[codec(index = 6)]
                    set_blocks_per_round { new: ::core::primitive::u32 },
                    #[codec(index = 7)]
                    join_candidates {
                        bond: ::core::primitive::u64,
                        candidate_count: ::core::primitive::u32,
                    },
                    #[codec(index = 8)]
                    leave_candidates {
                        candidate_count: ::core::primitive::u32,
                    },
                    #[codec(index = 9)]
                    go_offline,
                    #[codec(index = 10)]
                    go_online,
                    #[codec(index = 11)]
                    candidate_bond_more { more: ::core::primitive::u64 },
                    #[codec(index = 12)]
                    candidate_bond_less { less: ::core::primitive::u64 },
                    #[codec(index = 13)]
                    nominate {
                        collator: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u64,
                        collator_nominator_count: ::core::primitive::u32,
                        nomination_count: ::core::primitive::u32,
                    },
                    #[codec(index = 14)]
                    leave_nominators {
                        nomination_count: ::core::primitive::u32,
                    },
                    #[codec(index = 15)]
                    revoke_nomination {
                        collator: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 16)]
                    nominator_bond_more {
                        candidate: ::subxt::sp_core::crypto::AccountId32,
                        more: ::core::primitive::u64,
                    },
                    #[codec(index = 17)]
                    nominator_bond_less {
                        candidate: ::subxt::sp_core::crypto::AccountId32,
                        less: ::core::primitive::u64,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct Collator2<_0, _1> {
                    pub id: _0,
                    pub bond: _1,
                    pub nominators: runtime_types::parachain_staking::set::OrderedSet<_0>,
                    pub top_nominators:
                        ::std::vec::Vec<runtime_types::parachain_staking::pallet::Bond<_0, _1>>,
                    pub bottom_nominators:
                        ::std::vec::Vec<runtime_types::parachain_staking::pallet::Bond<_0, _1>>,
                    pub total_counted: _1,
                    pub total_backing: _1,
                    pub state: runtime_types::parachain_staking::pallet::CollatorStatus,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct CollatorSnapshot<_0, _1> {
                    pub bond: _1,
                    pub nominators:
                        ::std::vec::Vec<runtime_types::parachain_staking::pallet::Bond<_0, _1>>,
                    pub total: _1,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum CollatorStatus {
                    #[codec(index = 0)]
                    Active,
                    #[codec(index = 1)]
                    Idle,
                    #[codec(index = 2)]
                    Leaving(::core::primitive::u32),
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {
                    #[codec(index = 0)]
                    NominatorDNE,
                    #[codec(index = 1)]
                    CandidateDNE,
                    #[codec(index = 2)]
                    NominationDNE,
                    #[codec(index = 3)]
                    NominatorExists,
                    #[codec(index = 4)]
                    CandidateExists,
                    #[codec(index = 5)]
                    ValBondBelowMin,
                    #[codec(index = 6)]
                    NomBondBelowMin,
                    #[codec(index = 7)]
                    NominationBelowMin,
                    #[codec(index = 8)]
                    AlreadyOffline,
                    #[codec(index = 9)]
                    AlreadyActive,
                    #[codec(index = 10)]
                    NominatorAlreadyLeaving,
                    #[codec(index = 11)]
                    NominationAlreadyRevoked,
                    #[codec(index = 12)]
                    CandidateAlreadyLeaving,
                    #[codec(index = 13)]
                    CannotActBecauseLeaving,
                    #[codec(index = 14)]
                    CannotActBecauseRevoking,
                    #[codec(index = 15)]
                    ExceedMaxCollatorsPerNom,
                    #[codec(index = 16)]
                    AlreadyNominatedCollator,
                    #[codec(index = 17)]
                    InvalidSchedule,
                    #[codec(index = 18)]
                    CannotSetBelowMin,
                    #[codec(index = 19)]
                    NoWritingSameValue,
                    #[codec(index = 20)]
                    TooLowCandidateCountWeightHintJoinCandidates,
                    #[codec(index = 21)]
                    TooLowCollatorCandidateCountToLeaveCandidates,
                    #[codec(index = 22)]
                    TooLowNominationCountToNominate,
                    #[codec(index = 23)]
                    TooLowCollatorNominationCountToNominate,
                    #[codec(index = 24)]
                    TooLowNominationCountToLeaveNominators,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    #[codec(index = 0)]
                    NewRound(
                        ::core::primitive::u32,
                        ::core::primitive::u32,
                        ::core::primitive::u32,
                        ::core::primitive::u64,
                    ),
                    #[codec(index = 1)]
                    JoinedCollatorCandidates(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u64,
                        ::core::primitive::u64,
                    ),
                    #[codec(index = 2)]
                    CollatorChosen(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u64,
                    ),
                    #[codec(index = 3)]
                    CollatorBondedMore(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u64,
                        ::core::primitive::u64,
                    ),
                    #[codec(index = 4)]
                    CollatorBondedLess(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u64,
                        ::core::primitive::u64,
                    ),
                    #[codec(index = 5)]
                    CollatorWentOffline(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                    ),
                    #[codec(index = 6)]
                    CollatorBackOnline(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                    ),
                    #[codec(index = 7)]
                    CollatorScheduledExit(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    ),
                    #[codec(index = 8)]
                    CollatorLeft(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u64,
                        ::core::primitive::u64,
                    ),
                    #[codec(index = 9)]
                    NominationIncreased(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u64,
                        ::core::primitive::bool,
                        ::core::primitive::u64,
                    ),
                    #[codec(index = 10)]
                    NominationDecreased(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u64,
                        ::core::primitive::bool,
                        ::core::primitive::u64,
                    ),
                    #[codec(index = 11)]
                    NominatorExitScheduled(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    ),
                    #[codec(index = 12)]
                    NominationRevocationScheduled(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u32,
                    ),
                    #[codec(index = 13)]
                    NominatorLeft(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u64,
                    ),
                    #[codec(index = 14)]
                    Nomination(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u64,
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::parachain_staking::pallet::NominatorAdded<
                            ::core::primitive::u64,
                        >,
                    ),
                    #[codec(index = 15)]
                    NominatorLeftCollator(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u64,
                        ::core::primitive::u64,
                    ),
                    #[codec(index = 16)]
                    Rewarded(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u64,
                    ),
                    #[codec(index = 17)]
                    ReservedForParachainBond(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u64,
                    ),
                    #[codec(index = 18)]
                    ParachainBondAccountSet(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                    ),
                    #[codec(index = 19)]
                    ParachainBondReservePercentSet(
                        runtime_types::sp_arithmetic::per_things::Percent,
                        runtime_types::sp_arithmetic::per_things::Percent,
                    ),
                    #[codec(index = 20)]
                    InflationSet(
                        runtime_types::sp_arithmetic::per_things::Perbill,
                        runtime_types::sp_arithmetic::per_things::Perbill,
                        runtime_types::sp_arithmetic::per_things::Perbill,
                        runtime_types::sp_arithmetic::per_things::Perbill,
                        runtime_types::sp_arithmetic::per_things::Perbill,
                        runtime_types::sp_arithmetic::per_things::Perbill,
                    ),
                    #[codec(index = 21)]
                    StakeExpectationsSet(
                        ::core::primitive::u64,
                        ::core::primitive::u64,
                        ::core::primitive::u64,
                    ),
                    #[codec(index = 22)]
                    TotalSelectedSet(::core::primitive::u32, ::core::primitive::u32),
                    #[codec(index = 23)]
                    CollatorCommissionSet(
                        runtime_types::sp_arithmetic::per_things::Perbill,
                        runtime_types::sp_arithmetic::per_things::Perbill,
                    ),
                    #[codec(index = 24)]
                    BlocksPerRoundSet(
                        ::core::primitive::u32,
                        ::core::primitive::u32,
                        ::core::primitive::u32,
                        ::core::primitive::u32,
                        runtime_types::sp_arithmetic::per_things::Perbill,
                        runtime_types::sp_arithmetic::per_things::Perbill,
                        runtime_types::sp_arithmetic::per_things::Perbill,
                    ),
                    #[codec(index = 25)]
                    DelayNominationExitsMigrationExecuted,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct ExitQ<_0> {
                    pub candidates: runtime_types::parachain_staking::set::OrderedSet<_0>,
                    pub nominators_leaving: runtime_types::parachain_staking::set::OrderedSet<_0>,
                    pub candidate_schedule: ::std::vec::Vec<(_0, ::core::primitive::u32)>,
                    pub nominator_schedule:
                        ::std::vec::Vec<(_0, ::core::option::Option<_0>, ::core::primitive::u32)>,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct Nominator<_0, _1> {
                    pub nominations: runtime_types::parachain_staking::set::OrderedSet<
                        runtime_types::parachain_staking::pallet::Bond<_0, _1>,
                    >,
                    pub total: _1,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct Nominator2<_0, _1> {
                    pub nominations: runtime_types::parachain_staking::set::OrderedSet<
                        runtime_types::parachain_staking::pallet::Bond<_0, _1>,
                    >,
                    pub revocations: runtime_types::parachain_staking::set::OrderedSet<_0>,
                    pub total: _1,
                    pub scheduled_revocations_count: ::core::primitive::u32,
                    pub scheduled_revocations_total: _1,
                    pub status: runtime_types::parachain_staking::pallet::NominatorStatus,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum NominatorAdded<_0> {
                    #[codec(index = 0)]
                    AddedToTop { new_total: _0 },
                    #[codec(index = 1)]
                    AddedToBottom,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum NominatorStatus {
                    #[codec(index = 0)]
                    Active,
                    #[codec(index = 1)]
                    Leaving(::core::primitive::u32),
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct ParachainBondConfig<_0> {
                    pub account: _0,
                    pub percent: runtime_types::sp_arithmetic::per_things::Percent,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct RoundInfo<_0> {
                    pub current: _0,
                    pub first: _0,
                    pub length: _0,
                }
            }
            pub mod set {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct OrderedSet<_0>(pub ::std::vec::Vec<_0>);
            }
        }
        pub mod polkadot_core_primitives {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct InboundDownwardMessage<_0> {
                pub sent_at: _0,
                pub msg: ::std::vec::Vec<::core::primitive::u8>,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct InboundHrmpMessage<_0> {
                pub sent_at: _0,
                pub data: ::std::vec::Vec<::core::primitive::u8>,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct OutboundHrmpMessage<_0> {
                pub recipient: _0,
                pub data: ::std::vec::Vec<::core::primitive::u8>,
            }
        }
        pub mod polkadot_parachain {
            use super::runtime_types;
            pub mod primitives {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct HeadData(pub ::std::vec::Vec<::core::primitive::u8>);
                #[derive(
                    :: subxt :: codec :: CompactAs,
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                )]
                pub struct Id(pub ::core::primitive::u32);
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum XcmpMessageFormat {
                    #[codec(index = 0)]
                    ConcatenatedVersionedXcm,
                    #[codec(index = 1)]
                    ConcatenatedEncodedBlob,
                    #[codec(index = 2)]
                    Signals,
                }
            }
        }
        pub mod polkadot_primitives {
            use super::runtime_types;
            pub mod v1 {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct AbridgedHostConfiguration {
                    pub max_code_size: ::core::primitive::u32,
                    pub max_head_data_size: ::core::primitive::u32,
                    pub max_upward_queue_count: ::core::primitive::u32,
                    pub max_upward_queue_size: ::core::primitive::u32,
                    pub max_upward_message_size: ::core::primitive::u32,
                    pub max_upward_message_num_per_candidate: ::core::primitive::u32,
                    pub hrmp_max_message_num_per_candidate: ::core::primitive::u32,
                    pub validation_upgrade_frequency: ::core::primitive::u32,
                    pub validation_upgrade_delay: ::core::primitive::u32,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct AbridgedHrmpChannel {
                    pub max_capacity: ::core::primitive::u32,
                    pub max_total_size: ::core::primitive::u32,
                    pub max_message_size: ::core::primitive::u32,
                    pub msg_count: ::core::primitive::u32,
                    pub total_size: ::core::primitive::u32,
                    pub mqc_head: ::core::option::Option<::subxt::sp_core::H256>,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct PersistedValidationData<_0, _1> {
                    pub parent_head: runtime_types::polkadot_parachain::primitives::HeadData,
                    pub relay_parent_number: _1,
                    pub relay_parent_storage_root: _0,
                    pub max_pov_size: _1,
                }
            }
        }
        pub mod pontem_runtime {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum Call {
                #[codec(index = 0)]
                System(runtime_types::frame_system::pallet::Call),
                #[codec(index = 2)]
                Timestamp(runtime_types::pallet_timestamp::pallet::Call),
                #[codec(index = 4)]
                Sudo(runtime_types::pallet_sudo::pallet::Call),
                #[codec(index = 20)]
                ParachainSystem(runtime_types::cumulus_pallet_parachain_system::pallet::Call),
                #[codec(index = 30)]
                Balances(runtime_types::pallet_balances::pallet::Call),
                #[codec(index = 31)]
                Vesting(runtime_types::pallet_vesting::pallet::Call),
                #[codec(index = 40)]
                ParachainStaking(runtime_types::parachain_staking::pallet::Call),
                #[codec(index = 41)]
                AuthorInherent(runtime_types::pallet_author_inherent::pallet::Call),
                #[codec(index = 42)]
                AuthorFilter(runtime_types::pallet_author_slot_filter::pallet::Call),
                #[codec(index = 43)]
                AuthorMapping(runtime_types::pallet_author_mapping::pallet::Call),
                #[codec(index = 50)]
                XcmpQueue(runtime_types::cumulus_pallet_xcmp_queue::pallet::Call),
                #[codec(index = 51)]
                PolkadotXcm(runtime_types::pallet_xcm::pallet::Call),
                #[codec(index = 52)]
                CumulusXcm(runtime_types::cumulus_pallet_xcm::pallet::Call),
                #[codec(index = 53)]
                DmpQueue(runtime_types::cumulus_pallet_dmp_queue::pallet::Call),
                #[codec(index = 54)]
                Mvm(runtime_types::sp_mvm::pallet::Call),
                #[codec(index = 55)]
                MultiSig(runtime_types::pallet_multisig::pallet::Call),
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum Event {
                #[codec(index = 0)]
                System(runtime_types::frame_system::pallet::Event),
                #[codec(index = 4)]
                Sudo(runtime_types::pallet_sudo::pallet::Event),
                #[codec(index = 20)]
                ParachainSystem(runtime_types::cumulus_pallet_parachain_system::pallet::Event),
                #[codec(index = 30)]
                Balances(runtime_types::pallet_balances::pallet::Event),
                #[codec(index = 31)]
                Vesting(runtime_types::pallet_vesting::pallet::Event),
                #[codec(index = 40)]
                ParachainStaking(runtime_types::parachain_staking::pallet::Event),
                #[codec(index = 42)]
                AuthorFilter(runtime_types::pallet_author_slot_filter::pallet::Event),
                #[codec(index = 43)]
                AuthorMapping(runtime_types::pallet_author_mapping::pallet::Event),
                #[codec(index = 50)]
                XcmpQueue(runtime_types::cumulus_pallet_xcmp_queue::pallet::Event),
                #[codec(index = 51)]
                PolkadotXcm(runtime_types::pallet_xcm::pallet::Event),
                #[codec(index = 52)]
                CumulusXcm(runtime_types::cumulus_pallet_xcm::pallet::Event),
                #[codec(index = 53)]
                DmpQueue(runtime_types::cumulus_pallet_dmp_queue::pallet::Event),
                #[codec(index = 54)]
                Mvm(runtime_types::sp_mvm::pallet::Event),
                #[codec(index = 55)]
                MultiSig(runtime_types::pallet_multisig::pallet::Event),
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Runtime {}
        }
        pub mod primitive_types {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct H256(pub [::core::primitive::u8; 32usize]);
        }
        pub mod sp_arithmetic {
            use super::runtime_types;
            pub mod fixed_point {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: CompactAs,
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                )]
                pub struct FixedU128(pub ::core::primitive::u128);
            }
            pub mod per_things {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: CompactAs,
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                )]
                pub struct Perbill(pub ::core::primitive::u32);
                #[derive(
                    :: subxt :: codec :: CompactAs,
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                )]
                pub struct Percent(pub ::core::primitive::u8);
            }
        }
        pub mod sp_core {
            use super::runtime_types;
            pub mod changes_trie {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct ChangesTrieConfiguration {
                    pub digest_interval: ::core::primitive::u32,
                    pub digest_levels: ::core::primitive::u32,
                }
            }
            pub mod crypto {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct AccountId32(pub [::core::primitive::u8; 32usize]);
            }
            pub mod ecdsa {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct Signature(pub [::core::primitive::u8; 65usize]);
            }
            pub mod ed25519 {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct Signature(pub [::core::primitive::u8; 64usize]);
            }
            pub mod sr25519 {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct Public(pub [::core::primitive::u8; 32usize]);
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct Signature(pub [::core::primitive::u8; 64usize]);
            }
        }
        pub mod sp_mvm {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    #[codec(index = 0)]
                    execute {
                        tx_bc: ::std::vec::Vec<::core::primitive::u8>,
                        gas_limit: ::core::primitive::u64,
                    },
                    #[codec(index = 1)]
                    publish_module {
                        module_bc: ::std::vec::Vec<::core::primitive::u8>,
                        gas_limit: ::core::primitive::u64,
                    },
                    #[codec(index = 2)]
                    publish_package {
                        package: ::std::vec::Vec<::core::primitive::u8>,
                        gas_limit: ::core::primitive::u64,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {
                    #[codec(index = 0)]
                    NumConversionError,
                    #[codec(index = 1)]
                    InvalidVMConfig,
                    #[codec(index = 2)]
                    InvalidGasAmountMaxValue,
                    #[codec(index = 3)]
                    ScriptValidationError,
                    #[codec(index = 4)]
                    TransactionValidationError,
                    #[codec(index = 5)]
                    TransactionSignersNumError,
                    #[codec(index = 6)]
                    UnknownValidationStatus,
                    #[codec(index = 7)]
                    InvalidSignature,
                    #[codec(index = 8)]
                    InvalidAuthKey,
                    #[codec(index = 9)]
                    SequenceNumberTooOld,
                    #[codec(index = 10)]
                    SequenceNumberTooNew,
                    #[codec(index = 11)]
                    SequenceNumberTooBig,
                    #[codec(index = 12)]
                    InsufficientBalanceForTransactionFee,
                    #[codec(index = 13)]
                    TransactionExpired,
                    #[codec(index = 14)]
                    SendingAccountDoesNotExist,
                    #[codec(index = 15)]
                    RejectedWriteSet,
                    #[codec(index = 16)]
                    InvalidWriteSet,
                    #[codec(index = 17)]
                    ExceededMaxTransactionSize,
                    #[codec(index = 18)]
                    UnknownScript,
                    #[codec(index = 19)]
                    UnknownModule,
                    #[codec(index = 20)]
                    MaxGasUnitsExceedsMaxGasUnitsBound,
                    #[codec(index = 21)]
                    MaxGasUnitsBelowMinTransactionGasUnits,
                    #[codec(index = 22)]
                    GasUnitPriceBelowMinBound,
                    #[codec(index = 23)]
                    GasUnitPriceAboveMaxBound,
                    #[codec(index = 24)]
                    InvalidGasSpecifier,
                    #[codec(index = 25)]
                    SendingAccountFrozen,
                    #[codec(index = 26)]
                    UnableToDeserializeAccount,
                    #[codec(index = 27)]
                    CurrencyInfoDoesNotExist,
                    #[codec(index = 28)]
                    InvalidModulePublisher,
                    #[codec(index = 29)]
                    NoAccountRole,
                    #[codec(index = 30)]
                    BadChainId,
                    #[codec(index = 31)]
                    UnknownVerificationError,
                    #[codec(index = 32)]
                    IndexOutOfBounds,
                    #[codec(index = 33)]
                    InvalidSignatureToken,
                    #[codec(index = 34)]
                    RecursiveStructDefinition,
                    #[codec(index = 35)]
                    InvalidResourceField,
                    #[codec(index = 36)]
                    InvalidFallThrough,
                    #[codec(index = 37)]
                    NegativeStackSizeWithinBlock,
                    #[codec(index = 38)]
                    InvalidMainFunctionSignature,
                    #[codec(index = 39)]
                    DuplicateElement,
                    #[codec(index = 40)]
                    InvalidModuleHandle,
                    #[codec(index = 41)]
                    UnimplementedHandle,
                    #[codec(index = 42)]
                    LookupFailed,
                    #[codec(index = 43)]
                    TypeMismatch,
                    #[codec(index = 44)]
                    MissingDependency,
                    #[codec(index = 45)]
                    PopResourceError,
                    #[codec(index = 46)]
                    BrTypeMismatchError,
                    #[codec(index = 47)]
                    AbortTypeMismatchError,
                    #[codec(index = 48)]
                    StlocTypeMismatchError,
                    #[codec(index = 49)]
                    StlocUnsafeToDestroyError,
                    #[codec(index = 50)]
                    UnsafeRetLocalOrResourceStillBorrowed,
                    #[codec(index = 51)]
                    RetTypeMismatchError,
                    #[codec(index = 52)]
                    RetBorrowedMutableReferenceError,
                    #[codec(index = 53)]
                    FreezerefTypeMismatchError,
                    #[codec(index = 54)]
                    FreezerefExistsMutableBorrowError,
                    #[codec(index = 55)]
                    BorrowfieldTypeMismatchError,
                    #[codec(index = 56)]
                    BorrowfieldBadFieldError,
                    #[codec(index = 57)]
                    BorrowfieldExistsMutableBorrowError,
                    #[codec(index = 58)]
                    CopylocUnavailableError,
                    #[codec(index = 59)]
                    CopylocResourceError,
                    #[codec(index = 60)]
                    CopylocExistsBorrowError,
                    #[codec(index = 61)]
                    MovelocUnavailableError,
                    #[codec(index = 62)]
                    MovelocExistsBorrowError,
                    #[codec(index = 63)]
                    BorrowlocReferenceError,
                    #[codec(index = 64)]
                    BorrowlocUnavailableError,
                    #[codec(index = 65)]
                    BorrowlocExistsBorrowError,
                    #[codec(index = 66)]
                    CallTypeMismatchError,
                    #[codec(index = 67)]
                    CallBorrowedMutableReferenceError,
                    #[codec(index = 68)]
                    PackTypeMismatchError,
                    #[codec(index = 69)]
                    UnpackTypeMismatchError,
                    #[codec(index = 70)]
                    ReadrefTypeMismatchError,
                    #[codec(index = 71)]
                    ReadrefResourceError,
                    #[codec(index = 72)]
                    ReadrefExistsMutableBorrowError,
                    #[codec(index = 73)]
                    WriterefTypeMismatchError,
                    #[codec(index = 74)]
                    WriterefResourceError,
                    #[codec(index = 75)]
                    WriterefExistsBorrowError,
                    #[codec(index = 76)]
                    WriterefNoMutableReferenceError,
                    #[codec(index = 77)]
                    IntegerOpTypeMismatchError,
                    #[codec(index = 78)]
                    BooleanOpTypeMismatchError,
                    #[codec(index = 79)]
                    EqualityOpTypeMismatchError,
                    #[codec(index = 80)]
                    ExistsResourceTypeMismatchError,
                    #[codec(index = 81)]
                    BorrowglobalTypeMismatchError,
                    #[codec(index = 82)]
                    BorrowglobalNoResourceError,
                    #[codec(index = 83)]
                    MovefromTypeMismatchError,
                    #[codec(index = 84)]
                    MovefromNoResourceError,
                    #[codec(index = 85)]
                    MovetoTypeMismatchError,
                    #[codec(index = 86)]
                    MovetoNoResourceError,
                    #[codec(index = 87)]
                    ModuleAddressDoesNotMatchSender,
                    #[codec(index = 88)]
                    NoModuleHandles,
                    #[codec(index = 89)]
                    PositiveStackSizeAtBlockEnd,
                    #[codec(index = 90)]
                    MissingAcquiresResourceAnnotationError,
                    #[codec(index = 91)]
                    ExtraneousAcquiresResourceAnnotationError,
                    #[codec(index = 92)]
                    DuplicateAcquiresResourceAnnotationError,
                    #[codec(index = 93)]
                    InvalidAcquiresResourceAnnotationError,
                    #[codec(index = 94)]
                    GlobalReferenceError,
                    #[codec(index = 95)]
                    ConstraintKindMismatch,
                    #[codec(index = 96)]
                    NumberOfTypeArgumentsMismatch,
                    #[codec(index = 97)]
                    LoopInInstantiationGraph,
                    #[codec(index = 98)]
                    ZeroSizedStruct,
                    #[codec(index = 99)]
                    LinkerError,
                    #[codec(index = 100)]
                    InvalidConstantType,
                    #[codec(index = 101)]
                    MalformedConstantData,
                    #[codec(index = 102)]
                    EmptyCodeUnit,
                    #[codec(index = 103)]
                    InvalidLoopSplit,
                    #[codec(index = 104)]
                    InvalidLoopBreak,
                    #[codec(index = 105)]
                    InvalidLoopContinue,
                    #[codec(index = 106)]
                    UnsafeRetUnusedResources,
                    #[codec(index = 107)]
                    TooManyLocals,
                    #[codec(index = 108)]
                    GenericMemberOpcodeMismatch,
                    #[codec(index = 109)]
                    FunctionResolutionFailure,
                    #[codec(index = 110)]
                    InvalidOperationInScript,
                    #[codec(index = 111)]
                    DuplicateModuleName,
                    #[codec(index = 112)]
                    UnknownInvariantViolationError,
                    #[codec(index = 113)]
                    EmptyValueStack,
                    #[codec(index = 114)]
                    PcOverflow,
                    #[codec(index = 115)]
                    VerificationError,
                    #[codec(index = 116)]
                    StorageError,
                    #[codec(index = 117)]
                    InternalTypeError,
                    #[codec(index = 118)]
                    EventKeyMismatch,
                    #[codec(index = 119)]
                    Unreachable,
                    #[codec(index = 120)]
                    VmStartupFailure,
                    #[codec(index = 121)]
                    UnexpectedErrorFromKnownMoveFunction,
                    #[codec(index = 122)]
                    VerifierInvariantViolation,
                    #[codec(index = 123)]
                    UnexpectedVerifierError,
                    #[codec(index = 124)]
                    UnexpectedDeserializationError,
                    #[codec(index = 125)]
                    FailedToSerializeWriteSetChanges,
                    #[codec(index = 126)]
                    FailedToDeserializeResource,
                    #[codec(index = 127)]
                    TypeResolutionFailure,
                    #[codec(index = 128)]
                    UnknownBinaryError,
                    #[codec(index = 129)]
                    Malformed,
                    #[codec(index = 130)]
                    BadMagic,
                    #[codec(index = 131)]
                    UnknownVersion,
                    #[codec(index = 132)]
                    UnknownTableType,
                    #[codec(index = 133)]
                    UnknownSignatureType,
                    #[codec(index = 134)]
                    UnknownSerializedType,
                    #[codec(index = 135)]
                    UnknownOpcode,
                    #[codec(index = 136)]
                    BadHeaderTable,
                    #[codec(index = 137)]
                    UnexpectedSignatureType,
                    #[codec(index = 138)]
                    DuplicateTable,
                    #[codec(index = 139)]
                    UnknownNominalResource,
                    #[codec(index = 140)]
                    UnknownKind,
                    #[codec(index = 141)]
                    UnknownNativeStructFlag,
                    #[codec(index = 142)]
                    BadU64,
                    #[codec(index = 143)]
                    BadU128,
                    #[codec(index = 144)]
                    ValueSerializationError,
                    #[codec(index = 145)]
                    ValueDeserializationError,
                    #[codec(index = 146)]
                    CodeDeserializationError,
                    #[codec(index = 147)]
                    UnknownRuntimeStatus,
                    #[codec(index = 148)]
                    OutOfGas,
                    #[codec(index = 149)]
                    ResourceDoesNotExist,
                    #[codec(index = 150)]
                    ResourceAlreadyExists,
                    #[codec(index = 151)]
                    MissingData,
                    #[codec(index = 152)]
                    DataFormatError,
                    #[codec(index = 153)]
                    Aborted,
                    #[codec(index = 154)]
                    ArithmeticError,
                    #[codec(index = 155)]
                    ExecutionStackOverflow,
                    #[codec(index = 156)]
                    CallStackOverflow,
                    #[codec(index = 157)]
                    VmMaxTypeDepthReached,
                    #[codec(index = 158)]
                    VmMaxValueDepthReached,
                    #[codec(index = 159)]
                    UnknownStatus,
                    #[codec(index = 160)]
                    BadTransactionFeeCurrency,
                    #[codec(index = 161)]
                    FeatureUnderGating,
                    #[codec(index = 162)]
                    FieldMissingTypeAbility,
                    #[codec(index = 163)]
                    PopWithoutDropAbility,
                    #[codec(index = 164)]
                    CopylocWithoutCopyAbility,
                    #[codec(index = 165)]
                    ReadrefWithoutCopyAbility,
                    #[codec(index = 166)]
                    WriterefWithoutDropAbility,
                    #[codec(index = 167)]
                    ExistsWithoutKeyAbilityOrBadArgument,
                    #[codec(index = 168)]
                    BorrowglobalWithoutKeyAbility,
                    #[codec(index = 169)]
                    MovefromWithoutKeyAbility,
                    #[codec(index = 170)]
                    MovetoWithoutKeyAbility,
                    #[codec(index = 171)]
                    MissingAcquiresAnnotation,
                    #[codec(index = 172)]
                    ExtraneousAcquiresAnnotation,
                    #[codec(index = 173)]
                    DuplicateAcquiresAnnotation,
                    #[codec(index = 174)]
                    InvalidAcquiresAnnotation,
                    #[codec(index = 175)]
                    ConstraintNotSatisfied,
                    #[codec(index = 176)]
                    UnsafeRetUnusedValuesWithoutDrop,
                    #[codec(index = 177)]
                    BackwardIncompatibleModuleUpdate,
                    #[codec(index = 178)]
                    CyclicModuleDependency,
                    #[codec(index = 179)]
                    NumberOfArgumentsMismatch,
                    #[codec(index = 180)]
                    InvalidParamTypeForDeserialization,
                    #[codec(index = 181)]
                    FailedToDeserializeArgument,
                    #[codec(index = 182)]
                    NumberOfSignerArgumentsMismatch,
                    #[codec(index = 183)]
                    CalledScriptVisibleFromNonScriptVisible,
                    #[codec(index = 184)]
                    ExecuteScriptFunctionCalledOnNonScriptVisible,
                    #[codec(index = 185)]
                    InvalidFriendDeclWithSelf,
                    #[codec(index = 186)]
                    InvalidFriendDeclWithModulesOutsideAccountAddress,
                    #[codec(index = 187)]
                    InvalidFriendDeclWithModulesInDependencies,
                    #[codec(index = 188)]
                    CyclicModuleFriendship,
                    #[codec(index = 189)]
                    UnknownAbility,
                    #[codec(index = 190)]
                    InvalidFlagBits,
                    #[codec(index = 191)]
                    SecondaryKeysAddressesCountMismatch,
                    #[codec(index = 192)]
                    SignersContainDuplicates,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    #[codec(index = 0)]
                    Event(
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::std::vec::Vec<::core::primitive::u8>,
                    ),
                    #[codec(index = 1)]
                    ModulePublished(::subxt::sp_core::crypto::AccountId32),
                    #[codec(index = 2)]
                    StdModulePublished,
                }
            }
        }
        pub mod sp_runtime {
            use super::runtime_types;
            pub mod generic {
                use super::runtime_types;
                pub mod digest {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum ChangesTrieSignal {
                        #[codec(index = 0)]
                        NewConfiguration(
                            ::core::option::Option<
                                runtime_types::sp_core::changes_trie::ChangesTrieConfiguration,
                            >,
                        ),
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub struct Digest<_0> {
                        pub logs: ::std::vec::Vec<
                            runtime_types::sp_runtime::generic::digest::DigestItem<_0>,
                        >,
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum DigestItem<_0> {
                        #[codec(index = 2)]
                        ChangesTrieRoot(_0),
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
                        #[codec(index = 7)]
                        ChangesTrieSignal(
                            runtime_types::sp_runtime::generic::digest::ChangesTrieSignal,
                        ),
                        #[codec(index = 0)]
                        Other(::std::vec::Vec<::core::primitive::u8>),
                        #[codec(index = 8)]
                        RuntimeEnvironmentUpdated,
                    }
                }
                pub mod era {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
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
                pub mod unchecked_extrinsic {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub struct UncheckedExtrinsic<_0, _1, _2, _3>(
                        ::std::vec::Vec<::core::primitive::u8>,
                        #[codec(skip)] pub ::core::marker::PhantomData<(_1, _0, _2, _3)>,
                    );
                }
            }
            pub mod multiaddress {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
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
                Module {
                    index: ::core::primitive::u8,
                    error: ::core::primitive::u8,
                },
                #[codec(index = 4)]
                ConsumerRemaining,
                #[codec(index = 5)]
                NoProviders,
                #[codec(index = 6)]
                Token(runtime_types::sp_runtime::TokenError),
                #[codec(index = 7)]
                Arithmetic(runtime_types::sp_runtime::ArithmeticError),
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
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
        pub mod sp_trie {
            use super::runtime_types;
            pub mod storage_proof {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct StorageProof {
                    pub trie_nodes: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                }
            }
        }
        pub mod sp_version {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct RuntimeVersion {
                pub spec_name: ::std::string::String,
                pub impl_name: ::std::string::String,
                pub authoring_version: ::core::primitive::u32,
                pub spec_version: ::core::primitive::u32,
                pub impl_version: ::core::primitive::u32,
                pub apis:
                    ::std::vec::Vec<([::core::primitive::u8; 8usize], ::core::primitive::u32)>,
                pub transaction_version: ::core::primitive::u32,
            }
        }
        pub mod xcm {
            use super::runtime_types;
            pub mod double_encoded {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct DoubleEncoded {
                    pub encoded: ::std::vec::Vec<::core::primitive::u8>,
                }
            }
            pub mod v0 {
                use super::runtime_types;
                pub mod junction {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum BodyId {
                        #[codec(index = 0)]
                        Unit,
                        #[codec(index = 1)]
                        Named(::std::vec::Vec<::core::primitive::u8>),
                        #[codec(index = 2)]
                        Index(#[codec(compact)] ::core::primitive::u32),
                        #[codec(index = 3)]
                        Executive,
                        #[codec(index = 4)]
                        Technical,
                        #[codec(index = 5)]
                        Legislative,
                        #[codec(index = 6)]
                        Judicial,
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum BodyPart {
                        #[codec(index = 0)]
                        Voice,
                        #[codec(index = 1)]
                        Members {
                            #[codec(compact)]
                            count: ::core::primitive::u32,
                        },
                        #[codec(index = 2)]
                        Fraction {
                            #[codec(compact)]
                            nom: ::core::primitive::u32,
                            #[codec(compact)]
                            denom: ::core::primitive::u32,
                        },
                        #[codec(index = 3)]
                        AtLeastProportion {
                            #[codec(compact)]
                            nom: ::core::primitive::u32,
                            #[codec(compact)]
                            denom: ::core::primitive::u32,
                        },
                        #[codec(index = 4)]
                        MoreThanProportion {
                            #[codec(compact)]
                            nom: ::core::primitive::u32,
                            #[codec(compact)]
                            denom: ::core::primitive::u32,
                        },
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum Junction {
                        #[codec(index = 0)]
                        Parent,
                        #[codec(index = 1)]
                        Parachain(#[codec(compact)] ::core::primitive::u32),
                        #[codec(index = 2)]
                        AccountId32 {
                            network: runtime_types::xcm::v0::junction::NetworkId,
                            id: [::core::primitive::u8; 32usize],
                        },
                        #[codec(index = 3)]
                        AccountIndex64 {
                            network: runtime_types::xcm::v0::junction::NetworkId,
                            #[codec(compact)]
                            index: ::core::primitive::u64,
                        },
                        #[codec(index = 4)]
                        AccountKey20 {
                            network: runtime_types::xcm::v0::junction::NetworkId,
                            key: [::core::primitive::u8; 20usize],
                        },
                        #[codec(index = 5)]
                        PalletInstance(::core::primitive::u8),
                        #[codec(index = 6)]
                        GeneralIndex(#[codec(compact)] ::core::primitive::u128),
                        #[codec(index = 7)]
                        GeneralKey(::std::vec::Vec<::core::primitive::u8>),
                        #[codec(index = 8)]
                        OnlyChild,
                        #[codec(index = 9)]
                        Plurality {
                            id: runtime_types::xcm::v0::junction::BodyId,
                            part: runtime_types::xcm::v0::junction::BodyPart,
                        },
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum NetworkId {
                        #[codec(index = 0)]
                        Any,
                        #[codec(index = 1)]
                        Named(::std::vec::Vec<::core::primitive::u8>),
                        #[codec(index = 2)]
                        Polkadot,
                        #[codec(index = 3)]
                        Kusama,
                    }
                }
                pub mod multi_asset {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum MultiAsset {
                        #[codec(index = 0)]
                        None,
                        #[codec(index = 1)]
                        All,
                        #[codec(index = 2)]
                        AllFungible,
                        #[codec(index = 3)]
                        AllNonFungible,
                        #[codec(index = 4)]
                        AllAbstractFungible {
                            id: ::std::vec::Vec<::core::primitive::u8>,
                        },
                        #[codec(index = 5)]
                        AllAbstractNonFungible {
                            class: ::std::vec::Vec<::core::primitive::u8>,
                        },
                        #[codec(index = 6)]
                        AllConcreteFungible {
                            id: runtime_types::xcm::v0::multi_location::MultiLocation,
                        },
                        #[codec(index = 7)]
                        AllConcreteNonFungible {
                            class: runtime_types::xcm::v0::multi_location::MultiLocation,
                        },
                        #[codec(index = 8)]
                        AbstractFungible {
                            id: ::std::vec::Vec<::core::primitive::u8>,
                            #[codec(compact)]
                            amount: ::core::primitive::u128,
                        },
                        #[codec(index = 9)]
                        AbstractNonFungible {
                            class: ::std::vec::Vec<::core::primitive::u8>,
                            instance: runtime_types::xcm::v1::multiasset::AssetInstance,
                        },
                        #[codec(index = 10)]
                        ConcreteFungible {
                            id: runtime_types::xcm::v0::multi_location::MultiLocation,
                            #[codec(compact)]
                            amount: ::core::primitive::u128,
                        },
                        #[codec(index = 11)]
                        ConcreteNonFungible {
                            class: runtime_types::xcm::v0::multi_location::MultiLocation,
                            instance: runtime_types::xcm::v1::multiasset::AssetInstance,
                        },
                    }
                }
                pub mod multi_location {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum MultiLocation {
                        #[codec(index = 0)]
                        Null,
                        #[codec(index = 1)]
                        X1(runtime_types::xcm::v0::junction::Junction),
                        #[codec(index = 2)]
                        X2(
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                        ),
                        #[codec(index = 3)]
                        X3(
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                        ),
                        #[codec(index = 4)]
                        X4(
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                        ),
                        #[codec(index = 5)]
                        X5(
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                        ),
                        #[codec(index = 6)]
                        X6(
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                        ),
                        #[codec(index = 7)]
                        X7(
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                        ),
                        #[codec(index = 8)]
                        X8(
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                        ),
                    }
                }
                pub mod order {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum Order {
                        #[codec(index = 0)]
                        Null,
                        #[codec(index = 1)]
                        DepositAsset {
                            assets:
                                ::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
                            dest: runtime_types::xcm::v0::multi_location::MultiLocation,
                        },
                        #[codec(index = 2)]
                        DepositReserveAsset {
                            assets:
                                ::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
                            dest: runtime_types::xcm::v0::multi_location::MultiLocation,
                            effects: ::std::vec::Vec<runtime_types::xcm::v0::order::Order>,
                        },
                        #[codec(index = 3)]
                        ExchangeAsset {
                            give:
                                ::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
                            receive:
                                ::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
                        },
                        #[codec(index = 4)]
                        InitiateReserveWithdraw {
                            assets:
                                ::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
                            reserve: runtime_types::xcm::v0::multi_location::MultiLocation,
                            effects: ::std::vec::Vec<runtime_types::xcm::v0::order::Order>,
                        },
                        #[codec(index = 5)]
                        InitiateTeleport {
                            assets:
                                ::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
                            dest: runtime_types::xcm::v0::multi_location::MultiLocation,
                            effects: ::std::vec::Vec<runtime_types::xcm::v0::order::Order>,
                        },
                        #[codec(index = 6)]
                        QueryHolding {
                            #[codec(compact)]
                            query_id: ::core::primitive::u64,
                            dest: runtime_types::xcm::v0::multi_location::MultiLocation,
                            assets:
                                ::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
                        },
                        #[codec(index = 7)]
                        BuyExecution {
                            fees: runtime_types::xcm::v0::multi_asset::MultiAsset,
                            weight: ::core::primitive::u64,
                            debt: ::core::primitive::u64,
                            halt_on_error: ::core::primitive::bool,
                            xcm: ::std::vec::Vec<runtime_types::xcm::v0::Xcm>,
                        },
                    }
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum OriginKind {
                    #[codec(index = 0)]
                    Native,
                    #[codec(index = 1)]
                    SovereignAccount,
                    #[codec(index = 2)]
                    Superuser,
                    #[codec(index = 3)]
                    Xcm,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Response {
                    #[codec(index = 0)]
                    Assets(::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>),
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Xcm {
                    #[codec(index = 0)]
                    WithdrawAsset {
                        assets: ::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
                        effects: ::std::vec::Vec<runtime_types::xcm::v0::order::Order>,
                    },
                    #[codec(index = 1)]
                    ReserveAssetDeposit {
                        assets: ::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
                        effects: ::std::vec::Vec<runtime_types::xcm::v0::order::Order>,
                    },
                    #[codec(index = 2)]
                    TeleportAsset {
                        assets: ::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
                        effects: ::std::vec::Vec<runtime_types::xcm::v0::order::Order>,
                    },
                    #[codec(index = 3)]
                    QueryResponse {
                        #[codec(compact)]
                        query_id: ::core::primitive::u64,
                        response: runtime_types::xcm::v0::Response,
                    },
                    #[codec(index = 4)]
                    TransferAsset {
                        assets: ::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
                        dest: runtime_types::xcm::v0::multi_location::MultiLocation,
                    },
                    #[codec(index = 5)]
                    TransferReserveAsset {
                        assets: ::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
                        dest: runtime_types::xcm::v0::multi_location::MultiLocation,
                        effects: ::std::vec::Vec<runtime_types::xcm::v0::order::Order>,
                    },
                    #[codec(index = 6)]
                    Transact {
                        origin_type: runtime_types::xcm::v0::OriginKind,
                        require_weight_at_most: ::core::primitive::u64,
                        call: runtime_types::xcm::double_encoded::DoubleEncoded,
                    },
                    #[codec(index = 7)]
                    HrmpNewChannelOpenRequest {
                        #[codec(compact)]
                        sender: ::core::primitive::u32,
                        #[codec(compact)]
                        max_message_size: ::core::primitive::u32,
                        #[codec(compact)]
                        max_capacity: ::core::primitive::u32,
                    },
                    #[codec(index = 8)]
                    HrmpChannelAccepted {
                        #[codec(compact)]
                        recipient: ::core::primitive::u32,
                    },
                    #[codec(index = 9)]
                    HrmpChannelClosing {
                        #[codec(compact)]
                        initiator: ::core::primitive::u32,
                        #[codec(compact)]
                        sender: ::core::primitive::u32,
                        #[codec(compact)]
                        recipient: ::core::primitive::u32,
                    },
                    #[codec(index = 10)]
                    RelayedFrom {
                        who: runtime_types::xcm::v0::multi_location::MultiLocation,
                        message: ::std::boxed::Box<runtime_types::xcm::v0::Xcm>,
                    },
                }
            }
            pub mod v1 {
                use super::runtime_types;
                pub mod junction {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum Junction {
                        #[codec(index = 0)]
                        Parachain(#[codec(compact)] ::core::primitive::u32),
                        #[codec(index = 1)]
                        AccountId32 {
                            network: runtime_types::xcm::v0::junction::NetworkId,
                            id: [::core::primitive::u8; 32usize],
                        },
                        #[codec(index = 2)]
                        AccountIndex64 {
                            network: runtime_types::xcm::v0::junction::NetworkId,
                            #[codec(compact)]
                            index: ::core::primitive::u64,
                        },
                        #[codec(index = 3)]
                        AccountKey20 {
                            network: runtime_types::xcm::v0::junction::NetworkId,
                            key: [::core::primitive::u8; 20usize],
                        },
                        #[codec(index = 4)]
                        PalletInstance(::core::primitive::u8),
                        #[codec(index = 5)]
                        GeneralIndex(#[codec(compact)] ::core::primitive::u128),
                        #[codec(index = 6)]
                        GeneralKey(::std::vec::Vec<::core::primitive::u8>),
                        #[codec(index = 7)]
                        OnlyChild,
                        #[codec(index = 8)]
                        Plurality {
                            id: runtime_types::xcm::v0::junction::BodyId,
                            part: runtime_types::xcm::v0::junction::BodyPart,
                        },
                    }
                }
                pub mod multiasset {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum AssetId {
                        #[codec(index = 0)]
                        Concrete(runtime_types::xcm::v1::multilocation::MultiLocation),
                        #[codec(index = 1)]
                        Abstract(::std::vec::Vec<::core::primitive::u8>),
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum AssetInstance {
                        #[codec(index = 0)]
                        Undefined,
                        #[codec(index = 1)]
                        Index(#[codec(compact)] ::core::primitive::u128),
                        #[codec(index = 2)]
                        Array4([::core::primitive::u8; 4usize]),
                        #[codec(index = 3)]
                        Array8([::core::primitive::u8; 8usize]),
                        #[codec(index = 4)]
                        Array16([::core::primitive::u8; 16usize]),
                        #[codec(index = 5)]
                        Array32([::core::primitive::u8; 32usize]),
                        #[codec(index = 6)]
                        Blob(::std::vec::Vec<::core::primitive::u8>),
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum Fungibility {
                        #[codec(index = 0)]
                        Fungible(#[codec(compact)] ::core::primitive::u128),
                        #[codec(index = 1)]
                        NonFungible(runtime_types::xcm::v1::multiasset::AssetInstance),
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub struct MultiAsset {
                        pub id: runtime_types::xcm::v1::multiasset::AssetId,
                        pub fun: runtime_types::xcm::v1::multiasset::Fungibility,
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum MultiAssetFilter {
                        #[codec(index = 0)]
                        Definite(runtime_types::xcm::v1::multiasset::MultiAssets),
                        #[codec(index = 1)]
                        Wild(runtime_types::xcm::v1::multiasset::WildMultiAsset),
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub struct MultiAssets(
                        pub ::std::vec::Vec<runtime_types::xcm::v1::multiasset::MultiAsset>,
                    );
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum WildFungibility {
                        #[codec(index = 0)]
                        Fungible,
                        #[codec(index = 1)]
                        NonFungible,
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum WildMultiAsset {
                        #[codec(index = 0)]
                        All,
                        #[codec(index = 1)]
                        AllOf {
                            id: runtime_types::xcm::v1::multiasset::AssetId,
                            fun: runtime_types::xcm::v1::multiasset::WildFungibility,
                        },
                    }
                }
                pub mod multilocation {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum Junctions {
                        #[codec(index = 0)]
                        Here,
                        #[codec(index = 1)]
                        X1(runtime_types::xcm::v1::junction::Junction),
                        #[codec(index = 2)]
                        X2(
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                        ),
                        #[codec(index = 3)]
                        X3(
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                        ),
                        #[codec(index = 4)]
                        X4(
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                        ),
                        #[codec(index = 5)]
                        X5(
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                        ),
                        #[codec(index = 6)]
                        X6(
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                        ),
                        #[codec(index = 7)]
                        X7(
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                        ),
                        #[codec(index = 8)]
                        X8(
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                        ),
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub struct MultiLocation {
                        pub parents: ::core::primitive::u8,
                        pub interior: runtime_types::xcm::v1::multilocation::Junctions,
                    }
                }
                pub mod order {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum Order {
                        #[codec(index = 0)]
                        Noop,
                        #[codec(index = 1)]
                        DepositAsset {
                            assets: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
                            max_assets: ::core::primitive::u32,
                            beneficiary: runtime_types::xcm::v1::multilocation::MultiLocation,
                        },
                        #[codec(index = 2)]
                        DepositReserveAsset {
                            assets: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
                            max_assets: ::core::primitive::u32,
                            dest: runtime_types::xcm::v1::multilocation::MultiLocation,
                            effects: ::std::vec::Vec<runtime_types::xcm::v1::order::Order>,
                        },
                        #[codec(index = 3)]
                        ExchangeAsset {
                            give: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
                            receive: runtime_types::xcm::v1::multiasset::MultiAssets,
                        },
                        #[codec(index = 4)]
                        InitiateReserveWithdraw {
                            assets: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
                            reserve: runtime_types::xcm::v1::multilocation::MultiLocation,
                            effects: ::std::vec::Vec<runtime_types::xcm::v1::order::Order>,
                        },
                        #[codec(index = 5)]
                        InitiateTeleport {
                            assets: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
                            dest: runtime_types::xcm::v1::multilocation::MultiLocation,
                            effects: ::std::vec::Vec<runtime_types::xcm::v1::order::Order>,
                        },
                        #[codec(index = 6)]
                        QueryHolding {
                            #[codec(compact)]
                            query_id: ::core::primitive::u64,
                            dest: runtime_types::xcm::v1::multilocation::MultiLocation,
                            assets: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
                        },
                        #[codec(index = 7)]
                        BuyExecution {
                            fees: runtime_types::xcm::v1::multiasset::MultiAsset,
                            weight: ::core::primitive::u64,
                            debt: ::core::primitive::u64,
                            halt_on_error: ::core::primitive::bool,
                            instructions: ::std::vec::Vec<runtime_types::xcm::v1::Xcm>,
                        },
                    }
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Response {
                    #[codec(index = 0)]
                    Assets(runtime_types::xcm::v1::multiasset::MultiAssets),
                    #[codec(index = 1)]
                    Version(::core::primitive::u32),
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Xcm {
                    #[codec(index = 0)]
                    WithdrawAsset {
                        assets: runtime_types::xcm::v1::multiasset::MultiAssets,
                        effects: ::std::vec::Vec<runtime_types::xcm::v1::order::Order>,
                    },
                    #[codec(index = 1)]
                    ReserveAssetDeposited {
                        assets: runtime_types::xcm::v1::multiasset::MultiAssets,
                        effects: ::std::vec::Vec<runtime_types::xcm::v1::order::Order>,
                    },
                    #[codec(index = 2)]
                    ReceiveTeleportedAsset {
                        assets: runtime_types::xcm::v1::multiasset::MultiAssets,
                        effects: ::std::vec::Vec<runtime_types::xcm::v1::order::Order>,
                    },
                    #[codec(index = 3)]
                    QueryResponse {
                        #[codec(compact)]
                        query_id: ::core::primitive::u64,
                        response: runtime_types::xcm::v1::Response,
                    },
                    #[codec(index = 4)]
                    TransferAsset {
                        assets: runtime_types::xcm::v1::multiasset::MultiAssets,
                        beneficiary: runtime_types::xcm::v1::multilocation::MultiLocation,
                    },
                    #[codec(index = 5)]
                    TransferReserveAsset {
                        assets: runtime_types::xcm::v1::multiasset::MultiAssets,
                        dest: runtime_types::xcm::v1::multilocation::MultiLocation,
                        effects: ::std::vec::Vec<runtime_types::xcm::v1::order::Order>,
                    },
                    #[codec(index = 6)]
                    Transact {
                        origin_type: runtime_types::xcm::v0::OriginKind,
                        require_weight_at_most: ::core::primitive::u64,
                        call: runtime_types::xcm::double_encoded::DoubleEncoded,
                    },
                    #[codec(index = 7)]
                    HrmpNewChannelOpenRequest {
                        #[codec(compact)]
                        sender: ::core::primitive::u32,
                        #[codec(compact)]
                        max_message_size: ::core::primitive::u32,
                        #[codec(compact)]
                        max_capacity: ::core::primitive::u32,
                    },
                    #[codec(index = 8)]
                    HrmpChannelAccepted {
                        #[codec(compact)]
                        recipient: ::core::primitive::u32,
                    },
                    #[codec(index = 9)]
                    HrmpChannelClosing {
                        #[codec(compact)]
                        initiator: ::core::primitive::u32,
                        #[codec(compact)]
                        sender: ::core::primitive::u32,
                        #[codec(compact)]
                        recipient: ::core::primitive::u32,
                    },
                    #[codec(index = 10)]
                    RelayedFrom {
                        who: runtime_types::xcm::v1::multilocation::Junctions,
                        message: ::std::boxed::Box<runtime_types::xcm::v1::Xcm>,
                    },
                    #[codec(index = 11)]
                    SubscribeVersion {
                        #[codec(compact)]
                        query_id: ::core::primitive::u64,
                        #[codec(compact)]
                        max_response_weight: ::core::primitive::u64,
                    },
                    #[codec(index = 12)]
                    UnsubscribeVersion,
                }
            }
            pub mod v2 {
                use super::runtime_types;
                pub mod traits {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum Error {
                        #[codec(index = 0)]
                        Overflow,
                        #[codec(index = 1)]
                        Unimplemented,
                        #[codec(index = 2)]
                        UntrustedReserveLocation,
                        #[codec(index = 3)]
                        UntrustedTeleportLocation,
                        #[codec(index = 4)]
                        MultiLocationFull,
                        #[codec(index = 5)]
                        MultiLocationNotInvertible,
                        #[codec(index = 6)]
                        BadOrigin,
                        #[codec(index = 7)]
                        InvalidLocation,
                        #[codec(index = 8)]
                        AssetNotFound,
                        #[codec(index = 9)]
                        FailedToTransactAsset,
                        #[codec(index = 10)]
                        NotWithdrawable,
                        #[codec(index = 11)]
                        LocationCannotHold,
                        #[codec(index = 12)]
                        ExceedsMaxMessageSize,
                        #[codec(index = 13)]
                        DestinationUnsupported,
                        #[codec(index = 14)]
                        Transport,
                        #[codec(index = 15)]
                        Unroutable,
                        #[codec(index = 16)]
                        UnknownClaim,
                        #[codec(index = 17)]
                        FailedToDecode,
                        #[codec(index = 18)]
                        TooMuchWeightRequired,
                        #[codec(index = 19)]
                        NotHoldingFees,
                        #[codec(index = 20)]
                        TooExpensive,
                        #[codec(index = 21)]
                        Trap(::core::primitive::u64),
                        #[codec(index = 22)]
                        UnhandledXcmVersion,
                        #[codec(index = 23)]
                        WeightLimitReached(::core::primitive::u64),
                        #[codec(index = 24)]
                        Barrier,
                        #[codec(index = 25)]
                        WeightNotComputable,
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum Outcome {
                        #[codec(index = 0)]
                        Complete(::core::primitive::u64),
                        #[codec(index = 1)]
                        Incomplete(
                            ::core::primitive::u64,
                            runtime_types::xcm::v2::traits::Error,
                        ),
                        #[codec(index = 2)]
                        Error(runtime_types::xcm::v2::traits::Error),
                    }
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Instruction {
                    #[codec(index = 0)]
                    WithdrawAsset(runtime_types::xcm::v1::multiasset::MultiAssets),
                    #[codec(index = 1)]
                    ReserveAssetDeposited(runtime_types::xcm::v1::multiasset::MultiAssets),
                    #[codec(index = 2)]
                    ReceiveTeleportedAsset(runtime_types::xcm::v1::multiasset::MultiAssets),
                    #[codec(index = 3)]
                    QueryResponse {
                        #[codec(compact)]
                        query_id: ::core::primitive::u64,
                        response: runtime_types::xcm::v2::Response,
                        #[codec(compact)]
                        max_weight: ::core::primitive::u64,
                    },
                    #[codec(index = 4)]
                    TransferAsset {
                        assets: runtime_types::xcm::v1::multiasset::MultiAssets,
                        beneficiary: runtime_types::xcm::v1::multilocation::MultiLocation,
                    },
                    #[codec(index = 5)]
                    TransferReserveAsset {
                        assets: runtime_types::xcm::v1::multiasset::MultiAssets,
                        dest: runtime_types::xcm::v1::multilocation::MultiLocation,
                        xcm: runtime_types::xcm::v2::Xcm,
                    },
                    #[codec(index = 6)]
                    Transact {
                        origin_type: runtime_types::xcm::v0::OriginKind,
                        #[codec(compact)]
                        require_weight_at_most: ::core::primitive::u64,
                        call: runtime_types::xcm::double_encoded::DoubleEncoded,
                    },
                    #[codec(index = 7)]
                    HrmpNewChannelOpenRequest {
                        #[codec(compact)]
                        sender: ::core::primitive::u32,
                        #[codec(compact)]
                        max_message_size: ::core::primitive::u32,
                        #[codec(compact)]
                        max_capacity: ::core::primitive::u32,
                    },
                    #[codec(index = 8)]
                    HrmpChannelAccepted {
                        #[codec(compact)]
                        recipient: ::core::primitive::u32,
                    },
                    #[codec(index = 9)]
                    HrmpChannelClosing {
                        #[codec(compact)]
                        initiator: ::core::primitive::u32,
                        #[codec(compact)]
                        sender: ::core::primitive::u32,
                        #[codec(compact)]
                        recipient: ::core::primitive::u32,
                    },
                    #[codec(index = 10)]
                    ClearOrigin,
                    #[codec(index = 11)]
                    DescendOrigin(runtime_types::xcm::v1::multilocation::Junctions),
                    #[codec(index = 12)]
                    ReportError {
                        #[codec(compact)]
                        query_id: ::core::primitive::u64,
                        dest: runtime_types::xcm::v1::multilocation::MultiLocation,
                        #[codec(compact)]
                        max_response_weight: ::core::primitive::u64,
                    },
                    #[codec(index = 13)]
                    DepositAsset {
                        assets: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
                        #[codec(compact)]
                        max_assets: ::core::primitive::u32,
                        beneficiary: runtime_types::xcm::v1::multilocation::MultiLocation,
                    },
                    #[codec(index = 14)]
                    DepositReserveAsset {
                        assets: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
                        #[codec(compact)]
                        max_assets: ::core::primitive::u32,
                        dest: runtime_types::xcm::v1::multilocation::MultiLocation,
                        xcm: runtime_types::xcm::v2::Xcm,
                    },
                    #[codec(index = 15)]
                    ExchangeAsset {
                        give: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
                        receive: runtime_types::xcm::v1::multiasset::MultiAssets,
                    },
                    #[codec(index = 16)]
                    InitiateReserveWithdraw {
                        assets: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
                        reserve: runtime_types::xcm::v1::multilocation::MultiLocation,
                        xcm: runtime_types::xcm::v2::Xcm,
                    },
                    #[codec(index = 17)]
                    InitiateTeleport {
                        assets: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
                        dest: runtime_types::xcm::v1::multilocation::MultiLocation,
                        xcm: runtime_types::xcm::v2::Xcm,
                    },
                    #[codec(index = 18)]
                    QueryHolding {
                        #[codec(compact)]
                        query_id: ::core::primitive::u64,
                        dest: runtime_types::xcm::v1::multilocation::MultiLocation,
                        assets: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
                        #[codec(compact)]
                        max_response_weight: ::core::primitive::u64,
                    },
                    #[codec(index = 19)]
                    BuyExecution {
                        fees: runtime_types::xcm::v1::multiasset::MultiAsset,
                        weight_limit: runtime_types::xcm::v2::WeightLimit,
                    },
                    #[codec(index = 20)]
                    RefundSurplus,
                    #[codec(index = 21)]
                    SetErrorHandler(runtime_types::xcm::v2::Xcm),
                    #[codec(index = 22)]
                    SetAppendix(runtime_types::xcm::v2::Xcm),
                    #[codec(index = 23)]
                    ClearError,
                    #[codec(index = 24)]
                    ClaimAsset {
                        assets: runtime_types::xcm::v1::multiasset::MultiAssets,
                        ticket: runtime_types::xcm::v1::multilocation::MultiLocation,
                    },
                    #[codec(index = 25)]
                    Trap(#[codec(compact)] ::core::primitive::u64),
                    #[codec(index = 26)]
                    SubscribeVersion {
                        #[codec(compact)]
                        query_id: ::core::primitive::u64,
                        #[codec(compact)]
                        max_response_weight: ::core::primitive::u64,
                    },
                    #[codec(index = 27)]
                    UnsubscribeVersion,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Response {
                    #[codec(index = 0)]
                    Null,
                    #[codec(index = 1)]
                    Assets(runtime_types::xcm::v1::multiasset::MultiAssets),
                    #[codec(index = 2)]
                    ExecutionResult(
                        ::core::option::Option<(
                            ::core::primitive::u32,
                            runtime_types::xcm::v2::traits::Error,
                        )>,
                    ),
                    #[codec(index = 3)]
                    Version(::core::primitive::u32),
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum WeightLimit {
                    #[codec(index = 0)]
                    Unlimited,
                    #[codec(index = 1)]
                    Limited(#[codec(compact)] ::core::primitive::u64),
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct Xcm(pub ::std::vec::Vec<runtime_types::xcm::v2::Instruction>);
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum VersionedMultiAssets {
                #[codec(index = 0)]
                V0(::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>),
                #[codec(index = 1)]
                V1(runtime_types::xcm::v1::multiasset::MultiAssets),
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum VersionedMultiLocation {
                #[codec(index = 0)]
                V0(runtime_types::xcm::v0::multi_location::MultiLocation),
                #[codec(index = 1)]
                V1(runtime_types::xcm::v1::multilocation::MultiLocation),
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum VersionedXcm {
                #[codec(index = 0)]
                V0(runtime_types::xcm::v0::Xcm),
                #[codec(index = 1)]
                V1(runtime_types::xcm::v1::Xcm),
                #[codec(index = 2)]
                V2(runtime_types::xcm::v2::Xcm),
            }
        }
    }
    #[doc = r" Default configuration of common types for a target Substrate runtime."]
    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    pub struct DefaultConfig;
    impl ::subxt::Config for DefaultConfig {
        type Index = u32;
        type BlockNumber = u32;
        type Hash = ::subxt::sp_core::H256;
        type Hashing = ::subxt::sp_runtime::traits::BlakeTwo256;
        type AccountId = ::subxt::sp_runtime::AccountId32;
        type Address = ::subxt::sp_runtime::MultiAddress<Self::AccountId, u32>;
        type Header = ::subxt::sp_runtime::generic::Header<
            Self::BlockNumber,
            ::subxt::sp_runtime::traits::BlakeTwo256,
        >;
        type Signature = ::subxt::sp_runtime::MultiSignature;
        type Extrinsic = ::subxt::sp_runtime::OpaqueExtrinsic;
    }
    impl ::subxt::ExtrinsicExtraData<DefaultConfig> for DefaultConfig {
        type AccountData = AccountData;
        type Extra = ::subxt::DefaultExtra<DefaultConfig>;
    }
    pub type AccountData = self::system::storage::Account;
    impl ::subxt::AccountData<DefaultConfig> for AccountData {
        fn nonce(
            result: &<Self as ::subxt::StorageEntry>::Value,
        ) -> <DefaultConfig as ::subxt::Config>::Index {
            result.nonce
        }
        fn storage_entry(account_id: <DefaultConfig as ::subxt::Config>::AccountId) -> Self {
            Self(account_id)
        }
    }
    pub struct RuntimeApi<T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
        pub client: ::subxt::Client<T>,
    }
    impl<T> ::core::convert::From<::subxt::Client<T>> for RuntimeApi<T>
    where
        T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
    {
        fn from(client: ::subxt::Client<T>) -> Self {
            Self { client }
        }
    }
    impl<'a, T> RuntimeApi<T>
    where
        T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
    {
        pub fn storage(&'a self) -> StorageApi<'a, T> {
            StorageApi {
                client: &self.client,
            }
        }
        pub fn tx(&'a self) -> TransactionApi<'a, T> {
            TransactionApi {
                client: &self.client,
            }
        }
    }
    pub struct StorageApi<'a, T>
    where
        T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
    {
        client: &'a ::subxt::Client<T>,
    }
    impl<'a, T> StorageApi<'a, T>
    where
        T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
    {
        pub fn system(&self) -> system::storage::StorageApi<'a, T> {
            system::storage::StorageApi::new(self.client)
        }
        pub fn randomness_collective_flip(
            &self,
        ) -> randomness_collective_flip::storage::StorageApi<'a, T> {
            randomness_collective_flip::storage::StorageApi::new(self.client)
        }
        pub fn timestamp(&self) -> timestamp::storage::StorageApi<'a, T> {
            timestamp::storage::StorageApi::new(self.client)
        }
        pub fn transaction_payment(&self) -> transaction_payment::storage::StorageApi<'a, T> {
            transaction_payment::storage::StorageApi::new(self.client)
        }
        pub fn sudo(&self) -> sudo::storage::StorageApi<'a, T> {
            sudo::storage::StorageApi::new(self.client)
        }
        pub fn parachain_system(&self) -> parachain_system::storage::StorageApi<'a, T> {
            parachain_system::storage::StorageApi::new(self.client)
        }
        pub fn parachain_info(&self) -> parachain_info::storage::StorageApi<'a, T> {
            parachain_info::storage::StorageApi::new(self.client)
        }
        pub fn balances(&self) -> balances::storage::StorageApi<'a, T> {
            balances::storage::StorageApi::new(self.client)
        }
        pub fn vesting(&self) -> vesting::storage::StorageApi<'a, T> {
            vesting::storage::StorageApi::new(self.client)
        }
        pub fn parachain_staking(&self) -> parachain_staking::storage::StorageApi<'a, T> {
            parachain_staking::storage::StorageApi::new(self.client)
        }
        pub fn author_inherent(&self) -> author_inherent::storage::StorageApi<'a, T> {
            author_inherent::storage::StorageApi::new(self.client)
        }
        pub fn author_filter(&self) -> author_filter::storage::StorageApi<'a, T> {
            author_filter::storage::StorageApi::new(self.client)
        }
        pub fn author_mapping(&self) -> author_mapping::storage::StorageApi<'a, T> {
            author_mapping::storage::StorageApi::new(self.client)
        }
        pub fn xcmp_queue(&self) -> xcmp_queue::storage::StorageApi<'a, T> {
            xcmp_queue::storage::StorageApi::new(self.client)
        }
        pub fn dmp_queue(&self) -> dmp_queue::storage::StorageApi<'a, T> {
            dmp_queue::storage::StorageApi::new(self.client)
        }
        pub fn mvm(&self) -> mvm::storage::StorageApi<'a, T> {
            mvm::storage::StorageApi::new(self.client)
        }
        pub fn multi_sig(&self) -> multi_sig::storage::StorageApi<'a, T> {
            multi_sig::storage::StorageApi::new(self.client)
        }
    }
    pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
        client: &'a ::subxt::Client<T>,
    }
    impl<'a, T> TransactionApi<'a, T>
    where
        T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
    {
        pub fn system(&self) -> system::calls::TransactionApi<'a, T> {
            system::calls::TransactionApi::new(self.client)
        }
        pub fn timestamp(&self) -> timestamp::calls::TransactionApi<'a, T> {
            timestamp::calls::TransactionApi::new(self.client)
        }
        pub fn sudo(&self) -> sudo::calls::TransactionApi<'a, T> {
            sudo::calls::TransactionApi::new(self.client)
        }
        pub fn parachain_system(&self) -> parachain_system::calls::TransactionApi<'a, T> {
            parachain_system::calls::TransactionApi::new(self.client)
        }
        pub fn balances(&self) -> balances::calls::TransactionApi<'a, T> {
            balances::calls::TransactionApi::new(self.client)
        }
        pub fn vesting(&self) -> vesting::calls::TransactionApi<'a, T> {
            vesting::calls::TransactionApi::new(self.client)
        }
        pub fn parachain_staking(&self) -> parachain_staking::calls::TransactionApi<'a, T> {
            parachain_staking::calls::TransactionApi::new(self.client)
        }
        pub fn author_inherent(&self) -> author_inherent::calls::TransactionApi<'a, T> {
            author_inherent::calls::TransactionApi::new(self.client)
        }
        pub fn author_filter(&self) -> author_filter::calls::TransactionApi<'a, T> {
            author_filter::calls::TransactionApi::new(self.client)
        }
        pub fn author_mapping(&self) -> author_mapping::calls::TransactionApi<'a, T> {
            author_mapping::calls::TransactionApi::new(self.client)
        }
        pub fn xcmp_queue(&self) -> xcmp_queue::calls::TransactionApi<'a, T> {
            xcmp_queue::calls::TransactionApi::new(self.client)
        }
        pub fn polkadot_xcm(&self) -> polkadot_xcm::calls::TransactionApi<'a, T> {
            polkadot_xcm::calls::TransactionApi::new(self.client)
        }
        pub fn cumulus_xcm(&self) -> cumulus_xcm::calls::TransactionApi<'a, T> {
            cumulus_xcm::calls::TransactionApi::new(self.client)
        }
        pub fn dmp_queue(&self) -> dmp_queue::calls::TransactionApi<'a, T> {
            dmp_queue::calls::TransactionApi::new(self.client)
        }
        pub fn mvm(&self) -> mvm::calls::TransactionApi<'a, T> {
            mvm::calls::TransactionApi::new(self.client)
        }
        pub fn multi_sig(&self) -> multi_sig::calls::TransactionApi<'a, T> {
            multi_sig::calls::TransactionApi::new(self.client)
        }
    }
}
