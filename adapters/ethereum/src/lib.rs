use alloy::{
	eips::BlockNumberOrTag,
	primitives::Address as EthereumAddress,
	providers::{Provider, ProviderBuilder, RootProvider},
	rpc::types::BlockTransactionsKind,
	transports::http::{Client, Http},
};
use chainsmith_networks::{ethereum::Config, Config as NetworkConfig, Network, OnchainRpcProvider};
use chainsmith_primitives::{Address, Balance};
use eyre::{Result, WrapErr};
use tracing::info;

/// Configuration for the Ethereum-compatible network.
pub struct Ethereum;

impl Network for Ethereum {
	type Config = Config;
	type Provider = EthereumRpcProvider;
}

/// RPC Provider for the Ethereum-compatible network.
pub struct EthereumRpcProvider {
	inner: RootProvider<Http<Client>>,
}

impl OnchainRpcProvider<Config> for EthereumRpcProvider {
	async fn new(url: &str) -> Result<Self> {
		let parsed_rpc_url = url.parse()?;
		let provider = ProviderBuilder::new().on_http(parsed_rpc_url);
		Ok(Self { inner: provider })
	}

	async fn get_block_number(&self) -> Result<u64> {
		info!(method = "get_block_number");
		self.inner.get_block_number().await.wrap_err("Failed to get block number")
	}

	async fn get_balance(&self, address: Address) -> Result<Option<Balance>> {
		let address =
			EthereumAddress::parse_checksummed(address, None).wrap_err("Failed to get balance")?;
		Ok(self.inner.get_balance(address).await.map(|b| Some(b)).unwrap_or(None))
	}

	async fn get_transaction(
		&self,
		tx_hash: <Config as NetworkConfig>::TransactionQuery,
	) -> Result<Option<<Config as NetworkConfig>::Transaction>> {
		Ok(self.inner.get_transaction_by_hash(tx_hash).await?)
	}

	async fn get_account(
		&self,
		_address: Address,
	) -> Result<Option<<Config as NetworkConfig>::AccountData>> {
		unimplemented!()
	}

	async fn get_block_by_number(
		&self,
		block_number: <Config as NetworkConfig>::BlockQuery,
	) -> Result<Option<<Config as NetworkConfig>::BlockData>> {
		self.inner
			.get_block_by_number(
				BlockNumberOrTag::Number(block_number),
				BlockTransactionsKind::Full,
			)
			.await
			.wrap_err("Failed to get block by number")
	}
}
