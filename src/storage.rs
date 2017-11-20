//! Storage extensions for pwasm-std

use hash::H256;

#[cfg_attr(not(feature="std"), link(name = "env"))]
extern {
    fn storage_read(key: *const u8, dst: *mut u8);
    fn storage_write(key: *const u8, src: *const u8);
}

/// Performs read from storage
/// Can return `Error` if data is read from outside of the storage boundaries
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
