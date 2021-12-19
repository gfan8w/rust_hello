
/// 来源：https://doc.rust-lang.org/stable/book/ch04-01-what-is-ownership.html
///      https://doc.rust-lang.org/stable/book/ch10-03-lifetime-syntax.html

pub fn run () {
    println!("OwnerShip demo:");


    let hello ="hello".to_string();
    let (greeting,size) = takes_and_gives_back(hello);  // hello的所有权移动到函数内，函数又返回该所有权，并返回一个i32的所有权
    println!("greeting:{}, size:{}",greeting,size);

    let world=give_ownership();

    let size1=calculate_length(&world);
    let size2 =calculate_length2(&world);

    println!("{},{}",size1, size2);

    let dangling_str=dangling();

    // 多个引用，后不能跟可变引用
    multi_reference();
    // 多个可变引用 也是 不可以的
    multi_mut_reference();
    // 查找第一个单词，练习slice，切片
    find_first_words();
}

// 该函数获取了一个string和它的所有权，最后又返回一个string和它的所有权，并返回一个i32的所有权给调用者
fn takes_and_gives_back(str: String) ->(String,usize) {
    let len=str.len();
    (str,len)
}

fn give_ownership() -> String {                 // gives_ownership will move its
    let s=String::from("world");      // return value into the function
                                                // that calls it

     s                                          // s is returned and
                                                // moves out to the calling
                                                // function
}

fn calculate_length(s: &String) -> usize {      //s is a reference to a String
    s.len()
}// Here, s goes out of scope. But because it does not have ownership of what
// it refers to, nothing happens.

fn calculate_length2(s: &str) -> usize {
    s.len()
}

fn dangling() -> &'static str {   //这个能正常运行
    let s= "hello";
    s
}

/*
fn dangling_2() -> &String {          //这个不会运行，悬空指针，返回一个局部变量的引用
                                    // dangle returns a reference to a String
    let s= String::from("hello");  // s is a new String
    &s                              // we return a reference to the String, s
}// Here, s goes out of scope, and is dropped. Its memory goes away.
// Danger!
// The solution here is to return the String directly:
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
*/


fn multi_reference() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3); //ok

    /* 有问题的代码： 会引起数据竞争（Data races ）的都不允许
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}, and {}", r1, r2, r3);
    //这句话会报错，多个引用，还有一个可变引用是不行的。r3如果改变数据，r1、r2 的只读不可知，

    */
}

fn multi_mut_reference(){
    let mut s =String::from("hello");

    {   //增加一层作用域隔离，是可以 有多个 可变引用的
        let r1 =&mut s;
    }// r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 =&mut s;


    /* 有问题的代码：
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;    //这里有问题，会引起数据竞争(Data races)的都有问题

    println!("{}, {}", r1, r2);
    */
}

fn find_first_words(){
    let mut s= String::from("hello world Beijing I love you");
    let first =first_words(&s); //这里是一个不可变引用，不可变引用被first hold住了，
    //s.clear();                        // 这里是一个可变引用。
    println!("first word is:{}",first); //这里继续使用那个不可变引用，但是前面有个可变引用导致数据竞争后不可控，故报错
    //要正确运行 ^ ,注释掉 s.clear(); 或 println!这两行里的任意一行，为什么呢？因为 不可变引用与可变引用不能同时存在
    // 我这里临时注释掉 s.clear();
}


///查找一段字符串中的第一个单词，以空格为标记，如果没有空格，就返回整个字符串作为一个单词
fn first_words(s: &String) ->&str {  // fn first_words(s: &str) ->&str  -> 这里参数使用 &str会更通用
     let bytes = s.chars().into_iter().enumerate();
    for (pos,ch) in bytes {
        if ch==' ' {
           return &s[..pos]
        }
    }

    &s[..]

}













