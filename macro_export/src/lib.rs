mod bar_macro;

///演示 workspace，add-one是在workspace里定义的，这里还引入了 rand 随机库
/// 在 rusthello里引入了 add-one这个 crate
/// 这里还演示了 如在定义宏，在外部的crate引用这个宏，宏定义在 bar_macro 里

use rand;


pub fn add_one(x: i32) -> i32 {
    x + rand::random::<i32>()
}

pub fn call_baz(x: i32)  {
    //let a = crate::barz!(2); // 如果以这种方式访问，必须在barz上加 #[macro_export]
    let a =bar_macro::foo::barz!(1); //如果想以这种方式访问，必须在宏所在的模块加上 pub(crate) use barz;
    println!("a:{}",a)
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        let result = add_one(6); //因为 add_one 使用了外部的crate的rand，有个随机数，导致这里ut 不过。
        assert_eq!(result, 7);
    }

    #[test]
    fn barz_works() {
        let result = call_baz(6);
    }
}


