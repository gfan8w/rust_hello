

/*

假设我们要做一个字符串解析器，可以把字符串的某部分解析成某个类型，
那么可以这么定义这个 trait：它有一个方法是 parse，这个方法接受一个字符串引用，返回 Self。

这个 parse 方法是 trait 的静态方法，因为它的第一个参数和 self 无关，所以在调用时需要使用 T::parse(str) 。

*/

trait Parse {
    fn parse(str: &str) -> Self;
}

///尝试为 u8 这个数据结构来实现 parse，比如说：“123abc” 会被解析出整数 123，而 “abcd” 会被解析出 0。
impl Parse for u8 {
    fn parse(str: &str) -> Self {
        let regx = regex::Regex::new("^[0-9]+").unwrap();
        match regx.captures(str) {
            Some(cap) =>{
                // 取第一个 match，将其捕获的 digits 换成 u8
                cap.get(0).map_or(0,|s|s.as_str().parse::<u8>().unwrap())
            },
            None =>{
                0
            }
        }
    }
}

fn main() {

    let src ="123abc";
    let a_u8 =<u8 as Parse>::parse(src);
    println!("u8:{}",a_u8);

    let src ="1234abc";
    let a_u8 =u8::parse(src);
    println!("u8:{}",a_u8);


}












