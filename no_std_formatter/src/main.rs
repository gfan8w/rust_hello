
fn main() {
    println!("Hello, world!");
}

// 编译本文件： rustc +nightly  -Z unstable-options -C opt-level=2 src/main.rs -o ./std_main.exe

#[cfg(test)]
mod s_t_d {

    use std::io::Write;

    #[test]
    fn run_std() {
        let x = 123;
        let mut buf = [0 as u8; 20];
        let aa = &buf[..];
        write!(&mut buf[..], "{}",x).expect("Can't write");
        println!("{:?}",&buf[..]);
        assert_eq!(&buf[0..3], b"123")
    }


}












