//! Logger extensions for pwasm-std

mod external {
	#[cfg_attr(not(feature="std"), link(name = "env"))]
	extern {
		pub fn debug(str_ptr: *const u8, str_len: u32);
	}
}

/// Log debug message to the runtime
pub fn debug(msg: &str) {
	unsafe { external::debug(msg.as_ptr(), msg.len() as u32); }
}
