use std::fmt::{Debug, Formatter};
use std::io::Write;


// 演示 实现 std::io::Write 这个trait


struct BufBuilder{
    buf: Vec<u8>
}

impl BufBuilder {
    pub fn new() -> Self {
        Self {
            buf: Vec::with_capacity(2)
        }
    }
}

/// 实现 Debug trait，打印字符串
impl Debug for BufBuilder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",String::from_utf8_lossy(&self.buf))
    }
}

/// 对于 Write trait，只需要实现 write 和 flush 两个方法，其他都有缺省实现.
/// 两个特殊的关键字：Self 和 self。Self 代表当前的类型，
/// 比如 File 类型实现了 Write，那么实现过程中使用到的 Self 就指代 File。
/// self 在用作方法的第一个参数时，实际上是 self: Self 的简写，
/// 所以 &self 是 self: &Self, 而 &mut self 是 self: &mut Self。
impl Write for BufBuilder {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        // 把 buf 添加到 BufBuilder 的尾部
        self.buf.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())// 由于是在内存中操作，所以不需要 flush
    }
}







pub fn main(){

    let mut my_buf =BufBuilder::new();
    my_buf.write_all(b"hello");
    println!("my buffer:{:?}",my_buf);
}








