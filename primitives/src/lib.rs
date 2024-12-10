pub use eyre::Result;
pub use ruint::*;

pub type BlockNumber = u64;
pub type HexString = String;
pub type Balance = Uint<256, 4>;
pub type TxSignature = String;

// pub mod transaction {
// 	#[cfg(feature = "solana")]
// 	use solana_transaction_status::EncodedTransactionWithStatusMeta;

// 	pub enum MultichainTransaction {
// 		#[cfg(feature = "solana")]
// 		Solana(EncodedTransactionWithStatusMeta),
// 	}
// }
