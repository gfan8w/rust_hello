
// 问题来源： https://stackoverflow.com/questions/42613974/why-cant-i-add-a-blanket-impl-on-a-trait-with-a-type-parameter


/*

           -> trait Bar<i32>    -> trait Foo (with i32 baked-in)
struct Baz
           -> trait Bar<String> -> trait Foo (with String baked-in)
*/


pub trait Foo<A> {
    fn new(arg: u32) -> Self;
    fn phantom(_: A) {}
}

pub trait Bar<P>: Foo<P> {
    fn with_parameter(arg: u32, parameter: P) -> Self;
}

impl<T: Bar<P>, P: Default> Foo<P> for T {
    fn new(arg: u32) -> Self {
        Self::with_parameter(arg, P::default())
    }
}

#[derive(Debug)]
struct Baz{
}

impl Bar<i32> for Baz {
    fn with_parameter(arg: u32, parameter: i32) -> Self {
        println!("arg:{}, parameter is i32({})", arg, parameter);
        Self{}
    }
}
impl Bar<String> for Baz {
    fn with_parameter(arg: u32, parameter: String) -> Self {
        println!("arg:{}, parameter is String({})", arg, parameter);
        Self{}
    }
}

fn main(){

    let b = Baz{};
    let c = <Baz as Foo<i32>>::new(3);
    println!("{:?}",c)

}