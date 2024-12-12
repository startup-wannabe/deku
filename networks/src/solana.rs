use solana_sdk::account::Account;
use solana_signature::Signature;
use solana_transaction_status::{EncodedConfirmedBlock, EncodedTransactionWithStatusMeta};

/// Types for a mainnet-like Solana network.
#[derive(Clone, Copy, Debug)]
pub struct Config {
	_private: (),
}

impl crate::Config for Config {
	type AccountData = Account;
	type BlockData = EncodedConfirmedBlock;
	type BlockQuery = u64;
	type Transaction = EncodedTransactionWithStatusMeta;
	type TransactionQuery = Signature;
}
