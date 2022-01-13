use hex_literal;
use syncmacro::{make_hello,log_attr,Hello,SynTest,HelloTest};

make_hello!(world);
make_hello!(张三);

#[log_attr(struct, "world")]
struct Hello{
    pub name: String,
}

#[log_attr(func, "test")]
fn invoked(){}

#[derive(Hello)]
struct World;


#[derive(SynTest)]
struct East{}

#[derive(HelloTest)]
struct Test {}

pub fn main() {
   // 使用make_hello生成
    /*
    make_hello 使用#[proc_macro]
    定义自动生成一个传入参数函数。
    Hello world
    Hello 张三
    */
    hello_world();
    hello_张三();

    Test::hello();

    let hx = hex_literal::hex!("8a1ed431fa78b83f195e228c47777cc4661916fd8b1571ac4e9801ae56560952").to_vec();
    let hexStr =String::from_utf8(hx).unwrap();
    println!("hex:{}", hexStr)

}


/*
log_attr 使用#[proc_macro_attribute]

编译期间会打印结构类型和参数，后面可用修改替换原属性定义。

Attr:struct, "world"

Item:struct Hello { pub name : String, }

Attr:func, "test"

Item:fn invoked() { }

#[derive(Hello)]  使用#[proc_macro_derive(Hello)]·

会打印当前TokenStream 结点流，后面可以合 syn 与 quto 库结合，扩展定义。

TokenStream [Ident { ident: "struct", span: #0 bytes(286..292) }, Ident { ident: "World", span: #0 bytes(293..298) }, Punct { ch: ';', spacing: Alone, span: #0 bytes(298..299) }]


*/

