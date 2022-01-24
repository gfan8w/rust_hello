
//模拟自己实现 Iterator 这个 trait

struct SentenceIter<'a> {
    s: &'a mut &'a str,
    delimiter: char
}

impl<'a> SentenceIter<'a> {
    fn new(input: &'a mut &'a str, delimiter: char) -> Self {
        Self {
            s:input,
            delimiter,
        }
    }
}



impl<'a> Iterator for SentenceIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        // 如果内部的字符串引用指向空，则提前结束
        if self.s.is_empty() {
            return None;
        }

        let d_len = self.delimiter.len_utf8();
        match self.s.find(self.delimiter) {
            Some(i) =>{
                // 注意对于 utf8 char，取它的长度需要用 c.len_utf8()
                let got =&self.s[..i];
                // 更改内部字符串引用，指向剩余部分
                *self.s =&self.s[(i+d_len)..];

                Some(got.trim())
            },
            None =>{
                // 没有找到句号时，有可能后续还有最后一句内容
                let got =&self.s[..];
                *self.s ="";

                Some(got.trim())

            }
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1(){
        let mut a_full ="hello .wor ld ";
        let mut iter = SentenceIter::new(&mut a_full, '.');
        assert_eq!(iter.next(),Some("hello"));
        assert_eq!(iter.next(),Some("wor ld"));
        assert_eq!(iter.next(),None);
    }

    #[test]
    fn test_No_delimiter(){
        let mut a_full ="hello";
        let mut iter = SentenceIter::new(&mut a_full, '.');
        assert_eq!(iter.next(),Some("hello"));
        assert_eq!(iter.next(),None);
    }

    #[test]
    fn test_no_ascii_delimiter() {
        let mut s = "a。 b。 c ";
        let sentences: Vec<_> = SentenceIter::new(&mut s, '。').collect();
        println!("sentences: {:?}", sentences);
    }



}






















































