use alloy::{
	providers::{Provider, ProviderBuilder, RootProvider},
	transports::http::{Client, Http},
};
use deku_primitives::OnchainRpcProvider;
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

impl OnchainRpcProvider for EthereumRpcProvider {
	async fn get_block_number(&self) -> Result<u64> {
		info!(method = "get_block_number");
		self.inner.get_block_number().await.wrap_err("Failed to get block number")
	}
}
