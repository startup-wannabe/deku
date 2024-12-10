use alloy::{
	primitives::Address,
	providers::{Provider, ProviderBuilder, RootProvider},
	transports::http::{Client, Http},
};
use deku_networks::{ethereum::Ethereum, Network, OnchainRpcProvider};
use deku_primitives::{Balance, HexString};
use eyre::{Result, WrapErr};
use tracing::info;

pub struct EthereumRpcProvider {
	inner: RootProvider<Http<Client>>,
}

impl EthereumRpcProvider {
	pub fn new(url: &str) -> Result<Self> {
		let parsed_rpc_url = url.parse()?;
		let provider = ProviderBuilder::new().on_http(parsed_rpc_url);
		Ok(Self { inner: provider })
	}
}

impl OnchainRpcProvider<Ethereum> for EthereumRpcProvider {
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
		tx_hash: <Ethereum as Network>::GetTxParam,
	) -> Result<Option<<Ethereum as Network>::TxType>> {
		Ok(self.inner.get_transaction_by_hash(tx_hash).await?)
	}
}
