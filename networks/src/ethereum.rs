use alloy::{
	consensus::Account,
	primitives::{BlockNumber, TxHash},
	rpc::types::{Block, Transaction},
};

/// Types for a mainnet-like Ethereum network.
#[derive(Clone, Copy, Debug)]
pub struct Config {
	_private: (),
}

impl crate::Config for Config {
	type AccountData = Account;
	type BlockData = Block;
	type BlockQuery = BlockNumber;
	type Transaction = Transaction;
	type TransactionQuery = TxHash;
}
