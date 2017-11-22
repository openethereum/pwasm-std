//! Safe wrapper around externalities invokes.

use hash::H256;
use bigint::U256;
use hash::Address;

/// Generic wasm error
#[derive(Debug)]
pub struct Error;

mod external {

	#[cfg_attr(not(feature="std"), link(name = "env"))]
	extern "C" {
		// Various call variants

		/// Direct/classic call.
		/// Correspond to "CALL" opcode in EVM
		pub fn ccall(
			address: *const u8,
			val_ptr: *const u8,
			input_ptr: *const u8,
			input_len: u32,
			result_ptr: *mut u8,
			result_len: u32,
		) -> i32;

		/// Delegate call.
		/// Corresponds to "CALLCODE" opcode in EVM
		pub fn dcall(
			address: *const u8,
			input_ptr: *const u8,
			input_len: u32,
			result_ptr: *mut u8,
			result_len: u32,
		) -> i32;

		/// Static call.
		/// Corresponds to "STACICCALL" opcode in EVM
		pub fn scall(
			address: *const u8,
			input_ptr: *const u8,
			input_len: u32,
			result_ptr: *mut u8,
			result_len: u32,
		) -> i32;

		// enviromental blockchain functions (runtume might not provide all of these!)

		/// Block hash of the specific block
		pub fn blockhash(number: i64, dest: *mut u8);

		pub fn balance(address: *const u8, dest: *mut u8);

		pub fn coinbase(dest: *mut u8);

		pub fn timestamp() -> i64;

		pub fn blocknumber() -> i64;

		pub fn difficulty(dest: *mut u8);

		pub fn gaslimit(dest: *mut u8);

		pub fn sender(dest: *mut u8);

		pub fn address(dest: *mut u8);

		pub fn value(dest: *mut u8);

		pub fn origin(dest: *mut u8);

		pub fn elog(topic_ptr: *const u8, topic_count: u32, data_ptr: *const u8, data_len: u32);

		pub fn create(endowment: *const u8, code_ptr: *const u8, code_len: u32, result_ptr: *mut u8) -> i32;

		pub fn suicide(refund: *const u8) -> !;
	}
}

/// Halt execution and register account for deletion.
///
/// Value of the current account will be tranfered to `refund` address.
pub fn suicide(refund: &Address) -> ! {
	unsafe { external::suicide(refund.as_ptr()); }
}

/// Returns balance of the given address.
pub fn balance(address: &Address) -> U256 {
	unsafe { fetch_u256(|x| external::balance(address.as_ptr(), x) ) }
}

/// Create a new account with the given code.
///
/// # Errors
///
/// Returns [`Error`] in case contract constructor failed.
///
/// [`Error`]: struct.Error.html
pub fn create(endowment: U256, code: &[u8]) -> Result<Address, Error> {
	let mut endowment_arr = [0u8; 32];
	endowment.to_big_endian(&mut endowment_arr);
	let mut result = Address::new();
	unsafe {
		if external::create(endowment_arr.as_ptr(), code.as_ptr(), code.len() as u32, (&mut result).as_mut_ptr()) == 0 {
			Ok(result)
		} else {
			Err(Error)
		}
	}
}

/// Message-call into an account.
pub fn call(address: &Address, value: U256, input: &[u8], result: &mut [u8]) -> Result<(), Error> {
	let mut value_arr = [0u8; 32];
	value.to_big_endian(&mut value_arr);
	unsafe {
		match external::ccall(address.as_ptr(), value_arr.as_ptr(), input.as_ptr(), input.len() as u32, result.as_mut_ptr(), result.len() as u32) {
			0 => Ok(()),
			_ => Err(Error),
		}
	}
}

/// Like [`call`], but with code at the given `address`.
///
/// Effectively this function is like calling current account but with
/// different code (i.e. like `DELEGATECALL` EVM instruction).
///
/// [`call`]: fn.call.html
pub fn call_code(address: &Address, input: &[u8], result: &mut [u8]) -> Result<(), Error> {
	unsafe {
		match external::dcall(address.as_ptr(), input.as_ptr(), input.len() as u32, result.as_mut_ptr(), result.len() as u32) {
			0 => Ok(()),
			_ => Err(Error),
		}
	}
}

/// Like [`call`], but this call and any of it's subcalls are disallowed to modify any storage.
///
/// [`call`]: fn.call.html
pub fn static_call(address: &Address, input: &[u8], result: &mut [u8]) -> Result<(), Error> {
	unsafe {
		match external::scall(address.as_ptr(), input.as_ptr(), input.len() as u32, result.as_mut_ptr(), result.len() as u32) {
			0 => Ok(()),
			_ => Err(Error),
		}
	}
}

/// Returns the hash of one of the 256 most recent complete blocks.
pub fn block_hash(block_number: u64) -> H256 {
	let mut res = H256::zero();
	unsafe {
		external::blockhash(block_number as i64, res.as_mut_ptr())
	}
	res
}

unsafe fn fetch_address<F>(f: F) -> Address where F: Fn(*mut u8) {
	let mut res = Address::zero();
	f(res.as_mut_ptr());
	res
}

unsafe fn fetch_u256<F>(f: F) -> U256 where F: Fn(*mut u8) {
	let mut res = [0u8; 32];
	f(res.as_mut_ptr());
	U256::from_big_endian(&res)
}

/// Get the block’s beneficiary address (i.e miner's account address).
pub fn coinbase() -> Address {
	unsafe { fetch_address(|x| external::coinbase(x) ) }
}

/// Get the block's timestamp.
///
/// It can be viewed as an output of Unix's `time()` function at
/// current block's inception.
pub fn timestamp() -> u64 {
	unsafe { external::timestamp() as u64 }
}

/// Get the block's number.
///
/// This value represents number of ancestor blocks.
/// The genesis block has a number of zero.
pub fn block_number() -> u64 {
	unsafe { external::blocknumber()  as u64 }
}

/// Get the block's difficulty.
pub fn difficulty() -> U256 {
	unsafe { fetch_u256(|x| external::difficulty(x) ) }
}

/// Get the block's gas limit.
pub fn gas_limit() -> U256 {
	unsafe { fetch_u256(|x| external::gaslimit(x) ) }
}

/// Get caller address.
///
/// This is the address of the account that is directly responsible for this execution.
pub fn sender() -> Address {
	unsafe { fetch_address(|x| external::sender(x) ) }
}

/// Get execution origination address.
///
/// This is the sender of original transaction.
/// It is never an account with non-empty associated code.
pub fn origin() -> Address {
	unsafe { fetch_address(|x| external::origin(x) ) }
}

/// Get deposited value by the instruction/transaction responsible for this execution.
pub fn value() -> U256 {
	unsafe { fetch_u256(|x| external::value(x) ) }
}

/// Get address of currently executing account.
pub fn address() -> Address {
	unsafe { fetch_address(|x| external::address(x) ) }
}

/// Creates log entry with given topics and data.
///
/// There could be only up to 4 topics.
///
/// # Panics
///
/// If `topics` contains more than 4 elements then this function will trap.
pub fn log(topics: &[H256], data: &[u8]) {
	unsafe { external::elog(topics.as_ptr() as *const u8, topics.len() as u32, data.as_ptr(), data.len() as u32); }
}
