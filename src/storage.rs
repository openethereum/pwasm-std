//! Storage extensions for pwasm-std
//!
//! Storage api is a key-value storage where both key and value are 32 bytes in len
//!
//! # Examples
//!
//! ```rust
//! use pwasm_std::hash::H256;
//!
//! # // We can't use original definitions of storage::read/write
//! # // because they require externs to be defined.
//! # mod storage {
//! 	# use pwasm_std::hash::H256;
//! 	# pub fn read(key: &H256) -> [u8; 32] { [0u8; 32] }
//! 	# pub fn write(key: &H256, val: &[u8; 32]) { }
//! # }
//!
//! storage::write(&H256::from([1u8; 32]), &[1u8; 32]);
//! let _v = storage::read(&H256::from([1u8; 32]));
//! ```

use hash::H256;

#[cfg_attr(not(feature="std"), link(name = "env"))]
extern {
    fn storage_read(key: *const u8, dst: *mut u8);
    fn storage_write(key: *const u8, src: *const u8);
}

/// Performs read from the storage.
pub fn read(key: &H256) -> [u8; 32] {
    let mut dst = [0u8; 32];
    unsafe {
        storage_read(key.as_ptr(), dst.as_mut_ptr());
    }
	dst
}

/// Performs write to the storage
pub fn write(key: &H256, val: &[u8; 32]) {
    unsafe {
        storage_write(key.as_ptr(), val.as_ptr());
    }
}
