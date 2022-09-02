# tidext API

The Python package `tidext` exports two main classes: [Builder](#builder) and [Client](#client).

## Builder

### constructor(substrateNodeUrl: str, storagePath: str, storagePassword: str)

Constructs a new Builder instance, preparing a Client to use the given node URL,
using the Stronghold snapshot (`.stronghold` file) on the specified path with its password.

If the Stronghold snapshot does not exist in the provided path, a new seed will be generated and saved to that path.

### async build(): [Client](#client)

Loads the Stronghold snapshot if it exists, and returns a new [Client](#client) instance if the connection can be established.

## Client

### async system_health(): [NodeHealth](#nodehealth)

Returns the system health. Useful for health checks and heartbeats to keep the connection alive.

### get_account_id(): bytearray

Returns the account id associated with the Stronghold snapshot.

### get_account_id_ss58(): str

Returns the account ID representation with the [SS58 Address Format](https://docs.substrate.io/v3/advanced/ss58/).

### async get_regular_swap_fee(): float

Gets the swap percentage fee registered on-chain for a regular user.

### async get_market_maker_swap_fee(): float

Gets the swap percentage fee registered on-chain for a market maker.

### async extrinsic_cost(): int

Gets the cost (gas fee) of the extrinsic on-chain. It is always in TDFY.

### async submit_signed_extrinsic(extrinsic: str): bytearray

Submits an extrinsic. You can generate one using the [unstake_extrinsic], [stake_extrinsic], [swap_extrinsic], [cancel_swap_extrinsic], [transfer_extrinsic] and [withdrawal_extrinsic] APIs.

[unstake_extrinsic]: #async-unstake_extrinsicstake_id-bytearray-force_unstake-bool-str
[stake_extrinsic]: #async-stake_extrinsictoken_id-int-amount-int-duration-int-str
[swap_extrinsic]: #async-swap_extrinsicfrom_token_id-int-to_token_id-int-from_amount-int-to_amount-int-swap_type-slippage_tolerance-int-str
[cancel_swap_extrinsic]: #async-cancel_swap_extrinsicrequest_id-bytearray-str
[transfer_extrinsic]: #async-transfer_extrinsictoken_id-int-amount-int-destination-bytearray-str
[withdrawal_extrinsic]: #async-withdrawal_extrinsictoken_id-int-amount-int-external_address-bytearray

Returns a hash that can be used later to cancel a swap or unstake.

### async unstake_extrinsic(stake_id: bytearray, force_unstake: bool): str

Generates an unstake extrinsic associated with the given stake id (the return value of [submit_signed_extrinsic](#async-submit_signed_extrinsicextrinsic-str-bytearray)). Note that you must [submit the extrinsic](#async-submit_signed_extrinsicextrinsic-str-bytearray).

### async stake_extrinsic(token_id: int, amount: int, duration: int): str

Generates a stake extrinsic with the given token, amount and duration. Note that you must [submit the extrinsic](#async-submit_signed_extrinsicextrinsic-str-bytearray).

### async swap_extrinsic(from_token_id: int, to_token_id: int, from_amount: int, to_amount: int, swap_type, slippage_tolerance: int): str

Generates a swap extrinsic with the given from and to values. Note that you must [submit the extrinsic](#async-submit_signed_extrinsicextrinsic-str-bytearray).

> : This API does not work yet, the `swap_type` argument wasn't implemented.

### async cancel_swap_extrinsic(request_id: bytearray): str

Generates a cancel swap extrinsic for the given requestId returned by [submit_signed_extrinsic](#async-submit_signed_extrinsicextrinsic-str-bytearray). Note that you must [submit the extrinsic](#async-submit_signed_extrinsicextrinsic-str-bytearray).

### async transfer_extrinsic(token_id: int, amount: int, destination: bytearray): str

Generates an extrinsic to send _amount_ of the given _token_id_ to the _destination_ account id. Note that you must [submit the extrinsic](#async-submit_signed_extrinsicextrinsic-str-bytearray).

### async balance(token_id: int, account_id: bytearray | None): [Balance](#balance)

Returns the balance information for the given _token_id_ of the account id (defaults to the Stronghold account).

### async total_stake_for(token_id: int): int

Returns the total staked amount of the given token.

### async total_supply_for(token_id: int): int

Returns the total supply of the given token.

**Wrapped token** are minted when deposited and burned when withdrawed, the total supply represent the total tokens deposited in chain and not withdrawed.
Total issuance for **TiDE token**, represent the number of tokens issued by the runtime.

### async all_assets(): list\[[Currency](#currency)\]

Returns information for all assets registered on chain.

### async withdrawal(token_id: int, amount: int, external_address: bytearray)

Requests a withdrawal of the specified amount of the token, depositing to the provided external address.

### async withdrawal_extrinsic(token_id: int, amount: int, external_address: bytearray)

Generates an extrinsic to withdrawal the specified amount of the token. Note that you must [submit the extrinsic](#async-submit_signed_extrinsicextrinsic-str-bytearray).

## NodeHealth

An object with the following properties:

- peers: int
- is_syncing: bool
- should_have_peers: bool

## Balance

An object with the following properties:

- available: int
- reserved: int

## Currency

An object with the following properties:

- token_id: int
- metadata: [CurrencyMetadata](#currencymetadata)

### CurrencyMetadata

An object with the following properties:

- name: str
- symbol: str
- decimals: str
- is_frozen: str
