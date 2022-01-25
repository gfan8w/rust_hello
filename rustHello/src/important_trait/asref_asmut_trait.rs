use std::fs::File;
use std::path::Path;

#[allow(dead_code)]
enum Language {
    Rust,
    TypeScript,
    Java,
    Go,
    Haskell,
}

///在 trait 的定义上，都允许 T 使用大小可变的类型，如 str、[u8] 等。
/// AsMut 除了使用可变引用生成可变引用外，其它都和 AsRef 一样，
/// 所以我们重点看 AsRef
impl AsRef<str> for Language {
    fn as_ref(&self) -> &str {
        match self {
            Language::Rust => "Rust",
            Language::TypeScript => "TypeScript",
            Language::Java => "Java",
            Language::Go => "Go",
            Language::Haskell => "Haskell"
        }
    }
}

fn run_lan(){

    let rust = Language::Rust;
    let rust_ref = rust.as_ref();
    println!("rust_str:{}",rust_ref);

    //
    print_ref(rust); // 我们自己定义的 enum 也实现了 AsRef
    print_ref("hello"); // &str 实现了 AsRef
    print_ref("world".to_string()); // String 实现了 AsRef


    // 它的参数 path 是符合 AsRef 的类型，
    // 所以，你可以为这个参数传入 String、&str、PathBuf、Path 等类型。
    // 而且，当你使用 path.as_ref() 时，会得到一个 &Path。
    let f =File::open("./tmp_file.txt");

    //std::fs::rename 接受 &Path 参数， 写作如下：
    std::fs::rename(Path::new("a.txt"),Path::new("b.txt"));
    //因为 str 实现了 AsRef<Path> ，可以简写：
    std::fs::rename("b.txt","a.txt");
}


fn print_ref(var: impl AsRef<str>) {
    println!("ref val:{:?}",var.as_ref());
}




#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        run_lan();
    }
}




























