
// 问题来源： https://stackoverflow.com/questions/42613974/why-cant-i-add-a-blanket-impl-on-a-trait-with-a-type-parameter


/*
第一步 一个简单版本，正常的，能工作的。
Baz 实现了 Bar<i32> 和 Bar<String>，
Baz 实现了 Foo

均没有二义性

*/


pub trait Foo {
    fn new(arg: u32) -> Self;
}

pub trait Bar<P>: Foo {
    fn with_parameter(arg: u32, parameter: P) -> Self;
}


#[derive(Debug)]
struct Baz{
}
impl Foo for Baz {
    fn new(arg: u32) -> Self {
        Baz{}
    }
}

impl Bar<i32> for Baz {
    fn with_parameter(arg: u32, parameter: i32) -> Self {
        todo!()
    }
}
impl Bar<String> for Baz {
    fn with_parameter(arg: u32, parameter: String) -> Self {
        todo!()
    }
}

fn main(){

    let b = Baz{};
    let c = <Baz as Foo>::new(3);
    println!("{:?}",c)

}