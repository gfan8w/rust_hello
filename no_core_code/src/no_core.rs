#![feature(no_core)]
#![feature(lang_items)]

// Look at me.
// Look at me.
// I'm the libcore now.
#![no_core]

// Tell the compiler to link to appropriate runtime libs
// (This way I don't have to specify `-l` flags explicitly)
#[cfg(target_os = "linux")]
#[link(name = "c")]
extern {}
#[cfg(target_os = "macos")]
#[link(name = "System")]
extern {}

// Compiler needs these to proceed
#[lang = "sized"]
pub trait Sized {}
#[lang = "copy"]
pub trait Copy {}


pub trait Termination {
    fn report(self) -> i32;
}

impl Termination for () {
    fn report(self) -> i32 {
        0
    }
}


// `main` isn't the actual entry point, `start` is.
#[lang = "start"]
fn lang_start<T: Termination + 'static>(
    main: fn() -> T,
    argc: isize,
    argv: *const *const u8,
) -> isize {
    // we can't really do much in this benighted hellhole of
    // an environment without bringing in more libraries.
    // We can make syscalls, segfault, and set the exit code.
    // To be sure that this actually ran, let's set the exit code.
    42
}

// still need a main unless we want to use `#![no_main]`
// won't actually get called; `start()` is supposed to call it
fn main() {}


// 在本级目录下编译： rustc +nightly  -Z unstable-options -C opt-level=2 src/no_core.rs -o ./no_std_main