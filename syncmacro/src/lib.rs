

use proc_macro::{TokenStream};
use proc_macro2::{Ident};
use quote::{quote, ToTokens};
use syn::{braced, DeriveInput, parse_macro_input, Token, token};
use syn::parse::{Parse, ParseStream};

extern crate proc_macro;

//TokenStream 相当编译过程中的语法树的流。

/// 函数式宏
#[proc_macro]
pub fn make_hello(item: TokenStream) -> TokenStream {

    let name = item.to_string();
    let hell = "Hello ".to_string()+name.as_ref();
    let fn_name = "fn hello_".to_string() +name.as_ref() +"(){ println!(\""+hell.as_ref()+"\");}";

    fn_name.parse().unwrap()
}

/// 属性宏 （两个参数）
#[proc_macro_attribute]
pub fn log_attr(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("Attr: {}", attr.to_string());
    println!("Attr: {}", item.to_string());
    item
}

#[proc_macro_derive(Hello)]
pub fn hello_derive(input: TokenStream) -> TokenStream {

    println!("{:?}", input);
    TokenStream::new()
    // 如果直接返回input，编译会报重复定义，说明派生宏用于扩展定义
    // input
}



// 使用 sync 库

// 定义自己的存储结构
struct ItemStruct {
    pub struct_token: Token![struct],
    pub ident: Ident,
    pub brace_token: token::Brace,
}

// 实现syn::parse::Parse 接口
impl Parse for ItemStruct {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        let lookahead =input.lookahead1();

        if lookahead.peek(Token![struct]) {
            return Ok(ItemStruct{
                struct_token: input.parse()?,
                ident: input.parse()?,
                brace_token: braced!(content in input)
            });
        }
        Err(lookahead.error())
    }
}


#[proc_macro_derive(SynTest)]
pub fn syn_test(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);
    println!("MyParse:{:?}", input.struct_token.to_token_stream());
    println!("MyIdent:{:?}", input.ident.to_token_stream());
    TokenStream::new()
}

///通过Syn生成语法树后，后面可用使用quote! 处理源码树，实现需要的扩展功能。
//
// 使用过程宏为struct添加hello方法：
//
// 使用parse_macro_input 解析到DeriveInput
// 获取struct name TokenStream
// 使用quote! 扩展改结构体
#[proc_macro_derive(HelloTest)]
pub fn test_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    println!("{:?}", input.ident.to_token_stream());
    let name = input.ident.to_token_stream();

    let expanded =quote!{
        impl #name {
            pub fn hello() {
                println!("helllooo wooorld");
            }
        }
    };

    expanded.into()


}




































#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
