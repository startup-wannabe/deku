use solana_signature::Signature;
use solana_transaction_status::EncodedTransactionWithStatusMeta;

/// Types for a mainnet-like Solana network.
#[derive(Clone, Copy, Debug)]
pub struct Config {
	_private: (),
}

impl crate::Config for Config {
	type GetTxParam = Signature;
	type TxType = EncodedTransactionWithStatusMeta;
}
