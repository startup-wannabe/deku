use alloy::{primitives::TxHash, rpc::types::Transaction};

/// Types for a mainnet-like Ethereum network.
#[derive(Clone, Copy, Debug)]
pub struct Config {
	_private: (),
}

impl crate::Config for Config {
	type GetTxParam = TxHash;
	type TxType = Transaction;
}
