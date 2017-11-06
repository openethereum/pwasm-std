#![cfg_attr(feature = "strict", deny(warnings))]
#![no_std]
#![crate_type = "rlib"]
#![feature(global_allocator)]
#![feature(alloc)]
#![feature(allocator_api)]

extern crate alloc;

use alloc::heap::{Alloc, Layout, AllocErr};

pub mod stubs {
	// These function stubs are needed to convince Emscripten not to provide
	// its own implementations of these functions. They are expected to
	// be externilized by wasm-build.
	// It is important to mark these functions as `inline(never)`.

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
