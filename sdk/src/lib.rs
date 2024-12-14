use chain::ChainMiddleware;
use chainsmith_networks::Network;
pub use chainsmith_primitives::*;

pub mod chain;
pub mod constants;

#[derive(Default)]
pub struct ChainsmithSdk {
	_private: (),
}

impl ChainsmithSdk {
	pub fn chain<N: Network + 'static>() -> ChainMiddleware<N> {
		ChainMiddleware::<N>::default()
	}
}

#[cfg(test)]
mod test {
	use chainsmith_adapters::{
		ethereum::{Ethereum, EthereumRpcProvider},
		solana::{Solana, SolanaRpcProvider},
		substrate::SubstrateRpcProvider,
	};
	use chainsmith_networks::OnchainRpcProvider;
	use constants::*;

	use crate::*;

	#[tokio::test]
	async fn solana_get_block_number_works() {
		let provider = ChainsmithSdk::chain::<Solana>().rpc(SOLANA_HTTPS_URL).await.unwrap();
		let raw_provider = SolanaRpcProvider::new(SOLANA_HTTPS_URL).await.unwrap();

		// Solana is too fast, can't compare. Nice solana!
		assert!(provider.get_block_number().await.unwrap() > 0);
		assert!(raw_provider.get_block_number().await.unwrap() > 0);
	}

	#[tokio::test]
	async fn substrate_get_block_number_works() {
		let provider = ChainsmithSdk::chain::<Solana>().rpc(SUBSTRATE_HTTPS_URL).await.unwrap();
		let raw_provider = SubstrateRpcProvider::new(SUBSTRATE_HTTPS_URL).await.unwrap();

		assert!(provider.get_block_number().await.unwrap() > 0);
		assert!(raw_provider.get_block_number().await.unwrap() > 0);

		let result = provider.get_block_number().await.unwrap();
		let expected = raw_provider.get_block_number().await.unwrap();
		assert_eq!(result, expected);
	}

	#[tokio::test]
	async fn ethereum_get_block_number_works() {
		let provider = ChainsmithSdk::chain::<Ethereum>().rpc(ETHEREUM_HTTPS_URL).await.unwrap();
		let raw_provider = EthereumRpcProvider::new(ETHEREUM_HTTPS_URL).await.unwrap();

		assert!(provider.get_block_number().await.unwrap() > 0);
		assert!(raw_provider.get_block_number().await.unwrap() > 0);

		let result = provider.get_block_number().await.unwrap();
		let expected = raw_provider.get_block_number().await.unwrap();
		assert_eq!(result, expected);
	}
}
