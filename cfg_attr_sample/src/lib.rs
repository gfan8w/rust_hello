
/// 条件编译 模块引入，不同环境，有三个不同的文件：linux.rs, windows.rs,macos.rs
/// target_os = "macos" 表示是模式匹配，后一部分是 attr，表示如果匹配成功，后面的attr生效
#[cfg_attr(target_os = "linux", path = "linux.rs")]
#[cfg_attr(windows, path = "windows.rs")]
#[cfg_attr(target_os = "macos", path = "macos.rs")]
mod osshow;  // 这里要加上pub ，外部可以访问这个 osshow模块，os_different::osshow::osshow::show(); 这样访问。现在这里去掉pub，下面再重新导出。


pub use osshow::osshow::*; // 这里使用重新导出把内部的show() 导出，否则外部使用的时候，必须：os_different::osshow::osshow::show(); 名字太长

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        show();
        assert_eq!(4, 4);
    }
}
