use core::fmt::Arguments;

// Needed for rust to compile

#[lang = "panic_fmt"]
#[no_mangle]
pub fn rust_begin_panic(_msg: Arguments, _file: &'static str, _line: u32) -> ! {
    loop {}
}


