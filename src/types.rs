//! Provides primitive fixed size hash types.

// Constructed via `fixed_hash` crate macro.

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
