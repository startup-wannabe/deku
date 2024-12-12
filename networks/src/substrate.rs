/// Types for a mainnet-like Substrate-based network.
#[derive(Clone, Copy, Debug)]
pub struct SubstrateConfig {
	_private: (),
}

impl crate::Config for SubstrateConfig {
	type GetTxParam = String;
	type TxType = ();
}
