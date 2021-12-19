

/// 关联类型
/// 关联类型是 trait 定义中的类型占位符。定义的时候，并不定义它的具体的类型是什么。
/// 在 impl 这个 trait 的时候，
/// 才为这个关联类型赋予确定的类型。也就是说，在实现的时候，才知道它的具体类型是什么。
pub trait Convert {
    type Output;

    fn convert(&self) -> Self::Output;
}


struct MyInt;

impl Convert for MyInt {
    type Output = i32;

    fn convert(&self) -> Self::Output {
        42
    }
}



//主入口函数
pub fn main() {

    let my_int = MyInt;
    let output= my_int.convert();
    println!("{}",output)

}





