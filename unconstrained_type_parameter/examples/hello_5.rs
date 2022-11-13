
// https://stackoverflow.com/questions/69238420/the-type-parameter-t-is-not-constrained-by-the-impl-trait-self-type-or-predi


/*
这里是一个反例，
前面都是泛型 缺乏约束

这里是一个关联类型，泛型T被一个关联类型约束
*/

mod wrong {
    /*
    任何时候，我们在写下impl<T>的时候，都表示我们在定义一个泛型实现--它代表着一簇可能得实现。
    每一个可以填入T的类型，都代表着一种不同的实现。
    但是， 对于trait和该trait的实现，必须是唯一地可以一一对应的、可证明是同一的。
    不能有多个SendsMessages对应一个ImplementsSendsMessages。而下面的代码正尝试的去这样做。
    */
    pub trait Message {}

    pub trait SendsMessages {
        type Input: Message;
        fn send(msg: Self::Input);
    }

    pub struct ImplementsSendsMessages {
    }

    // 无法确定自己代码类库的某个类(实现了Message这个trait的T类型), 还是其他类库的类型(实现了Message这个trait的T类型)， 编译器是不允许
    impl<T> SendsMessages for ImplementsSendsMessages
        where
            T: Message,
    {
        type Input = T;

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