use std::str::FromStr;

use chainsmith_networks::{solana::Solana, Network, OnchainRpcProvider};
use chainsmith_primitives::{Address, Balance, Uint};
use eyre::{Result, WrapErr};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use solana_transaction_status::UiTransactionEncoding;
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

impl OnchainRpcProvider<Solana> for SolanaRpcProvider {
	async fn get_block_number(&self) -> Result<u64> {
		info!(method = "get_block_number");
		self.inner.get_block_height().await.wrap_err("Failed to get block number")
	}

	async fn get_balance(&self, address: Address) -> Result<Option<Balance>> {
		let pubkey = Pubkey::from_str(&address).wrap_err("Failed to parse string")?;
		Ok(self
			.inner
			.get_balance(&pubkey)
			.await
			.map(|b| Some(Uint::from(b)))
			.unwrap_or(None))
	}

	async fn get_transaction(
		&self,
		signature: <Solana as Network>::TxSignature,
	) -> Result<Option<<Solana as Network>::TxType>> {
		let tx = self
			.inner
			.get_transaction(&signature, UiTransactionEncoding::Json)
			.await
			.map(|tx| Some(tx.transaction))
			.unwrap_or(None);
		Ok(tx)
	}
}
