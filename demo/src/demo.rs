#![no_std]

#[macro_use] extern crate pwasm_std;

#[no_mangle]
pub fn call() {
	pwasm_std::logger::debug("Returing hash of 'something'");
	let hash = pwasm_std::keccak("something");

	pwasm_std::logger::debug(&format!("{:?}", hash.as_ref()));
}
