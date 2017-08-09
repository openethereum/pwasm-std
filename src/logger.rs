mod external {
    #[link(name = "env")]
    extern {
        pub fn debug(str_ptr: *const u8, str_len: u32);
    }
}

pub fn debug(msg: &str) {
    unsafe { external::debug(msg.as_ptr(), msg.len() as u32); }
}
