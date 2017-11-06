// These function stubs are needed to convince Emscripten not to provide
// his own implementations of these functions. They are expected to
// be externilized by wasm-build.
// It is important to mark these functions as `inline(never)`.

#[inline(never)]
#[no_mangle]
pub unsafe extern "C" fn memcpy(_dest: *mut u8, _src: *const u8, _n: usize) -> *mut u8 {
    panic!()
}

#[inline(never)]
#[no_mangle]
pub unsafe extern "C" fn memmove(_dest: *mut u8, _src: *const u8, _n: usize) -> *mut u8 {
    panic!()
}

#[inline(never)]
#[no_mangle]
pub unsafe extern "C" fn memset(_s: *mut u8, _c: i32, _n: usize) -> *mut u8 {
    panic!()
}
