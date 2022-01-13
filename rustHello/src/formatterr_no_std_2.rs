#![crate_type = "dylib"]
#![no_std]

// #[macro_use]
// extern crate alloc;





/// 在no_std环境下 如何使用format!
/// 参考：https://stackoverflow.com/questions/50200268/how-can-i-use-the-format-macro-in-a-no-std-environment
fn thing() -> String {
    let text = format!("example {:.1} test {:x} words {}", 1, 2, 3);
    text
}

pub fn run() {
    //let thing =thing();
    //println!("{}",thing)
}

