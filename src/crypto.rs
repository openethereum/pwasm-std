use tiny_keccak::Keccak;
use hash::H256;

/// Compute keccak hash.
///
/// # Examples
///
/// ```rust
/// use owasm_std::keccak;
///
/// let hash = keccak("hello world");
///
/// let expected = &[
/// 	71, 23, 50, 133, 168, 215, 52, 30,
/// 	94, 151, 47, 198, 119, 40, 99, 132,
/// 	248, 2, 248, 239, 66, 165, 236, 95,
/// 	3, 187, 250, 37, 76, 176, 31, 173
/// ];
///
/// assert_eq!(hash.as_ref(), expected);
/// ```
pub fn keccak<T>(input: T) -> H256 where T: AsRef<[u8]> {
	let mut keccak = Keccak::new_keccak256();
	let mut res = H256::new();
	keccak.update(input.as_ref());
	keccak.finalize(&mut res);
	res
}
