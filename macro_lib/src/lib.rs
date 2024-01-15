use proc_macro::TokenStream;
use std::ptr::write;

use quote::{quote, ToTokens};
use syn::{DeriveInput, Expr, ExprArray, Ident, ItemFn, parse_macro_input, Token};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Comma;

//<editor-fold desc="派生宏模拟">
///
/// 这是一个派生宏demo，核心要点是他们必须在单独的包里定义
/// 只能在lib.rs里面定义宏解析函数
/// 要派生的属性和类尽可能用全路径名称。避免派生后找不到该属性和函数
///
#[proc_macro_derive(SuperTrait)]
pub fn generate_trait(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    do_generate_trait(input)
}

fn do_generate_trait(input: DeriveInput) -> TokenStream {
    let name = input.ident;
    let stream = quote!(
            impl SuperTrait for  #name{
                fn do_some_echo() {
                    println!("struct name is value = {:?}", stringify!(#name))
                }

                fn do_some_obj_echo(&self) {
                    println!("obj echo {}", stringify!(#name));
                }
            }
        );
    TokenStream::from(stream)
}
//</editor-fold>


//<editor-fold desc="属性宏demo">
///
///本节介绍属性宏
/// 属性宏类似java的注解，能接受item作为参数
///
/// 本demo模拟定义一个GET方法的属性宏，并提取其中的uri
///
#[proc_macro_attribute]
pub fn get_method(meta: TokenStream, token: TokenStream) -> TokenStream {
    let args_path = parse_macro_input!(meta as ArgsPath);
    let expr_array = ExprArray {
        attrs: vec![],
        bracket_token: Default::default(),
        elems: args_path.path,
    };

    // 提取方法中的内容
    let item_fn = parse_macro_input!(token as ItemFn);
    let signature = &item_fn.sig;
    let ident = signature.ident.to_token_stream();
    let fn_token = signature.fn_token.to_token_stream();
    let visibility = &item_fn.vis.to_token_stream();
    eprintln!("test=={},{},{}", visibility, fn_token, ident);

    let output = quote! {
        pub fn just_t(){
            println!("the method ={:?}, path = {:?}", #ident, #expr_array);
        }
    };
    TokenStream::from(output)
}


///
/// 首先需要定义一个结构体，用于封装解析的结果
///
struct ArgsPath {
    pub path: Punctuated<Expr, Comma>,
}


impl Parse for ArgsPath {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let result: Punctuated<Expr, Comma> = Punctuated::<Expr, Token![,]>::parse_terminated(input)?;
        Ok(
            ArgsPath {
                path: result,
            }
        )
    }
}

//</editor-fold>
