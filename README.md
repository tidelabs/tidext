# tidext

A library to submit **tifi** e**xt**rinsics to a [tidechain](https://github.com/tidelabs/tidechain) node via RPC.

_This project is currently under active development_.

## Usage

Take a look in the [examples](./examples/examples) folder for various `tidext` usage examples.

```
cargo run --example local
```

### Initializing the API client

```rust
let client = ClientBuilder::new()
   .set_signer(signer)
   .build()
   .await?;

client.total_supply_for(CurrencyId::Tdfy).await?;
```

### Submitting Extrinsics

Submit an extrinsic, returning success once the transaction is accepted into the pool:

```rust
client.swap(
  CurrencyId::Tdfy,
  1_000_000_000_000,
  CurrencyId::Wrapped(4),
  1_000_000,
  SwapType::Limit,
  None,
).await?;
```

## Documentation

The package is not published to crates.io yet, but you can find the documentation [here](https://tidelabs.github.io/tidext/).

## Generate node metadata

```
cargo install --git https://github.com/tidelabs/subxt --branch=tidechain --force
subxt metadata > tidext/res/tidechain_metadata.scale --format bytes
```

## Integration Testing

Most tests require a running tidechain node to communicate with. This is done by spawning an instance of the
substrate node per test.

```
TIDECHAIN_NODE_PATH=~/.bin/tidechain cargo test
```

#### License

<sup>
The entire code within this repository is licensed under the <a href="LICENSE">GPLv3</a>.
</sup>
