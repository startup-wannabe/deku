pub use eyre::Result;
pub use ruint::*;

pub type Address = String;
pub type BlockNumber = u64;
pub type BlockHash = String;
pub type Balance = Uint<256, 4>;
pub type TxSignature = String;

pub struct UniversalTx<T> {
	/// Hash of block where transaction was included, `None` if pending
	pub block_hash: Option<BlockHash>,

	/// Number of block where transaction was included, `None` if pending
	pub block_number: Option<BlockHash>,

	/// Transaction Index
	pub transaction_index: Option<u64>,

	pub fee: Option<u128>,

	/// Sender
	pub from: Address,

	/// The inner transaction object
	pub inner: T,
}

pub trait ConvertIO<I, O> {
	fn convert(input: I) -> Result<O>;
}
