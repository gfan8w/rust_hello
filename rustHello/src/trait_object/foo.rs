

// Trait 默认实现的正确定义方法是在定义 Trait 时指定, 而不应该在 impl Trait {} 语句块中.
trait Foo {
    fn default_impl(&self) {
        println!("correct impl!");
    }
}

impl Foo {
    fn trait_object(&self) { // 原来的例子中，这里没有 &self，我在这里加上了，没有&self，这个是叫关联方法(associate function, not method)，演示效果不好
        println!("trait object impl");
    }
}

struct Bar {}

impl Foo for Bar {}

// Bar 在实现了 Foo 后可以通过 b.default_impl 调用, 无需额外实现,
// 但 b.trait_object 则不行, 因为 trait_object 方法是 Foo 的 trait object 上的方法.
fn run() {
    let b = Bar{};
    b.default_impl();
    //b.trait_object();  // 这里会报错。下面的 c 是一个 Trait Object类型可以直接调用trait_object这个函数，
    Foo::trait_object(&b); // 这里显式的传入self：b

    let c: &dyn Foo = &b;
    c.trait_object();
}


/*

impl Trait 和 dyn Trait 在 Rust 分别被称为静态分发和动态分发.

如下有一个trait，刚开始这样：
fn some_fn(param: SomeType) -> Button

随着开发进度增加, 这个函数需要返回 Button, TextView 等组件中的一个, 我下意识地写出了类似于下面的代码

fn some_fn(param1: i32, param2: i32) -> impl View {
    if param1 > param2 {
        // do something...
        return Button {};
    } else {
        // do something...
        return TextView {};
    }
}

Rust 编译器报错：
如果在if 中已经返回 Button，那else也必须返回Button。
Rust 要求 if else 两个分支的返回值类型相同的特性一致。

这就是静态分发。impl View 的具体类型一旦确定，就必须一致。

假设 Foo 和 Bar 都实现了 Noop 特性, Rust 会把函数
  fn x(...) -> impl Noop
展开为
  fn x_for_foo(...) -> Foo
  fn x_for_bar(...) -> Bar

*/




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        run()
    }
}