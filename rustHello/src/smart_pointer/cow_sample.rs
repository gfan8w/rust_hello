use std::borrow::{Borrow, Cow};
use std::mem::size_of;

// 这里 讲 Cow<'a,B:ToOwned> 这个 Copy-On-Write 类型。
//先介绍了 Borrow 这个trait


///borrow trait简单介绍，顺便牵扯出 &String he &str差别
///  str，&str，String 对懂C语言的人来说，差异如下：
//struct str {
//    char text[]; // 只知道起始地址，无结束地址，即 无长度
// }
//
// // &str 是一个胖指针，带起始地址 和一个长度，这样就能确定一段内容
// struct &str {
//    size_t length;
//    const char *text; // not owned
// };
//
// //String包含3个：长度、容量、起始地址
// struct String {
//    size_t length;
//    size_t free_capacity;
//    char *text; // malloc()'ed
// }
fn run_borrow(){
    let a ="hello".to_string();

    // 这里必须声明类型，因为 String 有多个 Borrow 实现
    // 这里借用为 &String
    let r1: &String = a.borrow();

    // 借用为 &str
    let r2: &str = &a.borrow();


    /*
    hello字符串的内存：
    (lldb) x/c 0x00007f85dc800190
    0x7f85dc800190: hello\0\0\0

    // 查看 a，r2，r1 3个值 在栈上的地址如下：
    (lldb) p &a
    (*mut alloc::string::String) &a = 0x000070000183e860
    (lldb) p &r2
    (*mut &str) &r2 = 0x000070000183e880
    (lldb) p &r1
    (*mut &String) &r1 = 0x000070000183e878

    // 3个地址 从 e860 开始增长，到 e880结束，
    // 1) 0x000070000183e860开始的，以8字节为步长，往后连续3个空间
    //    分别是  0x00007f85dc800190， 05， 05 ： 分别是 hello的位置，长度、容量
    // 2) 0x000070000183e878 的位置是 e870+8的位置， 以8字节为步长，往后连续1个空间:
    //    内容是 0x000070000183e860，它又回到 1)的地方
    // 3) 0x000070000183e880 以8字节为步长，往后连续2个空间,
    //    内容是 0x00007f85dc800190，05 ： 分别是 hello的位置，长度
    (lldb) x/8gx 0x000070000183e860
    0x70000183e860: 0x00007f85dc800190 0x0000000000000005
    0x70000183e870: 0x0000000000000005 0x000070000183e860
    0x70000183e880: 0x00007f85dc800190 0x0000000000000005
    0x70000183e890: 0x0000000000000000 0x0000000104b14000

    从2）这里看出 &String 是什么？它是一个指针，指向 完整的 String 结构体
    从3) 这里看出 &str 是什么？ 它是一个胖指针，指向 String结构体后端存数据的那个存储，另外还有一个长度信息
         这里长度 恰好等于5。如果我们 let r2: &str = &r2[1..3]; 这样的话，保存那段长度的内存值是 2.
    所以，&str 是把原来String的 len 和 capacity 都丢弃了，只保留数据，然后再补上自己需要的长度值。
    */

    /*
    str 和 [T] 是在所有编程语言里都有的基本概念。在Rust里，他们只表示一段连续的空间，但不知道该空间在何时结束,也不知道该空间具体在哪里，既可以在栈上，也可以在堆上
    Vec<T> 是一个堆上分配的，可动态增长的，具有所有权的[T]。引用是 &[T]
    String 是一个堆上分配的，可动态增长的，具有所有权的str。引用是 &str
    String 的底层是 Vec<u8>，String和 &str 会正确处理uft8字符，ascii码是一个u8表示一个字符，其他的是 2~4个u8表示一个字符。
    底层也可以用Vec<char>,但每个char 都要占4个字节，耗内存。
    不用Vec<u8>来直接表示String是因为utf8长度处理由String来处理，而不是交由自己的代码处理
    */


    /*
    [T]:        一段连续空间，其他都位置，有3个主要的未知：where，who，how；where to store？ who own it? how managed?
    Box<[T]>:   拥有数据，排他性的访问该数据能力，String 和 Vec<T> 位于此， String能处理utf8变长字符；Vec<T>可以动态增长，维护一个容量
    Rc<T>:      所有权是共享的。
    [T;n]:      长度为n的一段T数据。

    */





    println!("r1: {:p}, r2: {:p}", r1, r2);


    println!("size of char:{}",size_of::<char>());

}

fn run_borrow_1(){
    // 这里没有定义一个临时变量来存放 "hello".to_string()，导致 hello的所有权被立刻释放，导致编译错误
    //let r1:&String = "hello".to_string().borrow();
    //println!("r1: {:p}", &r1);

}











#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_run_borrow() {
        run_borrow()
    }

}











