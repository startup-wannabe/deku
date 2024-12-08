use deku_adapter_ethereum::EthereumRpcProvider;
use deku_adapter_solana::SolanaRpcProvider;
use deku_adapter_substrate::SubstrateRpcProvider;
use deku_primitives::OnchainRpcProvider;
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
	pub async fn new(chain: &str, url: &str) -> Result<RpcProvider> {
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
		impl_chain_method!(self, (Solana, Ethereum, Substrate)::get_block_number)
	}
}

/// Helper macro to match and execute the provided method without repeating the redundant variants
/// in pattern matching.
#[macro_export]
macro_rules! impl_chain_method {
	($self:ident, ($($variant:ident),*)::$method:ident) => {
		match &$self.inner {
			$(
				Inner::$variant(inner_value) => inner_value.$method().await,
			)*
			_ => unimplemented!(),
		}
	};
}
