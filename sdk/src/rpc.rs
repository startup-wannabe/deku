use std::{any::TypeId, marker::PhantomData};

use chainsmith_adapters::{
	ethereum::EthereumRpcProvider, solana::SolanaRpcProvider, substrate::SubstrateRpcProvider,
};
use chainsmith_networks::{
	ethereum::Ethereum, solana::Solana, substrate::Substrate, Network, OnchainRpcProvider,
};
use chainsmith_primitives::{Address, Balance};
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

pub struct RpcProvider<N: Network> {
	inner: Inner,
	network: PhantomData<N>,
}

impl<N: Network> Default for RpcProvider<N> {
	fn default() -> Self {
		Self { inner: Inner::Unknown(()), network: PhantomData::default() }
	}
}

impl<
		N: Network
			+ ConvertIO<TxSignature, N::TxSignature>
			+ ConvertIO<N::TxType, UniversalTx<N::TxType>>
			+ 'static,
	> RpcProvider<N>
{
	pub async fn new(url: &str) -> Result<RpcProvider<N>> {
		info!("Initialization with url: {:?}", url);
		if TypeId::of::<N>() == TypeId::of::<Solana>() {
			return Ok(RpcProvider {
				inner: Inner::Solana(SolanaRpcProvider::new(&url)?),
				..Default::default()
			});
		};

		if TypeId::of::<N>() == TypeId::of::<Substrate>() {
			let provider = SubstrateRpcProvider::new(&url).await?;
			return Ok(RpcProvider { inner: Inner::Substrate(provider), ..Default::default() });
		};

		if TypeId::of::<N>() == TypeId::of::<Ethereum>() {
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

	pub async fn get_balance(&self, address: Address) -> Result<Option<Balance>> {
		chain_call!(self, get_balance, address)
	}

	pub async fn get_transaction(&self, param: TxSignature) -> Result<Option<N::TxType>> {
		match &self.inner {
			Inner::Ethereum(v) => {
				let signature = Ethereum::convert(param)?;
				let tx = v.get_transaction(signature).await?.unwrap();
				let a: UniversalTx<()> = Ethereum::convert(tx)?;
			},
			Inner::Solana(v) => {
				let signature = Solana::convert(param)?;
				let tx = v.get_transaction(signature).await?.unwrap();
				let a: UniversalTx<()> = Solana::convert(tx)?;
			},
			Inner::Substrate(v) => {
				let signature = Substrate::convert(param)?;
				let tx = v.get_transaction(signature).await?.unwrap();
				let a: UniversalTx<()> = Substrate::convert(tx)?;
			},
			_ => unimplemented!(),
		};
		// chain_call!(self, get_transaction, param)
		Ok(None)
	}
}
