#![feature(unboxed_closures,core,fn_traits)]

// 更简单的解释： https://stackoverflow.com/questions/72323037/nested-generic-impl


/*
另外一个例子
*/

mod wrong {

    use std::mem;

    trait Foo {
        fn foo(&self) -> usize;
    }
    impl<F, A> Foo for F where F: FnOnce(A) {
        fn foo(&self) -> usize { mem::size_of::<A>() }
    }

    struct S;
    impl FnOnce<(u32,)> for S {
        type Output = ();
        extern "rust-call" fn call_once(self, _args: (u32,)) {} // 外部调用，头部要引入 feature
    }
    impl FnOnce<(u8,)> for S {
        type Output = ();
        extern "rust-call" fn call_once(self, _args: (u8,)) {}
    }
    fn run() {
        // S 是实现了 2种FnOnce， 一种是 u32， 另外一种是 u8，那将S转换为Foo后，foo()是哪个呢？
        println!("{}", <S as Foo>::foo(&S)); // which impl is used?
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