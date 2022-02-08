use std::borrow::Cow;
use serde::Deserialize;

/// 另外一个 cow的例子，serde 反序列化
#[derive(Debug, Deserialize)]
struct Student<'a> {
    #[serde(borrow)]
    name: Cow<'a, str>,
    age: u32
}

///Rust 下著名的 serde 库，可以非常高效地对 Rust 数据结构，进行序列化 / 反序列化操作，
/// 它对 Cow 就有很好的支持。我们可以通过如下代码将一个 JSON 数据反序列化成 User 类型，
/// 同时让 User 中的 name 使用 Cow 来引用 JSON 文本中的内容
fn run_serde_cow(){
    let s_str = r#"{"name":"ddl", "age":31}"#;
    let s_obj:Student = serde_json::from_str(s_str).unwrap();

    match s_obj.name {
        Cow::Borrowed(n) => println!("borrowed name:{}",n),
        Cow::Owned(n) => println!("owned name:{}",n)
    }
}











#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_run_serde_cow() {
        run_serde_cow()
    }

}




























