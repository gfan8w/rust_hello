use std::ops::Deref;
use std::rc::Rc;

// Deref coercions Deref 强制类型转换

// 自定义 Deref 的实现，给 * 一个特殊的含义。
struct DerefExample<T> {
    value: T,
}

impl<T> Deref for DerefExample<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
///演示 Deref
pub fn run() {
    println!("demo auto Deref");
    let hello = DerefExample { value: String::from("hello") };
    println!("{}", *hello)  // 这里 * 会自动调佣 deref(..)
}



// 另外一种是rust语言上的特性，deref coercions
// 规则是这样的： 如果你有 类型 U，然后实现了 Deref<Target=T>， 那么 &U 可以自动强制转换(coerce) 为 &T。
fn foo(s: &str){
    println!("{}", s)
}

fn show_coerce(){
    // String 实现了 Deref<Target=str>. &String 会自动转换为 &str
    let owned = "hello".to_string();
    // 这个能正常工作，在一个值前面加&，表示引用这个值，owned是String，&owned是&String，
    // 因为有 impl Deref<Target=str> for String，所以 &String 转换到 &str
    foo(&owned);

    //因为 Rc<T> 实现了 Deref<Target=T> 能从 &Rc<T> 直接通过 * 符号获得 &T。
    // 我们把 String 包装为Rc<T>
    let counted = Rc::new(owned);
    // 这里foo不需要更改，因为 &Rc<String> 会 deref &String，再由 &String deref到 &str
    // Rust 会执行很多次deref直到类型匹配
    foo(&counted);
}


fn goo(s: &[i32]){

}

fn show_coerce1() {
    let owned = vec![1,2,3];
    // Vec<T> 实现了 Deref<Target=[T]>
    // 我们能从 &Vec<T> 得到 &[T]
    goo(&owned);

}

// Deref 和 方法调用
struct DDL;

impl DDL {
    fn foo(&self){
        print!("DDL");
    }
}


fn call_ddl(){
    // 例如，f 是 &&DDL，而 foo的参数是&self，它 也是能正常工作的。
    // 即使 f 是 &&&&&&&&&&&&&&&&DDL， 也能工作。
    // 因为编译器会插入尽可能多的 * 来获得正确的类型，因为是 *，它其实就是 Deref。
    // 方法 会自动 Deref。
    let f = &&DDL;
    f.foo();
    (&f).foo();
    let g = &&&&&f;
    g.foo();

    // 这些都是一样的
    f.foo();
    (&f).foo();
    (&&f).foo();
    (&&&&&&&&f).foo();
}



// 降级coerce，从 &mut i64 降级为 &i64
// 更多参考： https://www.possiblerust.com/guide/what-can-coerce-and-where-in-rust
struct RefHolder<'a> {
    x: &'a i64,
}

impl<'a> RefHolder<'a> {
    fn new(x: &'a i64) -> RefHolder<'a> {
        RefHolder { x }
    }
}

fn print_num(y: &i64) {
    println!("y: {}", y);
}
fn reference_downgrade_coerce() {
    // Create `x`
    let mut x = 10;

    // Make sure `y` is `&mut i64`.
    let y = &mut x;

    // Package the downgraded reference into a struct.
    // y 降级 为 &i64.
    let z = RefHolder::new(y);

    // Print `y` downgrading it to an `&i64`.
    // y 降级 为 &i64.
    print_num(y);

    // Use the `z` reference again.
    println!("z.x: {}", z.x);

    //z.x=&43;
    //println!("z.x: {}", z.x);
}



// trait对象时，不会做降级 coerces。
trait Trait {}

fn koo<X: Trait>(t: X){}

impl<'a> Trait for &'a i32 {}

fn call_koo(){
    let t: i32 = 0;
    koo(&t);  // 类型能从 T coerce 到 &T

    let d:  &mut i32 = &mut 0;
    //koo(d); // 这里会报错， &mut i32 没有实现 Trait {}, 在做 trait 匹配的时候不会做 降级 coerces。
}





#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        run();
    }

    #[test]
    fn test_2() {
        show_coerce();
    }

    #[test]
    fn test_3() {
        show_coerce1();
    }

    #[test]
    fn test_4() {
        call_ddl();
    }

    #[test]
    fn test_5() {
        reference_downgrade_coerce();
    }

    #[test]
    fn test_6() {
        call_koo();
    }

}























