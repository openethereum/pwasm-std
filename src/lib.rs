#![no_std]
#![feature(lang_items)]
#![feature(link_args)]

extern "C" fn abort() {
}

#[link_args = "-s NO_EXIT_RUNTIME -s NO_FILESYSTEM"]
extern {
}

#[lang = "panic_fmt"]
pub fn panic_fmt(_fmt: core::fmt::Arguments, _file_line: &(&'static str, u32)) -> !
{
    abort();
    unreachable!();
}

#[link(name = "env")]
extern {
    pub fn storage_read();
    pub fn storage_write();
}