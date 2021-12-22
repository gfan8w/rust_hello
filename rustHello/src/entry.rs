
use super::front_of_house::hosting as host; // as 重新命名
use crate::front_of_house::traffic_light::Run;
use std::cmp::{min, max}; //同时引入 min，max

use std::collections::HashMap;
use crate::front_of_house::pubStruct::CloseBox;

//使用外部的crate
use add_one;

//引入父级的内容，否则的话， Ferris::run(); 这些调用都需要加上 super::Ferris::run(); 或者加上 crate::
use super::*;   // *, 英文叫：the glob operator

//别名
#[derive(Debug)]
struct Color(u8,u8,u8);

#[derive(Debug)]
struct Rectangle{
    width:u32,
    height:u32
}

impl Rectangle {
    //
    // 计算面积
    //
    fn area(&self) ->u32 { //带 &self的叫method
        &self.width * &self.height
    }

    fn wider(&self, rect: &Rectangle) ->bool {
        &self.width> &rect.width
    }

    fn create(width: u32, height: u32) -> Rectangle {
        Rectangle{width,height}
    }
}


mod nation {
    pub mod government {
        pub fn gover(){

        }
    }


    pub mod congress{
        pub fn legislate(){

        }
    }


    mod court{
        pub fn judicial(){}
    }


}

///cfg条件编译
#[cfg(target_os = "linux")]
fn are_you_at_linux(){
    println!("you are running on linux")
}
///cfg条件编译
#[cfg(not(target_os = "linux"))]
fn are_you_at_linux(){
    println!("you are *NOT* running on linux")
}


// This function is only included when either foo or bar is defined
#[cfg(any(foo, bar))]
fn needs_foo_or_bar() {
    // ...
}

// some_condition 是自定义条件
// $ rustc --cfg some_condition custom.rs && ./custom
/*#[cfg(some_condition)]
fn conditional_function() {
    println!("condition met!");
}*/


