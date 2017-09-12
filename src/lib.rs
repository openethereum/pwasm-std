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

use core::{slice, ptr, mem};
use byteorder::{LittleEndian, ByteOrder};
pub use alloc::boxed::Box;
pub use alloc::string::String;
pub use alloc::str;
pub use alloc::vec::Vec;
pub use wasm_alloc::WamsAllocator;
pub use parity_hash as hash;
use tiny_keccak::Keccak;
use hash::Address;
use hash::H256;
use bigint::U256;

/// Wrapper over storage read/write externs
/// Storage api is a key-value storage where both key and value are 32 bytes in len
pub mod storage;

/// Safe wrapper around debug logging
pub mod logger;

/// Safe wrapper around externalities invokes
pub mod ext;

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

/// Safe wrapper for call context
pub struct CallArgs {
    context: Box<[u8]>,
    result: Box<[u8]>,
}

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

pub fn sha3<T>(input: T) -> H256 where T: AsRef<[u8]> {
	let mut keccak = Keccak::new_keccak256();
    let mut res = H256::new();
	keccak.update(input.as_ref());
    keccak.finalize(&mut res);
	res
}

impl CallArgs {
	pub unsafe fn from_raw(ptr: *mut u8) -> CallArgs {
		let desc_slice = slice::from_raw_parts(ptr, 4 * 4);

		let context_ptr = read_ptr_mut(&desc_slice[0..4]);
		let context_len = read_u32(&desc_slice[4..8]) as usize;

		let result_ptr = read_ptr_mut(&desc_slice[8..12]);
		let result_len = read_u32(&desc_slice[12..16]) as usize;

		CallArgs {
			context: Box::<[u8]>::from_raw(slice::from_raw_parts_mut(context_ptr, context_len)),
			result: Box::<[u8]>::from_raw(slice::from_raw_parts_mut(result_ptr, result_len)),
		}
	}

	pub fn context(&self) -> &[u8] {
		&self.context
	}

	pub fn result_mut(&mut self) -> &mut Box<[u8]> {
		&mut self.result
	}

	pub unsafe fn save(self, ptr: *mut u8) {
		let dst = slice::from_raw_parts_mut(ptr.offset(8), 2 * 4);
		let context = self.context;
		let mut result = self.result;

		// context unmodified and memory is managed in calling code
		mem::forget(context);

		if result.len() > 0 {
			// result
			write_ptr(&mut dst[0..4], result.as_mut_ptr());
			write_u32(&mut dst[4..8], result.len() as u32);
			// managed in calling code
			mem::forget(result);
		}
	}

	pub fn params<'a> (&'a self) -> ParamsView<'a> {
		ParamsView::new(self.context())
	}
}

pub struct ParamsView<'a> {
	raw: &'a [u8],
}

impl<'a> ParamsView<'a> {
	fn new(raw: &'a [u8]) -> Self {
		ParamsView { raw: raw }
	}

	pub fn address(&self) -> Address {
		let mut addr = [0u8; 20];
		addr.copy_from_slice(&self.raw[0..20]);
		Address::from(addr)
	}

	pub fn sender(&self) -> Address {
		let mut sender = [0u8; 20];
		sender.copy_from_slice(&self.raw[20..40]);
		Address::from(sender)
	}

	pub fn origin(&self) -> Address {
		let mut origin = [0u8; 20];
		origin.copy_from_slice(&self.raw[40..60]);
		Address::from(origin)
	}

	pub fn value(&self) -> U256 {
		let mut value = [0u8; 32];
		value.copy_from_slice(&self.raw[60..92]);
		U256::from_big_endian(&value)
	}

	pub fn args(&self) -> &[u8] {
		&self.raw[92..]
	}
}
