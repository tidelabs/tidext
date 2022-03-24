# Substrate Client for Python

This is the Python binding for the TIDE Substrate client.

## Requirements

- Rust
- Python 3.7 or higher

Rust can be installed with [rustup](https://rustup.rs/).
Python can be installed with your package manager or the [official website](https://www.python.org/downloads/).

> : Currently PyO3 has an issue loading the dynamic library, so please remove the Node.js artifacts if you built the Node.js binding earlier: rm ../../target/release/libnodejs_tidext.\*

## Usage

Currently this binding is not published to PyPI, so you need to build it from source.
Create a virtual environment manually and install [maturin](https://github.com/PyO3/maturin):

```bash
$ python -m venv .venv
$ source .venv/bin/activate
$ pip install maturin
$ maturin develop
$ python -m asyncio
>>> import tidext,os
>>> builder = tidext.Builder(os.getenv('SUBSTRATE_URL'), os.getenv('STRONGHOLD_PATH'), os.getenv('STRONGHOLD_PASSWORD'))
>>> client = await builder.build()
```

## API

See the [API documentation](./api.md).

## Development

The binding can be built and tested with [tox](https://tox.wiki/en/latest/index.html).

The tests uses the following environment variables:

- SUBSTRATE_URL: the address of the Substrate RPC e.g. "wss://rpc.sandbox.tidefi.io:443"
- STRONGHOLD_PATH: the path to the Stronghold snapshot to use e.g. "/Users/tide/Library/Application\ Support/com.semnet.tidefi-mm/identities/Default.stronghold"
- STRONGHOLD_PASSWORD: the password of the Stronghold snapshot e.g. "tidefi123!@"

You can use dotenv to set up these environment variables, just run `$ cp .env.example .env` and fill the .env file with the values to use.

To build and test the package, run the following script:

```bash
$ python -m pip install tox
$ tox
```
