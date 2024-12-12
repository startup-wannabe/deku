use std::fmt::Debug;

use chainsmith_primitives::{Balance, BlockNumber, HexString, Result};
use ethereum::Ethereum;

pub mod ethereum;
pub mod solana;
pub mod substrate;

pub trait Network {
	type TxType: Clone + Debug + PartialEq;
	type GetTxParam;
}

#[allow(async_fn_in_trait)]
pub trait OnchainRpcProvider<N: Network = Ethereum> {
	async fn get_block_number(&self) -> Result<BlockNumber>;

	async fn get_balance(&self, address: HexString) -> Result<Option<Balance>>;

	async fn get_transaction(&self, signature: N::GetTxParam) -> Result<Option<N::TxType>>;
}
