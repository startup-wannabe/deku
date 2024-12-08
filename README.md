# Deku v0.0.1

Chain agnostic data orchestration library that empowers Web3.0 to work with onchain data seamlessly and effortlessly in Rust.

## Architecture

- `deku-sdk`: Core software development kit used for applications on top of the Deku data infrastructure to interact with multichain data.
- `deku-adapters/*`: Contains all data adapter used in the Deku SDK.
- `deku-primitvies`: Primitive types used by the adapters and the SDK.
- `deku-server`: Middleware backend to help applications work with the SDK via RESTful APIs.

## Usage

```rs
// Using a generic data provider.
let provider = DataProvider::chain("<CHAIN_NAME>").rpc("<RPC_URL>").await.unwrap();

// Using raw provider for onchain data.
let raw_provider = SubstrateRpcProvider::new("<RPC_URL>").await.unwrap();
```

- `<CHAIN_NAME>` = `solana` | `substrate` | `ethereum`
- `<RPC_URL>` = `SOLANA_HTTPS_URL` | `SUBSTRATE_HTTPS_URL` | `ETHEREUM_HTTPS_URL`
