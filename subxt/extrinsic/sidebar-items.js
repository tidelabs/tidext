initSidebarItems({"enum":[["Era","An era to describe the longevity of a transaction."]],"struct":[["AssetTip","A tip payment made in the form of a specific asset."],["BaseExtrinsicParams","An implementation of [`ExtrinsicParams`] that is suitable for constructing extrinsics that can be sent to a node with the same signed extra and additional parameters as a Polkadot/Substrate node. The way that tip payments are specified differs between Substrate and Polkadot nodes, and so we are generic over that in order to support both here with relative ease."],["BaseExtrinsicParamsBuilder","This builder allows you to provide the parameters that can be configured in order to construct a [`BaseExtrinsicParams`] value. This implements [`Default`], which allows [`BaseExtrinsicParams`] to be used with convenience methods like `sign_and_submit_default()`."],["PairSigner","A [`Signer`] implementation that can be constructed from an [`Pair`]."],["PlainTip","A tip payment."]],"trait":[["ExtrinsicParams","This trait allows you to configure the “signed extra” and “additional” parameters that are signed and used in transactions. see [`BaseExtrinsicParams`] for an implementation that is compatible with a Polkadot node."],["Signer","Signing transactions requires a [`Signer`]. This is responsible for providing the “from” account that the transaction is being signed by, as well as actually signing a SCALE encoded payload. Optionally, a signer can also provide the nonce for the transaction to use."]],"type":[["PolkadotExtrinsicParams","A struct representing the signed extra and additional parameters required to construct a transaction for a polkadot node."],["PolkadotExtrinsicParamsBuilder","A builder which leads to [`PolkadotExtrinsicParams`] being constructed. This is what you provide to methods like `sign_and_submit()`."],["SubstrateExtrinsicParams","A struct representing the signed extra and additional parameters required to construct a transaction for the default substrate node."],["SubstrateExtrinsicParamsBuilder","A builder which leads to [`SubstrateExtrinsicParams`] being constructed. This is what you provide to methods like `sign_and_submit()`."]]});