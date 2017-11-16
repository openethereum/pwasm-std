#![cfg(not(feature = "std"))]

use {Vec};
use byteorder::{LittleEndian, ByteOrder};

#[lang = "panic_fmt"]
pub fn panic_fmt(_fmt: ::core::fmt::Arguments, file: &'static str, line: u32, col: u32) -> ! {
	extern "C" {
		fn panic(payload_ptr: *const u8, payload_len: u32) -> !;
	}

	#[cfg(feature = "panic_with_msg")]
	let message = format!("{}", _fmt);

	#[cfg(not(feature = "panic_with_msg"))]
	let message = ::alloc::String::new();

	let mut payload = Vec::new();
	write_str(&mut payload, message.as_bytes());
	write_str(&mut payload, file.as_bytes());
	write_u32(&mut payload, line);
	write_u32(&mut payload, col);

	unsafe {
		panic(payload.as_ptr(), payload.len() as u32);
	}
}

fn write_u32(payload: &mut Vec<u8>, val: u32) {
	let mut val_bytes = [0u8; 4];
	LittleEndian::write_u32(&mut val_bytes, val);
	payload.extend(&val_bytes);
}

fn write_str(payload: &mut Vec<u8>, bytes: &[u8]) {
	write_u32(payload, bytes.len() as u32);
	payload.extend(bytes);
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}
