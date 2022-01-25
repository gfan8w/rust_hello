
//Sized trait 用于标记有具体大小的类型。在使用泛型参数时，
// Rust 编译器会自动为泛型参数加上 Sized 约束，比如下面的 Data 和处理 Data 的函数 process_data：
/*

struct Data<T: Sized> {
   inner: T,
}
fn process_data<T: Sized>(data: Data<T>)
{
   // .....
}

*/
struct Data<T> {
    inner: T,
}
fn process_data<T>(data: Data<T>)
{
    todo!();
}

// 大部分时候，我们都希望能自动添加这样的约束，因为这样定义出的泛型结构，在编译期，大小是固定的，可以作为参数传递给函数。
// 如果没有这个约束，T 是大小不固定的类型， process_data 函数会无法编译。
// 但是这个自动添加的约束有时候不太适用，在少数情况下，需要 T 是可变类型的，怎么办？
// Rust 提供了 ?Sized 来摆脱这个约束。如果开发者显式定义了T: ?Sized，那么 T 就可以是任意大小。
// Cow 类型（copy-on-write）就是这样一个例子， Cow 中泛型参数 B 的约束是 ?Sized：

// B 就可以是 [T] 或者 str 类型，大小都是不固定的。
// 要注意 Borrowed(&'a B) 大小是固定的，因为它内部是对 B 的一个引用，而引用的大小是固定的
pub enum Cow<'a, B: ?Sized + 'a> where B: ToOwned,
{
    // 借用的数据
    Borrowed(&'a B),
    // 拥有的数据
    Owned(<B as ToOwned>::Owned),
}

























