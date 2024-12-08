use std::str::FromStr;

use deku_primitives::{Balance, HexString, OnchainRpcProvider, Uint};
use eyre::{Result, WrapErr};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
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
	async fn get_latest_block_number(&self) -> Result<u64> {
		info!(method = "get_block_number");
		self.inner.get_block_height().await.wrap_err("Failed to get block number")
	}

	async fn get_balance(&self, address: HexString) -> Result<Balance> {
		let pubkey = Pubkey::from_str(&address).wrap_err("Failed to parse string")?;
		let balance = self.inner.get_balance(&pubkey).await.wrap_err("Failed to get balance")?;
		Ok(Uint::from(balance))
	}
}
