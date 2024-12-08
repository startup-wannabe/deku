pub use eyre::Result;
pub use ruint::*;

pub type BlockNumber = u64;
pub type HexString = String;
pub type Balance = Uint<256, 4>;

#[allow(async_fn_in_trait)]
pub trait OnchainRpcProvider {
	async fn get_latest_block_number(&self) -> Result<BlockNumber>;

	async fn get_balance(&self, address: HexString) -> Result<Balance>;
}

impl OnchainRpcProvider for () {
	async fn get_latest_block_number(&self) -> Result<u64> {
		unimplemented!()
	}

	async fn get_balance(&self, _address: HexString) -> Result<Balance> {
		unimplemented!()
	}
}
