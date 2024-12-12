use alloy::{
	primitives::{FixedBytes, TxHash},
	rpc::types::Transaction,
};
use chainsmith_primitives::{ConvertIO, Result, TxSignature, UniversalTx};

use crate::Network;

/// Types for a mainnet-like Ethereum network.
#[derive(Clone, Copy, Debug)]
pub struct Ethereum {
	_private: (),
}

impl Network for Ethereum {
	const CHAIN: &str = "ethereum";

	type TxSignature = TxHash;
	type TxType = Transaction;
}

impl ConvertIO<TxSignature, <Ethereum as Network>::TxSignature> for Ethereum {
	fn convert(input: TxSignature) -> Result<<Ethereum as Network>::TxSignature> {
		let bytes = FixedBytes::<32>::from_slice(input.as_bytes());
		let signature = <Ethereum as Network>::TxSignature::from(bytes);
		Ok(signature)
	}
}

impl ConvertIO<<Ethereum as Network>::TxType, UniversalTx<()>> for Ethereum {
	fn convert(_input: <Ethereum as Network>::TxType) -> Result<UniversalTx<()>> {
		unimplemented!()
	}
}
