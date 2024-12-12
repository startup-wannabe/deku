use std::{any::TypeId, marker::PhantomData};

use chainsmith_adapters::{
	ethereum::EthereumRpcProvider, solana::SolanaRpcProvider, substrate::SubstrateRpcProvider,
};
use chainsmith_networks::{
	ethereum::Ethereum, solana::Solana, substrate::Substrate, Network, OnchainRpcProvider,
};
use chainsmith_primitives::{Balance, HexString};
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

pub struct RpcProvider<T: Network> {
	inner: Inner,
	network: PhantomData<T>,
}

impl<T: Network> Default for RpcProvider<T> {
	fn default() -> Self {
		Self { inner: Inner::Unknown(()), network: PhantomData::default() }
	}
}

impl<T: Network + 'static> RpcProvider<T> {
	pub async fn new(url: &str) -> Result<RpcProvider<T>> {
		info!("Initialization with url: {:?}", url);
		if TypeId::of::<T>() == TypeId::of::<Solana>() {
			return Ok(RpcProvider {
				inner: Inner::Solana(SolanaRpcProvider::new(&url)?),
				..Default::default()
			});
		};

		if TypeId::of::<T>() == TypeId::of::<Substrate>() {
			let provider = SubstrateRpcProvider::new(&url).await?;
			return Ok(RpcProvider { inner: Inner::Substrate(provider), ..Default::default() });
		};

		if TypeId::of::<T>() == TypeId::of::<Ethereum>() {
			return Ok(RpcProvider {
				inner: Inner::Ethereum(EthereumRpcProvider::new(&url)?),
				..Default::default()
			});
		};

		Ok(Default::default())
	}

	pub async fn get_block_number(&self) -> Result<u64> {
		chain_call!(self, get_block_number)
	}

	pub async fn get_balance(&self, address: HexString) -> Result<Option<Balance>> {
		chain_call!(self, get_balance, address)
	}

	pub async fn get_transaction(&self, _param: T::GetTxParam) -> Result<T::TxType> {
		unimplemented!()
	}
}
