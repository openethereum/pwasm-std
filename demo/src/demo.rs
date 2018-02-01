#![no_std]

extern crate pwasm_std;

#[no_mangle]
pub fn call(descriptor: *mut u8) {
	let (_, result) = unsafe { pwasm_std::parse_args(descriptor) };

	pwasm_std::logger::debug("Returing hash of 'something'");
	let hash = pwasm_std::keccak("something");

	result.done(hash.to_vec());
}
