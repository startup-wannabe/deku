// TODO: Process any Rust file to get the functions
//
// TODO: Adapter pattern factory for data source provider
//
// TODO: Construct multiple different SDK to query the data

trait ProvideData {
	fn configure() {}
	fn provide() {}
}

trait FetchData {
	type SourceProvider: ProvideData + Default;
	type Output;
}

fn main() {
	println!("Hello, world!");
}
