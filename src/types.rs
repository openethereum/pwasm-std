//! Provides primitive fixed size hash types.

// Constructed via `fixed_hash` macro.

construct_hash!(H32, 4);
construct_hash!(H64, 8);
construct_hash!(H128, 16);
construct_hash!(H160, 20);
construct_hash!(H256, 32);
construct_hash!(H264, 33);
construct_hash!(H512, 64);
construct_hash!(H1024, 128);
construct_hash!(H2048, 256);

/// Represents an address in ethereum context.
/// 
/// # Note
/// 
/// Addresses have 160 bytes length.
pub type Address = H160;
