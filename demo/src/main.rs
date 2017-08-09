#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(alloc_system)]

extern crate wasm_std;

#[no_mangle]
pub fn call() {
    unsafe { wasm_std::storage_read(); }
}