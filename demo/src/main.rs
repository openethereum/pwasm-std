#![no_std]
#![no_main]
#![feature(lang_items)]

#[lang = "panic_fmt"]
pub fn panic_fmt(_fmt: core::fmt::Arguments, _file_line: &(&'static str, u32)) -> !
{
    loop { }
}

fn main() {
}