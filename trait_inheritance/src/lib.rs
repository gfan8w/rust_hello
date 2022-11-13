
/*

Rust除了支持组合，还支持继承。但Rust只支持Trait之间的继承，比如Trait A继承Trait B。实现继承的方式很简单，在定义Trait A时使用冒号加上Trait B即可。
*/

trait B {
    fn func_in_b(&self);
}

// Trait A继承Trait B
trait A : B{
    fn func_in_a(&self);
}

struct C{}

impl B for C {
    fn func_in_b(&self) {
        println!("impl: func_in_b");
    }
}

impl A for C {
    fn func_in_a(&self) {
        println!("impl: func_in_a");
    }
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let c = C{};
        c.func_in_a();
        c.func_in_b();
    }
}
