use std::fmt::Debug;

use chainsmith_primitives::{Address, Balance, BlockNumber, Result};
use ethereum::Ethereum;

pub mod ethereum;
pub mod solana;
pub mod substrate;

pub trait Network {
	const CHAIN: &str;
	type TxType: Clone + Debug + PartialEq;
	type TxSignature;
}

#[allow(async_fn_in_trait)]
pub trait OnchainRpcProvider<N: Network = Ethereum> {
	async fn get_block_number(&self) -> Result<BlockNumber>;

	async fn get_balance(&self, address: Address) -> Result<Option<Balance>>;

	async fn get_transaction(&self, signature: N::TxSignature) -> Result<Option<N::TxType>>;
}
