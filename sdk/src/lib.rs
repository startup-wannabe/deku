use chainsmith_networks::Network;
pub use chainsmith_primitives::*;
use rpc::RpcProvider;

pub mod mock;
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
	use chainsmith_adapters::solana::SolanaRpcProvider;
	use chainsmith_networks::{solana::Solana, OnchainRpcProvider};
	use mock::*;

	use crate::*;

	#[tokio::test]
	async fn solana_rpc_should_works() {
		let provider = DataProvider::default().rpc::<Solana>(SOLANA_HTTPS_URL).await.unwrap();
		let raw_provider = SolanaRpcProvider::new(SOLANA_HTTPS_URL).unwrap();

		// Solana is too fast, can't compare. Nice solana!
		assert!(provider.get_block_number().await.unwrap() > 0);
		assert!(raw_provider.get_block_number().await.unwrap() > 0);
	}

	// #[tokio::test]
	// async fn substrate_rpc_should_works() {
	// 	let provider = DataProvider::default().rpc::<Substrate>(SUBSTRATE_HTTPS_URL).await.unwrap();
	// 	let raw_provider = SubstrateRpcProvider::new(SUBSTRATE_HTTPS_URL).await.unwrap();

	// 	let latest_block_number = provider.get_block_number().await.unwrap();
	// 	let expected_block_number = raw_provider.get_block_number().await.unwrap();
	// 	assert!(latest_block_number > 0);
	// 	assert!(expected_block_number > 0);
	// 	assert_eq!(latest_block_number, expected_block_number);

	// 	let balance = provider.get_balance(SUBSTRATE_ADDRESS_1.to_string()).await.unwrap();
	// 	println!("{:?}", balance);
	// }

	// #[tokio::test]
	// async fn ethereum_rpc_should_works() {
	// 	let provider = DataProvider::default().rpc::<Ethereum>(ETHEREUM_HTTPS_URL).await.unwrap();
	// 	let raw_provider = EthereumRpcProvider::new(ETHEREUM_HTTPS_URL).unwrap();

	// 	let latest_block_number = provider.get_block_number().await.unwrap();
	// 	let expected_block_number = raw_provider.get_block_number().await.unwrap();
	// 	assert!(latest_block_number > 0);
	// 	assert!(expected_block_number > 0);
	// 	assert_eq!(latest_block_number, expected_block_number);
	// }
}
