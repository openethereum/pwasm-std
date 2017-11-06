#![cfg_attr(not(feature="std"), no_std)]
#![cfg_attr(feature = "strict", deny(warnings))]
#![feature(lang_items)]
#![feature(link_args)]
#![feature(alloc)]
#![feature(macro_reexport)]

#[cfg(feature="std")]
extern crate core;

extern crate wasm_alloc;
#[macro_reexport(vec, format)] extern crate alloc;
extern crate byteorder;

pub extern crate bigint;
pub extern crate parity_hash;
extern crate tiny_keccak;

use byteorder::{LittleEndian, ByteOrder};

pub use alloc::boxed::Box;
pub use alloc::string::String;
pub use alloc::str;
pub use alloc::vec::Vec;
pub use wasm_alloc::WamsAllocator;
pub use parity_hash as hash;

/// Wrapper over storage read/write externs
/// Storage api is a key-value storage where both key and value are 32 bytes in len
pub mod storage;

/// Safe wrapper around debug logging
pub mod logger;

/// Safe wrapper around externalities invokes
pub mod ext;

/// Wrapper for arguments and result
mod wrapped;

/// Crypto functions
mod crypto;

pub use wrapped::{WrappedArgs, WrappedResult, parse_args};
pub use crypto::keccak;

#[cfg(not(feature="std"))]
#[lang = "panic_fmt"]
pub fn panic_fmt(
	_fmt: core::fmt::Arguments,
	file: &'static str,
	_line: u32,
	_col: u32,
) -> ! {
	extern "C" {
		fn panic(str_ptr: *const u8, str_len: u32) -> !;
	}
	unsafe {
		let ptr = file.as_ptr();
		let len = file.len() as u32;
		panic(ptr, len);
	}
}

#[cfg(not(feature="std"))]
#[lang = "eh_personality"] extern fn eh_personality() {}

unsafe fn read_ptr_mut(slc: &[u8]) -> *mut u8 {
	read_u32(slc) as *const u8 as *mut u8
}

pub fn read_u32(slc: &[u8]) -> u32 {
	LittleEndian::read_u32(slc)
}

pub fn write_u32(dst: &mut [u8], val: u32) {
	LittleEndian::write_u32(dst, val)
}

pub fn write_ptr(dst: &mut [u8], ptr: *mut u8) {
	// todo: consider: add assert that arch is 32bit
	write_u32(dst, ptr as usize as u32);
}

pub fn read_u64(slc: &[u8]) -> u64 {
	LittleEndian::read_u64(slc)
}

pub fn write_u64(dst: &mut [u8], val: u64) {
	LittleEndian::write_u64(dst, val)
}
