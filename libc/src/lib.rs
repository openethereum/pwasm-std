#![no_std]

extern "C" {
	fn ext_memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8;
	fn ext_memmove(dest: *mut u8, src: *const u8, n: usize) -> *mut u8;
	fn ext_memset(dest: *mut u8, c: i32, n: usize) -> *mut u8;
	fn ext_malloc(size: usize) -> *mut u8;
	fn ext_free(ptr: *mut u8);
}

// Declaring these function here prevents Emscripten from including it's own verisons
// into final binary.

#[no_mangle]
pub unsafe extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
	ext_memcpy(dest, src, n)
}

#[no_mangle]
pub unsafe extern "C" fn memmove(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
	ext_memmove(dest, src, n)
}

#[no_mangle]
pub unsafe extern "C" fn memset(dest: *mut u8, c: i32, n: usize) -> *mut u8 {
	ext_memset(dest, c, n)
}

#[no_mangle]
pub unsafe extern "C" fn malloc(size: usize) -> *mut u8 {
	ext_malloc(size)
}

#[no_mangle]
pub unsafe extern "C" fn free(ptr: *mut u8) {
	ext_free(ptr);
}
