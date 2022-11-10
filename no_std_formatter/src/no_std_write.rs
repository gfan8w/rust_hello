#![feature(lang_items)]
#![no_std]

// #![] 是 叫 InnerAttribute
// #[] 是叫 OuterAttribute
// 他们2个区别不在于他们的行为，而在于他们作用到的对象上
// Inner 与 Outer的区别在于：作用在在本文件中的叫 Inner。Outer的意思是 定义在别处，但作用在外部对象上的。
// 例如 panic_handler 是在其他耨个地方定义的，但作用在此文件的panic函数上


// The difference between these two type of attributes is not the behavior, but the item it is applied to.
// The outer attribute is placed outside something - i.e. before the struct, or function, or module. This is usually what you want.
// The inner attribute is placed inside something. This is the only way to place attributes on the
// crate (by writing them in the root of it), and AFAIK this use-case is the only one where you'll really want them.


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







