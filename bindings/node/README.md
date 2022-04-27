# tidext for Node.js

This is the Node.js binding for the tidext library.

## Requirements

- Rust 1.57 or higher
- Node.js 10.0.0 or higher

Rust can be installed with [rustup](https://rustup.rs/).
Node.js can be installed with [NVM](https://github.com/nvm-sh/nvm) or the [official website](https://nodejs.org/).

## Usage

Currently this binding is not published to npm, so you need to build it from source.

```bash
yarn
yarn build
```

Then you can load it in your JavaScript file and run with Node.js:

```javascript
const { Builder } = require('/path/to/tidext/bindings/node')

const substrateUrl = '...'
const strongholdPath = '...'
const strongholdPassword = '...'
const builder = new Builder(substrateUrl, strongholdPath, strongholdPassword)
const client = await builder.build()
console.log(await client.allAssets())
```

## API

See the [API documentation](./api.md).
