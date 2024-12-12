use solana_signature::Signature;
use solana_transaction_status::EncodedTransactionWithStatusMeta;

use crate::Network;

/// Types for a mainnet-like Solana network.
#[derive(Clone, Copy, Debug)]
pub struct Solana {
	_private: (),
}

impl Network for Solana {
	type GetTxParam = Signature;
	type TxType = EncodedTransactionWithStatusMeta;
}
