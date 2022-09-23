# tidext API

The Node.js package `tidext` exports two main classes: [Builder](#builder) and [Client](#client).

## Builder

### constructor(substrateNodeUrl: string, storagePath: string, storagePassword: string)

Constructs a new Builder instance, preparing a Client to use the given node URL,
using the Stronghold snapshot (`.stronghold` file) on the specified path with its password.

If the Stronghold snapshot does not exist in the provided path, a new seed will be generated and saved to that path.

### async build(): [Client](#client)

Loads the Stronghold snapshot if it exists, and returns a new [Client](#client) instance if the connection can be established.

## Client

### async systemHealth(): [NodeHealth](#nodehealth)

Returns the system health. Useful for health checks and heartbeats to keep the connection alive.

### getAccountId(): Buffer

Returns the account id associated with the Stronghold snapshot.

### getAccountIdSs58(): string

Returns the account ID representation with the [SS58 Address Format](https://docs.substrate.io/v3/advanced/ss58/).

### async getRegularSwapFee(): number

Gets the swap percentage fee registered on-chain for a regular user.

### async getMarketMakerSwapFee(): number

Gets the swap percentage fee registered on-chain for a market maker.

### async extrinsicCost(): number

Gets the cost (gas fee) of the extrinsic on-chain. It is always in TDFY.

### async submitSignedExtrinsic(extrinsic: string): string

Submits an extrinsic. You can generate one using the [unstakeExtrinsic], [stakeExtrinsic], [swapExtrinsic], [cancelSwapExtrinsic], [transferExtrinsic] and [withdrawalExtrinsic] APIs.

[unstakeextrinsic]: #async-unstakeextrinsicstakeid-Buffer-forceunstake-boolean-string
[stakeextrinsic]: #async-stakeextrinsictokenId-number-amount-number-duration-number-string
[swapextrinsic]: #async-swapextrinsicfromTokenId-number-toTokenId-number-fromamount-number-toamount-number-swaptype-slippagetolerance-number-string
[cancelswapextrinsic]: #async-cancelswapextrinsicrequestId-string-string
[transferextrinsic]: #async-transferextrinsictokenId-number-amount-number-destination-Buffer-string
[withdrawalextrinsic]: #async-withdrawalextrinsictokenId-number-amount-number-externalAddress-Buffer-string

Returns a hash that can be used later to cancel a swap or unstake.

### async unstakeExtrinsic(stakeId: Buffer, forceUnstake: boolean): string

Generates an unstake extrinsic associated with the given stake id (the return value of [submitSignedExtrinsic](#async-submitsignedextrinsicextrinsic-string-string)). Note that you must [submit the extrinsic](#async-submitsignedextrinsicextrinsic-string-string).

### async stakeExtrinsic(tokenId: number, amount: number, duration: number): string

Generates a stake extrinsic with the given token, amount and duration. Note that you must [submit the extrinsic](#async-submitsignedextrinsicextrinsic-string-string).

### async swapExtrinsic(fromTokenId: number, toTokenId: number, fromAmount: number, toAmount: number, swapType, slippageTolerance: number): string

Generates a swap extrinsic with the given from and to values. Note that you must [submit the extrinsic](#async-submitsignedextrinsicextrinsic-string-string).

> : This API does not work yet, the `swapType` argument isn't implemented.

### async cancelSwapExtrinsic(requestId: string): string

Generates a cancel swap extrinsic for the given requestId hex string. Note that you must [submit the extrinsic](#async-submitsignedextrinsicextrinsic-string-string).

### async transferExtrinsic(tokenId: number, amount: number, destination: Buffer): string

Generates an extrinsic to send _amount_ of the given _tokenId_ to the _destination_ account id. Note that you must [submit the extrinsic](#async-submitsignedextrinsicextrinsic-string-string).

### async balance(tokenId: number, accountId: Buffer | None): [Balance](#balance)

Returns the balance information for the given _tokenId_ of the account id (defaults to the Stronghold account).

### async totalStakeFor(tokenId: number): number

Returns the total staked amount of the given token.

### async totalSupplyFor(tokenId: number): number

Returns the total supply of the given token.

**Wrapped token** are minted when deposited and burned when withdrawed, the total supply represent the total tokens deposited in chain and not withdrawed.
Total issuance for **TiDE token**, represent the number of tokens issued by the runtime.

### async allAssets(): list\[[Currency](#currency)\]

Returns information for all assets registered on chain.

### async withdrawal(tokenId: number, amount: number, externalAddress: Buffer)

Requests a withdrawal of the specified amount of the token, depositing to the provided external address.

### async withdrawalExtrinsic(tokenId: number, amount: number, externalAddress: Buffer): string

Generates an extrinsic to withdrawal the specified amount of the token. Note that you must [submit the extrinsic](#async-submitsignedextrinsicextrinsic-string-string).

## NodeHealth

An object with the following properties:

- peers: number
- isSyncing: boolean
- shouldHavePeers: boolean

## Balance

An object with the following properties:

- available: number
- reserved: number

## Currency

An object with the following properties:

- tokenId: number
- metadata: [CurrencyMetadata](#currencymetadata)

### CurrencyMetadata

An object with the following properties:

- name: string
- symbol: string
- decimals: string
- isFrozen: string
