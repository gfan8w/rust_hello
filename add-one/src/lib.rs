///演示 workspace


use rand;

pub fn add_one(x: i32) -> i32 {
    x + rand::random::<i32>()
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        let result = add_one(6);
        assert_eq!(result, 7);
    }

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
    }

    #[test]
    fn result_ok_err_type_omit_shold_malposition_Err_works() {
        let result: Result<u8,_> =Err(4);
        println!("{:?}",result);
        assert_eq!(result.unwrap_or(4),4);
    }
}


