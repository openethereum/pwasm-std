use core::{slice, mem, ops};
use {Vec};

#[repr(C)]
struct Descriptor {
	args_ptr: *const u8,
	args_len: usize,
	result_ptr: *const u8,
	result_len: usize,
}

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
				// It is UB to create a slice with the null ptr.
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

pub struct WrappedResult {
	desc: *mut Descriptor
}

impl WrappedResult {
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

pub unsafe fn parse_args(ptr: *mut u8) -> (WrappedArgs, WrappedResult) {
	let desc = ptr as *mut Descriptor;
	let args = WrappedArgs { desc: desc };
	let result = WrappedResult { desc: desc };
	(args, result)
}
