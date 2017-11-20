use tiny_keccak::Keccak;
use hash::H256;

/// Compute keccak hash
pub fn keccak<T>(input: T) -> H256 where T: AsRef<[u8]> {
	let mut keccak = Keccak::new_keccak256();
	let mut res = H256::new();
	keccak.update(input.as_ref());
	keccak.finalize(&mut res);
	res
}