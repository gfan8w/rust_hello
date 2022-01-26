use std::collections::HashMap;
use std::mem::size_of_val;

fn run_mem_layout(){
    // 长度 0
    let a =|| println!("hello");

    //与入参无关，长度0
    let b = |str: String| println!("{}",str);

    let name =String::from("Tom");
    let name1 =name.clone();
    let name2 =name.clone();
    let mut table =HashMap::new();
    table.insert("hello","world");

    //捕获一个引用，长度是8，胖指针
    let c =||println!("{}",name);

    // 捕获移动的数据 name1(长度 24) + table(长度 48)，closure 长度 72
    let d =move||println!("{},{:?}",name1,table);

    // 和局部变量无关，捕获了一个 String name2，closure 长度 24
    let e =|| {
        let x =1;
        let name3 ="Baby".to_string();
        println!("{},{},{}",x,name2,name3);
    };

    println!("size: a:{}, b:{}, c:{}, d:{},e:{}, main():{}",
            size_of_val(&a),  // a 没有参数，也没捕获任何变量，从代码输出可以看到，a 长度为 0；
             size_of_val(&b), //b 有一个 i32 作为参数，没有捕获任何变量，长度也为 0，可以看出参数跟闭包的大小无关；
             size_of_val(&c), // c 捕获了一个对变量 name 的引用，这个引用是 &String，长度为 8。而 c 的长度也是 8；
             size_of_val(&d), // d捕获了变量 name1 和 table，由于用了 move，它们的所有权移动到了 c4 中。
                                    // d 长度是 72，恰好等于 String 的 24 字节，加上 HashMap 的 48 字节
             size_of_val(&e),  // e 捕获了 name2，name2 的所有权移动到了 e，虽然 e 有局部变量，但它的大小和局部变量也无关，e 的大小等于 String 的 24 字节
             size_of_val(&run_mem_layout),
        )

}

/*
(lldb) frame variable   #查看所有栈上变量
(rustHello::closure_sample::closure_mem_layout::run_mem_layout::{closure#0}) a = {}
(rustHello::closure_sample::closure_mem_layout::run_mem_layout::{closure#1}) b = {}
(alloc::string::String) name = "Tom" {
  vec = size=3 {
    [0] = 84
    [1] = 111
    [2] = 109
  }
}
....
(rustHello::closure_sample::closure_mem_layout::run_mem_layout::{closure#2}) c = {
  _ref__name = 0x0000700000ae0728
}
....

以c 闭包为例，它捕获了 &String，长度为8

(lldb) p &c
(*mut rustHello::closure_sample::closure_mem_layout::run_mem_layout::{closure#2}) &c = 0x0000700000ae07a0


(lldb) x/xg &c                     #以8字节为单位，重复1次，16进制的格式显示
0x700000ae07a0: 0x0000700000ae0728
(lldb) x/3xg 0x0000700000ae0728    #以8字节为单位，重复3次，16进制的格式显示
0x700000ae0728: 0x00007fe595c024d0 0x0000000000000003
0x700000ae0738: 0x0000000000000003
(lldb) x/3cb 0x00007fe595c024d0   #以 1字节为单位，重复3次 字符c的格式显示,
0x7fe595c024d0: Tom


*/


/*
x命令查看内存的格式：
x/<n/f/u>  <addr>
n:repeat number 是正整数，表示需要显示的内存单元的个数，即从当前地址向后显示n个内存单元的内容，
一个内存单元的大小由第三个参数u定义。

f:display format 表示addr指向的内存内容的输出格式，s对应输出字符串，此处需特别注意输出整型数据的格式：
  x 按十六进制格式显示变量.
  d 按十进制格式显示变量。
  u 按十进制格式显示无符号整型。
  o 按八进制格式显示变量。
  t 按二进制格式显示变量。
  a 按十六进制格式显示变量。
  c 按字符格式显示变量。
  f 按浮点数格式显示变量。

u:就是指以多少个字节作为一个内存单元-unit,默认为4。u还可以用被一些字符表示:
  如b=1 byte, h=2 bytes,w=4 bytes,g=8 bytes.

  Bytes
  Halfwords (two bytes)
  Words (four bytes).
  Giant words (eight bytes)

<addr>:表示内存地址。

f与u的位置先后是没关系的。f与u的字符没有重叠的，先后顺序没关系。
 x/3xg  =x/3gx 一样的

*/



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_run_mem_layout() {
        run_mem_layout();
    }
}


