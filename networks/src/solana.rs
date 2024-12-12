use chainsmith_primitives::{ConvertIO, Result, TxSignature, UniversalTx};
use solana_signature::Signature;
use solana_transaction_status::EncodedTransactionWithStatusMeta;
use std::str::FromStr;

use crate::Network;

/// Types for a mainnet-like Solana network.
#[derive(Clone, Copy, Debug)]
pub struct Solana {
	_private: (),
}

impl Network for Solana {
	const CHAIN: &str = "solana";

	type TxSignature = Signature;
	type TxType = EncodedTransactionWithStatusMeta;
}

impl ConvertIO<TxSignature, <Solana as Network>::TxSignature> for Solana {
	fn convert(input: TxSignature) -> Result<<Solana as Network>::TxSignature> {
		Ok(Signature::from_str(&input)?)
	}
}

impl ConvertIO<<Solana as Network>::TxType, UniversalTx<()>> for Solana {
	fn convert(_input: <Solana as Network>::TxType) -> Result<UniversalTx<()>> {
		unimplemented!()
	}
}
