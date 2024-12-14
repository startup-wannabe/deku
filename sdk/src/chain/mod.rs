use std::marker::PhantomData;

use chainsmith_networks::{Network, OnchainRpcProvider};
use chainsmith_primitives::Result;
use rpc::RpcProvider;

pub mod rpc;

pub struct ChainMiddleware<N: Network + 'static>(PhantomData<N>);

impl<N: Network + 'static> Default for ChainMiddleware<N> {
	fn default() -> Self {
		Self(PhantomData::default())
	}
}

impl<N: Network + 'static> ChainMiddleware<N> {
	pub async fn rpc(&self, url: &str) -> Result<RpcProvider<N>> {
		RpcProvider::<N>::new(url).await
	}
}
