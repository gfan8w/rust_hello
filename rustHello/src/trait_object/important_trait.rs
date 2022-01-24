


//重要的trait 必须学会的


#[derive(Clone,Debug)]
struct A{
    buf: String
}


#[cfg(test)]
mod test {
    use super::*;

    ///对于 buf，也就是 String 类型的 Clone，其堆上的内存也被 Clone 了一份，所以 Clone 是深度拷贝，栈内存和堆内存一起拷贝。
    #[test]
    fn test_Clone_from_A() {

        let mut a =A {buf:"hellohellohello".to_string()};
        println!("a.buf(address:{:p}):{}", a.buf.as_str(),a.buf);

        let b =A {buf:"good good ".to_string()};
        println!("b.buf(address:{:p}):{}", b.buf.as_str(),b.buf);
        // a.clone_from(&b) ，和 a = b.clone() 是等价的
        //clone_from 如果a 已经存在，a的内存足够的话，会继续使用a 的内从，直接调 b.clone()  是重新分配内存

        a.clone_from(&b);
        println!("a.buf(address:{:p}):{}", a.buf.as_str(),a.buf);

        println!("a:{:?}",a)



    }
}







