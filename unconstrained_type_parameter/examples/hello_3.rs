
// 更简单的解释： https://stackoverflow.com/questions/72323037/nested-generic-impl


/*

第二.一步
b.call_foo() 内的调用有二义性，
Baz 实现了 Trait<i32> 和 Trait<i64>，
但我们无法知道 调用 foo()的时候，具体调用的是 Trait<i32>.foo 还是 Trait<i64>.foo

这才是编译器阻止我们的原因

*/

mod wrong {
    pub trait Trait<T> {
        fn foo(&self);
    }

    #[derive(Debug)]
    struct Baz;

    impl Trait<i32> for Baz {
        fn foo(&self) {
            println!("i32 impl")
        }
    }

    impl Trait<i64> for Baz {
        fn foo(&self) {
            println!("i64 impl")
        }
    }

    impl<T, U> U
        where
            U: Trait<T>,
    {
        fn call_foo(&self) {
            self.foo();
        }
    }

    pub fn run(){
        let b = Baz;
        b.call_foo(); // b.call_foo() 内的调用有二义性，
        // Baz 实现了 Trait<i32> 和 Trait<i64>，
        // 但我们无法知道 调用 foo()的时候，具体调用的是 Trait<i32>.foo 还是 Trait<i64>.foo
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