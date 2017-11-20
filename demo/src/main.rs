#![no_std]
#![no_main]

#[macro_use]
extern crate pwasm_std;

use pwasm_std::hash::H256;
use pwasm_std::storage;

#[no_mangle]
pub fn call(descriptor: *mut u8) {
    let (_, result) = unsafe { pwasm_std::parse_args(descriptor) };

    let _ = storage::write(&H256::from([1u8; 32]), &[1u8; 32]);
    let v = storage::read(&H256::from([1u8; 32]));

    let mut vec = vec![0u8; 16384];
    vec[32..64].copy_from_slice(&v);

    result.done(vec);
}
