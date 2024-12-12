use chainsmith_networks::{Network, OnchainRpcProvider};
pub use chainsmith_primitives::*;
use rpc::RpcProvider;

pub mod constants;
pub mod rpc;

#[derive(Default)]
pub struct ChainsmithSdk {
	_private: (),
}

impl ChainsmithSdk {
	pub async fn rpc<N: Network + 'static>(&self, url: &str) -> Result<RpcProvider<N>> {
		RpcProvider::<N>::new(url).await
	}
}

#[cfg(test)]
mod test {
	use chainsmith_adapters::{
		ethereum::{Ethereum, EthereumRpcProvider},
		solana::{Solana, SolanaRpcProvider},
		substrate::{Substrate, SubstrateRpcProvider},
	};
	use chainsmith_networks::OnchainRpcProvider;
	use constants::*;

	use crate::*;

	#[tokio::test]
	async fn solana_get_block_number_works() {
		let provider = ChainsmithSdk::default().rpc::<Solana>(SOLANA_HTTPS_URL).await.unwrap();
		let raw_provider = SolanaRpcProvider::new(SOLANA_HTTPS_URL).await.unwrap();

		// Solana is too fast, can't compare. Nice solana!
		assert!(provider.get_block_number().await.unwrap() > 0);
		assert!(raw_provider.get_block_number().await.unwrap() > 0);
	}

	#[tokio::test]
	async fn substrate_get_block_number_works() {
		let provider =
			ChainsmithSdk::default().rpc::<Substrate>(SUBSTRATE_HTTPS_URL).await.unwrap();
		let raw_provider = SubstrateRpcProvider::new(SUBSTRATE_HTTPS_URL).await.unwrap();

		assert!(provider.get_block_number().await.unwrap() > 0);
		assert!(raw_provider.get_block_number().await.unwrap() > 0);

		let result = provider.get_block_number().await.unwrap();
		let expected = raw_provider.get_block_number().await.unwrap();
		assert_eq!(result, expected);
	}

	#[tokio::test]
	async fn ethereum_get_block_number_works() {
		let provider = ChainsmithSdk::default().rpc::<Ethereum>(ETHEREUM_HTTPS_URL).await.unwrap();
		let raw_provider = EthereumRpcProvider::new(ETHEREUM_HTTPS_URL).await.unwrap();

		assert!(provider.get_block_number().await.unwrap() > 0);
		assert!(raw_provider.get_block_number().await.unwrap() > 0);

		let result = provider.get_block_number().await.unwrap();
		let expected = raw_provider.get_block_number().await.unwrap();
		assert_eq!(result, expected);
	}
}
