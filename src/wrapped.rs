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

impl AsRef<[u8]> for WrappedArgs {
	fn as_ref(&self) -> &[u8] {
		unsafe {
			slice::from_raw_parts((*self.desc).args_ptr, (*self.desc).args_len)
		}
	}
}

impl ops::Deref for WrappedArgs {
	type Target = [u8];

    fn deref(&self) -> &Self::Target {
		unsafe {
			slice::from_raw_parts((*self.desc).args_ptr, (*self.desc).args_len)
		}
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
