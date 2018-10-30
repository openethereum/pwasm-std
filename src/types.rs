//! Provides primitive fixed size hash types.

// Constructed via `fixed_hash` crate macro.

pub use uint::U256;

construct_fixed_hash!{
	/// A 160 bits (20 bytes) hash type.
	///
	/// # Note
	///
	/// Can be interchanged with `Address`
	pub struct H160(20);
}

construct_fixed_hash!{
	/// A 256-bits (32 bytes) hash type.
	pub struct H256(32);
}

/// Represents an address in ethereum context.
/// 
/// # Note
/// 
/// Addresses have 160 bytes length.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct Address(H160);

impl From<H160> for Address {
	fn from(hash: H160) -> Self {
		Address(hash)
	}
}

impl Address {
	/// Returns the inner `H160` hash type of `self`.
	pub fn into_inner(self) -> H160 {
		self.0
	}
}

impl From<U256> for H256 {
	fn from(uint: U256) -> H256 {
		let mut hash = H256::zero();
		uint.to_big_endian(hash.as_bytes_mut());
		hash
	}
}

impl<'a> From<&'a U256> for H256 {
	fn from(uint: &'a U256) -> H256 {
		let mut hash: H256 = H256::zero();
		uint.to_big_endian(hash.as_bytes_mut());
		hash
	}
}

impl From<H256> for U256 {
	fn from(hash: H256) -> U256 {
		U256::from(hash.as_bytes())
	}
}

impl<'a> From<&'a H256> for U256 {
	fn from(hash: &'a H256) -> U256 {
		U256::from(hash.as_bytes())
	}
}
