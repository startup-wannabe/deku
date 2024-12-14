use chainsmith_networks::{Config, Network, OnchainRpcProvider};
use chainsmith_primitives::Balance;
use tracing::info;

use crate::*;

pub struct RpcProvider<N: Network> {
	provider: N::Provider,
}

impl<N: Network + 'static> OnchainRpcProvider<N::Config> for RpcProvider<N> {
	async fn new(url: &str) -> Result<RpcProvider<N>> {
		info!("Initialization with url: {:?}", url);
		let provider = N::Provider::new(&url).await?;
		Ok(RpcProvider { provider })
	}

	async fn get_block_number(&self) -> Result<u64> {
		self.provider.get_block_number().await
	}

	async fn get_balance(&self, address: Address) -> Result<Option<Balance>> {
		self.provider.get_balance(address).await
	}

	async fn get_transaction(
		&self,
		param: <N::Config as Config>::TransactionQuery,
	) -> Result<Option<<N::Config as Config>::Transaction>> {
		self.provider.get_transaction(param).await
	}

	async fn get_account(
		&self,
		address: Address,
	) -> Result<Option<<N::Config as Config>::AccountData>> {
		self.provider.get_account(address).await
	}

	async fn get_block_by_number(
		&self,
		block_number: BlockNumber,
	) -> Result<Option<<N::Config as Config>::BlockData>> {
		self.provider.get_block_by_number(block_number).await
	}
}
