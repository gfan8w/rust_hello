
// https://stackoverflow.com/questions/69238420/the-type-parameter-t-is-not-constrained-by-the-impl-trait-self-type-or-predi


/*
前面的例子，trait有管理按类型，但实现了该trait时，指派了一个泛型为该trait的关联类型。这是不行的
如果要这么做，需要将trait改为泛型类型
*/

mod ok {
    /*
    这段代码是能通过编译的。对于不同的T，SendsMessages<T>也是不同的。 SendsMessages<T> 与 SendsMessages<B> 是不同的 trait。
    他们不会混淆.
    (这里也可以使用ImplementsSendsMessages<T>，它也不会造成二义性)

    另一种解读视角是：关联类型(associated types) 与 类型参数(type parameters)的区别是：
    关联类型更像是一种返回值(outputs), 而 类型参数 更像是一种输入值(inputs),
    一旦你选定一个trait，一个它的实现，把所有参数都填入后，关联类型是永远都知道的了，且是唯一确认的。
    而泛型类型是 根据填入的类型参数的不同，会编译出多个实现代码。
    之前的代码错误，是因为它正是要往关联类型里填入一个任意值(anything),而编译器要求这里必须是一个具体值。

    Another way to look at this is that the purpose of associated types as opposed to
    type parameters is to be “outputs”, rather than “inputs”: once you pick a trait, an implementing type,
    and fill in all the parameters, the associated type is always known and unique.
    Your code fails because it's trying to fill in the associated type with “anything”, when it has to be a single concrete type.
    */
    use std::marker::PhantomData;

    pub trait Message {}

    pub trait SendsMessages<T> {
        fn send(msg: T);
    }

    pub struct ImplementsSendsMessages {
    }

    // pub struct ImplementsSendsMessages<T> {
    //      _a: PhantomData<T>,
    // }

    impl<T> SendsMessages<T> for ImplementsSendsMessages
        where
            T: Message,
    {
        fn send(id: T) {
            todo!()
        }
    }



    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test(){

        }

    }

}

fn main(){

}