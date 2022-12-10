use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// 本例演示一下
lazy_static::lazy_static! {
    pub static ref OPERATIONS_MAP: HashMap<&'static str, i32> = {
        let mut h =HashMap::new();
        h.insert("Bob", 0);
        h.insert("Alice", 1);
        h.insert("Charilie", 2);
        h
    };
}

#[derive(Debug)]
struct LogOperation(i32);

// &String 和 &str 不是一样吗 ?
//
// HashMap 的定义：
// ** impl<K, V> HashMap<K, V, RandomState>
// get 方法的签名：
// pub fn get<Q: ?Sized>(&self, k: &Q) -> Option<&V>
// where
//     K: Borrow<Q>,
//     Q: Hash + Eq,
// 根据此列，我们知道 K=&str, V=i32, Q= String
// 我们能得到 &str : Borrow<String> 吗？ 编译器告诉我们不能。
// 搜索 Borrrow<T> 的实现，主要的有：
/*  impl Borrow<str> for String // String: Borrow<str>
    impl<T> Borrow<T> for &T // &T: Borrow<T>
    impl<T> Borrow<T> for &mut T // &mut T: Borrow<T>
    impl<T> Borrow<T> for T // T: Borrow<T>
*/
// 偏偏没有 impl Borrow<String> for &str
// 如果固定 K=&str，仅有：
//   &str: Borrow<str> // 其实只有这一条
//   &mut str: Borrow<str>
//   str: Borrow<str>
// 如果传入的是 &*value，get入参是 &str, 则 Q是 str， 有 &str: Borrow<str> ，则代码能通过！！ *value 为什么是 str 类型？？？？？？？
// 还有一种方法是：
// 继续思考的话，可能会问，&String 和 &str 不是一样吗？ coercion 就让代码通过了啊。
// 只要你在某个地方明确指明类型，当然可以利用 coercion 进行转换： OPERATIONS_MAP.get::<str>(&value)
// 但没指明任何类型的泛型情况下，Rust 不对 &String 做强制转换
//
// 更多解释:
// 感觉是因为HashMap的get参数类型是generic的，所以coercion没能正常工作。
// 展开解释一下：
// 正常情况下如果形参类型是&str而实参类型是&String，因为String实现了Deref<Target=str>，所以可以coerce过去。
// 但是对于get这个函数，它的参数类型是Q且要求满足K: Borrow，也就是要求HashMap的key:
// K能产生一个Q的借用（背后的设计思路是HashMap是持有所有key的ownership的，所以总可以对K随便借用）。
// 看起来rustc在处理这种形参类型不确定的情况时，不会做coercion。
fn parse(value: String) -> Result<LogOperation, String> {
   // if let Some(opt_number) = OPERATIONS_MAP.get(&value) {
   // if let Some(opt_number) = OPERATIONS_MAP.get(&*value) {  // 这总写法是正确的。传入类型是 &str
      if let Some(opt_number) = OPERATIONS_MAP.get::<str>(&value) {  // 这种写法是正确的。通过对get泛型函数指定类型发生 coercion
        return Ok(LogOperation(*opt_number));
    }
    return Err(format!("{} is not a valid number", value));
}


fn parse1(value: String) -> () {
    // let cc = *value;     // 这个不会编译通过，但通过ide推断，能知道 cc 是 str 类型。
    let aa = &value;  // aa 是 &String 类型。
    let bb: &str = &value;// bb是 &str， 这里编译能通过，是因为这里发生了 coercion， 只要你在某个地方明确指明类型，当然可以利用 coercion 进行转换
                          // 但没指明任何类型的泛型情况下，Rust 不对 &String 做强制转换
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test(){
        let lo = parse("Alice".to_string());
        println!("{:?}", lo);
    }

}








