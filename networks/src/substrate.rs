use crate::Network;
use chainsmith_primitives::{ConvertIO, Result, TxSignature, UniversalTx};
use std::str::FromStr;
use subxt::utils::H256;

/// Types for a mainnet-like Substrate-based network.
#[derive(Clone, Copy, Debug)]
pub struct Substrate {
	_private: (),
}

impl Network for Substrate {
	const CHAIN: &str = "substrate";

	type TxSignature = H256;
	type TxType = ();
}

impl ConvertIO<TxSignature, <Substrate as Network>::TxSignature> for Substrate {
	fn convert(input: TxSignature) -> Result<<Substrate as Network>::TxSignature> {
		Ok(H256::from_str(&input)?)
	}
}

impl ConvertIO<<Substrate as Network>::TxType, UniversalTx<()>> for Substrate {
	fn convert(_input: <Substrate as Network>::TxType) -> Result<UniversalTx<()>> {
		unimplemented!()
	}
}
