use std::str::FromStr;

use chainsmith_networks::{solana::Config, Config as NetworkConfig, Network, OnchainRpcProvider};
use chainsmith_primitives::{Balance, HexString, Uint};
use eyre::{Result, WrapErr};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use solana_transaction_status::UiTransactionEncoding;
use tracing::info;

pub struct Solana;

impl Network for Solana {
	type Config = Config;
	type Provider = SolanaRpcProvider;
}

pub struct SolanaRpcProvider {
	inner: RpcClient,
}

impl OnchainRpcProvider<Config> for SolanaRpcProvider {
	async fn new(url: &str) -> Result<Self> {
		let url = url.to_string();
		let client = RpcClient::new(url);
		Ok(Self { inner: client })
	}

	async fn get_block_number(&self) -> Result<u64> {
		info!(method = "get_block_number");
		self.inner.get_block_height().await.wrap_err("Failed to get block number")
	}

	async fn get_balance(&self, address: HexString) -> Result<Option<Balance>> {
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
		signature: <Config as NetworkConfig>::GetTxParam,
	) -> Result<Option<<Config as NetworkConfig>::TxType>> {
		let tx = self
			.inner
			.get_transaction(&signature, UiTransactionEncoding::Json)
			.await
			.map(|tx| Some(tx.transaction))
			.unwrap_or(None);
		Ok(tx)
	}
}
