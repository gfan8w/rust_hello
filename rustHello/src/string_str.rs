

pub fn run(){
    // 任何数据结构 T，都可以有指向它的引用 &T，
    // 所以 String 跟 &String 的区别，以及 String 跟 &str 的区别，压根是两个问题。
    // 所以 &str 是 String 的切片，也可以是 &str 的切片。它和 &[T] 一样
    // Vec<T> 的切片是 &[T]
    // &str 就是一个带着长度的胖指针，指向了一片连续的内存区域
    let s = "hello world";
    let slice1 = &s[..5]; //可以对字符串切片
    let slice2 = &slice1[1..3]; // 可以对切片再切片
    println!("String str slice: {},{}",slice1, slice2); // 打印 hello, el
    println!("");

    let a = String::from("hello你好");
    //let b = a[0];           // String 无法通过下标方式访问，因为不知道它是一个byte，还是一个codepoint
    let cha =a.chars();
    cha.for_each(|x|print!("{}",x));
    //println!("b:{}",b);
    println!("\nchar end");

    let 你='你';
    let len=你.to_string().len();
    let utf8_len = 你.len_utf8();
    println!("char:{},len:{},utf8_len:{} ",你,len,utf8_len);
}



