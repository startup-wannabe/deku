# CHANGELOG

## 12nd December, 2024

### Rebranding

> Authors: @chungquantin

Changing name of the repository from `deku` to `chainsmith`. There is an existing `deku` crate on the registry for binary deserialization. Second reason would be `chainsmith` is a better name for a project like this:

```rs
blockchain (chain) + blocksmith (smith) = chainsmith
```

## Network Configuration

Refactoring the network trait.

### Issue with the Substrate-based blockchains

> Authors: @chungquantin

Substrate brings a breaking change to the abstraction design of `OnchainRpcProvider` due to its limit in getting transaction data. Due to the fact that Substrate is designed to be agnostic and can be used to build blockchains, transaction types can be different across Substrate-based blockchains.

- Second point would be, Substrate's transaction hash can be duplicate across blocks. Reason for that is the transaction hash is calculated by hasing the hexadeciamal value of extrinsics.

- Related SE's discussion: https://stackoverflow.com/questions/63685722/how-to-query-for-polkadot-transaction-info-using-only-txhash

- Solution that most Substrate-based blockchain use is: https://github.com/paritytech/substrate-archive but I don't think this is a right approach for a high-level library.
- How the Substrate's transactions can be possibly index? Extrinsics are put inside the block body as a list, so hash of extrinsic can be defined by `hash(block_hash, extrinsic_index)` and then store these hashes to another database for querying later.

## 10th December, 2024

### Overview

> Authors: @chungquantin

Finished the initial structure of the chain agnostic SDK. This changelog section captures everything since the beginning. Below is the breakdown of the initial architecture:

- `deku-sdk`: Core software development kit used for applications on top of the Deku data infrastructure to interact with multichain data.
- `deku-networks`: Container crate of all blockchain network configuration.
- `deku-adapters/*`: Contains all data adapter used in the Deku SDK.
- `deku-primitvies`: Primitive types used by the adapters and the SDK.
- `deku-server`: Middleware backend to help applications work with the SDK via RESTful APIs.

### Query data with the `deku-sdk`

> Authors: @chungquantin

Initialize an abstract data provider that abstract away the types & RPC APIs between chains.

```rs
let provider = DataProvider::default().rpc::<Solana>(SOLANA_HTTPS_URL).await.unwrap();
```

Initialize a chain-specific provider. Some functions are specific to the raw provider. You can consider using this when need more customizability.

```rs
let solana_provider = SolanaRpcProvider::new(SOLANA_HTTPS_URL).unwrap();
let substrate_provider = SubstrateRpcProvider::new(SOLANA_HTTPS_URL).unwrap();
let ethereum_provider = EthereumRpcProvider::new(SOLANA_HTTPS_URL).unwrap();
```

Querying data with both providers.

```rs
assert!(provider.get_block_number().await.unwrap() > 0);
assert!(raw_provider.get_block_number().await.unwrap() > 0);
```

### Chain Configuration

> Authors: @chungquantin

All network configurations will be located under [deku-networks](/networks/). Every network configuration is defined by the struct implementing trait `Network`. For example, below is the chain configuration of `Ethereum`.

```rs
use alloy::{primitives::TxHash, rpc::types::Transaction};

use crate::Network;

/// Types for a mainnet-like Ethereum network.
#[derive(Clone, Copy, Debug)]
pub struct Ethereum {
	_private: (),
}

impl Network for Ethereum {
	type GetTxParam = TxHash;
	type TxType = Transaction;
}
```
