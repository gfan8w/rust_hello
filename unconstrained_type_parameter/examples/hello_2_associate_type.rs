
// 问题来源： https://stackoverflow.com/questions/42613974/why-cant-i-add-a-blanket-impl-on-a-trait-with-a-type-parameter


mod a {
    pub trait Foo {
        fn new(arg: u32) -> Self;
    }

    // 使用associate type来解决 第一步的问题
    pub trait Bar: Foo {
        type Parameter;
        fn with_parameter(arg: u32, parameter: Self::Parameter) -> Self;
    }

    impl<T> Foo for T
        where T: Bar,
              T::Parameter: Default,
    {
        fn new(arg: u32) -> Self {
            Self::with_parameter(arg, T::Parameter::default())
        }
    }

    #[derive(Debug)]
    struct Bazz;


    impl Bar for Bazz {
        type Parameter = i32;
        fn with_parameter(arg: u32, parameter: Self::Parameter) -> Self {
            println!("arg:{}, parameter:{}", arg, parameter);
            Bazz {}
        }
    }


    /*

           -> trait Bar<i32>    -> trait Foo (with i32 baked-in)
struct Baz
           -> trait Bar<String> -> trait Foo (with String baked-in)

*/

    pub fn run() {
        let b = Bazz {};
        let c = <Bazz as Foo>::new(3);
        println!("{:?}", c)
    }

}



pub fn main() {
    a::run()
}