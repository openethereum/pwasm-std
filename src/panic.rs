use core::mem;

#[repr(C)]
struct PanicPayload {
	file_ptr: *const u8,
	file_len: usize,
	line: u32,
	col: u32,
}

#[lang = "panic_fmt"]
pub fn panic_fmt(_fmt: ::core::fmt::Arguments, file: &'static str, line: u32, col: u32) -> ! {
	extern "C" {
		fn panic(payload_ptr: *const u8, payload_len: u32) -> !;
	}
	unsafe {
		let payload = PanicPayload {
			file_ptr: file.as_ptr(),
			file_len: file.len(),
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
