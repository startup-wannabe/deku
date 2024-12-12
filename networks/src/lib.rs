use std::fmt::Debug;

use chainsmith_primitives::{Balance, BlockNumber, HexString, Result};

pub mod ethereum;
pub mod solana;
pub mod substrate;

pub trait Network {
	type Config: Config;
	type Provider: OnchainRpcProvider<Self::Config>;
}

pub trait Config {
	type TxType: Clone + Debug + PartialEq;
	type GetTxParam;
}

#[allow(async_fn_in_trait)]
pub trait OnchainRpcProvider<N: Config> {
	async fn new(url: &str) -> Result<Self>
	where
		Self: Sized;

	async fn get_block_number(&self) -> Result<BlockNumber>;

	async fn get_balance(&self, address: HexString) -> Result<Option<Balance>>;

	async fn get_transaction(&self, signature: N::GetTxParam) -> Result<Option<N::TxType>>;
}
