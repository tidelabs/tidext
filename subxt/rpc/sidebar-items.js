initSidebarItems({"enum":[["JsonValue","Represents any valid JSON value."],["ListOrValue","RPC list or value wrapper."],["NumberOrHex","A number type that can be serialized both as a number or a string that encodes a number in a string."],["RpcError","Error type."],["SubstrateTransactionStatus","Possible transaction status events."]],"fn":[["to_json_value","Convert a `T` into `serde_json::Value` which is an enum that can represent any valid JSON data."],["ws_client","Build WS RPC client from URL"]],"macro":[["rpc_params","Convert the given values to a [`jsonrpsee_types::ParamsSer`] as expected by a jsonrpsee Client (http or websocket)."]],"struct":[["BlockNumber","Wrapper for NumberOrHex to allow custom From impls"],["InvalidUri","An error resulting from a failed attempt to construct a URI."],["ReadProof","ReadProof struct returned by the RPC"],["Rpc","Client for substrate rpc interfaces"],["RpcClient","Generic asyncronous client."],["RpcClientBuilder","Builder for [`Client`]."],["RuntimeVersion","This contains the runtime version information necessary to make transactions, as obtained from the RPC call `state_getRuntimeVersion`,"],["Subscription","Active subscription on the client."],["Uri","The URI component of a request."],["WsReceiver","Receiving end of WebSocket transport."],["WsSender","Sending end of WebSocket transport."],["WsTransportClientBuilder","Builder for a WebSocket transport [`Sender`] and [’Receiver`] pair."]],"trait":[["ClientT","JSON-RPC client interface that can make requests and notifications."],["DeserializeOwned","A data structure that can be deserialized without borrowing any data from the deserializer."],["SubscriptionClientT","JSON-RPC client interface that can make requests, notifications and subscriptions."]],"type":[["ChainBlock","Alias for the type of a block returned by `chain_getBlock`"],["SystemProperties","Arbitrary properties defined in the chain spec as a JSON object."]]});