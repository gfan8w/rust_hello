#![feature(lang_items, box_syntax, start, libc, core_intrinsics, rustc_private)]
#![no_std]
use core::intrinsics;
use core::panic::PanicInfo;

extern crate libc;

#[lang = "owned_box"]
pub struct Box<T>(*mut T);

#[lang = "exchange_malloc"]
unsafe fn allocate(size: usize, _align: usize) -> *mut u8 {
    let p = libc::malloc(size as libc::size_t) as *mut u8;

    // Check if `malloc` failed:
    if p as usize == 0 {
        intrinsics::abort();
    }

    p
}

#[lang = "box_free"]
unsafe fn box_free<T: ?Sized>(ptr: *mut T) {
    libc::free(ptr as *mut libc::c_void)
}

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    let _x = box 1;

    7
}

#[lang = "eh_personality"] extern fn rust_eh_personality() {}
#[lang = "panic_impl"] extern fn rust_begin_panic(info: &PanicInfo) -> ! { unsafe { intrinsics::abort() } }
#[no_mangle] pub extern fn rust_eh_register_frames () {}
#[no_mangle] pub extern fn rust_eh_unregister_frames () {}


//这里演示一个 不依赖stdlib的程序。 主要目的是了解一下 lang_item这个东西
// 把文件名改为 main.rs,
// 在no_core_code 目录下，编译：cargo +nightly build -Z unstable-options
// 运行程序：$ ../target/debug/no_core_code
// 检查返回值： echo $?   ,应该输出 7

//参考：https://doc.rust-lang.org/beta/unstable-book/language-features/lang-items.html





