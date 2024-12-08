use deku_adapter_ethereum::EthereumRpcProvider;
use deku_adapter_solana::SolanaRpcProvider;
use deku_adapter_substrate::SubstrateRpcProvider;
use deku_primitives::OnchainRpcProvider;
use spandoc::spandoc;
use tracing::info;

use crate::*;

#[allow(clippy::large_enum_variant)]
pub enum Inner {
	Unknown(()),
	Solana(SolanaRpcProvider),
	Ethereum(EthereumRpcProvider),
	Substrate(SubstrateRpcProvider),
}

pub struct RpcProvider {
	inner: Inner,
}

impl Default for RpcProvider {
	fn default() -> Self {
		Self { inner: Inner::Unknown(()) }
	}
}

impl RpcProvider {
	#[spandoc]
	pub async fn new(chain: &str, url: &str) -> Result<RpcProvider> {
		/// SPANDOC: RpcProvider {chain}
		info!("Initialization with url: {:?}", url);
		match chain {
			c if c == "solana" =>
				Ok(RpcProvider { inner: Inner::Solana(SolanaRpcProvider::new(&url)?) }),
			c if c == "substrate" => {
				let provider = SubstrateRpcProvider::new(&url).await?;
				Ok(RpcProvider { inner: Inner::Substrate(provider) })
			},
			c if c == "ethereum" =>
				Ok(RpcProvider { inner: Inner::Ethereum(EthereumRpcProvider::new(&url)?) }),
			_ => Err(eyre!("Unsupported chain")),
		}
	}
}

impl OnchainRpcProvider for RpcProvider {
	async fn get_block_number(&self) -> Result<u64> {
		match &self.inner {
			Inner::Solana(inner) => inner.get_block_number().await,
			Inner::Ethereum(inner) => inner.get_block_number().await,
			Inner::Substrate(inner) => inner.get_block_number().await,
			Inner::Unknown(inner) => inner.get_block_number().await,
		}?;

		Ok(0)
	}
}
