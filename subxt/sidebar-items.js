initSidebarItems({"attr":[["subxt",""]],"enum":[["DefaultConfig","Default set of commonly used types by Substrate runtimes."],["GenericError","The underlying error enum, generic over the type held by the `Runtime` variant. Prefer to use the [`Error<E>`] and [`BasicError`] aliases over using this type directly."],["MetadataError","Metadata error."],["Phase","A phase of a block’s execution."],["StorageHasher","Hasher used by storage maps"],["TransactionError","Transaction error."],["TransactionStatus","Possible transaction statuses returned from our [`TransactionProgress::next_item()`] call."]],"mod":[["events","For working with events."],["extrinsic","Create signed or unsigned extrinsics."],["rpc","RPC types and client for interacting with a substrate node."],["storage","For querying runtime storage."]],"struct":[["Client","Client to interface with a substrate node."],["ClientBuilder","ClientBuilder for constructing a Client."],["Encoded","Wraps an already encoded byte vector, prevents being encoded as a raw byte vector as part of the transaction payload"],["ErrorMetadata","Metadata for specific errors."],["EventDetails","A decoded event and associated details."],["Events","A collection of events obtained from a block, bundled with the necessary information needed to decode and iterate over them."],["Metadata","Runtime metadata."],["ModuleErrorData","The error details about a module error that has occurred."],["PairSigner","A [`Signer`] implementation that can be constructed from an [`Pair`]."],["PalletMetadata","Metadata for a specific pallet."],["RawEventDetails","The raw bytes for an event with associated details about where and when it was emitted."],["RpcClient","Generic asyncronous client."],["RuntimeError","This is used in the place of the `E` in [`GenericError<E>`] when we may have a Runtime Error. We use this wrapper so that it is possible to implement `From<Error<Infallible>` for `Error<RuntimeError<E>>`."],["SubmittableExtrinsic","A constructed call ready to be signed and submitted."],["TransactionEvents","This represents the events related to our transaction. We can iterate over the events, or look for a specific one."],["TransactionInBlock","This struct represents a transaction that has made it into a block."],["TransactionProgress","This struct represents a subscription to the progress of some transaction, and is returned from [`crate::SubmittableExtrinsic::sign_and_submit_then_watch()`]."],["WrapperKeepOpaque","A wrapper for any type `T` which implement encode/decode in a way compatible with `Vec<u8>`."]],"trait":[["Call","Call trait."],["Config","Runtime types."],["Event","Event trait."],["HasModuleError","This trait is automatically implemented for the generated `DispatchError`, so that we can pluck out information about the `Module` error variant, if` it exists."]],"type":[["BasicError","An error that will never contain a runtime error."],["Error","An error that may contain some runtime error `E`"],["KeyedVec","This represents a key-value collection and is SCALE compatible with collections like BTreeMap. This has the same type params as `BTreeMap` which allows us to easily swap the two during codegen."],["PolkadotExtrinsicParams","A struct representing the signed extra and additional parameters required to construct a transaction for a polkadot node."],["PolkadotExtrinsicParamsBuilder","A builder which leads to [`PolkadotExtrinsicParams`] being constructed. This is what you provide to methods like `sign_and_submit()`."],["SubstrateExtrinsicParams","A struct representing the signed extra and additional parameters required to construct a transaction for the default substrate node."],["SubstrateExtrinsicParamsBuilder","A builder which leads to [`SubstrateExtrinsicParams`] being constructed. This is what you provide to methods like `sign_and_submit()`."]]});