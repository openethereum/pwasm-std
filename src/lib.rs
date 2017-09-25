#![no_std]
#![feature(lang_items)]
#![feature(link_args)]
#![feature(alloc)]
#![feature(macro_reexport)]

extern crate wasm_alloc;
#[macro_use] #[macro_reexport(vec, format)] extern crate alloc;
extern crate byteorder;

pub extern crate bigint;
pub extern crate parity_hash;
extern crate tiny_keccak;

use core::ptr;
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

/// Fixed-size structures

#[link(name = "env")]
extern {
    fn panic(str_ptr: *const u8, str_len: u32);
}

#[lang = "panic_fmt"]
pub fn panic_fmt(fmt: core::fmt::Arguments, _file_line: &(&'static str, u32)) -> !
{
    let message = format!("{}", fmt);
    unsafe { panic(message.as_ptr(), message.len() as u32) }
    unreachable!("panic MUST return Err(UserTrap); interpreter will stop execution when Err is returned; qed")
}

#[lang = "eh_personality"] extern fn eh_personality() {}

unsafe fn read_ptr_mut(slc: &[u8]) -> *mut u8 {
	ptr::null_mut().offset(read_u32(slc) as isize)
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
