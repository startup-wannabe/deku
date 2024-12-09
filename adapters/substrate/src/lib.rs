use deku_networks::{substrate::Substrate, Network, OnchainRpcProvider};
use deku_primitives::{Balance, HexString, Result, Uint};
use eyre::WrapErr;
use subxt::{
	dynamic::{At, DecodedValue, Value},
	storage::Storage,
	OnlineClient, PolkadotConfig,
};
use tracing::info;

pub struct SubstrateRpcProvider {
	inner: OnlineClient<PolkadotConfig>,
}

impl SubstrateRpcProvider {
	pub async fn new(url: &str) -> Result<Self> {
		let api = OnlineClient::<PolkadotConfig>::from_url(url)
			.await
			.wrap_err("Failed to initialize the RPC provider")?;
		Ok(Self { inner: api })
	}

	pub async fn get_latest_storage(
		&self,
	) -> Result<Storage<PolkadotConfig, OnlineClient<PolkadotConfig>>> {
		self.inner.storage().at_latest().await.wrap_err("Failed to get latest storage")
	}

	pub async fn query_storage(
		&self,
		pallet_name: &str,
		entry_name: &str,
		params: Vec<Value>,
	) -> Result<DecodedValue> {
		let query = subxt::dynamic::storage(pallet_name, entry_name, params);
		let latest_storage = self.get_latest_storage().await?;
		let result = latest_storage.fetch(&query).await?;
		result.unwrap().to_value().wrap_err("Failed to decode value")
	}
}

impl OnchainRpcProvider<Substrate> for SubstrateRpcProvider {
	async fn get_block_number(&self) -> Result<u64> {
		info!(method = "get_block_number");
		let block_number = self.inner.blocks().at_latest().await?.number();
		Ok(block_number.into())
	}

	async fn get_balance(&self, address: HexString) -> Result<Option<Balance>> {
		info!(method = "get_balance");
		let value = self
			.query_storage("System", "Account", vec![Value::from_bytes(address.as_bytes())])
			.await?;
		let account_data = value.at("data");
		Ok(account_data
			.at("free")
			.map(|b| {
				let balance = b.as_u128().unwrap();
				Some(Uint::from(balance))
			})
			.unwrap_or(None))
	}

	async fn get_transaction(
		&self,
		_: <Substrate as Network>::GetTxParam,
	) -> Result<Option<<Substrate as Network>::TxType>> {
		unimplemented!()
	}
}
