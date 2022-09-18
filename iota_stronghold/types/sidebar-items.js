initSidebarItems({"constant":[["DEFAULT_RANDOM_HINT_SIZE",""]],"enum":[["ClientError",""],["Location","A `Location` type used to specify where in the `Stronghold` a piece of data should be stored. A generic location specifies a non-versioned location while a counter location specifies a versioned location. The Counter location can be used to get the head of the version chain by passing in `None` as the counter index. Otherwise, counter records are referenced through their associated index.  On Read, the `None` location is the latest record in the version chain while on Write, the `None` location is the next record in the version chain."],["RemoteMergeError",""],["RemoteVaultError",""],["SnapshotError",""],["UseKey",""]],"fn":[["derive_record_id",""],["derive_record_id_from_counter",""],["derive_vault_id",""]],"struct":[["Client",""],["ClientVault",""],["FatalEngineError",""],["Snapshot","Wrapper for the [`SnapshotState`] data structure."],["SnapshotPath","A handle for snapshot file locations."],["SnapshotState","Data structure that is written to the snapshot."],["Store",""],["Stronghold","The Stronghold is a secure storage for sensitive data. Secrets that are stored inside a Stronghold can never be read, but only be accessed via cryptographic procedures. Data inside a Stronghold is heavily protected by the `Runtime` by either being encrypted at rest, having kernel supplied memory guards, that prevent memory dumps, or a combination of both. The Stronghold also persists data written into a Stronghold by creating Snapshots of the current state. The Snapshot itself is encrypted and can be accessed by a key."]],"type":[["ClientState",""],["RecordError",""],["RemoteRecordError",""],["VaultError",""]]});