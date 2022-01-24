

trait Formatter {
    fn format(&self, input: &mut String) -> bool;
}


struct MarkDownFormatter;
impl Formatter for MarkDownFormatter {
    fn format(&self, input: &mut String) -> bool {
        input.push_str("formatted with markdown ");
        true
    }
}

struct JsonFormatter;
impl Formatter for JsonFormatter {
    fn format(&self, input: &mut String) -> bool {
        input.push_str("formatted with json ");
        true
    }
}


struct HtmlFormatter;
impl Formatter for HtmlFormatter {
    fn format(&self, input: &mut String) -> bool {
        input.push_str("formatted with html ");
        true
    }
}

/// 有一种手段，告诉编译器，此处需要并且仅需要任何实现了 Formatter 接口的数据类型。
/// 在 Rust 里，这种类型叫 Trait Object，表现为 &dyn Trait 或者 Box。
/// 这里，dyn 关键字只是用来帮助我们更好地区分普通类型和 Trait 类型，
/// 阅读代码时，看到 dyn 就知道后面跟的是一个 trait 了。
fn format_doc(input: &mut String, formatter_list: Vec<&dyn Formatter>) {
    for formatter in formatter_list {
        formatter.format(input);
    }
}


pub fn run(){
    let mut text = "hello world ".to_string();

    // HtmlFormatter 的引用赋值给 Formatter 后，会生成一个 Trait Object，在 trait-object-HtmlFormatter.png 图中可以看到，
    // Trait Object 的底层逻辑就是胖指针。其中，一个指针指向数据本身，另一个则指向虚函数表（vtable）
    // vtable 是一张静态的表，Rust 在编译时会为使用了 trait object 的类型的 trait 实现生成一张表，
    // 放在可执行文件中（一般在 TEXT 或 RODATA 段）。 参见：trait-object内存布局.png
    // 在这张表里，包含具体类型的一些信息，如 size、aligment 以及一系列函数指针：这个接口支持的所有的方法，比如 format() ；
    // 具体类型的 drop trait，当 Trait object 被释放，它用来释放其使用的所有资源。这样，当在运行时执行 formatter.format() 时，
    // formatter 就可以从 vtable 里找到对应的函数指针，执行具体的操作。

    // C++ / Java 指向 vtable 的指针，在编译时放在类结构里，而 Rust 放在 Trait object 中。
    // 这也是为什么 Rust 很容易对原生类型做动态分派，而 C++/Java 不行。
    // 事实上，Rust 也并不区分原生类型和组合类型，对 Rust 来说，所有类型的地位都是一致的。
    let html: &dyn Formatter= &HtmlFormatter{};

    let markdown: &dyn Formatter = &MarkDownFormatter{};

    let format_list =vec![html, markdown];
    format_doc(&mut text, format_list);

    println!("trait formatter:{}",text);

}

/*
你使用 trait object 的时候，要注意对象安全（object safety）。只有满足对象安全的 trait 才能使用 trait object

如果 trait 所有的方法，返回值是 Self 或者携带泛型参数，那么这个 trait 就不能产生 trait object。
不允许返回 Self，是因为 trait object 在产生时，原来的类型会被抹去，所以 Self 究竟是谁不知道。
比如 Clone trait 只有一个方法 clone()，返回 Self，所以它就不能产生 trait object。不允许携带泛型参数，
是因为 Rust 里带泛型的类型在编译时会做单态化，而 trait object 是运行时的产物，两者不能兼容。
比如 Fromtrait，因为整个 trait 带了泛型，每个方法也自然包含泛型，就不能产生 trait object。
如果一个 trait 只有部分方法返回 Self 或者使用了泛型参数，那么这部分方法在 trait object 中不能调用。


*/
