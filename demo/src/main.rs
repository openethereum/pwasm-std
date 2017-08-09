#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(alloc)]
#![feature(allocator_internals)]
#![feature(alloc_system)]

extern crate wasm_std;

use wasm_std::{Box, CallArgs, storage};

#[no_mangle]
pub fn call(descriptor: *mut u8) {
    let mut call_args = unsafe { CallArgs::from_raw(descriptor) };

    let k = [0u8; 32];
    let mut v = [0u8; 32];
    storage::read(&k, &mut v);

    *call_args.result_mut() = Box::new(v);

    unsafe { call_args.save(descriptor); }
}