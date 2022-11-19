use proc_macro::TokenStream;
/*定义自己的过程宏，必须在一个单独的crate里。*/
#[proc_macro_derive(WhoAmI)]
pub fn whatever_you_want(tokens: TokenStream) -> TokenStream {
    // convert the input tokens into an ast, specially from a derive
    let ast: syn::DeriveInput = syn::parse(tokens).unwrap();

    // 这里不能用 println! 这种一般性的Rust宏，这里只能制造一个 panic，在编译期报错，可以输出一些信息
    panic!("My struct name is: <{}>", ast.ident.to_string());

    TokenStream::new()
}







