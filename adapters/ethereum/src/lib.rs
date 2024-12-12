use alloy::{
	primitives::Address,
	providers::{Provider, ProviderBuilder, RootProvider},
	transports::http::{Client, Http},
};
use chainsmith_networks::{ethereum::Config, Config as NetworkConfig, Network, OnchainRpcProvider};
use chainsmith_primitives::{Balance, HexString};
use eyre::{Result, WrapErr};
use tracing::info;

pub struct Ethereum;

impl Network for Ethereum {
	type Config = Config;
	type Provider = EthereumRpcProvider;
}

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

	async fn get_balance(&self, address: HexString) -> Result<Option<Balance>> {
		let address =
			Address::parse_checksummed(address, None).wrap_err("Failed to get balance")?;
		Ok(self.inner.get_balance(address).await.map(|b| Some(b)).unwrap_or(None))
	}

	async fn get_transaction(
		&self,
		tx_hash: <Config as NetworkConfig>::GetTxParam,
	) -> Result<Option<<Config as NetworkConfig>::TxType>> {
		Ok(self.inner.get_transaction_by_hash(tx_hash).await?)
	}
}
