#![no_std]
#![no_main]

#[macro_use]
extern crate pwasm_std;

use pwasm_std::{CallArgs, storage};

#[no_mangle]
pub fn call(descriptor: *mut u8) {
    let mut call_args = unsafe { CallArgs::from_raw(descriptor) };

    let k = [0u8; 32];
    let mut v = [0u8; 32];
    storage::read(&k, &mut v);

    let mut vec = vec![0u8; 16384];
    vec[32..64].copy_from_slice(&v);

    *call_args.result_mut() = vec.into_boxed_slice();

    unsafe { call_args.save(descriptor); }
}