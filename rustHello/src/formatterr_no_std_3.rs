//#![feature(lang_items)]
//#![no_std]



/// 在no_std环境下 如何使用format!
/// 参考：https://stackoverflow.com/questions/39488327/how-to-format-output-to-a-byte-array-with-no-std-and-no-allocator

use std::io::Write;

//use core::panic::PanicInfo;

//#[lang = "eh_personality"]
//extern "C" fn eh_personality() {}

//#[panic_handler]
//fn panic(info: &PanicInfo) -> ! {
//    loop {}
//}

pub fn run() {
    let x = 123;
    let mut buf = [0 as u8; 20];
    write!(&mut buf[..], "{}", x).expect("Can't write");
    assert_eq!(&buf[0..3], b"123");

}

