#![cfg_attr(feature = "strict", deny(warnings))]
#![no_std]
#![crate_type = "rlib"]
#![feature(global_allocator)]
#![feature(alloc)]
#![feature(allocator_api)]

extern crate alloc;
extern crate wasm_libc;

use alloc::heap::{Alloc, Layout, AllocErr};

pub struct WasmAllocator;

unsafe impl<'a> Alloc for &'a WasmAllocator {
    unsafe fn alloc(&mut self, layout: Layout) -> Result<*mut u8, AllocErr> {
        Ok(wasm_libc::malloc(layout.size()))
    }

    unsafe fn dealloc(&mut self, ptr: *mut u8, _layout: Layout) {
        wasm_libc::free(ptr)
    }
}

#[global_allocator]
static ALLOCATOR: WasmAllocator = WasmAllocator;
