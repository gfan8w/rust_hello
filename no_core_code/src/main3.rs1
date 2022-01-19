#![feature(lang_items, core_intrinsics, rustc_private)]
#![feature(start)]
#![no_std]
use core::intrinsics;
use core::panic::PanicInfo;

// Pull in the system libc library for what crt0.o likely requires.
extern crate libc;

// Entry point for this program.
#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    6
}

// These functions are used by the compiler, but not
// for a bare-bones hello world. These are normally
// provided by libstd.
#[lang = "eh_personality"]
#[no_mangle]
pub extern fn rust_eh_personality() {
}

#[lang = "panic_impl"]
#[no_mangle]
pub extern fn rust_begin_panic(info: &PanicInfo) -> ! {
    unsafe { intrinsics::abort() }
}


//这里演示一个 不依赖stdlib的程序。 主要目的是了解一下 lang_item这个东西
// 把文件名改为 main.rs,
// 在no_core_code 目录下，编译：cargo +nightly build -Z unstable-options
// 运行程序：$ ../target/debug/no_core_code
// 检查返回值： echo $?   ,应该输出 6

//参考：https://doc.rust-lang.org/beta/unstable-book/language-features/lang-items.html







