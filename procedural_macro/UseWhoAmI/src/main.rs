use whoami::WhoAmI;

/*
编译这个会报错：

= help: message: My struct name is: <Point>

这段话就是我们制造出来的错误。

这个案例 是为了掩饰如何debug 输出信息

*/

fn main() {
    println!("Hello, world!");
}

#[derive(WhoAmI)]
struct Point {
    x: f64,
    y: f64
}










