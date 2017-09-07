#![no_std]
#![no_main]

#[macro_use]
extern crate pwasm_std;

use pwasm_std::{CallArgs, storage};
use pwasm_std::hash::H256;

#[no_mangle]
pub fn call(descriptor: *mut u8) {
    let mut call_args = unsafe { CallArgs::from_raw(descriptor) };

    let _ = storage::write(&H256::from([1u8; 32]), &[1u8; 32]).expect("OK");
    let v = storage::read(&H256::from([1u8; 32])).expect("OK");


    let mut vec = vec![0u8; 16384];
    vec[32..64].copy_from_slice(&v);

    *call_args.result_mut() = vec.into_boxed_slice();

    unsafe { call_args.save(descriptor); }
}
