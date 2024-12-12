<img src="https://github.com/user-attachments/assets/d2388e57-eec8-45f0-bf78-0b121d9d0554" width="80px"/>

# chainsmith-rs

`chainsmith-rs` is a high-performance chain-agnostic data orchestration Rust library designed to empower Web3.0 by simplifying the interaction with on-chain data.

Just like a blacksmith crafts tools with precision and care, ChainSmith enables developers to seamlessly and effortlessly work with data across multiple blockchains, abstracting away the complexities of multichain interactions.

Whether you're building decentralized applications or integrating cross-chain functionality, ChainSmith provides a powerful, efficient, and scalable solution to aggregate, process, and manage on-chain data with ease.

## Architecture

- `chainsmith-sdk`: Core software development kit used for applications on top of the Deku data infrastructure to interact with multichain data.
- `chainsmith-networks`: Container crate of all blockchain network configuration.
- `chainsmith-adapters/*`: Contains all data adapter used in the Deku SDK.
- `chainsmith-primitvies`: Primitive types used by the adapters and the SDK.
- `chainsmith-server`: Middleware backend to help applications work with the SDK via RESTful APIs.

## Features

- [ ] Chain abstraction SDK library to query data from multiple different blockchains without worrying about the inconsistent in RPC APIs and data types.
- [ ] Orchestration functionality.

## Usage

```rs
// Using a generic data provider.
let provider = DataProvider::chain("<CHAIN_NAME>").rpc("<RPC_URL>").await.unwrap();

// Using raw provider for onchain data.
let raw_provider = SubstrateRpcProvider::new("<RPC_URL>").await.unwrap();
```

- `<CHAIN_NAME>` = `solana` | `substrate` | `ethereum`
- `<RPC_URL>` = `SOLANA_HTTPS_URL` | `SUBSTRATE_HTTPS_URL` | `ETHEREUM_HTTPS_URL`
