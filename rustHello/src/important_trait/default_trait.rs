use std::fmt;
use std::fmt::Formatter;
use std::path::Display;

// struct 可以 derive Default，但我们需要所有字段都实现了 Default，必须给Language实现Default才不会报错
#[derive(Default,Debug)]
struct Developer {
    name: String,
    age: u8,
    lang: Language,
}

#[derive(Debug)]
enum Language {
    Rust,
    TypeScript,
    Elixir,
    Haskell,
}

/// 手工实现 Default
impl Default for Language {
    fn default() -> Self {
        Language::Haskell
    }
}

impl Developer {
    pub fn new(name: &str) -> Self {
        // 用 ..Default::default() 为剩余字段使用缺省值
        Self {
            name: name.into(),
            ..Default::default()
        }
    }
}

impl fmt::Display for Developer {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,"{} is a {} years old {:?} developer.",self.name,self.age,self.lang)
    }
}


pub fn run() {

    let d =Developer{
        name: "".to_string(),
        age: 0,
        lang: Language::Rust
    };
    println!("a dev:{:?}", d);

    // 使用 Default::default()，但此时类型无法通过上下文推断，需要提供类型
    let d:Developer =Default::default();    //这里必须指定类型。无法自动推导
    println!("a dev:{:?}",d);

    //使用 T::default()
    let d =Developer::default();
    println!("a dev:{}",d); //因为实现了Display，所以可以用 {} 打印

    // 使用 T::new
    let bob = Developer::new("Bob");
    println!("see a developer: {}",bob);

}









#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        run();
    }
}








