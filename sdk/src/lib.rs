pub use deku_primitives::*;
use eyre::eyre;
use rpc::RpcProvider;

pub mod constants;
pub mod rpc;

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

#[cfg(test)]
mod test {
	use constants::*;
	use deku_adapter_ethereum::EthereumRpcProvider;
	use deku_adapter_solana::SolanaRpcProvider;
	use deku_adapter_substrate::SubstrateRpcProvider;
	use deku_primitives::OnchainRpcProvider;

	use crate::*;

	#[tokio::test]
	async fn solana_get_block_number_works() {
		let provider = DataProvider::chain("solana").rpc(SOLANA_HTTPS_URL).await.unwrap();
		let raw_provider = SolanaRpcProvider::new(SOLANA_HTTPS_URL).unwrap();

		// Solana is too fast, can't compare. Nice solana!
		assert!(provider.get_latest_block_number().await.unwrap() > 0);
		assert!(raw_provider.get_latest_block_number().await.unwrap() > 0);
	}

	#[tokio::test]
	async fn substrate_get_latest_block_number_works() {
		let provider = DataProvider::chain("substrate").rpc(SUBSTRATE_HTTPS_URL).await.unwrap();
		let raw_provider = SubstrateRpcProvider::new(SUBSTRATE_HTTPS_URL).await.unwrap();

		assert!(provider.get_latest_block_number().await.unwrap() > 0);
		assert!(raw_provider.get_latest_block_number().await.unwrap() > 0);

		let result = provider.get_latest_block_number().await.unwrap();
		let expected = raw_provider.get_latest_block_number().await.unwrap();
		assert_eq!(result, expected);
	}

	#[tokio::test]
	async fn ethereum_get_latest_block_number_works() {
		let provider = DataProvider::chain("ethereum").rpc(ETHEREUM_HTTPS_URL).await.unwrap();
		let raw_provider = EthereumRpcProvider::new(ETHEREUM_HTTPS_URL).unwrap();

		assert!(provider.get_latest_block_number().await.unwrap() > 0);
		assert!(raw_provider.get_latest_block_number().await.unwrap() > 0);

		let result = provider.get_latest_block_number().await.unwrap();
		let expected = raw_provider.get_latest_block_number().await.unwrap();
		assert_eq!(result, expected);
	}
}
