#![cfg_attr(feature = "strict", deny(warnings))]
#![no_std]
#![crate_type = "rlib"]
#![feature(global_allocator)]
#![feature(alloc)]
#![feature(allocator_api)]

extern crate alloc;

use alloc::heap::{Alloc, Layout, AllocErr};

extern "C" {
    fn malloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}

pub struct WamsAllocator;

unsafe impl<'a> Alloc for &'a WamsAllocator {
    unsafe fn alloc(&mut self, layout: Layout) -> Result<*mut u8, AllocErr> {
        Ok(malloc(layout.size()))
    }

    unsafe fn dealloc(&mut self, ptr: *mut u8, _layout: Layout) {
        free(ptr)
    }
}

#[global_allocator]
static ALLOCATOR: WamsAllocator = WamsAllocator;
