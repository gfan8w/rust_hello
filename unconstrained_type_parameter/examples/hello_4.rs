
// 更简单的解释： https://stackoverflow.com/questions/72323037/nested-generic-impl


/*

第二.二步
b.call_foo() 内的调用有二义性，
Baz 实现了 Trait<i32> 和 Trait<i64>，
但我们无法知道 调用 foo()的时候，具体调用的是 Trait<i32>.foo 还是 Trait<i64>.foo

这才是编译器阻止我们的原因

*/

mod ok {
    use std::fmt::Formatter;
    use std::marker::PhantomData;

    trait Trait {
        fn foo(&self);
    }

    trait AnyTrait {

    }

    trait MyTrait<K: std::fmt::Display> {

    }

    impl MyTrait<i32> for  (){
    }

    /*
    这里使用了一个自定义的struct，里面放一个PhantomData来确定类型
    */
    struct Ugly<K: std::fmt::Display, S: MyTrait<K>>(S, PhantomData<K>);
    impl<K: std::fmt::Display, M: MyTrait<K>> std::fmt::Display for Ugly<K, M> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "ok")
        }
    }



    pub fn run(){
        let b = Ugly{ 0: (), 1: Default::default() };
        println!("{}",b);
    }

    fn main(){
        run();
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test(){
            run();
        }

    }

}

fn main(){

}