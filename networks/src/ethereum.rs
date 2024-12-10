use alloy::{primitives::TxHash, rpc::types::Transaction};

use crate::Network;

/// Types for a mainnet-like Ethereum network.
#[derive(Clone, Copy, Debug)]
pub struct Ethereum {
	_private: (),
}

impl Network for Ethereum {
	type TxType = Transaction;
	type GetTxParam = TxHash;
}
