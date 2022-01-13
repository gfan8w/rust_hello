pub fn main() {
    println!("Hello, world!");
}



#[cfg(test)]
mod tests {


    ///这里主要演示，如果你只想做一个变量，只有OK， 那需要把 Result的<E> 给定义出来，因为编译器 无法自动推定。
    /// 所以这里看起来很奇怪，你要定义一个4，却要指定 u8，那个u8其实是无用的
    /// 看起来 跟你想做的事，刚好相反.
    #[test]
    fn result_ok_err_type_omit_shold_malposition_Ok_works() {
        let result: Result<_,u8> =Ok(4);
        println!("{:?}",result);
        assert_eq!(result.unwrap(),4);

        ///真正优雅的做法是：不是定义u8，而是 void，即 ()
        let void_2: Result<_,()> =Ok(4);
        println!("{:?}",void_2);
        assert_eq!(void_2.unwrap(),4);

        ///更优雅的做法是如下，用turbofish语法
        /// Err 没有，用 () 定义。
        /// Ok类型是通过 后面的4推断出来
        let void_3 =Ok::<_,()>(4);
        println!("{:?}",void_3);
        assert_eq!(void_3.unwrap(),4);

        ///另外一个例子
        let void_4 =Ok::<u32,bool>(4);
        println!("{:?}",void_4);
        assert_eq!(void_4.unwrap(),4);
    }

    #[test]
    fn result_ok_err_type_omit_shold_malposition_Err_works() {
        let result: Result<u8,_> =Err(4); // Err 的类型通过后面的的 4来推断，推断出是 i32
        println!("{:?}",result);
        assert_eq!(result.unwrap_or(4),4);
    }


    #[test]
    fn result_ok_run_turbofish_works() {

        let numbers: Vec<u32> =vec![1,2,3,4,5,6,7,8,9,10];

        let even_numbers = numbers.into_iter()
            .filter(|n| n%2==0)
            //.collect(); //如果直接这样，编译器报错：consider giving `even_numbers` a type，无法自动推断
            .collect::<Vec<u32>>();  // 可以通过如下 turbofish的方式指定，这里的u32 可以替换为 _ ，让编译器来推断出内部元素是 u32
                                     // 注意collect的签名：fn collect<B>(self) -> B ，所以turbofish放在 collect上
        println!("{:?}",even_numbers);

        // 这个例子，主要演示 turbofish类型放在那里，是放在 into上？还是放在哪里？ 这个要看签名
        let s ="hello world";
        //let str = s.into();    //这里编译器提示要给 str 一个类型。
        //let str = s.into::<String>();  // 这里编译器提示： expected 0 generic arguments，
        //let str = Into::<String>::into(); // 这里 也是报错 expected 1 argument， 实际上是没有1个参数，因为把实例方法改为一般方法
        let str: String = Into::into(s);    // 这样可以，显示给 str一个类型 String
        let str = Into::<String>::into(s); //这个是可以的，把 s.into() 这样的实例方法 改为普通方法,double colon rather that dot
        let str = <&str as Into<String>>::into(&s); //全限定访问。 这里入参是 &s, 如果入参是 s，会发生一次自动的deref？ 会吗？ 我也不知道



        eprintln!("{}",str);


    }





}