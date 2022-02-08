// 实现一个能在栈上分配的最小的String，如果超过最小长度，就改为传统的堆上分配的String



use std::{ops::Deref,str};
use std::fmt::{Debug, Display, Formatter};

const MIN_String_MAX_LEN: usize=30;

// MyString 里，String 有 3 个 word，供 24 字节，所以它以 8 字节对齐
// 所以 enum 的 tag + padding 最少 8 字节，整个结构占 32 字节。
// MiniString 可以最多有 30 字节（再加上 1 字节长度和 1字节 tag），就是 32 字节.
struct MiniString{
    len: u8,
    data: [u8; MIN_String_MAX_LEN],
}


impl MiniString {
    // 这里 new 接口不暴露出去，保证传入的 v 的字节长度小于等于 30
    fn new(str: impl AsRef<str>) -> Self {
        let bytes = str.as_ref().as_bytes();
        // 我们在拷贝内容时一定要使用字符串的字节长度
        let len = bytes.len();
        let mut data = [0u8;MIN_String_MAX_LEN];
        data[..len].copy_from_slice(bytes);
        Self {
            len: len as u8,
            data
        }
    }
}

impl Deref for MiniString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        // 由于生成 MiniString 的接口是隐藏的，它只能来自字符串，所以下面这行是安全的
        //str::from_utf8(&self.data[..self.len as usize]).unwrap()

        // 也可以直接用 unsafe 版本
        unsafe { str::from_utf8_unchecked(&self.data[..self.len as usize]) }
    }
}

impl Debug for MiniString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // 这里由于实现了 Deref trait，可以直接得到一个 &str 输出
        write!(f,"{}",self.deref())
    }
}


//定义可以在栈上分配的 String
#[derive(Debug)]
enum StackString {
    Inline(MiniString),
    Standard(String)
}


impl Deref for StackString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match self {
            StackString::Standard(v) => v.deref(),
            StackString::Inline(m) =>m.deref(),
        }
    }
}

impl Display for StackString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}", self.deref())
    }
}

impl From<&str> for StackString {
    fn from(s: &str) -> Self {
        if s.len()> MIN_String_MAX_LEN {
            StackString::Standard(s.to_string())
        }else {
            StackString::Inline(MiniString::new(s))
        }

    }
}


fn run() {

    let len_String =std::mem::size_of::<String>();
    let len_StackString =std::mem::size_of::<StackString>();
    println!("String len:{}, StackString len:{}",len_String,len_StackString);

    let s:StackString ="hello".into();

    let l_s: StackString = "这是一个很长的字符串，它真的非常的长，DDL where?".into();

    // debug 输出
    println!("s1: {:?}, s2: {:?}", s, l_s);

    println!("{}:({} bytes, {} chars), {}:({} bytes, {} chars)",
                s,
             s.len(),
             s.chars().count(),
             l_s,
             l_s.len(),
             l_s.chars().count());





}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_run() {
        run()
    }
}





























