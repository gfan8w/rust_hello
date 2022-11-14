trait MyFn<In> {
    type Out;
}

trait Foo {}

impl<F, A> Foo for F where F: MyFn<A, Out=()> {}

struct Context{

}

pub trait Handler: Send + Sync + 'static {
    fn invoke(&self, context: Context) -> String;
}

impl<F: Send + Sync + 'static, Fut> Handler for F
    where
        F: Fn(Context) -> Fut,
        Fut: Send + 'static, // 对于 Response的约束还以：
        Fut::Output: String,
{
    fn invoke(&self, context: Context) -> String {
        (self)(context)
    }
}


fn main() {
    println!("hello")
}

/*

    trait MyTrait1 {}
    trait MyTrait2 {}
    trait MyTrait3<T> {}

    impl <T: MyTrait1, U: MyTrait3<T>> MyTrait2 for Vec<U> {}


 改为：

    trait MyTrait1 {}
    trait MyTrait2 {}
    trait MyTrait3 {
        type AssociatedType;
    }

    impl<T, U> MyTrait2 for Vec<U>
        where
            U: MyTrait3<AssociatedType = T>,
            T: MyTrait1,
    {
    }


*/