use core::mem;

#[repr(C)]
struct PanicPayload {
	msg_len: usize,
	msg_ptr: *const u8,
	file_len: usize,
	file_ptr: *const u8,
	line: u32,
	col: u32,
}

#[lang = "panic_fmt"]
pub fn panic_fmt(_fmt: ::core::fmt::Arguments, file: &'static str, line: u32, col: u32) -> ! {
	extern "C" {
		fn panic(payload_ptr: *const u8, payload_len: u32) -> !;
	}

	#[cfg(panic_with_msg)]
	let message = format!("{}", _fmt);

	#[cfg(not(panic_with_msg))]
	let message = &[];

	unsafe {
		let payload = PanicPayload {
			msg_len: message.len(),
			msg_ptr: message.as_ptr(),
			file_len: file.len(),
			file_ptr: file.as_ptr(),
			line: line,
			col: col,
		};
		panic(
			&payload as *const PanicPayload as *const u8,
			mem::size_of::<PanicPayload>() as u32,
		);
	}
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}
