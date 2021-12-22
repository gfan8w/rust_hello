use std::ops::Neg;
use std::fmt::Display;

/// Traits是什么？
/// Traits are something multiple types can have in common
/// 可以实现：
/// 1、one of your traits on anyone's type
/// 2、anyone's trait on one of your types
/// 3、but not a foreign trait on a foreign type

trait Signed {
    fn is_strickly_negative(self)->bool;
}

trait Summary {
    fn summarize(&self) -> String;
}

struct NewArticle {
    headline: String,
    location: String,
    author: String,
    content: String
}

struct Tweet {
    username: String,
    content: String,
    replay: bool,
    retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        todo!()
    }
}

impl Summary for NewArticle {
    fn summarize(&self) -> String {
        todo!()
    }
}


struct Number {
    odd:bool,
    value:i32
}

///把trait作为函数的参数
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
///多个trait作为参数传递
fn notify2(item: &(impl Summary + Display + Signed+Copy)) {
    println!("Breaking news! {}, {}", item.summarize(), item.is_strickly_negative());
}
/// 多个trait作为约束
fn notify3<T: Summary+Signed+Copy>(item: &T) {
    println!("Breaking news! {}, {}", item.summarize(), item.is_strickly_negative());
}

fn return_summarize(switch: bool) -> impl Summary {
    // 这里返回2种类型，是不允许的。只能返回一种类型
    if switch {
        Tweet {
            username: "Musk".to_string(),
            content: "Filcon is landing".to_string(),
            replay: false,
            retweet: false
        }
    }else {
        Tweet {
            username: "Ctrip".to_string(),
            content: "Ctrip is rename to Trip".to_string(),
            replay: false,
            retweet: false
        }
        // 这里返回2种类型，是不允许的。只能返回一种类型
        /*NewArticle {
            headline: "aa".to_string(),
            location: "bb".to_string(),
            author: "cc".to_string(),
            content: "dd".to_string()
        }*/
    }
}



impl Signed for Number {
    fn is_strickly_negative(self) -> bool {
        self.value<0
    }
}

/// Our trait on a foreign type (a primitive type, even)
/// 在一个内置的原生类型上施加一个trait
impl Signed for i32 {
    fn is_strickly_negative(self) -> bool {
        self<0
    }
}


impl std::ops::Neg for Number{
    /*type Output=Number;
    fn neg(self) -> Self::Output {
        Number{odd:self.odd,value:-self.value,}
    }*/

    ///还可以这么写：
    /// An impl block is always for a type, so, inside that block, Self means that type
    type Output=Self;
    fn neg(self)-> Self {
        Self{
            odd:self.odd,
            value:-self.value
        }
    }
}


pub fn runTraitSample(){

    println!("Trait sample");

    //自定义类型实现自定义的trait
    let n=Number{odd:true,value:3};
    println!("Number odd:true,value:3:{}",n.is_strickly_negative());

    //原生类型实现自定义的trait
    let m:i32=-2;
    println!("{} is {}",m,m.is_strickly_negative());

    //自定义类型实现内置的trait
    let nu=Number{odd:false,value:44};
    println!("value:{}",nu.neg().value);

    ///有一些特性叫：markers - 他们没有实现任何方法，只是表示他们能完成某些特定的功能
    /// 例如： i32 实现了 trait Copy (in short, i32 is Copy)，所以如下赋值可以， a 没有 move，没有borrow
    let a: i32 = 15;
    let b = a; // `a` is copied
    let c = a; // `a` is copied again
    println!("b is {}, c is {}",b,c);

    print_i32(a); // `a` is copied
    print_i32(a); // `a` is copied again

    find_largest();

}

fn print_i32(x: i32) {
    println!("x = {}", x);
}


fn largest_char(list: &[char]) -> char {
    let mut lag =list[0];
    for item in list {
        if *item > lag {
            lag = *item;
        }
    }
    lag
}

fn largest_int(list: &[i32]) -> i32 {
    let mut lag = list[0];
    for item in list {
        if *item > lag {
            lag =*item;
        }
    }
    lag
}

fn largest<T>(list: &[T]) -> T
    where T: PartialOrd+Copy
{
    let mut largetst = list[0];
    for i in list {
        if *i >largetst {
            largetst=*i;
        }
    }
    largetst

}

fn find_largest() {
    let int_array =vec![1,2,3,4,5,6];
    let int_large =largest(&int_array);
    println!("largest int {}",int_large);

    let char_array =vec!['a','b','c','h','y'];
    let char_large =largest(&char_array);
    println!("largest char:{}",char_large);

    let str_arr = vec!["ab","bc","cd","de"];
    let str_large=largest(&str_arr);
    println!("largest string:{}",str_large);

}





