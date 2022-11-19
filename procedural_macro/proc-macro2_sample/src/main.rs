use std::str::FromStr; // TokenStream::from_str 需要
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::ItemStruct;

fn main() {
    println!("Hello, procedural macro!");

    // 样例结构体
    let s = "struct Point { x: u16, y: u16 } ";

    // 创建一个tokenstream
    let tokens = TokenStream::from_str(s).unwrap();

    //创建 AST，我们使用syn::parse2，而不是syn::parse
    let ast: ItemStruct = syn::parse2(tokens).unwrap();

    // 保存结构体类型名称
    let struct_type = ast.ident.to_string();
    assert_eq!(struct_type, "Point");

    //我们有2个字段
    assert_eq!(ast.fields.len(), 2);

    // syn:Fields 实现了Iterator trait
    let mut iter = ast.fields.iter();

    //找到x
    let x_field = iter.next().unwrap();
    assert_eq!(x_field.ident.as_ref().unwrap(), "x");

    //找到y
    let y_field = iter.next().unwrap();
    assert_eq!(y_field.ident.as_ref().unwrap(), "y");


    // 使用 quote!()来产生新代码

    // 首先构建函数名： point_summation
    let function_name = format_ident!("{}_sumation", struct_type);
    // 构建参数类型，如果我们不使用format_ident! 宏，最后的方法是这样的：
    // pub fn point_summation (pt : "Point")，类型那里是 字符串
    let argument_type = format_ident!("{}", struct_type);
    // 构建参数x，y
    let x = format_ident!("{}", x_field.ident.as_ref().unwrap());
    let y = format_ident!("{}", y_field.ident.as_ref().unwrap());

    // 最神奇的地方： 使用 quote!() 类构建方法，它返回的是 TokenStream，
    // 这个stream是返回给编译器的"真实"的过程宏
    let summation_fn = quote! {
        pub fn #function_name(pt: &#argument_type) -> u16 {
            pt.#x + pt.#y
        }
    };
    // 打印我们的方法
    println!("{}", summation_fn);


    //如果我们不知道具体Point 有多少个成员，则可以遍历它
    // 创建一个 token的列表，tokens的类型是 impl Iterator<Item = TokenStream>
    let tokens = ast.fields.iter().map(|i| quote!(pt.#i));

    // 这里是  0 #(+ #tokens)*， 0是当tokens数目为0时的时候的一个默认值，避免出错，
    // * 是表示重复 + #tokens，有多个token，前面有一个 0+ ，不影响结果
    let summation_fn = quote! {
        pub fn #function_name(pt: &#argument_type) -> u16 {
             0 #(+ #tokens)*
        }
    };
    // 打印我们的方法
    println!("{}", summation_fn);









}














