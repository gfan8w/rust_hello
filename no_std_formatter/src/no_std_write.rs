#![feature(lang_items)]
#![no_std]

use core::panic::PanicInfo;

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop {}
}

fn main() {
    let x = 123;
    let mut buf = [0 as u8; 20];
    write!(&mut buf[..], "{}", x).expect("Can't write");
    assert_eq!(&buf[0..3], b"123");
}


// 编译本文件： rustc +nightly  -Z unstable-options -C opt-level=2 src/no_std_write.rs -o ./no_std_main
// 报错报 &mut [u8] 这个类型没有 `write_fmt` 方法







