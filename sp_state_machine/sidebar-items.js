initSidebarItems({"constant":[["MAX_NESTED_TRIE_DEPTH","State machine only allows a single level of child trie."]],"enum":[["BackendTrustLevel","Storage backend trust level."],["ExecutionError","Externalities Error."],["ExecutionError","Externalities Error."],["ExecutionManager","Like `ExecutionStrategy` only it also stores a handler in case of consensus failure."],["ExecutionStrategy","Strategy for executing a call into the runtime."],["IndexOperation","Transaction index operation."]],"fn":[["create_proof_check_backend","Create a backend used for checking the proof., using `H` as hasher."],["execution_proof_check","Check execution proof, generated by `prove_execution` call."],["execution_proof_check_on_trie_backend","Check execution proof on proving backend, generated by `prove_execution` call."],["native_else_wasm","Evaluate to ExecutionManager::NativeElseWasm, without having to figure out the type."],["new_in_mem","Create a new empty instance of in-memory backend."],["prove_child_read","Generate child storage read proof."],["prove_child_read_on_trie_backend","Generate storage read proof on pre-created trie backend."],["prove_execution","Prove execution using the given state backend, overlayed changes, and call executor."],["prove_execution_on_trie_backend","Prove execution using the given trie backend, overlayed changes, and call executor. Produces a state-backend-specific “transaction” which can be used to apply the changes to the backing store, such as the disk. Execution proof is the set of all ‘touched’ storage DBValues from the backend."],["prove_range_read_with_child_with_size","Generate range storage read proof, with child tries content. A size limit is applied to the proof with the exception that `start_at` and its following element are always part of the proof. If a key different than `start_at` is a child trie root, the child trie content will be included in the proof."],["prove_range_read_with_child_with_size_on_trie_backend","Generate range storage read proof, with child tries content. See `prove_range_read_with_child_with_size`."],["prove_range_read_with_size","Generate range storage read proof."],["prove_range_read_with_size_on_trie_backend","Generate range storage read proof on an existing trie backend."],["prove_read","Generate storage read proof."],["prove_read_on_trie_backend","Generate storage read proof on pre-created trie backend."],["read_child_proof_check","Check child storage read proof, generated by `prove_child_read` call."],["read_child_proof_check_on_proving_backend","Check child storage read proof on pre-created proving backend."],["read_proof_check","Check storage read proof, generated by `prove_read` call."],["read_proof_check_on_proving_backend","Check storage read proof on pre-created proving backend."],["read_range_proof_check","Check child storage range proof, generated by `prove_range_read_with_size` call."],["read_range_proof_check_on_proving_backend","Check storage range proof on pre-created proving backend."],["read_range_proof_check_with_child","Check storage range proof with child trie included, generated by `prove_range_read_with_child_with_size` call."],["read_range_proof_check_with_child_on_proving_backend","Check storage range proof on pre-created proving backend."]],"macro":[["debug","Logs a message at the debug level."],["log_error","Logs a message at the error level."],["trace","Constructs an event at the trace level."],["warn","Logs a message at the warn level."]],"mod":[["backend","State machine backends. These manage the code and storage of contracts."]],"struct":[["BasicExternalities","Simple Map-based Externalities impl."],["CompactProof","Storage proof in compact form."],["Ext","Wraps a read-only backend, call executor, and current overlayed changes."],["KeyValueStates","Multiple key value state. States are ordered by root storage key."],["KeyValueStorageLevel","A key value state at any storage level."],["LayoutV0","substrate trie layout"],["LayoutV1","substrate trie layout, with external value nodes."],["OffchainOverlayedChanges","In-memory storage for offchain workers recoding changes for the actual offchain storage implementation."],["OverlayedChanges","The set of changes that are overlaid onto the backend."],["ProofRecorder","Global proof recorder, act as a layer over a hash db for recording queried data."],["ProvingBackend","Patricia trie-based backend which also tracks all touched storage trie values. These can be sent to remote node and used as a proof of execution."],["ProvingBackendRecorder","Patricia trie-based backend specialized in get value proofs."],["ReadOnlyExternalities","Simple read-only externalities for any backend."],["StateMachine","The substrate state machine."],["StateMachineStats","Accumulated usage statistics specific to state machine crate."],["StorageChanges","A storage changes structure that can be generated by the data collected in [`OverlayedChanges`]."],["StorageProof","A proof that some set of key-value pairs are included in the storage trie. The proof contains the storage values so that the partial storage backend can be reconstructed by a verifier that does not already have access to the key-value pairs."],["StorageTransactionCache","Storage transactions are calculated as part of the `storage_root`. These transactions can be reused for importing the block into the storage. So, we cache them to not require a recomputation of those transactions."],["TestExternalities","Simple HashMap-based Externalities impl."],["TrieBackend","Patricia trie-based backend. Transaction type is an overlay of changes to commit."],["UsageInfo","Usage statistics for state backend."],["UsageUnit","Measured count of operations and total bytes."]],"trait":[["Error","State Machine Error bound."],["Error","State Machine Error bound."],["InspectState","Trait for inspecting state in any backend."],["Storage","Patricia trie-based storage trait."],["TrieBackendStorage","Key-value pairs storage that is used by trie backend essence."],["TrieMut","A key-value datastore implemented as a database-backed modified Merkle tree."]],"type":[["ChildStorageCollection","In memory arrays of storage values for multiple child tries."],["DBValue","Database value"],["DefaultError","Default error type to use with state machine trie backend."],["DefaultHandler","Default handler of the execution manager."],["InMemoryBackend","Trie backend with in-memory storage."],["InMemoryProvingBackend","Proving Trie backend with in-memory storage."],["MemoryDB","Reexport from `hash_db`, with genericity set for `Hasher` trait. This uses a noops `KeyFunction` (key addressing must be hashed or using an encoding scheme that avoid key conflict)."],["OffchainChangesCollection","In memory array of storage values."],["StorageCollection","In memory array of storage values."],["StorageKey","Storage key."],["StorageValue","Storage value."],["TrieDBMutV0","Persistent trie database write-access interface for the a given hasher."],["TrieDBMutV1","Persistent trie database write-access interface for the a given hasher."]]});