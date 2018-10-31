//! Standard library for parity wasm programs

#![cfg_attr(not(feature="std"), no_std)]
#![warn(missing_docs)]
#![cfg_attr(feature = "strict", deny(warnings))]
#![feature(lang_items)]
#![feature(link_args)]
#![feature(alloc)]
#![feature(core_intrinsics)]
#![feature(panic_implementation)]
#![feature(panic_info_message)]

#[cfg(feature="std")]
extern crate core;

#[cfg(not(feature = "std"))]
extern crate pwasm_alloc;

#[cfg(not(feature = "std"))]
extern crate pwasm_libc;

#[macro_use]
extern crate static_assertions;

#[allow(unused)]
#[macro_use]
extern crate alloc;
pub use alloc::{vec, format};

extern crate byteorder;

#[macro_use]
extern crate fixed_hash;

extern crate uint;

extern crate tiny_keccak;

use byteorder::{LittleEndian, ByteOrder};

pub use alloc::boxed::Box;
pub use alloc::string::String;
pub use alloc::str;
pub use alloc::vec::Vec;

pub mod types;

// Safe wrapper around debug logging
pub mod logger;

// Crypto functions
mod crypto;

mod panic;

#[no_mangle]
#[cfg(not(feature = "std"))]
pub use panic::panic_fmt;

#[no_mangle]
#[cfg(not(feature = "std"))]
pub use panic::oom;

pub use crypto::keccak;

/// Read u32 using native endianness
pub fn read_u32(slc: &[u8]) -> u32 {
	LittleEndian::read_u32(slc)
}

/// Write u32 using native endianness
pub fn write_u32(dst: &mut [u8], val: u32) {
	LittleEndian::write_u32(dst, val)
}

/// Write ptr using native endianness
pub fn write_ptr(dst: &mut [u8], ptr: *mut u8) {
	// todo: consider: add assert that arch is 32bit
	write_u32(dst, ptr as usize as u32);
}

/// Read u64 using native endianness
pub fn read_u64(slc: &[u8]) -> u64 {
	LittleEndian::read_u64(slc)
}

/// Write u64 using native endianness
pub fn write_u64(dst: &mut [u8], val: u64) {
	LittleEndian::write_u64(dst, val)
}
