// TODO: Process any Rust file to get the functions
//
// TODO: Feature gating
//
// TODO: Adapter pattern factory for data source provider
//
// TODO: Construct multiple different SDK to query the data

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
	let latest_block = provider.get_block_number().await?;
	println!("Solana | Latest block number: {latest_block}");

	let provider = DataProvider::chain("substrate").rpc(SUBSTRATE_HTTPS_URL).await?;
	let latest_block = provider.get_block_number().await?;
	println!("Substrate | Latest block number: {latest_block}");

	let provider = DataProvider::chain("ethereum").rpc(ETHEREUM_HTTPS_URL).await?;
	let latest_block = provider.get_block_number().await?;
	println!("Ethereum | Latest block number: {latest_block}");

	Ok(())
}
