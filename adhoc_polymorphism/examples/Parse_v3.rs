

/*

改造为 泛型
返回默认值，不是很好，默认值是0，那无法区分出 0abc

再次改造为返回错误Err

*/

use std::str::FromStr;

trait Parse {
    type Err;
    fn parse(str: &str) -> Result<Self,Self::Err>
    where Self: Sized;
}

/// 我们约束 T 必须同时实现了 FromStr 和 Default
/// 这样在使用的时候我们就可以用这两个 trait 的方法了
impl<T> Parse for T
where T: FromStr + Default {
    type Err = String;

    fn parse(str: &str) -> Result<Self,Self::Err> {
        let regx = regex::Regex::new(r"^[0-9]+(\.[0-9]+)?").unwrap();
        let aa =match regx.captures(str) {
            Some(cap) =>{
                // 取第一个 match，将其捕获的 digits 换成 u8
                cap
                    .get(0)
                    .map_or_else(||Err("can't capture the input number".to_string()),
                            |s|s.as_str().parse::<T>().map_err(|e|"can't parse the input".to_string()))
            },
            None =>{
                Err("none captured".to_string())
            }
        };
        aa
        //Ok(Default::default())


    }
}

fn main() {

    let src ="123abc";
    let a_u8 =<u8 as Parse>::parse(src).unwrap();
    println!("u8:{}",a_u8);

    let src ="1234abc";

    let rst =u8::parse(src);

    //把错误打印出来，然后返回一个默认值
    println!("u8:{}",rst.unwrap_or_else(|e|{println!("parse error:{}",e); 0}));


    if let Ok(a_u8) =u8::parse(src){
        println!("u8:{}",a_u8);
    }else {
        println!("failed to pasre u8")
    }


    let src = "0.34abc";
    let a_f64: f64 =f64::parse(src).unwrap();
    println!("a_f64:{}",a_f64);


}












