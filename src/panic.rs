#![cfg(not(feature = "std"))]

extern "C" {
	fn panic(payload_ptr: *const u8, payload_len: u32) -> !;
}

/// Overrides the default panic_fmt
#[cfg(not(feature = "panic_with_msg"))]
#[no_mangle]
#[lang = "panic_fmt"]
pub extern fn panic_fmt() -> ! {
	let msg = "Panic has occurred in Wasm module. Build Wasm with a 'panic_with_msg' feature for details";
	unsafe {
		panic(msg.as_ptr(), msg.len() as u32);
	}
}

/// Overrides the default panic_fmt
#[cfg(feature = "panic_with_msg")]
#[no_mangle]
#[lang = "panic_fmt"]
pub extern fn panic_fmt(_fmt: ::core::fmt::Arguments, file: &'static str, line: u32, col: u32) -> ! {
	use Vec;
	use byteorder::{LittleEndian, ByteOrder};

	let msg = format!("{}", _fmt);

	let mut sink = Sink::new(
		4 + msg.as_bytes().len() +		// len + [msg]
		4 + file.as_bytes().len() +		// len + [file]
		4 +								// line
		4								// col
	);
	sink.write_str(msg.as_bytes());
	sink.write_str(file.as_bytes());
	sink.write_u32(line);
	sink.write_u32(col);

	unsafe {
		panic(sink.as_ptr(), sink.len() as u32);
	}

	struct Sink {
		buf: Vec<u8>,
		pos: usize
	}

	impl Sink {
		#[inline(always)]
		fn new(capacity: usize) -> Sink {
			let mut buf = Vec::with_capacity(capacity);
			buf.resize(capacity, 0);
			Sink {
				buf: buf,
				pos: 0,
			}
		}

		#[inline(always)]
		fn reserve(&mut self, len: usize) -> &mut [u8] {
			let dst = &mut self.buf[self.pos..self.pos+len];
			self.pos += len;
			dst
		}

		#[inline(always)]
		fn write_u32(&mut self, val: u32) {
			LittleEndian::write_u32(self.reserve(4), val);
		}

		#[inline(always)]
		fn write_str(&mut self, bytes: &[u8]) {
			self.write_u32(bytes.len() as u32);
			self.reserve(bytes.len()).copy_from_slice(bytes)
		}
	}

	impl ::core::ops::Deref for Sink {
		type Target = [u8];
		fn deref(&self) -> &[u8] {
			&self.buf
		}
	}
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}
