

/*

改造为 泛型

在实现 trait 的时候，也可以用泛型参数来实现 trait，需要注意的是，要对泛型参数做一定的限制。

第一，不是任何类型都可以通过字符串解析出来，在例子中，我们只能处理数字类型，并且这个类型还要能够被 str::parse 处理。
看文档，str::parse 是一个泛型函数，它返回任何实现了 FromStr trait 的类型，
所以这里对泛型参数的第一个限制是，它必须实现了 FromStr trait。

第二，上面代码当无法正确解析字符串的时候，会直接返回 0，表示无法处理，但我们使用泛型参数后，无法返回 0，
因为 0 不一定是某个符合泛型参数的类型中的一个值。怎么办？

*/

use std::str::FromStr;

trait Parse {
    fn parse(str: &str) -> Self;
}

/// 我们约束 T 必须同时实现了 FromStr 和 Default
/// 这样在使用的时候我们就可以用这两个 trait 的方法了
impl<T> Parse for T
where T: FromStr + Default {
    fn parse(str: &str) -> Self {
        let regx = regex::Regex::new(r"^[0-9]+(\.[0-9]+)?").unwrap();
        match regx.captures(str) {
            Some(cap) =>{
                // 取第一个 match，将其捕获的 digits 换成 u8
                cap
                    .get(0)
                    .map_or(Default::default(),
                            |s|s.as_str().parse::<T>().unwrap_or(Default::default()))
            },
            None =>{
                Default::default()
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

    let src = "0.34abc";
    let a_f64: f64 =f64::parse(src);
    println!("a_f64:{}",a_f64);


}