// snake case 命名法
pub fn main() {
    // 显示一个小螃蟹 🦀 的吉祥物mascot
    Ferris::run();

    // 高亮错误的组件：
    // https://crates.io/crates/color-backtrace
    color_backtrace::install();

    //条件编译
    crate::osshow::osshow::show();

    SimpleTypeElement::run();

    // 复杂对象如何显示
    PrintComplex::run();

    //循环遍历
    Loopp::run();

    //字符串连接
    Strings::run();

    // 运行时 需要加参数，否则会报错，例如：run --package rustHello --bin rustHello complete hello
    // complete hello 是加的2个参数
    ReadFile::run();

    //条件编译
    are_you_at_linux();
    if cfg!(target_os="linux") {
        println!("Yes. It's definitely linux!");
    }else{
        println!("Yes. It's definitely *not* linux!");
    }

    //自定义条件编译
    //conditional_function();


    //一个简单的tcp server
    //front_of_house::tcp_srv::main();
    //front_of_house::tcp_singleThread_srv::run_signle_thread_tcp_srv();

    //::,  it operates on namespacesstd, std is a crate (~ a library), cmp is a module (~ a source file), and min is a function
    let least = std::cmp::min(3,4);
    println!("least is {}",least);

    println!("max is {}", max(5,6));

    println!("Hello, world!");
    hello();
    loopret();

    let s1=String::from("hello");
    let mut s2=&s1;
    let s3=s1;  //所有权转移了。
    //s2 = &s1;   //所有权转移后，这里无法再次租借所有权
    s2 = &s3;
    //println!("{}",s1);

    //run_struct();

    let balck=Color(0,0,254);

    println!("color  is {:#?}", balck); //需要引入Debug 库

    let r=Rectangle{ width:10, height:20};
    let r2=Rectangle{ width:12, height:20};

    println!("area of {} * {} is {}", r.width, r.height, r.area());
    println!("r is wider than r2? {}",r.wider(&r2));

    let r3 = Rectangle::create(21,34);
    println!("create a rectangle {:?}", r3);

    let r4 =Rectangle{
        width:34,
        ..r2        //简化的方式，初始化 剩余字段自动匹配完成
    };
    println!("rectangle r3 is {:?}",r4);


    let Rectangle {width,height} = r4;
    println!("width is {}", width);

    let Rectangle {width,..} = r3;  //解引用，把height丢去
    println!("width is {}", width);

    //枚举
    enum Book {
        Papery { index : u32},
        Electronic {url: String},
        Nothing
    }

    let book= Book::Papery{index:32};
    let ebook = Book::Electronic {url: String::from("http://www.jd.com/ebook")};
    let book3= Book::Nothing;

    match book3 {
        Book::Electronic {url} =>{
            println!(" Electronic book {}", url);
        },
        Book::Papery{index} =>{
            println!(" papery book {}", index);
        },
        _ =>{println!("no match");}
    }


    //Option 枚举
    let some = Option::Some("something555");
    let some2 :Option<&str>=Option::None; //初始值为空的 Option 必须明确类型
    match some2 {
        Option::Some(something) =>{
            println!("{}", something);
        },
        Option::None =>{
            println!("some is nothing")
        }

    }


    //由于 Option 是 Rust 编译器默认引入的，在使用时可以省略 Option:: 直接写 None 或者 Some()
    let some_val =None;//Some(64);
    match some_val {
        Some(64) =>{
            println!("some_val is yes");
        },
        _ =>{
            println!("some_val is nothing");
        }
    }



    //模块
    nation::congress::legislate();

    //模块,会输出  add to waitlist
    crate::front_of_house::hosting::add_to_waitlist();
    host::add_to_waitlist();

    struct_Info::run();


    //panic!("system error");
    read_text_from_file();


    run_g(4);

    front_of_house::sumVal::run();

    front_of_house::traffic_light::run();

    front_of_house::area::main();

    let mut contacts = HashMap::new();

    let acc= (123,345);

    contacts.insert("Daniel", acc);
    let a=contacts.get("Daniel");
    let vre=a.unwrap();


    let aabb=self::vec![1,2,3];  // 有区别吗？
    let ccbb=self::vec!(1,2,3);  // 有区别吗？

    let vloop = self::vec![1,2,3]
        .iter()
        .map(|x| x+3)
        .fold(0,|x,y|x+y);

    let varray :Vec<i32>=self::vec!(1,2,3);
    let varrsize=varray.len();


    NumberClone::runNumber();

    TraitSample::runTraitSample();


    //测试结构体的可见性
    // Public structs with public fields can be constructed as usual
    let openbox = front_of_house::pubStruct::OpenBox{content:"public information"};
    // and their fields can be normally accessed.
    println!("{}", openbox.content);
    //下面这句会报错：但可以通过一个pub的构造函数来初始化，但最后print的时候，还是无法访问私有成员
    //let close_box = front_of_house::pubStruct::CloseBox{content:"close content"};
    let closebox=CloseBox::new("close content");
    //println!("{}", closebox.content);  //无法访问私有成员
    let car = front_of_house::pubStruct::Car(String::from("Tesla"));
    println!("car name:{}", car.0);


    //使用宏
    front_of_house::use_macro::main();

    //关联类型的花式玩法
    front_of_house::associated_type1::main();
    front_of_house::associated_type2::main();
    front_of_house::associated_type3::main();
    front_of_house::associated_type4::main();
    front_of_house::associated_type5::main();


    // 泛型
    GenericFn::run();

    // 闭包
    ClosuresFun::run();

    //生命周期
    Lifetime::run();

    //所有权
    Ownership::run();
    //智能指正
    Boxing::run();
    //智能指针：RefCell<T>
    ReferenceCell::run();
    //traitObject
    TraitObject::run();
    // 多线程
    Threadss::run();
    // 多线程的渠道
    ThreadChannell::run();

    // 数组 和切片
    ArraySlice::run();

    //宏的示例
    macro_sample::hello::run();

    // str, String, vec<u8> vec<char> 的各种转换
    str_conversion::run();

    //演示 workspace
    let add_one_result=add_one::add_one(4);
    println!("add-one from another module:{}",add_one_result);

    // 演示外部模块可见性
    visibilitytestOuter::run();
    //演示match的模式匹配
    matchtest::run();





    println!("\n\nend")
}



fn hello(){
    let mut number = 1;
    while number !=4 {
        println!("{}", number);
        number+=1;
    }

    println!("EXIT");
}

fn loopret(){
    let s=['R', 'U', 'N', 'O', 'O', 'B'];
    let mut i=0;
    let location= loop {
        let ch=s[i];
        if ch=='O' {
            break i;
        }
        i+=1;
    };

    println!("\'O\' 的索引是{}", location);

}

/**
伴随着 dangle 函数的结束，其局部变量的值本身没有被当作返回值，被释放了。但它的引用却被返回，这个引用所指向的值已经不能确定的存在，故不允许其出现。
 **/
fn dangle() -> String {
    let s=String::from("dangle");
    //&s
    s
}

fn read_text_from_file() {

    let s= std::fs::File::open("file.txt");
    match s {
        Ok(file) =>{
            println!("file is ok");
        },
        Err(err) =>{
            println!("read file error:{}",err);
        }
    }


    let f=std::fs::File::open("file.txt");
    if let Ok(file) =f {
        println!("file is ok");
    }else{
        println!("file is not open");
    }


}



fn g(i: i32) -> Result<i32, bool>{
    let fc=f(i);
    return match fc {
        Ok(i) => Ok(i),
        Err(b) =>Err(b)
    }// 函数 g 传递了函数 f 可能出现的错误
}


fn f(i: i32) -> Result<i32, bool> {
    if (i>=0){
        Ok(i)
    }else {
        Err(false)
    }
}
fn g2() -> Result<i32, bool>{
    let v=f(6)?;
    Ok(v)
}
fn run_g(i: i32){
    let rd =f(6);
}

