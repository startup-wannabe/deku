use deku_networks::Network;
pub use deku_primitives::*;
use rpc::RpcProvider;

pub mod constants;
pub mod rpc;

#[derive(Default)]
pub struct DataProvider {
	_private: (),
}

impl DataProvider {
	pub async fn rpc<N: Network + 'static>(&self, url: &str) -> Result<RpcProvider<N>> {
		RpcProvider::<N>::new(url).await
	}
}

#[cfg(test)]
mod test {
	use constants::*;
	use deku_adapters::{
		ethereum::EthereumRpcProvider, solana::SolanaRpcProvider, substrate::SubstrateRpcProvider,
	};
	use deku_networks::{
		ethereum::Ethereum, solana::Solana, substrate::Substrate, OnchainRpcProvider,
	};

	use crate::*;

	#[tokio::test]
	async fn solana_get_block_number_works() {
		let provider = DataProvider::default().rpc::<Solana>(SOLANA_HTTPS_URL).await.unwrap();
		let raw_provider = SolanaRpcProvider::new(SOLANA_HTTPS_URL).unwrap();

		// Solana is too fast, can't compare. Nice solana!
		assert!(provider.get_block_number().await.unwrap() > 0);
		assert!(raw_provider.get_block_number().await.unwrap() > 0);
	}

	#[tokio::test]
	async fn substrate_get_block_number_works() {
		let provider = DataProvider::default().rpc::<Substrate>(SUBSTRATE_HTTPS_URL).await.unwrap();
		let raw_provider = SubstrateRpcProvider::new(SUBSTRATE_HTTPS_URL).await.unwrap();

		assert!(provider.get_block_number().await.unwrap() > 0);
		assert!(raw_provider.get_block_number().await.unwrap() > 0);

		let result = provider.get_block_number().await.unwrap();
		let expected = raw_provider.get_block_number().await.unwrap();
		assert_eq!(result, expected);
	}

	#[tokio::test]
	async fn ethereum_get_block_number_works() {
		let provider = DataProvider::default().rpc::<Ethereum>(ETHEREUM_HTTPS_URL).await.unwrap();
		let raw_provider = EthereumRpcProvider::new(ETHEREUM_HTTPS_URL).unwrap();

		assert!(provider.get_block_number().await.unwrap() > 0);
		assert!(raw_provider.get_block_number().await.unwrap() > 0);

		let result = provider.get_block_number().await.unwrap();
		let expected = raw_provider.get_block_number().await.unwrap();
		assert_eq!(result, expected);
	}
}
