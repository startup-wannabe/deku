use chainsmith_networks::{Config, Network, OnchainRpcProvider};
use chainsmith_primitives::{Balance, HexString};
use tracing::info;

use crate::*;

pub struct RpcProvider<N: Network> {
	provider: N::Provider,
}

impl<N: Network + 'static> RpcProvider<N> {
	pub async fn new(url: &str) -> Result<RpcProvider<N>> {
		info!("Initialization with url: {:?}", url);
		let provider = N::Provider::new(&url).await?;
		Ok(RpcProvider { provider })
	}

	pub async fn get_block_number(&self) -> Result<u64> {
		self.provider.get_block_number().await
	}

	pub async fn get_balance(&self, address: HexString) -> Result<Option<Balance>> {
		self.provider.get_balance(address).await
	}

	pub async fn get_transaction(
		&self,
		param: <N::Config as Config>::GetTxParam,
	) -> Result<Option<<N::Config as Config>::TxType>> {
		self.provider.get_transaction(param).await
	}
}
