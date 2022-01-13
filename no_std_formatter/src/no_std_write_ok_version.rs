#![feature(lang_items)]
#![feature(start)]
#![no_std]

use core::panic::PanicInfo;

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop {}
}

use core::fmt::{self, Write};

struct Wrapper<'a> {
    buf: &'a mut [u8],
    offset: usize,
}

impl<'a> Wrapper<'a> {
    fn new(buf: &'a mut [u8]) -> Self {
        Wrapper {
            buf: buf,
            offset: 0,
        }
    }
}

impl<'a> fmt::Write for Wrapper<'a> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let bytes = s.as_bytes();

        // Skip over already-copied data
        let remainder = &mut self.buf[self.offset..];
        // Check if there is space remaining (return error instead of panicking)
        if remainder.len() < bytes.len() { return Err(core::fmt::Error); }
        // Make the two slices the same length
        let remainder = &mut remainder[..bytes.len()];
        // Copy
        remainder.copy_from_slice(bytes);

        // Update offset to avoid overwriting
        self.offset += bytes.len();

        Ok(())
    }
}

#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    let x = 123;
    let mut buf = [0 as u8; 20];
    write!(Wrapper::new(&mut buf), "{}", x).expect("Can't write");
    assert_eq!(&buf[0..3], b"123");

    0
}


// 编译本文件： rustc +nightly  -Z unstable-options -C opt-level=2 src/no_std_write_ok_version.rs -o ./no_std_main
// 这里的 offset是重置的，所以只能format一次。多次操作会覆盖前面的。

// 这里有编译错误，暂时没解决，后面再看







