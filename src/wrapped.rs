use core::{slice, mem, ops};
use {Vec, read_u32, read_ptr_mut, write_u32, write_ptr};

pub struct WrappedArgs {
	inner: Vec<u8>,
}

impl AsRef<[u8]> for WrappedArgs {
	fn as_ref(&self) -> &[u8] {
		&self.inner
	}
}

impl ops::Deref for WrappedArgs {
	type Target = [u8];

    fn deref(&self) -> &Self::Target {
		&self.inner
	}
}

impl WrappedArgs {
	unsafe fn from_raw(ptr: *mut u8, len: u32) -> Self {
		WrappedArgs {
			inner: Vec::from_raw_parts(ptr, len as usize, len as usize),
		}
	}
}

impl Drop for WrappedArgs {
	fn drop(&mut self) {
		mem::forget(mem::replace(&mut self.inner, Vec::new()))
	}
}

pub struct WrappedResult {
	inner: *mut u8,
}

impl WrappedResult {
	pub fn done(self, val: Vec<u8>) {
		if val.len() > 0 {
			let mut dst = unsafe { slice::from_raw_parts_mut(self.inner, 2 * 4) };

			write_ptr(&mut dst[0..4], val.as_ptr() as *mut _);
			write_u32(&mut dst[4..8], val.len() as u32);
			// managed in calling code
			mem::forget(val);
		}
	}
}

pub unsafe fn parse_args(ptr: *mut u8) -> (WrappedArgs, WrappedResult) {
	let desc_slice = slice::from_raw_parts(ptr, 4 * 4);

	let input_ptr = read_ptr_mut(&desc_slice[0..4]);
	let input_len = read_u32(&desc_slice[4..8]);

	let result_ptr = ptr.offset(8);

	(
		WrappedArgs::from_raw(input_ptr, input_len),
		WrappedResult { inner: result_ptr }
	)
}
