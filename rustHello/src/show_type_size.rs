use std::collections::HashMap;
use std::mem::size_of;

enum E {
    A(f64),
    B(HashMap<String, String>),
    C(Result<Vec<u8>, String>),
}


// 这是一个声明宏，它会打印各种数据结构本身的大小，在 Option 中的大小，以及在 Result 中的大小
macro_rules! show_size {

    (header) =>{
        println!( "{:<24}    {:>4}    {}   {}", "Type", "T", "Option<T>", "Result<T,std::io::Error>" );
        println!("{}", "-".repeat(74));
    };
    ($tt:ty)=>{
        println!(
            "{:<24}    {:4}    {:8}   {:12}",
            stringify!($tt),
            size_of::<$tt>(),
            size_of::<Option<$tt>>(),
            size_of::<Result<$tt,std::io::Error>>()
        )
    }
}



///查看各种类型的内存大小
pub fn run(){

    show_size!(header);
    show_size!(u8);
    show_size!(f64);
    show_size!(&u8);
    show_size!(Box<u8>);
    show_size!(&[u8]);

    show_size!(String);
    show_size!(Vec<u8>);
    show_size!(HashMap<String, String>);
    show_size!(E);


}













