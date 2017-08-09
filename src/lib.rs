#![no_std]
#![feature(lang_items)]
#![feature(link_args)]
#![feature(alloc_system)]
#![feature(alloc)]
#![feature(allocator_internals)]
#![default_lib_allocator]

extern crate alloc_system;
extern crate alloc;

use core::{slice, ptr, mem};
use core::ops::Shl;
pub use alloc::boxed::Box;

/// Wrapper over storage read/write externs
/// Storage api is a key-value storage where both key and value are 32 bytes in len
pub mod storage;

/// Safe wrapper around debug logging
pub mod logger;

/// Safe wrapper around externalities invokes
pub mod ext;

extern "C" fn abort() {
}

#[lang = "panic_fmt"]
pub fn panic_fmt(_fmt: core::fmt::Arguments, _file_line: &(&'static str, u32)) -> !
{
    abort();
    unreachable!();
}

#[link_args = "-s NO_EXIT_RUNTIME=1 -s NO_FILESYSTEM=1 -s USE_PTHREADS=0"]
extern {
}

/// Safe wrapper for call context
pub struct CallArgs {
    context: Box<[u8]>,
    result: Box<[u8]>,
}

unsafe fn read_ptr_mut(slc: &[u8]) -> *mut u8 {
	ptr::null_mut().offset(read_u32(slc) as isize)
}

pub fn read_u32(slc: &[u8]) -> u32 {
	(slc[0] as u32) + (slc[1] as u32).shl(8) + (slc[2] as u32).shl(16) + (slc[3] as u32).shl(24)
}

pub fn write_u32(dst: &mut [u8], val: u32) {
	dst[0] = (val & 0x000000ff) as u8;
	dst[1] = ((val & 0x0000ff00) >> 8) as u8;
	dst[2] = ((val & 0x00ff0000) >> 16) as u8;
	dst[3] = ((val & 0xff000000) >> 24) as u8;
}

fn write_ptr(dst: &mut [u8], ptr: *mut u8) {
	// todo: consider: add assert that arch is 32bit
	write_u32(dst, ptr as usize as u32);
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

	pub fn address(&self) -> [u8; 20] {
		let mut addr = [0u8; 20];
		addr.copy_from_slice(&self.raw[0..20]);
		addr
	}

	pub fn sender(&self) -> [u8; 20] {
		let mut sender = [0u8; 20];
		sender.copy_from_slice(&self.raw[20..40]);
		sender
	}

	pub fn origin(&self) -> [u8; 20] {
		let mut origin = [0u8; 20];
		origin.copy_from_slice(&self.raw[40..60]);
		origin
	}

	pub fn value(&self) -> [u8; 32] {
		let mut value = [0u8; 32];
		value.copy_from_slice(&self.raw[60..92]);
		value
	}

	pub fn args(&self) -> &[u8] {
		&self.raw[92..]
	}
}