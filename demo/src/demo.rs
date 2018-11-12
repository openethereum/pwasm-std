#![no_std]

#[macro_use] extern crate owasm_std;

#[no_mangle]
pub fn call() {
	owasm_std::logger::debug("Returing hash of 'something'");
	let hash = owasm_std::keccak("something");

	owasm_std::logger::debug(&format!("{:?}", hash.as_ref()));
}
