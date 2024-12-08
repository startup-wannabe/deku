use deku_adapter_ethereum::EthereumRpcProvider;
use deku_adapter_solana::SolanaRpcProvider;
use deku_adapter_substrate::SubstrateRpcProvider;
use deku_primitives::{Balance, HexString, OnchainRpcProvider};
use tracing::info;

use crate::*;

#[allow(clippy::large_enum_variant)]
pub enum Inner {
	Unknown(()),
	Solana(SolanaRpcProvider),
	Ethereum(EthereumRpcProvider),
	Substrate(SubstrateRpcProvider),
}

macro_rules! chain_call {
				($self:ident, $method:ident $(, $args:expr)*) => {
								match &$self.inner {
												Inner::Ethereum(v) => v.$method($($args),*).await,
												Inner::Solana(v) => v.$method($($args),*).await,
												Inner::Substrate(v) => v.$method($($args),*).await,
												_ => unimplemented!(),
								}
				};
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
	async fn get_latest_block_number(&self) -> Result<u64> {
		chain_call!(self, get_latest_block_number)
	}

	async fn get_balance(&self, address: HexString) -> Result<Balance> {
		chain_call!(self, get_balance, address)
	}
}
