use deku_primitives::OnchainRpcProvider;
use eyre::{eyre, Result, WrapErr};
use subxt::{
	backend::{legacy::LegacyRpcMethods, rpc::RpcClient},
	SubstrateConfig,
};
use tracing::info;

pub struct SubstrateRpcProvider {
	inner: RpcClient,
}

impl SubstrateRpcProvider {
	pub async fn new(url: &str) -> Result<Self> {
		let client = RpcClient::from_url(url)
			.await
			.wrap_err("Failed to initialize the RPC provider")?;
		Ok(Self { inner: client })
	}
}

impl OnchainRpcProvider for SubstrateRpcProvider {
	async fn get_block_number(&self) -> Result<u64> {
		info!(method = "get_block_number");
		let rpc_methods = LegacyRpcMethods::<SubstrateConfig>::new(self.inner.clone());
		let header = rpc_methods
			.chain_get_header(None)
			.await
			.wrap_err("Failed to get block number")?;
		match header.map(|h| h.number) {
			Some(block_number) => Ok(block_number.into()),
			None => Err(eyre!("No block number found")),
		}
	}
}
