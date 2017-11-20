use core::{slice, mem, ops};
use Vec;

#[repr(C)]
struct Descriptor {
	args_ptr: *const u8,
	args_len: usize,
	result_ptr: *const u8,
	result_len: usize,
}

/// Input data of a contract.
///
/// Basically it can be viewed as
/// a byte slice (`&[u8]`) and it has `Deref<Target=[u8]>` impl indeed.
///
/// You should use [`parse_args`] to acquire `WrappedArgs`.
///
/// # Examples
///
/// ```rust
/// use pwasm_std::WrappedArgs;
///
/// fn takes_slice(input: &[u8]) {
/// 	// ...
/// 	# input.len(); // to silence unused var warnings
/// }
///
/// #[no_mangle]
/// pub fn call(descriptor: *mut u8) {
/// 	let (input, result): (WrappedArgs, _) = unsafe { pwasm_std::parse_args(descriptor) };
/// 	takes_slice(&input);
/// }
/// ```
///
/// [`parse_args`]: fn.parse_args.html
pub struct WrappedArgs {
	desc: *const Descriptor
}

impl ops::Deref for WrappedArgs {
	type Target = [u8];
    fn deref(&self) -> &Self::Target {
		unsafe {
			let ptr = (*self.desc).args_ptr;
			let len = (*self.desc).args_len;
			if len == 0 {
				// It is UB to create a slice with null ptr.
				&[]
			} else {
				slice::from_raw_parts(ptr, len)
			}
		}
	}
}

impl AsRef<[u8]> for WrappedArgs {
	fn as_ref(&self) -> &[u8] {
		&*self
	}
}

/// Writeable handle of execution results.
///
/// You can use this handle to write execution results of your contract.
pub struct WrappedResult {
	desc: *mut Descriptor
}

impl WrappedResult {
	/// Finalize writing result into the descriptor
	pub fn done(self, val: Vec<u8>) {
		unsafe {
			if !val.is_empty() {
				(*self.desc).result_ptr = val.as_ptr();
				(*self.desc).result_len = val.len();
			} else {
				(*self.desc).result_len = 0;
			}
		}
		mem::forget(val);
	}
}

/// Parse decriptor into wrapped args and result.
///
/// # Safety
///
/// `ptr` should be non-null and point to a valid descriptor.
///
/// # Examples
///
/// ```rust
/// #[no_mangle]
/// pub fn call(descriptor: *mut u8) {
/// 	let (input, result) = unsafe { pwasm_std::parse_args(descriptor) };
/// 	let echo: Vec<u8> = input.to_vec();
/// 	result.done(echo);
/// }
/// ```
///
pub unsafe fn parse_args(ptr: *mut u8) -> (WrappedArgs, WrappedResult) {
	let desc = ptr as *mut Descriptor;
	let args = WrappedArgs { desc: desc };
	let result = WrappedResult { desc: desc };
	(args, result)
}
