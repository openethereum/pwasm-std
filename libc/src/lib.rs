#![warn(missing_docs)]
#![cfg_attr(feature = "strict", deny(warnings))]
#![no_std]

//! libc crate

#[macro_use]
extern crate cfg_if;

cfg_if! {
	if #[cfg(feature="ext_memops")] {
		extern "C" {
			fn ext_memcmp(cx: *const u8, ct: *const u8, n: usize) -> i32;
			fn ext_memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8;
			fn ext_memmove(dest: *mut u8, src: *const u8, n: usize) -> *mut u8;
			fn ext_memset(dest: *mut u8, c: i32, n: usize) -> *mut u8;
		}

		// Declaring these function here prevents Emscripten from including it's own verisons
		// into final binary.

		/// memcmp extern
		#[no_mangle]
		pub unsafe extern "C" fn memcmp(cx: *const u8, ct: *const u8, n: usize) -> i32 {
			ext_memcmp(cx, ct, n)
		}

		/// memcpy extern
		#[no_mangle]
		pub unsafe extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
			ext_memcpy(dest, src, n)
		}

		/// memmove extern
		#[no_mangle]
		pub unsafe extern "C" fn memmove(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
			ext_memmove(dest, src, n)
		}

		/// memset extern
		#[no_mangle]
		pub unsafe extern "C" fn memset(dest: *mut u8, c: i32, n: usize) -> *mut u8 {
			ext_memset(dest, c, n)
		}
	} else {
		extern crate rlibc;
	}
}
