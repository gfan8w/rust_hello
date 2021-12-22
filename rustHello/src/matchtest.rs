
enum MatchMessage {
    Hello {id : i32}
}

///Match 的 @ at 符号绑定变量
pub fn run() {
    println!("test match patten");

    let msg = MatchMessage::Hello { id: 90 };

    match msg {
        MatchMessage::Hello { id: id_variable @ 1..=10 } => { println!("match:{}", id_variable) }, // 这里能访问匹配到的值, @ 可以保存匹配到的值到一个变量中
        MatchMessage::Hello { id: 10..=50 } => { println!("Found an id in another range") }, // 这里无法拿到匹配到的值，因为id 可能是 10~50里的任意一个
        MatchMessage::Hello { id } => { println!("Found some other id: {}", id) }       // 因为这里没有指定范围，可以拿到id的值
        MatchMessage::Hello { id } => { println!("Found some other id: {}", id) }
    }
}

// 参考：https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html#-bindings






