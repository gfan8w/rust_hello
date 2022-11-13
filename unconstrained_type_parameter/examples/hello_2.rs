
// 问题来源： https://stackoverflow.com/questions/42613974/why-cant-i-add-a-blanket-impl-on-a-trait-with-a-type-parameter


/*

第二步
Baz 实现了 Bar<i32> 和 Bar<String>，
有一个宽泛的类型T， Foo 在 那里实现了。 所以 impl Foo for Baz 不需要了
T有个约束，它必须是 Bar<P>，这里，它 是 Bar<i32> 和 Bar<String>

有二义性

the type parameter `P` is not constrained by the impl trait, self type, or predicates
  --> unconstrained_type_parameter/examples/hello.rs:21:17
   |
21 | impl<T: Bar<P>, P: Default> Foo for T {
   |                 ^ unconstrained type parameter

编译报错：P 没有被实现的trait 限制， P没有被Food限制。

Baz 在实现了 Bar<i32> 和 Bar<String> 后，Foo有2种类型，一个是i32， 一个是 String，有歧义。

           -> trait Bar<i32>    -> trait Foo (with i32 baked-in)
struct Baz
           -> trait Bar<String> -> trait Foo (with String baked-in)

那就是impl的这个Foo，没有被P约束，所以trait Foo需要一个phantom：


*/


pub trait Foo {
    fn new(arg: u32) -> Self;
}

pub trait Bar<P>: Foo {
    fn with_parameter(arg: u32, parameter: P) -> Self;
}

// 第二步，增加
impl<T: Bar<P>, P: Default> Foo for T {
    fn new(arg: u32) -> Self {
        Self::with_parameter(arg, P::default())  // 这里是调用 Bar<i32> 还是 Bar<String>，无法确定。
    }
}

#[derive(Debug)]
struct Baz{
}

// 第二步，增加这个约束后，这个就是多余的了？ T 可以代表 Baz ？
// impl Foo for Baz {
//     fn new(arg: u32) -> Self {
//         Baz{}
//     }
// }

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