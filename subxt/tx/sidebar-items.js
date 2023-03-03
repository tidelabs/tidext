window.SIDEBAR_ITEMS = {"enum":[["Era","An era to describe the longevity of a transaction."],["TxStatus","Possible transaction statuses returned from our [`TxProgress::next_item()`] call."]],"fn":[["dynamic","Construct a new dynamic transaction payload to submit to a node."]],"struct":[["AssetTip","A tip payment made in the form of a specific asset."],["BaseExtrinsicParams","An implementation of [`ExtrinsicParams`] that is suitable for constructing extrinsics that can be sent to a node with the same signed extra and additional parameters as a Polkadot/Substrate node. The way that tip payments are specified differs between Substrate and Polkadot nodes, and so we are generic over that in order to support both here with relative ease."],["BaseExtrinsicParamsBuilder","This builder allows you to provide the parameters that can be configured in order to construct a [`BaseExtrinsicParams`] value. This implements [`Default`], which allows [`BaseExtrinsicParams`] to be used with convenience methods like `sign_and_submit_default()`."],["DynamicTxPayload","This represents a dynamically generated transaction payload."],["PairSigner","A [`Signer`] implementation that can be constructed from an [`Pair`]."],["PlainTip","A tip payment."],["StaticTxPayload","This represents a statically generated transaction payload."],["SubmittableExtrinsic","This represents an extrinsic that has been signed and is ready to submit."],["TxClient","A client for working with transactions."],["TxEvents","This represents the events related to our transaction. We can iterate over the events, or look for a specific one."],["TxInBlock","This struct represents a transaction that has made it into a block."],["TxProgress","This struct represents a subscription to the progress of some transaction."]],"trait":[["ExtrinsicParams","This trait allows you to configure the “signed extra” and “additional” parameters that are signed and used in transactions. see [`BaseExtrinsicParams`] for an implementation that is compatible with a Polkadot node."],["Signer","Signing transactions requires a [`Signer`]. This is responsible for providing the “from” account that the transaction is being signed by, as well as actually signing a SCALE encoded payload. Optionally, a signer can also provide the nonce for the transaction to use."],["TxPayload","This represents a transaction payload that can be submitted to a node."]],"type":[["PolkadotExtrinsicParams","A struct representing the signed extra and additional parameters required to construct a transaction for a polkadot node."],["PolkadotExtrinsicParamsBuilder","A builder which leads to [`PolkadotExtrinsicParams`] being constructed. This is what you provide to methods like `sign_and_submit()`."],["SubstrateExtrinsicParams","A struct representing the signed extra and additional parameters required to construct a transaction for the default substrate node."],["SubstrateExtrinsicParamsBuilder","A builder which leads to [`SubstrateExtrinsicParams`] being constructed. This is what you provide to methods like `sign_and_submit()`."]]};