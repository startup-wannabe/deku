use eyre::Result;

pub type BlockNumber = u64;

#[allow(async_fn_in_trait)]
pub trait OnchainRpcProvider {
	async fn get_block_number(&self) -> Result<BlockNumber>;
}

impl OnchainRpcProvider for () {
	async fn get_block_number(&self) -> Result<u64> {
		Ok(0)
	}
}
