pub use eyre::Result;
pub use ruint::*;

pub type BlockNumber = u64;
pub type Address = String;
pub type Balance = Uint<256, 4>;
pub type Signature = String;
