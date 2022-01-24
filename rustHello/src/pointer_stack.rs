





pub fn run(){

    let a = "hello";
    let b = &a;

    println!("a: {}, b: {}", a, b);
    println!("&a: {:p}, &b: {:p}", &a, &b);

    let d_s= String::from("Hello");
    let mut d_str =&d_s[..];
    let a_r=strtok(&mut d_str);
    println!("a_r:{}",a_r)

}

///Rust 的引用实现了 Copy trait，所以按照 Copy 语义，
/// 这个引用会被复制一份交给要调用的函数。对这个函数来说，
/// 它并不拥有数据本身，数据只是临时借给它使用，所有权还在原来的拥有者那里。
/// str 的地址被copy 后传递到了 strtok
fn strtok<'a>(str: &'a mut &str) -> &'a str {
    let str_address = &str;
    println!("mut str:{}", str);
    ""

    /*

str所在地址：                 该地址上存储的内容( x 0x00007ffeef6eb6d8 )
0x00007ffeef6eb6d8  →   c0 b8 6e ef fe 7f 00 00
     |                       把该值当做地址：                      查看该地址上的内容( x 0x00007ffeef6eb8c0 )：
     |                       0x00007ffeef6eb8c0      →       0x7ffeef6eb8c0: 80 12 c0 c4 a9 7f 00 00 05  00 00 00 00 00 00 00
     |                              |                                        -----------------------  └这是len
     |                              |                                        ┌-------------------------
     |                              |                                        │↪     把该值当做地址   ↩
     |                              |                                        │        ↓
     |                              |                                        │        ↓ （x 0x00007fa9c4c01280）
     |                              |                                        │
     |                              |                                        │   0x7fa9c4c01280: 48 65 6c 6c 6f 00 00 a0 00 00 00 00 00 00 00 a0  Hello...........
     |                              |                                        │                   --------------
     |                              |                                        │                   这串内容是`hello`
     |                              |                                        │
     |                              |                                        │
     |                              |                                        │
     |                              |                                        |
    &str(或str_address)            str                                     *str

1）str 这个变量在 strtok 函数中是一个入参，它的值是 0x00007ffeef6eb8c0，
2）str 这个变量在 strtok 函数栈中的地址是 &str，该内存位置是 0x00007ffeef6eb6d8
3）如果把 0x00007ffeef6eb8c0 这个值当做地址看待，它所指向的内容是 *str
4）*str 的内容是 80 12 c0 c4 a9 7f 00 00 05 ，它的内容包含了 80 12 c0 c4 a9 7f 00 00  和  05 2部分。
5）如果以指针的方式来解释该内容，它是一个胖指针，包含 指针 0x7fa9c4c01280 和 05 长度
6）0x7fa9c4c01280 所指向的内容是 48 65 6c 6c 6f 00 00 a0 .. ..
7）我们不以地址的方式来解读该内容，并且只截取该地址起始的5个长度，即 48 65 6c 6c 6f ，它表示 hello
8）所以 如果以 str 为中心，
0x00007ffeef6eb6d8    c0 b8 6e ef fe 7f 00 00     @.P?............
  ↑ &str 表示找 左边的地址

    * str 表示找 右边的地址 ↓
0x00007ffeef6eb8c0    80 12 c0 c4 a9 7f 00 00 05

*/

}