
/// 在no_std环境下 如何使用format!
/// 参考：https://stackoverflow.com/questions/50200268/how-can-i-use-the-format-macro-in-a-no-std-environment
/// https://stackoverflow.com/questions/39488327/how-to-format-output-to-a-byte-array-with-no-std-and-no-allocator
use core::fmt::{self, Write};
use core::str;

pub fn run() {
    // For LCD 160 / 8 = 20 chars
    let mut buf = [0u8; 20];
    let mut buf = ByteMutWriter::new(&mut buf[..]);

    buf.clear();
    write!(&mut buf, "Hello {}!", "Rust").unwrap();

    let aa =buf.as_str();

    println!("strig:{}",aa)
}

pub fn get(size: usize) {
    // For LCD 160 / 8 = 20 chars
    let mut buf = [0u8; 20];
    let mut buf = ByteMutWriter::new(&mut buf[..]);

    buf.clear();
    write!(&mut buf, "Hello {}!", "Rust").unwrap();

    let aa =buf.as_str();

    println!("strig:{}",aa)
}


pub struct ByteMutWriter<'a> {
    buf: &'a mut [u8],
    cursor: usize,
}

impl<'a> ByteMutWriter<'a> {
    pub fn new(buf: &'a mut [u8]) -> Self {
        ByteMutWriter { buf, cursor: 0 }
    }

    pub fn as_str(&self) -> &str {
        str::from_utf8(&self.buf[0..self.cursor]).unwrap()
    }

    #[inline]
    pub fn capacity(&self) -> usize {
        self.buf.len()
    }

    pub fn clear(&mut self) {
        self.cursor = 0;
    }

    pub fn len(&self) -> usize {
        self.cursor
    }

    pub fn empty(&self) -> bool {
        self.cursor == 0
    }

    pub fn full(&self) -> bool {
        self.capacity() == self.cursor
    }
}

impl fmt::Write for ByteMutWriter<'_> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let cap = self.capacity();
        for (i, &b) in self.buf[self.cursor..cap]
            .iter_mut()
            .zip(s.as_bytes().iter())
        {
            *i = b;
        }
        self.cursor = usize::min(cap, self.cursor + s.as_bytes().len());
        Ok(())
    }
}

