use crate::Network;

/// Types for a mainnet-like Substrate-based network.
#[derive(Clone, Copy, Debug)]
pub struct Substrate {
	_private: (),
}

impl Network for Substrate {
	type TxType = ();
	type GetTxParam = String;
}
