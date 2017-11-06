#![cfg_attr(feature = "strict", deny(warnings))]
#![no_std]
#![crate_type = "rlib"]
#![feature(global_allocator)]
#![feature(alloc)]
#![feature(allocator_api)]

extern crate alloc;

use alloc::heap::{Alloc, Layout, AllocErr};

pub mod stubs {
	// Declare stubs for `malloc` and `free`.
	// This is needed to convince Emscripten not to provide
	// his own implementations of these functions.
	// It is important to mark this functions as `inline(never)`.

	#[inline(never)]
	#[no_mangle]
	pub fn malloc(_size: usize) -> *mut u8 {
		panic!()
	}

	#[inline(never)]
	#[no_mangle]
	pub fn free(_ptr: *mut u8) {
		panic!()
	}
}

pub struct WasmAllocator;

unsafe impl<'a> Alloc for &'a WasmAllocator {
    unsafe fn alloc(&mut self, layout: Layout) -> Result<*mut u8, AllocErr> {
        Ok(stubs::malloc(layout.size()))
    }

    unsafe fn dealloc(&mut self, ptr: *mut u8, _layout: Layout) {
        stubs::free(ptr)
    }
}

#[global_allocator]
static ALLOCATOR: WasmAllocator = WasmAllocator;
