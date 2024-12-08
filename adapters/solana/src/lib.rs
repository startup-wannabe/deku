use deku_primitives::OnchainRpcProvider;
use eyre::{Result, WrapErr};
use solana_client::rpc_client::RpcClient;
use tracing::info;

pub struct SolanaRpcProvider {
	inner: RpcClient,
}

impl SolanaRpcProvider {
	pub fn new(url: &str) -> Result<Self> {
		let url = url.to_string();
		let client = RpcClient::new(url);
		Ok(Self { inner: client })
	}
}

impl OnchainRpcProvider for SolanaRpcProvider {
	async fn get_block_number(&self) -> Result<u64> {
		info!(method = "get_block_number");
		self.inner.get_block_height().wrap_err("Failed to get block number")
	}
}
