// TODO: Process any Rust file to get the functions
//
// TODO: Feature gating
//
// TODO: Adapter pattern factory for data source provider
//
// TODO: Construct multiple different SDK to query the data

use deku_adapter_solana::SolanaRpcProvider;
use deku_primitives::OnchainRpcProvider;
use eyre::{eyre, Result};
use rpc::RpcProvider;

pub mod rpc;

const SUBSTRATE_HTTPS_URL: &str = "wss://rpc-asset-hub-polkadot.luckyfriday.io";
const ETHEREUM_HTTPS_URL: &str = "https://eth.merkle.io";
const SOLANA_HTTPS_URL: &str = "https://api.mainnet-beta.solana.com";

pub struct DataProvider {
	chain: String,
}

impl DataProvider {
	pub fn chain(chain: &str) -> Self {
		Self { chain: String::from(chain) }
	}

	async fn rpc(&self, url: &str) -> Result<RpcProvider> {
		RpcProvider::new(&self.chain, url).await
	}
}

#[tokio::main]
async fn main() -> Result<()> {
	tracing_subscriber::fmt::init();

	let provider = DataProvider::chain("solana").rpc(SOLANA_HTTPS_URL).await?;
	let result = provider.get_block_number().await?;
	let expected = SolanaRpcProvider::new(SOLANA_HTTPS_URL)?.get_block_number().await?;
	assert_eq!(result, expected);

	let provider = DataProvider::chain("substrate").rpc(SUBSTRATE_HTTPS_URL).await?;
	let latest_block = provider.get_block_number().await?;
	println!("Substrate | Latest block number: {latest_block}");

	let provider = DataProvider::chain("ethereum").rpc(ETHEREUM_HTTPS_URL).await?;
	let latest_block = provider.get_block_number().await?;
	println!("Ethereum | Latest block number: {latest_block}");

	Ok(())
}

#[cfg(test)]
mod test {
	use deku_adapter_ethereum::EthereumRpcProvider;
	use deku_adapter_substrate::SubstrateRpcProvider;

	use crate::*;

	#[tokio::test]
	async fn solana_get_block_number_works() {
		let provider = DataProvider::chain("solana").rpc(SOLANA_HTTPS_URL).await.unwrap();
		let raw_provider = SolanaRpcProvider::new(SOLANA_HTTPS_URL).unwrap();

		// Solana is too fast, can't compare. Nice solana!
		assert!(provider.get_block_number().await.is_ok());
		assert!(raw_provider.get_block_number().await.is_ok());
	}

	#[tokio::test]
	async fn substrate_get_block_number_works() {
		let provider = DataProvider::chain("substrate").rpc(SUBSTRATE_HTTPS_URL).await.unwrap();
		let raw_provider = SubstrateRpcProvider::new(SUBSTRATE_HTTPS_URL).await.unwrap();

		assert!(provider.get_block_number().await.is_ok());
		assert!(raw_provider.get_block_number().await.is_ok());

		let result = provider.get_block_number().await.unwrap();
		let expected = raw_provider.get_block_number().await.unwrap();
		assert_eq!(result, expected);
	}

	#[tokio::test]
	async fn ethereum_get_block_number_works() {
		let provider = DataProvider::chain("ethereum").rpc(ETHEREUM_HTTPS_URL).await.unwrap();
		let raw_provider = EthereumRpcProvider::new(ETHEREUM_HTTPS_URL).unwrap();

		assert!(provider.get_block_number().await.is_ok());
		assert!(raw_provider.get_block_number().await.is_ok());

		let result = provider.get_block_number().await.unwrap();
		let expected = raw_provider.get_block_number().await.unwrap();
		assert_eq!(result, expected);
	}
}
